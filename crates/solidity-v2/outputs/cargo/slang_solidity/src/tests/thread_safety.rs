use std::hint::black_box;
use std::sync::Arc;
use std::thread;

use crate::tests::fixtures;

#[test]
fn parallel_access_via_arc_consistent_with_serial_baseline() {
    const EXPECTED_DEFINITIONS: usize = 22;
    const EXPECTED_REFERENCES: usize = 48;
    const EXPECTED_CONTRACT_ABIS: usize = 1;

    let unit = fixtures::Counter::build_compilation_unit();

    let serial_definitions = unit.all_definitions().count();
    let serial_references = unit.all_references().count();
    let serial_contract_abis = unit.compute_contracts_abi().len();
    let serial_file_ids: Vec<_> = unit.file_ids();

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
                        .get_file_ast_root(file_id)
                        .unwrap_or_else(|| panic!("missing AST root for {file_id}"));
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
