use std::collections::HashMap;
use std::rc::Rc;

use crate::backend::l1_typed_cst::{builder, SourceUnit};
use crate::bindings::BindingGraph;
use crate::compilation;

pub struct Output {
    pub files: HashMap<String, SourceUnit>,
    pub binding_graph: Rc<BindingGraph>,
}

impl Output {
    pub fn build(input: &compilation::CompilationUnit) -> Result<Self, String> {
        let mut files = HashMap::new();
        for file in &input.files() {
            files.insert(file.id().into(), builder::build_source_unit(file.tree())?);
        }
        let binding_graph = Rc::clone(input.binding_graph());
        Ok(Self {
            files,
            binding_graph,
        })
    }
}
