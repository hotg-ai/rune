/// Declare a type that implements [`salsa::InternKey`] so it can be used with
/// Salsa's interning machinery.
macro_rules! intern_id {
    (
        $(
            $(#[$meta:meta] *)* $vis:vis struct $name:ident($intern:ty)
        );*
        $(;)?
    ) => {
        $(
            #[derive(
                Debug,
                Copy,
                Clone,
                PartialEq,
                Eq,
                PartialOrd,
                Ord,
                Hash,
                serde::Serialize,
                serde::Deserialize,
            )]
            #[repr(transparent)]
            #[serde(from = "u32", into = "u32")]
            $vis struct $name($intern);

            impl salsa::InternKey for $name {
                fn from_intern_id(v: $intern) -> Self {  v.into() }

                fn as_intern_id(&self) -> $intern { (*self).into() }
            }

            impl std::fmt::Display for $name {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    self.0.fmt(f)
                }
            }

            impl From<u32> for $name {
                fn from(v: u32) -> Self { $name(v.into()) }
            }

            impl From<$name> for u32 {
                fn from(v: $name) -> Self { v.0.into() }
            }

            impl From<$intern> for $name {
                fn from(v: $intern) -> Self { $name(v) }
            }

            impl From<$name> for $intern {
                fn from(v: $name) -> $intern { v.0 }
            }

            impl From<$name> for crate::diagnostics::Id {
                fn from(v: $name) -> crate::diagnostics::Id {
                    crate::diagnostics::Id::$name(v)
                }
            }
        )*
    };
}
