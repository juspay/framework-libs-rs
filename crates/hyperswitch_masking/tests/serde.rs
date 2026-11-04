//! masked_serialize tests

use std::collections::HashMap;

use hyperswitch_masking::{ErasedMaskSerialize, Secret, SerializableSecret};
use serde::Serialize;
use serde_json::json;

const MASKED_STRING: &str = "*** alloc::string::String ***";

#[derive(Serialize)]
struct PlainStruct {
    card: Secret<String>,
    visible: String,
}

impl SerializableSecret for PlainStruct {}

#[test]
fn secret_fields_are_masked() {
    let value = PlainStruct {
        card: Secret::new("4111 1111 1111 1111".to_string()),
        visible: "foo".to_string(),
    };

    let masked = value.masked_serialize().expect("masked_serialize failed");
    assert_eq!(masked, json!({ "card": MASKED_STRING, "visible": "foo" }));

    let exposed = serde_json::to_value(&value).expect("to_value failed");
    assert_eq!(
        exposed,
        json!({ "card": "4111 1111 1111 1111", "visible": "foo" })
    );
}

#[derive(Serialize)]
struct Inner {
    card: Secret<String>,
    visible: String,
}

impl SerializableSecret for Inner {}

#[derive(Serialize)]
struct Outer {
    name: String,
    #[serde(flatten)]
    inner: Inner,
}

impl SerializableSecret for Outer {}

#[test]
fn flatten_keeps_secrets_masked() {
    let value = Outer {
        name: "test-order".to_string(),
        inner: Inner {
            card: Secret::new("4111 1111 1111 1111".to_string()),
            visible: "foo".to_string(),
        },
    };

    let masked = value.masked_serialize().expect("masked_serialize failed");
    assert_eq!(
        masked,
        json!({ "name": "test-order", "card": MASKED_STRING, "visible": "foo" })
    );

    let exposed = serde_json::to_value(&value).expect("to_value failed");
    assert_eq!(
        exposed,
        json!({
            "name": "test-order",
            "card": "4111 1111 1111 1111",
            "visible": "foo"
        })
    );
}

#[derive(Serialize)]
struct AllSecret {
    token: Secret<String>,
    key: Secret<String>,
}

impl SerializableSecret for AllSecret {}

#[derive(Serialize)]
struct AllSecretHolder {
    id: String,
    #[serde(flatten)]
    secrets: AllSecret,
}

impl SerializableSecret for AllSecretHolder {}

#[test]
fn flatten_of_all_secret_struct() {
    let value = AllSecretHolder {
        id: "abc".to_string(),
        secrets: AllSecret {
            token: Secret::new("secret_123".to_string()),
            key: Secret::new("key_456".to_string()),
        },
    };

    let masked = value.masked_serialize().expect("masked_serialize failed");
    assert_eq!(
        masked,
        json!({ "id": "abc", "token": MASKED_STRING, "key": MASKED_STRING })
    );

    let exposed = serde_json::to_value(&value).expect("to_value failed");
    assert_eq!(
        exposed,
        json!({ "id": "abc", "token": "secret_123", "key": "key_456" })
    );
}

#[derive(Serialize)]
struct Grandchild {
    ssn: Secret<String>,
}

impl SerializableSecret for Grandchild {}

#[derive(Serialize)]
struct NestedParent {
    name: String,
    #[serde(flatten)]
    child: Grandchild,
}

impl SerializableSecret for NestedParent {}

#[derive(Serialize)]
struct NestedGrandparent {
    label: String,
    #[serde(flatten)]
    parent: NestedParent,
}

impl SerializableSecret for NestedGrandparent {}

#[test]
fn nested_flatten() {
    let value = NestedGrandparent {
        label: "top".to_string(),
        parent: NestedParent {
            name: "mid".to_string(),
            child: Grandchild {
                ssn: Secret::new("123-45-6789".to_string()),
            },
        },
    };

    let masked = value.masked_serialize().expect("masked_serialize failed");
    assert_eq!(
        masked,
        json!({ "label": "top", "name": "mid", "ssn": MASKED_STRING })
    );

    let exposed = serde_json::to_value(&value).expect("to_value failed");
    assert_eq!(
        exposed,
        json!({ "label": "top", "name": "mid", "ssn": "123-45-6789" })
    );
}

