use crate::compiler::analysis::Analysis;

pub(crate) fn analyze_queries(analysis: &mut Analysis) {
    for query in analysis.language.queries.values() {
        // TODO(#554): use the real parser to validate against existing kinds/names:
        if query.contains("test") {
            analysis.errors.add(query, &Errors::TestQuery);
        }
    }
}

#[allow(clippy::enum_variant_names)]
#[derive(thiserror::Error, Debug)]
enum Errors {
    #[error("Test queries should be replaced with the real thing.")]
    TestQuery,
}
