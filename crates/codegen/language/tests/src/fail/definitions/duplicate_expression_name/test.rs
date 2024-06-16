#![allow(unused_crate_dependencies)]

codegen_language_macros::compile!(Language(
    name = Foo,
    documentation_dir = "foo/bar",
    binding_rules_file = "bindings/rules.msgb",
    root_item = Bar,
    leading_trivia = Sequence([]),
    trailing_trivia = Sequence([]),
    versions = ["1.0.0"],
    sections = [Section(
        title = "Section One",
        topics = [Topic(
            title = "Topic One",
            items = [
                Precedence(
                    name = Bar,
                    precedence_expressions = [
                        PrecedenceExpression(
                            name = Expression1,
                            operators = [PrecedenceOperator(
                                model = BinaryLeftAssociative,
                                fields = (operator = Required(Baz))
                            )]
                        ),
                        PrecedenceExpression(
                            name = Expression2,
                            operators = [PrecedenceOperator(
                                model = BinaryLeftAssociative,
                                fields = (operator = Required(Baz))
                            )]
                        ),
                        PrecedenceExpression(
                            name = Expression1,
                            operators = [PrecedenceOperator(
                                model = BinaryLeftAssociative,
                                fields = (operator = Required(Baz))
                            )]
                        )
                    ],
                    primary_expressions = [PrimaryExpression(reference = Baz)]
                ),
                Token(
                    name = Baz,
                    definitions = [TokenDefinition(scanner = Atom("baz"))]
                )
            ]
        )]
    )]
));

fn main() {}