#[derive(Serialize)]
struct MapFlattenHolder {
    id: String,
    #[serde(flatten)]
    extra: HashMap<String, Secret<String>>,
}

impl SerializableSecret for MapFlattenHolder {}

#[test]
fn flatten_of_hashmap() {
    let mut extra = HashMap::new();
    extra.insert(
        "card".to_string(),
        Secret::new("4111 1111 1111 1111".to_string()),
    );
    extra.insert("pin".to_string(), Secret::new("9999".to_string()));

    let value = MapFlattenHolder {
        id: "abc".to_string(),
        extra,
    };

    let masked = value.masked_serialize().expect("masked_serialize failed");
    // HashMap iteration order is unspecified, so compare sorted keys
    let masked_obj = masked.as_object().expect("masked should be object");
    assert_eq!(masked_obj.len(), 3);
    assert_eq!(masked_obj["id"], "abc");
    assert_eq!(masked_obj["card"], MASKED_STRING);
    assert_eq!(masked_obj["pin"], MASKED_STRING);

    let exposed = serde_json::to_value(&value).expect("to_value failed");
    let exposed_obj = exposed.as_object().expect("exposed should be object");
    assert_eq!(exposed_obj.len(), 3);
    assert_eq!(exposed_obj["id"], "abc");
    assert_eq!(exposed_obj["card"], "4111 1111 1111 1111");
    assert_eq!(exposed_obj["pin"], "9999");
}

#[derive(Serialize)]
struct MaybeSecret {
    #[serde(skip_serializing_if = "Option::is_none")]
    token: Option<Secret<String>>,
}

impl SerializableSecret for MaybeSecret {}

#[derive(Serialize)]
struct OptionFlattenHolder {
    id: String,
    #[serde(flatten)]
    inner: Option<MaybeSecret>,
}

impl SerializableSecret for OptionFlattenHolder {}

#[test]
fn flatten_of_some_option() {
    let value = OptionFlattenHolder {
        id: "with-secret".to_string(),
        inner: Some(MaybeSecret {
            token: Some(Secret::new("secret_123".to_string())),
        }),
    };

    let masked = value.masked_serialize().expect("masked_serialize failed");
    assert_eq!(
        masked,
        json!({ "id": "with-secret", "token": MASKED_STRING })
    );

    let exposed = serde_json::to_value(&value).expect("to_value failed");
    assert_eq!(
        exposed,
        json!({ "id": "with-secret", "token": "secret_123" })
    );
}

#[test]
fn flatten_of_none_option() {
    let value = OptionFlattenHolder {
        id: "no-secret".to_string(),
        inner: None,
    };

    let masked = value.masked_serialize().expect("masked_serialize failed");
    assert_eq!(masked, json!({ "id": "no-secret" }));

    let exposed = serde_json::to_value(&value).expect("to_value failed");
    assert_eq!(exposed, json!({ "id": "no-secret" }));
}

#[derive(Serialize)]
struct CardDetails {
    number: Secret<String>,
    expiry: String,
}

impl SerializableSecret for CardDetails {}

#[derive(Serialize)]
#[serde(tag = "type")]
enum Payment {
    #[serde(rename = "card_payment")]
    Card {
        id: String,
        #[serde(flatten)]
        card: CardDetails,
    },
}

impl SerializableSecret for Payment {}

#[test]
fn flatten_inside_enum_variant() {
    let value = Payment::Card {
        id: "pay_123".to_string(),
        card: CardDetails {
            number: Secret::new("4111 1111 1111 1111".to_string()),
            expiry: "12/29".to_string(),
        },
    };

    let masked = value.masked_serialize().expect("masked_serialize failed");
    assert_eq!(
        masked,
        json!({
            "type": "card_payment",
            "id": "pay_123",
            "number": MASKED_STRING,
            "expiry": "12/29"
        })
    );

    let exposed = serde_json::to_value(&value).expect("to_value failed");
    assert_eq!(
        exposed,
        json!({
            "type": "card_payment",
            "id": "pay_123",
            "number": "4111 1111 1111 1111",
            "expiry": "12/29"
        })
    );
}

