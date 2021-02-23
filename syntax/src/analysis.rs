use crate::{
    ast::{
        CapabilityInstruction, Ident, Instruction, ModelInstruction,
        OutInstruction, ProcBlockInstruction, RunInstruction, Runefile,
    },
    hir::{
        HirId, Model, Pipeline, PipelineNode, ProcBlock, Rune, Sink, Source,
        SourceKind, Type,
    },
    Diagnostics,
};
use codespan::Span;
use codespan_reporting::diagnostic::{Diagnostic, Label};
use std::path::PathBuf;

type FileId = usize;

pub fn analyse(
    file_id: FileId,
    runefile: &Runefile,
    diags: &mut Diagnostics,
) -> Rune {
    let mut analyser = Analyser::new(file_id, diags);

    analyser.load_runefile(runefile);
    analyser.infer_types();

    analyser.rune
}

#[derive(Debug)]
struct Analyser<'diag> {
    diags: &'diag mut Diagnostics,
    file_id: FileId,
    rune: Rune,
    last_hir_id: HirId,
    unknown_type: HirId,
}

impl<'diag> Analyser<'diag> {
    fn new(file_id: FileId, diags: &'diag mut Diagnostics) -> Self {
        let mut rune = Rune::default();

        let first_id = HirId::ERROR;

        let unknown_type = first_id.next();
        rune.types.insert(unknown_type, Type::Unknown);

        Analyser {
            diags,
            file_id,
            rune,
            last_hir_id: unknown_type,
            unknown_type,
        }
    }

    fn next_id(&mut self) -> HirId {
        let id = self.last_hir_id.next();
        self.last_hir_id = id;
        id
    }

    /// Report an error to the user.
    fn error(&mut self, msg: impl Into<String>, span: Span) {
        let diag = Diagnostic::error()
            .with_message(msg)
            .with_labels(vec![Label::primary(self.file_id, span)]);
        self.diags.push(diag);
    }

    /// Report a warning to the user.
    fn warn(&mut self, msg: impl Into<String>, span: Span) {
        let diag = Diagnostic::warning()
            .with_message(msg)
            .with_labels(vec![Label::primary(self.file_id, span)]);
        self.diags.push(diag);
    }

    fn load_runefile(&mut self, runefile: &Runefile) {
        let mut instructions = runefile.instructions.iter();

        match instructions.next() {
            Some(first_instruction) => {
                if self.load_from(first_instruction).is_err() {
                    // The first instruction was dodgy but we want to process it
                    // anyway.
                    self.load_instruction(first_instruction);
                }
            },
            None => {
                self.error(
                    "A Runefile must contain at least a FROM instruction",
                    runefile.span,
                );
            },
        }

        for instruction in instructions {
            self.load_instruction(instruction);
        }
    }

    fn load_from(&mut self, instruction: &Instruction) -> Result<(), ()> {
        match instruction {
            Instruction::From(f) => {
                self.rune.base_image = Some(f.image.clone());
                Ok(())
            },
            other => {
                self.error(
                    "Runefiles should start with a FROM instruction",
                    other.span(),
                );

                Err(())
            },
        }
    }

    fn load_instruction(&mut self, instruction: &Instruction) -> HirId {
        match instruction {
            Instruction::From(f) => {
                self.error(
                    "A FROM instruction can only be at the top of a Runefile",
                    f.span,
                );
                HirId::ERROR
            },
            Instruction::Model(m) => self.load_model(m),
            Instruction::Capability(c) => self.load_capability(c),
            Instruction::Run(r) => self.load_run(r),
            Instruction::ProcBlock(p) => self.load_proc_block(p),
            Instruction::Out(out) => self.load_out(out),
        }
    }

    fn load_model(&mut self, model: &ModelInstruction) -> HirId {
        let hir = Model {
            input: self.unknown_type,
            output: self.unknown_type,
            model_file: PathBuf::from(&model.file),
        };
        let id = self.next_id();
        self.rune.models.insert(id, hir);
        self.rune.names.register(&model.name.value, id);
        id
    }

