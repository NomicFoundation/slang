use std::collections::BTreeSet;
use std::sync::Arc;

use metaslang_bindings::PathResolver;
use semver::Version;
use slang_solidity::bindings::{self, Bindings};
use slang_solidity::cst::Node;
use slang_solidity::kinds::{EdgeLabel, NonterminalKind};
use slang_solidity::language::Language;
use slang_solidity::query::Query;
use slang_solidity::text_index::TextIndex;

const SOLC_VERSION: Version = Version::new(0, 8, 20);

const SOURCES: &[&str] = &[
    include_str!("./sources/EnumerableMap.sol"),
    include_str!("./sources/ERC20.sol"),
    include_str!("./sources/ERC721.sol"),
    include_str!("./sources/Governor.sol"),
    include_str!("./sources/SafeCast.sol"),
];

const FUNCTION_NAMES: &[&str] = &[
    "CLOCK_MODE",
    "_approve",
    "_baseURI",
    "_burn",
    "_cancel",
    "_castVote",
    "_checkAuthorized",
    "_checkGovernance",
    "_checkOnERC721Received",
    "_countVote",
    "_defaultParams",
    "_encodeStateBitmap",
    "_executeOperations",
    "_executor",
    "_getApproved",
    "_getVotes",
    "_increaseBalance",
    "_isAuthorized",
    "_isValidDescriptionForProposer",
    "_mint",
    "_ownerOf",
    "_propose",
    "_queueOperations",
    "_quorumReached",
    "_requireOwned",
    "_safeMint",
    "_safeTransfer",
    "_setApprovalForAll",
    "_spendAllowance",
    "_transfer",
    "_tryHexToUint",
    "_update",
    "_validateStateBitmap",
    "_voteSucceeded",
    "allowance",
    "approve",
    "at",
    "balanceOf",
    "cancel",
    "castVote",
    "castVoteBySig",
    "castVoteWithReason",
    "castVoteWithReasonAndParams",
    "castVoteWithReasonAndParamsBySig",
    "clock",
    "contains",
    "decimals",
    "execute",
    "get",
    "getApproved",
    "getVotes",
    "getVotesWithParams",
    "hashProposal",
    "isApprovedForAll",
    "keys",
    "length",
    "name",
    "onERC1155BatchReceived",
    "onERC1155Received",
    "onERC721Received",
    "ownerOf",
    "proposalDeadline",
    "proposalEta",
    "proposalNeedsQueuing",
    "proposalProposer",
    "proposalSnapshot",
    "proposalThreshold",
    "propose",
    "queue",
    "quorum",
    "relay",
    "remove",
    "safeTransferFrom",
    "set",
    "setApprovalForAll",
    "state",
    "supportsInterface",
    "symbol",
    "toInt104",
    "toInt112",
    "toInt120",
    "toInt128",
    "toInt136",
    "toInt144",
    "toInt152",
    "toInt16",
    "toInt160",
    "toInt168",
    "toInt176",
    "toInt184",
    "toInt192",
    "toInt200",
    "toInt208",
    "toInt216",
    "toInt224",
    "toInt232",
    "toInt24",
    "toInt240",
    "toInt248",
    "toInt256",
    "toInt32",
    "toInt40",
    "toInt48",
    "toInt56",
    "toInt64",
    "toInt72",
    "toInt8",
    "toInt80",
    "toInt88",
    "toInt96",
    "toUint104",
    "toUint112",
    "toUint120",
    "toUint128",
    "toUint136",
    "toUint144",
    "toUint152",
    "toUint16",
    "toUint160",
    "toUint168",
    "toUint176",
    "toUint184",
    "toUint192",
    "toUint200",
    "toUint208",
    "toUint216",
    "toUint224",
    "toUint232",
    "toUint24",
    "toUint240",
    "toUint248",
    "toUint256",
    "toUint32",
    "toUint40",
    "toUint48",
    "toUint56",
    "toUint64",
    "toUint72",
    "toUint8",
    "toUint80",
    "toUint88",
    "toUint96",
    "tokenURI",
    "totalSupply",
    "transfer",
    "transferFrom",
    "tryGet",
    "version",
    "votingDelay",
    "votingPeriod",
];

pub fn run_parser() -> Vec<Node> {
    let language = Language::new(SOLC_VERSION).unwrap();

    let mut trees = vec![];

    for source in SOURCES {
        let parse_output = language.parse(Language::ROOT_KIND, source);

        assert!(
            parse_output.is_valid(),
            "Found parse errors:\n{0:#?}",
            parse_output.errors(),
        );

        trees.push(parse_output.tree());
    }

    trees
}

pub fn run_cursor(trees: &Vec<Node>) {
    let mut results = BTreeSet::new();

    for tree in trees {
        let mut cursor = tree.cursor_with_offset(TextIndex::ZERO);

        while cursor.go_to_next_nonterminal_with_kind(NonterminalKind::FunctionDefinition) {
            results.extend(
                cursor
                    .node()
                    .children()
                    .iter()
                    .filter(|edge| edge.label == Some(EdgeLabel::Name))
                    .map(|edge| edge.node.clone().unparse().trim().to_owned()),
            );
        }
    }

    assert!(
        results.iter().eq(FUNCTION_NAMES.iter()),
        "Function names don't match: {results:#?}"
    );
}

pub fn run_query(trees: &Vec<Node>) {
    let mut results = BTreeSet::new();

    let queries = vec![Query::parse(
        "[FunctionDefinition
            @name name: [_]
        ]",
    )
    .unwrap()];

    for tree in trees {
        let cursor = tree.cursor_with_offset(TextIndex::ZERO);

        for query_match in cursor.query(queries.clone()) {
            assert_eq!(query_match.captures.len(), 1);

            results.extend(
                query_match.captures["name"]
                    .iter()
                    .map(|cursor| cursor.node().unparse().trim().to_owned()),
            );
        }
    }

    assert!(
        results.iter().eq(FUNCTION_NAMES.iter()),
        "Function names don't match: {results:#?}"
    );
}

struct NoOpResolver;

impl PathResolver for NoOpResolver {
    fn resolve_path(&self, _context_path: &str, path_to_resolve: &str) -> Option<String> {
        Some(path_to_resolve.to_string())
    }
}

pub fn run_create_bindings() {
    let _ = bindings::create_with_resolver(SOLC_VERSION, Arc::new(NoOpResolver {}));
}

pub fn run_bindings(trees: &Vec<Node>) -> Vec<Bindings> {
    let mut result = vec![];
    let mut definition_count = 0_usize;

    for tree in trees {
        let mut bindings = bindings::create_with_resolver(SOLC_VERSION, Arc::new(NoOpResolver {}));
        bindings.add_file("input.sol", tree.cursor_with_offset(TextIndex::ZERO));
        definition_count += bindings.all_definitions().count();
        result.push(bindings);
    }

    // 723 definitions
    println!("A total of {definition_count} definitions were found");

    result
}

pub fn prepare_bindings() -> Vec<Bindings> {
    let trees = run_parser();
    run_bindings(&trees)
}

pub fn run_resolve_references(bindings_vec: &Vec<Bindings>) {
    let mut reference_count = 0_usize;
    let mut resolved_references = 0_usize;

    for bindings in bindings_vec {
        for reference in bindings.all_references() {
            reference_count += 1;
            let resolution = reference.jump_to_definition();
            if resolution.is_ok() {
                resolved_references += 1;
            }
        }
    }

    // 1491 references, 1170 resolved
    println!("Out of a total of {reference_count} references found, {resolved_references} were unambiguously resolved");
}
