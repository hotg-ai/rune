macro_rules! expect {
    ($condition:expr, $($rest:tt)*) => {
        if !$condition {
            let e = anyhow::anyhow!($($rest)*);
            return $crate::Error::boxed(e);
        }
    };
    ($condition:expr) => {
        expect!($condition, "Assumption was false: {}", stringify!($condition));
    };

}
