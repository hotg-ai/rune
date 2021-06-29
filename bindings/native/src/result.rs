/// Create an opaque `Result<T, E>` with methods for inspecting the value or
/// extracting it.
///
/// Note: This is a workaround because when we used the "true" representation
/// for a `Result<T, E>` dart choked on the union.
///
/// One day we might be able to reuse [this][proper-impl] (or push it upstream).
///
/// [proper-impl]: https://github.com/hotg-ai/rune/blob/db0ba625551e30f4a46fb7d6b3765e7bd17a6937/bindings/native/src/result.rs
macro_rules! decl_result_type {
    ($( type $name:ident = Result<$ok:ident, $err:ident>; )*) => {
        $(
            paste::paste! {
                #[safer_ffi::derive_ReprC]
                #[ReprC::opaque]
                pub struct $name {
                    inner: Result<$ok, $err>,
                }

                impl From<Result<$ok, $err>> for $name {
                    fn from(result: Result<$ok, $err>) -> Self {
                        $name {
                            inner: result,
                        }
                    }
                }

                impl From<$name> for Result<$ok, $err> {
                    fn from(result: $name) -> Result<$ok, $err> {
                        result.inner
                    }
                }

                impl $name {
                    pub fn into_std(self) -> Result<$ok, $err> {
                        self.into()
                    }

                    pub fn from_std(result: Result<$ok, $err>) -> Self {
                        result.into()
                    }
                }

                #[allow(bad_style, unused_imports)]
                const _: () = {
                    use std::ops::Not;

                    #[doc = "Create a new `" $name "` containing the success value."]
                    #[safer_ffi::ffi_export]
                    pub fn [< rune_result_ $name _new_ok >](value: $ok) -> safer_ffi::boxed::Box<$name> {
                        safer_ffi::boxed::Box::new($name { inner: Ok(value) })
                    }

                    #[doc = "Create a new `" $name "` containing the error value."]
                    #[safer_ffi::ffi_export]
                    pub fn [< rune_result_ $name _new_err >](error: $err) -> safer_ffi::boxed::Box<$name> {
                        safer_ffi::boxed::Box::new($name { inner: Err(error) })
                    }

                    #[doc = "Check if the result contains a `" $ok "`."]
                    #[safer_ffi::ffi_export]
                    pub fn [< rune_result_ $name _is_ok >](result: &$name) -> bool {
                        result.inner.is_ok()
                    }

                    #[doc = "Check if the result contains a `" $err "`."]
                    #[safer_ffi::ffi_export]
                    pub fn [< rune_result_ $name _is_err >](result: &$name) -> bool {
                        result.inner.is_err()
                    }

                    #[doc = "Free the `" $name "` after you are done with it."]
                    #[safer_ffi::ffi_export]
                    pub fn [< rune_result_ $name _free >](result: safer_ffi::boxed::Box<$name>) {
                        drop(result);
                    }

                    #[doc = "Get a reference to the `" $ok "` in this `" $name "`, or `null` if not present."]
                    #[safer_ffi::ffi_export]
                    pub fn [< rune_result_ $name _get_ok >](result: &$name) -> *const $ok {
                        match &result.inner {
                            Ok(value) => value as *const _,
                            Err(_) => std::ptr::null(),
                        }
                    }

                    #[doc = "Get a reference to the `" $err "` in this `" $name "`, or `null` if not present."]
                    #[safer_ffi::ffi_export]
                    pub fn [< rune_result_ $name _get_err >](result: &$name) -> *const $err {
                        match &result.inner {
                            Ok(_) => std::ptr::null(),
                            Err(e) => e as *const _,
                        }
                    }

                    #[doc = "Extract the `" $ok "`, freeing the `" $name "` and crashing if it actually contains a `" $err "`."]
                    #[safer_ffi::ffi_export]
                    pub fn [< rune_result_ $name _take_ok >](result: safer_ffi::boxed::Box<$name>) -> $ok {
                        let result: std::boxed::Box<_> = result.into();

                        match result.inner {
                            Ok(value) => value,
                            Err(_) => unreachable!(),
                        }
                    }

                    #[doc = "Extract the `" $err "`, freeing the `" $name "` and crashing if it actually contains a `" $ok "`."]
                    #[safer_ffi::ffi_export]
                    pub fn [< rune_result_ $name _take_err >](result: safer_ffi::boxed::Box<$name>) -> $err {
                        let result: std::boxed::Box<_> = result.into();

                        match result.inner {
                            Ok(_) => unreachable!(),
                            Err(e) => e,
                        }
                    }

                    // #[doc = "Extract the `" $err "`, freeing the `" $name "` and crashing if it actually contains a `" $ok "`."]
                    // pub fn [< rune_result_ $name _take_err >](result: safer_ffi::boxed::Box<$name>) -> $err {
                    //     let result: std::boxed::Box<_> = result.into();

                    //     match result.inner {
                    //         Ok(_) => unreachable!(),
                    //         Err(e) => e,
                    //     }
                    // }
                };
            }
        )*
    };
}
