pub use solidity::SolidityDefinition;

codegen_language_macros::compile!(Language(
    name = Solidity,
    documentation_dir = "crates/solidity/inputs/language/docs",
    root_item = SourceUnit,
    // TODO(#638): For now this is on par with the DSL v1 definition to minimize the fallout.
    // We should replace this with the new definition from #629.
    leading_trivia = OneOrMore(Choice([
        Trivia(Whitespace),
        Trivia(EndOfLine),
        Trivia(SingleLineComment),
        Trivia(MultiLineComment),
        Trivia(SingleLineNatSpecComment),
        Trivia(MultiLineNatSpecComment)
    ])),
    trailing_trivia = Sequence([
        Optional(Trivia(Whitespace)),
        Optional(Trivia(SingleLineComment)),
        Trivia(EndOfLine)
    ]),
    versions = [
        "0.4.11", "0.4.12", "0.4.13", "0.4.14", "0.4.15", "0.4.16", "0.4.17", "0.4.18", "0.4.19",
        "0.4.20", "0.4.21", "0.4.22", "0.4.23", "0.4.24", "0.4.25", "0.4.26", "0.5.0", "0.5.1",
        "0.5.2", "0.5.3", "0.5.4", "0.5.5", "0.5.6", "0.5.7", "0.5.8", "0.5.9", "0.5.10", "0.5.11",
        "0.5.12", "0.5.13", "0.5.14", "0.5.15", "0.5.16", "0.5.17", "0.6.0", "0.6.1", "0.6.2",
        "0.6.3", "0.6.4", "0.6.5", "0.6.6", "0.6.7", "0.6.8", "0.6.9", "0.6.10", "0.6.11",
        "0.6.12", "0.7.0", "0.7.1", "0.7.2", "0.7.3", "0.7.4", "0.7.5", "0.7.6", "0.8.0", "0.8.1",
        "0.8.2", "0.8.3", "0.8.4", "0.8.5", "0.8.6", "0.8.7", "0.8.8", "0.8.9", "0.8.10", "0.8.11",
        "0.8.12", "0.8.13", "0.8.14", "0.8.15", "0.8.16", "0.8.17", "0.8.18", "0.8.19", "0.8.20",
        "0.8.21", "0.8.22"
    ],
    sections = [
        Section(
            title = "File Structure",
            topics = [
                Topic(title = "License Specifiers", items = []),
                Topic(
                    title = "Source Unit",
                    items = [
                        Struct(
                            name = SourceUnit,
                            fields = (members = Optional(reference = SourceUnitMembers))
                        ),
                        Repeated(name = SourceUnitMembers, reference = SourceUnitMember),
                        Enum(
                            name = SourceUnitMember,
                            variants = [
                                EnumVariant(reference = PragmaDirective),
                                EnumVariant(reference = ImportDirective),
                                EnumVariant(reference = ContractDefinition),
                                EnumVariant(reference = InterfaceDefinition),
                                EnumVariant(reference = LibraryDefinition),
                                EnumVariant(reference = StructDefinition, enabled = From("0.6.0")),
                                EnumVariant(reference = EnumDefinition, enabled = From("0.6.0")),
                                EnumVariant(
                                    reference = FunctionDefinition,
                                    enabled = From("0.7.1")
                                ),
                                EnumVariant(
                                    reference = ConstantDefinition,
                                    enabled = From("0.7.4")
                                ),
                                EnumVariant(reference = ErrorDefinition, enabled = From("0.8.4")),
                                EnumVariant(
                                    reference = UserDefinedValueTypeDefinition,
                                    enabled = From("0.8.8")
                                ),
                                EnumVariant(reference = UsingDirective, enabled = From("0.8.13")),
                                EnumVariant(reference = EventDefinition, enabled = From("0.8.22"))
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
                                pragma_keyword = Required(PragmaKeyword),
                                pragma = Required(Pragma),
                                semicolon = Required(Semicolon)
                            )
                        ),
                        Enum(
                            name = Pragma,
                            variants = [
                                EnumVariant(reference = ABICoderPragma),
                                EnumVariant(reference = ExperimentalPragma),
                                EnumVariant(reference = VersionPragma)
                            ]
                        ),
                        Struct(
                            name = ABICoderPragma,
                            fields = (
                                abicoder_keyword = Required(AbicoderKeyword),
                                version = Required(Identifier)
                            )
                        ),
                        Struct(
                            name = ExperimentalPragma,
                            fields = (
                                experimental_keyword = Required(ExperimentalKeyword),
                                feature = Required(ExperimentalFeature)
                            )
                        ),
                        Enum(
                            name = ExperimentalFeature,
                            variants = [
                                EnumVariant(reference = Identifier),
                                EnumVariant(reference = AsciiStringLiteral)
                            ]
                        ),
                        Struct(
                            name = VersionPragma,
                            fields = (
                                solidity_keyword = Required(SolidityKeyword),
                                expressions = Required(VersionPragmaExpressions)
                            )
                        ),
                        Repeated(
                            name = VersionPragmaExpressions,
                            reference = VersionPragmaExpression
                        ),
                        Precedence(
                            name = VersionPragmaExpression,
                            precedence_expressions = [
                                PrecedenceExpression(
                                    name = VersionPragmaOrExpression,
                                    operators = [PrecedenceOperator(
                                        model = BinaryLeftAssociative,
                                        fields = (operator = Required(BarBar))
                                    )]
                                ),
                                PrecedenceExpression(
                                    name = VersionPragmaRangeExpression,
                                    operators = [PrecedenceOperator(
                                        model = BinaryLeftAssociative,
                                        fields = (operator = Required(Minus))
                                    )]
                                ),
                                PrecedenceExpression(
                                    name = VersionPragmaPrefixExpression,
                                    operators = [
                                        PrecedenceOperator(
                                            model = Prefix,
                                            fields = (operator = Required(Caret))
                                        ),
                                        PrecedenceOperator(
                                            model = Prefix,
                                            fields = (operator = Required(Tilde))
                                        ),
                                        PrecedenceOperator(
                                            model = Prefix,
                                            fields = (operator = Required(Equal))
                                        ),
                                        PrecedenceOperator(
                                            model = Prefix,
                                            fields = (operator = Required(LessThan))
                                        ),
                                        PrecedenceOperator(
                                            model = Prefix,
                                            fields = (operator = Required(GreaterThan))
                                        ),
                                        PrecedenceOperator(
                                            model = Prefix,
                                            fields = (operator = Required(LessThanEqual))
                                        ),
                                        PrecedenceOperator(
                                            model = Prefix,
                                            fields = (operator = Required(GreaterThanEqual))
                                        )
                                    ]
                                )
                            ],
                            primary_expressions =
                                [PrimaryExpression(reference = VersionPragmaSpecifier)]
                        ),
                        Separated(
                            name = VersionPragmaSpecifier,
                            reference = VersionPragmaValue,
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
                        ),
                        Keyword(
                            name = SolidityKeyword,
                            identifier = Identifier,
                            definitions = [KeywordDefinition(value = Atom("solidity"))]
                        ),
                        Keyword(
                            name = ExperimentalKeyword,
                            identifier = Identifier,
                            definitions = [KeywordDefinition(value = Atom("experimental"))]
                        ),
                        Keyword(
                            name = AbicoderKeyword,
                            identifier = Identifier,
                            definitions = [KeywordDefinition(value = Atom("abicoder"))]
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
                                import_keyword = Required(ImportKeyword),
                                clause = Required(ImportClause),
                                semicolon = Required(Semicolon)
                            )
                        ),
                        Enum(
                            name = ImportClause,
                            variants = [
                                EnumVariant(reference = PathImport),
                                EnumVariant(reference = NamedImport),
                                EnumVariant(reference = ImportDeconstruction)
                            ]
                        ),
                        Struct(
                            name = PathImport,
                            fields = (
                                path = Required(AsciiStringLiteral),
                                alias = Optional(reference = ImportAlias)
                            )
                        ),
                        Struct(
                            name = NamedImport,
                            fields = (
                                asterisk = Required(Asterisk),
                                alias = Required(ImportAlias),
                                from_keyword = Required(FromKeyword),
                                path = Required(AsciiStringLiteral)
                            )
                        ),
                        Struct(
                            name = ImportDeconstruction,
                            error_recovery = FieldsErrorRecovery(
                                delimiters =
                                    FieldDelimiters(open = open_brace, close = close_brace)
                            ),
                            fields = (
                                open_brace = Required(OpenBrace),
                                symbols = Required(ImportDeconstructionSymbols),
                                close_brace = Required(CloseBrace),
                                from_keyword = Required(FromKeyword),
                                path = Required(AsciiStringLiteral)
                            )
                        ),
                        Separated(
                            name = ImportDeconstructionSymbols,
                            reference = ImportDeconstructionSymbol,
                            separator = Comma
                        ),
                        Struct(
                            name = ImportDeconstructionSymbol,
                            fields = (
                                name = Required(Identifier),
                                alias = Optional(reference = ImportAlias)
                            )
                        ),
                        Struct(
                            name = ImportAlias,
                            fields = (
                                as_keyword = Required(AsKeyword),
                                identifier = Required(Identifier)
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
                                using_keyword = Required(UsingKeyword),
                                clause = Required(UsingClause),
                                for_keyword = Required(ForKeyword),
                                target = Required(UsingTarget),
                                global_keyword =
                                    Optional(reference = GlobalKeyword, enabled = From("0.8.13")),
                                semicolon = Required(Semicolon)
                            )
                        ),
                        Enum(
                            name = UsingClause,
                            variants = [
                                EnumVariant(reference = IdentifierPath),
                                EnumVariant(
                                    reference = UsingDeconstruction,
                                    enabled = From("0.8.13")
                                )
                            ]
                        ),
                        Struct(
                            name = UsingDeconstruction,
                            enabled = From("0.8.13"),
                            error_recovery = FieldsErrorRecovery(
                                delimiters =
                                    FieldDelimiters(open = open_brace, close = close_brace)
                            ),
                            fields = (
                                open_brace = Required(OpenBrace),
                                symbols = Required(UsingDeconstructionSymbols),
                                close_brace = Required(CloseBrace)
                            )
                        ),
                        Separated(
                            name = UsingDeconstructionSymbols,
                            reference = UsingDeconstructionSymbol,
                            separator = Comma,
                            enabled = From("0.8.13")
                        ),
                        Struct(
                            name = UsingDeconstructionSymbol,
                            enabled = From("0.8.13"),
                            fields = (
                                name = Required(IdentifierPath),
                                alias = Optional(reference = UsingAlias, enabled = From("0.8.19"))
                            )
                        ),
                        Struct(
                            name = UsingAlias,
                            enabled = From("0.8.19"),
                            fields = (
                                as_keyword = Required(AsKeyword),
                                operator = Required(UsingOperator)
                            )
                        ),
                        Enum(
                            name = UsingOperator,
                            enabled = From("0.8.19"),
                            variants = [
                                EnumVariant(reference = Ampersand),
                                EnumVariant(reference = Asterisk),
                                EnumVariant(reference = BangEqual),
                                EnumVariant(reference = Bar),
                                EnumVariant(reference = Caret),
                                EnumVariant(reference = EqualEqual),
                                EnumVariant(reference = GreaterThan),
                                EnumVariant(reference = GreaterThanEqual),
                                EnumVariant(reference = LessThan),
                                EnumVariant(reference = LessThanEqual),
                                EnumVariant(reference = Minus),
                                EnumVariant(reference = Percent),
                                EnumVariant(reference = Plus),
                                EnumVariant(reference = Slash),
                                EnumVariant(reference = Tilde)
                            ]
                        ),
                        Enum(
                            name = UsingTarget,
                            variants = [
                                EnumVariant(reference = TypeName),
                                EnumVariant(reference = Asterisk)
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
                                    TrailingContext(
                                        scanner = Atom("*"),
                                        not_followed_by = Atom("/")
                                    )
                                ])),
                                Atom("*/")
                            ])
                        ),
                        Trivia(
                            name = SingleLineNatSpecComment,
                            scanner = Sequence([Atom("///"), ZeroOrMore(Not(['\r', '\n']))])
                        ),
                        Trivia(
                            name = MultiLineNatSpecComment,
                            scanner = Sequence([
                                Atom("/**"),
                                ZeroOrMore(Choice([
                                    Not(['*']),
                                    TrailingContext(
                                        scanner = Atom("*"),
                                        not_followed_by = Atom("/")
                                    )
                                ])),
                                Atom("*/")
                            ])
                        )
                    ]
                ),
                Topic(title = "Nat Spec Format", items = []),
                Topic(
                    title = "Keywords",
                    items = [
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
                                    Optional(Choice([
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
                                    ]))
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
                            definitions = [TokenDefinition(scanner = Atom(":"))]
                        ),
                        Token(
                            name = ColonEqual,
                            definitions = [TokenDefinition(scanner = Atom(":="))]
                        ),
                        Token(
                            name = Equal,
                            definitions = [TokenDefinition(scanner = Atom("="))]
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
                            definitions = [TokenDefinition(scanner = Atom("*"))]
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
                            definitions = [TokenDefinition(scanner = Atom("|"))]
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
                            definitions = [TokenDefinition(scanner = Atom("&"))]
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
                            definitions = [TokenDefinition(scanner = Atom("<"))]
                        ),
                        Token(
                            name = LessThanEqual,
                            definitions = [TokenDefinition(scanner = Atom("<="))]
                        ),
                        Token(
                            name = LessThanLessThan,
                            definitions = [TokenDefinition(scanner = Atom("<<"))]
                        ),
                        Token(
                            name = LessThanLessThanEqual,
                            definitions = [TokenDefinition(scanner = Atom("<<="))]
                        ),
                        Token(
                            name = GreaterThan,
                            definitions = [TokenDefinition(scanner = Atom(">"))]
                        ),
                        Token(
                            name = GreaterThanEqual,
                            definitions = [TokenDefinition(scanner = Atom(">="))]
                        ),
                        Token(
                            name = GreaterThanGreaterThan,
                            definitions = [TokenDefinition(scanner = Atom(">>"))]
                        ),
                        Token(
                            name = GreaterThanGreaterThanEqual,
                            definitions = [TokenDefinition(scanner = Atom(">>="))]
                        ),
                        Token(
                            name = GreaterThanGreaterThanGreaterThan,
                            definitions = [TokenDefinition(scanner = Atom(">>>"))]
                        ),
                        Token(
                            name = GreaterThanGreaterThanGreaterThanEqual,
                            definitions = [TokenDefinition(scanner = Atom(">>>="))]
                        ),
                        Token(
                            name = Plus,
                            definitions = [TokenDefinition(scanner = Atom("+"))]
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
                            definitions = [TokenDefinition(scanner = Atom("-"))]
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
                                    not_followed_by = Choice([Atom("*"), Atom("/"), Atom("=")])
                                )
                            )]
                        ),
                        Token(
                            name = SlashEqual,
                            definitions = [TokenDefinition(scanner = Atom("/="))]
                        ),
                        Token(
                            name = Percent,
                            definitions = [TokenDefinition(scanner = Atom("%"))]
                        ),
                        Token(
                            name = PercentEqual,
                            definitions = [TokenDefinition(scanner = Atom("%="))]
                        ),
                        Token(
                            name = Bang,
                            definitions = [TokenDefinition(scanner = Atom("!"))]
                        ),
                        Token(
                            name = BangEqual,
                            definitions = [TokenDefinition(scanner = Atom("!="))]
                        ),
                        Token(
                            name = Caret,
                            definitions = [TokenDefinition(scanner = Atom("^"))]
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
                                abstract_keyword =
                                    Optional(reference = AbstractKeyword, enabled = From("0.6.0")),
                                contract_keyword = Required(ContractKeyword),
                                name = Required(Identifier),
                                inheritence = Optional(reference = InheritanceSpecifier),
                                open_brace = Required(OpenBrace),
                                members = Optional(reference = ContractMembers),
                                close_brace = Required(CloseBrace)
                            )
                        ),
                        Struct(
                            name = InheritanceSpecifier,
                            fields = (
                                is_keyword = Required(IsKeyword),
                                types = Required(InheritanceTypes)
                            )
                        ),
                        Separated(
                            name = InheritanceTypes,
                            reference = InheritanceType,
                            separator = Comma
                        ),
                        Struct(
                            name = InheritanceType,
                            fields = (
                                type_name = Required(IdentifierPath),
                                arguments = Optional(reference = ArgumentsDeclaration)
                            )
                        ),
                        Repeated(name = ContractMembers, reference = ContractMember),
                        Enum(
                            name = ContractMember,
                            variants = [
                                EnumVariant(reference = UsingDirective),
                                EnumVariant(reference = FunctionDefinition),
                                EnumVariant(
                                    reference = ConstructorDefinition,
                                    enabled = From("0.4.22")
                                ),
                                EnumVariant(
                                    reference = ReceiveFunctionDefinition,
                                    enabled = From("0.6.0")
                                ),
                                EnumVariant(
                                    reference = FallbackFunctionDefinition,
                                    enabled = From("0.6.0")
                                ),
                                EnumVariant(
                                    reference = UnnamedFunctionDefinition,
                                    enabled = Till("0.6.0")
                                ),
                                EnumVariant(reference = ModifierDefinition),
                                EnumVariant(reference = StructDefinition),
                                EnumVariant(reference = EnumDefinition),
                                EnumVariant(reference = EventDefinition),
                                EnumVariant(reference = StateVariableDefinition),
                                EnumVariant(reference = ErrorDefinition, enabled = From("0.8.4")),
                                EnumVariant(
                                    reference = UserDefinedValueTypeDefinition,
                                    enabled = From("0.8.8")
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
                                interface_keyword = Required(InterfaceKeyword),
                                name = Required(Identifier),
                                inheritence = Optional(reference = InheritanceSpecifier),
                                open_brace = Required(OpenBrace),
                                members = Optional(reference = InterfaceMembers),
                                close_brace = Required(CloseBrace)
                            )
                        ),
                        Repeated(name = InterfaceMembers, reference = ContractMember)
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
                                library_keyword = Required(LibraryKeyword),
                                name = Required(Identifier),
                                open_brace = Required(OpenBrace),
                                members = Optional(reference = LibraryMembers),
                                close_brace = Required(CloseBrace)
                            )
                        ),
                        Repeated(name = LibraryMembers, reference = ContractMember)
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
                                struct_keyword = Required(StructKeyword),
                                name = Required(Identifier),
                                open_brace = Required(OpenBrace),
                                members = Optional(reference = StructMembers),
                                close_brace = Required(CloseBrace)
                            )
                        ),
                        Repeated(name = StructMembers, reference = StructMember),
                        Struct(
                            name = StructMember,
                            error_recovery = FieldsErrorRecovery(terminator = semicolon),
                            fields = (
                                type_name = Required(TypeName),
                                name = Required(Identifier),
                                semicolon = Required(Semicolon)
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
                                enum_keyword = Required(EnumKeyword),
                                name = Required(Identifier),
                                open_brace = Required(OpenBrace),
                                members = Optional(reference = EnumMembers),
                                close_brace = Required(CloseBrace)
                            )
                        ),
                        Separated(
                            name = EnumMembers,
                            reference = Identifier,
                            separator = Comma
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
                            type_name = Required(TypeName),
                            constant_keyword = Required(ConstantKeyword),
                            name = Required(Identifier),
                            equal = Required(Equal),
                            value = Required(Expression),
                            semicolon = Required(Semicolon)
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
                                type_name = Required(TypeName),
                                attributes = Optional(reference = StateVariableAttributes),
                                name = Required(Identifier),
                                value = Optional(reference = StateVariableDefinitionValue),
                                semicolon = Required(Semicolon)
                            )
                        ),
                        Struct(
                            name = StateVariableDefinitionValue,
                            fields = (equal = Required(Equal), value = Required(Expression))
                        ),
                        Repeated(
                            name = StateVariableAttributes,
                            reference = StateVariableAttribute
                        ),
                        Enum(
                            name = StateVariableAttribute,
                            variants = [
                                EnumVariant(reference = OverrideSpecifier),
                                EnumVariant(reference = ConstantKeyword),
                                EnumVariant(reference = InternalKeyword),
                                EnumVariant(reference = PrivateKeyword),
                                EnumVariant(reference = PublicKeyword),
                                EnumVariant(reference = ImmutableKeyword, enabled = From("0.6.5"))
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
                                function_keyword = Required(FunctionKeyword),
                                name = Required(FunctionName),
                                parameters = Required(ParametersDeclaration),
                                attributes = Optional(reference = FunctionAttributes),
                                returns = Optional(reference = ReturnsDeclaration),
                                body = Required(FunctionBody)
                            )
                        ),
                        Enum(
                            name = FunctionName,
                            variants = [
                                EnumVariant(reference = Identifier),
                                EnumVariant(reference = FallbackKeyword),
                                EnumVariant(reference = ReceiveKeyword)
                            ]
                        ),
                        Struct(
                            name = ParametersDeclaration,
                            error_recovery = FieldsErrorRecovery(
                                delimiters =
                                    FieldDelimiters(open = open_paren, close = close_paren)
                            ),
                            fields = (
                                open_paren = Required(OpenParen),
                                parameters = Optional(reference = Parameters),
                                close_paren = Required(CloseParen)
                            )
                        ),
                        Separated(name = Parameters, reference = Parameter, separator = Comma),
                        Struct(
                            name = Parameter,
                            fields = (
                                type_name = Required(TypeName),
                                storage_location = Optional(reference = StorageLocation),
                                name = Optional(reference = Identifier)
                            )
                        ),
                        Repeated(name = FunctionAttributes, reference = FunctionAttribute),
                        Enum(
                            name = FunctionAttribute,
                            variants = [
                                EnumVariant(reference = ModifierInvocation),
                                EnumVariant(reference = OverrideSpecifier),
                                EnumVariant(reference = ConstantKeyword, enabled = Till("0.5.0")),
                                EnumVariant(reference = ExternalKeyword),
                                EnumVariant(reference = InternalKeyword),
                                EnumVariant(reference = PayableKeyword),
                                EnumVariant(reference = PrivateKeyword),
                                EnumVariant(reference = PublicKeyword),
                                EnumVariant(reference = PureKeyword),
                                EnumVariant(reference = ViewKeyword),
                                EnumVariant(reference = VirtualKeyword, enabled = From("0.6.0"))
                            ]
                        ),
                        Struct(
                            name = OverrideSpecifier,
                            fields = (
                                override_keyword = Required(OverrideKeyword),
                                overridden = Optional(reference = OverridePathsDeclaration)
                            )
                        ),
                        Struct(
                            name = OverridePathsDeclaration,
                            error_recovery = FieldsErrorRecovery(
                                delimiters =
                                    FieldDelimiters(open = open_paren, close = close_paren)
                            ),
                            fields = (
                                open_paren = Required(OpenParen),
                                paths = Required(OverridePaths),
                                close_paren = Required(CloseParen)
                            )
                        ),
                        Separated(
                            name = OverridePaths,
                            reference = IdentifierPath,
                            separator = Comma
                        ),
                        Struct(
                            name = ReturnsDeclaration,
                            fields = (
                                returns_keyword = Required(ReturnsKeyword),
                                variables = Required(ParametersDeclaration)
                            )
                        ),
                        Enum(
                            name = FunctionBody,
                            variants = [
                                EnumVariant(reference = Block),
                                EnumVariant(reference = Semicolon)
                            ]
                        ),
                        Struct(
                            name = ConstructorDefinition,
                            enabled = From("0.4.22"),
                            fields = (
                                constructor_keyword = Required(ConstructorKeyword),
                                parameters = Required(ParametersDeclaration),
                                attributes = Optional(reference = ConstructorAttributes),
                                body = Required(Block)
                            )
                        ),
                        Repeated(
                            name = ConstructorAttributes,
                            reference = ConstructorAttribute,
                            enabled = From("0.4.22")
                        ),
                        Enum(
                            name = ConstructorAttribute,
                            enabled = From("0.4.22"),
                            variants = [
                                EnumVariant(reference = ModifierInvocation),
                                EnumVariant(reference = InternalKeyword),
                                EnumVariant(reference = PayableKeyword),
                                EnumVariant(reference = PublicKeyword)
                            ]
                        ),
                        Struct(
                            name = UnnamedFunctionDefinition,
                            enabled = Till("0.6.0"),
                            fields = (
                                function_keyword = Required(FunctionKeyword),
                                parameters = Required(ParametersDeclaration),
                                attributes = Optional(reference = UnnamedFunctionAttributes),
                                body = Required(FunctionBody)
                            )
                        ),
                        Repeated(
                            name = UnnamedFunctionAttributes,
                            reference = UnnamedFunctionAttribute,
                            enabled = Till("0.6.0")
                        ),
                        Enum(
                            name = UnnamedFunctionAttribute,
                            enabled = Till("0.6.0"),
                            variants = [
                                EnumVariant(reference = ModifierInvocation),
                                EnumVariant(reference = OverrideSpecifier),
                                EnumVariant(reference = ExternalKeyword),
                                EnumVariant(reference = PayableKeyword),
                                EnumVariant(reference = PureKeyword),
                                EnumVariant(reference = ViewKeyword)
                            ]
                        ),
                        Struct(
                            name = FallbackFunctionDefinition,
                            enabled = From("0.6.0"),
                            fields = (
                                fallback_keyword = Required(FallbackKeyword),
                                parameters = Required(ParametersDeclaration),
                                attributes = Optional(reference = FallbackFunctionAttributes),
                                returns = Optional(reference = ReturnsDeclaration),
                                body = Required(FunctionBody)
                            )
                        ),
                        Repeated(
                            name = FallbackFunctionAttributes,
                            reference = FallbackFunctionAttribute,
                            enabled = From("0.6.0")
                        ),
                        Enum(
                            name = FallbackFunctionAttribute,
                            enabled = From("0.6.0"),
                            variants = [
                                EnumVariant(reference = ModifierInvocation),
                                EnumVariant(reference = OverrideSpecifier),
                                EnumVariant(reference = ExternalKeyword),
                                EnumVariant(reference = PayableKeyword),
                                EnumVariant(reference = PureKeyword),
                                EnumVariant(reference = ViewKeyword),
                                EnumVariant(reference = VirtualKeyword)
                            ]
                        ),
                        Struct(
                            name = ReceiveFunctionDefinition,
                            enabled = From("0.6.0"),
                            fields = (
                                receive_keyword = Required(ReceiveKeyword),
                                parameters = Required(ParametersDeclaration),
                                attributes = Optional(reference = ReceiveFunctionAttributes),
                                body = Required(FunctionBody)
                            )
                        ),
                        Repeated(
                            name = ReceiveFunctionAttributes,
                            reference = ReceiveFunctionAttribute,
                            enabled = From("0.6.0")
                        ),
                        Enum(
                            name = ReceiveFunctionAttribute,
                            enabled = From("0.6.0"),
                            variants = [
                                EnumVariant(reference = ModifierInvocation),
                                EnumVariant(reference = OverrideSpecifier),
                                EnumVariant(reference = ExternalKeyword),
                                EnumVariant(reference = PayableKeyword),
                                EnumVariant(reference = VirtualKeyword)
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
                                modifier_keyword = Required(ModifierKeyword),
                                name = Required(Identifier),
                                parameters = Optional(reference = ParametersDeclaration),
                                attributes = Optional(reference = ModifierAttributes),
                                body = Required(FunctionBody)
                            )
                        ),
                        Repeated(name = ModifierAttributes, reference = ModifierAttribute),
                        Enum(
                            name = ModifierAttribute,
                            variants = [
                                EnumVariant(reference = OverrideSpecifier),
                                EnumVariant(reference = VirtualKeyword, enabled = From("0.6.0"))
                            ]
                        ),
                        Struct(
                            name = ModifierInvocation,
                            fields = (
                                name = Required(IdentifierPath),
                                arguments = Optional(reference = ArgumentsDeclaration)
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
                                event_keyword = Required(EventKeyword),
                                name = Required(Identifier),
                                parameters = Required(EventParametersDeclaration),
                                anonymous_keyword = Optional(reference = AnonymousKeyword),
                                semicolon = Required(Semicolon)
                            )
                        ),
                        Struct(
                            name = EventParametersDeclaration,
                            error_recovery = FieldsErrorRecovery(
                                delimiters =
                                    FieldDelimiters(open = open_paren, close = close_paren)
                            ),
                            fields = (
                                open_paren = Required(OpenParen),
                                parameters = Optional(reference = EventParameters),
                                close_paren = Required(CloseParen)
                            )
                        ),
                        Separated(
                            name = EventParameters,
                            reference = EventParameter,
                            separator = Comma
                        ),
                        Struct(
                            name = EventParameter,
                            fields = (
                                type_name = Required(TypeName),
                                indexed_keyword = Optional(reference = IndexedKeyword),
                                name = Optional(reference = Identifier)
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
                            type_keyword = Required(TypeKeyword),
                            name = Required(Identifier),
                            is_keyword = Required(IsKeyword),
                            value_type = Required(ElementaryType),
                            semicolon = Required(Semicolon)
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
                                error_keyword = Required(ErrorKeyword),
                                name = Required(Identifier),
                                members = Required(ErrorParametersDeclaration),
                                semicolon = Required(Semicolon)
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
                                open_paren = Required(OpenParen),
                                parameters = Optional(reference = ErrorParameters),
                                close_paren = Required(CloseParen)
                            )
                        ),
                        Separated(
                            name = ErrorParameters,
                            reference = ErrorParameter,
                            separator = Comma,
                            enabled = From("0.8.4")
                        ),
                        Struct(
                            name = ErrorParameter,
                            enabled = From("0.8.4"),
                            fields = (
                                type_name = Required(TypeName),
                                name = Optional(reference = Identifier)
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
                                        open_bracket = Required(OpenBracket),
                                        index = Optional(reference = Expression),
                                        close_bracket = Required(CloseBracket)
                                    )
                                )]
                            )],
                            primary_expressions = [
                                PrimaryExpression(reference = FunctionType),
                                PrimaryExpression(reference = MappingType),
                                PrimaryExpression(reference = ElementaryType),
                                PrimaryExpression(reference = IdentifierPath)
                            ]
                        ),
                        Struct(
                            name = FunctionType,
                            fields = (
                                function_keyword = Required(FunctionKeyword),
                                parameters = Required(ParametersDeclaration),
                                attributes = Optional(reference = FunctionTypeAttributes),
                                returns = Optional(reference = ReturnsDeclaration)
                            )
                        ),
                        Repeated(
                            name = FunctionTypeAttributes,
                            reference = FunctionTypeAttribute
                        ),
                        Enum(
                            name = FunctionTypeAttribute,
                            variants = [
                                EnumVariant(reference = InternalKeyword),
                                EnumVariant(reference = ExternalKeyword),
                                EnumVariant(reference = PrivateKeyword),
                                EnumVariant(reference = PublicKeyword),
                                EnumVariant(reference = PureKeyword),
                                EnumVariant(reference = ViewKeyword),
                                EnumVariant(reference = PayableKeyword)
                            ]
                        ),
                        Struct(
                            name = MappingType,
                            error_recovery = FieldsErrorRecovery(
                                delimiters =
                                    FieldDelimiters(open = open_paren, close = close_paren)
                            ),
                            fields = (
                                mapping_keyword = Required(MappingKeyword),
                                open_paren = Required(OpenParen),
                                key_type = Required(MappingKey),
                                equal_greater_than = Required(EqualGreaterThan),
                                value_type = Required(MappingValue),
                                close_paren = Required(CloseParen)
                            )
                        ),
                        Struct(
                            name = MappingKey,
                            fields = (
                                key_type = Required(MappingKeyType),
                                name = Optional(reference = Identifier, enabled = From("0.8.18"))
                            )
                        ),
                        Enum(
                            name = MappingKeyType,
                            variants = [
                                EnumVariant(reference = ElementaryType),
                                EnumVariant(reference = IdentifierPath)
                            ]
                        ),
                        Struct(
                            name = MappingValue,
                            fields = (
                                type_name = Required(TypeName),
                                name = Optional(reference = Identifier, enabled = From("0.8.18"))
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
                                EnumVariant(reference = BoolKeyword),
                                EnumVariant(reference = ByteKeyword, enabled = Till("0.8.0")),
                                EnumVariant(reference = StringKeyword),
                                EnumVariant(reference = AddressType),
                                EnumVariant(reference = BytesKeyword),
                                EnumVariant(reference = IntKeyword),
                                EnumVariant(reference = UintKeyword),
                                EnumVariant(reference = FixedKeyword),
                                EnumVariant(reference = UfixedKeyword)
                            ]
                        ),
                        Struct(
                            name = AddressType,
                            fields = (
                                address_keyword = Required(AddressKeyword),
                                payable_keyword = Optional(reference = PayableKeyword)
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
                                open_brace = Required(OpenBrace),
                                statements = Optional(reference = Statements),
                                close_brace = Required(CloseBrace)
                            )
                        ),
                        Repeated(name = Statements, reference = Statement),
                        Enum(
                            name = Statement,
                            variants = [
                                // Simple statements
                                EnumVariant(reference = ExpressionStatement),
                                EnumVariant(reference = VariableDeclarationStatement),
                                EnumVariant(reference = TupleDeconstructionStatement),
                                // Control statements
                                EnumVariant(reference = IfStatement),
                                EnumVariant(reference = ForStatement),
                                EnumVariant(reference = WhileStatement),
                                EnumVariant(reference = DoWhileStatement),
                                EnumVariant(reference = ContinueStatement),
                                EnumVariant(reference = BreakStatement),
                                EnumVariant(reference = DeleteStatement),
                                EnumVariant(reference = ReturnStatement),
                                EnumVariant(reference = ThrowStatement, enabled = Till("0.5.0")),
                                EnumVariant(reference = EmitStatement, enabled = From("0.4.21")),
                                EnumVariant(reference = TryStatement, enabled = From("0.6.0")),
                                EnumVariant(reference = RevertStatement, enabled = From("0.8.4")),
                                EnumVariant(reference = AssemblyStatement),
                                EnumVariant(reference = Block),
                                EnumVariant(reference = UncheckedBlock, enabled = From("0.8.0"))
                            ]
                        ),
                        Struct(
                            name = UncheckedBlock,
                            enabled = From("0.8.0"),
                            fields = (
                                unchecked_keyword = Required(UncheckedKeyword),
                                block = Required(Block)
                            )
                        ),
                        Struct(
                            name = ExpressionStatement,
                            error_recovery = FieldsErrorRecovery(terminator = semicolon),
                            fields = (
                                expression = Required(Expression),
                                semicolon = Required(Semicolon)
                            )
                        ),
                        Struct(
                            name = AssemblyStatement,
                            fields = (
                                assembly_keyword = Required(AssemblyKeyword),
                                label = Optional(reference = AsciiStringLiteral),
                                flags = Optional(reference = AssemblyFlagsDeclaration),
                                body = Required(YulBlock)
                            )
                        ),
                        Struct(
                            name = AssemblyFlagsDeclaration,
                            error_recovery = FieldsErrorRecovery(
                                delimiters =
                                    FieldDelimiters(open = open_paren, close = close_paren)
                            ),
                            fields = (
                                open_paren = Required(OpenParen),
                                flags = Required(AssemblyFlags),
                                close_paren = Required(CloseParen)
                            )
                        ),
                        Separated(
                            name = AssemblyFlags,
                            reference = AsciiStringLiteral,
                            separator = Comma
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
                                open_paren = Required(OpenParen),
                                elements = Required(TupleDeconstructionElements),
                                close_paren = Required(CloseParen),
                                equal = Required(Equal),
                                expression = Required(Expression),
                                semicolon = Required(Semicolon)
                            )
                        ),
                        Separated(
                            name = TupleDeconstructionElements,
                            reference = TupleDeconstructionElement,
                            separator = Comma
                        ),
                        Struct(
                            name = TupleDeconstructionElement,
                            fields = (member = Optional(reference = TupleMember))
                        ),
                        Enum(
                            name = TupleMember,
                            variants = [
                                EnumVariant(reference = TypedTupleMember),
                                EnumVariant(reference = UntypedTupleMember)
                            ]
                        ),
                        Struct(
                            name = TypedTupleMember,
                            fields = (
                                type_name = Required(TypeName),
                                storage_location = Optional(reference = StorageLocation),
                                name = Required(Identifier)
                            )
                        ),
                        Struct(
                            name = UntypedTupleMember,
                            fields = (
                                storage_location = Optional(reference = StorageLocation),
                                name = Required(Identifier)
                            )
                        ),
                        Struct(
                            name = VariableDeclarationStatement,
                            error_recovery = FieldsErrorRecovery(terminator = semicolon),
                            fields = (
                                variable_type = Required(VariableDeclarationType),
                                storage_location = Optional(reference = StorageLocation),
                                name = Required(Identifier),
                                value = Optional(reference = VariableDeclarationValue),
                                semicolon = Required(Semicolon)
                            )
                        ),
                        Enum(
                            name = VariableDeclarationType,
                            variants = [
                                EnumVariant(reference = TypeName),
                                EnumVariant(reference = VarKeyword, enabled = Till("0.5.0"))
                            ]
                        ),
                        Struct(
                            name = VariableDeclarationValue,
                            fields = (equal = Required(Equal), expression = Required(Expression))
                        ),
                        Enum(
                            name = StorageLocation,
                            variants = [
                                EnumVariant(reference = MemoryKeyword),
                                EnumVariant(reference = StorageKeyword),
                                EnumVariant(reference = CallDataKeyword, enabled = From("0.5.0"))
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
                                if_keyword = Required(IfKeyword),
                                open_paren = Required(OpenParen),
                                condition = Required(Expression),
                                close_paren = Required(CloseParen),
                                body = Required(Statement),
                                else_branch = Optional(reference = ElseBranch)
                            )
                        ),
                        Struct(
                            name = ElseBranch,
                            fields = (
                                else_keyword = Required(ElseKeyword),
                                body = Required(Statement)
                            )
                        ),
                        Struct(
                            name = ForStatement,
                            error_recovery = FieldsErrorRecovery(
                                delimiters =
                                    FieldDelimiters(open = open_paren, close = close_paren)
                            ),
                            fields = (
                                for_keyword = Required(ForKeyword),
                                open_paren = Required(OpenParen),
                                initialization = Required(ForStatementInitialization),
                                condition = Required(ForStatementCondition),
                                iterator = Optional(reference = Expression),
                                close_paren = Required(CloseParen),
                                body = Required(Statement)
                            )
                        ),
                        Enum(
                            name = ForStatementInitialization,
                            variants = [
                                EnumVariant(reference = ExpressionStatement),
                                EnumVariant(reference = VariableDeclarationStatement),
                                EnumVariant(reference = TupleDeconstructionStatement),
                                EnumVariant(reference = Semicolon)
                            ]
                        ),
                        Enum(
                            name = ForStatementCondition,
                            variants = [
                                EnumVariant(reference = ExpressionStatement),
                                EnumVariant(reference = Semicolon)
                            ]
                        ),
                        Struct(
                            name = WhileStatement,
                            error_recovery = FieldsErrorRecovery(
                                delimiters =
                                    FieldDelimiters(open = open_paren, close = close_paren)
                            ),
                            fields = (
                                while_keyword = Required(WhileKeyword),
                                open_paren = Required(OpenParen),
                                condition = Required(Expression),
                                close_paren = Required(CloseParen),
                                body = Required(Statement)
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
                                do_keyword = Required(DoKeyword),
                                body = Required(Statement),
                                while_keyword = Required(WhileKeyword),
                                open_paren = Required(OpenParen),
                                condition = Required(Expression),
                                close_paren = Required(CloseParen),
                                semicolon = Required(Semicolon)
                            )
                        ),
                        Struct(
                            name = ContinueStatement,
                            error_recovery = FieldsErrorRecovery(terminator = semicolon),
                            fields = (
                                continue_keyword = Required(ContinueKeyword),
                                semicolon = Required(Semicolon)
                            )
                        ),
                        Struct(
                            name = BreakStatement,
                            error_recovery = FieldsErrorRecovery(terminator = semicolon),
                            fields = (
                                break_keyword = Required(BreakKeyword),
                                semicolon = Required(Semicolon)
                            )
                        ),
                        Struct(
                            name = ReturnStatement,
                            error_recovery = FieldsErrorRecovery(terminator = semicolon),
                            fields = (
                                return_keyword = Required(ReturnKeyword),
                                expression = Optional(reference = Expression),
                                semicolon = Required(Semicolon)
                            )
                        ),
                        Struct(
                            name = EmitStatement,
                            enabled = From("0.4.21"),
                            error_recovery = FieldsErrorRecovery(terminator = semicolon),
                            fields = (
                                emit_keyword = Required(EmitKeyword),
                                event = Required(IdentifierPath),
                                arguments = Required(ArgumentsDeclaration),
                                semicolon = Required(Semicolon)
                            )
                        ),
                        Struct(
                            name = DeleteStatement,
                            error_recovery = FieldsErrorRecovery(terminator = semicolon),
                            fields = (
                                delete_keyword = Required(DeleteKeyword),
                                expression = Required(Expression),
                                semicolon = Required(Semicolon)
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
                                try_keyword = Required(TryKeyword),
                                expression = Required(Expression),
                                returns = Optional(reference = ReturnsDeclaration),
                                body = Required(Block),
                                catch_clauses = Required(CatchClauses)
                            )
                        ),
                        Repeated(
                            name = CatchClauses,
                            reference = CatchClause,
                            enabled = From("0.6.0")
                        ),
                        Struct(
                            name = CatchClause,
                            enabled = From("0.6.0"),
                            fields = (
                                catch_keyword = Required(CatchKeyword),
                                error = Optional(reference = CatchClauseError),
                                body = Required(Block)
                            )
                        ),
                        Struct(
                            name = CatchClauseError,
                            enabled = From("0.6.0"),
                            fields = (
                                name = Optional(reference = Identifier),
                                parameters = Required(ParametersDeclaration)
                            )
                        ),
                        Struct(
                            name = RevertStatement,
                            enabled = From("0.8.4"),
                            error_recovery = FieldsErrorRecovery(terminator = semicolon),
                            fields = (
                                revert_keyword = Required(RevertKeyword),
                                error = Optional(reference = IdentifierPath),
                                arguments = Required(ArgumentsDeclaration),
                                semicolon = Required(Semicolon)
                            )
                        ),
                        Struct(
                            name = ThrowStatement,
                            enabled = Till("0.5.0"),
                            error_recovery = FieldsErrorRecovery(terminator = semicolon),
                            fields = (
                                throw_keyword = Required(ThrowKeyword),
                                semicolon = Required(Semicolon)
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
                                    operators = [
                                        PrecedenceOperator(
                                            model = BinaryLeftAssociative,
                                            fields = (operator = Required(Equal))
                                        ),
                                        PrecedenceOperator(
                                            model = BinaryLeftAssociative,
                                            fields = (operator = Required(BarEqual))
                                        ),
                                        PrecedenceOperator(
                                            model = BinaryLeftAssociative,
                                            fields = (operator = Required(PlusEqual))
                                        ),
                                        PrecedenceOperator(
                                            model = BinaryLeftAssociative,
                                            fields = (operator = Required(MinusEqual))
                                        ),
                                        PrecedenceOperator(
                                            model = BinaryLeftAssociative,
                                            fields = (operator = Required(CaretEqual))
                                        ),
                                        PrecedenceOperator(
                                            model = BinaryLeftAssociative,
                                            fields = (operator = Required(SlashEqual))
                                        ),
                                        PrecedenceOperator(
                                            model = BinaryLeftAssociative,
                                            fields = (operator = Required(PercentEqual))
                                        ),
                                        PrecedenceOperator(
                                            model = BinaryLeftAssociative,
                                            fields = (operator = Required(AsteriskEqual))
                                        ),
                                        PrecedenceOperator(
                                            model = BinaryLeftAssociative,
                                            fields = (operator = Required(AmpersandEqual))
                                        ),
                                        PrecedenceOperator(
                                            model = BinaryLeftAssociative,
                                            fields = (operator = Required(LessThanLessThanEqual))
                                        ),
                                        PrecedenceOperator(
                                            model = BinaryLeftAssociative,
                                            fields =
                                                (operator = Required(GreaterThanGreaterThanEqual))
                                        ),
                                        PrecedenceOperator(
                                            model = BinaryLeftAssociative,
                                            fields = (operator =
                                                Required(GreaterThanGreaterThanGreaterThanEqual))
                                        )
                                    ]
                                ),
                                PrecedenceExpression(
                                    name = ConditionalExpression,
                                    operators = [PrecedenceOperator(
                                        model = Postfix,
                                        fields = (
                                            question_mark = Required(QuestionMark),
                                            true_expression = Required(Expression),
                                            colon = Required(Colon),
                                            false_expression = Required(Expression)
                                        )
                                    )]
                                ),
                                PrecedenceExpression(
                                    name = OrExpression,
                                    operators = [PrecedenceOperator(
                                        model = BinaryLeftAssociative,
                                        fields = (operator = Required(BarBar))
                                    )]
                                ),
                                PrecedenceExpression(
                                    name = AndExpression,
                                    operators = [PrecedenceOperator(
                                        model = BinaryLeftAssociative,
                                        fields = (operator = Required(AmpersandAmpersand))
                                    )]
                                ),
                                PrecedenceExpression(
                                    name = EqualityExpression,
                                    operators = [
                                        PrecedenceOperator(
                                            model = BinaryLeftAssociative,
                                            fields = (operator = Required(EqualEqual))
                                        ),
                                        PrecedenceOperator(
                                            model = BinaryLeftAssociative,
                                            fields = (operator = Required(BangEqual))
                                        )
                                    ]
                                ),
                                PrecedenceExpression(
                                    name = ComparisonExpression,
                                    operators = [
                                        PrecedenceOperator(
                                            model = BinaryLeftAssociative,
                                            fields = (operator = Required(LessThan))
                                        ),
                                        PrecedenceOperator(
                                            model = BinaryLeftAssociative,
                                            fields = (operator = Required(GreaterThan))
                                        ),
                                        PrecedenceOperator(
                                            model = BinaryLeftAssociative,
                                            fields = (operator = Required(LessThanEqual))
                                        ),
                                        PrecedenceOperator(
                                            model = BinaryLeftAssociative,
                                            fields = (operator = Required(GreaterThanEqual))
                                        )
                                    ]
                                ),
                                PrecedenceExpression(
                                    name = BitwiseOrExpression,
                                    operators = [PrecedenceOperator(
                                        model = BinaryLeftAssociative,
                                        fields = (operator = Required(Bar))
                                    )]
                                ),
                                PrecedenceExpression(
                                    name = BitwiseXorExpression,
                                    operators = [PrecedenceOperator(
                                        model = BinaryLeftAssociative,
                                        fields = (operator = Required(Caret))
                                    )]
                                ),
                                PrecedenceExpression(
                                    name = BitwiseAndExpression,
                                    operators = [PrecedenceOperator(
                                        model = BinaryLeftAssociative,
                                        fields = (operator = Required(Ampersand))
                                    )]
                                ),
                                PrecedenceExpression(
                                    name = ShiftExpression,
                                    operators = [
                                        PrecedenceOperator(
                                            model = BinaryLeftAssociative,
                                            fields = (operator = Required(LessThanLessThan))
                                        ),
                                        PrecedenceOperator(
                                            model = BinaryLeftAssociative,
                                            fields = (operator = Required(GreaterThanGreaterThan))
                                        ),
                                        PrecedenceOperator(
                                            model = BinaryLeftAssociative,
                                            fields = (operator =
                                                Required(GreaterThanGreaterThanGreaterThan))
                                        )
                                    ]
                                ),
                                PrecedenceExpression(
                                    name = AdditiveExpression,
                                    operators = [
                                        PrecedenceOperator(
                                            model = BinaryLeftAssociative,
                                            fields = (operator = Required(Plus))
                                        ),
                                        PrecedenceOperator(
                                            model = BinaryLeftAssociative,
                                            fields = (operator = Required(Minus))
                                        )
                                    ]
                                ),
                                PrecedenceExpression(
                                    name = MultiplicativeExpression,
                                    operators = [
                                        PrecedenceOperator(
                                            model = BinaryLeftAssociative,
                                            fields = (operator = Required(Asterisk))
                                        ),
                                        PrecedenceOperator(
                                            model = BinaryLeftAssociative,
                                            fields = (operator = Required(Slash))
                                        ),
                                        PrecedenceOperator(
                                            model = BinaryLeftAssociative,
                                            fields = (operator = Required(Percent))
                                        )
                                    ]
                                ),
                                PrecedenceExpression(
                                    name = ExponentiationExpression,
                                    operators = [
                                        // Before '0.6.0', it was left-associative:
                                        PrecedenceOperator(
                                            model = BinaryLeftAssociative,
                                            enabled = Till("0.6.0"),
                                            fields = (operator = Required(AsteriskAsterisk))
                                        ),
                                        // In '0.6.0', it became right-associative:
                                        PrecedenceOperator(
                                            model = BinaryRightAssociative,
                                            enabled = From("0.6.0"),
                                            fields = (operator = Required(AsteriskAsterisk))
                                        )
                                    ]
                                ),
                                PrecedenceExpression(
                                    name = PostfixExpression,
                                    operators = [
                                        PrecedenceOperator(
                                            model = Postfix,
                                            fields = (operator = Required(PlusPlus))
                                        ),
                                        PrecedenceOperator(
                                            model = Postfix,
                                            fields = (operator = Required(MinusMinus))
                                        )
                                    ]
                                ),
                                PrecedenceExpression(
                                    name = PrefixExpression,
                                    operators = [
                                        PrecedenceOperator(
                                            model = Prefix,
                                            fields = (operator = Required(PlusPlus))
                                        ),
                                        PrecedenceOperator(
                                            model = Prefix,
                                            fields = (operator = Required(MinusMinus))
                                        ),
                                        PrecedenceOperator(
                                            model = Prefix,
                                            fields = (operator = Required(Tilde))
                                        ),
                                        PrecedenceOperator(
                                            model = Prefix,
                                            fields = (operator = Required(Bang))
                                        ),
                                        PrecedenceOperator(
                                            model = Prefix,
                                            fields = (operator = Required(Minus))
                                        ),
                                        PrecedenceOperator(
                                            model = Prefix,
                                            enabled = Till("0.5.0"),
                                            fields = (operator = Required(Plus))
                                        )
                                    ]
                                ),
                                PrecedenceExpression(
                                    name = FunctionCallExpression,
                                    operators = [PrecedenceOperator(
                                        model = Postfix,
                                        fields = (
                                            options = Optional(
                                                reference = FunctionCallOptions,
                                                enabled = From("0.6.2")
                                            ),
                                            arguments = Required(ArgumentsDeclaration)
                                        )
                                    )]
                                ),
                                PrecedenceExpression(
                                    name = MemberAccessExpression,
                                    operators = [PrecedenceOperator(
                                        model = Postfix,
                                        fields = (
                                            period = Required(Period),
                                            member = Required(MemberAccess)
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
                                            open_bracket = Required(OpenBracket),
                                            start = Optional(reference = Expression),
                                            end = Optional(reference = IndexAccessEnd),
                                            close_bracket = Required(CloseBracket)
                                        )
                                    )]
                                )
                            ],
                            primary_expressions = [
                                PrimaryExpression(reference = NewExpression),
                                PrimaryExpression(reference = TupleExpression),
                                PrimaryExpression(
                                    reference = TypeExpression,
                                    enabled = From("0.5.3")
                                ),
                                PrimaryExpression(reference = ArrayExpression),
                                PrimaryExpression(reference = HexNumberExpression),
                                PrimaryExpression(reference = DecimalNumberExpression),
                                PrimaryExpression(reference = StringExpression),
                                PrimaryExpression(reference = ElementaryType),
                                PrimaryExpression(
                                    reference = PayableKeyword,
                                    enabled = From("0.6.0")
                                ),
                                PrimaryExpression(reference = TrueKeyword),
                                PrimaryExpression(reference = FalseKeyword),
                                PrimaryExpression(reference = Identifier)
                            ]
                        ),
                        Enum(
                            name = MemberAccess,
                            variants = [
                                EnumVariant(reference = Identifier),
                                EnumVariant(reference = AddressKeyword)
                            ]
                        ),
                        Struct(
                            name = IndexAccessEnd,
                            fields = (
                                colon = Required(Colon),
                                end = Optional(reference = Expression)
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
                                    reference = NamedArgumentGroups,
                                    enabled = Range(from = "0.6.2", till = "0.8.0")
                                ),
                                EnumVariant(
                                    reference = NamedArgumentGroup,
                                    enabled = From("0.8.0")
                                )
                            ]
                        ),
                        Enum(
                            name = ArgumentsDeclaration,
                            variants = [
                                EnumVariant(reference = PositionalArgumentsDeclaration),
                                EnumVariant(reference = NamedArgumentsDeclaration)
                            ]
                        ),
                        Struct(
                            name = PositionalArgumentsDeclaration,
                            error_recovery = FieldsErrorRecovery(
                                delimiters =
                                    FieldDelimiters(open = open_paren, close = close_paren)
                            ),
                            fields = (
                                open_paren = Required(OpenParen),
                                arguments = Optional(reference = PositionalArguments),
                                close_paren = Required(CloseParen)
                            )
                        ),
                        Separated(
                            name = PositionalArguments,
                            reference = Expression,
                            separator = Comma
                        ),
                        Struct(
                            name = NamedArgumentsDeclaration,
                            error_recovery = FieldsErrorRecovery(
                                delimiters =
                                    FieldDelimiters(open = open_paren, close = close_paren)
                            ),
                            fields = (
                                open_paren = Required(OpenParen),
                                arguments = Optional(reference = NamedArgumentGroup),
                                close_paren = Required(CloseParen)
                            )
                        ),
                        Repeated(
                            name = NamedArgumentGroups,
                            reference = NamedArgumentGroup,
                            enabled = Range(from = "0.6.2", till = "0.8.0")
                        ),
                        Struct(
                            name = NamedArgumentGroup,
                            error_recovery = FieldsErrorRecovery(
                                delimiters =
                                    FieldDelimiters(open = open_brace, close = close_brace)
                            ),
                            fields = (
                                open_brace = Required(OpenBrace),
                                arguments = Optional(reference = NamedArguments),
                                close_brace = Required(CloseBrace)
                            )
                        ),
                        Separated(
                            name = NamedArguments,
                            reference = NamedArgument,
                            separator = Comma
                        ),
                        Struct(
                            name = NamedArgument,
                            fields = (
                                name = Required(Identifier),
                                colon = Required(Colon),
                                value = Required(Expression)
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
                                type_keyword = Required(TypeKeyword),
                                open_paren = Required(OpenParen),
                                type_name = Required(TypeName),
                                close_paren = Required(CloseParen)
                            )
                        ),
                        Struct(
                            name = NewExpression,
                            fields = (
                                new_keyword = Required(NewKeyword),
                                type_name = Required(TypeName)
                            )
                        ),
                        Struct(
                            name = TupleExpression,
                            error_recovery = FieldsErrorRecovery(
                                delimiters =
                                    FieldDelimiters(open = open_paren, close = close_paren)
                            ),
                            fields = (
                                open_paren = Required(OpenParen),
                                items = Required(TupleValues),
                                close_paren = Required(CloseParen)
                            )
                        ),
                        Separated(
                            name = TupleValues,
                            reference = TupleValue,
                            separator = Comma
                        ),
                        Struct(
                            name = TupleValue,
                            fields = (expression = Optional(reference = Expression))
                        ),
                        Struct(
                            name = ArrayExpression,
                            error_recovery = FieldsErrorRecovery(
                                delimiters =
                                    FieldDelimiters(open = open_bracket, close = close_bracket)
                            ),
                            fields = (
                                open_bracket = Required(OpenBracket),
                                items = Required(ArrayValues),
                                close_bracket = Required(CloseBracket)
                            )
                        ),
                        Separated(
                            name = ArrayValues,
                            reference = Expression,
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
                                literal = Required(HexLiteral),
                                unit = Optional(reference = NumberUnit, enabled = Till("0.5.0"))
                            )
                        ),
                        Struct(
                            name = DecimalNumberExpression,
                            fields = (
                                literal = Required(DecimalLiteral),
                                unit = Optional(reference = NumberUnit)
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
                                EnumVariant(reference = WeiKeyword),
                                // 1e-9 ETH
                                EnumVariant(reference = GweiKeyword, enabled = From("0.6.11")),
                                // 1e-6 ETH
                                EnumVariant(reference = SzaboKeyword, enabled = Till("0.7.0")),
                                // 1e-3 ETH
                                EnumVariant(reference = FinneyKeyword, enabled = Till("0.7.0")),
                                // 1 ETH
                                EnumVariant(reference = EtherKeyword),
                                EnumVariant(reference = SecondsKeyword),
                                EnumVariant(reference = MinutesKeyword),
                                EnumVariant(reference = HoursKeyword),
                                EnumVariant(reference = DaysKeyword),
                                EnumVariant(reference = WeeksKeyword),
                                EnumVariant(reference = YearsKeyword, enabled = Till("0.5.0"))
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
                                EnumVariant(reference = HexStringLiterals),
                                EnumVariant(reference = AsciiStringLiterals),
                                EnumVariant(
                                    reference = UnicodeStringLiterals,
                                    enabled = From("0.7.0")
                                )
                            ]
                        ),
                        Repeated(name = HexStringLiterals, reference = HexStringLiteral),
                        Token(
                            name = HexStringLiteral,
                            definitions = [
                                TokenDefinition(scanner = Fragment(SingleQuotedHexStringLiteral)),
                                TokenDefinition(scanner = Fragment(DoubleQuotedHexStringLiteral))
                            ]
                        ),
                        Fragment(
                            name = SingleQuotedHexStringLiteral,
                            scanner = Sequence([
                                Atom("hex'"),
                                Optional(Fragment(HexStringContents)),
                                Atom("'")
                            ])
                        ),
                        Fragment(
                            name = DoubleQuotedHexStringLiteral,
                            scanner = Sequence([
                                Atom("hex\""),
                                Optional(Fragment(HexStringContents)),
                                Atom("\"")
                            ])
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
                        Repeated(name = AsciiStringLiterals, reference = AsciiStringLiteral),
                        Token(
                            name = AsciiStringLiteral,
                            definitions = [
                                TokenDefinition(scanner = Fragment(SingleQuotedAsciiStringLiteral)),
                                TokenDefinition(scanner = Fragment(DoubleQuotedAsciiStringLiteral))
                            ]
                        ),
                        Fragment(
                            name = SingleQuotedAsciiStringLiteral,
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
                        ),
                        Fragment(
                            name = DoubleQuotedAsciiStringLiteral,
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
                        ),
                        Repeated(
                            name = UnicodeStringLiterals,
                            reference = UnicodeStringLiteral,
                            enabled = From("0.7.0")
                        ),
                        Token(
                            name = UnicodeStringLiteral,
                            definitions = [
                                TokenDefinition(
                                    enabled = From("0.7.0"),
                                    scanner = Fragment(SingleQuotedUnicodeStringLiteral)
                                ),
                                TokenDefinition(
                                    enabled = From("0.7.0"),
                                    scanner = Fragment(DoubleQuotedUnicodeStringLiteral)
                                )
                            ]
                        ),
                        Fragment(
                            name = SingleQuotedUnicodeStringLiteral,
                            enabled = From("0.7.0"),
                            scanner = Sequence([
                                Atom("unicode'"),
                                ZeroOrMore(Choice([
                                    Fragment(EscapeSequence),
                                    Not(['\'', '\\', '\r', '\n'])
                                ])),
                                Atom("'")
                            ])
                        ),
                        Fragment(
                            name = DoubleQuotedUnicodeStringLiteral,
                            enabled = From("0.7.0"),
                            scanner = Sequence([
                                Atom("unicode\""),
                                ZeroOrMore(Choice([
                                    Fragment(EscapeSequence),
                                    Not(['"', '\\', '\r', '\n'])
                                ])),
                                Atom("\"")
                            ])
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
                            reference = Identifier,
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
                            name = YulBlock,
                            error_recovery = FieldsErrorRecovery(
                                delimiters =
                                    FieldDelimiters(open = open_brace, close = close_brace)
                            ),
                            fields = (
                                open_brace = Required(OpenBrace),
                                statements = Optional(reference = YulStatements),
                                close_brace = Required(CloseBrace)
                            )
                        ),
                        Repeated(name = YulStatements, reference = YulStatement),
                        Enum(
                            name = YulStatement,
                            variants = [
                                EnumVariant(reference = YulBlock),
                                EnumVariant(reference = YulFunctionDefinition),
                                EnumVariant(reference = YulVariableDeclarationStatement),
                                EnumVariant(reference = YulAssignmentStatement),
                                EnumVariant(reference = YulIfStatement),
                                EnumVariant(reference = YulForStatement),
                                EnumVariant(reference = YulSwitchStatement),
                                EnumVariant(reference = YulLeaveStatement, enabled = From("0.6.0")),
                                EnumVariant(reference = YulBreakStatement),
                                EnumVariant(reference = YulContinueStatement),
                                EnumVariant(reference = YulExpression)
                            ]
                        ),
                        Struct(
                            name = YulFunctionDefinition,
                            fields = (
                                function_keyword = Required(YulFunctionKeyword),
                                name = Required(YulIdentifier),
                                parameters = Required(YulParametersDeclaration),
                                returns = Optional(reference = YulReturnsDeclaration),
                                body = Required(YulBlock)
                            )
                        ),
                        Struct(
                            name = YulParametersDeclaration,
                            error_recovery = FieldsErrorRecovery(
                                delimiters =
                                    FieldDelimiters(open = open_paren, close = close_paren)
                            ),
                            fields = (
                                open_paren = Required(OpenParen),
                                parameters = Optional(reference = YulParameters),
                                close_paren = Required(CloseParen)
                            )
                        ),
                        Separated(
                            name = YulParameters,
                            reference = YulIdentifier,
                            separator = Comma
                        ),
                        Struct(
                            name = YulReturnsDeclaration,
                            fields = (
                                minus_greater_than = Required(MinusGreaterThan),
                                variables = Required(YulReturnVariables)
                            )
                        ),
                        Separated(
                            name = YulReturnVariables,
                            reference = YulIdentifier,
                            separator = Comma
                        ),
                        Struct(
                            name = YulVariableDeclarationStatement,
                            fields = (
                                let_keyword = Required(YulLetKeyword),
                                names = Required(YulIdentifierPaths),
                                value = Optional(reference = YulVariableDeclarationValue)
                            )
                        ),
                        Struct(
                            name = YulVariableDeclarationValue,
                            fields = (
                                colon_equal = Required(ColonEqual),
                                expression = Required(YulExpression)
                            )
                        ),
                        Struct(
                            name = YulAssignmentStatement,
                            fields = (
                                names = Required(YulIdentifierPaths),
                                colon_equal = Required(ColonEqual),
                                expression = Required(YulExpression)
                            )
                        ),
                        Struct(
                            name = YulIfStatement,
                            fields = (
                                if_keyword = Required(YulIfKeyword),
                                condition = Required(YulExpression),
                                body = Required(YulBlock)
                            )
                        ),
                        Struct(
                            name = YulLeaveStatement,
                            enabled = From("0.6.0"),
                            fields = (leave_keyword = Required(YulLeaveKeyword))
                        ),
                        Struct(
                            name = YulBreakStatement,
                            fields = (break_keyword = Required(YulBreakKeyword))
                        ),
                        Struct(
                            name = YulContinueStatement,
                            fields = (continue_keyword = Required(YulContinueKeyword))
                        ),
                        Struct(
                            name = YulForStatement,
                            fields = (
                                for_keyword = Required(YulForKeyword),
                                initialization = Required(YulBlock),
                                condition = Required(YulExpression),
                                iterator = Required(YulBlock),
                                body = Required(YulBlock)
                            )
                        ),
                        Struct(
                            name = YulSwitchStatement,
                            fields = (
                                switch_keyword = Required(YulSwitchKeyword),
                                expression = Required(YulExpression),
                                cases = Required(YulSwitchCases)
                            )
                        ),
                        Repeated(name = YulSwitchCases, reference = YulSwitchCase),
                        Enum(
                            name = YulSwitchCase,
                            variants = [
                                EnumVariant(reference = YulDefaultCase),
                                EnumVariant(reference = YulValueCase)
                            ]
                        ),
                        Struct(
                            name = YulDefaultCase,
                            fields = (
                                default_keyword = Required(YulDefaultKeyword),
                                body = Required(YulBlock)
                            )
                        ),
                        Struct(
                            name = YulValueCase,
                            fields = (
                                case_keyword = Required(YulCaseKeyword),
                                value = Required(YulLiteral),
                                body = Required(YulBlock)
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
                                        open_paren = Required(OpenParen),
                                        arguments = Optional(reference = YulArguments),
                                        close_paren = Required(CloseParen)
                                    )
                                )]
                            )],
                            primary_expressions = [
                                PrimaryExpression(reference = YulLiteral),
                                PrimaryExpression(reference = YulBuiltInFunction),
                                PrimaryExpression(reference = YulIdentifierPath)
                            ]
                        ),
                        Separated(
                            name = YulArguments,
                            reference = YulExpression,
                            separator = Comma
                        ),
                        Separated(
                            name = YulIdentifierPaths,
                            reference = YulIdentifierPath,
                            separator = Comma
                        ),
                        Separated(
                            name = YulIdentifierPath,
                            reference = YulIdentifier,
                            separator = Period
                        ),
                        Token(
                            name = YulIdentifier,
                            definitions = [TokenDefinition(scanner = Fragment(RawIdentifier))]
                        ),
                        Enum(
                            name = YulBuiltInFunction,
                            variants = [
                                EnumVariant(reference = YulAddKeyword),
                                EnumVariant(reference = YulAddModKeyword),
                                EnumVariant(reference = YulAddressKeyword),
                                EnumVariant(reference = YulAndKeyword),
                                EnumVariant(reference = YulBalanceKeyword),
                                EnumVariant(reference = YulBlockHashKeyword),
                                EnumVariant(reference = YulByteKeyword),
                                EnumVariant(reference = YulCallCodeKeyword),
                                EnumVariant(reference = YulCallDataCopyKeyword),
                                EnumVariant(reference = YulCallDataLoadKeyword),
                                EnumVariant(reference = YulCallDataSizeKeyword),
                                EnumVariant(reference = YulCallerKeyword),
                                EnumVariant(reference = YulCallKeyword),
                                EnumVariant(reference = YulCallValueKeyword),
                                EnumVariant(reference = YulCoinBaseKeyword),
                                EnumVariant(reference = YulCreateKeyword),
                                EnumVariant(reference = YulDelegateCallKeyword),
                                EnumVariant(reference = YulDivKeyword),
                                EnumVariant(reference = YulEqKeyword),
                                EnumVariant(reference = YulExpKeyword),
                                EnumVariant(reference = YulExtCodeCopyKeyword),
                                EnumVariant(reference = YulExtCodeSizeKeyword),
                                EnumVariant(reference = YulGasKeyword),
                                EnumVariant(reference = YulGasLimitKeyword),
                                EnumVariant(reference = YulGasPriceKeyword),
                                EnumVariant(reference = YulGtKeyword),
                                EnumVariant(reference = YulInvalidKeyword),
                                EnumVariant(reference = YulIsZeroKeyword),
                                EnumVariant(reference = YulLog0Keyword),
                                EnumVariant(reference = YulLog1Keyword),
                                EnumVariant(reference = YulLog2Keyword),
                                EnumVariant(reference = YulLog3Keyword),
                                EnumVariant(reference = YulLog4Keyword),
                                EnumVariant(reference = YulLtKeyword),
                                EnumVariant(reference = YulMLoadKeyword),
                                EnumVariant(reference = YulModKeyword),
                                EnumVariant(reference = YulMSizeKeyword),
                                EnumVariant(reference = YulMStore8Keyword),
                                EnumVariant(reference = YulMStoreKeyword),
                                EnumVariant(reference = YulMulKeyword),
                                EnumVariant(reference = YulMulModKeyword),
                                EnumVariant(reference = YulNotKeyword),
                                EnumVariant(reference = YulNumberKeyword),
                                EnumVariant(reference = YulOriginKeyword),
                                EnumVariant(reference = YulOrKeyword),
                                EnumVariant(reference = YulPopKeyword),
                                EnumVariant(reference = YulReturnKeyword),
                                EnumVariant(reference = YulRevertKeyword),
                                EnumVariant(reference = YulSDivKeyword),
                                EnumVariant(reference = YulSelfDestructKeyword),
                                EnumVariant(reference = YulSgtKeyword),
                                EnumVariant(reference = YulSignExtendKeyword),
                                EnumVariant(reference = YulSLoadKeyword),
                                EnumVariant(reference = YulSltKeyword),
                                EnumVariant(reference = YulSModKeyword),
                                EnumVariant(reference = YulSStoreKeyword),
                                EnumVariant(reference = YulStopKeyword),
                                EnumVariant(reference = YulSubKeyword),
                                EnumVariant(reference = YulTimestampKeyword),
                                EnumVariant(reference = YulXorKeyword),
                                EnumVariant(
                                    reference = YulKeccak256Keyword,
                                    enabled = From("0.4.12")
                                ),
                                EnumVariant(reference = YulSha3Keyword, enabled = Till("0.5.0")),
                                EnumVariant(reference = YulSuicideKeyword, enabled = Till("0.5.0")),
                                // 'Byzantium' hard-fork updates:
                                EnumVariant(
                                    reference = YulReturnDataCopyKeyword,
                                    enabled = From("0.4.12")
                                ),
                                EnumVariant(
                                    reference = YulReturnDataSizeKeyword,
                                    enabled = From("0.4.12")
                                ),
                                EnumVariant(
                                    reference = YulStaticCallKeyword,
                                    enabled = From("0.4.12")
                                ),
                                // 'Constantinople' hard-fork updates:
                                EnumVariant(
                                    reference = YulCreate2Keyword,
                                    enabled = From("0.4.12")
                                ),
                                EnumVariant(
                                    reference = YulExtCodeHashKeyword,
                                    enabled = From("0.5.0")
                                ),
                                EnumVariant(reference = YulSarKeyword),
                                EnumVariant(reference = YulShlKeyword),
                                EnumVariant(reference = YulShrKeyword),
                                // 'Instanbul' hard-fork updates:
                                EnumVariant(reference = YulChainIdKeyword),
                                EnumVariant(reference = YulSelfBalanceKeyword),
                                // 'London' hard-fork updates:
                                EnumVariant(reference = YulBaseFeeKeyword, enabled = From("0.8.7")),
                                // 'Paris' hard-fork updates:
                                EnumVariant(
                                    reference = YulDifficultyKeyword,
                                    enabled = Till("0.8.18")
                                ),
                                EnumVariant(
                                    reference = YulPrevRandaoKeyword,
                                    enabled = From("0.8.18")
                                )
                            ]
                        ),
                        Enum(
                            name = YulLiteral,
                            variants = [
                                EnumVariant(reference = YulTrueKeyword),
                                EnumVariant(reference = YulFalseKeyword),
                                EnumVariant(reference = YulDecimalLiteral),
                                EnumVariant(reference = YulHexLiteral),
                                EnumVariant(reference = HexStringLiteral),
                                EnumVariant(reference = AsciiStringLiteral)
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
                    title = "Yul Keywords",
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
                            name = YulAddKeyword,
                            identifier = YulIdentifier,
                            definitions = [KeywordDefinition(value = Atom("add"))]
                        ),
                        Keyword(
                            name = YulAddModKeyword,
                            identifier = YulIdentifier,
                            definitions = [KeywordDefinition(value = Atom("addmod"))]
                        ),
                        Keyword(
                            name = YulAddressKeyword,
                            identifier = YulIdentifier,
                            definitions = [KeywordDefinition(value = Atom("address"))]
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
                            name = YulAndKeyword,
                            identifier = YulIdentifier,
                            definitions = [KeywordDefinition(value = Atom("and"))]
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
                            name = YulBalanceKeyword,
                            identifier = YulIdentifier,
                            definitions = [KeywordDefinition(value = Atom("balance"))]
                        ),
                        Keyword(
                            name = YulBaseFeeKeyword,
                            identifier = YulIdentifier,
                            definitions = [KeywordDefinition(
                                enabled = From("0.8.7"),
                                reserved = From("0.8.7"),
                                value = Atom("basefee")
                            )]
                        ),
                        Keyword(
                            name = YulBlockHashKeyword,
                            identifier = YulIdentifier,
                            definitions = [KeywordDefinition(value = Atom("blockhash"))]
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
                            definitions = [KeywordDefinition(value = Atom("byte"))]
                        ),
                        Keyword(
                            name = YulBytesKeyword,
                            identifier = YulIdentifier,
                            definitions = [KeywordDefinition(
                                enabled = Never,
                                reserved = Till("0.7.1"),
                                value = Sequence([
                                    Atom("bytes"),
                                    Optional(Choice([
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
                                    ]))
                                ])
                            )]
                        ),
                        Keyword(
                            name = YulCallCodeKeyword,
                            identifier = YulIdentifier,
                            definitions = [KeywordDefinition(value = Atom("callcode"))]
                        ),
                        Keyword(
                            name = YulCallDataCopyKeyword,
                            identifier = YulIdentifier,
                            definitions = [KeywordDefinition(value = Atom("calldatacopy"))]
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
                            name = YulCallDataLoadKeyword,
                            identifier = YulIdentifier,
                            definitions = [KeywordDefinition(value = Atom("calldataload"))]
                        ),
                        Keyword(
                            name = YulCallDataSizeKeyword,
                            identifier = YulIdentifier,
                            definitions = [KeywordDefinition(value = Atom("calldatasize"))]
                        ),
                        Keyword(
                            name = YulCallerKeyword,
                            identifier = YulIdentifier,
                            definitions = [KeywordDefinition(value = Atom("caller"))]
                        ),
                        Keyword(
                            name = YulCallKeyword,
                            identifier = YulIdentifier,
                            definitions = [KeywordDefinition(value = Atom("call"))]
                        ),
                        Keyword(
                            name = YulCallValueKeyword,
                            identifier = YulIdentifier,
                            definitions = [KeywordDefinition(value = Atom("callvalue"))]
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
                            name = YulChainIdKeyword,
                            identifier = YulIdentifier,
                            definitions = [KeywordDefinition(
                                reserved = From("0.5.12"),
                                value = Atom("chainid")
                            )]
                        ),
                        Keyword(
                            name = YulCoinBaseKeyword,
                            identifier = YulIdentifier,
                            definitions = [KeywordDefinition(value = Atom("coinbase"))]
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
                            name = YulCreateKeyword,
                            identifier = YulIdentifier,
                            definitions = [KeywordDefinition(value = Atom("create"))]
                        ),
                        Keyword(
                            name = YulCreate2Keyword,
                            identifier = YulIdentifier,
                            definitions = [KeywordDefinition(
                                enabled = From("0.4.12"),
                                reserved = From("0.4.12"),
                                value = Atom("create2")
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
                            name = YulDelegateCallKeyword,
                            identifier = YulIdentifier,
                            definitions = [KeywordDefinition(value = Atom("delegatecall"))]
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
                            // Replaced by 'YulPrevRandaoKeyword' in 'London' hard-fork update:
                            name = YulDifficultyKeyword,
                            identifier = YulIdentifier,
                            definitions = [KeywordDefinition(
                                enabled = Till("0.8.18"),
                                value = Atom("difficulty")
                            )]
                        ),
                        Keyword(
                            name = YulDivKeyword,
                            identifier = YulIdentifier,
                            definitions = [KeywordDefinition(value = Atom("div"))]
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
                            name = YulEqKeyword,
                            identifier = YulIdentifier,
                            definitions = [KeywordDefinition(value = Atom("eq"))]
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
                            name = YulExpKeyword,
                            identifier = YulIdentifier,
                            definitions = [KeywordDefinition(value = Atom("exp"))]
                        ),
                        Keyword(
                            name = YulExtCodeCopyKeyword,
                            identifier = YulIdentifier,
                            definitions = [KeywordDefinition(value = Atom("extcodecopy"))]
                        ),
                        Keyword(
                            // NOTE: Only considered as part of 'Constantinople' target by 'solc' since '0.5.5':
                            name = YulExtCodeHashKeyword,
                            identifier = YulIdentifier,
                            definitions = [KeywordDefinition(
                                enabled = From("0.5.0"),
                                reserved = From("0.5.0"),
                                value = Atom("extcodehash")
                            )]
                        ),
                        Keyword(
                            name = YulExtCodeSizeKeyword,
                            identifier = YulIdentifier,
                            definitions = [KeywordDefinition(value = Atom("extcodesize"))]
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
                            name = YulGasKeyword,
                            identifier = YulIdentifier,
                            definitions = [KeywordDefinition(value = Atom("gas"))]
                        ),
                        Keyword(
                            name = YulGasLimitKeyword,
                            identifier = YulIdentifier,
                            definitions = [KeywordDefinition(value = Atom("gaslimit"))]
                        ),
                        Keyword(
                            name = YulGasPriceKeyword,
                            identifier = YulIdentifier,
                            definitions = [KeywordDefinition(value = Atom("gasprice"))]
                        ),
                        Keyword(
                            name = YulGtKeyword,
                            identifier = YulIdentifier,
                            definitions = [KeywordDefinition(value = Atom("gt"))]
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
                            name = YulInvalidKeyword,
                            identifier = YulIdentifier,
                            definitions = [KeywordDefinition(value = Atom("invalid"))]
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
                            name = YulIsZeroKeyword,
                            identifier = YulIdentifier,
                            definitions = [KeywordDefinition(value = Atom("iszero"))]
                        ),
                        Keyword(
                            name = YulKeccak256Keyword,
                            identifier = YulIdentifier,
                            definitions = [KeywordDefinition(
                                enabled = From("0.4.12"),
                                reserved = From("0.4.12"),
                                value = Atom("keccak256")
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
                            name = YulLog0Keyword,
                            identifier = YulIdentifier,
                            definitions = [KeywordDefinition(value = Atom("log0"))]
                        ),
                        Keyword(
                            name = YulLog1Keyword,
                            identifier = YulIdentifier,
                            definitions = [KeywordDefinition(value = Atom("log1"))]
                        ),
                        Keyword(
                            name = YulLog2Keyword,
                            identifier = YulIdentifier,
                            definitions = [KeywordDefinition(value = Atom("log2"))]
                        ),
                        Keyword(
                            name = YulLog3Keyword,
                            identifier = YulIdentifier,
                            definitions = [KeywordDefinition(value = Atom("log3"))]
                        ),
                        Keyword(
                            name = YulLog4Keyword,
                            identifier = YulIdentifier,
                            definitions = [KeywordDefinition(value = Atom("log4"))]
                        ),
                        Keyword(
                            name = YulLtKeyword,
                            identifier = YulIdentifier,
                            definitions = [KeywordDefinition(value = Atom("lt"))]
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
                            name = YulMLoadKeyword,
                            identifier = YulIdentifier,
                            definitions = [KeywordDefinition(value = Atom("mload"))]
                        ),
                        Keyword(
                            name = YulModKeyword,
                            identifier = YulIdentifier,
                            definitions = [KeywordDefinition(value = Atom("mod"))]
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
                            name = YulMSizeKeyword,
                            identifier = YulIdentifier,
                            definitions = [KeywordDefinition(value = Atom("msize"))]
                        ),
                        Keyword(
                            name = YulMStoreKeyword,
                            identifier = YulIdentifier,
                            definitions = [KeywordDefinition(value = Atom("mstore"))]
                        ),
                        Keyword(
                            name = YulMStore8Keyword,
                            identifier = YulIdentifier,
                            definitions = [KeywordDefinition(value = Atom("mstore8"))]
                        ),
                        Keyword(
                            name = YulMulKeyword,
                            identifier = YulIdentifier,
                            definitions = [KeywordDefinition(value = Atom("mul"))]
                        ),
                        Keyword(
                            name = YulMulModKeyword,
                            identifier = YulIdentifier,
                            definitions = [KeywordDefinition(value = Atom("mulmod"))]
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
                            name = YulNotKeyword,
                            identifier = YulIdentifier,
                            definitions = [KeywordDefinition(value = Atom("not"))]
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
                            name = YulNumberKeyword,
                            identifier = YulIdentifier,
                            definitions = [KeywordDefinition(value = Atom("number"))]
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
                            name = YulOrKeyword,
                            identifier = YulIdentifier,
                            definitions = [KeywordDefinition(value = Atom("or"))]
                        ),
                        Keyword(
                            name = YulOriginKeyword,
                            identifier = YulIdentifier,
                            definitions = [KeywordDefinition(value = Atom("origin"))]
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
                            name = YulPopKeyword,
                            identifier = YulIdentifier,
                            definitions = [KeywordDefinition(value = Atom("pop"))]
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
                            name = YulPrevRandaoKeyword,
                            identifier = YulIdentifier,
                            definitions = [KeywordDefinition(
                                enabled = From("0.8.18"),
                                reserved = From("0.8.18"),
                                value = Atom("prevrandao")
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
                            name = YulReturnDataCopyKeyword,
                            identifier = YulIdentifier,
                            definitions = [KeywordDefinition(
                                enabled = From("0.4.12"),
                                reserved = From("0.4.12"),
                                value = Atom("returndatacopy")
                            )]
                        ),
                        Keyword(
                            name = YulReturnDataSizeKeyword,
                            identifier = YulIdentifier,
                            definitions = [KeywordDefinition(
                                enabled = From("0.4.12"),
                                reserved = From("0.4.12"),
                                value = Atom("returndatasize")
                            )]
                        ),
                        Keyword(
                            name = YulReturnKeyword,
                            identifier = YulIdentifier,
                            definitions = [KeywordDefinition(value = Atom("return"))]
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
                            definitions = [KeywordDefinition(value = Atom("revert"))]
                        ),
                        Keyword(
                            name = YulSarKeyword,
                            identifier = YulIdentifier,
                            definitions = [KeywordDefinition(
                                reserved = From("0.4.21"),
                                value = Atom("sar")
                            )]
                        ),
                        Keyword(
                            name = YulSDivKeyword,
                            identifier = YulIdentifier,
                            definitions = [KeywordDefinition(value = Atom("sdiv"))]
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
                            name = YulSelfBalanceKeyword,
                            identifier = YulIdentifier,
                            definitions = [KeywordDefinition(
                                reserved = From("0.5.12"),
                                value = Atom("selfbalance")
                            )]
                        ),
                        Keyword(
                            name = YulSelfDestructKeyword,
                            identifier = YulIdentifier,
                            definitions = [KeywordDefinition(value = Atom("selfdestruct"))]
                        ),
                        Keyword(
                            name = YulSgtKeyword,
                            identifier = YulIdentifier,
                            definitions = [KeywordDefinition(value = Atom("sgt"))]
                        ),
                        Keyword(
                            // Replaced by 'YulKeccak256Keyword' in '0.4.12', and removed in '0.5.0':
                            name = YulSha3Keyword,
                            identifier = YulIdentifier,
                            definitions = [KeywordDefinition(
                                enabled = Till("0.5.0"),
                                reserved = Till("0.5.0"),
                                value = Atom("sha3")
                            )]
                        ),
                        Keyword(
                            name = YulShlKeyword,
                            identifier = YulIdentifier,
                            definitions = [KeywordDefinition(
                                reserved = From("0.4.21"),
                                value = Atom("shl")
                            )]
                        ),
                        Keyword(
                            name = YulShrKeyword,
                            identifier = YulIdentifier,
                            definitions = [KeywordDefinition(
                                reserved = From("0.4.21"),
                                value = Atom("shr")
                            )]
                        ),
                        Keyword(
                            name = YulSignExtendKeyword,
                            identifier = YulIdentifier,
                            definitions = [KeywordDefinition(value = Atom("signextend"))]
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
                            name = YulSLoadKeyword,
                            identifier = YulIdentifier,
                            definitions = [KeywordDefinition(value = Atom("sload"))]
                        ),
                        Keyword(
                            name = YulSltKeyword,
                            identifier = YulIdentifier,
                            definitions = [KeywordDefinition(value = Atom("slt"))]
                        ),
                        Keyword(
                            name = YulSModKeyword,
                            identifier = YulIdentifier,
                            definitions = [KeywordDefinition(value = Atom("smod"))]
                        ),
                        Keyword(
                            name = YulSStoreKeyword,
                            identifier = YulIdentifier,
                            definitions = [KeywordDefinition(value = Atom("sstore"))]
                        ),
                        Keyword(
                            name = YulStaticCallKeyword,
                            identifier = YulIdentifier,
                            definitions = [KeywordDefinition(
                                enabled = From("0.4.12"),
                                reserved = From("0.4.12"),
                                value = Atom("staticcall")
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
                            name = YulStopKeyword,
                            identifier = YulIdentifier,
                            definitions = [KeywordDefinition(value = Atom("stop"))]
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
                            name = YulSubKeyword,
                            identifier = YulIdentifier,
                            definitions = [KeywordDefinition(value = Atom("sub"))]
                        ),
                        Keyword(
                            // Introduced as alias to 'YulSelfDestructKeyword' in '0.2.0', and removed in '0.5.0':
                            name = YulSuicideKeyword,
                            identifier = YulIdentifier,
                            definitions = [KeywordDefinition(
                                enabled = Till("0.5.0"),
                                reserved = Till("0.5.0"),
                                value = Atom("suicide")
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
                            name = YulTimestampKeyword,
                            identifier = YulIdentifier,
                            definitions = [KeywordDefinition(value = Atom("timestamp"))]
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
                        ),
                        Keyword(
                            name = YulXorKeyword,
                            identifier = YulIdentifier,
                            definitions = [KeywordDefinition(value = Atom("xor"))]
                        )
                    ]
                )
            ]
        )
    ]
));
