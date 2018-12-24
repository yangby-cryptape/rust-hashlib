// Copyright (C) 2018 Boyu Yang
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

/// Generic options for hash algorithm.
pub trait HashOption: ::std::default::Default {}

/// Unify all hash errors.
pub enum HashError {
    GenericError,
}

/// Kernel of hash algorithms.
pub trait HashAlgoKernel {
    type Option: HashOption;

    type Error: ::std::convert::Into<HashError> + ::std::fmt::Debug;

    /// The message digest of hash algorithms.
    type Output: ::std::cmp::Eq
        + ::std::convert::AsRef<[u8]>
        + ::std::fmt::LowerHex
        + ::std::fmt::UpperHex;

    /// The name of hash algorithm.
    fn name() -> &'static str;

    /// The size of the output in bytes.
    fn digest_size() -> usize;

    /// Create a new hash algorithm.
    fn new(opt: Self::Option) -> Self;

    /// Append bytes to compute the digest.
    fn update<T>(&mut self, data: T) -> Result<(), Self::Error>
    where
        T: ::std::convert::AsRef<[u8]>;

    /// Finalize and return the digest.
    fn finalize(self) -> Result<Self::Output, Self::Error>;
}

/// A hash algorithm.
pub trait HashAlgo: ::std::default::Default {
    type Kernel: HashAlgoKernel;

    fn setup() -> <<Self as HashAlgo>::Kernel as HashAlgoKernel>::Option;

    /// Input the complete bytes to compute the digest.
    fn hash(
        input: &[u8],
    ) -> Result<
        <<Self as HashAlgo>::Kernel as HashAlgoKernel>::Output,
        <<Self as HashAlgo>::Kernel as HashAlgoKernel>::Error,
    > {
        let opt = Self::setup();
        let mut algo = Self::Kernel::new(opt);
        algo.update(input)?;
        algo.finalize()
    }
}
