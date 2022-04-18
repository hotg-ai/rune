#[cfg(test)]
macro_rules! map {
    // map-like
    ($($k:ident : $v:expr),* $(,)?) => {
        std::iter::Iterator::collect(IntoIterator::into_iter([
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
