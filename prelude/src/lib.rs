// Copyright (C) 2018 Boyu Yang
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! An abstraction layer for wrapping implementations of various hash algorithms.

mod algorithm;
mod hashable;

pub use self::algorithm::{HashAlgo, HashAlgoKernel, HashError, HashOption};
pub use self::hashable::Hashable;
