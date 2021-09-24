macro_rules! map {
    // map-like
    ($($k:ident : $v:expr),* $(,)?) => {
        std::iter::Iterator::collect(std::array::IntoIter::new([
            $(
                (String::from(stringify!($k)), $v)
            ),*
        ]))
    };
    // set-like
    ($($v:expr),* $(,)?) => {
        std::iter::Iterator::collect(std::array::IntoIter::new([$($v,)*]))
    };
}

macro_rules! ty {
        ($type:ident [$($dim:expr),*]) => {
            crate::parse::Type {
                name: String::from(stringify!($type)),
                dimensions: vec![ $($dim),*],
            }
        };
        ($type:ident) => {
            crate::parse::Type {
                name: String::from(stringify!($type)),
                dimensions: vec![],
            }
        }
    }
