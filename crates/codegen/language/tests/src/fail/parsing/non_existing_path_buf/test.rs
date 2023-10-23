#![allow(unused_crate_dependencies)]

codegen_language_macros::compile!(Language(
    name = Foo,
    root_item = Bar1,
    leading_trivia = Sequence([]),
    trailing_trivia = Sequence([]),
    versions = ["1.0.0", "2.0.0", "3.0.0"],
    sections = [Section(
        title = "Section One",
        topics = [
            Topic(
                title = "Topic One",
                notes_file = "README.md", // should be correct
                items = [Struct(
                    name = Bar1,
                    fields = (field = Required(NonTerminal(Bar2)))
                )]
            ),
            Topic(
                title = "Topic Two",
                notes_file = "foo/bar/non-existing-file.md",
                items = [Struct(
                    name = Bar2,
                    fields = (field = Required(NonTerminal(Bar1)))
                )]
            )
        ]
    )]
));

fn main() {}
