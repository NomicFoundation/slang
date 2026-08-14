//! Shared scaffolding for the pass unit tests: building files, running a
//! prefix of the pipeline over them, and locating definitions in the result.
//!
//! Tests should reach for the narrowest constructor that runs the passes they
//! need, so a failure points at the pass that caused it rather than at a later
//! one consuming its output.

use slang_solidity_v2_common::collections::Map;
use slang_solidity_v2_common::diagnostics::DiagnosticCollection;
use slang_solidity_v2_common::diagnostics::kinds::DiagnosticKind;
use slang_solidity_v2_common::evm_targets::EvmTarget;
use slang_solidity_v2_common::files::FileId;
use slang_solidity_v2_common::nodes::NodeId;
use slang_solidity_v2_common::versions::LanguageVersion;
use slang_solidity_v2_ir::ir::{self, NodeIdGenerator};
use slang_solidity_v2_parser::{ParseOutput, Parser};

use crate::binder::Binder;
use crate::context::{
    FileNodeMapper, SemanticContext, SemanticFile, extract_import_paths_from_source_unit,
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

    sources
        .iter()
        .map(|(name, contents)| {
            let mut file = build_file(
                (*name).into(),
                contents,
                &mut id_generator,
                language_version,
            );

            file.resolved_imports = extract_import_paths_from_source_unit(&file.ir_root)
                .into_iter()
                .filter(|(_, path)| sources.iter().any(|(name, _)| name == path))
                .map(|(node_id, path)| (node_id, path.as_str().into()))
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
}

/// Configures an [`Analysis`]: the files to analyse and the settings to
/// analyse them under. Defaults to the latest language version and EVM target,
/// and to running every pass up to [`Analyse::References`].
///
/// Reach for [`analyze`] instead when one file at the defaults will do, which
/// is the common case.
pub(super) struct AnalysisBuilder<'a> {
    sources: Vec<(&'a str, &'a str)>,
    language_version: LanguageVersion,
    evm_target: EvmTarget,
    analyse: Analyse,
}

impl<'a> AnalysisBuilder<'a> {
    /// Adds a file, named by the path other files import it as. Files are
    /// analysed in the order they are added.
    pub(super) fn file(mut self, path: &'a str, contents: &'a str) -> Self {
        self.sources.push((path, contents));
        self
    }

    /// Private for now: `analyze_at` is the only caller. Widen it when a test
    /// needs a version together with something else the builder configures.
    fn version(mut self, language_version: LanguageVersion) -> Self {
        self.language_version = language_version;
        self
    }

    pub(super) fn target(mut self, evm_target: EvmTarget) -> Self {
        self.evm_target = evm_target;
        self
    }

    pub(super) fn analyse(mut self, analyse: Analyse) -> Self {
        self.analyse = analyse;
        self
    }

    /// Runs the passes, without asserting on the diagnostics.
    pub(super) fn run(self) -> Analysis {
        Analysis::of_files(
            build_files(&self.sources, self.language_version),
            self.language_version,
            self.evm_target,
            self.analyse,
        )
    }

    /// [`Self::run`], asserting that no pass reported a diagnostic.
    pub(super) fn expecting_no_diagnostics(self) -> Analysis {
        self.run().expect_no_diagnostics()
    }

    /// Runs *every* pass and returns the resulting [`SemanticContext`], for
    /// tests that assert on what the later passes derive rather than on the
    /// binder directly. `Analyse` doesn't apply: the context is always built
    /// from the full pipeline.
    pub(super) fn context_with_diagnostics(self) -> (SemanticContext, DiagnosticCollection) {
        let files = build_files(&self.sources, self.language_version);
        let mut diagnostics = DiagnosticCollection::default();
        let context = SemanticContext::build_from(
            self.language_version,
            self.evm_target,
            &files,
            None,
            &mut diagnostics,
        );
        (context, diagnostics)
    }

    /// [`Self::context_with_diagnostics`], asserting that no pass reported a
    /// diagnostic.
    pub(super) fn context(self) -> SemanticContext {
        let (context, diagnostics) = self.context_with_diagnostics();
        assert!(
            diagnostics.is_empty(),
            "Semantic diagnostics: {diagnostics:?}"
        );
        context
    }
}

/// The result of running the pipeline over one or more sources: everything the
/// passes produced, owned so tests can keep mutating the registry.
pub(super) struct Analysis {
    files: Vec<TestFile>,
    pub(super) binder: Binder,
    pub(super) types: TypeRegistry,
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
            analyse: Analyse::References,
        }
    }

    /// Starts configuring an analysis of a single source, named `test.sol`.
    pub(super) fn of_source(source: &str) -> AnalysisBuilder<'_> {
        Self::builder().file("test.sol", source)
    }

    fn of_files(
        files: Vec<TestFile>,
        language_version: LanguageVersion,
        evm_target: EvmTarget,
        analyse: Analyse,
    ) -> Self {
        assert!(!files.is_empty(), "an analysis needs at least one file");

        let mut binder = Binder::default();
        let mut types = TypeRegistry::new(language_version);
        let mut diagnostics = DiagnosticCollection::default();
        let file_node_mapper = FileNodeMapper::build_from(&files);

        p1_collect_definitions::run(&files, &mut binder, language_version, &mut diagnostics);
        p2_linearise_contracts::run(&files, &mut binder, &mut diagnostics);
        if analyse >= Analyse::Types {
            p3_type_definitions::run(
                &files,
                &mut binder,
                language_version,
                &mut types,
                &file_node_mapper,
                &mut diagnostics,
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
                &mut diagnostics,
            ));
            p5_resolve_references::run(
                &files,
                &mut binder,
                &mut types,
                &file_node_mapper,
                &mut diagnostics,
            );
        }
        if analyse >= Analyse::Yul {
            p6_resolve_yul::run(
                &mut binder,
                language_version,
                evm_target,
                &types,
                &file_node_mapper,
                &mut diagnostics,
            );
        }

        Self {
            files,
            binder,
            types,
            diagnostics,
        }
    }

    /// Panics if any pass reported a diagnostic. Tests that expect one should
    /// inspect [`Self::diagnostics`] instead.
    fn expect_no_diagnostics(self) -> Self {
        assert!(
            self.diagnostics.is_empty(),
            "Semantic diagnostics: {:?}",
            self.diagnostics
        );
        self
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
}

