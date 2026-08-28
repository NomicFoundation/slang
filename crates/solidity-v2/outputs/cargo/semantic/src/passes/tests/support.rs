//! Shared scaffolding for the pass unit tests: building files, running the
//! pipeline over them, and locating definitions in the result.
//!
//! Every test starts from [`Analysis::builder`] (or [`Analysis::of_source`]
//! for the single-file case) and ends at [`AnalysisBuilder::run`], which says
//! how far to take the pipeline, optionally chaining
//! [`Analysis::expect_no_diagnostics`]. There is deliberately no second way in:
//! a suite that wants a particular combination often enough should name it in a
//! local helper over this builder rather than grow one of its own.
//!
//! Tests should stop at the narrowest [`Analyse`] level that runs the passes
//! they need, so a failure points at the pass that caused it rather than at a
//! later one consuming its output.

use slang_solidity_v2_common::collections::{Map, Set};
use slang_solidity_v2_common::diagnostics::kinds::DiagnosticKind;
use slang_solidity_v2_common::diagnostics::{Diagnostic, DiagnosticCollection};
use slang_solidity_v2_common::evm_targets::EvmTarget;
use slang_solidity_v2_common::files::FileId;
use slang_solidity_v2_common::nodes::NodeId;
use slang_solidity_v2_common::versions::LanguageVersion;
use slang_solidity_v2_ir::ir::{self, NodeIdGenerator};
use slang_solidity_v2_parser::{ParseOutput, Parser};

use crate::binder::Binder;
use crate::context::{
    FileNodeMapper, SemanticContext, SemanticFile, extract_imports_from_source_unit,
};
use crate::passes::{
    p1_collect_definitions, p2_linearise_contracts, p3_type_definitions, p4_compute_linearisations,
    p5_resolve_references, p6_resolve_yul,
};
use crate::types::TypeRegistry;

struct TestFile {
    id: FileId,
    ir_root: ir::SourceUnit,
    /// Empty unless the file was built by [`build_files`], which is the only
    /// helper that knows about sibling files to resolve imports against.
    resolved_imports: Map<NodeId, FileId>,
}

impl SemanticFile for TestFile {
    fn id(&self) -> &FileId {
        &self.id
    }

    fn ir_root(&self) -> &ir::SourceUnit {
        &self.ir_root
    }

    fn resolved_import_by_node_id(&self, node_id: NodeId) -> Option<&FileId> {
        self.resolved_imports.get(&node_id)
    }
}

fn build_file(
    file_id: FileId,
    contents: &str,
    id_generator: &mut NodeIdGenerator,
    language_version: LanguageVersion,
) -> TestFile {
    let ParseOutput {
        source_unit,
        diagnostics,
    } = Parser::parse(&file_id, contents, language_version);

    assert!(
        diagnostics.is_empty(),
        "Parser diagnostics: {diagnostics:?}"
    );

    let ir::BuildOutput {
        ir_root,
        diagnostics,
    } = ir::build(
        &file_id,
        &source_unit,
        &contents,
        language_version,
        id_generator,
    );

    assert!(
        diagnostics.is_empty(),
        "IR builder diagnostics: {diagnostics:?}"
    );

    TestFile {
        id: file_id,
        ir_root,
        resolved_imports: Map::default(),
    }
}

