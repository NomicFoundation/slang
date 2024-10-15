pub use testlang::TestlangDefinition;

codegen_language_macros::compile!(Language(
    name = Testlang,
    documentation_dir = "crates/testlang/inputs/language/docs",
    binding_rules_file = "crates/testlang/inputs/language/bindings/rules.msgb",
    root_item = SourceUnit,
    leading_trivia = OneOrMore(Choice([
        Trivia(Whitespace),
        Trivia(EndOfLine),
        Trivia(SingleLineComment),
        Trivia(MultiLineComment)
    ])),
    trailing_trivia = Sequence([
        Optional(Trivia(Whitespace)),
        Optional(Trivia(SingleLineComment)),
        Trivia(EndOfLine)
    ]),
    versions = ["1.0.0", "1.0.1", "1.1.0", "1.1.1"],
    sections = [Section(
        title = "File Structure",
        topics = [
            Topic(
                title = "Source Unit",
                items = [
                    Struct(
                        name = SourceUnit,
                        fields = (members = Required(SourceUnitMembers))
                    ),
                    Repeated(name = SourceUnitMembers, reference = SourceUnitMember),
                    Enum(
                        name = SourceUnitMember,
                        variants = [
                            EnumVariant(reference = Tree),
                            EnumVariant(reference = Expression),
                            EnumVariant(reference = SeparatedIdentifiers),
                            EnumVariant(reference = Literal)
                        ]
                    )
                ]
            ),
            Topic(
                title = "Tree",
                lexical_context = Tree,
                items = [
                    Struct(
                        name = Tree,
                        error_recovery = FieldsErrorRecovery(terminator = semicolon),
                        fields = (
                            keyword = Required(TreeKeyword),
                            name = Optional(reference = Identifier),
                            node = Required(TreeNode),
                            semicolon = Required(Semicolon)
                        )
                    ),
                    Struct(
                        name = TreeNode,
                        error_recovery = FieldsErrorRecovery(
                            delimiters =
                                FieldDelimiters(open = open_bracket, close = close_bracket)
                        ),
                        fields = (
                            open_bracket = Required(OpenBracket),
                            members = Required(TreeNodeChildren),
                            close_bracket = Required(CloseBracket)
                        )
                    ),
                    Repeated(name = TreeNodeChildren, reference = TreeNodeChild),
                    Enum(
                        name = TreeNodeChild,
                        variants = [
                            EnumVariant(reference = TreeNode),
                            EnumVariant(reference = DelimitedIdentifier)
                        ]
                    ),
                    Keyword(
                        name = TreeKeyword,
                        identifier = Identifier,
                        definitions = [KeywordDefinition(value = Atom("tree"))]
                    )
                ]
            ),
            Topic(
                title = "DummyToAvoidWarnings",
                items = [
                    Precedence(
                        name = Expression,
                        precedence_expressions = [
                            PrecedenceExpression(
                                name = AdditionExpression,
                                operators = [PrecedenceOperator(
                                    model = BinaryLeftAssociative,
                                    fields = (operator = Required(Plus))
                                )]
                            ),
                            PrecedenceExpression(
                                name = NegationExpression,
                                operators = [PrecedenceOperator(
                                    model = Prefix,
                                    fields = (operator = Required(Bang))
                                )]
                            ),
                            PrecedenceExpression(
                                name = MemberAccessExpression,
                                operators = [PrecedenceOperator(
                                    model = Postfix,
                                    fields =
                                        (period = Required(Period), member = Required(Identifier))
                                )]
                            )
                        ],
                        primary_expressions = [
                            PrimaryExpression(reference = StringLiteral),
                            PrimaryExpression(reference = Identifier)
                        ]
                    ),
                    Separated(
                        name = SeparatedIdentifiers,
                        reference = Identifier,
                        separator = Period,
                        enabled = From("1.0.0")
                    )
                ]
            ),
            Topic(
                title = "Literals",
                items = [
                    Enum(
                        name = Literal,
                        variants = [EnumVariant(reference = StringLiteral)]
                    ),
                    Token(
                        name = StringLiteral,
                        definitions = [TokenDefinition(
                            scanner = Sequence([
                                Atom("\""),
                                ZeroOrMore(Choice([
                                    Fragment(EscapeSequence),
                                    Not(['"', '\\', '\r', '\n'])
                                ])),
                                Atom("\"")
                            ])
                        )]
                    ),
                    Fragment(
                        name = EscapeSequence,
                        scanner = Sequence([
                            Atom("\\"),
                            Choice([
                                Fragment(AsciiEscape),
                                Fragment(HexByteEscape),
                                Fragment(UnicodeEscape)
                            ])
                        ])
                    ),
                    Fragment(
                        name = AsciiEscape,
                        scanner = Choice([
                            Atom("n"),
                            Atom("r"),
                            Atom("t"),
                            Atom("'"),
                            Atom("\""),
                            Atom("\\"),
                            Atom("\n"),
                            Atom("\r")
                        ])
                    ),
                    Fragment(
                        name = HexByteEscape,
                        scanner =
                            Sequence([Atom("x"), Fragment(HexCharacter), Fragment(HexCharacter)])
                    ),
                    Fragment(
                        name = UnicodeEscape,
                        scanner = Sequence([
                            Atom("u"),
                            Fragment(HexCharacter),
                            Fragment(HexCharacter),
                            Fragment(HexCharacter),
                            Fragment(HexCharacter)
                        ])
                    ),
                    Fragment(
                        name = HexCharacter,
                        scanner = Choice([
                            Range(inclusive_start = '0', inclusive_end = '9'),
                            Range(inclusive_start = 'a', inclusive_end = 'f'),
                            Range(inclusive_start = 'A', inclusive_end = 'F')
                        ])
                    )
                ]
            ),
            Topic(
                title = "Identifiers",
                items = [
                    Token(
                        name = Identifier,
                        definitions = [TokenDefinition(scanner = Fragment(RawIdentifier))]
                    ),
                    Fragment(
                        name = RawIdentifier,
                        scanner = Sequence([
                            Fragment(IdentifierStart),
                            ZeroOrMore(Fragment(IdentifierPart))
                        ])
                    ),
                    Fragment(
                        name = IdentifierStart,
                        scanner = Choice([
                            Atom("_"),
                            Atom("$"),
                            Range(inclusive_start = 'a', inclusive_end = 'z'),
                            Range(inclusive_start = 'A', inclusive_end = 'Z')
                        ])
                    ),
                    Fragment(
                        name = IdentifierPart,
                        scanner = Choice([
                            Fragment(IdentifierStart),
                            Range(inclusive_start = '0', inclusive_end = '9')
                        ])
                    ),
                    Token(
                        name = DelimitedIdentifier,
                        definitions = [TokenDefinition(
                            scanner = Sequence([
                                Fragment(DelimitedIdentifierStart),
                                ZeroOrMore(Fragment(DelimitedIdentifierPart))
                            ])
                        )]
                    ),
                    Fragment(
                        name = DelimitedIdentifierStart,
                        scanner = Choice([Range(inclusive_start = 'A', inclusive_end = 'Z')])
                    ),
                    Fragment(
                        name = DelimitedIdentifierPart,
                        scanner = Choice([
                            Atom("_"),
                            Range(inclusive_start = 'a', inclusive_end = 'z'),
                            Range(inclusive_start = '0', inclusive_end = '9')
                        ])
                    )
                ]
            ),
            Topic(
                title = "Punctuation",
                items = [
                    Token(
                        name = Bang,
                        definitions = [TokenDefinition(scanner = Atom("!"))]
                    ),
                    Token(
                        name = OpenBracket,
                        definitions = [TokenDefinition(scanner = Atom("["))]
                    ),
                    Token(
                        name = CloseBracket,
                        definitions = [TokenDefinition(scanner = Atom("]"))]
                    ),
                    Token(
                        name = Period,
                        definitions = [TokenDefinition(scanner = Atom("."))]
                    ),
                    Token(
                        name = Plus,
                        definitions = [TokenDefinition(scanner = Atom("+"))]
                    ),
                    Token(
                        name = Semicolon,
                        definitions = [TokenDefinition(scanner = Atom(";"))]
                    )
                ]
            ),
            Topic(
                title = "Trivia",
                items = [
                    Trivia(
                        name = Whitespace,
                        scanner = OneOrMore(Choice([Atom(" "), Atom("\t")]))
                    ),
                    Trivia(
                        name = EndOfLine,
                        scanner = Sequence([Optional(Atom("\r")), Atom("\n")])
                    ),
                    Trivia(
                        name = SingleLineComment,
                        scanner = Sequence([
                            TrailingContext(scanner = Atom("//"), not_followed_by = Atom("/")),
                            ZeroOrMore(Not(['\r', '\n']))
                        ])
                    ),
                    Trivia(
                        name = MultiLineComment,
                        scanner = Sequence([
                            TrailingContext(scanner = Atom("/*"), not_followed_by = Atom("*")),
                            ZeroOrMore(Choice([
                                Not(['*']),
                                TrailingContext(scanner = Atom("*"), not_followed_by = Atom("/"))
                            ])),
                            Atom("*/")
                        ])
                    )
                ]
            )
        ]
    )],
    built_ins = []
));
