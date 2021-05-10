use crate::{
    ast::{self, Runefile, Instruction},
    yaml::{Path, Document},
};
use std::collections::HashMap;

impl From<Runefile> for Document {
    fn from(runefile: Runefile) -> Document {
        let mut image = Path::new("runicos/base", None, None);
        let mut pipeline = HashMap::new();

        for instruction in runefile.instructions {
            match instruction {
                Instruction::From(from) => {
                    image = from.image.into();
                },
                Instruction::Model(_) => todo!(),
                Instruction::Capability(_) => todo!(),
                Instruction::Run(_) => todo!(),
                Instruction::ProcBlock(_) => todo!(),
                Instruction::Out(_) => todo!(),
            }
        }

        Document { image, pipeline }
    }
}

impl From<ast::Path> for Path {
    fn from(p: ast::Path) -> Self {
        let ast::Path {
            base,
            sub_path,
            version,
            ..
        } = p;

        Path::new(base, sub_path, version)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn convert_existing_runefile() {
        let runefile = r#"
            FROM runicos/base

            CAPABILITY<I16[16000]> audio SOUND --hz 16000 --sample-duration-ms 1000
            PROC_BLOCK<I16[16000],I8[1960]> fft hotg-ai/rune#proc_blocks/fft
            MODEL<I8[1960],I8[4]> model ./model.tflite
            PROC_BLOCK<I8[4], UTF8> label hotg-ai/rune#proc_blocks/ohv_label --labels=silence,unknown,yes,no
            OUT serial

            RUN audio fft model label serial
        "#;
        crate::parse(runefile).unwrap();
    }
}
