/// Defines a diagnostic kind enum whose variants each wrap a single type,
/// and implements [`crate::diagnostics::extensions::DiagnosticExtensions`],
/// delegating its methods to the wrapped variants.
#[macro_export]
macro_rules! define_diagnostic_kind {
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

        impl $crate::diagnostics::DiagnosticExtensions for $name {
            fn severity(&self) -> $crate::diagnostics::DiagnosticSeverity {
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
    };
}
