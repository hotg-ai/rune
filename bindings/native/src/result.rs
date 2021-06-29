use safer_ffi::layout::ReprC;
#[allow(unused_imports)]
use std::ops::Not;

#[derive(Debug, Copy, Clone)]
#[repr(u8)]
pub enum Result<T, E> {
    Ok(T),
    Err(E),
}

impl<T, E> From<std::result::Result<T, E>> for Result<T, E> {
    fn from(r: std::result::Result<T, E>) -> Self {
        match r {
            Ok(value) => Result::Ok(value),
            Err(e) => Result::Err(e),
        }
    }
}

impl<T, E> From<Result<T, E>> for std::result::Result<T, E> {
    fn from(r: Result<T, E>) -> Self {
        match r {
            Result::Ok(value) => std::result::Result::Ok(value),
            Result::Err(e) => std::result::Result::Err(e),
        }
    }
}

unsafe impl<T: ReprC, E: ReprC> ReprC for Result<T, E> {
    type CLayout = result_tagged_union::Union<T::CLayout, E::CLayout>;

    fn is_valid(it: &'_ Self::CLayout) -> bool {
        let tag = unsafe { (it as *const _ as *const u8).read() };

        const OK: u8 = result_tagged_union::ResultTag::Ok as u8;
        const ERR: u8 = result_tagged_union::ResultTag::Err as u8;

        match tag {
            OK => unsafe { T::is_valid(&it.ok.value) },
            ERR => unsafe { E::is_valid(&it.err.error) },
            _ => false,
        }
    }
}

mod result_tagged_union {
    use std::mem::ManuallyDrop;
    #[safer_ffi::cfg_headers]
    use std::fmt::{self, Formatter};
    use safer_ffi::{
        derive_ReprC,
        layout::{CType, OpaqueKind, ReprC},
    };

    #[derive(Copy, Clone)]
    #[repr(C)]
    pub union Union<T: CType, E: CType> {
        pub ok: ManuallyDrop<OkVariant<T>>,
        pub err: ManuallyDrop<ErrVariant<E>>,
    }

    #[derive_ReprC]
    #[repr(u8)]
    #[derive(Copy, Clone)]
    pub enum ResultTag {
        Ok = 0,
        Err = 1,
    }

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct OkVariant<T: CType> {
        pub tag: <ResultTag as ReprC>::CLayout,
        pub value: T,
    }

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct ErrVariant<E: CType> {
        pub tag: <ResultTag as ReprC>::CLayout,
        pub error: E,
    }

    unsafe impl<T: CType, E: CType> CType for Union<T, E> {
        type OPAQUE_KIND = OpaqueKind::Concrete;

        #[safer_ffi::cfg_headers]
        fn c_short_name_fmt(fmt: &'_ mut Formatter<'_>) -> fmt::Result {
            write!(fmt, "Result_{}_{}", T::c_short_name(), E::c_short_name())
        }

        #[safer_ffi::cfg_headers]
        fn c_var_fmt(
            fmt: &'_ mut Formatter<'_>,
            var_name: &'_ str,
        ) -> fmt::Result {
            write!(fmt, "{}_t {}", Self::c_short_name(), var_name)
        }

        #[safer_ffi::cfg_headers]
        fn c_define_self(
            definer: &'_ mut dyn safer_ffi::headers::Definer,
        ) -> std::io::Result<()> {
            let typedef_name = &format!("{}_t", Self::c_short_name());

            <ResultTag as ReprC>::CLayout::c_define_self(definer)?;
            OkVariant::<T>::c_define_self(definer)?;
            ErrVariant::<E>::c_define_self(definer)?;

            definer.define_once(typedef_name, &mut |definer| {
                let w = definer.out();
                writeln!(w, "typedef union {{")?;
                writeln!(w, "  {};", OkVariant::<T>::c_var("ok"))?;
                writeln!(w, "  {};", ErrVariant::<E>::c_var("err"))?;
                writeln!(w, "}} {};", typedef_name)?;

                Ok(())
            })?;

            Ok(())
        }
    }

    unsafe impl<T: CType> CType for OkVariant<T> {
        type OPAQUE_KIND = OpaqueKind::Concrete;

        #[safer_ffi::cfg_headers]
        fn c_short_name_fmt(fmt: &'_ mut Formatter<'_>) -> fmt::Result {
            write!(fmt, "Ok_{}", T::c_short_name())
        }

        #[safer_ffi::cfg_headers]
        fn c_var_fmt(
            fmt: &'_ mut Formatter<'_>,
            var_name: &'_ str,
        ) -> fmt::Result {
            write!(fmt, "{}_t {}", Self::c_short_name(), var_name)
        }

        #[safer_ffi::cfg_headers]
        fn c_define_self(
            definer: &'_ mut dyn safer_ffi::headers::Definer,
        ) -> std::io::Result<()> {
            let typedef_name = &format!("{}_t", Self::c_short_name());
            T::c_define_self(definer)?;

            definer.define_once(typedef_name, &mut |definer| {
                let w = definer.out();
                writeln!(w, "typedef struct {{")?;
                writeln!(
                    w,
                    "  {};",
                    <ResultTag as ReprC>::CLayout::c_var("tag")
                )?;
                writeln!(w, "  {};", T::c_var("value"))?;
                writeln!(w, "}} {};", typedef_name)?;

                Ok(())
            })?;

            Ok(())
        }
    }

    unsafe impl<T: CType> CType for ErrVariant<T> {
        type OPAQUE_KIND = OpaqueKind::Concrete;

        #[safer_ffi::cfg_headers]
        fn c_short_name_fmt(fmt: &'_ mut Formatter<'_>) -> fmt::Result {
            write!(fmt, "Err_{}", T::c_short_name())
        }

        #[safer_ffi::cfg_headers]
        fn c_var_fmt(
            fmt: &'_ mut Formatter<'_>,
            var_name: &'_ str,
        ) -> fmt::Result {
            write!(fmt, "{}_t {}", Self::c_short_name(), var_name)
        }

        #[safer_ffi::cfg_headers]
        fn c_define_self(
            definer: &'_ mut dyn safer_ffi::headers::Definer,
        ) -> std::io::Result<()> {
            let typedef_name = &format!("{}_t", Self::c_short_name());
            T::c_define_self(definer)?;

            definer.define_once(typedef_name, &mut |definer| {
                let w = definer.out();
                writeln!(w, "typedef struct {{")?;
                writeln!(
                    w,
                    "  {};",
                    <ResultTag as ReprC>::CLayout::c_var("tag")
                )?;
                writeln!(w, "  {};", T::c_var("error"))?;
                writeln!(w, "}} {};", typedef_name)?;

                Ok(())
            })?;

            Ok(())
        }
    }
}
