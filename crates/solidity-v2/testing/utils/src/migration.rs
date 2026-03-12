use std::collections::HashSet;
use std::fmt::Write;
use std::path::{Path, PathBuf};

use anyhow::{bail, Result};
use infra_utils::paths::{FileWalker, PathExtensions};

/// Migrates v1 test inputs to v2 format by wrapping them in valid `SourceUnit` boilerplate.
///
/// Each v1 test input is a snippet for a specific nonterminal (e.g., `Expression`, `Block`).
/// The v2 parser only supports parsing `SourceUnit`, so we wrap each snippet in appropriate
/// Solidity context.
pub fn migrate_v1_tests_to_v2(v1_snapshots_dir: &Path, v2_output_dir: &Path) -> Result<()> {
    let mut written_files = HashSet::<PathBuf>::new();

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
                let output_path = v2_output_dir.join(format!("{group_name}/{test_name}/input.sol"));
                write_file(&output_path, &wrapped)?;
                written_files.insert(output_path);
            }
            [group_name, test_name, ".gitattributes"] => {
                let content = file.read_to_string()?;
                let output_path =
                    v2_output_dir.join(format!("{group_name}/{test_name}/.gitattributes"));
                write_file(&output_path, &content)?;
                written_files.insert(output_path);
            }
            _ => {
                bail!("Unexpected file in v1 snapshots: {file:?}");
            }
        }
    }

    // Clean up stale files that were not written in this run.
    // Only remove non-generated files (generated/ dirs are managed by the test runner).
    for file in FileWalker::from_directory(v2_output_dir).find_all()? {
        if file.is_generated() {
            continue;
        }
        if !written_files.contains(&file) {
            std::fs::remove_file(&file)?;
        }
    }

    Ok(())
}

fn write_file(path: &Path, contents: &str) -> Result<()> {
    std::fs::create_dir_all(path.unwrap_parent())?;

    // Skip writing if already up to date.
    if path.exists() && contents == path.read_to_string()? {
        return Ok(());
    }

    path.write_string(contents)
}

fn wrap_test_input(group_name: &str, test_name: &str, content: &str) -> String {
    let begin_marker = format!(
        "// >>> Copied from crates/solidity/testing/snapshots/cst_output/{group_name}/{test_name}/input.sol"
    );
    let end_marker = "// <<<";

    // Match the line ending style of the original content so that files
    // governed by `.gitattributes eol=crlf` stay stable across checkouts.
    let line_ending = if content.contains("\r\n") {
        "\r\n"
    } else {
        "\n"
    };

    let group = wrapping_group(group_name);
    apply_template(group, &begin_marker, end_marker, content, line_ending)
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

fn write_marked(out: &mut String, begin: &str, end: &str, content: &str, eol: &str) {
    write!(out, "{begin}{eol}").unwrap();
    write!(out, "{content}{eol}").unwrap();
    write!(out, "{end}{eol}").unwrap();
}

fn apply_template(
    group: WrappingGroup,
    begin_marker: &str,
    end_marker: &str,
    content: &str,
    eol: &str,
) -> String {
    let mut out = String::new();

    match group {
        WrappingGroup::None | WrappingGroup::SourceUnitMember => {
            write_marked(&mut out, begin_marker, end_marker, content, eol);
        }
        WrappingGroup::ContractMember => {
            write!(out, "contract C {{{eol}").unwrap();
            write_marked(&mut out, begin_marker, end_marker, content, eol);
            write!(out, "}}{eol}").unwrap();
        }
        WrappingGroup::Block => {
            write!(out, "contract C {{{eol}").unwrap();
            write!(out, "    function f(){eol}").unwrap();
            write_marked(&mut out, begin_marker, end_marker, content, eol);
            write!(out, "}}{eol}").unwrap();
        }
        WrappingGroup::Statement => {
            write!(out, "contract C {{{eol}").unwrap();
            write!(out, "    function f() {{{eol}").unwrap();
            write_marked(&mut out, begin_marker, end_marker, content, eol);
            write!(out, "    }}{eol}").unwrap();
            write!(out, "}}{eol}").unwrap();
        }
        WrappingGroup::Expression => {
            write!(out, "contract C {{{eol}").unwrap();
            write!(out, "    function f() {{{eol}").unwrap();
            write_marked(&mut out, begin_marker, end_marker, content, eol);
            write!(out, ";{eol}").unwrap();
            write!(out, "    }}{eol}").unwrap();
            write!(out, "}}{eol}").unwrap();
        }
        WrappingGroup::TypeName => {
            write!(out, "contract C {{{eol}").unwrap();
            write_marked(&mut out, begin_marker, end_marker, content, eol);
            write!(out, "x;{eol}").unwrap();
            write!(out, "}}{eol}").unwrap();
        }
        WrappingGroup::StringLiteral => {
            write!(out, "contract C {{{eol}").unwrap();
            write!(out, "    function f() {{{eol}").unwrap();
            write_marked(&mut out, begin_marker, end_marker, content, eol);
            write!(out, ";{eol}").unwrap();
            write!(out, "    }}{eol}").unwrap();
            write!(out, "}}{eol}").unwrap();
        }
        WrappingGroup::YulBlock => {
            write!(out, "contract C {{{eol}").unwrap();
            write!(out, "    function f() {{{eol}").unwrap();
            write!(out, "        assembly{eol}").unwrap();
            write_marked(&mut out, begin_marker, end_marker, content, eol);
            write!(out, "    }}{eol}").unwrap();
            write!(out, "}}{eol}").unwrap();
        }
        WrappingGroup::YulStatement => {
            write!(out, "contract C {{{eol}").unwrap();
            write!(out, "    function f() {{{eol}").unwrap();
            write!(out, "        assembly {{{eol}").unwrap();
            write_marked(&mut out, begin_marker, end_marker, content, eol);
            write!(out, "        }}{eol}").unwrap();
            write!(out, "    }}{eol}").unwrap();
            write!(out, "}}{eol}").unwrap();
        }
        WrappingGroup::VersionPragma => {
            write!(out, "pragma{eol}").unwrap();
            write_marked(&mut out, begin_marker, end_marker, content, eol);
            write!(out, ";{eol}").unwrap();
        }
        WrappingGroup::UsingDeconstructionSymbol => {
            write!(out, "using {{{eol}").unwrap();
            write_marked(&mut out, begin_marker, end_marker, content, eol);
            write!(out, "}} for uint;{eol}").unwrap();
        }
    }

    out
}
