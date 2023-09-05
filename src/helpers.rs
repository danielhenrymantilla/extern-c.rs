/// Stable Rust polyfill for a `where mem::size_of::<Self>() == 0` clause.
///
/// The trait will *technically* be implemented for all sizes, but any choice
/// of a non-zero-sized type that ever makes it to codegen (during
/// `cargo build`, but not `cargo check`) will thus cause a compilation error.
///
/// This mechanism, called a "post-monomorphization error", ss thus worse than a
/// proper "trait not implemented" error, but better than a runtime panic.
pub
trait ZeroSizedElseWrathOfTheGඞds : Sized {
    #[doc(hidden)] /** Not part of the public API */
    const WRATH_OF_THE_GඞDS: () = assert!(::core::mem::size_of::<Self>() == 0);
}

impl<F> ZeroSizedElseWrathOfTheGඞds for F {}

impls! {
    _11, _10, _9, _8, _7, _6,
    _5, _4, _3, _2, _1, _0,
}
// where
macro_rules! impls {(
    $(
        $Hd:ident $(,
        $Rest:ident )* $(,)?
    )?
) => (
    $(impls! { $($Rest),* })?

    impl<F, $($Hd $(, $Rest)* ,)? R>
        FnExt<($($Hd, $($Rest),*)?)>
    for
        F
    where
        F : Fn($($Hd $(, $Rest)*)?) -> R,
        F : 'static + Sync + ZeroSizedElseWrathOfTheGඞds,
    {
        type CSignature = extern "C" fn($($Hd $(, $Rest)*)?) -> R;

        fn extern_c(f: Self) -> Self::CSignature {
            _ = Self::WRATH_OF_THE_GඞDS;

            // Conceptually, a `Box::leak()` or `SOME_GLOBAL_MAP.insert(f)`
            let _leaked_into_zst_symbolic_heap = ::core::mem::forget(f);

            extern "C"
            fn extern_c_thunk<F, $($Hd $(, $Rest)* ,)? R>($(
                $Hd: $Hd $(,
                $Rest: $Rest )*)?
            ) -> R
            where
                F : Fn($($Hd $(, $Rest)*)?) -> R,
            {
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

            extern_c_thunk::<F, $($Hd $(, $Rest)* ,)? R>
        }
    }
)} use impls;

pub
trait FnExt<Args> : 'static + Sync + ZeroSizedElseWrathOfTheGඞds {
    type CSignature;

    fn extern_c(this: Self) -> Self::CSignature;
}