/// Runs every pass over `source` up to references, asserting none of them
/// reported a diagnostic. This is what most tests want.
pub(super) fn analyze(source: &str) -> Analysis {
    Analysis::of_source(source).expecting_no_diagnostics()
}

/// [`analyze`] at a specific language version, for version-gated behaviour.
pub(super) fn analyze_at(source: &str, language_version: LanguageVersion) -> Analysis {
    Analysis::of_source(source)
        .version(language_version)
        .expecting_no_diagnostics()
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

// Tests for the scaffolding itself. This module only compiles under `cfg(test)`
// already, so these need no gate of their own.

/// Looking a definition up searches every file, not just the first.
#[test]
fn test_lookups_span_every_file() {
    let analysis = Analysis::builder()
        .file("a.sol", "contract A {}")
        .file("b.sol", "library B {}")
        .analyse(Analyse::Definitions)
        .expecting_no_diagnostics();

    assert_eq!(2, analysis.source_units().count());
    assert_eq!("A", analysis.find_contract("A").name.unparse());
    assert_eq!("B", analysis.find_library("B").name.unparse());
}

/// A name declared by two files has no single answer, so asking for it fails
/// rather than picking whichever file came first.
#[test]
#[should_panic(expected = "more than one contract named `C`")]
fn test_a_name_declared_by_two_files_is_rejected() {
    Analysis::builder()
        .file("a.sol", "contract C {}")
        .file("b.sol", "contract C {}")
        .analyse(Analyse::Definitions)
        .run()
        .find_contract("C");
}
