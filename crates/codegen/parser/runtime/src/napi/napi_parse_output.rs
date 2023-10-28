use {napi::bindgen_prelude::*, napi_derive::napi};

use super::*;
use napi_cst::*;

#[napi(namespace = "parse_output")]
pub struct ParseOutput(RustParseOutput);

impl From<RustParseOutput> for ParseOutput {
    fn from(value: RustParseOutput) -> Self {
        Self(value)
    }
}

#[napi(namespace = "parse_output")]
impl ParseOutput {
    #[napi(getter, ts_return_type = "cst.RuleNode | cst.TokenNode")]
    pub fn parse_tree(&self, env: Env) -> napi::JsObject {
        return self.0.parse_tree().to_js(&env);
    }

    #[napi(getter, ts_return_type = "Array<parse_error.ParseError>")]
    pub fn errors(&self) -> Vec<napi_parse_error::ParseError> {
        self.0.errors().iter().map(|x| x.clone().into()).collect()
    }

    #[napi(getter)]
    pub fn is_valid(&self) -> bool {
        self.0.is_valid()
    }
}
