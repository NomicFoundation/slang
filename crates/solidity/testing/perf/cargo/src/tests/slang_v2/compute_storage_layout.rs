use slang_solidity_v2::compilation::CompilationUnit;

pub type Input = CompilationUnit;

#[allow(unused)]
pub struct Output {
    pub(crate) unit: CompilationUnit,
    pub(crate) storage_layout_json: Vec<String>,
}

pub fn setup(project: &str) -> Input {
    super::full_compilation::test(super::full_compilation::setup(project))
}

// Kept out of line: if inlined into the gungraun wrapper, Callgrind mistakes returns into
// the wrapper for fresh entries, and toggles collection off mid-run.
#[inline(never)]
pub fn run(unit: Input) -> Output {
    test(unit)
}

/// Computes the storage layout of every contract, and renders it as JSON.
pub fn test(unit: Input) -> Output {
    let storage_layout_json = unit
        .all_contracts()
        .filter_map(|contract| contract.compute_storage_layout())
        .map(|layout| serde_json::to_string(&layout).expect("the storage layout serializes"))
        .collect();

    Output {
        unit,
        storage_layout_json,
    }
}

// Contracts whose storage layout cannot be computed are skipped, so comparing this to the
// number of contracts detects any silently dropped.
pub fn count_storage_layouts(output: &Output) -> usize {
    output.storage_layout_json.len()
}
