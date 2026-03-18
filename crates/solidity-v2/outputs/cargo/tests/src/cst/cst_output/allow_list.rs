use slang_solidity_v2_common::versions::LanguageVersion;

use super::runner::DiffOutcome;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AllowedDiffKind {
    /// V1 parses incorrectly, V2 is correct
    V1Bug,
    /// V2 accepts code that should be rejected for older versions
    V2MissingValidation,
}

impl AllowedDiffKind {
    pub fn matches_outcome(self, outcome: DiffOutcome) -> bool {
        match self {
            AllowedDiffKind::V1Bug => matches!(
                outcome,
                DiffOutcome::DifferentOutput
                    | DiffOutcome::V1InvalidV2Valid
                    | DiffOutcome::V1ValidV2Invalid
            ),
            AllowedDiffKind::V2MissingValidation => {
                matches!(outcome, DiffOutcome::V1InvalidV2Valid)
            }
        }
    }
}

pub enum VersionPredicate {
    /// All versions are affected
    All,
    /// Versions strictly before the given version
    Before(LanguageVersion),
}

impl VersionPredicate {
    pub fn matches(&self, version: LanguageVersion) -> bool {
        match self {
            VersionPredicate::All => true,
            VersionPredicate::Before(v) => version < *v,
        }
    }
}

#[allow(dead_code)]
pub struct AllowListEntry {
    pub parser_name: &'static str,
    pub test_name: &'static str,
    pub kind: AllowedDiffKind,
    pub versions: VersionPredicate,
    pub reason: &'static str,
}

pub fn find_entry(parser_name: &str, test_name: &str) -> Option<&'static AllowListEntry> {
    ALLOW_LIST
        .iter()
        .find(|e| e.parser_name == parser_name && e.test_name == test_name)
}

use AllowedDiffKind::*;
use LanguageVersion::*;
use VersionPredicate::*;

