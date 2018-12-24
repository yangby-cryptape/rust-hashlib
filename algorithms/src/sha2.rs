// Copyright (C) 2018 Boyu Yang
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use nfhash;
use sha2;

use prelude::{HashAlgoKernel, HashError, HashOption};
use sha2::Digest;

#[derive(Debug, Default)]
pub struct Sha2Option;

impl HashOption for Sha2Option {}

#[derive(Debug)]
pub enum Sha2Error {
    GenericError,
}

impl ::std::convert::Into<HashError> for Sha2Error {
    fn into(self) -> HashError {
        HashError::GenericError
    }
}

macro_rules! impl_hash_algo_kernel_for {
    ($struct:ident, $name:expr, $size:expr, $inner:ident, $output:ident) => {
        pub struct $struct {
            kernel: sha2::$inner,
        }
        impl HashAlgoKernel for $struct {
            type Option = Sha2Option;
            type Error = Sha2Error;
            type Output = nfhash::$output;

            fn name() -> &'static str {
                $name
            }

            fn digest_size() -> usize {
                $size
            }

            fn new(_opt: Sha2Option) -> Self {
                let kernel = sha2::$inner::new();
                Self { kernel }
            }

            fn update<T>(&mut self, data: T) -> Result<(), Self::Error>
            where
                T: ::std::convert::AsRef<[u8]>,
            {
                self.kernel.input(data);
                Ok(())
            }

            fn finalize(self) -> Result<Self::Output, Self::Error> {
                let digest = self.kernel.result();
                Self::Output::from_slice(digest.as_slice()).map_err(|_| Sha2Error::GenericError)
            }
        }
    };
}

impl_hash_algo_kernel_for!(Sha224, "sha-224", 224, Sha224, H224);
impl_hash_algo_kernel_for!(Sha256, "sha-256", 256, Sha256, H256);
impl_hash_algo_kernel_for!(Sha384, "sha-384", 384, Sha384, H384);
impl_hash_algo_kernel_for!(Sha512, "sha-512", 512, Sha512, H512);
impl_hash_algo_kernel_for!(Sha512_224, "sha-512/224", 224, Sha512Trunc224, H224);
impl_hash_algo_kernel_for!(Sha512_256, "sha-512/256", 256, Sha512Trunc256, H256);
