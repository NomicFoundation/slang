use crate::schema::Grammar;

pub fn validate(_grammar: &Grammar) {
    // NOTE: any validation that CAN be added to the JSON Schema SHOULD be added there instead for intellisense

    // TODO: run validation against the existing JSON Schema
    // TODO: validate schema for errors that cannot be covered by the schema:
    // Possibly use custom validators: https://docs.rs/validator/

    // 1) Making sure there are no orphaned nodes by mistake.
    // 2) Making sure all referenced nodes actually exist in the grammar.
    // 3) Versions specified map to supported language versions.
    // 4) Ranges have valid from/to values, and map to valid Solidity characters.
    // 5) Make sure top-level rules have a name, so that we can refer to them from other rules.
    // 6) "Not" has a valid/supported character
    // 7) version breaks are constrained to top level nodes only

    // .... Any other domain-specific rules we want to add later.
}
