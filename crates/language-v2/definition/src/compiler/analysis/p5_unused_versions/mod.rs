use crate::compiler::analysis::Analysis;
use crate::compiler::utils::version_set::VersionSet;
use crate::model::Identifier;

pub(crate) fn run(analysis: &mut Analysis) {
    for (name, metadata) in &analysis.metadata {
        if name == &*analysis.language.root_item {
            continue;
        }

        let unused_in = metadata.defined_in.difference(&metadata.used_in);

        if !unused_in.is_empty() {
            analysis
                .errors
                .add(&metadata.name, &Errors::UnusedVersion(name, &unused_in));
        }
    }
}

#[derive(thiserror::Error, Debug)]
enum Errors<'err> {
    #[error("Item '{0}' is not used in versions: {1}")]
    UnusedVersion(&'err Identifier, &'err VersionSet),
}