    fn load_capability(&mut self, capability: &CapabilityInstruction) -> HirId {
        let kind = match capability.kind.value.as_str() {
            "RAND" => {
                // TODO: We should probably inspect the capability parameters
                // and pull the relevant ones out into actual fields on the Rand
                // variant. That way we aren't relying on the loosely-typed
                // parameter map.
                SourceKind::Rand
            },
            other => {
                self.warn(
                    "This isn't one of the builtin capabilities",
                    capability.kind.span,
                );
                SourceKind::Other(other.to_string())
            },
        };

        let id = self.next_id();
        let output_type = self.interpret_type(&capability.output_type);
        self.rune.sources.insert(
            id,
            Source {
                kind,
                output_type,
                parameters: capability.parameters.clone(),
            },
        );
        self.rune.names.register(&capability.name.value, id);

        id
    }

    fn interpret_type(&mut self, ty: &crate::ast::Type) -> HirId {
        match &ty.kind {
            crate::ast::TypeKind::Inferred => self.unknown_type,
            crate::ast::TypeKind::Named(name) => match name.value.as_str() {
                "U32" | "I32" | "F32" | "U64" | "I64" | "F64" => {
                    // TODO: Actually convert this to a known type
                    self.unknown_type
                },
                _ => {
                    self.warn("Unknown type", name.span);
                    self.unknown_type
                },
            },
            crate::ast::TypeKind::Buffer { .. } => todo!(),
        }
    }

    fn get_named(&mut self, name: &Ident) -> HirId {
        match self.rune.names.get_id(&name.value) {
            Some(id) => id,
            None => {
                self.error("Unknown name", name.span);
                HirId::ERROR
            },
        }
    }

    fn load_run(&mut self, run: &RunInstruction) -> HirId {
        let (first, rest) = match run.steps.as_slice() {
            [f, r @ ..] => (f, r),
            [] => {
                self.error("A RUN instruction can't be empty", run.span);
                return HirId::ERROR;
            },
        };

        let source = self.get_named(first);

        if !self.rune.sources.contains_key(&source) {
            self.error(
                "RUN instructions must start with a CAPABILITY",
                first.span,
            );
            return HirId::ERROR;
        }

        let mut pipeline_node = PipelineNode::Source(source);

        for step in rest {
            let id = self.get_named(step);
            if id.is_error() {
                // it's a dodgy name, we may as well bail.
                return HirId::ERROR;
            }

            if self.rune.models.contains_key(&id) {
                pipeline_node = PipelineNode::Model {
                    model: id,
                    previous: Box::new(pipeline_node),
                };
            } else if self.rune.proc_blocks.contains_key(&id) {
                pipeline_node = PipelineNode::ProcBlock {
                    model: id,
                    previous: Box::new(pipeline_node),
                };
            } else {
                self.error("Unknown pipeline node type", step.span);
                return HirId::ERROR;
            }
        }

        let pipeline = Pipeline {
            last_step: pipeline_node,
            output_type: self.unknown_type,
        };
        let id = self.next_id();
        self.rune.pipelines.insert(id, pipeline);

        id
    }

    fn load_proc_block(&mut self, proc_block: &ProcBlockInstruction) -> HirId {
        let id = self.next_id();
        self.rune.proc_blocks.insert(
            id,
            ProcBlock {
                input: self.unknown_type,
                output: self.unknown_type,
                path: proc_block.path.clone(),
                params: proc_block.params.clone(),
            },
        );
        self.rune.names.register(&proc_block.name.value, id);

        id
    }

    fn load_out(&mut self, out: &OutInstruction) -> HirId {
        match out.out_type.value.as_str() {
            "serial" => {
                let id = self.next_id();
                self.rune.sinks.insert(id, Sink::Serial);
                self.rune.names.register("serial", id);

                id
            },
            _ => {
                self.error("Unknown OUT type", out.out_type.span);

                HirId::ERROR
            },
        }
    }

    fn infer_types(&mut self) {
        // TODO: Go through each pipeline and try to figure out what the
        // input/output type at each stage should be.
        //
        // This will be a bit like a fixed-point iteration
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::{Argument, Ident, Literal, Path};
    use codespan::Span;
    use codespan_reporting::files::SimpleFiles;

    fn setup_analyser(diags: &mut Diagnostics) -> Analyser<'_> {
        let mut files = SimpleFiles::new();
        let id = files.add("", "");
        Analyser::new(id, diags)
    }