/// Builds several files that may import each other, given as `(file name,
/// contents)` pairs. Import paths name the files verbatim, so a source
/// importing `"b.sol"` resolves to the entry named `b.sol`; a path naming no
/// entry stays unresolved, as it would for a missing file.
///
/// The files are built in the order given, which is the order the binder sees
/// their nodes in, and hence the order a file-scope lookup resolves them in.
fn build_files(sources: &[(&str, &str)], language_version: LanguageVersion) -> Vec<TestFile> {
    let mut id_generator = NodeIdGenerator::default();

    let mut seen = Set::default();
    if let Some((first_duplicate, _)) = sources.iter().find(|(name, _)| !seen.insert(name)) {
        panic!("The file {first_duplicate} is duplicated in the source set");
    }

    sources
        .iter()
        .map(|(name, contents)| {
            let mut file = build_file(
                (*name).into(),
                contents,
                &mut id_generator,
                language_version,
            );

            file.resolved_imports = extract_imports_from_source_unit(&file.ir_root)
                .into_iter()
                .filter(|import| sources.iter().any(|(name, _)| name == &import.path))
                .map(|import| (import.node_id, import.path.as_str().into()))
                .collect();

            file
        })
        .collect()
}

/// How far down the pipeline an [`Analysis`] is taken. Every pass up to and
/// including the chosen one is run, in the same order as the real pipeline, so
/// the result only ever differs from a full analysis by the passes that come
/// after. The variants are named after what they make available rather than
/// after their numbers, so a test declares what it needs instead of encoding
/// the pass order.
///
/// The ordering is what selects the passes, so the variants must stay in
/// pipeline order.
#[derive(Clone, Copy, Eq, Ord, PartialEq, PartialOrd)]
pub(super) enum Analyse {
    /// `p1`-`p2`: definitions, scopes and linearised bases.
    Definitions,
    /// … through `p3`: the types of every definition.
    Types,
    /// … through `p5`: references and expression typings.
    References,
    /// … through `p6`: Yul definitions and references.
    Yul,
    /// The whole pipeline, run the way the real one is, which additionally
    /// makes [`Analysis::context`] available. The passes past `p6` only ever
    /// derive from a complete analysis, so there is no prefix to stop at
    /// between here and [`Self::Yul`].
    Context,
}

/// Configures an [`Analysis`]: the files to analyse and the settings to
/// analyse them under. Defaults to the latest language version and EVM target.
///
/// Reach for [`Analysis::of_source`] instead of [`Analysis::builder`] when a
/// single file will do, which is the common case.
pub(super) struct AnalysisBuilder<'a> {
    sources: Vec<(&'a str, &'a str)>,
    language_version: LanguageVersion,
    evm_target: EvmTarget,
}

impl<'a> AnalysisBuilder<'a> {
    /// Adds a file, named by the path other files import it as. Files are
    /// analysed in the order they are added.
    pub(super) fn file(mut self, path: &'a str, contents: &'a str) -> Self {
        self.sources.push((path, contents));
        self
    }

    pub(super) fn version(mut self, language_version: LanguageVersion) -> Self {
        self.language_version = language_version;
        self
    }

    pub(super) fn target(mut self, evm_target: EvmTarget) -> Self {
        self.evm_target = evm_target;
        self
    }

    /// Runs the passes up to and including `analyse`, without asserting on the
    /// diagnostics.
    pub(super) fn run(self, analyse: Analyse) -> Analysis {
        assert!(
            !self.sources.is_empty(),
            "an analysis needs at least one file"
        );

        let files = build_files(&self.sources, self.language_version);
        let mut diagnostics = DiagnosticCollection::default();

        let output = if analyse == Analyse::Context {
            // The full pipeline is the real one, so run it through the same
            // entry point production does rather than restating its pass list.
            Output::Context(SemanticContext::build_from(
                self.language_version,
                self.evm_target,
                &files,
                None,
                &mut diagnostics,
            ))
        } else {
            self.run_prefix(analyse, &files, &mut diagnostics)
        };

        Analysis {
            files,
            output,
            diagnostics,
        }
    }

