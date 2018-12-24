// Copyright (C) 2018 Boyu Yang
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use crate::{HashAlgo, HashAlgoKernel};

pub trait Hashable<Algo>
where
    Algo: HashAlgo,
{
    fn hash(
        &self,
    ) -> Result<
        <<Algo as HashAlgo>::Kernel as HashAlgoKernel>::Output,
        <<Algo as HashAlgo>::Kernel as HashAlgoKernel>::Error,
    >;
}

#[cfg(feature = "impl-as-ref")]
mod as_ref;

#[cfg(not(feature = "impl-as-ref"))]
mod for_each;
