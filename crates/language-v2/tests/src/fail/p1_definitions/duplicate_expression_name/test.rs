#![allow(unused_crate_dependencies)]

language_v2_macros::compile!(Language(
    name = Foo,
    binding_rules_file = "bindings/rules.msgb",
    root_item = Bar,
    leading_trivia = Sequence([]),
    trailing_trivia = Sequence([]),
    versions = ["1.0.0"],
    sections = [Section(
        title = "Section One",
        topics = [Topic(
            title = "Topic One",
            lexical_context = Foo,
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
                Token(name = Baz, definitions = [TokenDefinition(Atom("baz"))])
            ]
        )]
    )],
    built_ins = []
));

fn main() {}
