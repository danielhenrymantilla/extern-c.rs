#![doc = include_str!("../README.md")]
#![no_std]

use helpers::ZeroSizedElseWrathOfTheGඞds;

/// Convert a zero-sized closure into an `extern "C" fn(…)` pointer with the
/// "same" type signature
///
/// (but for the ABI, of course!).
///
/// ## Example
///
/// ```rust
/// use ::extern_c::extern_c;
///
/// let f: extern "C" fn(bool) -> u8 = extern_c(|b: bool| b as u8);
/// assert_eq!(f(true), 1);
/// ```
///
/// ### Post-monomorphization error for non-zero-sized closures
///
/// See the documentation of [`ZeroSizedElseWrathOfTheGඞds`] for more info.
pub
fn extern_c<F, Args>(f: F)
  -> F::CSignature
where
    F : helpers::FnExt<Args>,
    // for documentation purposes
    F : ZeroSizedElseWrathOfTheGඞds,
{
    F::extern_c(f)
}

pub mod helpers;
