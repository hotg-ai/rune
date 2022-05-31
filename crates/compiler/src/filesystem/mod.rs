#[cfg(feature = "builtins")]
mod builtin;

pub use self::builtin::StandardFileSystem;

use std::{
    collections::HashMap,
    fmt::{self, Debug, Display, Formatter},
    str::FromStr,
    sync::{Arc, RwLock},
};

use serde::Deserialize;
use uriparse::{Scheme, URI};

use crate::{im::Vector, Text};

/// An abstract filesystem.
pub trait FileSystem {
    /// Read a file's contents from somewhere, with the interpretation changing
    /// depending on the URI's scheme.
    fn read(&self, uri: &URI<'_>) -> Result<Vector<u8>, ReadError>;

    /// Wrap a [`FileSystem`] in a simple caching layer.
    fn cached(self) -> Cached<Self>
    where
        Self: Sized,
    {
        Cached::new(self)
    }
}

#[derive(Debug, Clone, thiserror::Error)]
pub enum ReadError {
    #[error("The \"{}\" scheme isn't supported", scheme)]
    UnsupportedScheme { scheme: Text },
    #[error("The item wasn't found")]
    NotFound,
    #[error(transparent)]
    Other(Arc<dyn std::error::Error + Send + Sync + 'static>),
}

impl ReadError {
    pub fn msg(error_message: impl Display + Send + Sync + 'static) -> Self {
        struct Message<T: Display>(T);

        impl<T: Display> Display for Message<T> {
            fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
                self.0.fmt(f)
            }
        }

        impl<T: Display> Debug for Message<T> {
            fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.debug_tuple("Message")
                    .field(&format_args!("{}", self.0))
                    .finish()
            }
        }

        impl<T: Display> std::error::Error for Message<T> {}

        ReadError::other(Message(error_message))
    }

    pub fn other(
        error: impl std::error::Error + Send + Sync + 'static,
    ) -> Self {
        ReadError::Other(Arc::new(error))
    }
}

impl From<std::io::Error> for ReadError {
    fn from(e: std::io::Error) -> Self {
        if e.kind() == std::io::ErrorKind::NotFound {
            ReadError::NotFound
        } else {
            ReadError::other(e)
        }
    }
}

#[derive(Debug)]
pub struct Cached<F> {
    fs: F,
    cache: RwLock<HashMap<URI<'static>, Vector<u8>>>,
}

impl<F> Cached<F> {
    fn new(fs: F) -> Self {
        Cached {
            fs,
            cache: RwLock::new(HashMap::new()),
        }
    }

    pub fn inner(&self) -> &F {
        &self.fs
    }

    pub fn into_inner(self) -> F {
        let Cached { fs, .. } = self;
        fs
    }
}

impl<F: FileSystem> FileSystem for Cached<F> {
    #[tracing::instrument(skip(self), err)]
    fn read(&self, uri: &URI<'_>) -> Result<Vector<u8>, ReadError> {
        if let Some(cached_value) =
            self.cache.read().ok().and_then(|c| c.get(uri).cloned())
        {
            tracing::debug!(%uri, bytes = cached_value.len(),"Cache hit!");
            return Ok(cached_value);
        }

        let value = self.fs.read(uri)?;

        tracing::debug!(%uri, bytes = value.len(),"Adding entry to cache");

        self.cache
            .write()
            .unwrap()
            .insert(uri.clone().into_owned(), value.clone());

        Ok(value)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct WapmUri {
    pub namespace: String,
    pub package_name: String,
    pub version: Option<String>,
}

impl FromStr for WapmUri {
    type Err = ParseWapmUriError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.try_into()
    }
}

impl<'a> TryFrom<&'a str> for WapmUri {
    type Error = ParseWapmUriError;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        let uri: URI = value.try_into()?;
        WapmUri::try_from(&uri)
    }
}

impl TryFrom<&'_ URI<'_>> for WapmUri {
    type Error = ParseWapmUriError;

    fn try_from(uri: &URI<'_>) -> Result<Self, Self::Error> {
        if uri.scheme().as_str() != "wapm" {
            return Err(ParseWapmUriError::IncorrectSchema(
                uri.scheme().clone().into_owned(),
            ));
        }

        let path = uri.path();

        if !path.is_absolute() {
            return Err(ParseWapmUriError::NotAbsolute);
        } else if let Some(uriparse::Host::RegisteredName(name)) =
            uri.authority().map(|a| a.host())
        {
            if !name.is_empty() {
                return Err(ParseWapmUriError::NotAbsolute);
            }
        }

        let (namespace, package_name) = match path.segments() {
            [ns, pkg] => (ns.to_string(), pkg.to_string()),
            _ => {
                return Err(ParseWapmUriError::MalformedPath(
                    path.clone().into_owned(),
                ))
            },
        };

        let version = parse_version_from_query(&uri)?;

        Ok(WapmUri {
            namespace,
            package_name,
            version,
        })
    }
}

fn parse_version_from_query(
    uri: &URI<'_>,
) -> Result<Option<String>, ParseWapmUriError> {
    let query = match uri.query() {
        Some(q) => q,
        None => return Ok(None),
    };

    let parsed = queryst::parse(query.as_borrowed().as_str())
        .map_err(|e| ParseWapmUriError::InvalidQueryString(e.message))?;

    #[derive(serde::Deserialize)]
    struct Query {
        version: Option<String>,
    }

    let Query { version } = Query::deserialize(&parsed).map_err(|e| {
        ParseWapmUriError::InvalidQueryParameters {
            error: e,
            query: query.as_str().to_string(),
        }
    })?;

    Ok(version)
}

#[derive(Debug, thiserror::Error)]
pub enum ParseWapmUriError {
    #[error("Expected a \"wapm\" scheme but found \"{_0}\"")]
    IncorrectSchema(Scheme<'static>),
    #[error("The URI should be an absolute path (i.e. wapm:///...)")]
    NotAbsolute,
    #[error("Expected a path like \"hotg-ai/normalize\", but found \"{_0}\"")]
    MalformedPath(uriparse::Path<'static>),
    #[error("Unable to parse the string as a URI")]
    InvalidUri(#[from] uriparse::URIError),
    #[error("{_0}")]
    InvalidQueryString(String),
    #[error("Unable to parse the version from \"{query}\"")]
    InvalidQueryParameters {
        #[source]
        error: serde_json::Error,
        query: String,
    },
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn incorrect_schema() {
        let uri = "https://example.com";

        let err = WapmUri::try_from(uri).unwrap_err();

        assert!(matches!(
            err,
            ParseWapmUriError::IncorrectSchema(Scheme::HTTPS)
        ));
    }

    #[test]
    fn absolute_path() {
        let uri = "wapm://hotg-ai/normalize";

        let err = WapmUri::try_from(uri).unwrap_err();

        assert!(matches!(err, ParseWapmUriError::NotAbsolute));
    }

    #[test]
    fn path_too_long() {
        let uri = "wapm:///path/to/normalize";

        let err = WapmUri::try_from(uri).unwrap_err();

        assert!(matches!(err, ParseWapmUriError::MalformedPath(_)));
    }

    #[test]
    fn parse_wapm_uri() {
        let uri = "wapm:///hotg-ai/normalize?version=0.12";
        let should_be = WapmUri {
            namespace: "hotg-ai".to_string(),
            package_name: "normalize".to_string(),
            version: Some("0.12".to_string()),
        };

        let got: WapmUri = uri.parse().unwrap();

        assert_eq!(got, should_be);
    }
}
