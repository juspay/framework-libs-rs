#![cfg_attr(docsrs, feature(doc_cfg))]
#![doc(test(attr(deny(warnings))))]

//! Personal Identifiable Information protection. Wrapper types and traits for secret management which help ensure they aren't accidentally copied, logged, or otherwise exposed (as much as possible), and also ensure secrets are securely wiped from memory when dropped.
//! Secret-keeping library inspired by secrecy.

#![doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR" ), "/", "README.md"))]

#[cfg(feature = "alloc")]
extern crate alloc;

mod abs;
#[cfg(feature = "alloc")]
mod boxed;
#[cfg(feature = "bytes")]
mod bytes;
#[cfg(feature = "cassandra")]
mod cassandra;
#[cfg(feature = "diesel")]
mod diesel;
mod maskable;
mod secret;
#[cfg(feature = "serde")]
mod serde;
mod strategy;
#[cfg(feature = "alloc")]
mod string;
mod strong_secret;
#[cfg(feature = "alloc")]
mod vec;

pub use zeroize::{self, DefaultIsZeroes, Zeroize as ZeroizableSecret};

#[cfg(feature = "bytes")]
pub use self::bytes::SecretBytesMut;
pub use self::{
    abs::{ExposeInterface, ExposeOptionInterface, PeekInterface, SwitchStrategy},
    maskable::{Mask, Maskable},
    secret::Secret,
    strategy::{Strategy, WithType, WithoutType},
    strong_secret::StrongSecret,
};
#[cfg(feature = "serde")]
pub use self::{
    secret::JsonMaskStrategy,
    serde::{Deserialize, ErasedMaskSerialize, SerializableSecret, Serialize, masked_serialize},
};
