use std::collections::{BTreeMap, BTreeSet};
use std::fmt::Write;
use std::path::Path;

use anyhow::{bail, Result};
use infra_utils::codegen::CodegenFileSystem;
use infra_utils::paths::{FileWalker, PathExtensions};

/// Migrates v1 test inputs to v2 format by wrapping them in valid `SourceUnit` boilerplate.
///
/// Each v1 test input is a snippet for a specific nonterminal (e.g., `Expression`, `Block`).
/// The v2 parser only supports parsing `SourceUnit`, so we wrap each snippet in appropriate
/// Solidity context.
///
/// Returns a map of group names to test names, suitable for passing to
/// `generate_cst_output_test_harness`.
pub fn migrate_v1_tests_to_v2(
    v1_snapshots_dir: &Path,
    v2_output_dir: &Path,
) -> Result<BTreeMap<String, BTreeSet<String>>> {
    let mut fs = CodegenFileSystem::default();
    let mut tests = BTreeMap::<String, BTreeSet<String>>::new();

    for file in FileWalker::from_directory(v1_snapshots_dir).find_all()? {
        if file.is_generated() {
            continue;
        }

        let parts: Vec<_> = file
            .strip_prefix(v1_snapshots_dir)?
            .iter()
            .map(|p| p.to_str().unwrap())
            .collect();

        match parts[..] {
            [group_name, test_name, "input.sol"] => {
                let content = file.read_to_string()?;
                let wrapped = wrap_test_input(group_name, test_name, &content);
                let output_path =
                    v2_output_dir.join(format!("{group_name}/{test_name}/input/generated/input.sol"));
                fs.write_file_raw(&output_path, wrapped)?;

                tests
                    .entry(group_name.to_owned())
                    .or_default()
                    .insert(test_name.to_owned());
            }
            [group_name, test_name, ".gitattributes"] => {
                let content = file.read_to_string()?;
                let output_path =
                    v2_output_dir.join(format!("{group_name}/{test_name}/input/generated/.gitattributes"));
                fs.write_file_raw(&output_path, content)?;
            }
            _ => {
                bail!("Unexpected file in v1 snapshots: {file:?}");
            }
        }
    }

    Ok(tests)
}

fn wrap_test_input(group_name: &str, test_name: &str, content: &str) -> String {
    let begin_marker = format!(
        "// >>> Copied from crates/solidity/testing/snapshots/cst_output/{group_name}/{test_name}/input.sol"
    );
    let end_marker = "// <<<";

    let group = wrapping_group(group_name);
    let wrapped = apply_template(group, &begin_marker, end_marker, content);

    wrapped
}

#[derive(Debug, Clone, Copy)]
enum WrappingGroup {
    None,
    SourceUnitMember,
    ContractMember,
    Block,
    Statement,
    Expression,
    TypeName,
    StringLiteral,
    YulBlock,
    YulStatement,
    VersionPragma,
    UsingDeconstructionSymbol,
}

fn wrapping_group(group_name: &str) -> WrappingGroup {
    match group_name {
        "SourceUnit" => WrappingGroup::None,

        "ContractDefinition"
        | "InterfaceDefinition"
        | "LibraryDefinition"
        | "FunctionDefinition"
        | "ErrorDefinition"
        | "EnumDefinition"
        | "StructDefinition"
        | "EventDefinition"
        | "UserDefinedValueTypeDefinition"
        | "UsingDirective"
        | "ConstantDefinition"
        | "PragmaDirective"
        | "ImportDirective" => WrappingGroup::SourceUnitMember,

        "ContractMembers"
        | "ConstructorDefinition"
        | "FallbackFunctionDefinition"
        | "ReceiveFunctionDefinition"
        | "ModifierDefinition"
        | "StateVariableDefinition"
        | "UnnamedFunctionDefinition" => WrappingGroup::ContractMember,

        "Block" => WrappingGroup::Block,

        "Statements"
        | "BreakStatement"
        | "ReturnStatement"
        | "ThrowStatement"
        | "VariableDeclarationStatement"
        | "TupleDeconstructionStatement"
        | "AssemblyStatement"
        | "TryStatement" => WrappingGroup::Statement,

        "Expression"
        | "ConditionalExpression"
        | "ExponentiationExpression"
        | "FunctionCallExpression"
        | "NewExpression"
        | "TupleExpression"
        | "DecimalNumberExpression"
        | "HexNumberExpression" => WrappingGroup::Expression,

        "TypeName" | "FunctionType" | "MappingType" => WrappingGroup::TypeName,

        "StringLiteral" | "StringLiterals" | "HexStringLiterals" | "UnicodeStringLiterals" => {
            WrappingGroup::StringLiteral
        }

        "YulBlock" => WrappingGroup::YulBlock,

        "YulExpression"
        | "YulFunctionCallExpression"
        | "YulLabel"
        | "YulLeaveStatement"
        | "YulStackAssignmentStatement"
        | "YulStatement"
        | "YulStatements"
        | "YulVariableAssignmentStatement"
        | "YulVariableDeclarationStatement" => WrappingGroup::YulStatement,

        "VersionPragma" => WrappingGroup::VersionPragma,

        "UsingDeconstructionSymbol" => WrappingGroup::UsingDeconstructionSymbol,

        _ => panic!("Unknown nonterminal group: {group_name}"),
    }
}

