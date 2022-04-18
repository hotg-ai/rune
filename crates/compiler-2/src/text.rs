use std::{
    borrow::{Borrow, Cow},
    fmt::{self, Display, Formatter},
    sync::Arc,
};

/// A reference-counted string.
#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct Text(Arc<str>);

impl Text {
    pub fn new(s: impl Into<Arc<str>>) -> Self { Text(s.into()) }

    pub fn as_str(&self) -> &str { &self.0 }
}

impl Display for Text {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl<T> From<T> for Text
where
    Arc<str>: From<T>,
{
    fn from(t: T) -> Self { Text::new(t) }
}

impl std::ops::Deref for Text {
    type Target = Arc<str>;

    fn deref(&self) -> &Self::Target { &self.0 }
}

impl Borrow<str> for Text {
    fn borrow(&self) -> &str { &self.0 }
}

impl<'de> serde::Deserialize<'de> for Text {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = Cow::<'de, str>::deserialize(deserializer)?;
        Ok(Text::new(s))
    }
}

impl serde::Serialize for Text {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.as_str().serialize(serializer)
    }
}
