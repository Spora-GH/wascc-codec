#![doc(html_logo_url = "https://avatars2.githubusercontent.com/u/52050279?s=200&v=4")]

//! # wascc-codec
//!
//! This library provides the core set of types and associated functions used for
//! the common set of functionality between the host runtime and capability providers. Data
//! types and models that need to be shared between guest and hosts should be _generated_
//! and should not show up in this crate.

/// The version of the codec as seen on crates.io
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
/// The string used for the originator of messages dispatched by the host runtime
pub const SYSTEM_ACTOR: &str = "system";

#[macro_use]
extern crate serde_derive;

extern crate rmp_serde as rmps;
use rmps::{Deserializer, Serializer};
use serde::{Deserialize, Serialize};
use std::io::Cursor;

/// The standard function for serializing codec structs into a format that can be
/// used for message exchange between actor and host. Use of any other function to
/// serialize could result in breaking incompatibilities.
pub fn serialize<T>(
    item: T,
) -> ::std::result::Result<Vec<u8>, Box<dyn std::error::Error + Send + Sync>>
where
    T: Serialize,
{
    let mut buf = Vec::new();
    item.serialize(&mut Serializer::new(&mut buf).with_struct_map())?;
    Ok(buf)
}

/// The standard function for de-serializing codec structs from a format suitable
/// for message exchange between actor and host. Use of any other function to
/// deserialize could result in breaking incompatibilities.
pub fn deserialize<'de, T: Deserialize<'de>>(
    buf: &[u8],
) -> ::std::result::Result<T, Box<dyn std::error::Error + Send + Sync>> {
    let mut de = Deserializer::new(Cursor::new(buf));
    match Deserialize::deserialize(&mut de) {
        Ok(t) => Ok(t),
        Err(e) => Err(format!("Failed to de-serialize: {}", e).into()),
    }
}

pub mod capabilities;
pub mod core;
