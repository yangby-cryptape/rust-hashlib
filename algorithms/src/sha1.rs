// Copyright (C) 2018 Boyu Yang
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use nfhash;
use sha1;

use prelude::{HashAlgoKernel, HashError, HashOption};

#[derive(Debug, Default)]
pub struct Sha1Option;

impl HashOption for Sha1Option {}

#[derive(Debug)]
pub enum Sha1Error {
    GenericError,
}

impl ::std::convert::Into<HashError> for Sha1Error {
    fn into(self) -> HashError {
        HashError::GenericError
    }
}

pub struct Sha1 {
    kernel: sha1::Sha1,
}

impl HashAlgoKernel for Sha1 {
    type Option = Sha1Option;
    type Error = Sha1Error;
    type Output = nfhash::H160;

    fn name() -> &'static str {
        "sha1"
    }

    fn digest_size() -> usize {
        160
    }

    fn new(_opt: Sha1Option) -> Self {
        Self {
            kernel: sha1::Sha1::new(),
        }
    }

    fn update<T>(&mut self, data: T) -> Result<(), Self::Error>
    where
        T: ::std::convert::AsRef<[u8]>,
    {
        self.kernel.update(data.as_ref());
        Ok(())
    }

    fn finalize(self) -> Result<Self::Output, Self::Error> {
        let digest = self.kernel.digest();
        let ret: Self::Output = digest.bytes().into();
        Ok(ret)
    }
}