static ALLOW_LIST: &[AllowListEntry] = &[
    // ── V1 Bugs ──
    AllowListEntry {
        parser_name: "SourceUnit",
        test_name: "state_variable_function",
        kind: V1Bug,
        versions: All,
        reason: "V1 parses `function (uint a) internal internal foo` incorrectly",
    },
    AllowListEntry {
        parser_name: "TupleDeconstructionStatement",
        test_name: "with_location",
        kind: V1Bug,
        versions: All,
        reason: "V1 parses `(memory with_location, without_location) = rhs;` incorrectly",
    },
    AllowListEntry {
        parser_name: "TupleDeconstructionStatement",
        test_name: "with_type",
        kind: V1Bug,
        versions: All,
        reason: "V1 parses `(bool with_type, without_type) = rhs;` incorrectly",
    },
    AllowListEntry {
        parser_name: "SourceUnit",
        test_name: "unreserved_keywords",
        kind: V1Bug,
        versions: All,
        reason: "V1 can't parse `uint transient;`",
    },
    // ── V2 Missing Validation ──
    AllowListEntry {
        parser_name: "AssemblyStatement",
        test_name: "with_flags",
        kind: V2MissingValidation,
        versions: Before(V0_8_13),
        reason: "Assembly flags only valid since 0.8.13",
    },
    AllowListEntry {
        parser_name: "ContractDefinition",
        test_name: "member_error_definition",
        kind: V2MissingValidation,
        versions: Before(V0_8_4),
        reason: "Error definition only available since 0.8.4",
    },
    AllowListEntry {
        parser_name: "ContractDefinition",
        test_name: "member_user_defined_value_type_definition",
        kind: V2MissingValidation,
        versions: Before(V0_8_8),
        reason: "User defined value type only available since 0.8.8",
    },
    AllowListEntry {
        parser_name: "ContractDefinition",
        test_name: "storage_specifier_after_inheritance",
        kind: V2MissingValidation,
        versions: Before(V0_8_29),
        reason: "Storage specifier only valid since 0.8.29",
    },
    AllowListEntry {
        parser_name: "ContractDefinition",
        test_name: "storage_specifier_before_inheritance",
        kind: V2MissingValidation,
        versions: Before(V0_8_29),
        reason: "Storage specifier only valid since 0.8.29",
    },
    AllowListEntry {
        parser_name: "ContractDefinition",
        test_name: "storage_specifier_only",
        kind: V2MissingValidation,
        versions: Before(V0_8_29),
        reason: "Storage specifier only valid since 0.8.29",
    },
    AllowListEntry {
        parser_name: "ErrorDefinition",
        test_name: "top_level",
        kind: V2MissingValidation,
        versions: Before(V0_8_4),
        reason: "Error definition only available since 0.8.4",
    },
    AllowListEntry {
        parser_name: "EventDefinition",
        test_name: "transfer",
        kind: V2MissingValidation,
        versions: Before(V0_8_22),
        reason: "Top-level event definition only valid since 0.8.22",
    },
    AllowListEntry {
        parser_name: "MappingType",
        test_name: "named_both",
        kind: V2MissingValidation,
        versions: Before(V0_8_18),
        reason: "Mapping type identifier only valid since 0.8.18",
    },
    AllowListEntry {
        parser_name: "MappingType",
        test_name: "named_key",
        kind: V2MissingValidation,
        versions: Before(V0_8_18),
        reason: "Mapping type identifier only valid since 0.8.18",
    },
    AllowListEntry {
        parser_name: "MappingType",
        test_name: "named_value",
        kind: V2MissingValidation,
        versions: Before(V0_8_18),
        reason: "Mapping type identifier only valid since 0.8.18",
    },
    AllowListEntry {
        parser_name: "SourceUnit",
        test_name: "error_definition",
        kind: V2MissingValidation,
        versions: Before(V0_8_4),
        reason: "Error definition only available since 0.8.4",
    },
    AllowListEntry {
        parser_name: "SourceUnit",
        test_name: "everything",
        kind: V2MissingValidation,
        versions: Before(V0_8_22),
        reason: "Top-level event definition only valid since 0.8.22",
    },
    AllowListEntry {
        parser_name: "SourceUnit",
        test_name: "layout_at",
        kind: V2MissingValidation,
        versions: Before(V0_8_29),
        reason: "Storage specifier only valid since 0.8.29",
    },
    AllowListEntry {
        parser_name: "SourceUnit",
        test_name: "revert_statement",
        kind: V2MissingValidation,
        versions: Before(V0_8_4),
        reason: "Revert statement only valid since 0.8.4",
    },
    AllowListEntry {
        parser_name: "SourceUnit",
        test_name: "top_level_event",
        kind: V2MissingValidation,
        versions: Before(V0_8_22),
        reason: "Top-level event definition only valid since 0.8.22",
    },
    AllowListEntry {
        parser_name: "SourceUnit",
        test_name: "using_directive",
        kind: V2MissingValidation,
        versions: Before(V0_8_13),
        reason: "Top-level using directive only valid since 0.8.13",
    },
    AllowListEntry {
        parser_name: "StateVariableDefinition",
        test_name: "transient",
        kind: V2MissingValidation,
        versions: Before(V0_8_27),
        reason: "Transient keyword only valid since 0.8.27",
    },
    AllowListEntry {
        parser_name: "UserDefinedValueTypeDefinition",
        test_name: "bool",
        kind: V2MissingValidation,
        versions: Before(V0_8_8),
        reason: "User defined value type only available since 0.8.8",
    },
    AllowListEntry {
        parser_name: "UsingDeconstructionSymbol",
        test_name: "identifier_path",
        kind: V2MissingValidation,
        versions: Before(V0_8_13),
        reason: "Using deconstruction symbol only valid since 0.8.13",
    },
    AllowListEntry {
        parser_name: "UsingDeconstructionSymbol",
        test_name: "identifier_path_as_operator",
        kind: V2MissingValidation,
        versions: Before(V0_8_19),
        reason: "Using deconstruction symbol with alias only valid since 0.8.19",
    },
    AllowListEntry {
        parser_name: "UsingDeconstructionSymbol",
        test_name: "single_id",
        kind: V2MissingValidation,
        versions: Before(V0_8_13),
        reason: "Using deconstruction symbol only valid since 0.8.13",
    },
    AllowListEntry {
        parser_name: "UsingDeconstructionSymbol",
        test_name: "single_id_as_operator",
        kind: V2MissingValidation,
        versions: Before(V0_8_19),
        reason: "Using deconstruction symbol with alias only valid since 0.8.19",
    },
    AllowListEntry {
        parser_name: "UsingDirective",
        test_name: "destructure_multiple",
        kind: V2MissingValidation,
        versions: Before(V0_8_19),
        reason: "Using destructure with alias only valid since 0.8.19",
    },
    AllowListEntry {
        parser_name: "UsingDirective",
        test_name: "destructure_single",
        kind: V2MissingValidation,
        versions: Before(V0_8_13),
        reason: "Using directive with global keyword only valid since 0.8.13",
    },
    AllowListEntry {
        parser_name: "UsingDirective",
        test_name: "path_named",
        kind: V2MissingValidation,
        versions: Before(V0_8_13),
        reason: "Using directive with global keyword only valid since 0.8.13",
    },
    AllowListEntry {
        parser_name: "UsingDirective",
        test_name: "path_unnamed",
        kind: V2MissingValidation,
        versions: Before(V0_8_13),
        reason: "Top-level using directive only valid since 0.8.13",
    },
    AllowListEntry {
        parser_name: "UsingDirective",
        test_name: "user_defined_operator",
        kind: V2MissingValidation,
        versions: Before(V0_8_19),
        reason: "User-defined operators in using directive only valid since 0.8.19",
    },
    AllowListEntry {
        parser_name: "PragmaDirective",
        test_name: "version_string",
        kind: V2MissingValidation,
        versions: All,
        reason: "V2 accepts string version pragma that V1 rejects",
    },
];
