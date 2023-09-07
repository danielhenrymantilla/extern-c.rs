#![doc = include_str!("../README.md")]
#![no_std]
#![allow(nonstandard_style)]

mod helpers;

pub use helpers::{extern_, unsafe_extern, ZeroSizedElseWrathOfTheGඞds};

/// Convert a zero-sized closure into an `extern "C" fn(…)` pointer with the
/// "same" type signature
///
/// (but for the ABI, of course!).
///
/// ## Example
///
/// ```rust
/// use ::extern_c::*;
///
/// let f: extern "C" fn(bool) -> u8 = extern_c(|b: bool| b as u8);
/// assert_eq!(f(true), 1);
/// ```
///
/// ### Post-monomorphization error for non-zero-sized closures
///
/// See the documentation of [`ZeroSizedElseWrathOfTheGඞds`] for more info.
#[inline]
pub
fn extern_c<F, Args, R>(f: F)
  -> F::FnPtr
where
    F : helpers::Fn<extern_::C<Args, R>>,
    // for documentation purposes
    F : ZeroSizedElseWrathOfTheGඞds,
{
    F::extern_fn(f)
}

/// Same as [`extern_c()`] but for using `extern "system"` instead.
pub
fn extern_system<F, Args, R>(f: F)
  -> F::FnPtr
where
    F : helpers::Fn<extern_::system<Args, R>>,
{
    F::extern_fn(f)
}

/// Same as [`extern_c()`], but with maximum genericity over both API and
/// safety.
///
/// Instead, any <code>[extern]::abi</code> can be used.
///
/// [extern]: extern_
///
/// ## `const`-compatible
///
/// This API is `const`-compatible.
///
/// ## Example
///
/// ```rust
/// use ::extern_c::*;
///
/// let f: extern "system" fn(bool) -> u8 = fun(extern_::system, |b: bool| b as u8).into();
/// assert_eq!(f(true), 1);
///
/// let g: unsafe extern "cdecl" fn(_) -> _ = fun(unsafe_extern::cdecl, |b: bool| b as u8).into();
/// assert_eq!(unsafe { g(true) }, 1);
///
/// const VTABLE: (extern fn() -> i32, extern fn() -> i32) = (
///     fun(extern_::C, || 42).into(),
///     fun(extern_::C, || 27).into(),
/// );
///
/// let vtable: &'static (_, _) = &VTABLE;
/// assert_eq!(vtable.0() + vtable.1(), 42 + 27);
/// ```
#[doc(inline)]
pub use helpers::fun;
