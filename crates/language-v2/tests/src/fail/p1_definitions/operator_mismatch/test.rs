#![allow(unused_crate_dependencies)]

language_v2_macros::compile!(Language(
    name = Foo,
    root_item = Bar,
    leading_trivia = Sequence([]),
    trailing_trivia = Sequence([]),
    versions = ["1.0.0"],
    contexts = [LexicalContext(
        name = Foo,
        sections = [Section(
            title = "Section One",
            topics = [Topic(
                title = "Topic One",
                items = [
                    Precedence(
                        name = Expression,
                        precedence_expressions = [PrecedenceExpression(
                            name = Foo,
                            operators = [
                                PrecedenceOperator(
                                    model = BinaryLeftAssociative,
                                    fields = (operator = Required(Baz1))
                                ),
                                PrecedenceOperator(
                                    model = Prefix,
                                    fields = (operator = Required(Baz1))
                                )
                            ]
                        )],
                        primary_expressions = [PrimaryExpression(reference = Baz1)]
                    ),
                    Token(name = Baz1, definitions = [TokenDefinition(Atom("baz1"))])
                ]
            )]
        )]
    )],
    built_ins = []
));

fn main() {}
