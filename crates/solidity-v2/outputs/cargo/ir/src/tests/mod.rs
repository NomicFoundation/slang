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
mod visitor;
