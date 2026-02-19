pub use solidity::SolidityDefinition;

language_macros::compile!(Language(
    name = Solidity,
    binding_rules_file = "crates/solidity/inputs/language/bindings/rules.msgb",
    root_item = SourceUnit,
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
        "0.8.21", "0.8.22", "0.8.23", "0.8.24", "0.8.25", "0.8.26", "0.8.27", "0.8.28", "0.8.29",
        "0.8.30", "0.8.31", "0.8.32", "0.8.33", "0.8.34"
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
                            fields = (members = Required(SourceUnitMembers))
                        ),
                        Repeated(
                            name = SourceUnitMembers,
                            reference = SourceUnitMember,
                            allow_empty = true
                        ),
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
                                EnumVariant(reference = ErrorDefinition, enabled = From("0.8.4")),
                                EnumVariant(
                                    reference = UserDefinedValueTypeDefinition,
                                    enabled = From("0.8.8")
                                ),
                                EnumVariant(reference = UsingDirective, enabled = From("0.8.13")),
                                EnumVariant(reference = EventDefinition, enabled = From("0.8.22")),
                                EnumVariant(
                                    reference = ConstantDefinition,
                                    enabled = From("0.7.4")
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
                                pragma_keyword = Required(PragmaKeyword),
                                pragma = Required(Pragma),
                                semicolon = Required(Semicolon)
                            )
                        ),
                        Enum(
                            name = Pragma,
                            variants = [
                                EnumVariant(reference = VersionPragma),
                                EnumVariant(reference = AbicoderPragma, enabled = From("0.7.5")),
                                EnumVariant(
                                    reference = ExperimentalPragma,
                                    enabled = From("0.4.16")
                                )
                            ]
                        ),
                        Struct(
                            name = AbicoderPragma,
                            enabled = From("0.7.5"),
                            fields = (
                                abicoder_keyword = Required(AbicoderKeyword),
                                version = Required(AbicoderVersion)
                            )
                        ),
                        Struct(
                            name = ExperimentalPragma,
                            enabled = From("0.4.16"),
                            fields = (
                                experimental_keyword = Required(ExperimentalKeyword),
                                feature = Required(ExperimentalFeature)
                            )
                        ),
                        Enum(
                            name = AbicoderVersion,
                            enabled = From("0.7.5"),
                            variants = [
                                EnumVariant(reference = AbicoderV1Keyword),
                                EnumVariant(reference = AbicoderV2Keyword)
                            ]
                        ),
                        Enum(
                            name = ExperimentalFeature,
                            enabled = From("0.4.16"),
                            variants = [
                                EnumVariant(reference = ABIEncoderV2Keyword),
                                EnumVariant(reference = SMTCheckerKeyword),
                                EnumVariant(reference = StringLiteral)
                            ]
                        ),
                        Struct(
                            name = VersionPragma,
                            fields = (
                                solidity_keyword = Required(SolidityKeyword),
                                sets = Required(VersionExpressionSets)
                            )
                        ),
                        Separated(
                            name = VersionExpressionSets,
                            reference = VersionExpressionSet,
                            separator = BarBar
                        ),
                        Repeated(name = VersionExpressionSet, reference = VersionExpression),
                        Enum(
                            name = VersionExpression,
                            variants = [
                                EnumVariant(reference = VersionRange),
                                EnumVariant(reference = VersionTerm)
                            ]
                        ),
                        Struct(
                            name = VersionRange,
                            fields = (
                                start = Required(VersionLiteral),
                                minus = Required(Minus),
                                end = Required(VersionLiteral)
                            )
                        ),
                        Struct(
                            name = VersionTerm,
                            fields = (
                                operator = Optional(reference = VersionOperator),
                                literal = Required(VersionLiteral)
                            )
                        ),
                        Enum(
                            name = VersionOperator,
                            variants = [
                                EnumVariant(reference = Caret),
                                EnumVariant(reference = Tilde),
                                EnumVariant(reference = Equal),
                                EnumVariant(reference = LessThan),
                                EnumVariant(reference = GreaterThan),
                                EnumVariant(reference = LessThanEqual),
                                EnumVariant(reference = GreaterThanEqual)
                            ]
                        ),
                        Enum(
                            name = VersionLiteral,
                            variants = [
                                EnumVariant(reference = SimpleVersionLiteral),
                                EnumVariant(reference = SingleQuotedVersionLiteral),
                                EnumVariant(reference = DoubleQuotedVersionLiteral)
                            ]
                        ),
                        Separated(
                            // __SLANG_VERSION_SPECIFIER_SYNTAX__ (keep in sync)
                            name = SimpleVersionLiteral,
                            reference = VersionSpecifier,
                            separator = Period
                        ),
                        Token(
                            // __SLANG_VERSION_SPECIFIER_SYNTAX__ (keep in sync)
                            name = VersionSpecifier,
                            definitions = [TokenDefinition(
                                scanner = Fragment(VersionSpecifierFragment)
                            )]
                        ),
                        Token(
                            // __SLANG_VERSION_SPECIFIER_SYNTAX__ (keep in sync)
                            name = SingleQuotedVersionLiteral,
                            definitions = [TokenDefinition(
                                scanner = Sequence([
                                    Atom("'"),
                                    Fragment(VersionSpecifierFragment),
                                    ZeroOrMore(Sequence([
                                        Atom("."),
                                        Fragment(VersionSpecifierFragment)
                                    ])),
                                    Atom("'")
                                ])
                            )]
                        ),
                        Token(
                            // __SLANG_VERSION_SPECIFIER_SYNTAX__ (keep in sync)
                            name = DoubleQuotedVersionLiteral,
                            definitions = [TokenDefinition(
                                scanner = Sequence([
                                    Atom("\""),
                                    Fragment(VersionSpecifierFragment),
                                    ZeroOrMore(Sequence([
                                        Atom("."),
                                        Fragment(VersionSpecifierFragment)
                                    ])),
                                    Atom("\"")
                                ])
                            )]
                        ),
                        Fragment(
                            name = VersionSpecifierFragment,
                            scanner = OneOrMore(Choice([
                                Range(inclusive_start = '0', inclusive_end = '9'),
                                Atom("x"),
                                Atom("X"),
                                Atom("*")
                            ]))
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
                                path = Required(StringLiteral),
                                alias = Optional(reference = ImportAlias)
                            )
                        ),
                        Struct(
                            name = NamedImport,
                            fields = (
                                asterisk = Required(Asterisk),
                                alias = Required(ImportAlias),
                                from_keyword = Required(FromKeyword),
                                path = Required(StringLiteral)
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
                                path = Required(StringLiteral)
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
                            scanner =
                                Choice([Atom("\n"), Sequence([Atom("\r"), Optional(Atom("\n"))])])
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
                                TrailingContext(
                                    scanner = Atom("/*"),
                                    not_followed_by = Sequence([Atom("*"), Not(['/'])])
                                ),
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
                                TrailingContext(scanner = Atom("/**"), not_followed_by = Atom("/")),
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
                Topic(
                    title = "Keywords",
                    items = [
                        Keyword(
                            name = AbicoderKeyword,
                            identifier = Identifier,
                            definitions = [KeywordDefinition(
                                enabled = From("0.7.5"),
                                reserved = Never,
                                value = Atom("abicoder")
                            )]
                        ),
                        Keyword(
                            name = AbicoderV1Keyword,
                            identifier = Identifier,
                            definitions = [KeywordDefinition(
                                enabled = From("0.7.5"),
                                reserved = Never,
                                value = Atom("v1")
                            )]
                        ),
                        Keyword(
                            name = AbicoderV2Keyword,
                            identifier = Identifier,
                            definitions = [KeywordDefinition(
                                enabled = From("0.7.5"),
                                reserved = Never,
                                value = Atom("v2")
                            )]
                        ),
                        Keyword(
                            name = ABIEncoderV2Keyword,
                            identifier = Identifier,
                            definitions = [KeywordDefinition(
                                enabled = From("0.4.16"),
                                reserved = Never,
                                value = Atom("ABIEncoderV2")
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
                            definitions =
                                [KeywordDefinition(reserved = Never, value = Atom("address"))]
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
                            name = AtKeyword,
                            identifier = Identifier,
                            definitions = [KeywordDefinition(
                                enabled = From("0.8.29"),
                                reserved = Never,
                                value = Atom("at")
                            )]
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
                            name = ExperimentalKeyword,
                            identifier = Identifier,
                            definitions = [KeywordDefinition(
                                enabled = From("0.4.16"),
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
                            name = LayoutKeyword,
                            identifier = Identifier,
                            definitions = [KeywordDefinition(
                                enabled = From("0.8.29"),
                                reserved = Never,
                                value = Atom("layout")
                            )]
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
                                enabled = From("0.6.0"),
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
                            definitions = [KeywordDefinition(
                                enabled = From("0.4.16"),
                                value = Atom("pure")
                            )]
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
                            name = SMTCheckerKeyword,
                            identifier = Identifier,
                            definitions = [KeywordDefinition(
                                enabled = From("0.4.16"),
                                reserved = Never,
                                value = Atom("SMTChecker")
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
                            name = SuperKeyword,
                            identifier = Identifier,
                            definitions = [KeywordDefinition(
                                reserved = From("0.8.0"),
                                value = Atom("super")
                            )]
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
                            name = ThisKeyword,
                            identifier = Identifier,
                            definitions = [KeywordDefinition(
                                reserved = From("0.8.0"),
                                value = Atom("this")
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
                            name = TransientKeyword,
                            identifier = Identifier,
                            definitions = [KeywordDefinition(
                                enabled = From("0.8.27"),
                                reserved = Never,
                                value = Atom("transient")
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
                            definitions = [KeywordDefinition(
                                enabled = From("0.4.16"),
                                value = Atom("view")
                            )]
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
                            name = EqualColon,
                            definitions = [TokenDefinition(
                                enabled = Till("0.5.0"),
                                scanner = Atom("=:")
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
                                specifiers = Required(ContractSpecifiers),
                                open_brace = Required(OpenBrace),
                                members = Required(ContractMembers),
                                close_brace = Required(CloseBrace)
                            )
                        ),
                        Repeated(
                            name = ContractSpecifiers,
                            reference = ContractSpecifier,
                            allow_empty = true
                        ),
                        Enum(
                            name = ContractSpecifier,
                            variants = [
                                EnumVariant(reference = InheritanceSpecifier),
                                EnumVariant(
                                    reference = StorageLayoutSpecifier,
                                    enabled = From("0.8.29")
                                )
                            ]
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
                        Struct(
                            name = StorageLayoutSpecifier,
                            enabled = From("0.8.29"),
                            fields = (
                                layout_keyword = Required(LayoutKeyword),
                                at_keyword = Required(AtKeyword),
                                expression = Required(Expression)
                            )
                        ),
                        Repeated(
                            name = ContractMembers,
                            reference = ContractMember,
                            allow_empty = true
                        ),
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
                                EnumVariant(reference = ErrorDefinition, enabled = From("0.8.4")),
                                EnumVariant(
                                    reference = UserDefinedValueTypeDefinition,
                                    enabled = From("0.8.8")
                                ),
                                EnumVariant(reference = StateVariableDefinition)
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
                                inheritance = Optional(reference = InheritanceSpecifier),
                                open_brace = Required(OpenBrace),
                                members = Required(InterfaceMembers),
                                close_brace = Required(CloseBrace)
                            )
                        ),
                        Repeated(
                            name = InterfaceMembers,
                            reference = ContractMember,
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
                                library_keyword = Required(LibraryKeyword),
                                name = Required(Identifier),
                                open_brace = Required(OpenBrace),
                                members = Required(LibraryMembers),
                                close_brace = Required(CloseBrace)
                            )
                        ),
                        Repeated(
                            name = LibraryMembers,
                            reference = ContractMember,
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
                                struct_keyword = Required(StructKeyword),
                                name = Required(Identifier),
                                open_brace = Required(OpenBrace),
                                members = Required(StructMembers),
                                close_brace = Required(CloseBrace)
                            )
                        ),
                        Repeated(
                            name = StructMembers,
                            reference = StructMember,
                            allow_empty = true
                        ),
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
                                members = Required(EnumMembers),
                                close_brace = Required(CloseBrace)
                            )
                        ),
                        Separated(
                            name = EnumMembers,
                            reference = Identifier,
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
                                attributes = Required(StateVariableAttributes),
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
                            reference = StateVariableAttribute,
                            allow_empty = true
                        ),
                        Enum(
                            name = StateVariableAttribute,
                            variants = [
                                EnumVariant(reference = OverrideSpecifier, enabled = From("0.6.0")),
                                EnumVariant(reference = ConstantKeyword),
                                EnumVariant(reference = InternalKeyword),
                                EnumVariant(reference = PrivateKeyword),
                                EnumVariant(reference = PublicKeyword),
                                EnumVariant(reference = ImmutableKeyword, enabled = From("0.6.5")),
                                EnumVariant(reference = TransientKeyword, enabled = From("0.8.27"))
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
                                attributes = Required(FunctionAttributes),
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
                                parameters = Required(Parameters),
                                close_paren = Required(CloseParen)
                            )
                        ),
                        Separated(
                            name = Parameters,
                            reference = Parameter,
                            separator = Comma,
                            allow_empty = true
                        ),
                        Struct(
                            name = Parameter,
                            fields = (
                                type_name = Required(TypeName),
                                storage_location = Optional(reference = StorageLocation),
                                name = Optional(reference = Identifier)
                            )
                        ),
                        Repeated(
                            name = FunctionAttributes,
                            reference = FunctionAttribute,
                            allow_empty = true
                        ),
                        Enum(
                            name = FunctionAttribute,
                            variants = [
                                EnumVariant(reference = ModifierInvocation),
                                EnumVariant(reference = OverrideSpecifier, enabled = From("0.6.0")),
                                EnumVariant(reference = ConstantKeyword, enabled = Till("0.5.0")),
                                EnumVariant(reference = ExternalKeyword),
                                EnumVariant(reference = InternalKeyword),
                                EnumVariant(reference = PayableKeyword),
                                EnumVariant(reference = PrivateKeyword),
                                EnumVariant(reference = PublicKeyword),
                                EnumVariant(reference = PureKeyword, enabled = From("0.4.16")),
                                EnumVariant(reference = ViewKeyword, enabled = From("0.4.16")),
                                EnumVariant(reference = VirtualKeyword, enabled = From("0.6.0"))
                            ]
                        ),
                        Struct(
                            name = OverrideSpecifier,
                            enabled = From("0.6.0"),
                            fields = (
                                override_keyword = Required(OverrideKeyword),
                                overridden = Optional(reference = OverridePathsDeclaration)
                            )
                        ),
                        Struct(
                            name = OverridePathsDeclaration,
                            enabled = From("0.6.0"),
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
                            separator = Comma,
                            enabled = From("0.6.0")
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
                                attributes = Required(ConstructorAttributes),
                                body = Required(Block)
                            )
                        ),
                        Repeated(
                            name = ConstructorAttributes,
                            reference = ConstructorAttribute,
                            enabled = From("0.4.22"),
                            allow_empty = true
                        ),
                        Enum(
                            name = ConstructorAttribute,
                            enabled = From("0.4.22"),
                            variants = [
                                EnumVariant(reference = ModifierInvocation),
                                EnumVariant(reference = InternalKeyword),
                                EnumVariant(
                                    reference = OverrideKeyword,
                                    enabled = Range(from = "0.6.0", till = "0.6.7")
                                ),
                                EnumVariant(reference = PayableKeyword),
                                EnumVariant(reference = PublicKeyword),
                                EnumVariant(
                                    reference = VirtualKeyword,
                                    enabled = Range(from = "0.6.0", till = "0.6.7")
                                )
                            ]
                        ),
                        Struct(
                            name = UnnamedFunctionDefinition,
                            enabled = Till("0.6.0"),
                            fields = (
                                function_keyword = Required(FunctionKeyword),
                                parameters = Required(ParametersDeclaration),
                                attributes = Required(UnnamedFunctionAttributes),
                                body = Required(FunctionBody)
                            )
                        ),
                        Repeated(
                            name = UnnamedFunctionAttributes,
                            reference = UnnamedFunctionAttribute,
                            enabled = Till("0.6.0"),
                            allow_empty = true
                        ),
                        Enum(
                            name = UnnamedFunctionAttribute,
                            enabled = Till("0.6.0"),
                            variants = [
                                EnumVariant(reference = ModifierInvocation),
                                EnumVariant(reference = ConstantKeyword, enabled = Till("0.5.0")),
                                EnumVariant(reference = ExternalKeyword),
                                EnumVariant(reference = InternalKeyword, enabled = Till("0.5.0")),
                                EnumVariant(reference = PayableKeyword),
                                EnumVariant(reference = PrivateKeyword, enabled = Till("0.5.0")),
                                EnumVariant(reference = PublicKeyword, enabled = Till("0.5.0")),
                                EnumVariant(
                                    reference = PureKeyword,
                                    enabled = Range(from = "0.4.16", till = "0.6.0")
                                ),
                                EnumVariant(
                                    reference = ViewKeyword,
                                    enabled = Range(from = "0.4.16", till = "0.6.0")
                                )
                            ]
                        ),
                        Struct(
                            name = FallbackFunctionDefinition,
                            enabled = From("0.6.0"),
                            fields = (
                                fallback_keyword = Required(FallbackKeyword),
                                parameters = Required(ParametersDeclaration),
                                attributes = Required(FallbackFunctionAttributes),
                                returns = Optional(reference = ReturnsDeclaration),
                                body = Required(FunctionBody)
                            )
                        ),
                        Repeated(
                            name = FallbackFunctionAttributes,
                            reference = FallbackFunctionAttribute,
                            enabled = From("0.6.0"),
                            allow_empty = true
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
                                attributes = Required(ReceiveFunctionAttributes),
                                body = Required(FunctionBody)
                            )
                        ),
                        Repeated(
                            name = ReceiveFunctionAttributes,
                            reference = ReceiveFunctionAttribute,
                            enabled = From("0.6.0"),
                            allow_empty = true
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
                                attributes = Required(ModifierAttributes),
                                body = Required(FunctionBody)
                            )
                        ),
                        Repeated(
                            name = ModifierAttributes,
                            reference = ModifierAttribute,
                            allow_empty = true
                        ),
                        Enum(
                            name = ModifierAttribute,
                            variants = [
                                EnumVariant(reference = OverrideSpecifier, enabled = From("0.6.0")),
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
                                parameters = Required(EventParameters),
                                close_paren = Required(CloseParen)
                            )
                        ),
                        Separated(
                            name = EventParameters,
                            reference = EventParameter,
                            separator = Comma,
                            allow_empty = true
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
                                parameters = Required(ErrorParameters),
                                close_paren = Required(CloseParen)
                            )
                        ),
                        Separated(
                            name = ErrorParameters,
                            reference = ErrorParameter,
                            separator = Comma,
                            enabled = From("0.8.4"),
                            allow_empty = true
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
                                attributes = Required(FunctionTypeAttributes),
                                returns = Optional(reference = ReturnsDeclaration)
                            )
                        ),
                        Repeated(
                            name = FunctionTypeAttributes,
                            reference = FunctionTypeAttribute,
                            allow_empty = true
                        ),
                        Enum(
                            name = FunctionTypeAttribute,
                            variants = [
                                EnumVariant(reference = InternalKeyword),
                                EnumVariant(reference = ExternalKeyword),
                                EnumVariant(reference = PrivateKeyword),
                                EnumVariant(reference = PublicKeyword),
                                EnumVariant(reference = ConstantKeyword, enabled = Till("0.5.0")),
                                EnumVariant(reference = PureKeyword, enabled = From("0.4.16")),
                                EnumVariant(reference = ViewKeyword, enabled = From("0.4.16")),
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
                                payable_keyword =
                                    Optional(reference = PayableKeyword, enabled = From("0.5.0"))
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
                                statements = Required(Statements),
                                close_brace = Required(CloseBrace)
                            )
                        ),
                        Repeated(name = Statements, reference = Statement, allow_empty = true),
                        Enum(
                            name = Statement,
                            variants = [
                                EnumVariant(reference = IfStatement),
                                EnumVariant(reference = ForStatement),
                                EnumVariant(reference = WhileStatement),
                                EnumVariant(reference = DoWhileStatement),
                                EnumVariant(reference = ContinueStatement),
                                EnumVariant(reference = BreakStatement),
                                EnumVariant(reference = ReturnStatement),
                                EnumVariant(reference = ThrowStatement, enabled = Till("0.5.0")),
                                EnumVariant(reference = EmitStatement, enabled = From("0.4.21")),
                                EnumVariant(reference = TryStatement, enabled = From("0.6.0")),
                                EnumVariant(reference = RevertStatement, enabled = From("0.8.4")),
                                EnumVariant(reference = AssemblyStatement),
                                EnumVariant(reference = Block),
                                EnumVariant(reference = UncheckedBlock, enabled = From("0.8.0")),
                                EnumVariant(reference = TupleDeconstructionStatement),
                                EnumVariant(reference = VariableDeclarationStatement),
                                EnumVariant(reference = ExpressionStatement)
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
                                label = Optional(reference = StringLiteral),
                                flags = Optional(
                                    reference = AssemblyFlagsDeclaration,
                                    enabled = From("0.8.13")
                                ),
                                body = Required(YulBlock)
                            )
                        ),
                        Struct(
                            name = AssemblyFlagsDeclaration,
                            enabled = From("0.8.13"),
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
                            reference = StringLiteral,
                            separator = Comma,
                            enabled = From("0.8.13")
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
                                var_keyword =
                                    Optional(reference = VarKeyword, enabled = Till("0.5.0")),
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
                                EnumVariant(reference = TupleDeconstructionStatement),
                                EnumVariant(reference = VariableDeclarationStatement),
                                EnumVariant(reference = ExpressionStatement),
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
                                error = Required(IdentifierPath),
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
                                    name = InequalityExpression,
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
                                        // Before '0.8.0', it was left-associative:
                                        PrecedenceOperator(
                                            model = BinaryLeftAssociative,
                                            enabled = Till("0.8.0"),
                                            fields = (operator = Required(AsteriskAsterisk))
                                        ),
                                        // In '0.8.0', it became right-associative:
                                        PrecedenceOperator(
                                            model = BinaryRightAssociative,
                                            enabled = From("0.8.0"),
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
                                        ),
                                        PrecedenceOperator(
                                            model = Prefix,
                                            fields = (operator = Required(DeleteKeyword))
                                        )
                                    ]
                                ),
                                PrecedenceExpression(
                                    name = FunctionCallExpression,
                                    operators = [PrecedenceOperator(
                                        model = Postfix,
                                        fields = (arguments = Required(ArgumentsDeclaration))
                                    )]
                                ),
                                PrecedenceExpression(
                                    name = CallOptionsExpression,
                                    operators = [PrecedenceOperator(
                                        model = Postfix,
                                        enabled = From("0.6.2"),
                                        error_recovery = FieldsErrorRecovery(
                                            delimiters = FieldDelimiters(
                                                open = open_brace,
                                                close = close_brace,
                                                // NOTE: Despite `CallOptions` requiring at least one element, we should
                                                // only recover if we found at least two tokens ('Identifier' + 'Colon')
                                                // between the braces, as  otherwise, this may be ambiguous when followed
                                                // by an empty 'Block' node, for example, in a 'TryStatement':
                                                //
                                                //     try <EXPR> {
                                                //         /* not call options  */
                                                //     } catch {
                                                //     }
                                                //
                                                // Or in 'ContractDefinition' that has a 'StorageLayoutSpecifier':
                                                //
                                                //     contract Foo layout at <EXPR> {
                                                //         /* not call options  */
                                                //     }
                                                //
                                                terminals_matched_acceptance_threshold = 2
                                            )
                                        ),
                                        fields = (
                                            open_brace = Required(OpenBrace),
                                            options = Required(CallOptions),
                                            close_brace = Required(CloseBrace)
                                        )
                                    )]
                                ),
                                PrecedenceExpression(
                                    name = MemberAccessExpression,
                                    operators = [PrecedenceOperator(
                                        model = Postfix,
                                        fields = (
                                            period = Required(Period),
                                            member = Required(Identifier)
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
                                PrimaryExpression(reference = ThisKeyword),
                                PrimaryExpression(reference = SuperKeyword),
                                PrimaryExpression(reference = TrueKeyword),
                                PrimaryExpression(reference = FalseKeyword),
                                PrimaryExpression(reference = Identifier)
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
                                arguments = Required(PositionalArguments),
                                close_paren = Required(CloseParen)
                            )
                        ),
                        Separated(
                            name = PositionalArguments,
                            reference = Expression,
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
                                open_paren = Required(OpenParen),
                                arguments = Optional(reference = NamedArgumentGroup),
                                close_paren = Required(CloseParen)
                            )
                        ),
                        Struct(
                            name = NamedArgumentGroup,
                            error_recovery = FieldsErrorRecovery(
                                delimiters =
                                    FieldDelimiters(open = open_brace, close = close_brace)
                            ),
                            fields = (
                                open_brace = Required(OpenBrace),
                                arguments = Required(NamedArguments),
                                close_brace = Required(CloseBrace)
                            )
                        ),
                        Separated(
                            name = NamedArguments,
                            reference = NamedArgument,
                            separator = Comma,
                            allow_empty = true
                        ),
                        Separated(
                            name = CallOptions,
                            reference = NamedArgument,
                            separator = Comma,
                            enabled = From("0.6.2"),
                            // These cannot be empty as they're ambiguous with `try <EXPR> {} catch {}`
                            allow_empty = false
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
                                // A bare integer (without a dot or a fraction) is enabled in all versions:
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
                                // Till 0.5.0, the following lone dot was considered a part of the literal:
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
                                // As well as the full form of digits followed by a dot followed by digits...
                                TokenDefinition(
                                    enabled = Till("0.5.0"),
                                    scanner = TrailingContext(
                                        scanner = Sequence([
                                            Fragment(DecimalDigits),
                                            Atom("."),
                                            Fragment(DecimalDigits),
                                            Optional(Fragment(DecimalExponent))
                                        ]),
                                        not_followed_by = Fragment(IdentifierStart)
                                    )
                                ),
                                // ...both of which was subsumed by a more general form that only included
                                // the dot if it was followed by a fraction:
                                TokenDefinition(
                                    enabled = From("0.5.0"),
                                    scanner = TrailingContext(
                                        scanner = Sequence([
                                            Fragment(DecimalDigits),
                                            Optional(Sequence([
                                                Atom("."),
                                                Fragment(DecimalDigits)
                                            ])),
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
                                EnumVariant(reference = StringLiteral, enabled = Till("0.5.14")),
                                EnumVariant(reference = StringLiterals, enabled = From("0.5.14")),
                                EnumVariant(reference = HexStringLiteral, enabled = Till("0.5.14")),
                                EnumVariant(
                                    reference = HexStringLiterals,
                                    enabled = From("0.5.14")
                                ),
                                EnumVariant(
                                    reference = UnicodeStringLiterals,
                                    enabled = From("0.7.0")
                                )
                            ]
                        ),
                        Repeated(
                            name = StringLiterals,
                            reference = StringLiteral,
                            enabled = From("0.5.14")
                        ),
                        Enum(
                            name = StringLiteral,
                            variants = [
                                EnumVariant(reference = SingleQuotedStringLiteral),
                                EnumVariant(reference = DoubleQuotedStringLiteral)
                            ]
                        ),
                        Token(
                            name = SingleQuotedStringLiteral,
                            definitions = [
                                // Allows unicode characters and arbitrary escape sequences:
                                TokenDefinition(
                                    enabled = Till("0.4.25"),
                                    scanner = Sequence([
                                        Atom("'"),
                                        ZeroOrMore(Choice([
                                            Fragment(EscapeSequenceArbitrary),
                                            Not(['\'', '\\', '\r', '\n'])
                                        ])),
                                        Atom("'")
                                    ])
                                ),
                                // Allows unicode characters but allows only known ASCII escape sequences:
                                TokenDefinition(
                                    enabled = Range(from = "0.4.25", till = "0.7.0"),
                                    scanner = Sequence([
                                        Atom("'"),
                                        ZeroOrMore(Choice([
                                            Fragment(EscapeSequence),
                                            Not(['\'', '\\', '\r', '\n'])
                                        ])),
                                        Atom("'")
                                    ])
                                ),
                                // Rejects unicode characters:
                                TokenDefinition(
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
                                )
                            ]
                        ),
                        Token(
                            name = DoubleQuotedStringLiteral,
                            definitions = [
                                // Allows unicode characters and arbitrary escape sequences:
                                TokenDefinition(
                                    enabled = Till("0.4.25"),
                                    scanner = Sequence([
                                        Atom("\""),
                                        ZeroOrMore(Choice([
                                            Fragment(EscapeSequenceArbitrary),
                                            Not(['"', '\\', '\r', '\n'])
                                        ])),
                                        Atom("\"")
                                    ])
                                ),
                                // Allows unicode characters but allows only known ASCII escape sequences:
                                TokenDefinition(
                                    enabled = Range(from = "0.4.25", till = "0.7.0"),
                                    scanner = Sequence([
                                        Atom("\""),
                                        ZeroOrMore(Choice([
                                            Fragment(EscapeSequence),
                                            Not(['"', '\\', '\r', '\n'])
                                        ])),
                                        Atom("\"")
                                    ])
                                ),
                                // Rejects unicode characters:
                                TokenDefinition(
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
                                )
                            ]
                        ),
                        Repeated(
                            name = HexStringLiterals,
                            reference = HexStringLiteral,
                            enabled = From("0.5.14")
                        ),
                        Enum(
                            name = HexStringLiteral,
                            variants = [
                                EnumVariant(reference = SingleQuotedHexStringLiteral),
                                EnumVariant(reference = DoubleQuotedHexStringLiteral)
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
                        Repeated(
                            name = UnicodeStringLiterals,
                            reference = UnicodeStringLiteral,
                            enabled = From("0.7.0")
                        ),
                        Enum(
                            name = UnicodeStringLiteral,
                            enabled = From("0.7.0"),
                            variants = [
                                EnumVariant(reference = SingleQuotedUnicodeStringLiteral),
                                EnumVariant(reference = DoubleQuotedUnicodeStringLiteral)
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
                            name = EscapeSequenceArbitrary,
                            enabled = Till("0.4.25"),
                            scanner = Sequence([
                                Atom("\\"),
                                Choice([
                                    // Prior to 0.4.25, it was legal to "escape" any character (incl. unicode),
                                    // however only the ones from `AsciiEscape` were escaped in practice.
                                    Not(['x', 'u']),
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
                                Atom("\r\n"),
                                Atom("\r"),
                                Atom("\n")
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
                            definitions = [TokenDefinition(
                                scanner = Sequence([
                                    Fragment(IdentifierStart),
                                    ZeroOrMore(Fragment(IdentifierPart))
                                ])
                            )]
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
                                statements = Required(YulStatements),
                                close_brace = Required(CloseBrace)
                            )
                        ),
                        Repeated(
                            name = YulStatements,
                            reference = YulStatement,
                            allow_empty = true
                        ),
                        Enum(
                            name = YulStatement,
                            variants = [
                                EnumVariant(reference = YulBlock),
                                EnumVariant(reference = YulFunctionDefinition),
                                EnumVariant(
                                    reference = YulStackAssignmentStatement,
                                    enabled = Till("0.5.0")
                                ),
                                EnumVariant(reference = YulIfStatement),
                                EnumVariant(reference = YulForStatement),
                                EnumVariant(reference = YulSwitchStatement),
                                EnumVariant(reference = YulLeaveStatement, enabled = From("0.6.0")),
                                EnumVariant(reference = YulBreakStatement),
                                EnumVariant(reference = YulContinueStatement),
                                EnumVariant(reference = YulVariableAssignmentStatement),
                                EnumVariant(reference = YulLabel, enabled = Till("0.5.0")),
                                EnumVariant(reference = YulVariableDeclarationStatement),
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
                                parameters = Required(YulParameters),
                                close_paren = Required(CloseParen)
                            )
                        ),
                        Separated(
                            name = YulParameters,
                            reference = YulIdentifier,
                            separator = Comma,
                            allow_empty = true
                        ),
                        Struct(
                            name = YulReturnsDeclaration,
                            fields = (
                                minus_greater_than = Required(MinusGreaterThan),
                                variables = Required(YulVariableNames)
                            )
                        ),
                        Separated(
                            name = YulVariableNames,
                            reference = YulIdentifier,
                            separator = Comma
                        ),
                        Struct(
                            name = YulVariableDeclarationStatement,
                            fields = (
                                let_keyword = Required(YulLetKeyword),
                                variables = Required(YulVariableNames),
                                value = Optional(reference = YulVariableDeclarationValue)
                            )
                        ),
                        Struct(
                            name = YulVariableDeclarationValue,
                            fields = (
                                assignment = Required(YulAssignmentOperator),
                                expression = Required(YulExpression)
                            )
                        ),
                        Struct(
                            name = YulVariableAssignmentStatement,
                            fields = (
                                variables = Required(YulPaths),
                                assignment = Required(YulAssignmentOperator),
                                expression = Required(YulExpression)
                            )
                        ),
                        Enum(
                            name = YulAssignmentOperator,
                            variants = [
                                EnumVariant(reference = ColonEqual),
                                EnumVariant(reference = YulColonAndEqual, enabled = Till("0.5.5"))
                            ]
                        ),
                        Struct(
                            name = YulColonAndEqual,
                            enabled = Till("0.5.5"),
                            fields = (colon = Required(Colon), equal = Required(Equal))
                        ),
                        Struct(
                            name = YulStackAssignmentStatement,
                            enabled = Till("0.5.0"),
                            fields = (
                                assignment = Required(YulStackAssignmentOperator),
                                variable = Required(YulIdentifier)
                            )
                        ),
                        Enum(
                            name = YulStackAssignmentOperator,
                            enabled = Till("0.5.0"),
                            variants = [
                                EnumVariant(reference = EqualColon),
                                EnumVariant(reference = YulEqualAndColon)
                            ]
                        ),
                        Struct(
                            name = YulEqualAndColon,
                            enabled = Till("0.5.0"),
                            fields = (equal = Required(Equal), colon = Required(Colon))
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
                            name = YulLabel,
                            enabled = Till("0.5.0"),
                            fields = (label = Required(YulIdentifier), colon = Required(Colon))
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
                                        arguments = Required(YulArguments),
                                        close_paren = Required(CloseParen)
                                    )
                                )]
                            )],
                            primary_expressions = [
                                PrimaryExpression(reference = YulLiteral),
                                PrimaryExpression(reference = YulPath)
                            ]
                        ),
                        Separated(
                            name = YulArguments,
                            reference = YulExpression,
                            separator = Comma,
                            allow_empty = true
                        ),
                        Separated(name = YulPaths, reference = YulPath, separator = Comma),
                        Separated(
                            name = YulPath,
                            reference = YulIdentifier,
                            separator = Period
                        ),
                        Token(
                            name = YulIdentifier,
                            definitions = [
                                // Dots were allowed specifically between these versions:
                                TokenDefinition(
                                    enabled = Range(from = "0.5.8", till = "0.7.0"),
                                    scanner = Sequence([
                                        Fragment(IdentifierStart),
                                        ZeroOrMore(Choice([Fragment(IdentifierPart), Atom(".")]))
                                    ])
                                ),
                                // Otherwise, parse as a regular identifier:
                                TokenDefinition(
                                    scanner = Sequence([
                                        Fragment(IdentifierStart),
                                        ZeroOrMore(Fragment(IdentifierPart))
                                    ])
                                )
                            ]
                        ),
                        Enum(
                            name = YulLiteral,
                            variants = [
                                EnumVariant(reference = YulTrueKeyword, enabled = From("0.6.2")),
                                EnumVariant(reference = YulFalseKeyword, enabled = From("0.6.2")),
                                EnumVariant(reference = YulDecimalLiteral),
                                EnumVariant(reference = YulHexLiteral),
                                EnumVariant(reference = HexStringLiteral),
                                EnumVariant(reference = StringLiteral)
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
                            definitions = [KeywordDefinition(
                                enabled = From("0.6.2"),
                                value = Atom("false")
                            )]
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
                            name = YulReturnsKeyword,
                            identifier = YulIdentifier,
                            definitions = [KeywordDefinition(
                                enabled = Never,
                                reserved = Till("0.7.1"),
                                value = Atom("returns")
                            )]
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
                            name = YulSuperKeyword,
                            identifier = YulIdentifier,
                            definitions = [KeywordDefinition(
                                enabled = Never,
                                reserved = From("0.8.0"),
                                value = Atom("super")
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
                            name = YulThisKeyword,
                            identifier = YulIdentifier,
                            definitions = [KeywordDefinition(
                                enabled = Never,
                                reserved = From("0.8.0"),
                                value = Atom("this")
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
                            definitions = [KeywordDefinition(
                                enabled = From("0.6.2"),
                                value = Atom("true")
                            )]
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
    ],
    built_ins = [
        BuiltInContext(
            name = "SolidityBuiltIns",
            definitions = [
                BuiltInFunction(
                    name = "addmod",
                    parameters = ["uint x", "uint y", "uint k"],
                    return_type = "uint"
                ),
                BuiltInFunction(name = "assert", parameters = ["bool condition"]),
                BuiltInFunction(
                    name = "blockhash",
                    parameters = ["uint blockNumber"],
                    return_type = "bytes32",
                    enabled = From("0.4.22")
                ),
                BuiltInFunction(
                    name = "blobhash",
                    parameters = ["uint index"],
                    return_type = "bytes32",
                    enabled = From("0.8.24")
                ),
                BuiltInFunction(
                    name = "ecrecover",
                    parameters = ["bytes32 hash", "uint8 v", "bytes32 r", "bytes32 s"],
                    return_type = "address"
                ),
                BuiltInFunction(
                    name = "gasleft",
                    parameters = [],
                    return_type = "uint256",
                    enabled = From("0.4.21")
                ),
                BuiltInFunction(
                    name = "keccak256",
                    parameters = ["bytes memory"],
                    return_type = "bytes32"
                ),
                BuiltInFunction(
                    name = "log0",
                    parameters = ["bytes32"],
                    enabled = Till("0.8.0")
                ),
                BuiltInFunction(
                    name = "log1",
                    parameters = ["bytes32", "bytes32"],
                    enabled = Till("0.8.0")
                ),
                BuiltInFunction(
                    name = "log2",
                    parameters = ["bytes32", "bytes32", "bytes32"],
                    enabled = Till("0.8.0")
                ),
                BuiltInFunction(
                    name = "log3",
                    parameters = ["bytes32", "bytes32", "bytes32", "bytes32"],
                    enabled = Till("0.8.0")
                ),
                BuiltInFunction(
                    name = "log4",
                    parameters = ["bytes32", "bytes32", "bytes32", "bytes32", "bytes32"],
                    enabled = Till("0.8.0")
                ),
                BuiltInFunction(
                    name = "mulmod",
                    parameters = ["uint x", "uint y", "uint k"],
                    return_type = "uint"
                ),
                BuiltInFunction(name = "require", parameters = ["bool condition"]),
                BuiltInFunction(
                    name = "require",
                    parameters = ["bool condition", "string memory message"],
                    enabled = From("0.4.22")
                ),
                BuiltInFunction(
                    name = "require",
                    parameters = ["bool condition", "Error error"],
                    enabled = From("0.8.26")
                ),
                BuiltInFunction(name = "revert", parameters = []),
                BuiltInFunction(
                    name = "revert",
                    parameters = ["string memory reason"],
                    enabled = From("0.4.22")
                ),
                BuiltInFunction(
                    name = "ripemd160",
                    parameters = ["bytes memory"],
                    return_type = "bytes20"
                ),
                BuiltInFunction(
                    name = "selfdestruct",
                    parameters = ["address payable recipient"]
                ),
                BuiltInFunction(
                    name = "sha256",
                    parameters = ["bytes memory"],
                    return_type = "bytes32"
                ),
                BuiltInFunction(
                    name = "sha3",
                    parameters = ["bytes memory"],
                    return_type = "bytes32",
                    enabled = Till("0.5.0")
                ),
                BuiltInFunction(
                    name = "suicide",
                    parameters = ["address payable recipient"],
                    enabled = Till("0.5.0")
                ),
                BuiltInType(
                    name = "%AbiType",
                    fields = [],
                    functions = [
                        BuiltInFunction(
                            name = "decode",
                            parameters = ["bytes memory encodedData", "%Type[] encodedTypesTuple"],
                            return_type = "%Any[]",
                            enabled = From("0.5.0")
                        ),
                        BuiltInFunction(
                            name = "encode",
                            parameters = ["%Any[] valuesToEncode"],
                            return_type = "bytes memory",
                            enabled = From("0.4.22")
                        ),
                        BuiltInFunction(
                            name = "encodeCall",
                            parameters = [
                                "function() functionPointer",
                                "%Any[] functionArgumentsTuple"
                            ],
                            return_type = "bytes memory",
                            enabled = From("0.8.11")
                        ),
                        BuiltInFunction(
                            name = "encodePacked",
                            parameters = ["%Any[] valuesToEncode"],
                            return_type = "bytes memory",
                            enabled = From("0.4.22")
                        ),
                        BuiltInFunction(
                            name = "encodeWithSelector",
                            parameters = ["bytes4 selector", "%Any[] functionArgumentsTuple"],
                            return_type = "bytes memory",
                            enabled = From("0.4.22")
                        ),
                        BuiltInFunction(
                            name = "encodeWithSignature",
                            parameters = ["string memory signature", "%Any[] valuesToEncode"],
                            return_type = "bytes memory",
                            enabled = From("0.4.22")
                        )
                    ]
                ),
                BuiltInType(
                    name = "address",
                    fields = [
                        BuiltInField(definition = "uint256 balance"),
                        BuiltInField(definition = "bytes code", enabled = From("0.8.0")),
                        BuiltInField(definition = "bytes32 codehash", enabled = From("0.8.0"))
                    ],
                    functions = [
                        BuiltInFunction(
                            name = "call",
                            parameters = ["bytes memory"],
                            return_type = "bool",
                            enabled = Till("0.5.0")
                        ),
                        BuiltInFunction(
                            name = "call",
                            parameters = ["bytes memory"],
                            return_type = "bool, bytes memory",
                            enabled = From("0.5.0")
                        ),
                        BuiltInFunction(
                            name = "callcode",
                            parameters = ["bytes memory"],
                            return_type = "bool, bytes memory",
                            enabled = Till("0.5.0")
                        ),
                        BuiltInFunction(
                            name = "delegatecall",
                            parameters = ["bytes memory"],
                            return_type = "bool",
                            enabled = Till("0.5.0")
                        ),
                        BuiltInFunction(
                            name = "delegatecall",
                            parameters = ["bytes memory"],
                            return_type = "bool, bytes memory",
                            enabled = From("0.5.0")
                        ),
                        BuiltInFunction(
                            name = "send",
                            parameters = ["uint256 amount"],
                            return_type = "bool",
                            enabled = Till("0.5.0")
                        ),
                        BuiltInFunction(
                            name = "staticcall",
                            parameters = ["bytes memory"],
                            return_type = "bool, bytes memory",
                            enabled = From("0.5.0")
                        ),
                        BuiltInFunction(
                            name = "transfer",
                            parameters = ["uint256 amount"],
                            // `transfer` is disallowed on non-payable address
                            // types since 0.5.0, but there's code in the wild
                            // which uses type casting to do eg.
                            // `address(uint160(to)).transfer(..)`.
                            enabled = Till("0.8.0")
                        )
                    ]
                ),
                BuiltInType(
                    name = "address payable",
                    fields = [
                        BuiltInField(definition = "uint256 balance"),
                        BuiltInField(definition = "bytes code", enabled = From("0.8.0")),
                        BuiltInField(definition = "bytes32 codehash", enabled = From("0.8.0"))
                    ],
                    functions = [
                        BuiltInFunction(
                            name = "call",
                            parameters = ["bytes memory"],
                            return_type = "bool, bytes memory"
                        ),
                        BuiltInFunction(
                            name = "delegatecall",
                            parameters = ["bytes memory"],
                            return_type = "bool, bytes memory"
                        ),
                        BuiltInFunction(
                            name = "send",
                            parameters = ["uint256 amount"],
                            return_type = "bool"
                        ),
                        BuiltInFunction(
                            name = "staticcall",
                            parameters = ["bytes memory"],
                            return_type = "bool, bytes memory"
                        ),
                        BuiltInFunction(name = "transfer", parameters = ["uint256 amount"])
                    ],
                    enabled = From("0.5.0")
                ),
                BuiltInType(
                    name = "%Array",
                    fields = [BuiltInField(definition = "uint length")],
                    functions = [
                        BuiltInFunction(
                            name = "push",
                            parameters = [],
                            return_type = "%ValueType",
                            enabled = From("0.6.0")
                        ),
                        BuiltInFunction(
                            name = "push",
                            parameters = ["%ValueType element"],
                            return_type = "uint",
                            enabled = Till("0.6.0")
                        ),
                        BuiltInFunction(
                            name = "push",
                            parameters = ["%ValueType element"],
                            enabled = From("0.6.0")
                        ),
                        BuiltInFunction(name = "pop", parameters = [])
                    ]
                ),
                BuiltInType(
                    name = "%FixedArray",
                    fields = [BuiltInField(definition = "uint length")],
                    functions = []
                ),
                BuiltInType(
                    name = "%BlockType",
                    fields = [
                        BuiltInField(definition = "uint basefee", enabled = From("0.8.7")),
                        BuiltInField(definition = "uint blobbasefee", enabled = From("0.8.24")),
                        BuiltInField(definition = "uint chainid", enabled = From("0.8.0")),
                        BuiltInField(definition = "address payable coinbase"),
                        BuiltInField(definition = "uint difficulty"),
                        BuiltInField(definition = "uint gaslimit"),
                        BuiltInField(definition = "uint number"),
                        BuiltInField(definition = "uint prevrandao", enabled = From("0.8.18")),
                        BuiltInField(definition = "uint timestamp")
                    ],
                    functions = [BuiltInFunction(
                        name = "blockhash",
                        parameters = ["uint"],
                        return_type = "bytes32",
                        enabled = Till("0.5.0")
                    )]
                ),
                BuiltInType(
                    name = "bytes",
                    fields = [BuiltInField(definition = "uint length")],
                    functions = [
                        BuiltInFunction(
                            name = "push",
                            parameters = ["bytes1 element"],
                            return_type = "uint",
                            enabled = Till("0.6.0")
                        ),
                        BuiltInFunction(
                            name = "push",
                            parameters = ["bytes1 element"],
                            enabled = From("0.6.0")
                        ),
                        BuiltInFunction(name = "pop", parameters = [], enabled = From("0.5.0"))
                    ]
                ),
                BuiltInType(
                    name = "bytes1",
                    fields = [BuiltInField(definition = "uint length")],
                    functions = []
                ),
                BuiltInType(
                    name = "bytes2",
                    fields = [BuiltInField(definition = "uint length")],
                    functions = []
                ),
                BuiltInType(
                    name = "bytes3",
                    fields = [BuiltInField(definition = "uint length")],
                    functions = []
                ),
                BuiltInType(
                    name = "bytes4",
                    fields = [BuiltInField(definition = "uint length")],
                    functions = []
                ),
                BuiltInType(
                    name = "bytes5",
                    fields = [BuiltInField(definition = "uint length")],
                    functions = []
                ),
                BuiltInType(
                    name = "bytes6",
                    fields = [BuiltInField(definition = "uint length")],
                    functions = []
                ),
                BuiltInType(
                    name = "bytes7",
                    fields = [BuiltInField(definition = "uint length")],
                    functions = []
                ),
                BuiltInType(
                    name = "bytes8",
                    fields = [BuiltInField(definition = "uint length")],
                    functions = []
                ),
                BuiltInType(
                    name = "bytes9",
                    fields = [BuiltInField(definition = "uint length")],
                    functions = []
                ),
                BuiltInType(
                    name = "bytes10",
                    fields = [BuiltInField(definition = "uint length")],
                    functions = []
                ),
                BuiltInType(
                    name = "bytes11",
                    fields = [BuiltInField(definition = "uint length")],
                    functions = []
                ),
                BuiltInType(
                    name = "bytes12",
                    fields = [BuiltInField(definition = "uint length")],
                    functions = []
                ),
                BuiltInType(
                    name = "bytes13",
                    fields = [BuiltInField(definition = "uint length")],
                    functions = []
                ),
                BuiltInType(
                    name = "bytes14",
                    fields = [BuiltInField(definition = "uint length")],
                    functions = []
                ),
                BuiltInType(
                    name = "bytes15",
                    fields = [BuiltInField(definition = "uint length")],
                    functions = []
                ),
                BuiltInType(
                    name = "bytes16",
                    fields = [BuiltInField(definition = "uint length")],
                    functions = []
                ),
                BuiltInType(
                    name = "bytes17",
                    fields = [BuiltInField(definition = "uint length")],
                    functions = []
                ),
                BuiltInType(
                    name = "bytes18",
                    fields = [BuiltInField(definition = "uint length")],
                    functions = []
                ),
                BuiltInType(
                    name = "bytes19",
                    fields = [BuiltInField(definition = "uint length")],
                    functions = []
                ),
                BuiltInType(
                    name = "bytes20",
                    fields = [BuiltInField(definition = "uint length")],
                    functions = []
                ),
                BuiltInType(
                    name = "bytes21",
                    fields = [BuiltInField(definition = "uint length")],
                    functions = []
                ),
                BuiltInType(
                    name = "bytes22",
                    fields = [BuiltInField(definition = "uint length")],
                    functions = []
                ),
                BuiltInType(
                    name = "bytes23",
                    fields = [BuiltInField(definition = "uint length")],
                    functions = []
                ),
                BuiltInType(
                    name = "bytes24",
                    fields = [BuiltInField(definition = "uint length")],
                    functions = []
                ),
                BuiltInType(
                    name = "bytes25",
                    fields = [BuiltInField(definition = "uint length")],
                    functions = []
                ),
                BuiltInType(
                    name = "bytes26",
                    fields = [BuiltInField(definition = "uint length")],
                    functions = []
                ),
                BuiltInType(
                    name = "bytes27",
                    fields = [BuiltInField(definition = "uint length")],
                    functions = []
                ),
                BuiltInType(
                    name = "bytes28",
                    fields = [BuiltInField(definition = "uint length")],
                    functions = []
                ),
                BuiltInType(
                    name = "bytes29",
                    fields = [BuiltInField(definition = "uint length")],
                    functions = []
                ),
                BuiltInType(
                    name = "bytes30",
                    fields = [BuiltInField(definition = "uint length")],
                    functions = []
                ),
                BuiltInType(
                    name = "bytes31",
                    fields = [BuiltInField(definition = "uint length")],
                    functions = []
                ),
                BuiltInType(
                    name = "bytes32",
                    fields = [BuiltInField(definition = "uint length")],
                    functions = []
                ),
                BuiltInType(
                    name = "byte",
                    fields = [BuiltInField(definition = "uint length")],
                    functions = [],
                    enabled = Till("0.8.0")
                ),
                BuiltInType(
                    name = "%BytesType",
                    fields = [],
                    functions = [BuiltInFunction(
                        name = "concat",
                        parameters = ["bytes[] bytesToConcatenate"],
                        return_type = "bytes memory"
                    )]
                ),
                BuiltInType(
                    name = "%CallOptions",
                    fields = [
                        BuiltInField(definition = "uint gas"),
                        BuiltInField(definition = "uint salt"),
                        BuiltInField(definition = "uint value")
                    ],
                    functions = [],
                    enabled = From("0.6.2")
                ),
                BuiltInType(
                    name = "Error",
                    fields = [BuiltInField(definition = "string reason")],
                    functions = [],
                    enabled = From("0.6.0")
                ),
                BuiltInType(
                    name = "%ErrorType",
                    fields = [BuiltInField(definition = "bytes4 selector")],
                    functions = [],
                    enabled = From("0.8.4")
                ),
                BuiltInType(
                    name = "%EventType",
                    fields = [BuiltInField(definition = "bytes4 selector")],
                    functions = [],
                    enabled = From("0.8.15")
                ),
                BuiltInType(
                    name = "%Function",
                    fields = [],
                    functions = [
                        BuiltInFunction(
                            name = "gas",
                            parameters = ["uint amount"],
                            return_type = "%ExternalFunction",
                            enabled = Till("0.7.0")
                        ),
                        BuiltInFunction(
                            name = "value",
                            parameters = ["uint amount"],
                            return_type = "%ExternalFunction",
                            enabled = Till("0.7.0")
                        )
                    ]
                ),
                BuiltInType(
                    name = "%ExternalFunction",
                    fields = [
                        BuiltInField(definition = "address address", enabled = From("0.8.2")),
                        BuiltInField(definition = "bytes4 selector", enabled = From("0.4.17"))
                    ],
                    functions = [
                        BuiltInFunction(
                            name = "gas",
                            parameters = ["uint amount"],
                            return_type = "%ExternalFunction",
                            enabled = Till("0.7.0")
                        ),
                        BuiltInFunction(
                            name = "value",
                            parameters = ["uint amount"],
                            return_type = "%ExternalFunction",
                            enabled = Till("0.7.0")
                        )
                    ]
                ),
                BuiltInType(
                    name = "%MessageType",
                    fields = [
                        BuiltInField(definition = "bytes data"),
                        BuiltInField(definition = "uint256 gas", enabled = Till("0.5.0")),
                        BuiltInField(
                            definition = "address payable sender",
                            enabled = Range(from = "0.5.0", till = "0.8.0")
                        ),
                        BuiltInField(definition = "address sender", enabled = Till("0.5.0")),
                        BuiltInField(definition = "address sender", enabled = From("0.8.0")),
                        BuiltInField(definition = "bytes4 sig"),
                        BuiltInField(definition = "uint value")
                    ],
                    functions = []
                ),
                BuiltInType(
                    name = "Panic",
                    fields = [BuiltInField(definition = "uint errorCode")],
                    functions = [],
                    enabled = From("0.6.0")
                ),
                BuiltInType(
                    name = "%StringType",
                    fields = [],
                    functions = [BuiltInFunction(
                        name = "concat",
                        parameters = ["string[] stringsToConcatenate"],
                        return_type = "string memory"
                    )]
                ),
                BuiltInType(
                    name = "%TransactionType",
                    fields = [
                        BuiltInField(definition = "uint gasprice"),
                        BuiltInField(
                            definition = "address payable origin",
                            enabled = Till("0.8.0")
                        ),
                        BuiltInField(definition = "address origin", enabled = From("0.8.0"))
                    ],
                    functions = []
                ),
                BuiltInType(
                    name = "%ContractTypeType",
                    fields = [
                        BuiltInField(definition = "string name"),
                        BuiltInField(definition = "bytes creationCode", enabled = From("0.5.3")),
                        BuiltInField(definition = "bytes runtimeCode", enabled = From("0.5.3")),
                        BuiltInField(definition = "bytes4 interfaceId", enabled = From("0.6.7"))
                    ],
                    functions = []
                ),
                BuiltInType(
                    name = "%InterfaceTypeType",
                    fields = [
                        BuiltInField(definition = "string name"),
                        BuiltInField(definition = "bytes4 interfaceId", enabled = From("0.6.7"))
                    ],
                    functions = []
                ),
                BuiltInType(
                    name = "%IntTypeType",
                    fields = [
                        BuiltInField(definition = "int min", enabled = From("0.6.8")),
                        BuiltInField(definition = "int max", enabled = From("0.6.8"))
                    ],
                    functions = []
                ),
                BuiltInType(
                    name = "%UserDefinedValueType",
                    fields = [],
                    functions = [
                        BuiltInFunction(
                            name = "wrap",
                            parameters = ["%WrappedType elementaryType"],
                            return_type = "%UserType"
                        ),
                        BuiltInFunction(
                            name = "unwrap",
                            parameters = ["%UserType userType"],
                            return_type = "%WrappedType"
                        )
                    ],
                    enabled = From("0.8.8")
                ),
                // Placeholder for modifiers, to be used to inject the modified function body (`_;`)
                BuiltInVariable(definition = "%Function %_"),
                BuiltInVariable(definition = "%AbiType abi"),
                BuiltInVariable(definition = "%BlockType block"),
                BuiltInVariable(definition = "%BytesType bytes"),
                BuiltInVariable(definition = "%MessageType msg"),
                BuiltInVariable(definition = "uint now", enabled = Till("0.7.0")),
                BuiltInVariable(definition = "%StringType string"),
                BuiltInVariable(definition = "%TransactionType tx")
            ]
        ),
        BuiltInContext(
            // __SLANG_YUL_BUILT_INS_CONTEXT_NAME__ keep in sync with built-ins code generation
            name = "YulBuiltIns",
            definitions = [
                BuiltInType(
                    name = "%YulExternal",
                    fields = [
                        // These apply to state and storage variables
                        BuiltInField(definition = "uint256 slot"),
                        BuiltInField(definition = "uint256 offset"),
                        // Dynamic calldata arrays also have a length
                        BuiltInField(definition = "uint256 length")
                    ],
                    functions = []
                ),
                BuiltInFunction(
                    name = "add",
                    parameters = ["uint256 x", "uint256 y"],
                    return_type = "uint256"
                ),
                BuiltInFunction(
                    name = "addmod",
                    parameters = ["uint256 x", "uint256 y", "uint256 m"],
                    return_type = "uint256"
                ),
                BuiltInFunction(name = "address", parameters = [], return_type = "uint256"),
                BuiltInFunction(
                    name = "and",
                    parameters = ["uint256 x", "uint256 y"],
                    return_type = "uint256"
                ),
                BuiltInFunction(
                    name = "balance",
                    parameters = ["uint256 a"],
                    return_type = "uint256"
                ),
                BuiltInFunction(
                    name = "blockhash",
                    parameters = ["uint256 b"],
                    return_type = "uint256"
                ),
                BuiltInFunction(
                    name = "byte",
                    parameters = ["uint256 n", "uint256 x"],
                    return_type = "uint256"
                ),
                BuiltInFunction(
                    name = "callcode",
                    parameters = [
                        "uint256 g",
                        "uint256 a",
                        "uint256 v",
                        "uint256 in_",
                        "uint256 insize",
                        "uint256 out",
                        "uint256 outsize"
                    ],
                    return_type = "uint256"
                ),
                BuiltInFunction(
                    name = "calldatacopy",
                    parameters = ["uint256 t", "uint256 f", "uint256 s"]
                ),
                BuiltInFunction(
                    name = "calldataload",
                    parameters = ["uint256 p"],
                    return_type = "uint256"
                ),
                BuiltInFunction(
                    name = "calldatasize",
                    parameters = [],
                    return_type = "uint256"
                ),
                BuiltInFunction(name = "caller", parameters = [], return_type = "uint256"),
                BuiltInFunction(
                    name = "call",
                    parameters = [
                        "uint256 g",
                        "uint256 a",
                        "uint256 v",
                        "uint256 in_",
                        "uint256 insize",
                        "uint256 out",
                        "uint256 outsize"
                    ],
                    return_type = "uint256"
                ),
                BuiltInFunction(name = "callvalue", parameters = [], return_type = "uint256"),
                BuiltInFunction(
                    name = "codecopy",
                    parameters = ["uint256 t", "uint256 f", "uint256 s"]
                ),
                BuiltInFunction(name = "codesize", parameters = [], return_type = "uint256"),
                BuiltInFunction(name = "coinbase", parameters = [], return_type = "uint256"),
                BuiltInFunction(
                    name = "create",
                    parameters = ["uint256 v", "uint256 p", "uint256 n"],
                    return_type = "uint256"
                ),
                BuiltInFunction(
                    name = "delegatecall",
                    parameters = [
                        "uint256 g",
                        "uint256 a",
                        "uint256 in_",
                        "uint256 insize",
                        "uint256 out",
                        "uint256 outsize"
                    ],
                    return_type = "uint256"
                ),
                BuiltInFunction(
                    name = "div",
                    parameters = ["uint256 x", "uint256 y"],
                    return_type = "uint256"
                ),
                BuiltInFunction(
                    name = "eq",
                    parameters = ["uint256 x", "uint256 y"],
                    return_type = "uint256"
                ),
                BuiltInFunction(
                    name = "exp",
                    parameters = ["uint256 x", "uint256 y"],
                    return_type = "uint256"
                ),
                BuiltInFunction(
                    name = "extcodecopy",
                    parameters = ["uint256 a", "uint256 t", "uint256 f", "uint256 s"]
                ),
                BuiltInFunction(
                    name = "extcodesize",
                    parameters = ["uint256 a"],
                    return_type = "uint256"
                ),
                BuiltInFunction(name = "gas", parameters = [], return_type = "uint256"),
                BuiltInFunction(name = "gaslimit", parameters = [], return_type = "uint256"),
                BuiltInFunction(name = "gasprice", parameters = [], return_type = "uint256"),
                BuiltInFunction(
                    name = "gt",
                    parameters = ["uint256 x", "uint256 y"],
                    return_type = "uint256"
                ),
                BuiltInFunction(name = "invalid", parameters = []),
                BuiltInFunction(
                    name = "iszero",
                    parameters = ["uint256 x"],
                    return_type = "uint256"
                ),
                BuiltInFunction(name = "jump", parameters = [], enabled = Till("0.5.0")),
                BuiltInFunction(name = "jumpi", parameters = [], enabled = Till("0.5.0")),
                BuiltInFunction(name = "log0", parameters = ["uint256 p", "uint256 s"]),
                BuiltInFunction(
                    name = "log1",
                    parameters = ["uint256 p", "uint256 s", "uint256 t1"]
                ),
                BuiltInFunction(
                    name = "log2",
                    parameters = ["uint256 p", "uint256 s", "uint256 t1", "uint256 t2"]
                ),
                BuiltInFunction(
                    name = "log3",
                    parameters = [
                        "uint256 p",
                        "uint256 s",
                        "uint256 t1",
                        "uint256 t2",
                        "uint256 t3"
                    ]
                ),
                BuiltInFunction(
                    name = "log4",
                    parameters = [
                        "uint256 p",
                        "uint256 s",
                        "uint256 t1",
                        "uint256 t2",
                        "uint256 t3"
                    ]
                ),
                BuiltInFunction(
                    name = "lt",
                    parameters = ["uint256 x", "uint256 y"],
                    return_type = "uint256"
                ),
                BuiltInFunction(
                    name = "mload",
                    parameters = ["uint256 p"],
                    return_type = "uint256"
                ),
                BuiltInFunction(
                    name = "mod",
                    parameters = ["uint256 x", "uint256 y"],
                    return_type = "uint256"
                ),
                BuiltInFunction(name = "msize", parameters = [], return_type = "uint256"),
                BuiltInFunction(name = "mstore8", parameters = ["uint256 p", "uint256 v"]),
                BuiltInFunction(name = "mstore", parameters = ["uint256 p", "uint256 v"]),
                BuiltInFunction(
                    name = "mul",
                    parameters = ["uint256 x", "uint256 y"],
                    return_type = "uint256"
                ),
                BuiltInFunction(
                    name = "mulmod",
                    parameters = ["uint256 x", "uint256 y", "uint256 m"],
                    return_type = "uint256"
                ),
                BuiltInFunction(
                    name = "not",
                    parameters = ["uint256 x"],
                    return_type = "uint256"
                ),
                BuiltInFunction(
                    name = "number",
                    parameters = ["uint256 x"],
                    return_type = "uint256"
                ),
                BuiltInFunction(name = "origin", parameters = [], return_type = "uint256"),
                BuiltInFunction(
                    name = "or",
                    parameters = ["uint256 x", "uint256 y"],
                    return_type = "uint256"
                ),
                BuiltInFunction(
                    name = "pop",
                    parameters = ["uint256 x"],
                    return_type = "uint256"
                ),
                BuiltInFunction(name = "return", parameters = ["uint256 p", "uint256 s"]),
                BuiltInFunction(name = "revert", parameters = ["uint256 p", "uint256 s"]),
                BuiltInFunction(
                    name = "sdiv",
                    parameters = ["uint256 x", "uint256 y"],
                    return_type = "uint256"
                ),
                BuiltInFunction(name = "selfdestruct", parameters = ["uint256 a"]),
                BuiltInFunction(
                    name = "sgt",
                    parameters = ["uint256 x", "uint256 y"],
                    return_type = "uint256"
                ),
                BuiltInFunction(
                    name = "signextend",
                    parameters = ["uint256 i", "uint256 x"],
                    return_type = "uint256"
                ),
                BuiltInFunction(
                    name = "sload",
                    parameters = ["uint256 p"],
                    return_type = "uint256"
                ),
                BuiltInFunction(
                    name = "slt",
                    parameters = ["uint256 x", "uint256 y"],
                    return_type = "uint256"
                ),
                BuiltInFunction(
                    name = "smod",
                    parameters = ["uint256 x", "uint256 y"],
                    return_type = "uint256"
                ),
                BuiltInFunction(name = "sstore", parameters = ["uint256 p", "uint256 v"]),
                BuiltInFunction(name = "stop", parameters = []),
                BuiltInFunction(
                    name = "sub",
                    parameters = ["uint256 x", "uint256 y"],
                    return_type = "uint256"
                ),
                BuiltInFunction(name = "timestamp", parameters = [], return_type = "uint256"),
                BuiltInFunction(
                    name = "xor",
                    parameters = ["uint256 x", "uint256 y"],
                    return_type = "uint256"
                ),
                BuiltInFunction(
                    name = "keccak256",
                    parameters = ["uint256 p", "uint256 n"],
                    return_type = "uint256",
                    enabled = From("0.4.12")
                ),
                BuiltInFunction(
                    name = "sha3",
                    parameters = [],
                    return_type = "uint256",
                    enabled = Till("0.5.0")
                ),
                BuiltInFunction(
                    name = "suicide",
                    parameters = [],
                    return_type = "uint256",
                    enabled = Till("0.5.0")
                ),
                // 'Byzantium' hard-fork updates:
                BuiltInFunction(
                    name = "returndatacopy",
                    parameters = ["uint256 t", "uint256 f", "uint256 s"]
                ),
                BuiltInFunction(
                    name = "returndatasize",
                    parameters = [],
                    return_type = "uint256"
                ),
                BuiltInFunction(
                    name = "staticcall",
                    parameters = [
                        "uint256 g",
                        "uint256 a",
                        "uint256 in_",
                        "uint256 insize",
                        "uint256 out",
                        "uint256 outsize"
                    ],
                    return_type = "uint256",
                    enabled = From("0.4.12")
                ),
                // 'Constantinople' hard-fork updates:
                BuiltInFunction(
                    name = "create2",
                    parameters = ["uint256 v", "uint256 p", "uint256 n", "uint256 s"],
                    return_type = "uint256",
                    enabled = From("0.4.12")
                ),
                BuiltInFunction(
                    name = "extcodehash",
                    parameters = ["uint256 a"],
                    return_type = "uint256",
                    enabled = From("0.5.0")
                ),
                BuiltInFunction(
                    name = "sar",
                    parameters = ["uint256 x", "uint256 y"],
                    return_type = "uint256"
                ),
                BuiltInFunction(
                    name = "shl",
                    parameters = ["uint256 x", "uint256 y"],
                    return_type = "uint256"
                ),
                BuiltInFunction(
                    name = "shr",
                    parameters = ["uint256 x", "uint256 y"],
                    return_type = "uint256"
                ),
                // 'Instanbul' hard-fork updates:
                BuiltInFunction(name = "chainid", parameters = [], return_type = "uint256"),
                BuiltInFunction(
                    name = "selfbalance",
                    parameters = [],
                    return_type = "uint256"
                ),
                // 'London' hard-fork updates:
                BuiltInFunction(
                    name = "basefee",
                    parameters = [],
                    return_type = "uint256",
                    enabled = From("0.8.7")
                ),
                // 'Paris' hard-fork updates:
                BuiltInFunction(
                    name = "difficulty",
                    parameters = [],
                    return_type = "uint256",
                    enabled = Till("0.8.18")
                ),
                BuiltInFunction(
                    name = "prevrandao",
                    parameters = [],
                    return_type = "uint256",
                    enabled = From("0.8.18")
                ),
                // 'Cancun' hard-fork updates:
                BuiltInFunction(
                    name = "blobbasefee",
                    parameters = [],
                    return_type = "uint256",
                    enabled = From("0.8.24")
                ),
                BuiltInFunction(
                    name = "blobhash",
                    parameters = ["uint256 i"],
                    return_type = "uint256",
                    enabled = From("0.8.24")
                ),
                BuiltInFunction(
                    name = "tload",
                    parameters = ["uint256 p"],
                    return_type = "uint256",
                    enabled = From("0.8.24")
                ),
                BuiltInFunction(
                    name = "tstore",
                    parameters = ["uint256 p", "uint256 v"],
                    enabled = From("0.8.24")
                ),
                BuiltInFunction(
                    name = "mcopy",
                    parameters = ["uint256 t", "uint256 f", "uint256 s"],
                    enabled = From("0.8.24")
                ),
                BuiltInFunction(
                    name = "clz",
                    parameters = ["uint256 x"],
                    return_type = "uint256",
                    enabled = From("0.8.31")
                )
            ]
        )
    ]
));
