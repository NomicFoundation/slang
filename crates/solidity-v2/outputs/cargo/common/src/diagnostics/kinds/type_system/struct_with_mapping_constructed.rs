use serde::Serialize;

use crate::diagnostics::extensions::DiagnosticExtensions;
use crate::diagnostics::severity::DiagnosticSeverity;

/// A struct that contains a mapping, directly or through its members, is
/// constructed, eg. `S(1)` for `struct S { uint a; mapping(uint => uint) m; }`.
/// A mapping can only live in storage, so such a struct has no memory value.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct StructWithMappingConstructed;

impl DiagnosticExtensions for StructWithMappingConstructed {
    fn severity(&self) -> DiagnosticSeverity {
        DiagnosticSeverity::Error
    }

    fn code(&self) -> &'static str {
        "type-system/struct-with-mapping-constructed"
    }

    fn message(&self) -> String {
        "A struct containing a mapping cannot be constructed.".to_owned()
    }
}
