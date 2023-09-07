/// Stable Rust polyfill for a `where mem::size_of::<Self>() == 0` clause.
///
/// The trait will *technically* be implemented for all sizes, but any choice
/// of a non-zero-sized type that ever makes it to codegen (during
/// `cargo build`, but not `cargo check`) will thus cause a compilation error.
///
/// This mechanism, called a "post-monomorphization error", ss thus worse than a
/// proper "trait not implemented" error, but better than a runtime panic.
///
/// ## Example
///
/// The following may pass `cargo check`, but shall fail with `cargo build` and
/// whatnot.
///
/// ```rust ,compile_fail
/// let captured = 42;
/// ::extern_c::extern_c(move || drop(captured));
/// ```
///
/// yielding:
///
/// ```rust
/// # const _IGNORED: &str = r#"
/// error[E0080]: evaluation of `<[closure@src/your_code.rs:5:22: 5:29]
///     as extern_c::ZeroSizedElseWrathOfTheGඞds>::WRATH_OF_THE_GඞDS` failed
///   --> extern-c/src/helpers.rs:21:35
///    |
/// 21 | const WRATH_OF_THE_GඞDS: () = assert!(::core::mem::size_of::<Self>() == 0);
///    |                               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
///    |                                the evaluated program panicked at
///    |                                'assertion failed: ::core::mem::size_of::<Self>() == 0'
///    |
/// # "#;
/// ```
pub
trait ZeroSizedElseWrathOfTheGඞds : Sized {
    #[doc(hidden)] /** Not part of the public API */
    const WRATH_OF_THE_GඞDS: () = assert!(::core::mem::size_of::<Self>() == 0);
}

impl<F> ZeroSizedElseWrathOfTheGඞds for F {}

/// The helper trait powering the genericity of [`crate::extern_c()`] _& co._
pub
trait Fn<Signature> : 'static + Sync + ZeroSizedElseWrathOfTheGඞds {
    type Output;

    type FnPtr;

    fn extern_fn(f: Self) -> Self::FnPtr;
}

// Generic _struct_ trick in order to be `const`-compatible.
#[doc(hidden)]
#[allow(nonstandard_style)]
pub
struct fun<Extern, F>(pub Extern, pub F);

supported_abis! {
    "Rust" => Rust,
    "C" => C,
    "system" => system,
    "cdecl" => cdecl,
}
// where
macro_rules! supported_abis {(
    $($abi:tt => $Abi:ident),* $(,)?
) => (
    pub mod extern_ {
        use super::*;

        $(
            #[doc = concat!("`extern \"", $abi, "\" fn()`")]
            pub type $Abi<Args, R> = ඞ::$Abi<Args, R>;
        )*

        #[doc(hidden)]
        pub use self::ඞ::{$($Abi::*),*};
        pub(in crate) mod ඞ {
            $(
                #[doc = concat!("`extern \"", $abi, "\" fn()`")]
                pub enum $Abi<Args, R> {
                    $Abi,

                    #[doc(hidden)]
                    ඞ(::never_say_never::Never, ::core::marker::PhantomData<fn(Args) -> R>),
                }
            )*
        }

        $(
            impls! {
                $abi => $Abi,
                _11, _10, _9, _8, _7, _6, _5, _4, _3, _2, _1, _0,
            }
        )*
    }

    pub mod unsafe_extern {
        use super::*;

        $(
            #[doc = concat!("`unsafe extern \"", $abi, "\" fn()`")]
            pub type $Abi<Args, R> = ඞ::$Abi<Args, R>;
        )*

        #[doc(hidden)]
        pub use self::ඞ::{$($Abi::*),*};

        pub(in crate) mod ඞ {
            $(
                #[doc = concat!("`unsafe extern \"", $abi, "\" fn()`")]
                pub enum $Abi<Args, R> {
                    $Abi,

                    #[doc(hidden)]
                    ඞ(::never_say_never::Never, ::core::marker::PhantomData<fn(Args) -> R>),
                }
            )*
        }

        $(
            impls! {
                $abi => $Abi,
                _11, _10, _9, _8, _7, _6, _5, _4, _3, _2, _1, _0,
            }
        )*
    }
)} use supported_abis;
// where
macro_rules! impls {(
    $abi:tt => $Abi:ident, $(
    $Hd:ident $(,
    $Rest:ident )* $(,)? )?
) => (
    $(impls! {
        $abi => $Abi,
        $($Rest),*
    })?

    impl<F, $($Hd $(, $Rest)* ,)? R> fun<$Abi<($($Hd, $($Rest),*)?), R>, F>
    where
        F : ::core::ops::Fn($($Hd $(, $Rest)*)?) -> R,
        F : 'static + Sync + ZeroSizedElseWrathOfTheGඞds,
    {
        #[inline]
        pub
        const
        fn into(self)
          -> extern $abi fn($($Hd $(, $Rest)*)?) -> R
        {
            // Conceptually, a `Box::leak()` or `SOME_GLOBAL_MAP.insert(f)`
            let _leaked_into_zst_symbolic_heap = ::core::mem::forget(self);

            extern $abi
            fn extern_thunk<F, $($Hd $(, $Rest)* ,)? R>($(
                $Hd: $Hd $(,
                $Rest: $Rest )*)?
            ) -> R
            where
                F : ::core::ops::Fn($($Hd $(, $Rest)*)?) -> R,
            {
                let _compile_time_assert_zero_sized = F::WRATH_OF_THE_GඞDS;
                let out_of_thin_air: [F; 0] = [];
                (unsafe {
                    // SAFETY:
                    //   - conceptually, this is `SOME_GLOBAL_MAP.get(f)`
                    //   - in practice, since this is a ZST which we have
                    //     `mem::forget`-ten, we can just produce any kind of
                    //      well-aligned and non-deallocated pointer and say it
                    //      is a valid reference.
                    //      The "end" of an empty array properly fits both
                    //      requirements.
                    &*out_of_thin_air.as_ptr()
                })(
                    // now Just Call It™
                    $($Hd $(, $Rest)*)?
                )
            }

            extern_thunk::<F, $($Hd $(, $Rest)* ,)? R>
        }
    }

    impl<F, $($Hd $(, $Rest)* ,)? R>
        Fn<$Abi<($($Hd, $($Rest),*)?), R>>
    for
        F
    where
        F : ::core::ops::Fn($($Hd $(, $Rest)*)?) -> R,
        F : 'static + Sync + ZeroSizedElseWrathOfTheGඞds,
    {
        type Output = R;

        type FnPtr = extern $abi fn($($Hd $(, $Rest)*)?) -> R;

        #[inline]
        fn extern_fn(f: Self) -> Self::FnPtr {
            fun::<$Abi<($($Hd, $($Rest),*)?), R>, _>($Abi, f)
                .into()
        }
    }
)} use impls;
