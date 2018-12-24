// Copyright (C) 2018 Boyu Yang
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use md5;
use nfhash;

use prelude::{HashAlgoKernel, HashError, HashOption};

#[derive(Debug, Default)]
pub struct Md5Option;

impl HashOption for Md5Option {}

#[derive(Debug)]
pub enum Md5Error {
    GenericError,
}

impl ::std::convert::Into<HashError> for Md5Error {
    fn into(self) -> HashError {
        HashError::GenericError
    }
}

pub struct Md5 {
    kernel: md5::Context,
}

impl HashAlgoKernel for Md5 {
    type Option = Md5Option;
    type Error = Md5Error;
    type Output = nfhash::H128;

    fn name() -> &'static str {
        "md5"
    }

    fn digest_size() -> usize {
        128
    }

    fn new(_opt: Md5Option) -> Self {
        Self {
            kernel: md5::Context::new(),
        }
    }

    fn update<T>(&mut self, data: T) -> Result<(), Self::Error>
    where
        T: ::std::convert::AsRef<[u8]>,
    {
        self.kernel.consume(data);
        Ok(())
    }

    fn finalize(self) -> Result<Self::Output, Self::Error> {
        let digest = self.kernel.compute();
        let inner: [u8; 16] = digest.into();
        let ret: Self::Output = inner.into();
        Ok(ret)
    }
}