#[derive(Serialize)]
#[serde(rename_all = "lowercase")]
enum CredentialKind {
    Password(Secret<String>),
}

impl SerializableSecret for CredentialKind {}

#[derive(Serialize)]
struct AuthDetails {
    username: String,
    credential: CredentialKind,
}

impl SerializableSecret for AuthDetails {}

#[derive(Serialize)]
struct AuthRequest {
    request_id: String,
    #[serde(flatten)]
    auth: AuthDetails,
}

impl SerializableSecret for AuthRequest {}

#[test]
fn enum_with_secret_inside_flatten() {
    let value = AuthRequest {
        request_id: "req_1".to_string(),
        auth: AuthDetails {
            username: "alice".to_string(),
            credential: CredentialKind::Password(Secret::new("hunter2".to_string())),
        },
    };

    let masked = value.masked_serialize().expect("masked_serialize failed");
    // Newtype variant of a non-unit enum serializes as {"password": <inner>}
    assert_eq!(
        masked,
        json!({
            "request_id": "req_1",
            "username": "alice",
            "credential": { "password": MASKED_STRING }
        })
    );

    let exposed = serde_json::to_value(&value).expect("to_value failed");
    assert_eq!(
        exposed,
        json!({
            "request_id": "req_1",
            "username": "alice",
            "credential": { "password": "hunter2" }
        })
    );
}

#[derive(Serialize)]
struct NestedPayload {
    reference: Secret<String>,
}

impl SerializableSecret for NestedPayload {}

#[derive(Serialize)]
struct IndirectedInner {
    codes: Vec<Secret<String>>,
    payload: NestedPayload,
    #[serde(skip_serializing_if = "Option::is_none")]
    pin: Option<Secret<String>>,
}

impl SerializableSecret for IndirectedInner {}

#[derive(Serialize)]
struct IndirectedHolder {
    id: String,
    #[serde(flatten)]
    inner: IndirectedInner,
}

impl SerializableSecret for IndirectedHolder {}

#[test]
fn secrets_in_collections_and_nested_struct() {
    let value = IndirectedHolder {
        id: "xyz".to_string(),
        inner: IndirectedInner {
            codes: vec![
                Secret::new("code1".to_string()),
                Secret::new("code2".to_string()),
            ],
            payload: NestedPayload {
                reference: Secret::new("ref_9".to_string()),
            },
            pin: None,
        },
    };

    let masked = value.masked_serialize().expect("masked_serialize failed");
    assert_eq!(
        masked,
        json!({
            "id": "xyz",
            "codes": [MASKED_STRING, MASKED_STRING],
            "payload": { "reference": MASKED_STRING }
        })
    );

    let exposed = serde_json::to_value(&value).expect("to_value failed");
    assert_eq!(
        exposed,
        json!({
            "id": "xyz",
            "codes": ["code1", "code2"],
            "payload": { "reference": "ref_9" }
        })
    );
}

#[test]
fn option_secret_emitted_when_some() {
    let value = IndirectedHolder {
        id: "xyz".to_string(),
        inner: IndirectedInner {
            codes: vec![Secret::new("code1".to_string())],
            payload: NestedPayload {
                reference: Secret::new("ref_9".to_string()),
            },
            pin: Some(Secret::new("9999".to_string())),
        },
    };

    let masked = value.masked_serialize().expect("masked_serialize failed");
    assert_eq!(
        masked,
        json!({
            "id": "xyz",
            "codes": [MASKED_STRING],
            "payload": { "reference": MASKED_STRING },
            "pin": MASKED_STRING
        })
    );

    let exposed = serde_json::to_value(&value).expect("to_value failed");
    assert_eq!(
        exposed,
        json!({
            "id": "xyz",
            "codes": ["code1"],
            "payload": { "reference": "ref_9" },
            "pin": "9999"
        })
    );
}
