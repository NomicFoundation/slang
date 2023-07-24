use anyhow::Result;
use codegen_utils::context::CodegenContext;
use serde::Serialize;
use tera::{Context, Tera};

pub trait TemplateContext: Serialize {
    const TEMPLATE_RELATIVE_PATH: &'static str;

    fn render(&self, tera: &Tera) -> Result<String> {
        let context = Context::from_serialize(self)?;
        let result = tera.render(Self::TEMPLATE_RELATIVE_PATH, &context)?;

        return Ok(result);
    }
}

pub fn compile_templates(codegen: &mut CodegenContext) -> Result<Tera> {
    let templates_dir = codegen.repo_root.join("crates/codegen/syntax/src");

    // Make sure to rebuild after any changes to templates:
    codegen.track_input_dir(&templates_dir);

    // This compilation is expensive, so it should only happen once per crate:
    let mut tera = Tera::new(templates_dir.join("**/*.tera").to_str().unwrap())?;

    // disable autoescaping completely for now
    tera.autoescape_on(vec![]);

    // Here is when we can register crate-wide functions and filters
    // to the `tera` instance before returning...

    return Ok(tera);
}
