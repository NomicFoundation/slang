use std::fmt::Write;
use std::path::Path;

use anyhow::{bail, Result};
use infra_utils::codegen::CodegenFileSystem;
use infra_utils::paths::{FileWalker, PathExtensions};

/// Migrates v1 test inputs to v2 format by wrapping them in valid SourceUnit boilerplate.
///
/// Each v1 test input is a snippet for a specific nonterminal (e.g., `Expression`, `Block`).
/// The v2 parser only supports parsing `SourceUnit`, so we wrap each snippet in appropriate
/// Solidity context.
pub fn migrate_v1_tests_to_v2(v1_snapshots_dir: &Path, v2_output_dir: &Path) -> Result<()> {
    let mut fs = CodegenFileSystem::default();

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
                let wrapped = wrap_test_input(group_name, test_name, &content)?;
                let output_path = v2_output_dir.join(format!("{group_name}/{test_name}/input.sol"));
                fs.write_file_raw(&output_path, wrapped)?;
            }
            [.., ".gitattributes"] => {}
            _ => {
                bail!("Unexpected file in v1 snapshots: {file:?}");
            }
        }
    }

    Ok(())
}

fn wrap_test_input(group_name: &str, test_name: &str, content: &str) -> Result<String> {
    let begin_marker = format!(
        "// >>> Copied from crates/solidity/testing/snapshots/cst_output/{group_name}/{test_name}/input.sol"
    );
    let end_marker = "// <<<";

    let group = wrapping_group(group_name);
    let wrapped = apply_template(group, &begin_marker, end_marker, content);

    Ok(wrapped)
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

// @Claude: This function repeats this too many times:
// ```
// writeln!(out, "{begin_marker}").unwrap();
// write!(out, "{content}").unwrap();
// writeln!(out, "{end_marker}").unwrap();
// ```
// Can you make it better?
fn apply_template(
    group: WrappingGroup,
    begin_marker: &str,
    end_marker: &str,
    content: &str,
) -> String {
    let content_trimmed = content.trim_end();
    let mut out = String::new();

    match group {
        WrappingGroup::None => {
            writeln!(out, "{begin_marker}").unwrap();
            write!(out, "{content}").unwrap();
            writeln!(out, "{end_marker}").unwrap();
        }
        WrappingGroup::SourceUnitMember => {
            writeln!(out, "{begin_marker}").unwrap();
            write!(out, "{content}").unwrap();
            writeln!(out, "{end_marker}").unwrap();
        }
        WrappingGroup::ContractMember => {
            writeln!(out, "contract C {{").unwrap();
            writeln!(out, "{begin_marker}").unwrap();
            write!(out, "{content}").unwrap();
            writeln!(out, "{end_marker}").unwrap();
            writeln!(out, "}}").unwrap();
        }
        WrappingGroup::Block => {
            writeln!(out, "contract C {{").unwrap();
            writeln!(out, "    function f()").unwrap();
            writeln!(out, "{begin_marker}").unwrap();
            write!(out, "{content}").unwrap();
            writeln!(out, "{end_marker}").unwrap();
            writeln!(out, "}}").unwrap();
        }
        WrappingGroup::Statement => {
            writeln!(out, "contract C {{").unwrap();
            writeln!(out, "    function f() {{").unwrap();
            writeln!(out, "{begin_marker}").unwrap();
            write!(out, "{content}").unwrap();
            writeln!(out, "{end_marker}").unwrap();
            writeln!(out, "    }}").unwrap();
            writeln!(out, "}}").unwrap();
        }
        WrappingGroup::Expression => {
            writeln!(out, "contract C {{").unwrap();
            writeln!(out, "    function f() {{").unwrap();
            writeln!(out, "{begin_marker}").unwrap();
            writeln!(out, "        ({content_trimmed});").unwrap();
            writeln!(out, "{end_marker}").unwrap();
            writeln!(out, "    }}").unwrap();
            writeln!(out, "}}").unwrap();
        }
        WrappingGroup::TypeName => {
            writeln!(out, "contract C {{").unwrap();
            writeln!(out, "{begin_marker}").unwrap();
            writeln!(out, "    {content_trimmed} x;").unwrap();
            writeln!(out, "{end_marker}").unwrap();
            writeln!(out, "}}").unwrap();
        }
        WrappingGroup::StringLiteral => {
            writeln!(out, "contract C {{").unwrap();
            writeln!(out, "    function f() {{").unwrap();
            writeln!(out, "{begin_marker}").unwrap();
            writeln!(out, "        {content_trimmed};").unwrap();
            writeln!(out, "{end_marker}").unwrap();
            writeln!(out, "    }}").unwrap();
            writeln!(out, "}}").unwrap();
        }
        WrappingGroup::YulBlock => {
            writeln!(out, "contract C {{").unwrap();
            writeln!(out, "    function f() {{").unwrap();
            writeln!(out, "        assembly").unwrap();
            writeln!(out, "{begin_marker}").unwrap();
            write!(out, "{content}").unwrap();
            writeln!(out, "{end_marker}").unwrap();
            writeln!(out, "    }}").unwrap();
            writeln!(out, "}}").unwrap();
        }
        WrappingGroup::YulStatement => {
            writeln!(out, "contract C {{").unwrap();
            writeln!(out, "    function f() {{").unwrap();
            writeln!(out, "        assembly {{").unwrap();
            writeln!(out, "{begin_marker}").unwrap();
            write!(out, "{content}").unwrap();
            writeln!(out, "{end_marker}").unwrap();
            writeln!(out, "        }}").unwrap();
            writeln!(out, "    }}").unwrap();
            writeln!(out, "}}").unwrap();
        }
        WrappingGroup::VersionPragma => {
            writeln!(out, "{begin_marker}").unwrap();
            writeln!(out, "pragma {content_trimmed};").unwrap();
            writeln!(out, "{end_marker}").unwrap();
        }
        WrappingGroup::UsingDeconstructionSymbol => {
            writeln!(out, "using {{").unwrap();
            writeln!(out, "{begin_marker}").unwrap();
            writeln!(out, "    {content_trimmed}").unwrap();
            writeln!(out, "{end_marker}").unwrap();
            writeln!(out, "}} for uint;").unwrap();
        }
    }

    out
}
