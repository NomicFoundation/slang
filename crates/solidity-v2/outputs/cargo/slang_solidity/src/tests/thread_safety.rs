use std::hint::black_box;
use std::sync::Arc;
use std::thread;

use crate::ast::NodeId;
use crate::compilation::{CompilationUnit, FileId};
use crate::diagnostics::Diagnostic;
use crate::tests::fixtures;
use crate::tests::support::{compile, file_with_empty_contract};

#[test]
fn parallel_access_via_arc_consistent_with_serial_baseline() {
    const EXPECTED_DEFINITIONS: usize = 22;
    const EXPECTED_REFERENCES: usize = 48;
    const EXPECTED_CONTRACT_ABIS: usize = 1;

    let unit = fixtures::Counter::build_compilation_unit();

    let serial_definitions = unit.all_definitions().count();
    let serial_references = unit.all_references().count();
    let serial_contract_abis = unit.compute_contracts_abi().len();
    let serial_file_ids: Vec<_> = unit.files().map(|f| f.id().clone()).collect();

    assert_eq!(serial_definitions, EXPECTED_DEFINITIONS);
    assert_eq!(serial_references, EXPECTED_REFERENCES);
    assert_eq!(serial_contract_abis, EXPECTED_CONTRACT_ABIS);

    let worker_count = 8;
    let handles: Vec<_> = (0..worker_count)
        .map(|worker_id| {
            let unit = Arc::clone(&unit);
            let expected_file_ids = serial_file_ids.clone();

            thread::spawn(move || {
                // Exercise iterator-returning methods.
                let definitions = unit.all_definitions().count();
                let references = unit.all_references().count();
                let contract_abis = unit.compute_contracts_abi();

                assert_eq!(
                    definitions, EXPECTED_DEFINITIONS,
                    "worker {worker_id} definition count diverged"
                );
                assert_eq!(
                    references, EXPECTED_REFERENCES,
                    "worker {worker_id} reference count diverged"
                );
                assert_eq!(
                    contract_abis.len(),
                    EXPECTED_CONTRACT_ABIS,
                    "worker {worker_id} contract ABI count diverged"
                );

                // Walk the AST root for every file and exercise some accessors
                // so the AST node `Arc<…Struct>` handles are cloned across threads.
                for file_id in &expected_file_ids {
                    let root = unit
                        .file(file_id)
                        .unwrap_or_else(|| panic!("missing {file_id}"))
                        .ast();
                    let members = root.members();
                    assert!(!members.is_empty());
                    let _collected: Vec<_> = members.iter().collect();
                }

                // Exercise type resolution via references in each thread.
                for reference in unit.all_references() {
                    let _ = black_box(reference.resolve_to_definition());
                }
            })
        })
        .collect();

    for handle in handles {
        handle.join().expect("worker thread panicked");
    }
}

/// Enough files to spread over several threads, with broken and dangling ones
/// mixed in so diagnostics take part in the comparisons.
fn mixed_sources() -> Vec<(FileId, String)> {
    (0..32)
        .map(|index| {
            let file_id: FileId = format!("file{index}.sol").into();
            let contents = match index % 4 {
                // Imports the next file, which exists.
                0 => file_with_empty_contract(
                    &format!("C{index}"),
                    &[&format!("file{}.sol", index + 1)],
                ),
                // Imports a file that is not part of the compilation.
                1 => file_with_empty_contract(&format!("C{index}"), &["absent.sol"]),
                // Fails to parse, while also declaring an import.
                2 => {
                    format!("pragma solidity ^0.8.0;\nimport \"absent.sol\";\ncontract C{index} {{")
                }
                _ => file_with_empty_contract(&format!("C{index}"), &[]),
            };

            (file_id, contents)
        })
        .collect()
}

/// Borrows the sources the way [`compile`] takes them.
fn borrow_sources(sources: &[(FileId, String)]) -> impl Iterator<Item = (FileId, &str)> {
    sources
        .iter()
        .map(|(file_id, contents)| (file_id.clone(), contents.as_str()))
}

/// Everything about a unit that a caller can observe.
struct Observable {
    files: Vec<(String, NodeId)>,
    definitions: Vec<NodeId>,
    diagnostics: Vec<Diagnostic>,
}

impl Observable {
    fn from_compilation_unit(unit: &CompilationUnit) -> Self {
        Self {
            files: unit
                .files()
                .map(|file| (file.id().as_str().to_owned(), file.ast().node_id()))
                .collect(),
            definitions: unit
                .all_definitions()
                .map(|definition| definition.node_id())
                .collect(),
            // Via `iter()`, which sorts: compares the diagnostics, not their storage order.
            diagnostics: unit.diagnostics().iter().cloned().collect(),
        }
    }

    /// Compared field by field rather than as a whole, so a failure names which
    /// part diverged instead of dumping the entire unit twice.
    fn assert_same(&self, expected: &Self, context: &str) {
        assert_eq!(self.files, expected.files, "files diverged {context}");
        assert_eq!(
            self.definitions, expected.definitions,
            "definition node ids diverged {context}"
        );
        assert_eq!(
            self.diagnostics, expected.diagnostics,
            "diagnostics diverged {context}"
        );
    }
}

/// Source files are parsed in parallel, so the size of the pool must not change
/// what comes out — not the file order, not the node ids, not the diagnostics.
#[test]
fn build_output_is_independent_of_the_thread_count() {
    let sources = mixed_sources();
    let build = || compile(borrow_sources(&sources));

    let baseline = pool_of(1).install(build);

    // Guard against the comparisons below being vacuous. Half the corpus is
    // built to produce diagnostics, so requiring every one of those files to be
    // represented is what would catch a file's diagnostics going missing on the
    // way out of the parallel phase — merely requiring *some* would not.
    assert_eq!(baseline.files().count(), sources.len());
    assert!(baseline.all_definitions().next().is_some());
    let mut files_with_diagnostics: Vec<&str> = baseline
        .diagnostics()
        .iter()
        .map(|diagnostic| diagnostic.file_id().as_str())
        .collect();
    files_with_diagnostics.sort_unstable();
    files_with_diagnostics.dedup();
    assert_eq!(files_with_diagnostics.len(), sources.len() / 2);

    let baseline = Observable::from_compilation_unit(&baseline);
    for threads in [2, 4, 8, 16] {
        let unit = Observable::from_compilation_unit(&pool_of(threads).install(build));
        unit.assert_same(&baseline, &format!("on {threads} threads"));
    }
}

/// The test above varies the pool *within* one build, so it cannot catch state
/// shared more widely — a process-wide cache would stay consistent there and
/// still corrupt two builds at once. So run several against the same pool.
#[test]
fn concurrent_builds_do_not_interfere() {
    let sources = mixed_sources();
    let baseline = Observable::from_compilation_unit(&compile(borrow_sources(&sources)));

    thread::scope(|scope| {
        for worker in 0..8 {
            let sources = &sources;
            let baseline = &baseline;

            scope.spawn(move || {
                let unit = Observable::from_compilation_unit(&compile(borrow_sources(sources)));
                unit.assert_same(baseline, &format!("in worker {worker}"));
            });
        }
    });
}

fn pool_of(threads: usize) -> rayon::ThreadPool {
    rayon::ThreadPoolBuilder::new()
        .num_threads(threads)
        .build()
        .expect("thread pool builds")
}