fn write_marked(out: &mut String, begin: &str, end: &str, content: &str) {
    writeln!(out, "{begin}").unwrap();
    writeln!(out, "{content}").unwrap();
    writeln!(out, "{end}").unwrap();
}

fn apply_template(
    group: WrappingGroup,
    begin_marker: &str,
    end_marker: &str,
    content: &str,
) -> String {
    let mut out = String::new();

    match group {
        WrappingGroup::None | WrappingGroup::SourceUnitMember => {
            write_marked(&mut out, begin_marker, end_marker, content);
        }
        WrappingGroup::ContractMember => {
            writeln!(out, "contract C {{").unwrap();
            write_marked(&mut out, begin_marker, end_marker, content);
            writeln!(out, "}}").unwrap();
        }
        WrappingGroup::Block => {
            writeln!(out, "contract C {{").unwrap();
            writeln!(out, "    function f()").unwrap();
            write_marked(&mut out, begin_marker, end_marker, content);
            writeln!(out, "}}").unwrap();
        }
        WrappingGroup::Statement => {
            writeln!(out, "contract C {{").unwrap();
            writeln!(out, "    function f() {{").unwrap();
            write_marked(&mut out, begin_marker, end_marker, content);
            writeln!(out, "    }}").unwrap();
            writeln!(out, "}}").unwrap();
        }
        WrappingGroup::Expression => {
            writeln!(out, "contract C {{").unwrap();
            writeln!(out, "    function f() {{").unwrap();
            write_marked(&mut out, begin_marker, end_marker, content);
            writeln!(out, ";").unwrap();
            writeln!(out, "    }}").unwrap();
            writeln!(out, "}}").unwrap();
        }
        WrappingGroup::TypeName => {
            writeln!(out, "contract C {{").unwrap();
            write_marked(&mut out, begin_marker, end_marker, content);
            writeln!(out, "x;").unwrap();
            writeln!(out, "}}").unwrap();
        }
        WrappingGroup::StringLiteral => {
            writeln!(out, "contract C {{").unwrap();
            writeln!(out, "    function f() {{").unwrap();
            write_marked(&mut out, begin_marker, end_marker, content);
            writeln!(out, ";").unwrap();
            writeln!(out, "    }}").unwrap();
            writeln!(out, "}}").unwrap();
        }
        WrappingGroup::YulBlock => {
            writeln!(out, "contract C {{").unwrap();
            writeln!(out, "    function f() {{").unwrap();
            writeln!(out, "        assembly").unwrap();
            write_marked(&mut out, begin_marker, end_marker, content);
            writeln!(out, "    }}").unwrap();
            writeln!(out, "}}").unwrap();
        }
        WrappingGroup::YulStatement => {
            writeln!(out, "contract C {{").unwrap();
            writeln!(out, "    function f() {{").unwrap();
            writeln!(out, "        assembly {{").unwrap();
            write_marked(&mut out, begin_marker, end_marker, content);
            writeln!(out, "        }}").unwrap();
            writeln!(out, "    }}").unwrap();
            writeln!(out, "}}").unwrap();
        }
        WrappingGroup::VersionPragma => {
            writeln!(out, "pragma").unwrap();
            write_marked(&mut out, begin_marker, end_marker, content);
            writeln!(out, ";").unwrap();
        }
        WrappingGroup::UsingDeconstructionSymbol => {
            writeln!(out, "using {{").unwrap();
            write_marked(&mut out, begin_marker, end_marker, content);
            writeln!(out, "}} for uint;").unwrap();
        }
    }

    out
}
