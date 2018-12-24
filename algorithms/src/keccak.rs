// Copyright (C) 2018 Boyu Yang
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use keccak;
use nfhash;

use prelude::{HashAlgoKernel, HashError, HashOption};

#[derive(Debug, Default)]
pub struct KeccakOption;

impl HashOption for KeccakOption {}

#[derive(Debug)]
pub enum KeccakError {
    GenericError,
}

impl ::std::convert::Into<HashError> for KeccakError {
    fn into(self) -> HashError {
        HashError::GenericError
    }
}

macro_rules! impl_hash_algo_kernel_for {
    ($struct:ident, $name:expr, $size:expr, $func:ident, $output:ident) => {
        pub struct $struct {
            kernel: keccak::Keccak,
        }
        impl HashAlgoKernel for $struct {
            type Option = KeccakOption;
            type Error = KeccakError;
            type Output = nfhash::$output;

            fn name() -> &'static str {
                $name
            }

            fn digest_size() -> usize {
                $size
            }

            fn new(_opt: KeccakOption) -> Self {
                let kernel = keccak::Keccak::$func();
                Self { kernel }
            }

            fn update<T>(&mut self, data: T) -> Result<(), Self::Error>
            where
                T: ::std::convert::AsRef<[u8]>,
            {
                self.kernel.update(data.as_ref());
                Ok(())
            }

            fn finalize(self) -> Result<Self::Output, Self::Error> {
                let mut output = [0u8; $size / 8];
                self.kernel.finalize(&mut output);
                let ret: Self::Output = output.into();
                Ok(ret)
            }
        }
    };
}

impl_hash_algo_kernel_for!(Shake128, "shake-128", 128, new_shake128, H128);
impl_hash_algo_kernel_for!(Shake256, "shake-256", 256, new_shake256, H256);
impl_hash_algo_kernel_for!(Keccak224, "keccak-224", 224, new_keccak224, H224);
impl_hash_algo_kernel_for!(Keccak256, "keccak-256", 256, new_keccak256, H256);
impl_hash_algo_kernel_for!(Keccak384, "keccak-384", 384, new_keccak384, H384);
impl_hash_algo_kernel_for!(Keccak512, "keccak-512", 512, new_keccak512, H512);
impl_hash_algo_kernel_for!(Sha3_224, "sha3-224", 224, new_sha3_224, H224);
impl_hash_algo_kernel_for!(Sha3_256, "sha3-256", 256, new_sha3_256, H256);
impl_hash_algo_kernel_for!(Sha3_384, "sha3-384", 384, new_sha3_384, H384);
impl_hash_algo_kernel_for!(Sha3_512, "sha3-512", 512, new_sha3_512, H512);
