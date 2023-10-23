#![allow(unused_crate_dependencies)]

codegen_language_macros::compile!(
    name = Foo,
    root_item = Bar1,
    leading_trivia = Sequence([]),
    trailing_trivia = Sequence([]),
    versions = ["1.0.0"],
    sections = [Section(
        title = "Section One",
        topics = [Topic(
            title = "Topic One",
            items = [
                Struct(
                    name = Bar1,
                    fields = (
                        field1 = Required(NonTerminal(Bar2)),
                        field2 = Required(NonTerminal(Bar3))
                    )
                ),
                Precedence(
                    name = Bar2,
                    operators = [
                        Operator(
                            expression_name = Op1,
                            model = Prefix,
                            fields = (operator = Required(NonTerminal(Bar2)))
                        ),
                        Operator(
                            expression_name = Op1,
                            model = Prefix,
                            fields = (operator = Required(NonTerminal(Bar2)))
                        )
                    ],
                    primary_expressions = []
                ),
                Precedence(
                    name = Bar3,
                    operators = [
                        Operator(
                            expression_name = Op1,
                            model = Prefix,
                            fields = (operator = Required(NonTerminal(Bar3)))
                        ),
                        Operator(
                            expression_name = Op2,
                            model = Prefix,
                            fields = (operator = Required(NonTerminal(Bar3)))
                        )
                    ],
                    primary_expressions = []
                )
            ]
        )]
    )]
);

fn main() {}
