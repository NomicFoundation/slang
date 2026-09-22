use crate::ir;

impl ir::ParameterStruct {
    pub fn name_as_string(&self) -> Option<String> {
        self.name.as_ref().map(|name| name.unparse().to_string())
    }
}
