/// Defines a diagnostic kind enum whose variants each wrap a single type.
/// Generates the enum, an `impl From<Inner> for Self` for each variant, and an
/// [`crate::diagnostics::extensions::DiagnosticExtensions`] impl that
/// delegates to the wrapped variants.
///
/// The optional `parent_kind = ParentEnum::Variant;` prefix marks this as an
/// intermediate kind wrapped by a top-level one. When present, an additional
/// `impl From<Inner> for ParentEnum` is emitted for every variant — wrapping
/// the leaf in the named `ParentEnum::Variant` — so leaf types flow directly
/// into the top-level kind via `.into()`. The arm delegates to the base arm
/// for the shared expansion.
macro_rules! define_diagnostic_kind {
    (
        parent_kind = $parent_enum:ident :: $parent_variant:ident ;

        $(#[$enum_attr:meta])*
        pub enum $name:ident {
            $(
                $(#[$variant_attr:meta])*
                $variant:ident($inner:ty),
            )+
        }
    ) => {
        $crate::diagnostics::kinds::utils::define_diagnostic_kind! {
            $(#[$enum_attr])*
            pub enum $name {
                $(
                    $(#[$variant_attr])*
                    $variant($inner),
                )+
            }
        }

        $(
            impl ::std::convert::From<$inner> for $parent_enum {
                fn from(d: $inner) -> Self {
                    $parent_enum::$parent_variant($name::$variant(d))
                }
            }
        )+
    };

    (
        $(#[$enum_attr:meta])*
        pub enum $name:ident {
            $(
                $(#[$variant_attr:meta])*
                $variant:ident($inner:ty),
            )+
        }
    ) => {
        $(#[$enum_attr])*
        pub enum $name {
            $(
                $(#[$variant_attr])*
                $variant($inner),
            )+
        }

        impl $crate::diagnostics::extensions::DiagnosticExtensions for $name {
            fn severity(&self) -> $crate::diagnostics::severity::DiagnosticSeverity {
                match self {
                    $( Self::$variant(d) => d.severity(), )+
                }
            }

            fn code(&self) -> &'static str {
                match self {
                    $( Self::$variant(d) => d.code(), )+
                }
            }

            fn message(&self) -> ::std::string::String {
                match self {
                    $( Self::$variant(d) => d.message(), )+
                }
            }
        }

        $(
            impl ::std::convert::From<$inner> for $name {
                fn from(d: $inner) -> Self {
                    Self::$variant(d)
                }
            }
        )+
    };
}

pub(crate) use define_diagnostic_kind;
