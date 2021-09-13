#[cfg(test)]
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

#[cfg(test)]
macro_rules! ty {
        ($type:ident [$($dim:expr),*]) => {
            crate::yaml::Type {
                name: String::from(stringify!($type)),
                dimensions: vec![ $($dim),*],
            }
        };
        ($type:ident) => {
            crate::yaml::Type {
                name: String::from(stringify!($type)),
                dimensions: vec![],
            }
        }
    }
