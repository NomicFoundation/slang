// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use crate::bindings_support::Bindings;
use crate::language::Language;

pub fn create_for(language: Language) -> Bindings {
    _ = language;
    unreachable!("Language does not define binding rules");
}
mod supress_dependencies {
    use stack_graphs as _;
}
