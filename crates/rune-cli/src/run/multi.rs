use std::{
    convert::TryInto,
    fmt::{self, Debug, Formatter},
    sync::Arc,
};
use anyhow::{Context, Error};
use hotg_rune_core::Value;
use hotg_rune_runtime::{Capability, ParameterError};

/// Get a function for creating new [`Capability`] objects which can be
/// initialized by switching between different source values based on the
/// `"source"` parameter (i.e. an image or audio clip).
pub fn new_multiplexer<S, I>(
    sources: I,
) -> impl Fn() -> Result<Box<dyn Capability>, Error>
where
    I: IntoIterator<Item = S::Source>,
    S: SourceBackedCapability,
{
    let sources: Arc<[S::Source]> = sources.into_iter().collect();

    move || {
        anyhow::ensure!(
            sources.len() > 0,
            "No sources were provided for this capability type ({})",
            std::any::type_name::<S>(),
        );

        Ok(Box::new(LazilyInitializedCapability::<S>::Incomplete {
            builder: Default::default(),
            sources: Arc::clone(&sources),
            selected_source: 0,
        }))
    }
}

/// A [`Capability`] which may be partially initialized.
///
/// # Note
///
/// This exists because of the way capabilities were initially designed and
/// switching to [an improved API][issue-153] would break every Rune that has
/// been compiled (which I'm okay with because we have version numbers) as well
/// as the C++ runtime (which I'm not okay with because un-breaking it requires
/// a lot of work).
///
/// The process for setting up a capability is very similar to what you'd do
/// in an OO language.
///
/// 1. Create a new instance of the object in an incomplete state
/// 2. Go through each of the fields and set them
/// 3. Use the object to generate data
///
/// [issue-153]: https://github.com/hotg-ai/rune/issues/153
enum LazilyInitializedCapability<S: SourceBackedCapability> {
    Incomplete {
        sources: Arc<[S::Source]>,
        builder: S::Builder,
        selected_source: usize,
    },
    Initialized(S),
}

impl<S: SourceBackedCapability> LazilyInitializedCapability<S> {
    /// Get the [`SourceBackedCapability`], initializing it if necessary.
    fn initialize(&mut self) -> Result<&mut S, Error> {
        if let LazilyInitializedCapability::Incomplete {
            builder,
            selected_source,
            sources,
        } = self
        {
            let builder = std::mem::take(builder);
            let source = sources.get(*selected_source).with_context(|| {
                format!(
                    "There is no source with index {} (# sources: {})",
                    selected_source,
                    sources.len(),
                )
            })?;

            log::debug!(
                "Initializing the \"{}\" with {:?} and {:?}",
                std::any::type_name::<S>(),
                builder,
                source,
            );

            let cap = S::from_builder(builder, source)
                .context("Unable to initialize the capability")?;
            *self = LazilyInitializedCapability::Initialized(cap);
        }

        match self {
            LazilyInitializedCapability::Initialized(c) => Ok(c),
            LazilyInitializedCapability::Incomplete { .. } => {
                unreachable!("We just initialized the capability")
            },
        }
    }
}

impl<S: SourceBackedCapability> Capability for LazilyInitializedCapability<S> {
    fn generate(&mut self, buffer: &mut [u8]) -> Result<usize, Error> {
        self.initialize()?.generate(buffer)
    }

    fn set_parameter(
        &mut self,
        name: &str,
        value: Value,
    ) -> Result<(), ParameterError> {
        match self {
            LazilyInitializedCapability::Incomplete {
                selected_source, ..
            } if name == "source" => {
                *selected_source = to_usize(value).map_err(|reason| {
                    ParameterError::InvalidValue { value, reason }
                })?;
                Ok(())
            },
            LazilyInitializedCapability::Incomplete { builder, .. } => {
                builder.set_parameter(name, value)
            },
            LazilyInitializedCapability::Initialized(_) => {
                Err(ParameterError::UnsupportedParameter)
            },
        }
    }
}

fn to_usize(value: Value) -> Result<usize, Error> {
    let value: i32 = value.try_into()?;
    let value: usize = value.try_into()?;
    Ok(value)
}

impl<S: SourceBackedCapability> Debug for LazilyInitializedCapability<S> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            LazilyInitializedCapability::Incomplete {
                builder,
                selected_source,
                ..
            } => f
                .debug_struct("Initialized")
                .field("builder", builder)
                .field("selected_source", selected_source)
                .finish_non_exhaustive(),
            LazilyInitializedCapability::Initialized(cap) => {
                f.debug_tuple("Initialized").field(cap).finish()
            },
        }
    }
}

/// A [`Capability`] which can be created by from some source object and a
/// [`Builder`].
pub trait SourceBackedCapability: Send + Debug + Sized + 'static {
    type Source: Debug + Send + Sync + 'static;
    type Builder: Builder + Debug + Send + Sync + 'static;

    fn generate(&mut self, buffer: &mut [u8]) -> Result<usize, Error>;

    fn from_builder(
        builder: Self::Builder,
        source: &Self::Source,
    ) -> Result<Self, Error>;
}

/// A [`Capability`] builder.
pub trait Builder: Default + Debug {
    fn set_parameter(
        &mut self,
        key: &str,
        value: Value,
    ) -> Result<(), ParameterError>;
}