    /// Runs the passes up to `analyse`, in the same order the real pipeline
    /// does. Only for levels short of [`Analyse::Context`]: the passes past
    /// `p6` derive from a complete analysis, so a prefix of them is not a thing
    /// to ask for.
    fn run_prefix(
        &self,
        analyse: Analyse,
        files: &[TestFile],
        diagnostics: &mut DiagnosticCollection,
    ) -> Output {
        let mut binder = Binder::default();
        let mut types = TypeRegistry::new(self.language_version);
        let file_node_mapper = FileNodeMapper::build_from(files);

        p1_collect_definitions::run(files, &mut binder, self.language_version, diagnostics);
        p2_linearise_contracts::run(files, &mut binder, diagnostics);
        if analyse >= Analyse::Types {
            p3_type_definitions::run(
                files,
                &mut binder,
                self.language_version,
                &mut types,
                &file_node_mapper,
                diagnostics,
            );
        }
        if analyse >= Analyse::References {
            // `p4` runs for its hierarchy diagnostics; the `ContractData` it
            // computes is only consumed by `p7`/`p8`, which these tests stop
            // short of.
            drop(p4_compute_linearisations::run(
                &binder,
                &types,
                &file_node_mapper,
                diagnostics,
            ));
            p5_resolve_references::run(
                files,
                &mut binder,
                &mut types,
                &file_node_mapper,
                diagnostics,
            );
        }
        if analyse >= Analyse::Yul {
            p6_resolve_yul::run(
                &mut binder,
                self.language_version,
                self.evm_target,
                &types,
                &file_node_mapper,
                diagnostics,
            );
        }

        Output::Prefix { binder, types }
    }
}

/// What the passes left behind, which depends on how far the pipeline was
/// taken: a prefix hands back the pieces it filled in, while the full pipeline
/// assembles them into a [`SemanticContext`] and keeps them there.
enum Output {
    Prefix { binder: Binder, types: TypeRegistry },
    Context(SemanticContext),
}

/// The result of running the pipeline over one or more sources.
pub(super) struct Analysis {
    files: Vec<TestFile>,
    output: Output,
    pub(super) diagnostics: DiagnosticCollection,
}

impl Analysis {
    /// Starts configuring an analysis. Add at least one [`file`].
    ///
    /// [`file`]: AnalysisBuilder::file
    pub(super) fn builder<'a>() -> AnalysisBuilder<'a> {
        AnalysisBuilder {
            sources: Vec::new(),
            language_version: LanguageVersion::LATEST,
            evm_target: EvmTarget::LATEST,
        }
    }

