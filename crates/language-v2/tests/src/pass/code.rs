use language_v2_definition::model::{Item, ParserOptions};

language_v2_macros::compile!(Language(
    name = CodeTest,
    root_item = Foo,
    versions = ["1.0.0"],
    contexts = [LexicalContext(
        name = CodeTest,
        sections = [Section(
            title = "S",
            topics = [Topic(
                title = "T",
                items = [
                    Struct(
                        name = Foo,
                        fields = (bar = Required(Bar)),
                        parser_options = ParserOptions(
                            inline = false,
                            verbatim = Code(
                                Foo: Foo = {
                                    <bar: Bar> => new_foo(bar),
                                };
                            )
                        )
                    ),
                    Token(name = Bar, scanner = Atom("bar"))
                ]
            )]
        )]
    )]
));

#[test]
fn captures_body_tokens() {
    let lang = code_test::CodeTestDefinition::create();

    // Navigate to Foo's parser_options.verbatim. Foo is the first item of the first topic.
    let Item::Struct { item } = &lang.contexts[0].sections[0].topics[0].items[0] else {
        panic!("expected first item to be a Struct");
    };
    let Some(ParserOptions {
        verbatim: Some(code),
        ..
    }) = &item.parser_options
    else {
        panic!("expected parser_options.verbatim to be Some");
    };
    let text = code.to_string();

    // Body tokens round-trip (whitespace is normalized by TokenStream::to_string, so
    // spacing between individual tokens is not guaranteed — we just check key fragments
    // appear in the body).
    for fragment in ["Foo", "Bar", "new_foo", "bar"] {
        assert!(
            text.contains(fragment),
            "missing {fragment:?} in body, got: {text:?}",
        );
    }

    // Outer delimiter chars are not in the captured string (they are the DSL delimiters,
    // consumed by `parse_named_value` before the body parser runs).
    assert!(!text.starts_with('('));
    assert!(!text.trim_end().ends_with(')'));
}
