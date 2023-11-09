pub use solidity::SolidityDefinition;

codegen_language_macros::compile!(Language(
    name = Solidity,
    root_item = SourceUnit,
    leading_trivia = ZeroOrMore(Choice([
        Trivia(Whitespace),
        Trivia(EndOfLine),
        Trivia(SingleLineComment),
        Trivia(MultilineComment)
    ])),
    trailing_trivia = Sequence([
        ZeroOrMore(Choice([
            Trivia(Whitespace),
            Trivia(SingleLineComment),
            Trivia(MultilineComment)
        ])),
        Choice([Trivia(EndOfLine), EndOfInput])
    ]),
    versions = [
        "0.4.11", "0.4.12", "0.4.13", "0.4.14", "0.4.15", "0.4.16", "0.4.17", "0.4.18", "0.4.19",
        "0.4.20", "0.4.21", "0.4.22", "0.4.23", "0.4.24", "0.4.25", "0.4.26", "0.5.0", "0.5.1",
        "0.5.2", "0.5.3", "0.5.4", "0.5.5", "0.5.6", "0.5.7", "0.5.8", "0.5.9", "0.5.10", "0.5.11",
        "0.5.12", "0.5.13", "0.5.14", "0.5.15", "0.5.16", "0.5.17", "0.6.0", "0.6.1", "0.6.2",
        "0.6.3", "0.6.4", "0.6.5", "0.6.6", "0.6.7", "0.6.8", "0.6.9", "0.6.10", "0.6.11",
        "0.6.12", "0.7.0", "0.7.1", "0.7.2", "0.7.3", "0.7.4", "0.7.5", "0.7.6", "0.8.0", "0.8.1",
        "0.8.2", "0.8.3", "0.8.4", "0.8.5", "0.8.6", "0.8.7", "0.8.8", "0.8.9", "0.8.10", "0.8.11",
        "0.8.12", "0.8.13", "0.8.14", "0.8.15", "0.8.16", "0.8.17", "0.8.18", "0.8.19"
    ],
    sections = [
        Section(
            title = "File Structure",
            topics = [
                Topic(
                    title = "Source Unit",
                    items = [
                        Struct(
                            name = SourceUnit,
                            fields = (members = Required(NonTerminal(SourceUnitMembers)))
                        ),
                        Repeated(
                            name = SourceUnitMembers,
                            repeated = SourceUnitMember,
                            allow_empty = true
                        ),
                        Enum(
                            name = SourceUnitMember,
                            variants = [
                                EnumVariant(name = Pragma, reference = PragmaDirective),
                                EnumVariant(name = Import, reference = ImportDirective),
                                EnumVariant(name = Contract, reference = ContractDefinition),
                                EnumVariant(name = Interface, reference = InterfaceDefinition),
                                EnumVariant(name = Library, reference = LibraryDefinition),
                                EnumVariant(
                                    name = Struct,
                                    enabled = From("0.6.0"),
                                    reference = StructDefinition
                                ),
                                EnumVariant(
                                    name = Enum,
                                    enabled = From("0.6.0"),
                                    reference = EnumDefinition
                                ),
                                EnumVariant(
                                    name = Function,
                                    enabled = From("0.7.1"),
                                    reference = FunctionDefinition
                                ),
                                EnumVariant(
                                    name = Constant,
                                    enabled = From("0.7.4"),
                                    reference = ConstantDefinition
                                ),
                                EnumVariant(
                                    name = Error,
                                    enabled = From("0.8.4"),
                                    reference = ErrorDefinition
                                ),
                                EnumVariant(
                                    name = UserDefinedValueType,
                                    enabled = From("0.8.8"),
                                    reference = UserDefinedValueTypeDefinition
                                ),
                                EnumVariant(
                                    name = Using,
                                    enabled = From("0.8.13"),
                                    reference = UsingDirective
                                )
                            ]
                        )
                    ]
                ),
                Topic(
                    title = "Pragma Directives",
                    lexical_context = Pragma,
                    items = [
                        Struct(
                            name = PragmaDirective,
                            error_recovery = FieldsErrorRecovery(terminator = semicolon),
                            fields = (
                                pragma_keyword = Required(Terminal([PragmaKeyword])),
                                pragma = Required(NonTerminal(Pragma)),
                                semicolon = Required(Terminal([Semicolon]))
                            )
                        ),
                        Enum(
                            name = Pragma,
                            variants = [
                                EnumVariant(name = ABICoder, reference = ABICoderPragma),
                                EnumVariant(name = Experimental, reference = ExperimentalPragma),
                                EnumVariant(name = Version, reference = VersionPragma)
                            ]
                        ),
                        Struct(
                            name = ABICoderPragma,
                            fields = (
                                abicoder_keyword = Required(Terminal([AbicoderKeyword])),
                                version = Required(Terminal([Identifier]))
                            )
                        ),
                        Struct(
                            name = ExperimentalPragma,
                            fields = (
                                experimental_keyword = Required(Terminal([ExperimentalKeyword])),
                                feature = Required(NonTerminal(ExperimentalFeature))
                            )
                        ),
                        Enum(
                            name = ExperimentalFeature,
                            variants = [
                                EnumVariant(name = Identifier, reference = Identifier),
                                EnumVariant(name = String, reference = AsciiStringLiteral)
                            ]
                        ),
                        Struct(
                            name = VersionPragma,
                            fields = (
                                solidity_keyword = Required(Terminal([SolidityKeyword])),
                                expressions = Required(NonTerminal(VersionPragmaExpressions))
                            )
                        ),
                        Repeated(
                            name = VersionPragmaExpressions,
                            repeated = VersionPragmaExpression
                        ),
                        Precedence(
                            name = VersionPragmaExpression,
                            precedence_expressions = [
                                PrecedenceExpression(
                                    name = VersionPragmaOrExpression,
                                    operators = [PrecedenceOperator(
                                        model = BinaryLeftAssociative,
                                        fields = (operator = Required(Terminal([BarBar])))
                                    )]
                                ),
                                PrecedenceExpression(
                                    name = VersionPragmaRangeExpression,
                                    operators = [PrecedenceOperator(
                                        model = BinaryLeftAssociative,
                                        fields = (operator = Required(Terminal([Minus])))
                                    )]
                                ),
                                PrecedenceExpression(
                                    name = VersionPragmaPrefixExpression,
                                    operators = [PrecedenceOperator(
                                        model = Prefix,
                                        fields = (operator = Required(Terminal([
                                            Caret,
                                            Tilde,
                                            Equal,
                                            LessThan,
                                            GreaterThan,
                                            LessThanEqual,
                                            GreaterThanEqual
                                        ])))
                                    )]
                                )
                            ],
                            primary_expressions =
                                [PrimaryExpression(expression = VersionPragmaSpecifier)]
                        ),
                        Separated(
                            name = VersionPragmaSpecifier,
                            separated = VersionPragmaValue,
                            separator = Period
                        ),
                        Token(
                            name = VersionPragmaValue,
                            definitions = [TokenDefinition(
                                scanner = OneOrMore(Choice([
                                    Range(inclusive_start = '0', inclusive_end = '9'),
                                    Atom("x"),
                                    Atom("X"),
                                    Atom("*")
                                ]))
                            )]
                        )
                    ]
                ),
                Topic(
                    title = "Import Directives",
                    items = [
                        Struct(
                            name = ImportDirective,
                            error_recovery = FieldsErrorRecovery(terminator = semicolon),
                            fields = (
                                import_keyword = Required(Terminal([ImportKeyword])),
                                symbol = Required(NonTerminal(ImportSymbol)),
                                semicolon = Required(Terminal([Semicolon]))
                            )
                        ),
                        Enum(
                            name = ImportSymbol,
                            variants = [
                                EnumVariant(name = Path, reference = PathImportSymbol),
                                EnumVariant(name = Named, reference = NamedImportSymbol),
                                EnumVariant(
                                    name = Deconstruction,
                                    reference = ImportSymbolDeconstruction
                                )
                            ]
                        ),
                        Struct(
                            name = PathImportSymbol,
                            fields = (
                                path = Required(NonTerminal(AsciiStringLiteral)),
                                alias = Optional(kind = NonTerminal(ImportAlias))
                            )
                        ),
                        Struct(
                            name = NamedImportSymbol,
                            fields = (
                                asterisk = Required(Terminal([Asterisk])),
                                alias = Required(NonTerminal(ImportAlias)),
                                from_keyword = Required(Terminal([FromKeyword])),
                                path = Required(NonTerminal(AsciiStringLiteral))
                            )
                        ),
                        Struct(
                            name = ImportSymbolDeconstruction,
                            error_recovery = FieldsErrorRecovery(
                                delimiters =
                                    FieldDelimiters(open = open_brace, close = close_brace)
                            ),
                            fields = (
                                open_brace = Required(Terminal([OpenBrace])),
                                fields = Required(NonTerminal(ImportDeconstructionFields)),
                                close_brace = Required(Terminal([CloseBrace])),
                                from_keyword = Required(Terminal([FromKeyword])),
                                path = Required(NonTerminal(AsciiStringLiteral))
                            )
                        ),
                        Separated(
                            name = ImportDeconstructionFields,
                            separated = ImportDeconstructionField,
                            separator = Comma,
                            allow_empty = true
                        ),
                        Struct(
                            name = ImportDeconstructionField,
                            fields = (
                                name = Required(Terminal([Identifier])),
                                alias = Optional(kind = NonTerminal(ImportAlias))
                            )
                        ),
                        Struct(
                            name = ImportAlias,
                            fields = (
                                as_keyword = Required(Terminal([AsKeyword])),
                                identifier = Required(Terminal([Identifier]))
                            )
                        )
                    ]
                ),
                Topic(
                    title = "Using Directives",
                    items = [
                        Struct(
                            name = UsingDirective,
                            error_recovery = FieldsErrorRecovery(terminator = semicolon),
                            fields = (
                                using_keyword = Required(Terminal([UsingKeyword])),
                                symbol = Required(NonTerminal(UsingSymbol)),
                                for_keyword = Required(Terminal([ForKeyword])),
                                target = Required(NonTerminal(UsingTarget)),
                                global_keyword = Optional(
                                    kind = Terminal([GlobalKeyword]),
                                    enabled = From("0.8.13")
                                ),
                                semicolon = Required(Terminal([Semicolon]))
                            )
                        ),
                        Enum(
                            name = UsingSymbol,
                            variants = [
                                EnumVariant(name = Path, reference = IdentifierPath),
                                EnumVariant(
                                    name = Deconstruction,
                                    enabled = From("0.8.13"),
                                    reference = UsingSymbolDeconstruction
                                )
                            ]
                        ),
                        Struct(
                            name = UsingSymbolDeconstruction,
                            enabled = From("0.8.13"),
                            error_recovery = FieldsErrorRecovery(
                                delimiters =
                                    FieldDelimiters(open = open_brace, close = close_brace)
                            ),
                            fields = (
                                open_brace = Required(Terminal([OpenBrace])),
                                fields = Required(NonTerminal(UsingDeconstructionFields)),
                                close_brace = Required(Terminal([CloseBrace]))
                            )
                        ),
                        Separated(
                            name = UsingDeconstructionFields,
                            separated = UsingDeconstructionField,
                            separator = Comma,
                            enabled = From("0.8.13"),
                            allow_empty = true
                        ),
                        Struct(
                            name = UsingDeconstructionField,
                            enabled = From("0.8.13"),
                            fields = (
                                name = Required(NonTerminal(IdentifierPath)),
                                alias = Optional(
                                    kind = NonTerminal(UsingAlias),
                                    enabled = From("0.8.19")
                                )
                            )
                        ),
                        Struct(
                            name = UsingAlias,
                            enabled = From("0.8.19"),
                            fields = (
                                as_keyword = Required(Terminal([AsKeyword])),
                                operator = Required(Terminal([
                                    Ampersand,
                                    Asterisk,
                                    BangEqual,
                                    Bar,
                                    Caret,
                                    EqualEqual,
                                    GreaterThan,
                                    GreaterThanEqual,
                                    LessThan,
                                    LessThanEqual,
                                    Minus,
                                    Percent,
                                    Plus,
                                    Slash,
                                    Tilde
                                ]))
                            )
                        ),
                        Enum(
                            name = UsingTarget,
                            variants = [
                                EnumVariant(name = TypeName, reference = TypeName),
                                EnumVariant(name = Asterisk, reference = Asterisk)
                            ]
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
                            name = MultilineComment,
                            scanner = Sequence([
                                Atom("/"),
                                Atom("*"),
                                ZeroOrMore(Choice([
                                    Not(['*']),
                                    TrailingContext(
                                        scanner = Atom("*"),
                                        not_followed_by = Atom("/")
                                    )
                                ])),
                                Atom("*"),
                                Atom("/")
                            ])
                        ),
                        Trivia(
                            name = SingleLineComment,
                            scanner = Sequence([Atom("//"), ZeroOrMore(Not(['\r', '\n']))])
                        )
                    ]
                ),
                Topic(
                    title = "Keywords",
                    items = [
                        Keyword(
                            name = AbicoderKeyword,
                            identifier = Identifier,
                            definitions = [KeywordDefinition(
                                reserved = Never,
                                value = Atom("abicoder")
                            )]
                        ),
                        Keyword(
                            name = AbstractKeyword,
                            identifier = Identifier,
                            definitions = [KeywordDefinition(
                                enabled = From("0.6.0"),
                                value = Atom("abstract")
                            )]
                        ),
                        Keyword(
                            name = AddressKeyword,
                            identifier = Identifier,
                            definitions = [KeywordDefinition(value = Atom("address"))]
                        ),
                        Keyword(
                            name = AfterKeyword,
                            identifier = Identifier,
                            definitions =
                                [KeywordDefinition(enabled = Never, value = Atom("after"))]
                        ),
                        Keyword(
                            name = AliasKeyword,
                            identifier = Identifier,
                            definitions = [KeywordDefinition(
                                enabled = Never,
                                reserved = From("0.5.0"),
                                value = Atom("alias")
                            )]
                        ),
                        Keyword(
                            name = AnonymousKeyword,
                            identifier = Identifier,
                            definitions = [KeywordDefinition(value = Atom("anonymous"))]
                        ),
                        Keyword(
                            name = ApplyKeyword,
                            identifier = Identifier,
                            definitions = [KeywordDefinition(
                                enabled = Never,
                                reserved = From("0.5.0"),
                                value = Atom("apply")
                            )]
                        ),
                        Keyword(
                            name = AsKeyword,
                            identifier = Identifier,
                            definitions = [KeywordDefinition(value = Atom("as"))]
                        ),
                        Keyword(
                            name = AssemblyKeyword,
                            identifier = Identifier,
                            definitions = [KeywordDefinition(value = Atom("assembly"))]
                        ),
                        Keyword(
                            name = AutoKeyword,
                            identifier = Identifier,
                            definitions = [KeywordDefinition(
                                enabled = Never,
                                reserved = From("0.5.0"),
                                value = Atom("auto")
                            )]
                        ),
                        Keyword(
                            name = BoolKeyword,
                            identifier = Identifier,
                            definitions = [KeywordDefinition(value = Atom("bool"))]
                        ),
                        Keyword(
                            name = BreakKeyword,
                            identifier = Identifier,
                            definitions = [KeywordDefinition(value = Atom("break"))]
                        ),
                        Keyword(
                            name = ByteKeyword,
                            identifier = Identifier,
                            definitions = [KeywordDefinition(
                                enabled = Till("0.8.0"),
                                value = Atom("byte")
                            )]
                        ),
                        Keyword(
                            name = BytesKeyword,
                            identifier = Identifier,
                            definitions = [KeywordDefinition(
                                value = Sequence([
                                    Atom("bytes"),
                                    Choice([
                                        Atom("1"),
                                        Atom("2"),
                                        Atom("3"),
                                        Atom("4"),
                                        Atom("5"),
                                        Atom("6"),
                                        Atom("7"),
                                        Atom("8"),
                                        Atom("9"),
                                        Atom("10"),
                                        Atom("11"),
                                        Atom("12"),
                                        Atom("13"),
                                        Atom("14"),
                                        Atom("15"),
                                        Atom("16"),
                                        Atom("17"),
                                        Atom("18"),
                                        Atom("19"),
                                        Atom("20"),
                                        Atom("21"),
                                        Atom("22"),
                                        Atom("23"),
                                        Atom("24"),
                                        Atom("25"),
                                        Atom("26"),
                                        Atom("27"),
                                        Atom("28"),
                                        Atom("29"),
                                        Atom("30"),
                                        Atom("31"),
                                        Atom("32")
                                    ])
                                ])
                            )]
                        ),
                        Keyword(
                            name = CallDataKeyword,
                            identifier = Identifier,
                            definitions = [KeywordDefinition(
                                enabled = From("0.5.0"),
                                reserved = From("0.5.0"),
                                value = Atom("calldata")
                            )]
                        ),
                        Keyword(
                            name = CaseKeyword,
                            identifier = Identifier,
                            definitions =
                                [KeywordDefinition(enabled = Never, value = Atom("case"))]
                        ),
                        Keyword(
                            name = CatchKeyword,
                            identifier = Identifier,
                            definitions = [KeywordDefinition(
                                enabled = From("0.6.0"),
                                value = Atom("catch")
                            )]
                        ),
                        Keyword(
                            name = ConstantKeyword,
                            identifier = Identifier,
                            definitions = [KeywordDefinition(value = Atom("constant"))]
                        ),
                        Keyword(
                            name = ConstructorKeyword,
                            identifier = Identifier,
                            definitions = [KeywordDefinition(
                                enabled = From("0.4.22"),
                                reserved = From("0.5.0"),
                                value = Atom("constructor")
                            )]
                        ),
                        Keyword(
                            name = ContinueKeyword,
                            identifier = Identifier,
                            definitions = [KeywordDefinition(value = Atom("continue"))]
                        ),
                        Keyword(
                            name = ContractKeyword,
                            identifier = Identifier,
                            definitions = [KeywordDefinition(value = Atom("contract"))]
                        ),
                        Keyword(
                            name = CopyOfKeyword,
                            identifier = Identifier,
                            definitions = [KeywordDefinition(
                                enabled = Never,
                                reserved = From("0.5.0"),
                                value = Atom("copyof")
                            )]
                        ),
                        Keyword(
                            name = DaysKeyword,
                            identifier = Identifier,
                            definitions = [KeywordDefinition(value = Atom("days"))]
                        ),
                        Keyword(
                            name = DefaultKeyword,
                            identifier = Identifier,
                            definitions =
                                [KeywordDefinition(enabled = Never, value = Atom("default"))]
                        ),
                        Keyword(
                            name = DefineKeyword,
                            identifier = Identifier,
                            definitions = [KeywordDefinition(
                                enabled = Never,
                                reserved = From("0.5.0"),
                                value = Atom("define")
                            )]
                        ),
                        Keyword(
                            name = DeleteKeyword,
                            identifier = Identifier,
                            definitions = [KeywordDefinition(value = Atom("delete"))]
                        ),
                        Keyword(
                            name = DoKeyword,
                            identifier = Identifier,
                            definitions = [KeywordDefinition(value = Atom("do"))]
                        ),
                        Keyword(
                            name = ElseKeyword,
                            identifier = Identifier,
                            definitions = [KeywordDefinition(value = Atom("else"))]
                        ),
                        Keyword(
                            name = EmitKeyword,
                            identifier = Identifier,
                            definitions = [KeywordDefinition(
                                enabled = From("0.4.21"),
                                reserved = From("0.5.0"),
                                value = Atom("emit")
                            )]
                        ),
                        Keyword(
                            name = EnumKeyword,
                            identifier = Identifier,
                            definitions = [KeywordDefinition(value = Atom("enum"))]
                        ),
                        Keyword(
                            name = ErrorKeyword,
                            identifier = Identifier,
                            definitions = [KeywordDefinition(
                                enabled = From("0.8.4"),
                                reserved = Never,
                                value = Atom("error")
                            )]
                        ),
                        Keyword(
                            name = EtherKeyword,
                            identifier = Identifier,
                            definitions = [KeywordDefinition(value = Atom("ether"))]
                        ),
                        Keyword(
                            name = EventKeyword,
                            identifier = Identifier,
                            definitions = [KeywordDefinition(value = Atom("event"))]
                        ),
                        Keyword(
                            name = ExperimentalKeyword,
                            identifier = Identifier,
                            definitions = [KeywordDefinition(
                                reserved = Never,
                                value = Atom("experimental")
                            )]
                        ),
                        Keyword(
                            name = ExternalKeyword,
                            identifier = Identifier,
                            definitions = [KeywordDefinition(value = Atom("external"))]
                        ),
                        Keyword(
                            name = FallbackKeyword,
                            identifier = Identifier,
                            definitions = [KeywordDefinition(
                                reserved = From("0.6.0"),
                                value = Atom("fallback")
                            )]
                        ),
                        Keyword(
                            name = FalseKeyword,
                            identifier = Identifier,
                            definitions = [KeywordDefinition(value = Atom("false"))]
                        ),
                        Keyword(
                            name = FinalKeyword,
                            identifier = Identifier,
                            definitions =
                                [KeywordDefinition(enabled = Never, value = Atom("final"))]
                        ),
                        Keyword(
                            name = FinneyKeyword,
                            identifier = Identifier,
                            definitions = [KeywordDefinition(
                                enabled = Till("0.7.0"),
                                reserved = Till("0.7.0"),
                                value = Atom("finney")
                            )]
                        ),
                        Keyword(
                            name = FixedKeyword,
                            identifier = Identifier,
                            definitions = [
                                KeywordDefinition(value = Atom("fixed")),
                                KeywordDefinition(
                                    value = Sequence([
                                        Atom("fixed"),
                                        Choice([
                                            Atom("8"),
                                            Atom("16"),
                                            Atom("24"),
                                            Atom("32"),
                                            Atom("40"),
                                            Atom("48"),
                                            Atom("56"),
                                            Atom("64"),
                                            Atom("72"),
                                            Atom("80"),
                                            Atom("88"),
                                            Atom("96"),
                                            Atom("104"),
                                            Atom("112"),
                                            Atom("120"),
                                            Atom("128"),
                                            Atom("136"),
                                            Atom("144"),
                                            Atom("152"),
                                            Atom("160"),
                                            Atom("168"),
                                            Atom("176")
                                        ]),
                                        Atom("x"),
                                        Choice([
                                            Atom("8"),
                                            Atom("16"),
                                            Atom("24"),
                                            Atom("32"),
                                            Atom("40"),
                                            Atom("48"),
                                            Atom("56"),
                                            Atom("64"),
                                            Atom("72"),
                                            Atom("80")
                                        ])
                                    ])
                                ),
                                KeywordDefinition(
                                    value = Sequence([
                                        Atom("fixed"),
                                        Choice([
                                            Atom("184x8"),
                                            Atom("184x16"),
                                            Atom("184x24"),
                                            Atom("184x32"),
                                            Atom("184x40"),
                                            Atom("184x48"),
                                            Atom("184x56"),
                                            Atom("184x64"),
                                            Atom("184x72"),
                                            Atom("192x8"),
                                            Atom("192x16"),
                                            Atom("192x24"),
                                            Atom("192x32"),
                                            Atom("192x40"),
                                            Atom("192x48"),
                                            Atom("192x56"),
                                            Atom("192x64"),
                                            Atom("200x8"),
                                            Atom("200x16"),
                                            Atom("200x24"),
                                            Atom("200x32"),
                                            Atom("200x40"),
                                            Atom("200x48"),
                                            Atom("200x56"),
                                            Atom("208x8"),
                                            Atom("208x16"),
                                            Atom("208x24"),
                                            Atom("208x32"),
                                            Atom("208x40"),
                                            Atom("208x48"),
                                            Atom("216x8"),
                                            Atom("216x16"),
                                            Atom("216x24"),
                                            Atom("216x32"),
                                            Atom("216x40"),
                                            Atom("224x8"),
                                            Atom("224x16"),
                                            Atom("224x24"),
                                            Atom("224x32"),
                                            Atom("232x8"),
                                            Atom("232x16"),
                                            Atom("232x24"),
                                            Atom("240x8"),
                                            Atom("240x16"),
                                            Atom("248x8")
                                        ])
                                    ])
                                ),
                                KeywordDefinition(
                                    reserved = From("0.4.14"),
                                    value = Sequence([
                                        Atom("fixed"),
                                        Choice([
                                            Atom("184x80"),
                                            Atom("192x72"),
                                            Atom("192x80"),
                                            Atom("200x64"),
                                            Atom("200x72"),
                                            Atom("200x80"),
                                            Atom("208x56"),
                                            Atom("208x64"),
                                            Atom("208x72"),
                                            Atom("208x80"),
                                            Atom("216x48"),
                                            Atom("216x56"),
                                            Atom("216x64"),
                                            Atom("216x72"),
                                            Atom("216x80"),
                                            Atom("224x40"),
                                            Atom("224x48"),
                                            Atom("224x56"),
                                            Atom("224x64"),
                                            Atom("224x72"),
                                            Atom("224x80"),
                                            Atom("232x32"),
                                            Atom("232x40"),
                                            Atom("232x48"),
                                            Atom("232x56"),
                                            Atom("232x64"),
                                            Atom("232x72"),
                                            Atom("232x80"),
                                            Atom("240x24"),
                                            Atom("240x32"),
                                            Atom("240x40"),
                                            Atom("240x48"),
                                            Atom("240x56"),
                                            Atom("240x64"),
                                            Atom("240x72"),
                                            Atom("240x80"),
                                            Atom("248x16"),
                                            Atom("248x24"),
                                            Atom("248x32"),
                                            Atom("248x40"),
                                            Atom("248x48"),
                                            Atom("248x56"),
                                            Atom("248x64"),
                                            Atom("248x72"),
                                            Atom("248x80"),
                                            Atom("256x8"),
                                            Atom("256x16"),
                                            Atom("256x24"),
                                            Atom("256x32"),
                                            Atom("256x40"),
                                            Atom("256x48"),
                                            Atom("256x56"),
                                            Atom("256x64"),
                                            Atom("256x72"),
                                            Atom("256x80")
                                        ])
                                    ])
                                ),
                                KeywordDefinition(
                                    reserved = From("0.4.14"),
                                    value = Sequence([
                                        Atom("fixed"),
                                        Choice([
                                            Atom("8"),
                                            Atom("16"),
                                            Atom("24"),
                                            Atom("32"),
                                            Atom("40"),
                                            Atom("48"),
                                            Atom("56"),
                                            Atom("64"),
                                            Atom("72"),
                                            Atom("80"),
                                            Atom("88"),
                                            Atom("96"),
                                            Atom("104"),
                                            Atom("112"),
                                            Atom("120"),
                                            Atom("128"),
                                            Atom("136"),
                                            Atom("144"),
                                            Atom("152"),
                                            Atom("160"),
                                            Atom("168"),
                                            Atom("176"),
                                            Atom("184"),
                                            Atom("192"),
                                            Atom("200"),
                                            Atom("208"),
                                            Atom("216"),
                                            Atom("224"),
                                            Atom("232"),
                                            Atom("240"),
                                            Atom("248"),
                                            Atom("256")
                                        ]),
                                        Atom("x"),
                                        Choice([
                                            Atom("0"),
                                            Atom("1"),
                                            Atom("2"),
                                            Atom("3"),
                                            Atom("4"),
                                            Atom("5"),
                                            Atom("6"),
                                            Atom("7"),
                                            Atom("9"),
                                            Atom("10"),
                                            Atom("11"),
                                            Atom("12"),
                                            Atom("13"),
                                            Atom("14"),
                                            Atom("15"),
                                            Atom("17"),
                                            Atom("18"),
                                            Atom("19"),
                                            Atom("20"),
                                            Atom("21"),
                                            Atom("22"),
                                            Atom("23"),
                                            Atom("25"),
                                            Atom("26"),
                                            Atom("27"),
                                            Atom("28"),
                                            Atom("29"),
                                            Atom("30"),
                                            Atom("31"),
                                            Atom("33"),
                                            Atom("34"),
                                            Atom("35"),
                                            Atom("36"),
                                            Atom("37"),
                                            Atom("38"),
                                            Atom("39"),
                                            Atom("41"),
                                            Atom("42"),
                                            Atom("43"),
                                            Atom("44"),
                                            Atom("45"),
                                            Atom("46"),
                                            Atom("47"),
                                            Atom("49"),
                                            Atom("50"),
                                            Atom("51"),
                                            Atom("52"),
                                            Atom("53"),
                                            Atom("54"),
                                            Atom("55"),
                                            Atom("57"),
                                            Atom("58"),
                                            Atom("59"),
                                            Atom("60"),
                                            Atom("61"),
                                            Atom("62"),
                                            Atom("63"),
                                            Atom("65"),
                                            Atom("66"),
                                            Atom("67"),
                                            Atom("68"),
                                            Atom("69"),
                                            Atom("70"),
                                            Atom("71"),
                                            Atom("73"),
                                            Atom("74"),
                                            Atom("75"),
                                            Atom("76"),
                                            Atom("77"),
                                            Atom("78"),
                                            Atom("79")
                                        ])
                                    ])
                                )
                            ]
                        ),
                        Keyword(
                            name = ForKeyword,
                            identifier = Identifier,
                            definitions = [KeywordDefinition(value = Atom("for"))]
                        ),
                        Keyword(
                            name = FromKeyword,
                            identifier = Identifier,
                            definitions =
                                [KeywordDefinition(reserved = Never, value = Atom("from"))]
                        ),
                        Keyword(
                            name = FunctionKeyword,
                            identifier = Identifier,
                            definitions = [KeywordDefinition(value = Atom("function"))]
                        ),
                        Keyword(
                            name = GlobalKeyword,
                            identifier = Identifier,
                            definitions = [KeywordDefinition(
                                enabled = From("0.8.13"),
                                reserved = Never,
                                value = Atom("global")
                            )]
                        ),
                        Keyword(
                            name = GweiKeyword,
                            identifier = Identifier,
                            definitions = [KeywordDefinition(
                                enabled = From("0.6.11"),
                                reserved = From("0.7.0"),
                                value = Atom("gwei")
                            )]
                        ),
                        Keyword(
                            name = HexKeyword,
                            identifier = Identifier,
                            definitions = [KeywordDefinition(enabled = Never, value = Atom("hex"))]
                        ),
                        Keyword(
                            name = HoursKeyword,
                            identifier = Identifier,
                            definitions = [KeywordDefinition(value = Atom("hours"))]
                        ),
                        Keyword(
                            name = IfKeyword,
                            identifier = Identifier,
                            definitions = [KeywordDefinition(value = Atom("if"))]
                        ),
                        Keyword(
                            name = ImmutableKeyword,
                            identifier = Identifier,
                            definitions = [KeywordDefinition(
                                enabled = From("0.6.5"),
                                reserved = From("0.5.0"),
                                value = Atom("immutable")
                            )]
                        ),
                        Keyword(
                            name = ImplementsKeyword,
                            identifier = Identifier,
                            definitions = [KeywordDefinition(
                                enabled = Never,
                                reserved = From("0.5.0"),
                                value = Atom("implements")
                            )]
                        ),
                        Keyword(
                            name = ImportKeyword,
                            identifier = Identifier,
                            definitions = [KeywordDefinition(value = Atom("import"))]
                        ),
                        Keyword(
                            name = IndexedKeyword,
                            identifier = Identifier,
                            definitions = [KeywordDefinition(value = Atom("indexed"))]
                        ),
                        Keyword(
                            name = InKeyword,
                            identifier = Identifier,
                            definitions = [KeywordDefinition(enabled = Never, value = Atom("in"))]
                        ),
                        Keyword(
                            name = InlineKeyword,
                            identifier = Identifier,
                            definitions =
                                [KeywordDefinition(enabled = Never, value = Atom("inline"))]
                        ),
                        Keyword(
                            name = InterfaceKeyword,
                            identifier = Identifier,
                            definitions = [KeywordDefinition(value = Atom("interface"))]
                        ),
                        Keyword(
                            name = InternalKeyword,
                            identifier = Identifier,
                            definitions = [KeywordDefinition(value = Atom("internal"))]
                        ),
                        Keyword(
                            name = IntKeyword,
                            identifier = Identifier,
                            definitions = [KeywordDefinition(
                                value = Sequence([
                                    Atom("int"),
                                    Optional(Choice([
                                        Atom("8"),
                                        Atom("16"),
                                        Atom("24"),
                                        Atom("32"),
                                        Atom("40"),
                                        Atom("48"),
                                        Atom("56"),
                                        Atom("64"),
                                        Atom("72"),
                                        Atom("80"),
                                        Atom("88"),
                                        Atom("96"),
                                        Atom("104"),
                                        Atom("112"),
                                        Atom("120"),
                                        Atom("128"),
                                        Atom("136"),
                                        Atom("144"),
                                        Atom("152"),
                                        Atom("160"),
                                        Atom("168"),
                                        Atom("176"),
                                        Atom("184"),
                                        Atom("192"),
                                        Atom("200"),
                                        Atom("208"),
                                        Atom("216"),
                                        Atom("224"),
                                        Atom("232"),
                                        Atom("240"),
                                        Atom("248"),
                                        Atom("256")
                                    ]))
                                ])
                            )]
                        ),
                        Keyword(
                            name = IsKeyword,
                            identifier = Identifier,
                            definitions = [KeywordDefinition(value = Atom("is"))]
                        ),
                        Keyword(
                            name = LetKeyword,
                            identifier = Identifier,
                            definitions = [KeywordDefinition(enabled = Never, value = Atom("let"))]
                        ),
                        Keyword(
                            name = LibraryKeyword,
                            identifier = Identifier,
                            definitions = [KeywordDefinition(value = Atom("library"))]
                        ),
                        Keyword(
                            name = MacroKeyword,
                            identifier = Identifier,
                            definitions = [KeywordDefinition(
                                enabled = Never,
                                reserved = From("0.5.0"),
                                value = Atom("macro")
                            )]
                        ),
                        Keyword(
                            name = MappingKeyword,
                            identifier = Identifier,
                            definitions = [KeywordDefinition(value = Atom("mapping"))]
                        ),
                        Keyword(
                            name = MatchKeyword,
                            identifier = Identifier,
                            definitions =
                                [KeywordDefinition(enabled = Never, value = Atom("match"))]
                        ),
                        Keyword(
                            name = MemoryKeyword,
                            identifier = Identifier,
                            definitions = [KeywordDefinition(value = Atom("memory"))]
                        ),
                        Keyword(
                            name = MinutesKeyword,
                            identifier = Identifier,
                            definitions = [KeywordDefinition(value = Atom("minutes"))]
                        ),
                        Keyword(
                            name = ModifierKeyword,
                            identifier = Identifier,
                            definitions = [KeywordDefinition(value = Atom("modifier"))]
                        ),
                        Keyword(
                            name = MutableKeyword,
                            identifier = Identifier,
                            definitions = [KeywordDefinition(
                                enabled = Never,
                                reserved = From("0.5.0"),
                                value = Atom("mutable")
                            )]
                        ),
                        Keyword(
                            name = NewKeyword,
                            identifier = Identifier,
                            definitions = [KeywordDefinition(value = Atom("new"))]
                        ),
                        Keyword(
                            name = NullKeyword,
                            identifier = Identifier,
                            definitions =
                                [KeywordDefinition(enabled = Never, value = Atom("null"))]
                        ),
                        Keyword(
                            name = OfKeyword,
                            identifier = Identifier,
                            definitions = [KeywordDefinition(enabled = Never, value = Atom("of"))]
                        ),
                        Keyword(
                            name = OverrideKeyword,
                            identifier = Identifier,
                            definitions = [KeywordDefinition(
                                reserved = From("0.5.0"),
                                value = Atom("override")
                            )]
                        ),
                        Keyword(
                            name = PartialKeyword,
                            identifier = Identifier,
                            definitions = [KeywordDefinition(
                                enabled = Never,
                                reserved = From("0.5.0"),
                                value = Atom("partial")
                            )]
                        ),
                        Keyword(
                            name = PayableKeyword,
                            identifier = Identifier,
                            definitions = [KeywordDefinition(value = Atom("payable"))]
                        ),
                        Keyword(
                            name = PragmaKeyword,
                            identifier = Identifier,
                            definitions = [KeywordDefinition(value = Atom("pragma"))]
                        ),
                        Keyword(
                            name = PrivateKeyword,
                            identifier = Identifier,
                            definitions = [KeywordDefinition(value = Atom("private"))]
                        ),
                        Keyword(
                            name = PromiseKeyword,
                            identifier = Identifier,
                            definitions = [KeywordDefinition(
                                enabled = Never,
                                reserved = From("0.5.0"),
                                value = Atom("promise")
                            )]
                        ),
                        Keyword(
                            name = PublicKeyword,
                            identifier = Identifier,
                            definitions = [KeywordDefinition(value = Atom("public"))]
                        ),
                        Keyword(
                            name = PureKeyword,
                            identifier = Identifier,
                            definitions = [KeywordDefinition(value = Atom("pure"))]
                        ),
                        Keyword(
                            name = ReceiveKeyword,
                            identifier = Identifier,
                            definitions = [KeywordDefinition(
                                reserved = From("0.6.0"),
                                value = Atom("receive")
                            )]
                        ),
                        Keyword(
                            name = ReferenceKeyword,
                            identifier = Identifier,
                            definitions = [KeywordDefinition(
                                enabled = Never,
                                reserved = From("0.5.0"),
                                value = Atom("reference")
                            )]
                        ),
                        Keyword(
                            name = RelocatableKeyword,
                            identifier = Identifier,
                            definitions = [KeywordDefinition(
                                enabled = Never,
                                value = Atom("relocatable")
                            )]
                        ),
                        Keyword(
                            name = ReturnKeyword,
                            identifier = Identifier,
                            definitions = [KeywordDefinition(value = Atom("return"))]
                        ),
                        Keyword(
                            name = ReturnsKeyword,
                            identifier = Identifier,
                            definitions = [KeywordDefinition(value = Atom("returns"))]
                        ),
                        Keyword(
                            name = RevertKeyword,
                            identifier = Identifier,
                            definitions = [KeywordDefinition(
                                enabled = From("0.8.4"),
                                reserved = Never,
                                value = Atom("revert")
                            )]
                        ),
                        Keyword(
                            name = SealedKeyword,
                            identifier = Identifier,
                            definitions = [KeywordDefinition(
                                enabled = Never,
                                reserved = From("0.5.0"),
                                value = Atom("sealed")
                            )]
                        ),
                        Keyword(
                            name = SecondsKeyword,
                            identifier = Identifier,
                            definitions = [KeywordDefinition(value = Atom("seconds"))]
                        ),
                        Keyword(
                            name = SizeOfKeyword,
                            identifier = Identifier,
                            definitions = [KeywordDefinition(
                                enabled = Never,
                                reserved = From("0.5.0"),
                                value = Atom("sizeof")
                            )]
                        ),
                        Keyword(
                            name = SolidityKeyword,
                            identifier = Identifier,
                            definitions = [KeywordDefinition(
                                reserved = Never,
                                value = Atom("solidity")
                            )]
                        ),
                        Keyword(
                            name = StaticKeyword,
                            identifier = Identifier,
                            definitions =
                                [KeywordDefinition(enabled = Never, value = Atom("static"))]
                        ),
                        Keyword(
                            name = StorageKeyword,
                            identifier = Identifier,
                            definitions = [KeywordDefinition(value = Atom("storage"))]
                        ),
                        Keyword(
                            name = StringKeyword,
                            identifier = Identifier,
                            definitions = [KeywordDefinition(value = Atom("string"))]
                        ),
                        Keyword(
                            name = StructKeyword,
                            identifier = Identifier,
                            definitions = [KeywordDefinition(value = Atom("struct"))]
                        ),
                        Keyword(
                            name = SupportsKeyword,
                            identifier = Identifier,
                            definitions = [KeywordDefinition(
                                enabled = Never,
                                reserved = From("0.5.0"),
                                value = Atom("supports")
                            )]
                        ),
                        Keyword(
                            name = SwitchKeyword,
                            identifier = Identifier,
                            definitions =
                                [KeywordDefinition(enabled = Never, value = Atom("switch"))]
                        ),
                        Keyword(
                            name = SzaboKeyword,
                            identifier = Identifier,
                            definitions = [KeywordDefinition(
                                enabled = Till("0.7.0"),
                                reserved = Till("0.7.0"),
                                value = Atom("szabo")
                            )]
                        ),
                        Keyword(
                            name = ThrowKeyword,
                            identifier = Identifier,
                            definitions = [KeywordDefinition(
                                enabled = Till("0.5.0"),
                                value = Atom("throw")
                            )]
                        ),
                        Keyword(
                            name = TrueKeyword,
                            identifier = Identifier,
                            definitions = [KeywordDefinition(value = Atom("true"))]
                        ),
                        Keyword(
                            name = TryKeyword,
                            identifier = Identifier,
                            definitions = [KeywordDefinition(
                                enabled = From("0.6.0"),
                                value = Atom("try")
                            )]
                        ),
                        Keyword(
                            name = TypeDefKeyword,
                            identifier = Identifier,
                            definitions = [KeywordDefinition(
                                enabled = Never,
                                reserved = From("0.5.0"),
                                value = Atom("typedef")
                            )]
                        ),
                        Keyword(
                            name = TypeKeyword,
                            identifier = Identifier,
                            definitions = [KeywordDefinition(
                                enabled = From("0.5.3"),
                                value = Atom("type")
                            )]
                        ),
                        Keyword(
                            name = TypeOfKeyword,
                            identifier = Identifier,
                            definitions =
                                [KeywordDefinition(enabled = Never, value = Atom("typeof"))]
                        ),
                        Keyword(
                            name = UfixedKeyword,
                            identifier = Identifier,
                            definitions = [
                                KeywordDefinition(value = Atom("ufixed")),
                                KeywordDefinition(
                                    value = Sequence([
                                        Atom("ufixed"),
                                        Choice([
                                            Atom("8"),
                                            Atom("16"),
                                            Atom("24"),
                                            Atom("32"),
                                            Atom("40"),
                                            Atom("48"),
                                            Atom("56"),
                                            Atom("64"),
                                            Atom("72"),
                                            Atom("80"),
                                            Atom("88"),
                                            Atom("96"),
                                            Atom("104"),
                                            Atom("112"),
                                            Atom("120"),
                                            Atom("128"),
                                            Atom("136"),
                                            Atom("144"),
                                            Atom("152"),
                                            Atom("160"),
                                            Atom("168"),
                                            Atom("176")
                                        ]),
                                        Atom("x"),
                                        Choice([
                                            Atom("8"),
                                            Atom("16"),
                                            Atom("24"),
                                            Atom("32"),
                                            Atom("40"),
                                            Atom("48"),
                                            Atom("56"),
                                            Atom("64"),
                                            Atom("72"),
                                            Atom("80")
                                        ])
                                    ])
                                ),
                                KeywordDefinition(
                                    value = Sequence([
                                        Atom("ufixed"),
                                        Choice([
                                            Atom("184x8"),
                                            Atom("184x16"),
                                            Atom("184x24"),
                                            Atom("184x32"),
                                            Atom("184x40"),
                                            Atom("184x48"),
                                            Atom("184x56"),
                                            Atom("184x64"),
                                            Atom("184x72"),
                                            Atom("192x8"),
                                            Atom("192x16"),
                                            Atom("192x24"),
                                            Atom("192x32"),
                                            Atom("192x40"),
                                            Atom("192x48"),
                                            Atom("192x56"),
                                            Atom("192x64"),
                                            Atom("200x8"),
                                            Atom("200x16"),
                                            Atom("200x24"),
                                            Atom("200x32"),
                                            Atom("200x40"),
                                            Atom("200x48"),
                                            Atom("200x56"),
                                            Atom("208x8"),
                                            Atom("208x16"),
                                            Atom("208x24"),
                                            Atom("208x32"),
                                            Atom("208x40"),
                                            Atom("208x48"),
                                            Atom("216x8"),
                                            Atom("216x16"),
                                            Atom("216x24"),
                                            Atom("216x32"),
                                            Atom("216x40"),
                                            Atom("224x8"),
                                            Atom("224x16"),
                                            Atom("224x24"),
                                            Atom("224x32"),
                                            Atom("232x8"),
                                            Atom("232x16"),
                                            Atom("232x24"),
                                            Atom("240x8"),
                                            Atom("240x16"),
                                            Atom("248x8")
                                        ])
                                    ])
                                ),
                                KeywordDefinition(
                                    reserved = From("0.4.14"),
                                    value = Sequence([
                                        Atom("ufixed"),
                                        Choice([
                                            Atom("184x80"),
                                            Atom("192x72"),
                                            Atom("192x80"),
                                            Atom("200x64"),
                                            Atom("200x72"),
                                            Atom("200x80"),
                                            Atom("208x56"),
                                            Atom("208x64"),
                                            Atom("208x72"),
                                            Atom("208x80"),
                                            Atom("216x48"),
                                            Atom("216x56"),
                                            Atom("216x64"),
                                            Atom("216x72"),
                                            Atom("216x80"),
                                            Atom("224x40"),
                                            Atom("224x48"),
                                            Atom("224x56"),
                                            Atom("224x64"),
                                            Atom("224x72"),
                                            Atom("224x80"),
                                            Atom("232x32"),
                                            Atom("232x40"),
                                            Atom("232x48"),
                                            Atom("232x56"),
                                            Atom("232x64"),
                                            Atom("232x72"),
                                            Atom("232x80"),
                                            Atom("240x24"),
                                            Atom("240x32"),
                                            Atom("240x40"),
                                            Atom("240x48"),
                                            Atom("240x56"),
                                            Atom("240x64"),
                                            Atom("240x72"),
                                            Atom("240x80"),
                                            Atom("248x16"),
                                            Atom("248x24"),
                                            Atom("248x32"),
                                            Atom("248x40"),
                                            Atom("248x48"),
                                            Atom("248x56"),
                                            Atom("248x64"),
                                            Atom("248x72"),
                                            Atom("248x80"),
                                            Atom("256x8"),
                                            Atom("256x16"),
                                            Atom("256x24"),
                                            Atom("256x32"),
                                            Atom("256x40"),
                                            Atom("256x48"),
                                            Atom("256x56"),
                                            Atom("256x64"),
                                            Atom("256x72"),
                                            Atom("256x80")
                                        ])
                                    ])
                                ),
                                KeywordDefinition(
                                    reserved = From("0.4.14"),
                                    value = Sequence([
                                        Atom("ufixed"),
                                        Choice([
                                            Atom("8"),
                                            Atom("16"),
                                            Atom("24"),
                                            Atom("32"),
                                            Atom("40"),
                                            Atom("48"),
                                            Atom("56"),
                                            Atom("64"),
                                            Atom("72"),
                                            Atom("80"),
                                            Atom("88"),
                                            Atom("96"),
                                            Atom("104"),
                                            Atom("112"),
                                            Atom("120"),
                                            Atom("128"),
                                            Atom("136"),
                                            Atom("144"),
                                            Atom("152"),
                                            Atom("160"),
                                            Atom("168"),
                                            Atom("176"),
                                            Atom("184"),
                                            Atom("192"),
                                            Atom("200"),
                                            Atom("208"),
                                            Atom("216"),
                                            Atom("224"),
                                            Atom("232"),
                                            Atom("240"),
                                            Atom("248"),
                                            Atom("256")
                                        ]),
                                        Atom("x"),
                                        Choice([
                                            Atom("0"),
                                            Atom("1"),
                                            Atom("2"),
                                            Atom("3"),
                                            Atom("4"),
                                            Atom("5"),
                                            Atom("6"),
                                            Atom("7"),
                                            Atom("9"),
                                            Atom("10"),
                                            Atom("11"),
                                            Atom("12"),
                                            Atom("13"),
                                            Atom("14"),
                                            Atom("15"),
                                            Atom("17"),
                                            Atom("18"),
                                            Atom("19"),
                                            Atom("20"),
                                            Atom("21"),
                                            Atom("22"),
                                            Atom("23"),
                                            Atom("25"),
                                            Atom("26"),
                                            Atom("27"),
                                            Atom("28"),
                                            Atom("29"),
                                            Atom("30"),
                                            Atom("31"),
                                            Atom("33"),
                                            Atom("34"),
                                            Atom("35"),
                                            Atom("36"),
                                            Atom("37"),
                                            Atom("38"),
                                            Atom("39"),
                                            Atom("41"),
                                            Atom("42"),
                                            Atom("43"),
                                            Atom("44"),
                                            Atom("45"),
                                            Atom("46"),
                                            Atom("47"),
                                            Atom("49"),
                                            Atom("50"),
                                            Atom("51"),
                                            Atom("52"),
                                            Atom("53"),
                                            Atom("54"),
                                            Atom("55"),
                                            Atom("57"),
                                            Atom("58"),
                                            Atom("59"),
                                            Atom("60"),
                                            Atom("61"),
                                            Atom("62"),
                                            Atom("63"),
                                            Atom("65"),
                                            Atom("66"),
                                            Atom("67"),
                                            Atom("68"),
                                            Atom("69"),
                                            Atom("70"),
                                            Atom("71"),
                                            Atom("73"),
                                            Atom("74"),
                                            Atom("75"),
                                            Atom("76"),
                                            Atom("77"),
                                            Atom("78"),
                                            Atom("79")
                                        ])
                                    ])
                                )
                            ]
                        ),
                        Keyword(
                            name = UintKeyword,
                            identifier = Identifier,
                            definitions = [KeywordDefinition(
                                value = Sequence([
                                    Atom("uint"),
                                    Optional(Choice([
                                        Atom("8"),
                                        Atom("16"),
                                        Atom("24"),
                                        Atom("32"),
                                        Atom("40"),
                                        Atom("48"),
                                        Atom("56"),
                                        Atom("64"),
                                        Atom("72"),
                                        Atom("80"),
                                        Atom("88"),
                                        Atom("96"),
                                        Atom("104"),
                                        Atom("112"),
                                        Atom("120"),
                                        Atom("128"),
                                        Atom("136"),
                                        Atom("144"),
                                        Atom("152"),
                                        Atom("160"),
                                        Atom("168"),
                                        Atom("176"),
                                        Atom("184"),
                                        Atom("192"),
                                        Atom("200"),
                                        Atom("208"),
                                        Atom("216"),
                                        Atom("224"),
                                        Atom("232"),
                                        Atom("240"),
                                        Atom("248"),
                                        Atom("256")
                                    ]))
                                ])
                            )]
                        ),
                        Keyword(
                            name = UncheckedKeyword,
                            identifier = Identifier,
                            definitions = [KeywordDefinition(
                                enabled = From("0.8.0"),
                                reserved = From("0.5.0"),
                                value = Atom("unchecked")
                            )]
                        ),
                        Keyword(
                            name = UsingKeyword,
                            identifier = Identifier,
                            definitions = [KeywordDefinition(value = Atom("using"))]
                        ),
                        Keyword(
                            name = VarKeyword,
                            identifier = Identifier,
                            definitions = [KeywordDefinition(
                                enabled = Till("0.5.0"),
                                value = Atom("var")
                            )]
                        ),
                        Keyword(
                            name = ViewKeyword,
                            identifier = Identifier,
                            definitions = [KeywordDefinition(value = Atom("view"))]
                        ),
                        Keyword(
                            name = VirtualKeyword,
                            identifier = Identifier,
                            definitions = [KeywordDefinition(
                                enabled = From("0.6.0"),
                                reserved = From("0.6.0"),
                                value = Atom("virtual")
                            )]
                        ),
                        Keyword(
                            name = WeeksKeyword,
                            identifier = Identifier,
                            definitions = [KeywordDefinition(value = Atom("weeks"))]
                        ),
                        Keyword(
                            name = WeiKeyword,
                            identifier = Identifier,
                            definitions = [KeywordDefinition(value = Atom("wei"))]
                        ),
                        Keyword(
                            name = WhileKeyword,
                            identifier = Identifier,
                            definitions = [KeywordDefinition(value = Atom("while"))]
                        ),
                        Keyword(
                            name = YearsKeyword,
                            identifier = Identifier,
                            definitions = [KeywordDefinition(
                                enabled = Till("0.5.0"),
                                value = Atom("years")
                            )]
                        )
                    ]
                ),
                Topic(
                    title = "Punctuation",
                    items = [
                        Token(
                            name = OpenParen,
                            definitions = [TokenDefinition(scanner = Atom("("))]
                        ),
                        Token(
                            name = CloseParen,
                            definitions = [TokenDefinition(scanner = Atom(")"))]
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
                            name = OpenBrace,
                            definitions = [TokenDefinition(scanner = Atom("{"))]
                        ),
                        Token(
                            name = CloseBrace,
                            definitions = [TokenDefinition(scanner = Atom("}"))]
                        ),
                        Token(
                            name = Comma,
                            definitions = [TokenDefinition(scanner = Atom(","))]
                        ),
                        Token(
                            name = Period,
                            definitions = [TokenDefinition(scanner = Atom("."))]
                        ),
                        Token(
                            name = QuestionMark,
                            definitions = [TokenDefinition(scanner = Atom("?"))]
                        ),
                        Token(
                            name = Semicolon,
                            definitions = [TokenDefinition(scanner = Atom(";"))]
                        ),
                        Token(
                            name = Colon,
                            definitions = [TokenDefinition(
                                scanner = TrailingContext(
                                    scanner = Atom(":"),
                                    not_followed_by = Atom("=")
                                )
                            )]
                        ),
                        Token(
                            name = ColonEqual,
                            definitions = [TokenDefinition(scanner = Atom(":="))]
                        ),
                        Token(
                            name = Equal,
                            definitions = [TokenDefinition(
                                scanner = TrailingContext(
                                    scanner = Atom("="),
                                    not_followed_by = Choice([Atom("="), Atom(">")])
                                )
                            )]
                        ),
                        Token(
                            name = EqualEqual,
                            definitions = [TokenDefinition(scanner = Atom("=="))]
                        ),
                        Token(
                            name = EqualGreaterThan,
                            definitions = [TokenDefinition(scanner = Atom("=>"))]
                        ),
                        Token(
                            name = Asterisk,
                            definitions = [TokenDefinition(
                                scanner = TrailingContext(
                                    scanner = Atom("*"),
                                    not_followed_by = Choice([Atom("="), Atom("*")])
                                )
                            )]
                        ),
                        Token(
                            name = AsteriskEqual,
                            definitions = [TokenDefinition(scanner = Atom("*="))]
                        ),
                        Token(
                            name = AsteriskAsterisk,
                            definitions = [TokenDefinition(scanner = Atom("**"))]
                        ),
                        Token(
                            name = Bar,
                            definitions = [TokenDefinition(
                                scanner = TrailingContext(
                                    scanner = Atom("|"),
                                    not_followed_by = Choice([Atom("="), Atom("|")])
                                )
                            )]
                        ),
                        Token(
                            name = BarEqual,
                            definitions = [TokenDefinition(scanner = Atom("|="))]
                        ),
                        Token(
                            name = BarBar,
                            definitions = [TokenDefinition(scanner = Atom("||"))]
                        ),
                        Token(
                            name = Ampersand,
                            definitions = [TokenDefinition(
                                scanner = TrailingContext(
                                    scanner = Atom("&"),
                                    not_followed_by = Choice([Atom("="), Atom("&")])
                                )
                            )]
                        ),
                        Token(
                            name = AmpersandEqual,
                            definitions = [TokenDefinition(scanner = Atom("&="))]
                        ),
                        Token(
                            name = AmpersandAmpersand,
                            definitions = [TokenDefinition(scanner = Atom("&&"))]
                        ),
                        Token(
                            name = LessThan,
                            definitions = [TokenDefinition(
                                scanner = TrailingContext(
                                    scanner = Atom("<"),
                                    not_followed_by = Choice([Atom("="), Atom("<")])
                                )
                            )]
                        ),
                        Token(
                            name = LessThanEqual,
                            definitions = [TokenDefinition(scanner = Atom("<="))]
                        ),
                        Token(
                            name = LessThanLessThan,
                            definitions = [TokenDefinition(
                                scanner = TrailingContext(
                                    scanner = Atom("<<"),
                                    not_followed_by = Atom("=")
                                )
                            )]
                        ),
                        Token(
                            name = LessThanLessThanEqual,
                            definitions = [TokenDefinition(scanner = Atom("<<="))]
                        ),
                        Token(
                            name = GreaterThan,
                            definitions = [TokenDefinition(
                                scanner = TrailingContext(
                                    scanner = Atom(">"),
                                    not_followed_by = Choice([Atom("="), Atom(">")])
                                )
                            )]
                        ),
                        Token(
                            name = GreaterThanEqual,
                            definitions = [TokenDefinition(scanner = Atom(">="))]
                        ),
                        Token(
                            name = GreaterThanGreaterThan,
                            definitions = [TokenDefinition(
                                scanner = TrailingContext(
                                    scanner = Atom(">>"),
                                    not_followed_by = Choice([Atom("="), Atom(">")])
                                )
                            )]
                        ),
                        Token(
                            name = GreaterThanGreaterThanEqual,
                            definitions = [TokenDefinition(scanner = Atom(">>="))]
                        ),
                        Token(
                            name = GreaterThanGreaterThanGreaterThan,
                            definitions = [TokenDefinition(
                                scanner = TrailingContext(
                                    scanner = Atom(">>>"),
                                    not_followed_by = Atom("=")
                                )
                            )]
                        ),
                        Token(
                            name = GreaterThanGreaterThanGreaterThanEqual,
                            definitions = [TokenDefinition(scanner = Atom(">>>="))]
                        ),
                        Token(
                            name = Plus,
                            definitions = [TokenDefinition(
                                scanner = TrailingContext(
                                    scanner = Atom("+"),
                                    not_followed_by = Choice([Atom("="), Atom("+")])
                                )
                            )]
                        ),
                        Token(
                            name = PlusEqual,
                            definitions = [TokenDefinition(scanner = Atom("+="))]
                        ),
                        Token(
                            name = PlusPlus,
                            definitions = [TokenDefinition(scanner = Atom("++"))]
                        ),
                        Token(
                            name = Minus,
                            definitions = [TokenDefinition(
                                scanner = TrailingContext(
                                    scanner = Atom("-"),
                                    not_followed_by = Choice([Atom("="), Atom("-"), Atom(">")])
                                )
                            )]
                        ),
                        Token(
                            name = MinusEqual,
                            definitions = [TokenDefinition(scanner = Atom("-="))]
                        ),
                        Token(
                            name = MinusMinus,
                            definitions = [TokenDefinition(scanner = Atom("--"))]
                        ),
                        Token(
                            name = MinusGreaterThan,
                            definitions = [TokenDefinition(scanner = Atom("->"))]
                        ),
                        Token(
                            name = Slash,
                            definitions = [TokenDefinition(
                                scanner = TrailingContext(
                                    scanner = Atom("/"),
                                    not_followed_by = Atom("=")
                                )
                            )]
                        ),
                        Token(
                            name = SlashEqual,
                            definitions = [TokenDefinition(scanner = Atom("/="))]
                        ),
                        Token(
                            name = Percent,
                            definitions = [TokenDefinition(
                                scanner = TrailingContext(
                                    scanner = Atom("%"),
                                    not_followed_by = Atom("=")
                                )
                            )]
                        ),
                        Token(
                            name = PercentEqual,
                            definitions = [TokenDefinition(scanner = Atom("%="))]
                        ),
                        Token(
                            name = Bang,
                            definitions = [TokenDefinition(
                                scanner = TrailingContext(
                                    scanner = Atom("!"),
                                    not_followed_by = Atom("=")
                                )
                            )]
                        ),
                        Token(
                            name = BangEqual,
                            definitions = [TokenDefinition(scanner = Atom("!="))]
                        ),
                        Token(
                            name = Caret,
                            definitions = [TokenDefinition(
                                scanner = TrailingContext(
                                    scanner = Atom("^"),
                                    not_followed_by = Atom("=")
                                )
                            )]
                        ),
                        Token(
                            name = CaretEqual,
                            definitions = [TokenDefinition(scanner = Atom("^="))]
                        ),
                        Token(
                            name = Tilde,
                            definitions = [TokenDefinition(scanner = Atom("~"))]
                        )
                    ]
                )
            ]
        ),
        Section(
            title = "Definitions",
            topics = [
                Topic(
                    title = "Contracts",
                    items = [
                        Struct(
                            name = ContractDefinition,
                            error_recovery = FieldsErrorRecovery(
                                delimiters =
                                    FieldDelimiters(open = open_brace, close = close_brace)
                            ),
                            fields = (
                                abstract_keyword = Optional(
                                    kind = Terminal([AbstractKeyword]),
                                    enabled = From("0.6.0")
                                ),
                                contract_keyword = Required(Terminal([ContractKeyword])),
                                name = Required(Terminal([Identifier])),
                                inheritence = Optional(kind = NonTerminal(InheritanceSpecifier)),
                                open_brace = Required(Terminal([OpenBrace])),
                                members = Required(NonTerminal(ContractMembers)),
                                close_brace = Required(Terminal([CloseBrace]))
                            )
                        ),
                        Struct(
                            name = InheritanceSpecifier,
                            fields = (
                                is_keyword = Required(Terminal([IsKeyword])),
                                types = Required(NonTerminal(InheritanceTypes))
                            )
                        ),
                        Separated(
                            name = InheritanceTypes,
                            separated = InheritanceType,
                            separator = Comma
                        ),
                        Struct(
                            name = InheritanceType,
                            fields = (
                                type_name = Required(NonTerminal(IdentifierPath)),
                                arguments = Optional(kind = NonTerminal(ArgumentsDeclaration))
                            )
                        ),
                        Repeated(
                            name = ContractMembers,
                            repeated = ContractMember,
                            allow_empty = true
                        ),
                        Enum(
                            name = ContractMember,
                            variants = [
                                EnumVariant(name = Using, reference = UsingDirective),
                                EnumVariant(name = Function, reference = FunctionDefinition),
                                EnumVariant(
                                    name = Constructor,
                                    enabled = From("0.4.22"),
                                    reference = ConstructorDefinition
                                ),
                                EnumVariant(
                                    name = ReceiveFunction,
                                    enabled = From("0.6.0"),
                                    reference = ReceiveFunctionDefinition
                                ),
                                EnumVariant(
                                    name = FallbackFunction,
                                    enabled = From("0.6.0"),
                                    reference = FallbackFunctionDefinition
                                ),
                                EnumVariant(
                                    name = UnnamedFunction,
                                    enabled = Till("0.6.0"),
                                    reference = UnnamedFunctionDefinition
                                ),
                                EnumVariant(name = Modifier, reference = ModifierDefinition),
                                EnumVariant(name = Struct, reference = StructDefinition),
                                EnumVariant(name = Enum, reference = EnumDefinition),
                                EnumVariant(name = Event, reference = EventDefinition),
                                EnumVariant(
                                    name = StateVariable,
                                    reference = StateVariableDefinition
                                ),
                                EnumVariant(
                                    name = Error,
                                    enabled = From("0.8.4"),
                                    reference = ErrorDefinition
                                ),
                                EnumVariant(
                                    name = UserDefinedValueType,
                                    enabled = From("0.8.8"),
                                    reference = UserDefinedValueTypeDefinition
                                )
                            ]
                        )
                    ]
                ),
                Topic(
                    title = "Interfaces",
                    items = [
                        Struct(
                            name = InterfaceDefinition,
                            error_recovery = FieldsErrorRecovery(
                                delimiters =
                                    FieldDelimiters(open = open_brace, close = close_brace)
                            ),
                            fields = (
                                interface_keyword = Required(Terminal([InterfaceKeyword])),
                                name = Required(Terminal([Identifier])),
                                inheritence = Optional(kind = NonTerminal(InheritanceSpecifier)),
                                open_brace = Required(Terminal([OpenBrace])),
                                members = Required(NonTerminal(InterfaceMembers)),
                                close_brace = Required(Terminal([CloseBrace]))
                            )
                        ),
                        Repeated(
                            name = InterfaceMembers,
                            repeated = ContractMember,
                            allow_empty = true
                        )
                    ]
                ),
                Topic(
                    title = "Libraries",
                    items = [
                        Struct(
                            name = LibraryDefinition,
                            error_recovery = FieldsErrorRecovery(
                                delimiters =
                                    FieldDelimiters(open = open_brace, close = close_brace)
                            ),
                            fields = (
                                library_keyword = Required(Terminal([LibraryKeyword])),
                                name = Required(Terminal([Identifier])),
                                open_brace = Required(Terminal([OpenBrace])),
                                members = Required(NonTerminal(LibraryMembers)),
                                close_brace = Required(Terminal([CloseBrace]))
                            )
                        ),
                        Repeated(
                            name = LibraryMembers,
                            repeated = ContractMember,
                            allow_empty = true
                        )
                    ]
                ),
                Topic(
                    title = "Structs",
                    items = [
                        Struct(
                            name = StructDefinition,
                            error_recovery = FieldsErrorRecovery(
                                delimiters =
                                    FieldDelimiters(open = open_brace, close = close_brace)
                            ),
                            fields = (
                                struct_keyword = Required(Terminal([StructKeyword])),
                                name = Required(Terminal([Identifier])),
                                open_brace = Required(Terminal([OpenBrace])),
                                members = Required(NonTerminal(StructMembers)),
                                close_brace = Required(Terminal([CloseBrace]))
                            )
                        ),
                        Repeated(
                            name = StructMembers,
                            repeated = StructMember,
                            allow_empty = true
                        ),
                        Struct(
                            name = StructMember,
                            error_recovery = FieldsErrorRecovery(terminator = semicolon),
                            fields = (
                                type_name = Required(NonTerminal(TypeName)),
                                name = Required(Terminal([Identifier])),
                                semicolon = Required(Terminal([Semicolon]))
                            )
                        )
                    ]
                ),
                Topic(
                    title = "Enums",
                    items = [
                        Struct(
                            name = EnumDefinition,
                            error_recovery = FieldsErrorRecovery(
                                delimiters =
                                    FieldDelimiters(open = open_brace, close = close_brace)
                            ),
                            fields = (
                                enum_keyword = Required(Terminal([EnumKeyword])),
                                name = Required(Terminal([Identifier])),
                                open_brace = Required(Terminal([OpenBrace])),
                                members = Required(NonTerminal(EnumMembers)),
                                close_brace = Required(Terminal([CloseBrace]))
                            )
                        ),
                        Separated(
                            name = EnumMembers,
                            separated = Identifier,
                            separator = Comma,
                            allow_empty = true
                        )
                    ]
                ),
                Topic(
                    title = "Constants",
                    items = [Struct(
                        name = ConstantDefinition,
                        enabled = From("0.7.4"),
                        error_recovery = FieldsErrorRecovery(terminator = semicolon),
                        fields = (
                            type_name = Required(NonTerminal(TypeName)),
                            constant_keyword = Required(Terminal([ConstantKeyword])),
                            name = Required(Terminal([Identifier])),
                            equal = Required(Terminal([Equal])),
                            value = Required(NonTerminal(Expression)),
                            semicolon = Required(Terminal([Semicolon]))
                        )
                    )]
                ),
                Topic(
                    title = "State Variables",
                    items = [
                        Struct(
                            name = StateVariableDefinition,
                            error_recovery = FieldsErrorRecovery(terminator = semicolon),
                            fields = (
                                type_name = Required(NonTerminal(TypeName)),
                                attributes = Required(NonTerminal(StateVariableAttributes)),
                                name = Required(Terminal([Identifier])),
                                value = Optional(kind = NonTerminal(StateVariableDefinitionValue)),
                                semicolon = Required(Terminal([Semicolon]))
                            )
                        ),
                        Struct(
                            name = StateVariableDefinitionValue,
                            fields = (
                                equal = Required(Terminal([Equal])),
                                value = Required(NonTerminal(Expression))
                            )
                        ),
                        Repeated(
                            name = StateVariableAttributes,
                            repeated = StateVariableAttribute,
                            allow_empty = true
                        ),
                        Enum(
                            name = StateVariableAttribute,
                            variants = [
                                EnumVariant(name = Override, reference = OverrideSpecifier),
                                EnumVariant(name = Constant, reference = ConstantKeyword),
                                EnumVariant(name = Internal, reference = InternalKeyword),
                                EnumVariant(name = Private, reference = PrivateKeyword),
                                EnumVariant(name = Public, reference = PublicKeyword),
                                EnumVariant(
                                    name = Immutable,
                                    enabled = From("0.6.5"),
                                    reference = ImmutableKeyword
                                )
                            ]
                        )
                    ]
                ),
                Topic(
                    title = "Functions",
                    items = [
                        Struct(
                            name = FunctionDefinition,
                            fields = (
                                function_keyword = Required(Terminal([FunctionKeyword])),
                                name = Required(Terminal([
                                    Identifier,
                                    FallbackKeyword,
                                    ReceiveKeyword
                                ])),
                                parameters = Required(NonTerminal(ParametersDeclaration)),
                                attributes = Required(NonTerminal(FunctionAttributes)),
                                returns = Optional(kind = NonTerminal(ReturnsDeclaration)),
                                body = Required(NonTerminal(FunctionBody))
                            )
                        ),
                        Struct(
                            name = ParametersDeclaration,
                            error_recovery = FieldsErrorRecovery(
                                delimiters =
                                    FieldDelimiters(open = open_paren, close = close_paren)
                            ),
                            fields = (
                                open_paren = Required(Terminal([OpenParen])),
                                parameters = Required(NonTerminal(Parameters)),
                                close_paren = Required(Terminal([CloseParen]))
                            )
                        ),
                        Separated(
                            name = Parameters,
                            separated = Parameter,
                            separator = Comma,
                            allow_empty = true
                        ),
                        Struct(
                            name = Parameter,
                            fields = (
                                type_name = Required(NonTerminal(TypeName)),
                                storage_location = Optional(kind = NonTerminal(StorageLocation)),
                                name = Optional(kind = Terminal([Identifier]))
                            )
                        ),
                        Repeated(
                            name = FunctionAttributes,
                            repeated = FunctionAttribute,
                            allow_empty = true
                        ),
                        Enum(
                            name = FunctionAttribute,
                            variants = [
                                EnumVariant(name = Modifier, reference = ModifierInvocation),
                                EnumVariant(name = Override, reference = OverrideSpecifier),
                                EnumVariant(
                                    name = Constant,
                                    enabled = Till("0.5.0"),
                                    reference = ConstantKeyword
                                ),
                                EnumVariant(name = External, reference = ExternalKeyword),
                                EnumVariant(name = Internal, reference = InternalKeyword),
                                EnumVariant(name = Payable, reference = PayableKeyword),
                                EnumVariant(name = Private, reference = PrivateKeyword),
                                EnumVariant(name = Public, reference = PublicKeyword),
                                EnumVariant(name = Pure, reference = PureKeyword),
                                EnumVariant(name = View, reference = ViewKeyword),
                                EnumVariant(
                                    name = Virtual,
                                    enabled = From("0.6.0"),
                                    reference = VirtualKeyword
                                )
                            ]
                        ),
                        Struct(
                            name = OverrideSpecifier,
                            error_recovery = FieldsErrorRecovery(
                                delimiters =
                                    FieldDelimiters(open = open_paren, close = close_paren)
                            ),
                            fields = (
                                override_keyword = Required(Terminal([OverrideKeyword])),
                                open_paren = Required(Terminal([OpenParen])),
                                overridden = Required(NonTerminal(OverridePaths)),
                                close_paren = Required(Terminal([CloseParen]))
                            )
                        ),
                        Separated(
                            name = OverridePaths,
                            separated = IdentifierPath,
                            separator = Comma
                        ),
                        Struct(
                            name = ReturnsDeclaration,
                            fields = (
                                returns_keyword = Required(Terminal([ReturnsKeyword])),
                                variables = Required(NonTerminal(ParametersDeclaration))
                            )
                        ),
                        Enum(
                            name = FunctionBody,
                            variants = [
                                EnumVariant(name = Block, reference = Block),
                                EnumVariant(name = Semicolon, reference = Semicolon)
                            ]
                        ),
                        Struct(
                            name = ConstructorDefinition,
                            enabled = From("0.4.22"),
                            fields = (
                                constructor_keyword = Required(Terminal([ConstructorKeyword])),
                                parameters = Required(NonTerminal(ParametersDeclaration)),
                                attributes = Required(NonTerminal(ConstructorAttributes)),
                                body = Required(NonTerminal(Block))
                            )
                        ),
                        Repeated(
                            name = ConstructorAttributes,
                            repeated = ConstructorAttribute,
                            enabled = From("0.4.22"),
                            allow_empty = true
                        ),
                        Enum(
                            name = ConstructorAttribute,
                            enabled = From("0.4.22"),
                            variants = [
                                EnumVariant(name = Modifier, reference = ModifierInvocation),
                                EnumVariant(name = Override, reference = OverrideSpecifier),
                                EnumVariant(name = Payable, reference = PayableKeyword),
                                EnumVariant(name = Public, reference = PublicKeyword)
                            ]
                        ),
                        Struct(
                            name = UnnamedFunctionDefinition,
                            enabled = Till("0.6.0"),
                            fields = (
                                function_keyword = Required(Terminal([FunctionKeyword])),
                                parameters = Required(NonTerminal(ParametersDeclaration)),
                                attributes = Required(NonTerminal(UnnamedFunctionAttributes)),
                                body = Required(NonTerminal(FunctionBody))
                            )
                        ),
                        Repeated(
                            name = UnnamedFunctionAttributes,
                            repeated = UnnamedFunctionAttribute,
                            enabled = Till("0.6.0"),
                            allow_empty = true
                        ),
                        Enum(
                            name = UnnamedFunctionAttribute,
                            enabled = Till("0.6.0"),
                            variants = [
                                EnumVariant(name = Modifier, reference = ModifierInvocation),
                                EnumVariant(name = Override, reference = OverrideSpecifier),
                                EnumVariant(name = External, reference = ExternalKeyword),
                                EnumVariant(name = Payable, reference = PayableKeyword),
                                EnumVariant(name = Pure, reference = PureKeyword),
                                EnumVariant(name = View, reference = ViewKeyword)
                            ]
                        ),
                        Struct(
                            name = FallbackFunctionDefinition,
                            enabled = From("0.6.0"),
                            fields = (
                                fallback_keyword = Required(Terminal([FallbackKeyword])),
                                parameters = Required(NonTerminal(ParametersDeclaration)),
                                attributes = Required(NonTerminal(FallbackFunctionAttributes)),
                                returns = Optional(kind = NonTerminal(ReturnsDeclaration)),
                                body = Required(NonTerminal(FunctionBody))
                            )
                        ),
                        Repeated(
                            name = FallbackFunctionAttributes,
                            repeated = FallbackFunctionAttribute,
                            enabled = From("0.6.0"),
                            allow_empty = true
                        ),
                        Enum(
                            name = FallbackFunctionAttribute,
                            enabled = From("0.6.0"),
                            variants = [
                                EnumVariant(name = Modifier, reference = ModifierInvocation),
                                EnumVariant(name = Override, reference = OverrideSpecifier),
                                EnumVariant(name = External, reference = ExternalKeyword),
                                EnumVariant(name = Payable, reference = PayableKeyword),
                                EnumVariant(name = Pure, reference = PureKeyword),
                                EnumVariant(name = View, reference = ViewKeyword),
                                EnumVariant(name = Virtual, reference = VirtualKeyword)
                            ]
                        ),
                        Struct(
                            name = ReceiveFunctionDefinition,
                            enabled = From("0.6.0"),
                            fields = (
                                receive_keyword = Required(Terminal([ReceiveKeyword])),
                                parameters = Required(NonTerminal(ParametersDeclaration)),
                                attributes = Required(NonTerminal(ReceiveFunctionAttributes)),
                                body = Required(NonTerminal(FunctionBody))
                            )
                        ),
                        Repeated(
                            name = ReceiveFunctionAttributes,
                            repeated = ReceiveFunctionAttribute,
                            enabled = From("0.6.0"),
                            allow_empty = true
                        ),
                        Enum(
                            name = ReceiveFunctionAttribute,
                            enabled = From("0.6.0"),
                            variants = [
                                EnumVariant(name = Modifier, reference = ModifierInvocation),
                                EnumVariant(name = Override, reference = OverrideSpecifier),
                                EnumVariant(name = External, reference = ExternalKeyword),
                                EnumVariant(name = Payable, reference = PayableKeyword),
                                EnumVariant(name = Virtual, reference = VirtualKeyword)
                            ]
                        )
                    ]
                ),
                Topic(
                    title = "Modifiers",
                    items = [
                        Struct(
                            name = ModifierDefinition,
                            fields = (
                                modifier_keyword = Required(Terminal([ModifierKeyword])),
                                name = Required(Terminal([Identifier])),
                                parameters = Optional(kind = NonTerminal(ParametersDeclaration)),
                                attributes = Required(NonTerminal(ModifierAttributes)),
                                body = Required(NonTerminal(FunctionBody))
                            )
                        ),
                        Repeated(
                            name = ModifierAttributes,
                            repeated = ModifierAttribute,
                            allow_empty = true
                        ),
                        Enum(
                            name = ModifierAttribute,
                            variants = [
                                EnumVariant(name = Override, reference = OverrideSpecifier),
                                EnumVariant(
                                    name = Virtual,
                                    enabled = From("0.6.0"),
                                    reference = VirtualKeyword
                                )
                            ]
                        ),
                        Struct(
                            name = ModifierInvocation,
                            fields = (
                                name = Required(NonTerminal(IdentifierPath)),
                                arguments = Optional(kind = NonTerminal(ArgumentsDeclaration))
                            )
                        )
                    ]
                ),
                Topic(
                    title = "Events",
                    items = [
                        Struct(
                            name = EventDefinition,
                            error_recovery = FieldsErrorRecovery(terminator = semicolon),
                            fields = (
                                event_keyword = Required(Terminal([EventKeyword])),
                                name = Required(Terminal([Identifier])),
                                parameters =
                                    Optional(kind = NonTerminal(EventParametersDeclaration)),
                                anonymous_keyword = Optional(kind = Terminal([AnonymousKeyword])),
                                semicolon = Required(Terminal([Semicolon]))
                            )
                        ),
                        Struct(
                            name = EventParametersDeclaration,
                            error_recovery = FieldsErrorRecovery(
                                delimiters =
                                    FieldDelimiters(open = open_paren, close = close_paren)
                            ),
                            fields = (
                                open_paren = Required(Terminal([OpenParen])),
                                parameters = Required(NonTerminal(EventParameters)),
                                close_paren = Required(Terminal([CloseParen]))
                            )
                        ),
                        Separated(
                            name = EventParameters,
                            separated = EventParameter,
                            separator = Comma,
                            allow_empty = true
                        ),
                        Struct(
                            name = EventParameter,
                            fields = (
                                type_name = Required(NonTerminal(TypeName)),
                                indexed_keyword = Optional(kind = Terminal([IndexedKeyword])),
                                name = Optional(kind = Terminal([Identifier]))
                            )
                        )
                    ]
                ),
                Topic(
                    title = "User Defined Value Types",
                    items = [Struct(
                        name = UserDefinedValueTypeDefinition,
                        enabled = From("0.8.8"),
                        error_recovery = FieldsErrorRecovery(terminator = semicolon),
                        fields = (
                            type_keyword = Required(Terminal([TypeKeyword])),
                            name = Required(Terminal([Identifier])),
                            is_keyword = Required(Terminal([IsKeyword])),
                            value_type = Required(NonTerminal(ElementaryType)),
                            semicolon = Required(Terminal([Semicolon]))
                        )
                    )]
                ),
                Topic(
                    title = "Errors",
                    items = [
                        Struct(
                            name = ErrorDefinition,
                            enabled = From("0.8.4"),
                            error_recovery = FieldsErrorRecovery(terminator = semicolon),
                            fields = (
                                error_keyword = Required(Terminal([ErrorKeyword])),
                                name = Required(Terminal([Identifier])),
                                members = Required(NonTerminal(ErrorParametersDeclaration)),
                                semicolon = Required(Terminal([Semicolon]))
                            )
                        ),
                        Struct(
                            name = ErrorParametersDeclaration,
                            enabled = From("0.8.4"),
                            error_recovery = FieldsErrorRecovery(
                                delimiters =
                                    FieldDelimiters(open = open_paren, close = close_paren)
                            ),
                            fields = (
                                open_paren = Required(Terminal([OpenParen])),
                                parameters = Required(NonTerminal(ErrorParameters)),
                                close_paren = Required(Terminal([CloseParen]))
                            )
                        ),
                        Separated(
                            name = ErrorParameters,
                            separated = ErrorParameter,
                            separator = Comma,
                            enabled = From("0.8.4"),
                            allow_empty = true
                        ),
                        Struct(
                            name = ErrorParameter,
                            enabled = From("0.8.4"),
                            fields = (
                                type_name = Required(NonTerminal(TypeName)),
                                name = Optional(kind = Terminal([Identifier]))
                            )
                        )
                    ]
                )
            ]
        ),
        Section(
            title = "Types",
            topics = [
                Topic(
                    title = "Advanced Types",
                    items = [
                        Precedence(
                            name = TypeName,
                            precedence_expressions = [PrecedenceExpression(
                                name = ArrayTypeName,
                                operators = [PrecedenceOperator(
                                    model = Postfix,
                                    error_recovery = FieldsErrorRecovery(
                                        delimiters = FieldDelimiters(
                                            open = open_bracket,
                                            close = close_bracket
                                        )
                                    ),
                                    fields = (
                                        open_bracket = Required(Terminal([OpenBracket])),
                                        index = Optional(kind = NonTerminal(Expression)),
                                        close_bracket = Required(Terminal([CloseBracket]))
                                    )
                                )]
                            )],
                            primary_expressions = [
                                PrimaryExpression(expression = FunctionType),
                                PrimaryExpression(expression = MappingType),
                                PrimaryExpression(expression = ElementaryType),
                                PrimaryExpression(expression = IdentifierPath)
                            ]
                        ),
                        Struct(
                            name = FunctionType,
                            fields = (
                                function_keyword = Required(Terminal([FunctionKeyword])),
                                parameters = Required(NonTerminal(ParametersDeclaration)),
                                attributes = Required(NonTerminal(FunctionTypeAttributes)),
                                returns = Optional(kind = NonTerminal(ReturnsDeclaration))
                            )
                        ),
                        Repeated(
                            name = FunctionTypeAttributes,
                            repeated = FunctionTypeAttribute,
                            allow_empty = true
                        ),
                        Enum(
                            name = FunctionTypeAttribute,
                            variants = [
                                EnumVariant(name = Internal, reference = InternalKeyword),
                                EnumVariant(name = External, reference = ExternalKeyword),
                                EnumVariant(name = Private, reference = PrivateKeyword),
                                EnumVariant(name = Public, reference = PublicKeyword),
                                EnumVariant(name = Pure, reference = PureKeyword),
                                EnumVariant(name = View, reference = ViewKeyword),
                                EnumVariant(name = Payable, reference = PayableKeyword)
                            ]
                        ),
                        Struct(
                            name = MappingType,
                            error_recovery = FieldsErrorRecovery(
                                delimiters =
                                    FieldDelimiters(open = open_paren, close = close_paren)
                            ),
                            fields = (
                                mapping_keyword = Required(Terminal([MappingKeyword])),
                                open_paren = Required(Terminal([OpenParen])),
                                key_type = Required(NonTerminal(MappingKey)),
                                equal_greater_than = Required(Terminal([EqualGreaterThan])),
                                value_type = Required(NonTerminal(MappingValue)),
                                close_paren = Required(Terminal([CloseParen]))
                            )
                        ),
                        Struct(
                            name = MappingKey,
                            fields = (
                                key_type = Required(NonTerminal(MappingKeyType)),
                                name = Optional(
                                    kind = Terminal([Identifier]),
                                    enabled = From("0.8.18")
                                )
                            )
                        ),
                        Enum(
                            name = MappingKeyType,
                            variants = [
                                EnumVariant(name = ElementaryType, reference = ElementaryType),
                                EnumVariant(name = IdentifierPath, reference = IdentifierPath)
                            ]
                        ),
                        Struct(
                            name = MappingValue,
                            fields = (
                                type_name = Required(NonTerminal(TypeName)),
                                name = Optional(
                                    kind = Terminal([Identifier]),
                                    enabled = From("0.8.18")
                                )
                            )
                        )
                    ]
                ),
                Topic(
                    title = "Elementary Types",
                    items = [
                        Enum(
                            name = ElementaryType,
                            variants = [
                                EnumVariant(name = Bool, reference = BoolKeyword),
                                EnumVariant(
                                    name = Byte,
                                    enabled = Till("0.8.0"),
                                    reference = ByteKeyword
                                ),
                                EnumVariant(name = String, reference = StringKeyword),
                                EnumVariant(name = Address, reference = AddressType),
                                EnumVariant(name = Payable, reference = PayableKeyword),
                                EnumVariant(name = ByteArray, reference = BytesKeyword),
                                EnumVariant(name = SignedInteger, reference = IntKeyword),
                                EnumVariant(name = UnsignedInteger, reference = UintKeyword),
                                EnumVariant(
                                    name = SignedFixedPointNumber,
                                    reference = FixedKeyword
                                ),
                                EnumVariant(
                                    name = UnsignedFixedPointNumber,
                                    reference = UfixedKeyword
                                )
                            ]
                        ),
                        Struct(
                            name = AddressType,
                            fields = (
                                address_keyword = Required(Terminal([AddressKeyword])),
                                payable_keyword = Optional(kind = Terminal([PayableKeyword]))
                            )
                        )
                    ]
                )
            ]
        ),
        Section(
            title = "Statements",
            topics = [
                Topic(
                    title = "Blocks",
                    items = [
                        Struct(
                            name = Block,
                            error_recovery = FieldsErrorRecovery(
                                delimiters =
                                    FieldDelimiters(open = open_brace, close = close_brace)
                            ),
                            fields = (
                                open_brace = Required(Terminal([OpenBrace])),
                                statements = Required(NonTerminal(Statements)),
                                close_brace = Required(Terminal([CloseBrace]))
                            )
                        ),
                        Repeated(name = Statements, repeated = Statement, allow_empty = true),
                        Enum(
                            name = Statement,
                            variants = [
                                EnumVariant(
                                    name = TupleDeconstruction,
                                    reference = TupleDeconstructionStatement
                                ),
                                EnumVariant(
                                    name = VariableDeclaration,
                                    reference = VariableDeclarationStatement
                                ),
                                EnumVariant(name = If, reference = IfStatement),
                                EnumVariant(name = For, reference = ForStatement),
                                EnumVariant(name = While, reference = WhileStatement),
                                EnumVariant(name = DoWhile, reference = DoWhileStatement),
                                EnumVariant(name = Continue, reference = ContinueStatement),
                                EnumVariant(name = Break, reference = BreakStatement),
                                EnumVariant(name = Delete, reference = DeleteStatement),
                                EnumVariant(name = Return, reference = ReturnStatement),
                                EnumVariant(
                                    name = Throw,
                                    enabled = Till("0.5.0"),
                                    reference = ThrowStatement
                                ),
                                EnumVariant(
                                    name = Emit,
                                    enabled = From("0.4.21"),
                                    reference = EmitStatement
                                ),
                                EnumVariant(
                                    name = Try,
                                    enabled = From("0.6.0"),
                                    reference = TryStatement
                                ),
                                EnumVariant(
                                    name = Revert,
                                    enabled = From("0.8.4"),
                                    reference = RevertStatement
                                ),
                                EnumVariant(name = Assembly, reference = AssemblyStatement),
                                EnumVariant(name = Block, reference = Block),
                                EnumVariant(
                                    name = UncheckedBlock,
                                    enabled = From("0.8.0"),
                                    reference = UncheckedBlock
                                ),
                                EnumVariant(name = Expression, reference = ExpressionStatement)
                            ]
                        ),
                        Struct(
                            name = UncheckedBlock,
                            enabled = From("0.8.0"),
                            fields = (
                                unchecked_keyword = Required(Terminal([UncheckedKeyword])),
                                block = Required(NonTerminal(Block))
                            )
                        ),
                        Struct(
                            name = ExpressionStatement,
                            error_recovery = FieldsErrorRecovery(terminator = semicolon),
                            fields = (
                                expression = Required(NonTerminal(Expression)),
                                semicolon = Required(Terminal([Semicolon]))
                            )
                        )
                    ]
                ),
                Topic(
                    title = "Declaration Statements",
                    items = [
                        Struct(
                            name = TupleDeconstructionStatement,
                            error_recovery = FieldsErrorRecovery(
                                terminator = semicolon,
                                delimiters =
                                    FieldDelimiters(open = open_paren, close = close_paren)
                            ),
                            fields = (
                                open_paren = Required(Terminal([OpenParen])),
                                members = Required(NonTerminal(TupleMembersDeconstruction)),
                                close_paren = Required(Terminal([CloseParen])),
                                equal = Required(Terminal([Equal])),
                                expression = Required(NonTerminal(Expression)),
                                semicolon = Required(Terminal([Semicolon]))
                            )
                        ),
                        Separated(
                            name = TupleMembersDeconstruction,
                            separated = TupleMemberDeconstruction,
                            separator = Comma,
                            allow_empty = true
                        ),
                        Struct(
                            name = TupleMemberDeconstruction,
                            fields = (member = Optional(kind = NonTerminal(TupleMember)))
                        ),
                        Enum(
                            name = TupleMember,
                            variants = [
                                EnumVariant(name = Typed, reference = TypedTupleMember),
                                EnumVariant(name = Untyped, reference = UntypedTupleMember)
                            ]
                        ),
                        Struct(
                            name = TypedTupleMember,
                            fields = (
                                type_name = Required(NonTerminal(TypeName)),
                                storage_location = Optional(kind = NonTerminal(StorageLocation)),
                                name = Required(Terminal([Identifier]))
                            )
                        ),
                        Struct(
                            name = UntypedTupleMember,
                            fields = (
                                storage_location = Optional(kind = NonTerminal(StorageLocation)),
                                name = Required(Terminal([Identifier]))
                            )
                        ),
                        Struct(
                            name = VariableDeclarationStatement,
                            error_recovery = FieldsErrorRecovery(terminator = semicolon),
                            fields = (
                                variable_type = Required(NonTerminal(VariableDeclarationType)),
                                storage_location = Optional(kind = NonTerminal(StorageLocation)),
                                name = Required(Terminal([Identifier])),
                                value = Optional(kind = NonTerminal(VariableDeclarationValue)),
                                semicolon = Required(Terminal([Semicolon]))
                            )
                        ),
                        Enum(
                            name = VariableDeclarationType,
                            variants = [
                                EnumVariant(name = Typed, reference = TypeName),
                                EnumVariant(
                                    name = Untyped,
                                    enabled = Till("0.5.0"),
                                    reference = VarKeyword
                                )
                            ]
                        ),
                        Struct(
                            name = VariableDeclarationValue,
                            fields = (
                                equal = Required(Terminal([Equal])),
                                expression = Required(NonTerminal(Expression))
                            )
                        ),
                        Enum(
                            name = StorageLocation,
                            variants = [
                                EnumVariant(name = Memory, reference = MemoryKeyword),
                                EnumVariant(name = Storage, reference = StorageKeyword),
                                EnumVariant(
                                    name = CallData,
                                    enabled = From("0.5.0"),
                                    reference = CallDataKeyword
                                )
                            ]
                        )
                    ]
                ),
                Topic(
                    title = "Control Statements",
                    items = [
                        Struct(
                            name = IfStatement,
                            error_recovery = FieldsErrorRecovery(
                                delimiters =
                                    FieldDelimiters(open = open_paren, close = close_paren)
                            ),
                            fields = (
                                if_keyword = Required(Terminal([IfKeyword])),
                                open_paren = Required(Terminal([OpenParen])),
                                condition = Required(NonTerminal(Expression)),
                                close_paren = Required(Terminal([CloseParen])),
                                body = Required(NonTerminal(Statement)),
                                else_branch = Optional(kind = NonTerminal(ElseBranch))
                            )
                        ),
                        Struct(
                            name = ElseBranch,
                            fields = (
                                else_keyword = Optional(kind = Terminal([ElseKeyword])),
                                body = Optional(kind = NonTerminal(Statement))
                            )
                        ),
                        Struct(
                            name = ForStatement,
                            error_recovery = FieldsErrorRecovery(
                                delimiters =
                                    FieldDelimiters(open = open_paren, close = close_paren)
                            ),
                            fields = (
                                for_keyword = Required(Terminal([ForKeyword])),
                                open_paren = Required(Terminal([OpenParen])),
                                initialization = Required(NonTerminal(ForStatementInitialization)),
                                condition = Required(NonTerminal(ForStatementCondition)),
                                iterator = Optional(kind = NonTerminal(Expression)),
                                close_paren = Required(Terminal([CloseParen])),
                                body = Required(NonTerminal(Statement))
                            )
                        ),
                        Enum(
                            name = ForStatementInitialization,
                            variants = [
                                EnumVariant(name = Expression, reference = ExpressionStatement),
                                EnumVariant(
                                    name = VariableDeclaration,
                                    reference = VariableDeclarationStatement
                                ),
                                EnumVariant(
                                    name = TupleDeconstruction,
                                    reference = TupleDeconstructionStatement
                                ),
                                EnumVariant(name = Semicolon, reference = Semicolon)
                            ]
                        ),
                        Enum(
                            name = ForStatementCondition,
                            variants = [
                                EnumVariant(name = Expression, reference = ExpressionStatement),
                                EnumVariant(name = Semicolon, reference = Semicolon)
                            ]
                        ),
                        Struct(
                            name = WhileStatement,
                            error_recovery = FieldsErrorRecovery(
                                delimiters =
                                    FieldDelimiters(open = open_paren, close = close_paren)
                            ),
                            fields = (
                                while_keyword = Required(Terminal([WhileKeyword])),
                                open_paren = Required(Terminal([OpenParen])),
                                condition = Required(NonTerminal(Expression)),
                                close_paren = Required(Terminal([CloseParen])),
                                body = Required(NonTerminal(Statement))
                            )
                        ),
                        Struct(
                            name = DoWhileStatement,
                            error_recovery = FieldsErrorRecovery(
                                terminator = semicolon,
                                delimiters =
                                    FieldDelimiters(open = open_paren, close = close_paren)
                            ),
                            fields = (
                                do_keyword = Required(Terminal([DoKeyword])),
                                body = Required(NonTerminal(Statement)),
                                while_keyword = Required(Terminal([WhileKeyword])),
                                open_paren = Required(Terminal([OpenParen])),
                                condition = Required(NonTerminal(Expression)),
                                close_paren = Required(Terminal([CloseParen])),
                                semicolon = Required(Terminal([Semicolon]))
                            )
                        ),
                        Struct(
                            name = ContinueStatement,
                            error_recovery = FieldsErrorRecovery(terminator = semicolon),
                            fields = (
                                continue_keyword = Required(Terminal([ContinueKeyword])),
                                semicolon = Required(Terminal([Semicolon]))
                            )
                        ),
                        Struct(
                            name = BreakStatement,
                            error_recovery = FieldsErrorRecovery(terminator = semicolon),
                            fields = (
                                break_keyword = Required(Terminal([BreakKeyword])),
                                semicolon = Required(Terminal([Semicolon]))
                            )
                        ),
                        Struct(
                            name = ReturnStatement,
                            error_recovery = FieldsErrorRecovery(terminator = semicolon),
                            fields = (
                                return_keyword = Required(Terminal([ReturnKeyword])),
                                expression = Optional(kind = NonTerminal(Expression)),
                                semicolon = Required(Terminal([Semicolon]))
                            )
                        ),
                        Struct(
                            name = EmitStatement,
                            enabled = From("0.4.21"),
                            error_recovery = FieldsErrorRecovery(terminator = semicolon),
                            fields = (
                                emit_keyword = Required(Terminal([EmitKeyword])),
                                event = Required(NonTerminal(IdentifierPath)),
                                arguments = Required(NonTerminal(ArgumentsDeclaration)),
                                semicolon = Required(Terminal([Semicolon]))
                            )
                        ),
                        Struct(
                            name = DeleteStatement,
                            error_recovery = FieldsErrorRecovery(terminator = semicolon),
                            fields = (
                                delete_keyword = Required(Terminal([DeleteKeyword])),
                                expression = Required(NonTerminal(Expression)),
                                semicolon = Required(Terminal([Semicolon]))
                            )
                        )
                    ]
                ),
                Topic(
                    title = "Error Handling",
                    items = [
                        Struct(
                            name = TryStatement,
                            enabled = From("0.6.0"),
                            fields = (
                                try_keyword = Required(Terminal([TryKeyword])),
                                expression = Required(NonTerminal(Expression)),
                                returns = Optional(kind = NonTerminal(ReturnsDeclaration)),
                                body = Required(NonTerminal(Block)),
                                catch_clauses = Required(NonTerminal(CatchClauses))
                            )
                        ),
                        Repeated(
                            name = CatchClauses,
                            repeated = CatchClause,
                            enabled = From("0.6.0")
                        ),
                        Struct(
                            name = CatchClause,
                            enabled = From("0.6.0"),
                            fields = (
                                catch_keyword = Required(Terminal([CatchKeyword])),
                                error = Optional(kind = NonTerminal(CatchClauseError)),
                                body = Required(NonTerminal(Block))
                            )
                        ),
                        Struct(
                            name = CatchClauseError,
                            enabled = From("0.6.0"),
                            fields = (
                                name = Optional(kind = Terminal([Identifier])),
                                parameters = Required(NonTerminal(ParametersDeclaration))
                            )
                        ),
                        Struct(
                            name = RevertStatement,
                            enabled = From("0.8.4"),
                            error_recovery = FieldsErrorRecovery(terminator = semicolon),
                            fields = (
                                revert_keyword = Required(Terminal([RevertKeyword])),
                                error = Optional(kind = NonTerminal(IdentifierPath)),
                                arguments = Required(NonTerminal(ArgumentsDeclaration)),
                                semicolon = Required(Terminal([Semicolon]))
                            )
                        ),
                        Struct(
                            name = ThrowStatement,
                            enabled = Till("0.5.0"),
                            error_recovery = FieldsErrorRecovery(terminator = semicolon),
                            fields = (
                                throw_keyword = Required(Terminal([ThrowKeyword])),
                                semicolon = Required(Terminal([Semicolon]))
                            )
                        )
                    ]
                )
            ]
        ),
        Section(
            title = "Expressions",
            topics = [
                Topic(
                    title = "Base Expressions",
                    items = [
                        Precedence(
                            name = Expression,
                            precedence_expressions = [
                                PrecedenceExpression(
                                    name = AssignmentExpression,
                                    operators = [PrecedenceOperator(
                                        model = BinaryLeftAssociative,
                                        fields = (operator = Required(Terminal([
                                            Equal,
                                            BarEqual,
                                            PlusEqual,
                                            MinusEqual,
                                            CaretEqual,
                                            SlashEqual,
                                            PercentEqual,
                                            AsteriskEqual,
                                            AmpersandEqual,
                                            LessThanLessThanEqual,
                                            GreaterThanGreaterThanEqual,
                                            GreaterThanGreaterThanGreaterThanEqual
                                        ])))
                                    )]
                                ),
                                PrecedenceExpression(
                                    name = ConditionalExpression,
                                    operators = [PrecedenceOperator(
                                        model = Postfix,
                                        fields = (
                                            question_mark = Required(Terminal([QuestionMark])),
                                            true_expression = Required(NonTerminal(Expression)),
                                            colon = Required(Terminal([Colon])),
                                            false_expression = Required(NonTerminal(Expression))
                                        )
                                    )]
                                ),
                                PrecedenceExpression(
                                    name = OrExpression,
                                    operators = [PrecedenceOperator(
                                        model = BinaryLeftAssociative,
                                        fields = (operator = Required(Terminal([BarBar])))
                                    )]
                                ),
                                PrecedenceExpression(
                                    name = AndExpression,
                                    operators = [PrecedenceOperator(
                                        model = BinaryLeftAssociative,
                                        fields =
                                            (operator = Required(Terminal([AmpersandAmpersand])))
                                    )]
                                ),
                                PrecedenceExpression(
                                    name = EqualityExpression,
                                    operators = [PrecedenceOperator(
                                        model = BinaryLeftAssociative,
                                        fields = (operator =
                                            Required(Terminal([EqualEqual, BangEqual])))
                                    )]
                                ),
                                PrecedenceExpression(
                                    name = ComparisonExpression,
                                    operators = [PrecedenceOperator(
                                        model = BinaryLeftAssociative,
                                        fields = (operator = Required(Terminal([
                                            LessThan,
                                            GreaterThan,
                                            LessThanEqual,
                                            GreaterThanEqual
                                        ])))
                                    )]
                                ),
                                PrecedenceExpression(
                                    name = BitwiseOrExpression,
                                    operators = [PrecedenceOperator(
                                        model = BinaryLeftAssociative,
                                        fields = (operator = Required(Terminal([Bar])))
                                    )]
                                ),
                                PrecedenceExpression(
                                    name = BitwiseXorExpression,
                                    operators = [PrecedenceOperator(
                                        model = BinaryLeftAssociative,
                                        fields = (operator = Required(Terminal([Caret])))
                                    )]
                                ),
                                PrecedenceExpression(
                                    name = BitwiseAndExpression,
                                    operators = [PrecedenceOperator(
                                        model = BinaryLeftAssociative,
                                        fields = (operator = Required(Terminal([Ampersand])))
                                    )]
                                ),
                                PrecedenceExpression(
                                    name = ShiftExpression,
                                    operators = [PrecedenceOperator(
                                        model = BinaryLeftAssociative,
                                        fields = (operator = Required(Terminal([
                                            LessThanLessThan,
                                            GreaterThanGreaterThan,
                                            GreaterThanGreaterThanGreaterThan
                                        ])))
                                    )]
                                ),
                                PrecedenceExpression(
                                    name = AdditiveExpression,
                                    operators = [PrecedenceOperator(
                                        model = BinaryLeftAssociative,
                                        fields = (operator = Required(Terminal([Plus, Minus])))
                                    )]
                                ),
                                PrecedenceExpression(
                                    name = MultiplicativeExpression,
                                    operators = [PrecedenceOperator(
                                        model = BinaryLeftAssociative,
                                        fields = (operator =
                                            Required(Terminal([Asterisk, Slash, Percent])))
                                    )]
                                ),
                                PrecedenceExpression(
                                    name = ExponentiationExpression,
                                    operators = [
                                        // Before '0.6.0', it was left-associative:
                                        PrecedenceOperator(
                                            model = BinaryLeftAssociative,
                                            enabled = Till("0.6.0"),
                                            fields =
                                                (operator = Required(Terminal([AsteriskAsterisk])))
                                        ),
                                        // In '0.6.0', it became right-associative:
                                        PrecedenceOperator(
                                            model = BinaryRightAssociative,
                                            enabled = From("0.6.0"),
                                            fields =
                                                (operator = Required(Terminal([AsteriskAsterisk])))
                                        )
                                    ]
                                ),
                                PrecedenceExpression(
                                    name = PostfixExpression,
                                    operators = [PrecedenceOperator(
                                        model = Postfix,
                                        fields =
                                            (operator = Required(Terminal([PlusPlus, MinusMinus])))
                                    )]
                                ),
                                PrecedenceExpression(
                                    name = PrefixExpression,
                                    operators = [
                                        // Before '0.5.0', 'Plus' was supported:
                                        PrecedenceOperator(
                                            model = Prefix,
                                            enabled = Till("0.5.0"),
                                            fields = (operator = Required(Terminal([
                                                PlusPlus, MinusMinus, Tilde, Bang, Minus, Plus
                                            ])))
                                        ),
                                        // In '0.5.0', 'Plus' was removed:
                                        PrecedenceOperator(
                                            model = Prefix,
                                            enabled = From("0.5.0"),
                                            fields = (operator = Required(Terminal([
                                                PlusPlus, MinusMinus, Tilde, Bang, Minus
                                            ])))
                                        )
                                    ]
                                ),
                                PrecedenceExpression(
                                    name = FunctionCallExpression,
                                    operators = [PrecedenceOperator(
                                        model = Postfix,
                                        fields = (
                                            options = Optional(
                                                kind = NonTerminal(FunctionCallOptions),
                                                enabled = From("0.6.2")
                                            ),
                                            arguments = Required(NonTerminal(ArgumentsDeclaration))
                                        )
                                    )]
                                ),
                                PrecedenceExpression(
                                    name = MemberAccessExpression,
                                    operators = [PrecedenceOperator(
                                        model = Postfix,
                                        fields = (
                                            period = Required(Terminal([Period])),
                                            member =
                                                Required(Terminal([Identifier, AddressKeyword]))
                                        )
                                    )]
                                ),
                                PrecedenceExpression(
                                    name = IndexAccessExpression,
                                    operators = [PrecedenceOperator(
                                        model = Postfix,
                                        error_recovery = FieldsErrorRecovery(
                                            delimiters = FieldDelimiters(
                                                open = open_bracket,
                                                close = close_bracket
                                            )
                                        ),
                                        fields = (
                                            open_bracket = Required(Terminal([OpenBracket])),
                                            start = Optional(kind = NonTerminal(Expression)),
                                            end = Optional(kind = NonTerminal(IndexAccessEnd)),
                                            close_bracket = Required(Terminal([CloseBracket]))
                                        )
                                    )]
                                )
                            ],
                            primary_expressions = [
                                PrimaryExpression(expression = NewExpression),
                                PrimaryExpression(expression = TupleExpression),
                                PrimaryExpression(
                                    expression = TypeExpression,
                                    enabled = From("0.5.3")
                                ),
                                PrimaryExpression(expression = ArrayExpression),
                                PrimaryExpression(expression = HexNumberExpression),
                                PrimaryExpression(expression = DecimalNumberExpression),
                                PrimaryExpression(expression = StringExpression),
                                PrimaryExpression(expression = ElementaryType),
                                PrimaryExpression(expression = TrueKeyword),
                                PrimaryExpression(expression = FalseKeyword),
                                PrimaryExpression(expression = Identifier)
                            ]
                        ),
                        Struct(
                            name = IndexAccessEnd,
                            fields = (
                                colon = Required(Terminal([Colon])),
                                end = Optional(kind = NonTerminal(Expression))
                            )
                        )
                    ]
                ),
                Topic(
                    title = "Function Calls",
                    items = [
                        Enum(
                            name = FunctionCallOptions,
                            enabled = From("0.6.2"),
                            variants = [
                                EnumVariant(
                                    name = Multiple,
                                    enabled = Range(from = "0.6.2", till = "0.8.0"),
                                    reference = NamedArgumentGroups
                                ),
                                EnumVariant(
                                    name = Single,
                                    enabled = From("0.8.0"),
                                    reference = NamedArgumentGroup
                                )
                            ]
                        ),
                        Enum(
                            name = ArgumentsDeclaration,
                            variants = [
                                EnumVariant(
                                    name = Positional,
                                    reference = PositionalArgumentsDeclaration
                                ),
                                EnumVariant(name = Named, reference = NamedArgumentsDeclaration)
                            ]
                        ),
                        Struct(
                            name = PositionalArgumentsDeclaration,
                            error_recovery = FieldsErrorRecovery(
                                delimiters =
                                    FieldDelimiters(open = open_paren, close = close_paren)
                            ),
                            fields = (
                                open_paren = Required(Terminal([OpenParen])),
                                arguments = Required(NonTerminal(PositionalArguments)),
                                close_paren = Required(Terminal([CloseParen]))
                            )
                        ),
                        Separated(
                            name = PositionalArguments,
                            separated = Expression,
                            separator = Comma,
                            allow_empty = true
                        ),
                        Struct(
                            name = NamedArgumentsDeclaration,
                            error_recovery = FieldsErrorRecovery(
                                delimiters =
                                    FieldDelimiters(open = open_paren, close = close_paren)
                            ),
                            fields = (
                                open_paren = Required(Terminal([OpenParen])),
                                arguments = Optional(kind = NonTerminal(NamedArgumentGroup)),
                                close_paren = Required(Terminal([CloseParen]))
                            )
                        ),
                        Repeated(
                            name = NamedArgumentGroups,
                            repeated = NamedArgumentGroup,
                            enabled = Range(from = "0.6.2", till = "0.8.0")
                        ),
                        Struct(
                            name = NamedArgumentGroup,
                            error_recovery = FieldsErrorRecovery(
                                delimiters =
                                    FieldDelimiters(open = open_brace, close = close_brace)
                            ),
                            fields = (
                                open_brace = Required(Terminal([OpenBrace])),
                                arguments = Required(NonTerminal(NamedArguments)),
                                close_brace = Required(Terminal([CloseBrace]))
                            )
                        ),
                        Separated(
                            name = NamedArguments,
                            separated = NamedArgument,
                            separator = Comma,
                            allow_empty = true
                        ),
                        Struct(
                            name = NamedArgument,
                            fields = (
                                name = Required(Terminal([Identifier])),
                                colon = Required(Terminal([Colon])),
                                value = Required(NonTerminal(Expression))
                            )
                        )
                    ]
                ),
                Topic(
                    title = "Primary Expressions",
                    items = [
                        Struct(
                            name = TypeExpression,
                            enabled = From("0.5.3"),
                            error_recovery = FieldsErrorRecovery(
                                delimiters =
                                    FieldDelimiters(open = open_paren, close = close_paren)
                            ),
                            fields = (
                                type_keyword = Required(Terminal([TypeKeyword])),
                                open_paren = Required(Terminal([OpenParen])),
                                type_name = Required(NonTerminal(TypeName)),
                                close_paren = Required(Terminal([CloseParen]))
                            )
                        ),
                        Struct(
                            name = NewExpression,
                            fields = (
                                new_keyword = Required(Terminal([NewKeyword])),
                                type_name = Required(NonTerminal(TypeName))
                            )
                        ),
                        Struct(
                            name = TupleExpression,
                            error_recovery = FieldsErrorRecovery(
                                delimiters =
                                    FieldDelimiters(open = open_paren, close = close_paren)
                            ),
                            fields = (
                                open_paren = Required(Terminal([OpenParen])),
                                items = Required(NonTerminal(TupleValues)),
                                close_paren = Required(Terminal([CloseParen]))
                            )
                        ),
                        Separated(
                            name = TupleValues,
                            separated = TupleValue,
                            separator = Comma
                        ),
                        Struct(
                            name = TupleValue,
                            fields = (expression = Optional(kind = NonTerminal(Expression)))
                        ),
                        Struct(
                            name = ArrayExpression,
                            error_recovery = FieldsErrorRecovery(
                                delimiters =
                                    FieldDelimiters(open = open_bracket, close = close_bracket)
                            ),
                            fields = (
                                open_bracket = Required(Terminal([OpenBracket])),
                                items = Required(NonTerminal(ArrayValues)),
                                close_bracket = Required(Terminal([CloseBracket]))
                            )
                        ),
                        Separated(
                            name = ArrayValues,
                            separated = Expression,
                            separator = Comma
                        )
                    ]
                ),
                Topic(
                    title = "Numbers",
                    items = [
                        Struct(
                            name = HexNumberExpression,
                            fields = (
                                literal = Required(Terminal([HexLiteral])),
                                unit = Optional(
                                    kind = NonTerminal(NumberUnit),
                                    enabled = Till("0.5.0")
                                )
                            )
                        ),
                        Struct(
                            name = DecimalNumberExpression,
                            fields = (
                                literal = Required(Terminal([DecimalLiteral])),
                                unit = Optional(kind = NonTerminal(NumberUnit))
                            )
                        ),
                        Token(
                            name = HexLiteral,
                            definitions = [
                                // Lowercase "0x" enabled in all versions:
                                TokenDefinition(
                                    scanner = TrailingContext(
                                        scanner = Sequence([
                                            Atom("0x"),
                                            OneOrMore(Fragment(HexCharacter)),
                                            ZeroOrMore(Sequence([
                                                Atom("_"),
                                                OneOrMore(Fragment(HexCharacter))
                                            ]))
                                        ]),
                                        not_followed_by = Fragment(IdentifierStart)
                                    )
                                ),
                                // Uppercase "0X" only enabled before "0.5.0":
                                TokenDefinition(
                                    enabled = Till("0.5.0"),
                                    scanner = TrailingContext(
                                        scanner = Sequence([
                                            Atom("0X"),
                                            OneOrMore(Fragment(HexCharacter)),
                                            ZeroOrMore(Sequence([
                                                Atom("_"),
                                                OneOrMore(Fragment(HexCharacter))
                                            ]))
                                        ]),
                                        not_followed_by = Fragment(IdentifierStart)
                                    )
                                )
                            ]
                        ),
                        Token(
                            name = DecimalLiteral,
                            definitions = [
                                // An integer (without a dot or a fraction) is enabled in all versions:
                                TokenDefinition(
                                    scanner = TrailingContext(
                                        scanner = Sequence([
                                            TrailingContext(
                                                scanner = Fragment(DecimalDigits),
                                                not_followed_by = Atom(".")
                                            ),
                                            Optional(Fragment(DecimalExponent))
                                        ]),
                                        not_followed_by = Fragment(IdentifierStart)
                                    )
                                ),
                                // An integer and a dot (without a fraction) is disabled in "0.5.0"
                                TokenDefinition(
                                    enabled = Till("0.5.0"),
                                    scanner = TrailingContext(
                                        scanner = Sequence([
                                            TrailingContext(
                                                scanner =
                                                    Sequence([Fragment(DecimalDigits), Atom(".")]),
                                                not_followed_by = Fragment(DecimalDigits)
                                            ),
                                            Optional(Fragment(DecimalExponent))
                                        ]),
                                        not_followed_by = Fragment(IdentifierStart)
                                    )
                                ),
                                // A dot and a fraction (without an integer) is enabled in all versions:
                                TokenDefinition(
                                    scanner = TrailingContext(
                                        scanner = Sequence([
                                            Atom("."),
                                            Fragment(DecimalDigits),
                                            Optional(Fragment(DecimalExponent))
                                        ]),
                                        not_followed_by = Fragment(IdentifierStart)
                                    )
                                ),
                                // An integer, a dot, and a fraction is enabled in all versions:
                                TokenDefinition(
                                    scanner = TrailingContext(
                                        scanner = Sequence([
                                            Fragment(DecimalDigits),
                                            Atom("."),
                                            Fragment(DecimalDigits),
                                            Optional(Fragment(DecimalExponent))
                                        ]),
                                        not_followed_by = Fragment(IdentifierStart)
                                    )
                                )
                            ]
                        ),
                        Fragment(
                            name = DecimalDigits,
                            scanner = Sequence([
                                OneOrMore(Range(inclusive_start = '0', inclusive_end = '9')),
                                ZeroOrMore(Sequence([
                                    Atom("_"),
                                    OneOrMore(Range(inclusive_start = '0', inclusive_end = '9'))
                                ]))
                            ])
                        ),
                        Fragment(
                            name = DecimalExponent,
                            scanner = Sequence([
                                Choice([Atom("e"), Atom("E")]),
                                Optional(Atom("-")),
                                Fragment(DecimalDigits)
                            ])
                        ),
                        Enum(
                            name = NumberUnit,
                            variants = [
                                // 1e-18 ETH
                                EnumVariant(name = Wei, reference = WeiKeyword),
                                // 1e-9 ETH
                                EnumVariant(
                                    name = Gwei,
                                    enabled = From("0.6.11"),
                                    reference = GweiKeyword
                                ),
                                // 1e-6 ETH
                                EnumVariant(
                                    name = Szabo,
                                    enabled = Till("0.7.0"),
                                    reference = SzaboKeyword
                                ),
                                // 1e-3 ETH
                                EnumVariant(
                                    name = Finney,
                                    enabled = Till("0.7.0"),
                                    reference = FinneyKeyword
                                ),
                                // 1 ETH
                                EnumVariant(name = Ether, reference = EtherKeyword),
                                EnumVariant(name = Seconds, reference = SecondsKeyword),
                                EnumVariant(name = Minutes, reference = MinutesKeyword),
                                EnumVariant(name = Hours, reference = HoursKeyword),
                                EnumVariant(name = Days, reference = DaysKeyword),
                                EnumVariant(name = Weeks, reference = WeeksKeyword),
                                EnumVariant(
                                    name = Years,
                                    enabled = Till("0.5.0"),
                                    reference = YearsKeyword
                                )
                            ]
                        )
                    ]
                ),
                Topic(
                    title = "Strings",
                    items = [
                        Enum(
                            name = StringExpression,
                            variants = [
                                EnumVariant(name = Hex, reference = HexStringLiterals),
                                EnumVariant(name = Ascii, reference = AsciiStringLiterals),
                                EnumVariant(
                                    name = Unicode,
                                    enabled = From("0.7.0"),
                                    reference = UnicodeStringLiterals
                                )
                            ]
                        ),
                        Repeated(name = HexStringLiterals, repeated = HexStringLiteral),
                        Enum(
                            name = HexStringLiteral,
                            variants = [
                                EnumVariant(
                                    name = SingleQuoted,
                                    reference = SingleQuotedHexStringLiteral
                                ),
                                EnumVariant(
                                    name = DoubleQuoted,
                                    reference = DoubleQuotedHexStringLiteral
                                )
                            ]
                        ),
                        Token(
                            name = SingleQuotedHexStringLiteral,
                            definitions = [TokenDefinition(
                                scanner = Sequence([
                                    Atom("hex'"),
                                    Optional(Fragment(HexStringContents)),
                                    Atom("'")
                                ])
                            )]
                        ),
                        Token(
                            name = DoubleQuotedHexStringLiteral,
                            definitions = [TokenDefinition(
                                scanner = Sequence([
                                    Atom("hex\""),
                                    Optional(Fragment(HexStringContents)),
                                    Atom("\"")
                                ])
                            )]
                        ),
                        Fragment(
                            name = HexStringContents,
                            scanner = Sequence([
                                Fragment(HexCharacter),
                                Fragment(HexCharacter),
                                ZeroOrMore(Sequence([
                                    Optional(Atom("_")),
                                    Fragment(HexCharacter),
                                    Fragment(HexCharacter)
                                ]))
                            ])
                        ),
                        Fragment(
                            name = HexCharacter,
                            scanner = Choice([
                                Range(inclusive_start = '0', inclusive_end = '9'),
                                Range(inclusive_start = 'a', inclusive_end = 'f'),
                                Range(inclusive_start = 'A', inclusive_end = 'F')
                            ])
                        ),
                        Repeated(name = AsciiStringLiterals, repeated = AsciiStringLiteral),
                        Enum(
                            name = AsciiStringLiteral,
                            variants = [
                                EnumVariant(
                                    name = SingleQuoted,
                                    reference = SingleQuotedAsciiStringLiteral
                                ),
                                EnumVariant(
                                    name = DoubleQuoted,
                                    reference = DoubleQuotedAsciiStringLiteral
                                )
                            ]
                        ),
                        Token(
                            name = SingleQuotedAsciiStringLiteral,
                            definitions = [TokenDefinition(
                                scanner = Sequence([
                                    Atom("'"),
                                    ZeroOrMore(Choice([
                                        Fragment(EscapeSequence),
                                        Range(inclusive_start = ' ', inclusive_end = '&'),
                                        Range(inclusive_start = '(', inclusive_end = '['),
                                        Range(inclusive_start = ']', inclusive_end = '~')
                                    ])),
                                    Atom("'")
                                ])
                            )]
                        ),
                        Token(
                            name = DoubleQuotedAsciiStringLiteral,
                            definitions = [TokenDefinition(
                                scanner = Sequence([
                                    Atom("\""),
                                    ZeroOrMore(Choice([
                                        Fragment(EscapeSequence),
                                        Range(inclusive_start = ' ', inclusive_end = '!'),
                                        Range(inclusive_start = '#', inclusive_end = '['),
                                        Range(inclusive_start = ']', inclusive_end = '~')
                                    ])),
                                    Atom("\"")
                                ])
                            )]
                        ),
                        Repeated(
                            name = UnicodeStringLiterals,
                            repeated = UnicodeStringLiteral,
                            enabled = From("0.7.0")
                        ),
                        Enum(
                            name = UnicodeStringLiteral,
                            enabled = From("0.7.0"),
                            variants = [
                                EnumVariant(
                                    name = SingleQuoted,
                                    reference = SingleQuotedUnicodeStringLiteral
                                ),
                                EnumVariant(
                                    name = DoubleQuoted,
                                    reference = DoubleQuotedUnicodeStringLiteral
                                )
                            ]
                        ),
                        Token(
                            name = SingleQuotedUnicodeStringLiteral,
                            definitions = [TokenDefinition(
                                enabled = From("0.7.0"),
                                scanner = Sequence([
                                    Atom("unicode'"),
                                    ZeroOrMore(Choice([
                                        Fragment(EscapeSequence),
                                        Not(['\'', '\\', '\r', '\n'])
                                    ])),
                                    Atom("'")
                                ])
                            )]
                        ),
                        Token(
                            name = DoubleQuotedUnicodeStringLiteral,
                            definitions = [TokenDefinition(
                                enabled = From("0.7.0"),
                                scanner = Sequence([
                                    Atom("unicode\""),
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
                            scanner = Sequence([
                                Atom("x"),
                                Fragment(HexCharacter),
                                Fragment(HexCharacter)
                            ])
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
                        )
                    ]
                ),
                Topic(
                    title = "Identifiers",
                    items = [
                        Separated(
                            name = IdentifierPath,
                            separated = Identifier,
                            separator = Period
                        ),
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
                        )
                    ]
                )
            ]
        ),
        Section(
            title = "Yul",
            topics = [
                Topic(
                    title = "Yul Statements",
                    lexical_context = Yul,
                    items = [
                        Struct(
                            name = AssemblyStatement,
                            fields = (
                                assembly_keyword = Required(Terminal([AssemblyKeyword])),
                                label = Optional(kind = NonTerminal(AsciiStringLiteral)),
                                flags = Optional(kind = NonTerminal(AssemblyFlagsDeclaration)),
                                body = Required(NonTerminal(YulBlock))
                            )
                        ),
                        Struct(
                            name = AssemblyFlagsDeclaration,
                            error_recovery = FieldsErrorRecovery(
                                delimiters =
                                    FieldDelimiters(open = open_paren, close = close_paren)
                            ),
                            fields = (
                                open_paren = Required(Terminal([OpenParen])),
                                flags = Required(NonTerminal(AssemblyFlags)),
                                close_paren = Required(Terminal([CloseParen]))
                            )
                        ),
                        Separated(
                            name = AssemblyFlags,
                            separated = AsciiStringLiteral,
                            separator = Comma,
                            allow_empty = true
                        ),
                        Struct(
                            name = YulBlock,
                            error_recovery = FieldsErrorRecovery(
                                delimiters =
                                    FieldDelimiters(open = open_brace, close = close_brace)
                            ),
                            fields = (
                                open_brace = Required(Terminal([OpenBrace])),
                                statements = Required(NonTerminal(YulStatements)),
                                close_brace = Required(Terminal([CloseBrace]))
                            )
                        ),
                        Repeated(
                            name = YulStatements,
                            repeated = YulStatement,
                            allow_empty = true
                        ),
                        Enum(
                            name = YulStatement,
                            variants = [
                                EnumVariant(name = Block, reference = YulBlock),
                                EnumVariant(name = Function, reference = YulFunctionDefinition),
                                EnumVariant(
                                    name = VariableDeclaration,
                                    reference = YulVariableDeclarationStatement
                                ),
                                EnumVariant(name = Assignment, reference = YulAssignmentStatement),
                                EnumVariant(name = If, reference = YulIfStatement),
                                EnumVariant(name = For, reference = YulForStatement),
                                EnumVariant(name = Switch, reference = YulSwitchStatement),
                                EnumVariant(
                                    name = Leave,
                                    enabled = From("0.6.0"),
                                    reference = YulLeaveStatement
                                ),
                                EnumVariant(name = Break, reference = YulBreakStatement),
                                EnumVariant(name = Continue, reference = YulContinueStatement),
                                EnumVariant(name = Expression, reference = YulExpression)
                            ]
                        ),
                        Struct(
                            name = YulFunctionDefinition,
                            fields = (
                                function_keyword = Required(Terminal([YulFunctionKeyword])),
                                name = Required(Terminal([YulIdentifier])),
                                parameters = Required(NonTerminal(YulParametersDeclaration)),
                                returns = Optional(kind = NonTerminal(YulReturnsDeclaration)),
                                body = Required(NonTerminal(YulBlock))
                            )
                        ),
                        Struct(
                            name = YulParametersDeclaration,
                            error_recovery = FieldsErrorRecovery(
                                delimiters =
                                    FieldDelimiters(open = open_paren, close = close_paren)
                            ),
                            fields = (
                                open_paren = Required(Terminal([OpenParen])),
                                parameters = Required(NonTerminal(YulParameters)),
                                close_paren = Required(Terminal([CloseParen]))
                            )
                        ),
                        Separated(
                            name = YulParameters,
                            separated = YulIdentifier,
                            separator = Comma,
                            allow_empty = true
                        ),
                        Struct(
                            name = YulReturnsDeclaration,
                            fields = (
                                minus_greater_than = Required(Terminal([MinusGreaterThan])),
                                variables = Required(NonTerminal(YulReturnVariables))
                            )
                        ),
                        Separated(
                            name = YulReturnVariables,
                            separated = YulIdentifier,
                            separator = Comma
                        ),
                        Struct(
                            name = YulVariableDeclarationStatement,
                            fields = (
                                let_keyword = Required(Terminal([YulLetKeyword])),
                                names = Required(NonTerminal(YulIdentifierPaths)),
                                value = Optional(kind = NonTerminal(YulVariableDeclarationValue))
                            )
                        ),
                        Struct(
                            name = YulVariableDeclarationValue,
                            fields = (
                                colon_equal = Required(Terminal([ColonEqual])),
                                expression = Required(NonTerminal(YulExpression))
                            )
                        ),
                        Struct(
                            name = YulAssignmentStatement,
                            fields = (
                                names = Required(NonTerminal(YulIdentifierPaths)),
                                colon_equal = Required(Terminal([ColonEqual])),
                                expression = Required(NonTerminal(YulExpression))
                            )
                        ),
                        Struct(
                            name = YulIfStatement,
                            fields = (
                                if_keyword = Required(Terminal([YulIfKeyword])),
                                condition = Required(NonTerminal(YulExpression)),
                                body = Required(NonTerminal(YulBlock))
                            )
                        ),
                        Struct(
                            name = YulLeaveStatement,
                            enabled = From("0.6.0"),
                            fields = (leave_keyword = Required(Terminal([YulLeaveKeyword])))
                        ),
                        Struct(
                            name = YulBreakStatement,
                            fields = (break_keyword = Required(Terminal([YulBreakKeyword])))
                        ),
                        Struct(
                            name = YulContinueStatement,
                            fields = (continue_keyword = Required(Terminal([YulContinueKeyword])))
                        ),
                        Struct(
                            name = YulForStatement,
                            fields = (
                                for_keyword = Required(Terminal([YulForKeyword])),
                                initialization = Required(NonTerminal(YulBlock)),
                                condition = Required(NonTerminal(YulExpression)),
                                iterator = Required(NonTerminal(YulBlock)),
                                body = Required(NonTerminal(YulBlock))
                            )
                        ),
                        Struct(
                            name = YulSwitchStatement,
                            fields = (
                                switch_keyword = Required(Terminal([YulSwitchKeyword])),
                                expression = Required(NonTerminal(YulExpression)),
                                cases = Required(NonTerminal(YulSwitchCases))
                            )
                        ),
                        Repeated(name = YulSwitchCases, repeated = YulSwitchCase),
                        Enum(
                            name = YulSwitchCase,
                            variants = [
                                EnumVariant(name = Default, reference = YulDefaultCase),
                                EnumVariant(name = Value, reference = YulValueCase)
                            ]
                        ),
                        Struct(
                            name = YulDefaultCase,
                            fields = (
                                default_keyword = Required(Terminal([YulDefaultKeyword])),
                                body = Required(NonTerminal(YulBlock))
                            )
                        ),
                        Struct(
                            name = YulValueCase,
                            fields = (
                                case_keyword = Required(Terminal([YulCaseKeyword])),
                                value = Required(NonTerminal(YulLiteral)),
                                body = Required(NonTerminal(YulBlock))
                            )
                        )
                    ]
                ),
                Topic(
                    title = "Yul Expressions",
                    lexical_context = Yul,
                    items = [
                        Precedence(
                            name = YulExpression,
                            precedence_expressions = [PrecedenceExpression(
                                name = YulFunctionCallExpression,
                                operators = [PrecedenceOperator(
                                    model = Postfix,
                                    error_recovery = FieldsErrorRecovery(
                                        delimiters =
                                            FieldDelimiters(open = open_paren, close = close_paren)
                                    ),
                                    fields = (
                                        open_paren = Required(Terminal([OpenParen])),
                                        arguments = Required(NonTerminal(YulArguments)),
                                        close_paren = Required(Terminal([CloseParen]))
                                    )
                                )]
                            )],
                            primary_expressions = [
                                PrimaryExpression(expression = YulLiteral),
                                PrimaryExpression(expression = YulIdentifierPath)
                            ]
                        ),
                        Separated(
                            name = YulArguments,
                            separated = YulExpression,
                            separator = Comma,
                            allow_empty = true
                        ),
                        Separated(
                            name = YulIdentifierPaths,
                            separated = YulIdentifierPath,
                            separator = Comma
                        ),
                        Separated(
                            name = YulIdentifierPath,
                            separated = YulIdentifier,
                            separator = Period
                        ),
                        Token(
                            name = YulIdentifier,
                            definitions = [TokenDefinition(scanner = Fragment(RawIdentifier))]
                        ),
                        Enum(
                            name = YulLiteral,
                            variants = [
                                EnumVariant(name = True, reference = YulTrueKeyword),
                                EnumVariant(name = False, reference = YulFalseKeyword),
                                EnumVariant(name = Decimal, reference = YulDecimalLiteral),
                                EnumVariant(name = Hex, reference = YulHexLiteral),
                                EnumVariant(name = HexString, reference = HexStringLiteral),
                                EnumVariant(name = AsciiString, reference = AsciiStringLiteral)
                            ]
                        ),
                        Token(
                            name = YulDecimalLiteral,
                            definitions = [TokenDefinition(
                                scanner = TrailingContext(
                                    scanner = Choice([
                                        Atom("0"),
                                        Sequence([
                                            Range(inclusive_start = '1', inclusive_end = '9'),
                                            ZeroOrMore(Range(
                                                inclusive_start = '0',
                                                inclusive_end = '9'
                                            ))
                                        ])
                                    ]),
                                    not_followed_by = Fragment(IdentifierStart)
                                )
                            )]
                        ),
                        Token(
                            name = YulHexLiteral,
                            definitions = [TokenDefinition(
                                scanner = TrailingContext(
                                    scanner =
                                        Sequence([Atom("0x"), OneOrMore(Fragment(HexCharacter))]),
                                    not_followed_by = Fragment(IdentifierStart)
                                )
                            )]
                        )
                    ]
                ),
                Topic(
                    title = "Keywords",
                    lexical_context = Yul,
                    items = [
                        Keyword(
                            name = YulAbstractKeyword,
                            identifier = YulIdentifier,
                            definitions = [KeywordDefinition(
                                enabled = Never,
                                reserved = Till("0.7.1"),
                                value = Atom("abstract")
                            )]
                        ),
                        Keyword(
                            name = YulAddressKeyword,
                            identifier = YulIdentifier,
                            definitions =
                                [KeywordDefinition(enabled = Never, value = Atom("address"))]
                        ),
                        Keyword(
                            name = YulAfterKeyword,
                            identifier = YulIdentifier,
                            definitions = [KeywordDefinition(
                                enabled = Never,
                                reserved = Till("0.7.1"),
                                value = Atom("after")
                            )]
                        ),
                        Keyword(
                            name = YulAliasKeyword,
                            identifier = YulIdentifier,
                            definitions = [KeywordDefinition(
                                enabled = Never,
                                reserved = Range(from = "0.5.0", till = "0.7.1"),
                                value = Atom("alias")
                            )]
                        ),
                        Keyword(
                            name = YulAnonymousKeyword,
                            identifier = YulIdentifier,
                            definitions = [KeywordDefinition(
                                enabled = Never,
                                reserved = Till("0.7.1"),
                                value = Atom("anonymous")
                            )]
                        ),
                        Keyword(
                            name = YulApplyKeyword,
                            identifier = YulIdentifier,
                            definitions = [KeywordDefinition(
                                enabled = Never,
                                reserved = Range(from = "0.5.0", till = "0.7.1"),
                                value = Atom("apply")
                            )]
                        ),
                        Keyword(
                            name = YulAsKeyword,
                            identifier = YulIdentifier,
                            definitions = [KeywordDefinition(
                                enabled = Never,
                                reserved = Till("0.7.1"),
                                value = Atom("as")
                            )]
                        ),
                        Keyword(
                            name = YulAssemblyKeyword,
                            identifier = YulIdentifier,
                            definitions = [KeywordDefinition(
                                enabled = Never,
                                reserved = Till("0.7.1"),
                                value = Atom("assembly")
                            )]
                        ),
                        Keyword(
                            name = YulAutoKeyword,
                            identifier = YulIdentifier,
                            definitions = [KeywordDefinition(
                                enabled = Never,
                                reserved = Range(from = "0.5.0", till = "0.7.1"),
                                value = Atom("auto")
                            )]
                        ),
                        Keyword(
                            name = YulBoolKeyword,
                            identifier = YulIdentifier,
                            definitions = [KeywordDefinition(
                                enabled = Never,
                                reserved = Till("0.5.10"),
                                value = Atom("bool")
                            )]
                        ),
                        Keyword(
                            name = YulBreakKeyword,
                            identifier = YulIdentifier,
                            definitions = [KeywordDefinition(value = Atom("break"))]
                        ),
                        Keyword(
                            name = YulByteKeyword,
                            identifier = YulIdentifier,
                            definitions =
                                [KeywordDefinition(enabled = Never, value = Atom("byte"))]
                        ),
                        Keyword(
                            name = YulBytesKeyword,
                            identifier = YulIdentifier,
                            definitions = [KeywordDefinition(
                                enabled = Never,
                                reserved = Till("0.7.1"),
                                value = Sequence([
                                    Atom("bytes"),
                                    Choice([
                                        Atom("1"),
                                        Atom("2"),
                                        Atom("3"),
                                        Atom("4"),
                                        Atom("5"),
                                        Atom("6"),
                                        Atom("7"),
                                        Atom("8"),
                                        Atom("9"),
                                        Atom("10"),
                                        Atom("11"),
                                        Atom("12"),
                                        Atom("13"),
                                        Atom("14"),
                                        Atom("15"),
                                        Atom("16"),
                                        Atom("17"),
                                        Atom("18"),
                                        Atom("19"),
                                        Atom("20"),
                                        Atom("21"),
                                        Atom("22"),
                                        Atom("23"),
                                        Atom("24"),
                                        Atom("25"),
                                        Atom("26"),
                                        Atom("27"),
                                        Atom("28"),
                                        Atom("29"),
                                        Atom("30"),
                                        Atom("31"),
                                        Atom("32")
                                    ])
                                ])
                            )]
                        ),
                        Keyword(
                            name = YulCallDataKeyword,
                            identifier = YulIdentifier,
                            definitions = [KeywordDefinition(
                                enabled = Never,
                                reserved = Range(from = "0.5.0", till = "0.7.1"),
                                value = Atom("calldata")
                            )]
                        ),
                        Keyword(
                            name = YulCaseKeyword,
                            identifier = YulIdentifier,
                            definitions = [KeywordDefinition(value = Atom("case"))]
                        ),
                        Keyword(
                            name = YulCatchKeyword,
                            identifier = YulIdentifier,
                            definitions = [KeywordDefinition(
                                enabled = Never,
                                reserved = Till("0.7.1"),
                                value = Atom("catch")
                            )]
                        ),
                        Keyword(
                            name = YulConstantKeyword,
                            identifier = YulIdentifier,
                            definitions = [KeywordDefinition(
                                enabled = Never,
                                reserved = Till("0.7.1"),
                                value = Atom("constant")
                            )]
                        ),
                        Keyword(
                            name = YulConstructorKeyword,
                            identifier = YulIdentifier,
                            definitions = [KeywordDefinition(
                                enabled = Never,
                                reserved = Range(from = "0.5.0", till = "0.7.1"),
                                value = Atom("constructor")
                            )]
                        ),
                        Keyword(
                            name = YulContinueKeyword,
                            identifier = YulIdentifier,
                            definitions = [KeywordDefinition(value = Atom("continue"))]
                        ),
                        Keyword(
                            name = YulContractKeyword,
                            identifier = YulIdentifier,
                            definitions = [KeywordDefinition(
                                enabled = Never,
                                reserved = Till("0.7.1"),
                                value = Atom("contract")
                            )]
                        ),
                        Keyword(
                            name = YulCopyOfKeyword,
                            identifier = YulIdentifier,
                            definitions = [KeywordDefinition(
                                enabled = Never,
                                reserved = Range(from = "0.5.0", till = "0.7.1"),
                                value = Atom("copyof")
                            )]
                        ),
                        Keyword(
                            name = YulDaysKeyword,
                            identifier = YulIdentifier,
                            definitions = [KeywordDefinition(
                                enabled = Never,
                                reserved = Till("0.7.1"),
                                value = Atom("days")
                            )]
                        ),
                        Keyword(
                            name = YulDefaultKeyword,
                            identifier = YulIdentifier,
                            definitions = [KeywordDefinition(value = Atom("default"))]
                        ),
                        Keyword(
                            name = YulDefineKeyword,
                            identifier = YulIdentifier,
                            definitions = [KeywordDefinition(
                                enabled = Never,
                                reserved = Range(from = "0.5.0", till = "0.7.1"),
                                value = Atom("define")
                            )]
                        ),
                        Keyword(
                            name = YulDeleteKeyword,
                            identifier = YulIdentifier,
                            definitions = [KeywordDefinition(
                                enabled = Never,
                                reserved = Till("0.7.1"),
                                value = Atom("delete")
                            )]
                        ),
                        Keyword(
                            name = YulDoKeyword,
                            identifier = YulIdentifier,
                            definitions = [KeywordDefinition(
                                enabled = Never,
                                reserved = Till("0.7.1"),
                                value = Atom("do")
                            )]
                        ),
                        Keyword(
                            name = YulElseKeyword,
                            identifier = YulIdentifier,
                            definitions = [KeywordDefinition(
                                enabled = Never,
                                reserved = Till("0.7.1"),
                                value = Atom("else")
                            )]
                        ),
                        Keyword(
                            name = YulEmitKeyword,
                            identifier = YulIdentifier,
                            definitions = [KeywordDefinition(
                                enabled = Never,
                                reserved = Range(from = "0.5.0", till = "0.7.1"),
                                value = Atom("emit")
                            )]
                        ),
                        Keyword(
                            name = YulEnumKeyword,
                            identifier = YulIdentifier,
                            definitions = [KeywordDefinition(
                                enabled = Never,
                                reserved = Till("0.7.1"),
                                value = Atom("enum")
                            )]
                        ),
                        Keyword(
                            name = YulEtherKeyword,
                            identifier = YulIdentifier,
                            definitions = [KeywordDefinition(
                                enabled = Never,
                                reserved = Till("0.7.1"),
                                value = Atom("ether")
                            )]
                        ),
                        Keyword(
                            name = YulEventKeyword,
                            identifier = YulIdentifier,
                            definitions = [KeywordDefinition(
                                enabled = Never,
                                reserved = Till("0.7.1"),
                                value = Atom("event")
                            )]
                        ),
                        Keyword(
                            name = YulExternalKeyword,
                            identifier = YulIdentifier,
                            definitions = [KeywordDefinition(
                                enabled = Never,
                                reserved = Till("0.7.1"),
                                value = Atom("external")
                            )]
                        ),
                        Keyword(
                            name = YulFallbackKeyword,
                            identifier = YulIdentifier,
                            definitions = [KeywordDefinition(
                                enabled = Never,
                                reserved = Range(from = "0.6.0", till = "0.7.1"),
                                value = Atom("fallback")
                            )]
                        ),
                        Keyword(
                            name = YulFalseKeyword,
                            identifier = YulIdentifier,
                            definitions = [KeywordDefinition(value = Atom("false"))]
                        ),
                        Keyword(
                            name = YulFinalKeyword,
                            identifier = YulIdentifier,
                            definitions = [KeywordDefinition(
                                enabled = Never,
                                reserved = Till("0.7.1"),
                                value = Atom("final")
                            )]
                        ),
                        Keyword(
                            name = YulFinneyKeyword,
                            identifier = YulIdentifier,
                            definitions = [KeywordDefinition(
                                enabled = Never,
                                reserved = Till("0.7.0"),
                                value = Atom("finney")
                            )]
                        ),
                        Keyword(
                            name = YulFixedKeyword,
                            identifier = YulIdentifier,
                            definitions = [
                                KeywordDefinition(
                                    enabled = Never,
                                    reserved = Till("0.7.1"),
                                    value = Atom("fixed")
                                ),
                                KeywordDefinition(
                                    enabled = Never,
                                    reserved = Till("0.7.1"),
                                    value = Sequence([
                                        Atom("fixed"),
                                        Choice([
                                            Atom("8"),
                                            Atom("16"),
                                            Atom("24"),
                                            Atom("32"),
                                            Atom("40"),
                                            Atom("48"),
                                            Atom("56"),
                                            Atom("64"),
                                            Atom("72"),
                                            Atom("80"),
                                            Atom("88"),
                                            Atom("96"),
                                            Atom("104"),
                                            Atom("112"),
                                            Atom("120"),
                                            Atom("128"),
                                            Atom("136"),
                                            Atom("144"),
                                            Atom("152"),
                                            Atom("160"),
                                            Atom("168"),
                                            Atom("176")
                                        ]),
                                        Atom("x"),
                                        Choice([
                                            Atom("8"),
                                            Atom("16"),
                                            Atom("24"),
                                            Atom("32"),
                                            Atom("40"),
                                            Atom("48"),
                                            Atom("56"),
                                            Atom("64"),
                                            Atom("72"),
                                            Atom("80")
                                        ])
                                    ])
                                ),
                                KeywordDefinition(
                                    enabled = Never,
                                    reserved = Till("0.7.1"),
                                    value = Sequence([
                                        Atom("fixed"),
                                        Choice([
                                            Atom("184x8"),
                                            Atom("184x16"),
                                            Atom("184x24"),
                                            Atom("184x32"),
                                            Atom("184x40"),
                                            Atom("184x48"),
                                            Atom("184x56"),
                                            Atom("184x64"),
                                            Atom("184x72"),
                                            Atom("192x8"),
                                            Atom("192x16"),
                                            Atom("192x24"),
                                            Atom("192x32"),
                                            Atom("192x40"),
                                            Atom("192x48"),
                                            Atom("192x56"),
                                            Atom("192x64"),
                                            Atom("200x8"),
                                            Atom("200x16"),
                                            Atom("200x24"),
                                            Atom("200x32"),
                                            Atom("200x40"),
                                            Atom("200x48"),
                                            Atom("200x56"),
                                            Atom("208x8"),
                                            Atom("208x16"),
                                            Atom("208x24"),
                                            Atom("208x32"),
                                            Atom("208x40"),
                                            Atom("208x48"),
                                            Atom("216x8"),
                                            Atom("216x16"),
                                            Atom("216x24"),
                                            Atom("216x32"),
                                            Atom("216x40"),
                                            Atom("224x8"),
                                            Atom("224x16"),
                                            Atom("224x24"),
                                            Atom("224x32"),
                                            Atom("232x8"),
                                            Atom("232x16"),
                                            Atom("232x24"),
                                            Atom("240x8"),
                                            Atom("240x16"),
                                            Atom("248x8")
                                        ])
                                    ])
                                ),
                                KeywordDefinition(
                                    enabled = Never,
                                    reserved = Range(from = "0.4.14", till = "0.7.1"),
                                    value = Sequence([
                                        Atom("fixed"),
                                        Choice([
                                            Atom("184x80"),
                                            Atom("192x72"),
                                            Atom("192x80"),
                                            Atom("200x64"),
                                            Atom("200x72"),
                                            Atom("200x80"),
                                            Atom("208x56"),
                                            Atom("208x64"),
                                            Atom("208x72"),
                                            Atom("208x80"),
                                            Atom("216x48"),
                                            Atom("216x56"),
                                            Atom("216x64"),
                                            Atom("216x72"),
                                            Atom("216x80"),
                                            Atom("224x40"),
                                            Atom("224x48"),
                                            Atom("224x56"),
                                            Atom("224x64"),
                                            Atom("224x72"),
                                            Atom("224x80"),
                                            Atom("232x32"),
                                            Atom("232x40"),
                                            Atom("232x48"),
                                            Atom("232x56"),
                                            Atom("232x64"),
                                            Atom("232x72"),
                                            Atom("232x80"),
                                            Atom("240x24"),
                                            Atom("240x32"),
                                            Atom("240x40"),
                                            Atom("240x48"),
                                            Atom("240x56"),
                                            Atom("240x64"),
                                            Atom("240x72"),
                                            Atom("240x80"),
                                            Atom("248x16"),
                                            Atom("248x24"),
                                            Atom("248x32"),
                                            Atom("248x40"),
                                            Atom("248x48"),
                                            Atom("248x56"),
                                            Atom("248x64"),
                                            Atom("248x72"),
                                            Atom("248x80"),
                                            Atom("256x8"),
                                            Atom("256x16"),
                                            Atom("256x24"),
                                            Atom("256x32"),
                                            Atom("256x40"),
                                            Atom("256x48"),
                                            Atom("256x56"),
                                            Atom("256x64"),
                                            Atom("256x72"),
                                            Atom("256x80")
                                        ])
                                    ])
                                ),
                                KeywordDefinition(
                                    enabled = Never,
                                    reserved = Range(from = "0.4.14", till = "0.7.1"),
                                    value = Sequence([
                                        Atom("fixed"),
                                        Choice([
                                            Atom("8"),
                                            Atom("16"),
                                            Atom("24"),
                                            Atom("32"),
                                            Atom("40"),
                                            Atom("48"),
                                            Atom("56"),
                                            Atom("64"),
                                            Atom("72"),
                                            Atom("80"),
                                            Atom("88"),
                                            Atom("96"),
                                            Atom("104"),
                                            Atom("112"),
                                            Atom("120"),
                                            Atom("128"),
                                            Atom("136"),
                                            Atom("144"),
                                            Atom("152"),
                                            Atom("160"),
                                            Atom("168"),
                                            Atom("176"),
                                            Atom("184"),
                                            Atom("192"),
                                            Atom("200"),
                                            Atom("208"),
                                            Atom("216"),
                                            Atom("224"),
                                            Atom("232"),
                                            Atom("240"),
                                            Atom("248"),
                                            Atom("256")
                                        ]),
                                        Atom("x"),
                                        Choice([
                                            Atom("0"),
                                            Atom("1"),
                                            Atom("2"),
                                            Atom("3"),
                                            Atom("4"),
                                            Atom("5"),
                                            Atom("6"),
                                            Atom("7"),
                                            Atom("9"),
                                            Atom("10"),
                                            Atom("11"),
                                            Atom("12"),
                                            Atom("13"),
                                            Atom("14"),
                                            Atom("15"),
                                            Atom("17"),
                                            Atom("18"),
                                            Atom("19"),
                                            Atom("20"),
                                            Atom("21"),
                                            Atom("22"),
                                            Atom("23"),
                                            Atom("25"),
                                            Atom("26"),
                                            Atom("27"),
                                            Atom("28"),
                                            Atom("29"),
                                            Atom("30"),
                                            Atom("31"),
                                            Atom("33"),
                                            Atom("34"),
                                            Atom("35"),
                                            Atom("36"),
                                            Atom("37"),
                                            Atom("38"),
                                            Atom("39"),
                                            Atom("41"),
                                            Atom("42"),
                                            Atom("43"),
                                            Atom("44"),
                                            Atom("45"),
                                            Atom("46"),
                                            Atom("47"),
                                            Atom("49"),
                                            Atom("50"),
                                            Atom("51"),
                                            Atom("52"),
                                            Atom("53"),
                                            Atom("54"),
                                            Atom("55"),
                                            Atom("57"),
                                            Atom("58"),
                                            Atom("59"),
                                            Atom("60"),
                                            Atom("61"),
                                            Atom("62"),
                                            Atom("63"),
                                            Atom("65"),
                                            Atom("66"),
                                            Atom("67"),
                                            Atom("68"),
                                            Atom("69"),
                                            Atom("70"),
                                            Atom("71"),
                                            Atom("73"),
                                            Atom("74"),
                                            Atom("75"),
                                            Atom("76"),
                                            Atom("77"),
                                            Atom("78"),
                                            Atom("79")
                                        ])
                                    ])
                                )
                            ]
                        ),
                        Keyword(
                            name = YulForKeyword,
                            identifier = YulIdentifier,
                            definitions = [KeywordDefinition(value = Atom("for"))]
                        ),
                        Keyword(
                            name = YulFunctionKeyword,
                            identifier = YulIdentifier,
                            definitions = [KeywordDefinition(value = Atom("function"))]
                        ),
                        Keyword(
                            name = YulGweiKeyword,
                            identifier = YulIdentifier,
                            definitions = [KeywordDefinition(
                                enabled = Never,
                                reserved = Range(from = "0.7.0", till = "0.7.1"),
                                value = Atom("gwei")
                            )]
                        ),
                        Keyword(
                            name = YulHexKeyword,
                            identifier = YulIdentifier,
                            definitions = [KeywordDefinition(enabled = Never, value = Atom("hex"))]
                        ),
                        Keyword(
                            name = YulHoursKeyword,
                            identifier = YulIdentifier,
                            definitions = [KeywordDefinition(
                                enabled = Never,
                                reserved = Till("0.7.1"),
                                value = Atom("hours")
                            )]
                        ),
                        Keyword(
                            name = YulIfKeyword,
                            identifier = YulIdentifier,
                            definitions = [KeywordDefinition(value = Atom("if"))]
                        ),
                        Keyword(
                            name = YulImmutableKeyword,
                            identifier = YulIdentifier,
                            definitions = [KeywordDefinition(
                                enabled = Never,
                                reserved = Range(from = "0.5.0", till = "0.7.1"),
                                value = Atom("immutable")
                            )]
                        ),
                        Keyword(
                            name = YulImplementsKeyword,
                            identifier = YulIdentifier,
                            definitions = [KeywordDefinition(
                                enabled = Never,
                                reserved = Range(from = "0.5.0", till = "0.7.1"),
                                value = Atom("implements")
                            )]
                        ),
                        Keyword(
                            name = YulImportKeyword,
                            identifier = YulIdentifier,
                            definitions = [KeywordDefinition(
                                enabled = Never,
                                reserved = Till("0.7.1"),
                                value = Atom("import")
                            )]
                        ),
                        Keyword(
                            name = YulIndexedKeyword,
                            identifier = YulIdentifier,
                            definitions = [KeywordDefinition(
                                enabled = Never,
                                reserved = Till("0.7.1"),
                                value = Atom("indexed")
                            )]
                        ),
                        Keyword(
                            name = YulInKeyword,
                            identifier = YulIdentifier,
                            definitions = [KeywordDefinition(
                                enabled = Never,
                                reserved = Till("0.6.8"),
                                value = Atom("in")
                            )]
                        ),
                        Keyword(
                            name = YulInlineKeyword,
                            identifier = YulIdentifier,
                            definitions = [KeywordDefinition(
                                enabled = Never,
                                reserved = Till("0.7.1"),
                                value = Atom("inline")
                            )]
                        ),
                        Keyword(
                            name = YulInterfaceKeyword,
                            identifier = YulIdentifier,
                            definitions = [KeywordDefinition(
                                enabled = Never,
                                reserved = Till("0.7.1"),
                                value = Atom("interface")
                            )]
                        ),
                        Keyword(
                            name = YulInternalKeyword,
                            identifier = YulIdentifier,
                            definitions = [KeywordDefinition(
                                enabled = Never,
                                reserved = Till("0.7.1"),
                                value = Atom("internal")
                            )]
                        ),
                        Keyword(
                            name = YulIntKeyword,
                            identifier = YulIdentifier,
                            definitions = [KeywordDefinition(
                                enabled = Never,
                                reserved = Till("0.7.1"),
                                value = Sequence([
                                    Atom("int"),
                                    Optional(Choice([
                                        Atom("8"),
                                        Atom("16"),
                                        Atom("24"),
                                        Atom("32"),
                                        Atom("40"),
                                        Atom("48"),
                                        Atom("56"),
                                        Atom("64"),
                                        Atom("72"),
                                        Atom("80"),
                                        Atom("88"),
                                        Atom("96"),
                                        Atom("104"),
                                        Atom("112"),
                                        Atom("120"),
                                        Atom("128"),
                                        Atom("136"),
                                        Atom("144"),
                                        Atom("152"),
                                        Atom("160"),
                                        Atom("168"),
                                        Atom("176"),
                                        Atom("184"),
                                        Atom("192"),
                                        Atom("200"),
                                        Atom("208"),
                                        Atom("216"),
                                        Atom("224"),
                                        Atom("232"),
                                        Atom("240"),
                                        Atom("248"),
                                        Atom("256")
                                    ]))
                                ])
                            )]
                        ),
                        Keyword(
                            name = YulIsKeyword,
                            identifier = YulIdentifier,
                            definitions = [KeywordDefinition(
                                enabled = Never,
                                reserved = Till("0.7.1"),
                                value = Atom("is")
                            )]
                        ),
                        Keyword(
                            name = YulLeaveKeyword,
                            identifier = YulIdentifier,
                            definitions = [KeywordDefinition(
                                enabled = From("0.6.0"),
                                reserved = From("0.7.1"),
                                value = Atom("leave")
                            )]
                        ),
                        Keyword(
                            name = YulLetKeyword,
                            identifier = YulIdentifier,
                            definitions = [KeywordDefinition(value = Atom("let"))]
                        ),
                        Keyword(
                            name = YulLibraryKeyword,
                            identifier = YulIdentifier,
                            definitions = [KeywordDefinition(
                                enabled = Never,
                                reserved = Till("0.7.1"),
                                value = Atom("library")
                            )]
                        ),
                        Keyword(
                            name = YulMacroKeyword,
                            identifier = YulIdentifier,
                            definitions = [KeywordDefinition(
                                enabled = Never,
                                reserved = Range(from = "0.5.0", till = "0.7.1"),
                                value = Atom("macro")
                            )]
                        ),
                        Keyword(
                            name = YulMappingKeyword,
                            identifier = YulIdentifier,
                            definitions = [KeywordDefinition(
                                enabled = Never,
                                reserved = Till("0.7.1"),
                                value = Atom("mapping")
                            )]
                        ),
                        Keyword(
                            name = YulMatchKeyword,
                            identifier = YulIdentifier,
                            definitions = [KeywordDefinition(
                                enabled = Never,
                                reserved = Till("0.7.1"),
                                value = Atom("match")
                            )]
                        ),
                        Keyword(
                            name = YulMemoryKeyword,
                            identifier = YulIdentifier,
                            definitions = [KeywordDefinition(
                                enabled = Never,
                                reserved = Till("0.7.1"),
                                value = Atom("memory")
                            )]
                        ),
                        Keyword(
                            name = YulMinutesKeyword,
                            identifier = YulIdentifier,
                            definitions = [KeywordDefinition(
                                enabled = Never,
                                reserved = Till("0.7.1"),
                                value = Atom("minutes")
                            )]
                        ),
                        Keyword(
                            name = YulModifierKeyword,
                            identifier = YulIdentifier,
                            definitions = [KeywordDefinition(
                                enabled = Never,
                                reserved = Till("0.7.1"),
                                value = Atom("modifier")
                            )]
                        ),
                        Keyword(
                            name = YulMutableKeyword,
                            identifier = YulIdentifier,
                            definitions = [KeywordDefinition(
                                enabled = Never,
                                reserved = Range(from = "0.5.0", till = "0.7.1"),
                                value = Atom("mutable")
                            )]
                        ),
                        Keyword(
                            name = YulNewKeyword,
                            identifier = YulIdentifier,
                            definitions = [KeywordDefinition(
                                enabled = Never,
                                reserved = Till("0.7.1"),
                                value = Atom("new")
                            )]
                        ),
                        Keyword(
                            name = YulNullKeyword,
                            identifier = YulIdentifier,
                            definitions = [KeywordDefinition(
                                enabled = Never,
                                reserved = Till("0.7.1"),
                                value = Atom("null")
                            )]
                        ),
                        Keyword(
                            name = YulOfKeyword,
                            identifier = YulIdentifier,
                            definitions = [KeywordDefinition(
                                enabled = Never,
                                reserved = Till("0.7.1"),
                                value = Atom("of")
                            )]
                        ),
                        Keyword(
                            name = YulOverrideKeyword,
                            identifier = YulIdentifier,
                            definitions = [KeywordDefinition(
                                enabled = Never,
                                reserved = Range(from = "0.5.0", till = "0.7.1"),
                                value = Atom("override")
                            )]
                        ),
                        Keyword(
                            name = YulPartialKeyword,
                            identifier = YulIdentifier,
                            definitions = [KeywordDefinition(
                                enabled = Never,
                                reserved = Range(from = "0.5.0", till = "0.7.1"),
                                value = Atom("partial")
                            )]
                        ),
                        Keyword(
                            name = YulPayableKeyword,
                            identifier = YulIdentifier,
                            definitions = [KeywordDefinition(
                                enabled = Never,
                                reserved = Till("0.7.1"),
                                value = Atom("payable")
                            )]
                        ),
                        Keyword(
                            name = YulPragmaKeyword,
                            identifier = YulIdentifier,
                            definitions = [KeywordDefinition(
                                enabled = Never,
                                reserved = Till("0.7.1"),
                                value = Atom("pragma")
                            )]
                        ),
                        Keyword(
                            name = YulPrivateKeyword,
                            identifier = YulIdentifier,
                            definitions = [KeywordDefinition(
                                enabled = Never,
                                reserved = Till("0.7.1"),
                                value = Atom("private")
                            )]
                        ),
                        Keyword(
                            name = YulPromiseKeyword,
                            identifier = YulIdentifier,
                            definitions = [KeywordDefinition(
                                enabled = Never,
                                reserved = Range(from = "0.5.0", till = "0.7.1"),
                                value = Atom("promise")
                            )]
                        ),
                        Keyword(
                            name = YulPublicKeyword,
                            identifier = YulIdentifier,
                            definitions = [KeywordDefinition(
                                enabled = Never,
                                reserved = Till("0.7.1"),
                                value = Atom("public")
                            )]
                        ),
                        Keyword(
                            name = YulPureKeyword,
                            identifier = YulIdentifier,
                            definitions = [KeywordDefinition(
                                enabled = Never,
                                reserved = Till("0.7.1"),
                                value = Atom("pure")
                            )]
                        ),
                        Keyword(
                            name = YulReceiveKeyword,
                            identifier = YulIdentifier,
                            definitions = [KeywordDefinition(
                                enabled = Never,
                                reserved = Range(from = "0.6.0", till = "0.7.1"),
                                value = Atom("receive")
                            )]
                        ),
                        Keyword(
                            name = YulReferenceKeyword,
                            identifier = YulIdentifier,
                            definitions = [KeywordDefinition(
                                enabled = Never,
                                reserved = Range(from = "0.5.0", till = "0.7.1"),
                                value = Atom("reference")
                            )]
                        ),
                        Keyword(
                            name = YulRelocatableKeyword,
                            identifier = YulIdentifier,
                            definitions = [KeywordDefinition(
                                enabled = Never,
                                reserved = Till("0.7.1"),
                                value = Atom("relocatable")
                            )]
                        ),
                        Keyword(
                            name = YulReturnKeyword,
                            identifier = YulIdentifier,
                            definitions =
                                [KeywordDefinition(enabled = Never, value = Atom("return"))]
                        ),
                        Keyword(
                            name = YulReturnsKeyword,
                            identifier = YulIdentifier,
                            definitions = [KeywordDefinition(
                                enabled = Never,
                                reserved = Till("0.7.1"),
                                value = Atom("returns")
                            )]
                        ),
                        Keyword(
                            name = YulRevertKeyword,
                            identifier = YulIdentifier,
                            definitions =
                                [KeywordDefinition(enabled = Never, value = Atom("revert"))]
                        ),
                        Keyword(
                            name = YulSealedKeyword,
                            identifier = YulIdentifier,
                            definitions = [KeywordDefinition(
                                enabled = Never,
                                reserved = Range(from = "0.5.0", till = "0.7.1"),
                                value = Atom("sealed")
                            )]
                        ),
                        Keyword(
                            name = YulSecondsKeyword,
                            identifier = YulIdentifier,
                            definitions = [KeywordDefinition(
                                enabled = Never,
                                reserved = Till("0.7.1"),
                                value = Atom("seconds")
                            )]
                        ),
                        Keyword(
                            name = YulSizeOfKeyword,
                            identifier = YulIdentifier,
                            definitions = [KeywordDefinition(
                                enabled = Never,
                                reserved = Range(from = "0.5.0", till = "0.7.1"),
                                value = Atom("sizeof")
                            )]
                        ),
                        Keyword(
                            name = YulStaticKeyword,
                            identifier = YulIdentifier,
                            definitions = [KeywordDefinition(
                                enabled = Never,
                                reserved = Till("0.7.1"),
                                value = Atom("static")
                            )]
                        ),
                        Keyword(
                            name = YulStorageKeyword,
                            identifier = YulIdentifier,
                            definitions = [KeywordDefinition(
                                enabled = Never,
                                reserved = Till("0.7.1"),
                                value = Atom("storage")
                            )]
                        ),
                        Keyword(
                            name = YulStringKeyword,
                            identifier = YulIdentifier,
                            definitions = [KeywordDefinition(
                                enabled = Never,
                                reserved = Till("0.7.1"),
                                value = Atom("string")
                            )]
                        ),
                        Keyword(
                            name = YulStructKeyword,
                            identifier = YulIdentifier,
                            definitions = [KeywordDefinition(
                                enabled = Never,
                                reserved = Till("0.7.1"),
                                value = Atom("struct")
                            )]
                        ),
                        Keyword(
                            name = YulSupportsKeyword,
                            identifier = YulIdentifier,
                            definitions = [KeywordDefinition(
                                enabled = Never,
                                reserved = Range(from = "0.5.0", till = "0.7.1"),
                                value = Atom("supports")
                            )]
                        ),
                        Keyword(
                            name = YulSwitchKeyword,
                            identifier = YulIdentifier,
                            definitions = [KeywordDefinition(value = Atom("switch"))]
                        ),
                        Keyword(
                            name = YulSzaboKeyword,
                            identifier = YulIdentifier,
                            definitions = [KeywordDefinition(
                                enabled = Never,
                                reserved = Till("0.7.0"),
                                value = Atom("szabo")
                            )]
                        ),
                        Keyword(
                            name = YulThrowKeyword,
                            identifier = YulIdentifier,
                            definitions = [KeywordDefinition(
                                enabled = Never,
                                reserved = Till("0.7.1"),
                                value = Atom("throw")
                            )]
                        ),
                        Keyword(
                            name = YulTrueKeyword,
                            identifier = YulIdentifier,
                            definitions = [KeywordDefinition(value = Atom("true"))]
                        ),
                        Keyword(
                            name = YulTryKeyword,
                            identifier = YulIdentifier,
                            definitions = [KeywordDefinition(
                                enabled = Never,
                                reserved = Till("0.7.1"),
                                value = Atom("try")
                            )]
                        ),
                        Keyword(
                            name = YulTypeDefKeyword,
                            identifier = YulIdentifier,
                            definitions = [KeywordDefinition(
                                enabled = Never,
                                reserved = Range(from = "0.5.0", till = "0.7.1"),
                                value = Atom("typedef")
                            )]
                        ),
                        Keyword(
                            name = YulTypeKeyword,
                            identifier = YulIdentifier,
                            definitions = [KeywordDefinition(
                                enabled = Never,
                                reserved = Till("0.7.1"),
                                value = Atom("type")
                            )]
                        ),
                        Keyword(
                            name = YulTypeOfKeyword,
                            identifier = YulIdentifier,
                            definitions = [KeywordDefinition(
                                enabled = Never,
                                reserved = Till("0.7.1"),
                                value = Atom("typeof")
                            )]
                        ),
                        Keyword(
                            name = YulUfixedKeyword,
                            identifier = YulIdentifier,
                            definitions = [
                                KeywordDefinition(
                                    enabled = Never,
                                    reserved = Till("0.7.1"),
                                    value = Atom("ufixed")
                                ),
                                KeywordDefinition(
                                    enabled = Never,
                                    reserved = Till("0.7.1"),
                                    value = Sequence([
                                        Atom("ufixed"),
                                        Choice([
                                            Atom("8"),
                                            Atom("16"),
                                            Atom("24"),
                                            Atom("32"),
                                            Atom("40"),
                                            Atom("48"),
                                            Atom("56"),
                                            Atom("64"),
                                            Atom("72"),
                                            Atom("80"),
                                            Atom("88"),
                                            Atom("96"),
                                            Atom("104"),
                                            Atom("112"),
                                            Atom("120"),
                                            Atom("128"),
                                            Atom("136"),
                                            Atom("144"),
                                            Atom("152"),
                                            Atom("160"),
                                            Atom("168"),
                                            Atom("176")
                                        ]),
                                        Atom("x"),
                                        Choice([
                                            Atom("8"),
                                            Atom("16"),
                                            Atom("24"),
                                            Atom("32"),
                                            Atom("40"),
                                            Atom("48"),
                                            Atom("56"),
                                            Atom("64"),
                                            Atom("72"),
                                            Atom("80")
                                        ])
                                    ])
                                ),
                                KeywordDefinition(
                                    enabled = Never,
                                    reserved = Till("0.7.1"),
                                    value = Sequence([
                                        Atom("ufixed"),
                                        Choice([
                                            Atom("184x8"),
                                            Atom("184x16"),
                                            Atom("184x24"),
                                            Atom("184x32"),
                                            Atom("184x40"),
                                            Atom("184x48"),
                                            Atom("184x56"),
                                            Atom("184x64"),
                                            Atom("184x72"),
                                            Atom("192x8"),
                                            Atom("192x16"),
                                            Atom("192x24"),
                                            Atom("192x32"),
                                            Atom("192x40"),
                                            Atom("192x48"),
                                            Atom("192x56"),
                                            Atom("192x64"),
                                            Atom("200x8"),
                                            Atom("200x16"),
                                            Atom("200x24"),
                                            Atom("200x32"),
                                            Atom("200x40"),
                                            Atom("200x48"),
                                            Atom("200x56"),
                                            Atom("208x8"),
                                            Atom("208x16"),
                                            Atom("208x24"),
                                            Atom("208x32"),
                                            Atom("208x40"),
                                            Atom("208x48"),
                                            Atom("216x8"),
                                            Atom("216x16"),
                                            Atom("216x24"),
                                            Atom("216x32"),
                                            Atom("216x40"),
                                            Atom("224x8"),
                                            Atom("224x16"),
                                            Atom("224x24"),
                                            Atom("224x32"),
                                            Atom("232x8"),
                                            Atom("232x16"),
                                            Atom("232x24"),
                                            Atom("240x8"),
                                            Atom("240x16"),
                                            Atom("248x8")
                                        ])
                                    ])
                                ),
                                KeywordDefinition(
                                    enabled = Never,
                                    reserved = Range(from = "0.4.14", till = "0.7.1"),
                                    value = Sequence([
                                        Atom("ufixed"),
                                        Choice([
                                            Atom("184x80"),
                                            Atom("192x72"),
                                            Atom("192x80"),
                                            Atom("200x64"),
                                            Atom("200x72"),
                                            Atom("200x80"),
                                            Atom("208x56"),
                                            Atom("208x64"),
                                            Atom("208x72"),
                                            Atom("208x80"),
                                            Atom("216x48"),
                                            Atom("216x56"),
                                            Atom("216x64"),
                                            Atom("216x72"),
                                            Atom("216x80"),
                                            Atom("224x40"),
                                            Atom("224x48"),
                                            Atom("224x56"),
                                            Atom("224x64"),
                                            Atom("224x72"),
                                            Atom("224x80"),
                                            Atom("232x32"),
                                            Atom("232x40"),
                                            Atom("232x48"),
                                            Atom("232x56"),
                                            Atom("232x64"),
                                            Atom("232x72"),
                                            Atom("232x80"),
                                            Atom("240x24"),
                                            Atom("240x32"),
                                            Atom("240x40"),
                                            Atom("240x48"),
                                            Atom("240x56"),
                                            Atom("240x64"),
                                            Atom("240x72"),
                                            Atom("240x80"),
                                            Atom("248x16"),
                                            Atom("248x24"),
                                            Atom("248x32"),
                                            Atom("248x40"),
                                            Atom("248x48"),
                                            Atom("248x56"),
                                            Atom("248x64"),
                                            Atom("248x72"),
                                            Atom("248x80"),
                                            Atom("256x8"),
                                            Atom("256x16"),
                                            Atom("256x24"),
                                            Atom("256x32"),
                                            Atom("256x40"),
                                            Atom("256x48"),
                                            Atom("256x56"),
                                            Atom("256x64"),
                                            Atom("256x72"),
                                            Atom("256x80")
                                        ])
                                    ])
                                ),
                                KeywordDefinition(
                                    enabled = Never,
                                    reserved = Range(from = "0.4.14", till = "0.7.1"),
                                    value = Sequence([
                                        Atom("ufixed"),
                                        Choice([
                                            Atom("8"),
                                            Atom("16"),
                                            Atom("24"),
                                            Atom("32"),
                                            Atom("40"),
                                            Atom("48"),
                                            Atom("56"),
                                            Atom("64"),
                                            Atom("72"),
                                            Atom("80"),
                                            Atom("88"),
                                            Atom("96"),
                                            Atom("104"),
                                            Atom("112"),
                                            Atom("120"),
                                            Atom("128"),
                                            Atom("136"),
                                            Atom("144"),
                                            Atom("152"),
                                            Atom("160"),
                                            Atom("168"),
                                            Atom("176"),
                                            Atom("184"),
                                            Atom("192"),
                                            Atom("200"),
                                            Atom("208"),
                                            Atom("216"),
                                            Atom("224"),
                                            Atom("232"),
                                            Atom("240"),
                                            Atom("248"),
                                            Atom("256")
                                        ]),
                                        Atom("x"),
                                        Choice([
                                            Atom("0"),
                                            Atom("1"),
                                            Atom("2"),
                                            Atom("3"),
                                            Atom("4"),
                                            Atom("5"),
                                            Atom("6"),
                                            Atom("7"),
                                            Atom("9"),
                                            Atom("10"),
                                            Atom("11"),
                                            Atom("12"),
                                            Atom("13"),
                                            Atom("14"),
                                            Atom("15"),
                                            Atom("17"),
                                            Atom("18"),
                                            Atom("19"),
                                            Atom("20"),
                                            Atom("21"),
                                            Atom("22"),
                                            Atom("23"),
                                            Atom("25"),
                                            Atom("26"),
                                            Atom("27"),
                                            Atom("28"),
                                            Atom("29"),
                                            Atom("30"),
                                            Atom("31"),
                                            Atom("33"),
                                            Atom("34"),
                                            Atom("35"),
                                            Atom("36"),
                                            Atom("37"),
                                            Atom("38"),
                                            Atom("39"),
                                            Atom("41"),
                                            Atom("42"),
                                            Atom("43"),
                                            Atom("44"),
                                            Atom("45"),
                                            Atom("46"),
                                            Atom("47"),
                                            Atom("49"),
                                            Atom("50"),
                                            Atom("51"),
                                            Atom("52"),
                                            Atom("53"),
                                            Atom("54"),
                                            Atom("55"),
                                            Atom("57"),
                                            Atom("58"),
                                            Atom("59"),
                                            Atom("60"),
                                            Atom("61"),
                                            Atom("62"),
                                            Atom("63"),
                                            Atom("65"),
                                            Atom("66"),
                                            Atom("67"),
                                            Atom("68"),
                                            Atom("69"),
                                            Atom("70"),
                                            Atom("71"),
                                            Atom("73"),
                                            Atom("74"),
                                            Atom("75"),
                                            Atom("76"),
                                            Atom("77"),
                                            Atom("78"),
                                            Atom("79")
                                        ])
                                    ])
                                )
                            ]
                        ),
                        Keyword(
                            name = YulUintKeyword,
                            identifier = YulIdentifier,
                            definitions = [KeywordDefinition(
                                enabled = Never,
                                reserved = Till("0.7.1"),
                                value = Sequence([
                                    Atom("uint"),
                                    Optional(Choice([
                                        Atom("8"),
                                        Atom("16"),
                                        Atom("24"),
                                        Atom("32"),
                                        Atom("40"),
                                        Atom("48"),
                                        Atom("56"),
                                        Atom("64"),
                                        Atom("72"),
                                        Atom("80"),
                                        Atom("88"),
                                        Atom("96"),
                                        Atom("104"),
                                        Atom("112"),
                                        Atom("120"),
                                        Atom("128"),
                                        Atom("136"),
                                        Atom("144"),
                                        Atom("152"),
                                        Atom("160"),
                                        Atom("168"),
                                        Atom("176"),
                                        Atom("184"),
                                        Atom("192"),
                                        Atom("200"),
                                        Atom("208"),
                                        Atom("216"),
                                        Atom("224"),
                                        Atom("232"),
                                        Atom("240"),
                                        Atom("248"),
                                        Atom("256")
                                    ]))
                                ])
                            )]
                        ),
                        Keyword(
                            name = YulUncheckedKeyword,
                            identifier = YulIdentifier,
                            definitions = [KeywordDefinition(
                                enabled = Never,
                                reserved = Range(from = "0.5.0", till = "0.7.1"),
                                value = Atom("unchecked")
                            )]
                        ),
                        Keyword(
                            name = YulUsingKeyword,
                            identifier = YulIdentifier,
                            definitions = [KeywordDefinition(
                                enabled = Never,
                                reserved = Till("0.7.1"),
                                value = Atom("using")
                            )]
                        ),
                        Keyword(
                            name = YulVarKeyword,
                            identifier = YulIdentifier,
                            definitions = [KeywordDefinition(
                                enabled = Never,
                                reserved = Till("0.6.5"),
                                value = Atom("var")
                            )]
                        ),
                        Keyword(
                            name = YulViewKeyword,
                            identifier = YulIdentifier,
                            definitions = [KeywordDefinition(
                                enabled = Never,
                                reserved = Till("0.7.1"),
                                value = Atom("view")
                            )]
                        ),
                        Keyword(
                            name = YulVirtualKeyword,
                            identifier = YulIdentifier,
                            definitions = [KeywordDefinition(
                                enabled = Never,
                                reserved = Range(from = "0.6.0", till = "0.7.1"),
                                value = Atom("virtual")
                            )]
                        ),
                        Keyword(
                            name = YulWeeksKeyword,
                            identifier = YulIdentifier,
                            definitions = [KeywordDefinition(
                                enabled = Never,
                                reserved = Till("0.7.1"),
                                value = Atom("weeks")
                            )]
                        ),
                        Keyword(
                            name = YulWeiKeyword,
                            identifier = YulIdentifier,
                            definitions = [KeywordDefinition(
                                enabled = Never,
                                reserved = Till("0.7.1"),
                                value = Atom("wei")
                            )]
                        ),
                        Keyword(
                            name = YulWhileKeyword,
                            identifier = YulIdentifier,
                            definitions = [KeywordDefinition(
                                enabled = Never,
                                reserved = Till("0.7.1"),
                                value = Atom("while")
                            )]
                        ),
                        Keyword(
                            name = YulYearsKeyword,
                            identifier = YulIdentifier,
                            definitions = [KeywordDefinition(
                                enabled = Never,
                                reserved = Till("0.7.1"),
                                value = Atom("years")
                            )]
                        )
                    ]
                )
            ]
        )
    ]
));
