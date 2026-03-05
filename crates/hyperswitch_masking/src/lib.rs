#![cfg_attr(docsrs, feature(doc_cfg))]
#![doc(test(attr(deny(warnings))))]

//! Personal Identifiable Information protection. Wrapper types and traits for secret management which help ensure they aren't accidentally copied, logged, or otherwise exposed (as much as possible), and also ensure secrets are securely wiped from memory when dropped.
//! Secret-keeping library inspired by secrecy.

#![doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR" ), "/", "README.md"))]

pub use zeroize::{self, DefaultIsZeroes, Zeroize as ZeroizableSecret};

mod strategy;

pub use strategy::{Strategy, WithType, WithoutType};
mod abs;
pub use abs::{ExposeInterface, ExposeOptionInterface, PeekInterface, SwitchStrategy};

mod secret;
mod strong_secret;
#[cfg(feature = "serde")]
pub use secret::JsonMaskStrategy;
pub use secret::Secret;
pub use strong_secret::StrongSecret;

#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg(feature = "alloc")]
mod boxed;

#[cfg(feature = "bytes")]
mod bytes;
#[cfg(feature = "bytes")]
pub use self::bytes::SecretBytesMut;

#[cfg(feature = "alloc")]
mod string;

#[cfg(feature = "alloc")]
mod vec;

#[cfg(feature = "serde")]
mod serde;
#[cfg(feature = "serde")]
pub use crate::serde::{
    Deserialize, ErasedMaskSerialize, SerializableSecret, Serialize, masked_serialize,
};

/// This module should be included with asterisk.
///
/// `use masking::prelude::*;`
pub mod prelude {
    pub use super::{ExposeInterface, ExposeOptionInterface, PeekInterface};
}

#[cfg(feature = "diesel")]
mod diesel;

#[cfg(feature = "cassandra")]
mod cassandra;

pub mod maskable;

pub use maskable::*;
