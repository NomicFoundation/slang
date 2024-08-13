mod file_system;
mod formatting;
mod runtime;
mod tera;
mod wit;

const JINJA_GLOB: &str = "**/*.jinja2";
const WIT_GLOB: &str = "**/*.wit";

pub use file_system::CodegenFileSystem;
pub use runtime::CodegenRuntime;
pub use tera::TeraWrapper;
