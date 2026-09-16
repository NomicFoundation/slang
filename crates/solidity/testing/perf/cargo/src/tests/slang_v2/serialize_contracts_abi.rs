use slang_solidity_v2_ast::abi;

pub struct Input {
    pub(crate) abi: Vec<abi::ContractAbi>,
}

#[allow(unused)]
pub struct Output {
    pub(crate) abi: Vec<abi::ContractAbi>,
    pub(crate) json: Vec<String>,
}

pub fn setup(project: &str) -> Input {
    let abi_output =
        super::compute_contracts_abi::run(super::compute_contracts_abi::setup(project));
    Input {
        abi: abi_output.abi,
    }
}

pub fn run(input: Input) -> Output {
    test(input)
}

/// Renders every contract's ABI as solc's JSON. This is where the ABI's
/// solc-specific spellings are paid for, overload ordering by selector
/// included, so it is measured apart from `compute_contracts_abi`.
pub fn test(input: Input) -> Output {
    let json = input
        .abi
        .iter()
        .map(|abi| serde_json::to_string(&abi.json()).expect("the ABI serializes"))
        .collect();
    Output {
        abi: input.abi,
        json,
    }
}

pub fn count_concrete_contracts(output: &Output) -> usize {
    output.json.len()
}
