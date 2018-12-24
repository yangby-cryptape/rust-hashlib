// Copyright (C) 2018 Boyu Yang
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use super::{HashAlgo, HashAlgoKernel, Hashable};

/* note:
 *      upstream crates may add new impl of trait `std::*::*<T>` for type `&'static [u8]`
 *      in future versions
 * T: Into / AsRef / ...
 */
#[cfg(feature = "impl-as-ref")]
impl<Algo, T> Hashable<Algo> for T
where
    Algo: HashAlgo,
    T: std::convert::AsRef<[u8]>,
{
    fn hash(
        &self,
    ) -> Result<
        <<Algo as HashAlgo>::Kernel as HashAlgoKernel>::Output,
        <<Algo as HashAlgo>::Kernel as HashAlgoKernel>::Error,
    > {
        Algo::hash(self.as_ref())
    }
}
