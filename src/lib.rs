//! `crdts` is a library of thoroughly-tested, serializable CRDT's
//! ported from the riak_dt library to rust.
#![crate_type = "lib"]
#![deny(missing_docs)]

pub use error::{Result, Error};
pub use gcounter::GCounter;
pub use lwwreg::LWWReg;
pub use orswot::Orswot;
pub use pncounter::PNCounter;
pub use vclock::VClock;
pub use map::Map;
pub use traits::{CvRDT, CmRDT, Causal};


/// `traits` contains Trait commonly used when working with CRDT's
pub mod traits;
/// `lwwreg` contains the last-write-wins register.
pub mod lwwreg;
/// `vclock` contains the vector clock.
pub mod vclock;
/// `orswot` contains the addition-biased or-set without tombstone.
pub mod orswot;
/// `gcounter` contains the grow-only counter
pub mod gcounter;
/// `pncounter` contains the positive-negative counter
pub mod pncounter;
/// `map` contains a map CRDT which allows nesting of CRDT's
pub mod map;

/// `error` contains possible Error codes generated by CRDT operations
pub mod error;

#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate bincode;

#[cfg(test)]
#[macro_use]
extern crate quickcheck;

use bincode::{Infinite, deserialize, serialize};
use serde::Serialize;
use serde::de::DeserializeOwned;

/// Dumps this type to binary.
///
/// # Examples
///
/// ```
/// use crdts::{Orswot, to_binary, from_binary};
/// let mut a = Orswot::new();
/// a.add(1, 1);
/// let encoded = to_binary(&a);
/// let decoded = from_binary(encoded).unwrap();
/// assert_eq!(a, decoded);
/// ```
pub fn to_binary<A: Serialize>(s: &A) -> Vec<u8> {
    serialize(s, Infinite).unwrap()
}

/// Attempts to reconstruct a type from binary.
///
/// # Examples
///
/// ```
/// use crdts::{Orswot, to_binary, from_binary};
/// let mut a = Orswot::new();
/// a.add(1, 1);
/// let encoded = to_binary(&a);
/// let decoded = from_binary(encoded).unwrap();
/// assert_eq!(a, decoded);
/// ```
pub fn from_binary<A: DeserializeOwned>(
    encoded: Vec<u8>,
) -> bincode::Result<A> {
    deserialize(&encoded[..])
}
