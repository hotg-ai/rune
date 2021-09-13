//!

use atomic_refcell::AtomicRef;
use legion::{Resources, World};

use crate::{Diagnostics, yaml::Document};

/// Callbacks that are fired at different points in the compilation process.
pub trait Hooks {
    /// Callback fired before the Runefile is parsed, giving the [`Hooks`] a
    /// chance to do setup.
    fn before_parse(&mut self, _ctx: &mut dyn Context) -> Continuation {
        Continuation::Continue
    }

    /// Callback fired after parsing the Runefile.
    fn after_parse(
        &mut self,
        _ctx: &mut dyn AfterParseContext,
    ) -> Continuation {
        Continuation::Continue
    }

    /// Callback fired after lowering a [`crate::yaml::Document`] to
    /// [`crate::hir`] types but before any type checking is applied.
    fn after_lowering(
        &mut self,
        _ctx: &mut dyn AfterLoweringContext,
    ) -> Continuation {
        Continuation::Continue
    }

    /// Callback fired after type checking and before the Rune is compiled.
    fn after_type_checking(
        &mut self,
        _ctx: &mut dyn AfterTypeCheckingContext,
    ) -> Continuation {
        Continuation::Continue
    }

    /// Callback fired after compiling the Rune.
    fn after_compile(
        &mut self,
        _ctx: &mut dyn AfterCompileContext,
    ) -> Continuation {
        Continuation::Continue
    }
}

/// How to proceed after calling a [`Hooks`] method.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Continuation {
    /// Keep going.
    Continue,
    /// Stop.
    Halt,
}

/// A [`Hooks`] implementation which does nothing.
pub struct NopHooks;

impl Hooks for NopHooks {}

/// Basic contextual information passed to all [`Hooks`].
pub trait Context {
    fn resources(&self) -> &Resources;
    fn resources_mut(&mut self) -> &mut Resources;
    fn world(&self) -> &World;
    fn world_mut(&mut self) -> &mut World;
    fn world_and_resources(&mut self) -> (&mut World, &mut Resources);
}

pub trait AfterParseContext: Context {
    fn document(&self) -> AtomicRef<'_, Document> {
        self.resources().get().unwrap()
    }
}

pub trait AfterLoweringContext: AfterParseContext {
    fn diagnostics(&self) -> AtomicRef<'_, Diagnostics> {
        self.resources().get().unwrap()
    }
}
pub trait AfterTypeCheckingContext: AfterLoweringContext {}
pub trait AfterCompileContext: AfterTypeCheckingContext {}

pub(crate) struct Ctx<'world, 'res> {
    pub(crate) world: &'world mut World,
    pub(crate) res: &'res mut Resources,
}

impl<'world, 'res> Context for Ctx<'world, 'res> {
    fn resources(&self) -> &Resources { self.res }

    fn resources_mut(&mut self) -> &mut Resources { self.res }

    fn world(&self) -> &World { self.world }

    fn world_mut(&mut self) -> &mut World { self.world }

    fn world_and_resources(&mut self) -> (&mut World, &mut Resources) {
        (self.world, self.res)
    }
}

impl<'world, 'res> AfterParseContext for Ctx<'world, 'res> {}

impl<'world, 'res> AfterLoweringContext for Ctx<'world, 'res> {}

impl<'world, 'res> AfterTypeCheckingContext for Ctx<'world, 'res> {}

impl<'world, 'res> AfterCompileContext for Ctx<'world, 'res> {}
