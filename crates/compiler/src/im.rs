//! Immutable collections.

use std::{
    borrow::{Borrow, Cow},
    collections::BTreeMap,
    fmt::{self, Display, Formatter},
    sync::Arc,
};

/// A reference-counted string.
#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct Text(Arc<str>);

impl Text {
    pub fn new(s: impl Into<Arc<str>>) -> Self {
        Text(s.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl Display for Text {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<Arc<str>> for Text {
    fn from(s: Arc<str>) -> Self {
        Text(s)
    }
}

impl From<String> for Text {
    fn from(s: String) -> Self {
        Text(s.into())
    }
}

impl From<&'_ str> for Text {
    fn from(s: &'_ str) -> Self {
        Text(s.into())
    }
}

impl From<&'_ String> for Text {
    fn from(s: &'_ String) -> Self {
        Text(s.as_str().into())
    }
}

impl std::ops::Deref for Text {
    type Target = Arc<str>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Borrow<str> for Text {
    fn borrow(&self) -> &str {
        &self.0
    }
}

impl<T> PartialEq<T> for Text
where
    T: PartialEq<str>,
{
    fn eq(&self, other: &T) -> bool {
        other == self.as_str()
    }
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

#[derive(
    Debug,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
)]
#[repr(transparent)]
#[serde(from = "Vec<T>", into = "Vec<T>")]
#[serde(bound(serialize = "T: Clone + serde::Serialize"))]
pub struct Vector<T>(Arc<[T]>);

impl<T> std::ops::Deref for Vector<T> {
    type Target = Arc<[T]>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> Clone for Vector<T> {
    fn clone(&self) -> Self {
        Vector(Arc::clone(&self.0))
    }
}

impl<T> Default for Vector<T> {
    fn default() -> Self {
        Vector::from(Vec::new())
    }
}

impl<T> From<Vec<T>> for Vector<T> {
    fn from(v: Vec<T>) -> Self {
        Vector(v.into_boxed_slice().into())
    }
}

impl<T: Clone> From<&'_ [T]> for Vector<T> {
    fn from(v: &'_ [T]) -> Self {
        Vector(v.into())
    }
}

impl<T: Clone> From<Vector<T>> for Vec<T> {
    fn from(v: Vector<T>) -> Self {
        v.0.to_vec()
    }
}

impl<A> FromIterator<A> for Vector<A> {
    fn from_iter<T: IntoIterator<Item = A>>(iter: T) -> Self {
        Vector(iter.into_iter().collect())
    }
}

impl<A> AsRef<[A]> for Vector<A> {
    fn as_ref(&self) -> &[A] {
        self.0.as_ref()
    }
}

#[derive(
    Debug,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
)]
#[repr(transparent)]
#[serde(from = "BTreeMap<K, V>", into = "BTreeMap<K, V>")]
#[serde(bound(
    serialize = "K: Clone + serde::Serialize + Ord, V: Clone + serde::Serialize + Ord",
))]
pub struct OrdMap<K: Ord, V>(Arc<BTreeMap<K, V>>);

impl<K: Ord, V> Clone for OrdMap<K, V> {
    fn clone(&self) -> Self {
        OrdMap(Arc::clone(&self.0))
    }
}

impl<K: Ord, V> Default for OrdMap<K, V> {
    fn default() -> Self {
        OrdMap::from(BTreeMap::new())
    }
}

impl<K: Ord, V> From<BTreeMap<K, V>> for OrdMap<K, V> {
    fn from(v: BTreeMap<K, V>) -> Self {
        OrdMap(Arc::new(v))
    }
}

impl<K: Ord + Clone, V: Clone> From<OrdMap<K, V>> for BTreeMap<K, V> {
    fn from(v: OrdMap<K, V>) -> Self {
        BTreeMap::clone(&v.0)
    }
}

impl<'a, K: Ord, V> IntoIterator for &'a OrdMap<K, V> {
    type Item = (&'a K, &'a V);

    type IntoIter =
        <&'a std::collections::BTreeMap<K, V> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}

impl<K: Ord, V> FromIterator<(K, V)> for OrdMap<K, V> {
    fn from_iter<T: IntoIterator<Item = (K, V)>>(iter: T) -> Self {
        let map: BTreeMap<K, V> = iter.into_iter().collect();
        map.into()
    }
}