    /// Starts configuring an analysis of a single source, named `test.sol`.
    pub(super) fn of_source(source: &str) -> AnalysisBuilder<'_> {
        Self::builder().file("test.sol", source)
    }

    /// Panics if any pass reported a diagnostic. Tests that expect one should
    /// inspect [`Self::diagnostics`] instead.
    pub(super) fn expect_no_diagnostics(self) -> Self {
        assert!(
            self.diagnostics.is_empty(),
            "Semantic diagnostics: {:?}",
            self.diagnostics
        );
        self
    }

    pub(super) fn binder(&self) -> &Binder {
        match &self.output {
            Output::Prefix { binder, .. } => binder,
            Output::Context(context) => context.binder(),
        }
    }

    pub(super) fn types(&self) -> &TypeRegistry {
        match &self.output {
            Output::Prefix { types, .. } => types,
            Output::Context(context) => context.types(),
        }
    }

    /// The registry, for tests that go on to register types of their own.
    /// Panics under [`Analyse::Context`], which keeps its registry inside the
    /// context; use [`Self::types`] there.
    pub(super) fn into_type_registry(self) -> TypeRegistry {
        match self.output {
            Output::Prefix { types, .. } => types,
            Output::Context(_) => panic!("`Analyse::Context` owns its type registry"),
        }
    }

    /// What the passes past `p6` derived. Panics unless the analysis ran
    /// [`Analyse::Context`], the only level that computes it.
    pub(super) fn context(&self) -> &SemanticContext {
        match &self.output {
            Output::Context(context) => context,
            Output::Prefix { .. } => panic!("a context needs `Analyse::Context`"),
        }
    }

    /// [`Self::context`], handed over so a helper can return it on its own.
    /// Any diagnostic the passes reported is dropped, so callers that care
    /// must either assert on it first or keep the [`Analysis`] around.
    pub(super) fn into_context(self) -> SemanticContext {
        match self.output {
            Output::Context(context) => context,
            Output::Prefix { .. } => panic!("a context needs `Analyse::Context`"),
        }
    }

    /// The IR of every file analysed, in the order they were added, for tests
    /// that walk it themselves.
    pub(super) fn source_units(&self) -> impl Iterator<Item = &ir::SourceUnit> {
        self.files.iter().map(|file| &file.ir_root)
    }

    /// The one top-level member of the files analysed that `matches` selects.
    /// Panics unless exactly one does, so a name declared by two files is a
    /// failure rather than a silent pick of whichever came first.
    fn find_unique<'a, T>(
        &'a self,
        kind: &str,
        name: &str,
        matches: impl FnMut(&'a ir::SourceUnitMember) -> Option<&'a T>,
    ) -> &'a T {
        let mut found = self
            .source_units()
            .flat_map(|source_unit| source_unit.members.iter())
            .filter_map(matches);

        let first = found
            .next()
            .unwrap_or_else(|| panic!("{kind} `{name}` not found"));
        assert!(
            found.next().is_none(),
            "more than one {kind} named `{name}`"
        );
        first
    }

    pub(super) fn find_contract(&self, name: &str) -> &ir::ContractDefinition {
        self.find_unique("contract", name, |member| match member {
            ir::SourceUnitMember::ContractDefinition(contract)
                if contract.name.unparse() == name =>
            {
                Some(contract)
            }
            _ => None,
        })
    }

    pub(super) fn find_library(&self, name: &str) -> &ir::LibraryDefinition {
        self.find_unique("library", name, |member| match member {
            ir::SourceUnitMember::LibraryDefinition(library) if library.name.unparse() == name => {
                Some(library)
            }
            _ => None,
        })
    }

    /// The members of the contract or library named `name`, for tests that
    /// don't care which of the two declared it.
    pub(super) fn find_members(&self, name: &str) -> &[ir::ContractMember] {
        self.find_unique("contract or library", name, |member| match member {
            ir::SourceUnitMember::ContractDefinition(contract)
                if contract.name.unparse() == name =>
            {
                Some(&contract.members)
            }
            ir::SourceUnitMember::LibraryDefinition(library) if library.name.unparse() == name => {
                Some(&library.members)
            }
            _ => None,
        })
    }

    /// The body of `function`, declared by the contract or library `owner`.
    /// This is the way into a function's statements: a test should not have to
    /// spell out the walk from a source unit down to a block.
    pub(super) fn function_body(&self, owner: &str, function: &str) -> &ir::Block {
        find_function(self.find_members(owner), function)
            .unwrap_or_else(|| panic!("no function `{function}` in `{owner}`"))
            .body
            .as_ref()
            .unwrap_or_else(|| panic!("`{owner}.{function}` has no body"))
    }
}

/// Finds the function named `name` among a contract's or library's `members`.
pub(super) fn find_function<'a>(
    members: &'a [ir::ContractMember],
    name: &str,
) -> Option<&'a ir::FunctionDefinition> {
    members.iter().find_map(|member| match member {
        ir::ContractMember::FunctionDefinition(function)
            if function.name.as_ref().is_some_and(|n| n.unparse() == name) =>
        {
            Some(function)
        }
        _ => None,
    })
}

/// The kind of the single diagnostic in `diagnostics`, or `None` when there
/// are none. Panics if there is more than one, so a test asserting on a
/// diagnostic can't silently miss a second.
pub(super) fn diagnostic_kind(diagnostics: &DiagnosticCollection) -> Option<DiagnosticKind> {
    let mut iter = diagnostics.iter();
    let first = iter.next()?;
    assert!(
        iter.next().is_none(),
        "expected a single diagnostic: {diagnostics:?}"
    );
    Some(first.kind().clone())
}