    fn setup(src: &str) -> (FileId, Runefile) {
        let mut files = SimpleFiles::new();
        let id = files.add("", src.to_string());

        let runefile = match crate::parse(src) {
            Ok(r) => r,
            Err(e) => panic!("{}", e),
        };

        (id, runefile)
    }

    #[test]
    fn empty_runefile_is_error() {
        let (id, runefile) = setup("");
        let mut diags = Diagnostics::new();

        let got = analyse(id, &runefile, &mut diags);

        assert!(diags.has_errors());
        assert!(got.base_image.is_none());
    }

    #[test]
    fn runefiles_must_start_with_a_from_line() {
        let (id, runefile) = setup("OUT serial");
        let mut diags = Diagnostics::new();

        let got = analyse(id, &runefile, &mut diags);

        assert!(diags.has_errors());
        assert!(got.base_image.is_none());
    }

    #[test]
    fn valid_base_image() {
        let (id, runefile) = setup("FROM runicos/base@1.0");
        let mut diags = Diagnostics::new();

        let got = analyse(id, &runefile, &mut diags);

        assert!(!diags.has_errors());
        assert_eq!(
            got.base_image,
            Some(Path::new(
                "runicos/base",
                "1.0".to_string(),
                Span::new(5, 21)
            ))
        );
    }

    #[test]
    fn unknown_sink_type() {
        let (id, runefile) = setup("FROM runicos/base\nOUT asdf");
        let mut diags = Diagnostics::new();

        let got = analyse(id, &runefile, &mut diags);

        assert!(diags.has_errors());
        assert!(got.sinks.is_empty());
    }

    #[test]
    fn output_serial() {
        let mut diags = Diagnostics::new();
        let mut analyser = setup_analyser(&mut diags);
        let out = OutInstruction {
            out_type: Ident::dangling("serial"),
            span: Span::new(0, 0),
        };

        let id = analyser.load_out(&out);

        assert!(!analyser.diags.has_errors());
        assert_eq!(analyser.rune.sinks.len(), 1);
        assert_eq!(analyser.rune.sinks.get(&id), Some(&Sink::Serial));
        assert_eq!(analyser.rune.names.get_name(id), Some("serial"));
    }

    #[test]
    fn add_model_to_rune() {
        let mut diags = Diagnostics::new();
        let mut analyser = setup_analyser(&mut diags);
        let model = ModelInstruction {
            name: Ident::dangling("sine"),
            file: String::from("./sine.tflite"),
            input_type: crate::ast::Type::inferred_dangling(),
            output_type: crate::ast::Type::inferred_dangling(),
            parameters: Vec::new(),
            span: Span::new(0, 0),
        };

        let id = analyser.load_model(&model);

        assert!(!analyser.diags.has_errors());
        assert!(!id.is_error());
        assert_eq!(analyser.rune.names.get_name(id), Some("sine"));
        assert!(analyser.rune.models.contains_key(&id));
    }

    #[test]
    fn add_rand_capability_to_rune() {
        let mut diags = Diagnostics::new();
        let mut analyser = setup_analyser(&mut diags);
        // CAPABILITY<_,I32> RAND rand --n 1
        let capability = CapabilityInstruction {
            kind: Ident::dangling("RAND"),
            name: Ident::new("rand", Span::new(0, 0)),
            parameters: vec![Argument::literal(
                Ident::new("n", Span::new(0, 0)),
                Literal::new(1, Span::new(0, 0)),
                Span::new(0, 0),
            )]
            .into_iter()
            .collect(),
            output_type: crate::ast::Type::named_dangling("I32"),
            span: Span::new(0, 0),
        };

        let id = analyser.load_capability(&capability);

        assert!(!analyser.diags.has_errors());
        assert!(!id.is_error());
        assert_eq!(analyser.rune.names.get_name(id), Some("rand"));
        let source = &analyser.rune.sources[&id];
        let should_be = Source {
            kind: SourceKind::Rand,
            output_type: analyser.unknown_type,
            parameters: capability.parameters.clone(),
        };
        assert_eq!(source, &should_be);
    }
}
