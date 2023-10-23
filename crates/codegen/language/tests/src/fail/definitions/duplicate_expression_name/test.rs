#![allow(unused_crate_dependencies)]

codegen_language_macros::compile!(Language(
    name = Foo,
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
                        PrecedenceExpression(name = Expression1, operators = []),
                        PrecedenceExpression(name = Expression2, operators = []),
                        PrecedenceExpression(name = Expression1, operators = [])
                    ],
                    default_primary_expression = Baz,
                    primary_expressions = [PrimaryExpression(expression = Baz)]
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
