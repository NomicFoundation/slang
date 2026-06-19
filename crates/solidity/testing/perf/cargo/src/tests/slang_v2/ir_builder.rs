use slang_solidity_v2_cst::structured_cst::nodes::SourceUnit as InputSourceUnit;
use slang_solidity_v2_ir::ir::visitor::{accept_source_unit, Visitor};
use slang_solidity_v2_ir::ir::{self, NodeIdGenerator, SourceUnit, SourceUnitMember};

use crate::dataset::SolidityProject;

pub struct Input {
    pub(crate) project: &'static SolidityProject,
    pub(crate) source_units: Vec<(String, InputSourceUnit)>,
}

pub struct Output {
    pub(crate) ir_source_units: Vec<SourceUnit>,
}

pub fn setup(project: &str) -> Input {
    let project = super::parser::setup(project);
    let parsed = super::parser::test(project);

    Input {
        project,
        source_units: parsed.source_units,
    }
}

pub fn run(input: Input) -> Output {
    test(input)
}

pub fn test(input: Input) -> Output {
    let mut id_generator = NodeIdGenerator::default();
    let mut ir_source_units = Vec::new();
    for (name, source) in &input.source_units {
        let contents = input.project
            .sources
            .get(name)
            .expect("Source not found in project");

        let ir::BuildOutput {
            ir_root,
            diagnostics,
        } = ir::build(name, source, contents, &mut id_generator);

        assert!(
            diagnostics.is_empty(),
            "IR builder produced diagnostics: {diagnostics:#?}"
        );

        ir_source_units.push(ir_root);
    }

    Output { ir_source_units }
}

pub fn count_contracts(output: &Output) -> usize {
    let mut contract_count = 0;
    for source_unit in &output.ir_source_units {
        for member in &source_unit.members {
            match member {
                SourceUnitMember::ContractDefinition(_)
                | SourceUnitMember::InterfaceDefinition(_)
                | SourceUnitMember::LibraryDefinition(_) => contract_count += 1,
                _ => {}
            }
        }
    }
    contract_count
}

// Counts non-abstract `contract` definitions only — the same set that
// `SourceUnit::compute_contracts_abi` operates on. Used as the IR-level
// ground truth to detect whether ABI computation silently drops any
// concrete contract.
pub fn count_concrete_contracts(output: &Output) -> usize {
    let mut count = 0;
    for source_unit in &output.ir_source_units {
        for member in &source_unit.members {
            if let SourceUnitMember::ContractDefinition(contract) = member {
                if !contract.is_abstract {
                    count += 1;
                }
            }
        }
    }
    count
}

pub fn count_identifiers(output: &Output) -> usize {
    struct Checker {
        total: usize,
    }

    impl Visitor for Checker {
        fn visit_identifier(&mut self, _node: &ir::Identifier) {
            self.total += 1;
        }
    }

    let mut checker = Checker { total: 0 };
    for source_unit in &output.ir_source_units {
        accept_source_unit(source_unit, &mut checker);
    }
    checker.total
}
