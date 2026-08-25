pub mod text_range;

#[path = "nodes.generated.rs"]
pub mod nodes;

pub use text_range::*;

/// All cst nodes should be [`Send`] so that we can move the tree across threads.
/// We only assert [`nodes::SourceUnit`] as the root of the structure.
const _: () = {
    const fn assert_send<T: Send>() {}

    assert_send::<nodes::SourceUnit>();
};
