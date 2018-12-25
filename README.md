# Rust-HashLib

[![License]](#license)
[![Travis CI]](https://travis-ci.com/yangby-cryptape/rust-hashlib)

An abstraction layer for wrapping implementations of various hash algorithms.

**This crate DO NOT implement any hash algorithms.**

My intention is to create an abstraction layer for various hash algorithms,
so as to change the specific hash algorithm without care about the specifics
of the code.

[License]: https://img.shields.io/badge/License-Apache--2.0%20OR%20MIT-blue.svg
[Travis CI]: https://img.shields.io/travis/com/yangby-cryptape/rust-hashlib.svg

## Crates

| Name                | Crate                                                        | Documentation                                     | Description              |
| ------------------- | ------------------------------------------------------------ | ------------------------------------------------- | ------------------------ |
| [`hashlib-prelude`] | [![Prelude Badge]](https://crates.io/crates/hashlib-prelude) | [![Prelude Doc]](https://docs.rs/hashlib-prelude) | An abstraction layer.    |
| [`hashlib`]         | [![Algorithms Badge]](https://crates.io/crates/hashlib)      | [![Algorithms Doc]](https://docs.rs/hashlib)      | Various hash algorithms. |

[`hashlib-prelude`]: prelude
[`hashlib`]: algorithms

[Prelude Badge]: https://img.shields.io/crates/v/hashlib-prelude.svg
[Algorithms Badge]: https://img.shields.io/crates/v/hashlib.svg

[Prelude Doc]: https://docs.rs/hashlib-prelude/badge.svg
[Algorithms Doc]: https://docs.rs/hashlib/badge.svg

## Built-in Algorithms

Since this crate doesn't implement any hash algorithms, some real hash
algorithms crates are used as backend.

| Name    | Backend Crate    |
| ------- | ---------------- |
| md5     | [`md5`]          |
| sha1    | [`sha1`]         |
| sha2    | [`sha2`]         |
| keccak  | [`tiny-keccak`]  |
| blake2b | [`blake2b_simd`] |

[`md5`]: https://crates.io/crates/md5
[`sha1`]: https://crates.io/crates/sha1
[`sha2`]: https://crates.io/crates/sha2
[`tiny-keccak`]: https://crates.io/crates/tiny-keccak
[`blake2b_simd`]: https://crates.io/crates/blake2b_simd

## License

Licensed under either of [Apache License, Version 2.0] or [MIT License], at
your option.

[Apache License, Version 2.0]: LICENSE-APACHE
[MIT License]: LICENSE-MIT
