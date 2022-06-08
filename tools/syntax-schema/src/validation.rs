use crate::schema::Grammar;

impl Grammar {
    pub fn validate(&self) {
        // NOTE: any validation that CAN be added to the JSON Schema SHOULD be added there instead for intellisense

        // TODO: run validation against the existing JSON Schema
        // TODO: validate schema for errors that cannot be covered by the schema:
        // Possibly use custom validators: https://docs.rs/validator/

        // 1) Making sure there are no orphaned nodes by mistake.
        // 2) Making sure all referenced nodes actually exist in the grammar.
        // 3) Versions specified map to supported language versions.
        // 4) no duplicate rule names
        // 5) `topic.notes` and `topic.definition` point to the correct respective folders, and their sub-paths match.

        // .... Any other domain-specific rules we want to add later.
    }
}
