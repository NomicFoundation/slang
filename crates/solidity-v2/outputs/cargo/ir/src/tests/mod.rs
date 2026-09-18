/// Extracts the payload of an enum variant, panicking with a descriptive
/// message if the value matches a different variant.
///
/// Works on owned values and references alike (via match ergonomics), so
/// `expect_variant!(&foo, my::Enum::Variant)` yields a reference to the payload.
macro_rules! expect_variant {
    ($value:expr, $variant:path) => {
        match $value {
            $variant(inner) => inner,
            other => panic!(
                concat!("expected ", stringify!($variant), ", got {:?}"),
                other
            ),
        }
    };
}

mod builder;
mod text_range;
mod version_pragma;
mod visitor;

/// The generator for the first group of a fresh id space, for the tests that
/// build a single file and so only ever need the one.
fn single_file_id_generator() -> crate::ir::NodeIdGenerator {
    crate::ir::NodeIdGroups::default()
        .next()
        .expect("a fresh id space has a first group")
}
