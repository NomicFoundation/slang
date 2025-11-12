// Takes a vec of strings and a slice of str and checks they are permutations one another.
// Used in tests.
macro_rules! assert_eq_defs {
    ($xs:expr, $ys:expr) => {
        assert_eq!(
            $xs.iter()
                .map(|def| def.identifier().unparse())
                .collect::<std::collections::HashSet<String>>(),
            $ys.iter().map(|s| (*s).to_string()).collect()
        );
    };
}

mod collect_definitions;
mod find_unused_definitions;
mod follow_all_references;
mod pipeline;
mod test_find_unused_definitions;
mod visit_definition;
