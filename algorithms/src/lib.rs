// Copyright (C) 2018 Boyu Yang
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Provide various hash algorithms.

pub use prelude;

#[cfg(feature = "support-blake2b")]
pub mod blake2b;
#[cfg(feature = "support-keccak")]
pub mod keccak;
#[cfg(feature = "support-md5")]
pub mod md5;
#[cfg(feature = "support-sha1")]
pub mod sha1;
#[cfg(feature = "support-sha2")]
pub mod sha2;
