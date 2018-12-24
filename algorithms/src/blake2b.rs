// Copyright (C) 2018 Boyu Yang
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use blake2b;
use nfhash;

use prelude::{HashAlgoKernel, HashError, HashOption};

#[derive(Debug)]
pub struct Blake2bOption {
    pub key: &'static [u8],
    pub salt: &'static [u8],
    pub personalization: &'static [u8],
    pub fanout: u8,
    pub max_depth: u8,
    pub max_leaf_length: u32,
    pub node_offset: u64,
    pub node_depth: u8,
    pub inner_hash_length: usize,
    pub last_node: bool,
}

impl HashOption for Blake2bOption {}

impl ::std::default::Default for Blake2bOption {
    fn default() -> Self {
        Blake2bOption {
            key: b"",
            salt: b"",
            personalization: b"",
            fanout: 1,
            max_depth: 1,
            max_leaf_length: 0,
            node_offset: 0,
            node_depth: 0,
            inner_hash_length: 0,
            last_node: false,
        }
    }
}

impl ::std::convert::Into<blake2b::Params> for Blake2bOption {
    fn into(self) -> blake2b::Params {
        let Blake2bOption {
            key,
            salt,
            personalization,
            fanout,
            max_depth,
            max_leaf_length,
            node_offset,
            node_depth,
            inner_hash_length,
            last_node,
        } = self;
        let mut params = blake2b::Params::new();
        params
            .key(key)
            .salt(salt)
            .personal(personalization)
            .fanout(fanout)
            .max_depth(max_depth)
            .max_leaf_length(max_leaf_length)
            .node_offset(node_offset)
            .node_depth(node_depth)
            .inner_hash_length(inner_hash_length)
            .last_node(last_node);
        params
    }
}

#[derive(Debug)]
pub enum Blake2bError {
    GenericError,
}

impl ::std::convert::Into<HashError> for Blake2bError {
    fn into(self) -> HashError {
        HashError::GenericError
    }
}

macro_rules! impl_hash_algo_kernel_for {
    ($struct:ident, $name:expr, $size:expr, $length:expr, $output:ident) => {
        pub struct $struct {
            kernel: blake2b::State,
        }

        impl HashAlgoKernel for $struct {
            type Option = Blake2bOption;
            type Error = Blake2bError;
            type Output = nfhash::$output;

            fn name() -> &'static str {
                $name
            }

            fn digest_size() -> usize {
                $size
            }

            fn new(opt: Blake2bOption) -> Self {
                let mut params: blake2b::Params = opt.into();
                let kernel = params.hash_length($length).to_state();
                Self { kernel }
            }

            fn update<T>(&mut self, data: T) -> Result<(), Self::Error>
            where
                T: ::std::convert::AsRef<[u8]>,
            {
                self.kernel.update(data.as_ref());
                Ok(())
            }

            fn finalize(mut self) -> Result<Self::Output, Self::Error> {
                let hash = self.kernel.finalize();
                let ret = Self::Output::from_slice(hash.as_bytes()).unwrap();
                Ok(ret)
            }
        }
    };
}

impl_hash_algo_kernel_for!(Blake2b224, "blake2b-224", 224, 28, H224);
impl_hash_algo_kernel_for!(Blake2b256, "blake2b-256", 256, 32, H256);
impl_hash_algo_kernel_for!(Blake2b384, "blake2b-384", 384, 48, H384);
impl_hash_algo_kernel_for!(Blake2b512, "blake2b-512", 512, 64, H512);
