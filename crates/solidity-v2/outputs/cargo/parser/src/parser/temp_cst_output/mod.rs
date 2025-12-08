//! TODO(v2): This temporary API is used to test (sort of) arbitrary non terminals
//! against the previous parser.
//! We shouldn't support arbitrary non terminal parsing in the future, the LR parser
//! generated is just too big (~10M LOC).

#[path = "cursor_checker.generated.rs"]
pub mod cursor_checker;
