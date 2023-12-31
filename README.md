# `::extern-c`

Convert a zero-sized closure into an `extern "C" fn(…)` pointer with the
same[^abi] type signature.

[^abi]: but for the ABI, of course!

[![Repository](https://img.shields.io/badge/repository-GitHub-brightgreen.svg)](
https://github.com/danielhenrymantilla/extern-c.rs)
[![Latest version](https://img.shields.io/crates/v/extern-c.svg)](
https://crates.io/crates/extern-c)
[![Documentation](https://docs.rs/extern-c/badge.svg)](
https://docs.rs/extern-c)
[![MSRV](https://img.shields.io/badge/MSRV-1.65.0-white)](
https://gist.github.com/danielhenrymantilla/8e5b721b3929084562f8f65668920c33)
![no_std compatible](https://img.shields.io/badge/no__std-compatible-success.svg)
[![License](https://img.shields.io/crates/l/extern-c.svg)](
https://github.com/danielhenrymantilla/extern-c.rs/blob/master/LICENSE-ZLIB)
[![CI](https://github.com/danielhenrymantilla/extern-c.rs/workflows/CI/badge.svg)](
https://github.com/danielhenrymantilla/extern-c.rs/actions)

<!-- Templated by `cargo-generate` using https://github.com/danielhenrymantilla/proc-macro-template -->

## Example

```rust
use ::extern_c::extern_c;

let f: extern "C" fn(bool) -> u8 = extern_c(|b: bool| b as u8);
assert_eq!(f(true), 1);
```
