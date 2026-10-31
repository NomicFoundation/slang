use slang_solidity_v2::compilation::CompilationUnit;

pub type Input = CompilationUnit;

#[allow(unused)]
pub struct Output {
    pub(crate) unit: CompilationUnit,
    pub(crate) abi_json: Vec<String>,
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

/// Computes the ABI of every contract, interface, and library, and renders it as JSON.
pub fn test(unit: Input) -> Output {
    let abi_json = unit
        .compute_abi()
        .iter()
        .map(|abi| serde_json::to_string(abi).expect("the ABI serializes"))
        .collect();

    Output { unit, abi_json }
}

// Definitions whose ABI cannot be computed are skipped, so comparing this to the
// number of contracts, interfaces, and libraries detects any silently dropped.
pub fn count_abis(output: &Output) -> usize {
    output.abi_json.len()
}
