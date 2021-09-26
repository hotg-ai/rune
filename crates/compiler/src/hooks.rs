//! Callbacks that allow users to hook into the build process.

use atomic_refcell::{AtomicRef, AtomicRefMut};
use legion::{Resources, World};

use crate::{
    BuildContext, Diagnostics, FeatureFlags, compile::CompilationResult,
    lowering::NameTable, parse::DocumentV1,
};

/// Callbacks that are fired at different points in the compilation process.
///
/// Each hook is optional, with the default implementation returning
/// [`Continuation::Halt`] when [`Diagnostics::has_errors()`] indicates there
/// were errors.
pub trait Hooks {
    /// Callback fired before the Runefile is parsed, giving the [`Hooks`] a
    /// chance to do setup (e.g. by registering global [`Resources`] used in
    /// later steps).
    fn before_parse(&mut self, _ctx: &mut dyn Context) -> Continuation {
        Continuation::Continue
    }

    /// Callback fired after parsing the Runefile.
    fn after_parse(&mut self, ctx: &mut dyn AfterParseContext) -> Continuation {
        if ctx.diagnostics().has_errors() {
            Continuation::Halt
        } else {
            Continuation::Continue
        }
    }

    /// Callback fired after lowering a [`crate::parse::Document`] to
    /// [`crate::lowering`] types but before any type checking is applied.
    fn after_lowering(
        &mut self,
        ctx: &mut dyn AfterLoweringContext,
    ) -> Continuation {
        if ctx.diagnostics().has_errors() {
            Continuation::Halt
        } else {
            Continuation::Continue
        }
    }

    /// Callback fired after type checking and before codegen.
    fn after_type_checking(
        &mut self,
        ctx: &mut dyn AfterTypeCheckingContext,
    ) -> Continuation {
        if ctx.diagnostics().has_errors() {
            Continuation::Halt
        } else {
            Continuation::Continue
        }
    }

    /// Callback fired after generating the Rust project but immediately before
    /// it is compiled to WebAssembly.
    fn after_codegen(
        &mut self,
        ctx: &mut dyn AfterCodegenContext,
    ) -> Continuation {
        if ctx.diagnostics().has_errors() {
            Continuation::Halt
        } else {
            Continuation::Continue
        }
    }

    fn after_compile(
        &mut self,
        ctx: &mut dyn AfterCompileContext,
    ) -> Continuation {
        if ctx.diagnostics().has_errors() {
            Continuation::Halt
        } else {
            Continuation::Continue
        }
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

/// Basic contextual information passed to all [`Hooks`].
pub trait Context {
    fn resources(&self) -> &Resources;
    fn resources_mut(&mut self) -> &mut Resources;
    fn world(&self) -> &World;
    fn world_mut(&mut self) -> &mut World;
    fn world_and_resources(&mut self) -> (&mut World, &mut Resources);

    fn build_context(&self) -> AtomicRef<'_, BuildContext> {
        self.resources().get().unwrap()
    }

    fn feature_flags(&self) -> AtomicRef<'_, FeatureFlags> {
        self.resources().get().unwrap()
    }
}

/// Context passed to the [`Hooks::after_parse()`] method.
pub trait AfterParseContext: Context {
    fn document(&self) -> AtomicRef<'_, DocumentV1> {
        self.resources().get().unwrap()
    }

    fn document_mut(&self) -> AtomicRefMut<'_, DocumentV1> {
        self.resources().get_mut().unwrap()
    }

    fn diagnostics(&self) -> AtomicRef<'_, Diagnostics> {
        self.resources().get().unwrap()
    }

    fn diagnostics_mut(&self) -> AtomicRefMut<'_, Diagnostics> {
        self.resources().get_mut().unwrap()
    }
}

/// Context passed to the [`Hooks::after_lowering()`] method.
pub trait AfterLoweringContext: AfterParseContext {
    fn names(&self) -> AtomicRef<'_, NameTable> {
        self.resources().get().unwrap()
    }
}

/// Context passed to the [`Hooks::after_type_checking()`] method.
pub trait AfterTypeCheckingContext: AfterLoweringContext {}

/// Context passed to the [`Hooks::after_codegen()`] method.
pub trait AfterCodegenContext: AfterTypeCheckingContext {}

/// Context passed to the [`Hooks::after_compile()`] method.
pub trait AfterCompileContext: AfterCodegenContext {
    fn take_compilation_result(&mut self) -> CompilationResult {
        self.resources_mut().remove().unwrap()
    }
}

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

impl<'world, 'res> AfterCodegenContext for Ctx<'world, 'res> {}

impl<'world, 'res> AfterCompileContext for Ctx<'world, 'res> {}
