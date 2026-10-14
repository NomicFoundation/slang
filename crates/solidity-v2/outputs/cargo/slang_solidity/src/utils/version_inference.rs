use std::sync::Arc;

use semver::Version;
use slang_solidity_v2_common::files::FileId;
use slang_solidity_v2_common::versions::LanguageVersion;
use slang_solidity_v2_ir::ir::{self, NodeIdGenerator, NodeIdGroups};
use slang_solidity_v2_parser::{LexemeKind, Lexer, ParseOutput, Parser};

/// Infers the language versions that `source` can be compiled with, from the
/// `pragma solidity` directives it holds.
///
/// The result is exact for valid 0.8.x source files. Anything else, older code
/// or invalid Solidity, is best effort: the directives are still looked for,
/// but some may be missed or misread.
pub fn infer_language_versions(source: &str) -> VersionConstraint {
    VersionConstraint {
        pragmas: find_version_pragmas(source),
    }
}

/// The version that directives are lexed, parsed, and built with.
///
/// The pragma grammar is the same in every version, so any of them reads a
/// directive the same way, but the lexer, parser, and IR builder should all agree
/// on it.
///
/// Should the grammar ever start to differ between versions, a single
/// version would no longer do, and each directive would have to be read with
/// every version it could be meant for.
const SCAN_LANGUAGE_VERSION: LanguageVersion = LanguageVersion::LATEST;

/// The versions a source admits, as its `pragma solidity` directives state
/// them. See [`infer_language_versions`].
///
/// A version is admitted when every directive admits it, so a source with no
/// directive at all admits every version. Since that holds for any version,
/// released or not, supported by Slang or not, the constraint is not a list of
/// versions but a test: [`Self::matches`] answers it for any version, and
/// [`Self::supported_versions`] lists the ones Slang supports.
#[derive(Clone, Debug)]
pub struct VersionConstraint {
    pragmas: Vec<ir::VersionPragma>,
}

impl VersionConstraint {
    /// Whether the source admits `version`, which may be any version: older or
    /// newer than the ones Slang supports, or not released yet.
    pub fn matches(&self, version: impl Into<Version>) -> bool {
        let version = version.into();

        self.pragmas
            .iter()
            .all(|pragma| pragma.matches_version(version.clone()))
    }

    /// The versions supported by Slang that the source admits, in ascending
    /// order.
    pub fn supported_versions(&self) -> impl Iterator<Item = LanguageVersion> + '_ {
        LanguageVersion::ALL
            .iter()
            .copied()
            .filter(|&version| self.matches(version))
    }
}

/// Collects every version pragma in `source`.
///
/// The lexer tells a directive apart from the same text inside a comment or a
/// string, and it never gives up on the input, so scanning the lexemes finds
/// every directive without having to parse the rest of the source. Each one is
/// then parsed and built on its own, which is what keeps a syntax error
/// elsewhere from hiding it.
fn find_version_pragmas(source: &str) -> Vec<ir::VersionPragma> {
    let file_id = FileId::from("version-pragma.sol");
    let mut id_generator = NodeIdGroups::default()
        .next()
        .expect("the node-id space holds at least one group");

    // Every directive starts with the `pragma` keyword, so none can start past
    // the last place the source spells it, in code or not.
    let Some(last_keyword_start) = source.rfind("pragma") else {
        return Vec::new();
    };

    let mut lexer = Lexer::new(source, SCAN_LANGUAGE_VERSION);
    let mut directive_start = None;
    let mut pragmas = Vec::new();

    while let Some(lexeme) = lexer.next_lexeme() {
        if directive_start.is_none() && lexeme.range.start > last_keyword_start {
            break;
        }

        match lexeme.kind {
            LexemeKind::PragmaKeyword_Reserved => {
                directive_start = Some(lexeme.range.start);
            }
            // A `pragma` lexeme within the Pragma context resets the current directive,
            // this helps to correctly extract versions from invalid Solidity files
            LexemeKind::PragmaIdentifier if &source[lexeme.range.clone()] == "pragma" => {
                directive_start = Some(lexeme.range.start);
            }
            LexemeKind::PragmaSemicolon => {
                if let Some(start) = directive_start.take() {
                    let directive = &source[start..lexeme.range.end];
                    pragmas.extend(build_version_pragma(&file_id, directive, &mut id_generator));
                }
            }
            _ => {}
        }
    }

    pragmas
}

/// Parses a single `pragma … ;` directive, which is a source unit on its own,
/// and returns it if it is a version pragma.
fn build_version_pragma(
    file_id: &FileId,
    directive: &str,
    id_generator: &mut NodeIdGenerator,
) -> Option<ir::VersionPragma> {
    // Diagnostics are dropped: a directive that does not parse comes out as an
    // empty source unit, and one that does is only used for its version
    // constraints.
    let ParseOutput { source_unit, .. } = Parser::parse(file_id, directive, SCAN_LANGUAGE_VERSION);

    let ir::BuildOutput { ir_root, .. } = ir::build(
        file_id,
        &source_unit,
        &directive,
        SCAN_LANGUAGE_VERSION,
        id_generator,
    );

    ir_root.members.iter().find_map(|member| match member {
        ir::SourceUnitMember::PragmaDirective(pragma_directive) => match &pragma_directive.pragma {
            ir::Pragma::VersionPragma(pragma) => Some(Arc::clone(pragma)),
            _ => None,
        },
        _ => None,
    })
}