/// The kind of every diagnostic in `diagnostics`, ordered by `Diagnostic` (ie.
/// file+location+kind), for tests expecting more than one.
pub(super) fn diagnostic_kinds(diagnostics: &DiagnosticCollection) -> Vec<DiagnosticKind> {
    diagnostics
        .iter()
        .map(|diagnostic| diagnostic.kind().clone())
        .collect()
}

/// The single diagnostic in `diagnostics`, for tests that assert on more of it
/// than its kind. Panics unless there is exactly one; reach for
/// [`diagnostic_kind`] where reporting nothing is also a valid outcome.
pub(super) fn only_diagnostic(diagnostics: &DiagnosticCollection) -> &Diagnostic {
    let mut iter = diagnostics.iter();
    let first = iter.next();
    let (Some(first), None) = (first, iter.next()) else {
        panic!("expected a single diagnostic: {diagnostics:?}");
    };
    first
}

// Tests for the scaffolding itself. This module only compiles under `cfg(test)`
// already, so these need no gate of their own.

/// Looking a definition up searches every file, not just the first.
#[test]
fn test_lookups_span_every_file() {
    let analysis = Analysis::builder()
        .file("a.sol", "contract A {}")
        .file("b.sol", "library B {}")
        .run(Analyse::Definitions)
        .expect_no_diagnostics();

    assert_eq!(2, analysis.source_units().count());
    assert_eq!("A", analysis.find_contract("A").name.unparse());
    assert_eq!("B", analysis.find_library("B").name.unparse());
}

/// The binder and the registry read back the same either way, so a test can
/// move up to [`Analyse::Context`] without rewriting what it asserts.
#[test]
fn test_the_context_level_still_exposes_the_binder_and_types() {
    const CONTENTS: &str = "contract C { uint256 public x; }";

    let prefix = Analysis::of_source(CONTENTS)
        .run(Analyse::References)
        .expect_no_diagnostics();
    let full = Analysis::of_source(CONTENTS)
        .run(Analyse::Context)
        .expect_no_diagnostics();

    assert_eq!(
        prefix.binder().definitions().len(),
        full.binder().definitions().len()
    );
    assert_eq!(
        prefix.types().iter_types().count(),
        full.types().iter_types().count()
    );
    // The IR lookups work at either level too, and the context is reachable
    // only from the full one.
    assert_eq!("C", full.find_contract("C").name.unparse());
    assert_eq!(1, full.context().all_contracts().count());
}

/// A function body is reachable by name whether a contract or a library
/// declares it, so a test doesn't pick the lookup based on the owner's kind.
#[test]
fn test_function_bodies_are_found_in_contracts_and_libraries() {
    let analysis = Analysis::of_source(
        "contract C { function f() internal { 1; } }
         library L { function g() internal { 1; 2; } }",
    )
    .run(Analyse::Definitions)
    .expect_no_diagnostics();

    assert_eq!(1, analysis.function_body("C", "f").statements.len());
    assert_eq!(2, analysis.function_body("L", "g").statements.len());
}

/// Only [`Analyse::Context`] computes a context, so asking a prefix for one
/// fails rather than handing back something the later passes never filled in.
#[test]
#[should_panic(expected = "a context needs `Analyse::Context`")]
fn test_a_prefix_analysis_has_no_context() {
    Analysis::of_source("contract C {}")
        .run(Analyse::Yul)
        .context();
}

/// A name declared by two files has no single answer, so asking for it fails
/// rather than picking whichever file came first.
#[test]
#[should_panic(expected = "more than one contract named `C`")]
fn test_a_name_declared_by_two_files_is_rejected() {
    Analysis::builder()
        .file("a.sol", "contract C {}")
        .file("b.sol", "contract C {}")
        .run(Analyse::Definitions)
        .find_contract("C");
}
