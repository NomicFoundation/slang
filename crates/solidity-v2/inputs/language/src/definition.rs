pub use solidity::SolidityDefinition;

language_v2_macros::compile!(Language(
    name = Solidity,
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
    contexts = [
        LexicalContext(
            name = Pragma,
            sections = [Section(
                title = "Pragma",
                topics = [
                    Topic(
                        title = "Pragma Directives",
                        items = [
                            Enum(
                                name = Pragma,
                                variants = [
                                    EnumVariant(reference = VersionPragma),
                                    EnumVariant(
                                        reference = AbicoderPragma,
                                        enabled = From("0.7.5")
                                    ),
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
                                definitions = [TokenDefinition(Fragment(VersionSpecifierFragment))]
                            ),
                            Token(
                                // __SLANG_VERSION_SPECIFIER_SYNTAX__ (keep in sync)
                                name = SingleQuotedVersionLiteral,
                                definitions = [TokenDefinition(Sequence([
                                    Atom("'"),
                                    Fragment(VersionSpecifierFragment),
                                    ZeroOrMore(Sequence([
                                        Atom("."),
                                        Fragment(VersionSpecifierFragment)
                                    ])),
                                    Atom("'")
                                ]))]
                            ),
                            Token(
                                // __SLANG_VERSION_SPECIFIER_SYNTAX__ (keep in sync)
                                name = DoubleQuotedVersionLiteral,
                                definitions = [TokenDefinition(Sequence([
                                    Atom("\""),
                                    Fragment(VersionSpecifierFragment),
                                    ZeroOrMore(Sequence([
                                        Atom("."),
                                        Fragment(VersionSpecifierFragment)
                                    ])),
                                    Atom("\"")
                                ]))]
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
                        title = "Pragma Keywords",
                        items = [
                            Keyword(
                                name = AbicoderKeyword,
                                enabled = From("0.7.5"),
                                definitions = [KeywordDefinition(value = Atom("abicoder"))]
                            ),
                            Keyword(
                                name = ExperimentalKeyword,
                                enabled = From("0.4.16"),
                                definitions = [KeywordDefinition(value = Atom("experimental"))]
                            ),
                            Keyword(
                                name = AbicoderV1Keyword,
                                enabled = From("0.7.5"),
                                definitions = [KeywordDefinition(value = Atom("v1"))]
                            ),
                            Keyword(
                                name = AbicoderV2Keyword,
                                enabled = From("0.7.5"),
                                definitions = [KeywordDefinition(value = Atom("v2"))]
                            ),
                            Keyword(
                                name = ABIEncoderV2Keyword,
                                enabled = From("0.4.16"),
                                definitions = [KeywordDefinition(value = Atom("ABIEncoderV2"))]
                            ),
                            Keyword(
                                name = SMTCheckerKeyword,
                                enabled = From("0.4.16"),
                                definitions = [KeywordDefinition(value = Atom("SMTChecker"))]
                            ),
                            Keyword(
                                name = SolidityKeyword,
                                definitions = [KeywordDefinition(value = Atom("solidity"))]
                            )
                        ]
                    )
                ]
            )]
        ),
        LexicalContext(
            name = Solidity,
            identifier_token = Identifier,
            sections = [
                Section(
                    title = "File Structure",
                    topics = [
                        Topic(
                            title = "Source Unit",
                            items = [
                                Struct(
                                    name = SourceUnit,
                                    fields = (members = Required(SourceUnitMembers)),
                                    parser_options = ParserOptions(inline = false, public = true)
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
                                        EnumVariant(
                                            reference = StructDefinition,
                                            enabled = From("0.6.0")
                                        ),
                                        EnumVariant(
                                            reference = EnumDefinition,
                                            enabled = From("0.6.0")
                                        ),
                                        EnumVariant(
                                            reference = FunctionDefinition,
                                            enabled = From("0.7.1")
                                        ),
                                        EnumVariant(
                                            reference = ErrorDefinition,
                                            enabled = From("0.8.4")
                                        ),
                                        EnumVariant(
                                            reference = UserDefinedValueTypeDefinition,
                                            enabled = From("0.8.8")
                                        ),
                                        EnumVariant(
                                            reference = UsingDirective,
                                            enabled = From("0.8.13")
                                        ),
                                        EnumVariant(
                                            reference = EventDefinition,
                                            enabled = From("0.8.22")
                                        ),
                                        EnumVariant(
                                            reference = ConstantDefinition,
                                            enabled = From("0.7.4")
                                        )
                                    ]
                                ),
                                Struct(
                                    name = PragmaDirective,
                                    error_recovery = FieldsErrorRecovery(terminator = semicolon),
                                    fields = (
                                        pragma_keyword = Required(PragmaKeyword),
                                        pragma = Required(Pragma),
                                        semicolon = Required(Semicolon)
                                    ),
                                    // TODO(v2): Until the lexer can perform context switching, we ignore pragmas
                                    parser_options = ParserOptions(
                                        inline = false,
                                        public = false,
                                        verbatim = r#"
PragmaDirective: PragmaDirective = {
    <_pragma_keyword: PragmaKeyword>  ! <_semicolon: Semicolon>  => {
    let abicoder = new_pragma_abicoder_pragma(new_abicoder_pragma(new_abicoder_keyword(0..0, source), new_abicoder_version_abicoder_v1_keyword(new_abicoder_v1_keyword(0..0, source))));
    new_pragma_directive(_pragma_keyword, abicoder , _semicolon)
    }
};
"#
                                    )
                                ),
                                Struct(
                                    name = ImportDirective,
                                    error_recovery = FieldsErrorRecovery(terminator = semicolon),
                                    fields = (
                                        import_keyword = Required(ImportKeyword),
                                        clause = Required(ImportClause),
                                        semicolon = Required(Semicolon)
                                    )
                                ),
                                Struct(
                                    name = UsingDirective,
                                    error_recovery = FieldsErrorRecovery(terminator = semicolon),
                                    fields = (
                                        using_keyword = Required(UsingKeyword),
                                        clause = Required(UsingClause),
                                        for_keyword = Required(ForKeyword),
                                        target = Required(UsingTarget),
                                        global_keyword = Optional(
                                            reference = GlobalKeyword,
                                            enabled = From("0.8.13")
                                        ),
                                        semicolon = Required(Semicolon)
                                    )
                                )
                            ]
                        ),
                        Topic(
                            title = "Import Directives",
                            items = [
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
                                        alias = Optional(
                                            reference = UsingAlias,
                                            enabled = From("0.8.19")
                                        )
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
                                    scanner = Choice([
                                        Atom("\n"),
                                        Sequence([Atom("\r"), Optional(Atom("\n"))])
                                    ])
                                ),
                                Trivia(
                                    name = SingleLineComment,
                                    scanner = Sequence([Atom("//"), ZeroOrMore(Not(['\r', '\n']))])
                                ),
                                Trivia(
                                    name = MultiLineComment,
                                    // https://stackoverflow.com/a/36328890
                                    scanner = Sequence([
                                        Atom("/*"),
                                        ZeroOrMore(Not(['*'])),
                                        OneOrMore(Atom("*")),
                                        ZeroOrMore(Sequence([
                                            Not(['/', '*']),
                                            ZeroOrMore(Not(['*'])),
                                            OneOrMore(Atom("*"))
                                        ])),
                                        Atom("/")
                                    ])
                                ),
                                Trivia(
                                    name = SingleLineNatSpecComment,
                                    scanner =
                                        Sequence([Atom("///"), ZeroOrMore(Not(['\r', '\n']))])
                                ),
                                Trivia(
                                    name = MultiLineNatSpecComment,
                                    // https://stackoverflow.com/a/36328890
                                    scanner = Sequence([
                                        Atom("/**"),
                                        Optional(Sequence([
                                            Not(['/', '*']),
                                            ZeroOrMore(Not(['*']))
                                        ])),
                                        OneOrMore(Atom("*")),
                                        ZeroOrMore(Sequence([
                                            Not(['/', '*']),
                                            ZeroOrMore(Not(['*'])),
                                            OneOrMore(Atom("*"))
                                        ])),
                                        Atom("/")
                                    ])
                                )
                            ]
                        ),
                        Topic(
                            title = "Keywords",
                            items = [
                                Keyword(
                                    name = AbstractKeyword,
                                    enabled = From("0.6.0"),
                                    definitions = [KeywordDefinition(value = Atom("abstract"))]
                                ),
                                Keyword(
                                    // `address` is a reserved keyword, but it can still be used as an identifier in some contexts,
                                    // in particular as a member access (e.g., `myPayload.address`) or as an identifier
                                    // path
                                    // See `IdentifierPathElement` for details
                                    name = AddressKeyword,
                                    definitions = [KeywordDefinition(value = Atom("address"))]
                                ),
                                Keyword(
                                    name = AfterKeyword,
                                    enabled = Never,
                                    definitions = [KeywordDefinition(value = Atom("after"))]
                                ),
                                Keyword(
                                    name = AliasKeyword,
                                    enabled = Never,
                                    definitions = [KeywordDefinition(
                                        reserved = From("0.5.0"),
                                        value = Atom("alias")
                                    )]
                                ),
                                Keyword(
                                    name = AnonymousKeyword,
                                    definitions = [KeywordDefinition(value = Atom("anonymous"))]
                                ),
                                Keyword(
                                    name = ApplyKeyword,
                                    enabled = Never,
                                    definitions = [KeywordDefinition(
                                        reserved = From("0.5.0"),
                                        value = Atom("apply")
                                    )]
                                ),
                                Keyword(
                                    name = AsKeyword,
                                    definitions = [KeywordDefinition(value = Atom("as"))]
                                ),
                                Keyword(
                                    name = AssemblyKeyword,
                                    definitions = [KeywordDefinition(value = Atom("assembly"))]
                                ),
                                Keyword(
                                    name = AtKeyword,
                                    enabled = From("0.8.29"),
                                    definitions =
                                        [KeywordDefinition(reserved = Never, value = Atom("at"))]
                                ),
                                Keyword(
                                    name = AutoKeyword,
                                    enabled = Never,
                                    definitions = [KeywordDefinition(
                                        reserved = From("0.5.0"),
                                        value = Atom("auto")
                                    )]
                                ),
                                Keyword(
                                    name = BoolKeyword,
                                    definitions = [KeywordDefinition(value = Atom("bool"))]
                                ),
                                Keyword(
                                    name = BreakKeyword,
                                    definitions = [KeywordDefinition(value = Atom("break"))]
                                ),
                                Keyword(
                                    name = ByteKeyword,
                                    enabled = Till("0.8.0"),
                                    definitions = [KeywordDefinition(value = Atom("byte"))]
                                ),
                                Keyword(
                                    name = BytesKeyword,
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
                                    enabled = From("0.5.0"),
                                    definitions = [KeywordDefinition(
                                        reserved = From("0.5.0"),
                                        value = Atom("calldata")
                                    )]
                                ),
                                Keyword(
                                    name = CaseKeyword,
                                    enabled = Never,
                                    definitions = [KeywordDefinition(value = Atom("case"))]
                                ),
                                Keyword(
                                    name = CatchKeyword,
                                    enabled = From("0.6.0"),
                                    definitions = [KeywordDefinition(value = Atom("catch"))]
                                ),
                                Keyword(
                                    name = ConstantKeyword,
                                    definitions = [KeywordDefinition(value = Atom("constant"))]
                                ),
                                Keyword(
                                    name = ConstructorKeyword,
                                    enabled = From("0.4.22"),
                                    definitions = [KeywordDefinition(
                                        reserved = From("0.5.0"),
                                        value = Atom("constructor")
                                    )]
                                ),
                                Keyword(
                                    name = ContinueKeyword,
                                    definitions = [KeywordDefinition(value = Atom("continue"))]
                                ),
                                Keyword(
                                    name = ContractKeyword,
                                    definitions = [KeywordDefinition(value = Atom("contract"))]
                                ),
                                Keyword(
                                    name = CopyOfKeyword,
                                    enabled = Never,
                                    definitions = [KeywordDefinition(
                                        reserved = From("0.5.0"),
                                        value = Atom("copyof")
                                    )]
                                ),
                                Keyword(
                                    name = DaysKeyword,
                                    definitions = [KeywordDefinition(value = Atom("days"))]
                                ),
                                Keyword(
                                    name = DefaultKeyword,
                                    enabled = Never,
                                    definitions = [KeywordDefinition(value = Atom("default"))]
                                ),
                                Keyword(
                                    name = DefineKeyword,
                                    enabled = Never,
                                    definitions = [KeywordDefinition(
                                        reserved = From("0.5.0"),
                                        value = Atom("define")
                                    )]
                                ),
                                Keyword(
                                    name = DeleteKeyword,
                                    definitions = [KeywordDefinition(value = Atom("delete"))]
                                ),
                                Keyword(
                                    name = DoKeyword,
                                    definitions = [KeywordDefinition(value = Atom("do"))]
                                ),
                                Keyword(
                                    name = ElseKeyword,
                                    definitions = [KeywordDefinition(value = Atom("else"))]
                                ),
                                Keyword(
                                    name = EmitKeyword,
                                    enabled = From("0.4.21"),
                                    definitions = [KeywordDefinition(
                                        reserved = From("0.5.0"),
                                        value = Atom("emit")
                                    )]
                                ),
                                Keyword(
                                    name = EnumKeyword,
                                    definitions = [KeywordDefinition(value = Atom("enum"))]
                                ),
                                Keyword(
                                    name = ErrorKeyword,
                                    enabled = From("0.8.4"),
                                    definitions = [KeywordDefinition(
                                        reserved = Never,
                                        value = Atom("error")
                                    )]
                                ),
                                Keyword(
                                    name = EtherKeyword,
                                    definitions = [KeywordDefinition(value = Atom("ether"))]
                                ),
                                Keyword(
                                    name = EventKeyword,
                                    definitions = [KeywordDefinition(value = Atom("event"))]
                                ),
                                Keyword(
                                    name = ExternalKeyword,
                                    definitions = [KeywordDefinition(value = Atom("external"))]
                                ),
                                Keyword(
                                    name = FallbackKeyword,
                                    definitions = [KeywordDefinition(
                                        reserved = From("0.6.0"),
                                        value = Atom("fallback")
                                    )]
                                ),
                                Keyword(
                                    name = FalseKeyword,
                                    definitions = [KeywordDefinition(value = Atom("false"))]
                                ),
                                Keyword(
                                    name = FinalKeyword,
                                    enabled = Never,
                                    definitions = [KeywordDefinition(value = Atom("final"))]
                                ),
                                Keyword(
                                    name = FinneyKeyword,
                                    enabled = Till("0.7.0"),
                                    definitions = [KeywordDefinition(
                                        reserved = Till("0.7.0"),
                                        value = Atom("finney")
                                    )]
                                ),
                                Keyword(
                                    name = FixedKeyword,
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
                                    definitions = [KeywordDefinition(value = Atom("for"))]
                                ),
                                Keyword(
                                    name = FromKeyword,
                                    definitions =
                                        [KeywordDefinition(reserved = Never, value = Atom("from"))]
                                ),
                                Keyword(
                                    name = FunctionKeyword,
                                    definitions = [KeywordDefinition(value = Atom("function"))]
                                ),
                                Keyword(
                                    name = GlobalKeyword,
                                    enabled = From("0.8.13"),
                                    definitions = [KeywordDefinition(
                                        reserved = Never,
                                        value = Atom("global")
                                    )]
                                ),
                                Keyword(
                                    name = GweiKeyword,
                                    enabled = From("0.6.11"),
                                    definitions = [KeywordDefinition(
                                        reserved = From("0.7.0"),
                                        value = Atom("gwei")
                                    )]
                                ),
                                Keyword(
                                    name = HexKeyword,
                                    enabled = Never,
                                    definitions = [KeywordDefinition(value = Atom("hex"))]
                                ),
                                Keyword(
                                    name = HoursKeyword,
                                    definitions = [KeywordDefinition(value = Atom("hours"))]
                                ),
                                Keyword(
                                    name = IfKeyword,
                                    definitions = [KeywordDefinition(value = Atom("if"))]
                                ),
                                Keyword(
                                    name = ImmutableKeyword,
                                    enabled = From("0.6.5"),
                                    definitions = [KeywordDefinition(
                                        reserved = From("0.5.0"),
                                        value = Atom("immutable")
                                    )]
                                ),
                                Keyword(
                                    name = ImplementsKeyword,
                                    enabled = Never,
                                    definitions = [KeywordDefinition(
                                        reserved = From("0.5.0"),
                                        value = Atom("implements")
                                    )]
                                ),
                                Keyword(
                                    name = ImportKeyword,
                                    definitions = [KeywordDefinition(value = Atom("import"))]
                                ),
                                Keyword(
                                    name = IndexedKeyword,
                                    definitions = [KeywordDefinition(value = Atom("indexed"))]
                                ),
                                Keyword(
                                    name = InKeyword,
                                    enabled = Never,
                                    definitions = [KeywordDefinition(value = Atom("in"))]
                                ),
                                Keyword(
                                    name = InlineKeyword,
                                    enabled = Never,
                                    definitions = [KeywordDefinition(value = Atom("inline"))]
                                ),
                                Keyword(
                                    name = InterfaceKeyword,
                                    definitions = [KeywordDefinition(value = Atom("interface"))]
                                ),
                                Keyword(
                                    name = InternalKeyword,
                                    definitions = [KeywordDefinition(value = Atom("internal"))]
                                ),
                                Keyword(
                                    name = IntKeyword,
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
                                    definitions = [KeywordDefinition(value = Atom("is"))]
                                ),
                                Keyword(
                                    name = LayoutKeyword,
                                    enabled = From("0.8.29"),
                                    definitions = [KeywordDefinition(
                                        reserved = Never,
                                        value = Atom("layout")
                                    )]
                                ),
                                Keyword(
                                    name = LetKeyword,
                                    enabled = Never,
                                    definitions = [KeywordDefinition(value = Atom("let"))]
                                ),
                                Keyword(
                                    name = LibraryKeyword,
                                    definitions = [KeywordDefinition(value = Atom("library"))]
                                ),
                                Keyword(
                                    name = MacroKeyword,
                                    enabled = Never,
                                    definitions = [KeywordDefinition(
                                        reserved = From("0.5.0"),
                                        value = Atom("macro")
                                    )]
                                ),
                                Keyword(
                                    name = MappingKeyword,
                                    definitions = [KeywordDefinition(value = Atom("mapping"))]
                                ),
                                Keyword(
                                    name = MatchKeyword,
                                    enabled = Never,
                                    definitions = [KeywordDefinition(value = Atom("match"))]
                                ),
                                Keyword(
                                    name = MemoryKeyword,
                                    definitions = [KeywordDefinition(value = Atom("memory"))]
                                ),
                                Keyword(
                                    name = MinutesKeyword,
                                    definitions = [KeywordDefinition(value = Atom("minutes"))]
                                ),
                                Keyword(
                                    name = ModifierKeyword,
                                    definitions = [KeywordDefinition(value = Atom("modifier"))]
                                ),
                                Keyword(
                                    name = MutableKeyword,
                                    enabled = Never,
                                    definitions = [KeywordDefinition(
                                        reserved = From("0.5.0"),
                                        value = Atom("mutable")
                                    )]
                                ),
                                Keyword(
                                    name = NewKeyword,
                                    definitions = [KeywordDefinition(value = Atom("new"))]
                                ),
                                Keyword(
                                    name = NullKeyword,
                                    enabled = Never,
                                    definitions = [KeywordDefinition(value = Atom("null"))]
                                ),
                                Keyword(
                                    name = OfKeyword,
                                    enabled = Never,
                                    definitions = [KeywordDefinition(value = Atom("of"))]
                                ),
                                Keyword(
                                    name = OverrideKeyword,
                                    enabled = From("0.6.0"),
                                    definitions = [KeywordDefinition(
                                        reserved = From("0.5.0"),
                                        value = Atom("override")
                                    )]
                                ),
                                Keyword(
                                    name = PartialKeyword,
                                    enabled = Never,
                                    definitions = [KeywordDefinition(
                                        reserved = From("0.5.0"),
                                        value = Atom("partial")
                                    )]
                                ),
                                Keyword(
                                    name = PayableKeyword,
                                    definitions = [KeywordDefinition(value = Atom("payable"))]
                                ),
                                Keyword(
                                    name = PragmaKeyword,
                                    definitions = [KeywordDefinition(value = Atom("pragma"))]
                                ),
                                Keyword(
                                    name = PrivateKeyword,
                                    definitions = [KeywordDefinition(value = Atom("private"))]
                                ),
                                Keyword(
                                    name = PromiseKeyword,
                                    enabled = Never,
                                    definitions = [KeywordDefinition(
                                        reserved = From("0.5.0"),
                                        value = Atom("promise")
                                    )]
                                ),
                                Keyword(
                                    name = PublicKeyword,
                                    definitions = [KeywordDefinition(value = Atom("public"))]
                                ),
                                Keyword(
                                    name = PureKeyword,
                                    enabled = From("0.4.16"),
                                    definitions = [KeywordDefinition(value = Atom("pure"))]
                                ),
                                Keyword(
                                    name = ReceiveKeyword,
                                    definitions = [KeywordDefinition(
                                        reserved = From("0.6.0"),
                                        value = Atom("receive")
                                    )]
                                ),
                                Keyword(
                                    name = ReferenceKeyword,
                                    enabled = Never,
                                    definitions = [KeywordDefinition(
                                        reserved = From("0.5.0"),
                                        value = Atom("reference")
                                    )]
                                ),
                                Keyword(
                                    name = RelocatableKeyword,
                                    enabled = Never,
                                    definitions = [KeywordDefinition(value = Atom("relocatable"))]
                                ),
                                Keyword(
                                    name = ReturnKeyword,
                                    definitions = [KeywordDefinition(value = Atom("return"))]
                                ),
                                Keyword(
                                    name = ReturnsKeyword,
                                    definitions = [KeywordDefinition(value = Atom("returns"))]
                                ),
                                Keyword(
                                    name = RevertKeyword,
                                    enabled = From("0.8.4"),
                                    definitions = [KeywordDefinition(
                                        reserved = Never,
                                        value = Atom("revert")
                                    )]
                                ),
                                Keyword(
                                    name = SealedKeyword,
                                    enabled = Never,
                                    definitions = [KeywordDefinition(
                                        reserved = From("0.5.0"),
                                        value = Atom("sealed")
                                    )]
                                ),
                                Keyword(
                                    name = SecondsKeyword,
                                    definitions = [KeywordDefinition(value = Atom("seconds"))]
                                ),
                                Keyword(
                                    name = SizeOfKeyword,
                                    enabled = Never,
                                    definitions = [KeywordDefinition(
                                        reserved = From("0.5.0"),
                                        value = Atom("sizeof")
                                    )]
                                ),
                                Keyword(
                                    name = StaticKeyword,
                                    enabled = Never,
                                    definitions = [KeywordDefinition(value = Atom("static"))]
                                ),
                                Keyword(
                                    name = StorageKeyword,
                                    definitions = [KeywordDefinition(value = Atom("storage"))]
                                ),
                                Keyword(
                                    name = StringKeyword,
                                    definitions = [KeywordDefinition(value = Atom("string"))]
                                ),
                                Keyword(
                                    name = StructKeyword,
                                    definitions = [KeywordDefinition(value = Atom("struct"))]
                                ),
                                Keyword(
                                    name = SuperKeyword,
                                    definitions = [KeywordDefinition(
                                        reserved = From("0.8.0"),
                                        value = Atom("super")
                                    )]
                                ),
                                Keyword(
                                    name = SupportsKeyword,
                                    enabled = Never,
                                    definitions = [KeywordDefinition(
                                        reserved = From("0.5.0"),
                                        value = Atom("supports")
                                    )]
                                ),
                                Keyword(
                                    name = SwitchKeyword,
                                    enabled = Never,
                                    definitions = [KeywordDefinition(value = Atom("switch"))]
                                ),
                                Keyword(
                                    name = SzaboKeyword,
                                    enabled = Till("0.7.0"),
                                    definitions = [KeywordDefinition(
                                        reserved = Till("0.7.0"),
                                        value = Atom("szabo")
                                    )]
                                ),
                                Keyword(
                                    name = ThisKeyword,
                                    definitions = [KeywordDefinition(
                                        reserved = From("0.8.0"),
                                        value = Atom("this")
                                    )]
                                ),
                                Keyword(
                                    name = ThrowKeyword,
                                    enabled = Till("0.5.0"),
                                    definitions = [KeywordDefinition(value = Atom("throw"))]
                                ),
                                Keyword(
                                    name = TransientKeyword,
                                    enabled = From("0.8.27"),
                                    definitions = [KeywordDefinition(
                                        reserved = Never,
                                        value = Atom("transient")
                                    )]
                                ),
                                Keyword(
                                    name = TrueKeyword,
                                    definitions = [KeywordDefinition(value = Atom("true"))]
                                ),
                                Keyword(
                                    name = TryKeyword,
                                    enabled = From("0.6.0"),
                                    definitions = [KeywordDefinition(value = Atom("try"))]
                                ),
                                Keyword(
                                    name = TypeDefKeyword,
                                    enabled = Never,
                                    definitions = [KeywordDefinition(
                                        reserved = From("0.5.0"),
                                        value = Atom("typedef")
                                    )]
                                ),
                                Keyword(
                                    name = TypeKeyword,
                                    enabled = From("0.5.3"),
                                    definitions = [KeywordDefinition(value = Atom("type"))]
                                ),
                                Keyword(
                                    name = TypeOfKeyword,
                                    enabled = Never,
                                    definitions = [KeywordDefinition(value = Atom("typeof"))]
                                ),
                                Keyword(
                                    name = UfixedKeyword,
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
                                    enabled = From("0.8.0"),
                                    definitions = [KeywordDefinition(
                                        reserved = From("0.5.0"),
                                        value = Atom("unchecked")
                                    )]
                                ),
                                Keyword(
                                    name = UsingKeyword,
                                    definitions = [KeywordDefinition(value = Atom("using"))]
                                ),
                                Keyword(
                                    name = VarKeyword,
                                    enabled = Till("0.5.0"),
                                    definitions = [KeywordDefinition(value = Atom("var"))]
                                ),
                                Keyword(
                                    name = ViewKeyword,
                                    enabled = From("0.4.16"),
                                    definitions = [KeywordDefinition(value = Atom("view"))]
                                ),
                                Keyword(
                                    name = VirtualKeyword,
                                    enabled = From("0.6.0"),
                                    definitions = [KeywordDefinition(
                                        reserved = From("0.6.0"),
                                        value = Atom("virtual")
                                    )]
                                ),
                                Keyword(
                                    name = WeeksKeyword,
                                    definitions = [KeywordDefinition(value = Atom("weeks"))]
                                ),
                                Keyword(
                                    name = WeiKeyword,
                                    definitions = [KeywordDefinition(value = Atom("wei"))]
                                ),
                                Keyword(
                                    name = WhileKeyword,
                                    definitions = [KeywordDefinition(value = Atom("while"))]
                                ),
                                Keyword(
                                    name = YearsKeyword,
                                    enabled = Till("0.5.0"),
                                    definitions = [KeywordDefinition(value = Atom("years"))]
                                )
                            ]
                        ),
                        Topic(
                            title = "Punctuation",
                            items = [
                                Token(name = OpenParen, definitions = [TokenDefinition(Atom("("))]),
                                Token(
                                    name = CloseParen,
                                    definitions = [TokenDefinition(Atom(")"))]
                                ),
                                Token(
                                    name = OpenBracket,
                                    definitions = [TokenDefinition(Atom("["))]
                                ),
                                Token(
                                    name = CloseBracket,
                                    definitions = [TokenDefinition(Atom("]"))]
                                ),
                                Token(name = OpenBrace, definitions = [TokenDefinition(Atom("{"))]),
                                Token(
                                    name = CloseBrace,
                                    definitions = [TokenDefinition(Atom("}"))]
                                ),
                                Token(name = Comma, definitions = [TokenDefinition(Atom(","))]),
                                Token(name = Period, definitions = [TokenDefinition(Atom("."))]),
                                Token(
                                    name = QuestionMark,
                                    definitions = [TokenDefinition(Atom("?"))]
                                ),
                                Token(name = Semicolon, definitions = [TokenDefinition(Atom(";"))]),
                                Token(name = Colon, definitions = [TokenDefinition(Atom(":"))]),
                                Token(
                                    name = ColonEqual,
                                    definitions = [TokenDefinition(Atom(":="))]
                                ),
                                Token(name = Equal, definitions = [TokenDefinition(Atom("="))]),
                                Token(
                                    name = EqualColon,
                                    enabled = Till("0.5.0"),
                                    definitions = [TokenDefinition(Atom("=:"))]
                                ),
                                Token(
                                    name = EqualEqual,
                                    definitions = [TokenDefinition(Atom("=="))]
                                ),
                                Token(
                                    name = EqualGreaterThan,
                                    definitions = [TokenDefinition(Atom("=>"))]
                                ),
                                Token(name = Asterisk, definitions = [TokenDefinition(Atom("*"))]),
                                Token(
                                    name = AsteriskEqual,
                                    definitions = [TokenDefinition(Atom("*="))]
                                ),
                                Token(
                                    name = AsteriskAsterisk,
                                    definitions = [TokenDefinition(Atom("**"))]
                                ),
                                Token(name = Bar, definitions = [TokenDefinition(Atom("|"))]),
                                Token(name = BarEqual, definitions = [TokenDefinition(Atom("|="))]),
                                Token(name = BarBar, definitions = [TokenDefinition(Atom("||"))]),
                                Token(name = Ampersand, definitions = [TokenDefinition(Atom("&"))]),
                                Token(
                                    name = AmpersandEqual,
                                    definitions = [TokenDefinition(Atom("&="))]
                                ),
                                Token(
                                    name = AmpersandAmpersand,
                                    definitions = [TokenDefinition(Atom("&&"))]
                                ),
                                Token(name = LessThan, definitions = [TokenDefinition(Atom("<"))]),
                                Token(
                                    name = LessThanEqual,
                                    definitions = [TokenDefinition(Atom("<="))]
                                ),
                                Token(
                                    name = LessThanLessThan,
                                    definitions = [TokenDefinition(Atom("<<"))]
                                ),
                                Token(
                                    name = LessThanLessThanEqual,
                                    definitions = [TokenDefinition(Atom("<<="))]
                                ),
                                Token(
                                    name = GreaterThan,
                                    definitions = [TokenDefinition(Atom(">"))]
                                ),
                                Token(
                                    name = GreaterThanEqual,
                                    definitions = [TokenDefinition(Atom(">="))]
                                ),
                                Token(
                                    name = GreaterThanGreaterThan,
                                    definitions = [TokenDefinition(Atom(">>"))]
                                ),
                                Token(
                                    name = GreaterThanGreaterThanEqual,
                                    definitions = [TokenDefinition(Atom(">>="))]
                                ),
                                Token(
                                    name = GreaterThanGreaterThanGreaterThan,
                                    definitions = [TokenDefinition(Atom(">>>"))]
                                ),
                                Token(
                                    name = GreaterThanGreaterThanGreaterThanEqual,
                                    definitions = [TokenDefinition(Atom(">>>="))]
                                ),
                                Token(name = Plus, definitions = [TokenDefinition(Atom("+"))]),
                                Token(
                                    name = PlusEqual,
                                    definitions = [TokenDefinition(Atom("+="))]
                                ),
                                Token(name = PlusPlus, definitions = [TokenDefinition(Atom("++"))]),
                                Token(name = Minus, definitions = [TokenDefinition(Atom("-"))]),
                                Token(
                                    name = MinusEqual,
                                    definitions = [TokenDefinition(Atom("-="))]
                                ),
                                Token(
                                    name = MinusMinus,
                                    definitions = [TokenDefinition(Atom("--"))]
                                ),
                                Token(
                                    name = MinusGreaterThan,
                                    definitions = [TokenDefinition(Atom("->"))]
                                ),
                                Token(name = Slash, definitions = [TokenDefinition(Atom("/"))]),
                                Token(
                                    name = SlashEqual,
                                    definitions = [TokenDefinition(Atom("/="))]
                                ),
                                Token(name = Percent, definitions = [TokenDefinition(Atom("%"))]),
                                Token(
                                    name = PercentEqual,
                                    definitions = [TokenDefinition(Atom("%="))]
                                ),
                                Token(name = Bang, definitions = [TokenDefinition(Atom("!"))]),
                                Token(
                                    name = BangEqual,
                                    definitions = [TokenDefinition(Atom("!="))]
                                ),
                                Token(name = Caret, definitions = [TokenDefinition(Atom("^"))]),
                                Token(
                                    name = CaretEqual,
                                    definitions = [TokenDefinition(Atom("^="))]
                                ),
                                Token(name = Tilde, definitions = [TokenDefinition(Atom("~"))])
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
                                            reference = AbstractKeyword,
                                            enabled = From("0.6.0")
                                        ),
                                        contract_keyword = Required(ContractKeyword),
                                        name = Required(Identifier),
                                        specifiers = Required(ContractSpecifiers),
                                        open_brace = Required(OpenBrace),
                                        members = Required(ContractMembers),
                                        close_brace = Required(CloseBrace)
                                    ),
                                    parser_options = ParserOptions(inline = false, public = true, verbatim = "
// Contracts are syntactically complex (for an LR parser) since the storage layout specifier
// has a trailing expression, which can have a trailing option call (`{ ... }`), which conflicts
// with the contract members block.
//
// In order to solve this we use a trailing expression that captures both the expression and the members,
// and then extract the members from it.
pub ContractDefinition: ContractDefinition = {
    // If no specifiers are present, we simply capture the members directly
    <_abstract_keyword: (AbstractKeyword)?>  <_contract_keyword: ContractKeyword>  <_name: Identifier>  <_open_brace: OpenBrace>  <_members: ContractMembers>  <_close_brace: CloseBrace>  => {
        new_contract_definition(_abstract_keyword, _contract_keyword, _name, new_contract_specifiers(vec![]), _open_brace, _members, _close_brace)
    },
    // If specifiers are present, we extract the trailing members from them
    <_abstract_keyword: (AbstractKeyword)?>  <_contract_keyword: ContractKeyword>  <_name: Identifier>  <_specifiers: ContractSpecifiersTrailingMembers>  => {
        let (specifiers, (_open_brace, _members, _close_brace)) = _specifiers;
        new_contract_definition(_abstract_keyword, _contract_keyword, _name, specifiers, _open_brace, _members, _close_brace)
    },
};
")
                                ),
                                Repeated(
                                    name = ContractSpecifiers,
                                    reference = ContractSpecifier,
                                    allow_empty = true,
                                    parser_options = ParserOptions(inline = false, public = false, verbatim = "
// In this case, we require at least one specifier, the case with zero is handled above.
// Note that the return type now includes the trailing members
ContractSpecifiersTrailingMembers: (ContractSpecifiers, (OpenBrace, ContractMembers, CloseBrace)) = {
    <mut _contract_specifier: RepeatedAllowEmpty<<ContractSpecifier>>> <_tail: ContractSpecifierTrailingMembers>  => {
        let (specifier, tail) = _tail;
        _contract_specifier.push(specifier);
        (new_contract_specifiers(_contract_specifier), tail)
    },
};
ContractSpecifierTrailingMembers: (ContractSpecifier, (OpenBrace, ContractMembers, CloseBrace)) = {
    // Since there's no conflict with inheritance specifiers, we can parse them directly and
    // then parse the members
    <_inheritance_specifier: InheritanceSpecifier> <_open_brace: OpenBrace>  <_members: ContractMembers>  <_close_brace: CloseBrace>  => {
        (new_contract_specifier_inheritance_specifier(_inheritance_specifier), (_open_brace, _members, _close_brace))
    },
    // For storage layout specifiers, we need to extract the trailing members from them
    <_storage_layout_specifier: StorageLayoutSpecifierTrailingMembers>  => {
        let (storage_layout_specifier, tail) = _storage_layout_specifier;
        (new_contract_specifier_storage_layout_specifier(storage_layout_specifier), tail)
    },
};

StorageLayoutSpecifierTrailingMembers: (StorageLayoutSpecifier, (OpenBrace, ContractMembers, CloseBrace)) = {
    // Instead of parsing a regular Expression, we parse one that captures the trailing members
    <_layout_keyword: LayoutKeyword>  <_at_keyword: AtKeyword>  <_expression: ExpressionTrailingMembers>  => {
        let (expr, tail) = _expression;
        (new_storage_layout_specifier(_layout_keyword, _at_keyword, expr), tail)
    },
};

// An expression followed by contract members
// See the Expression rule for details
ExpressionTrailingMembers: (Expression, (OpenBrace, ContractMembers, CloseBrace)) = {
        <expression: Expression19<BracedContractMembers>>  => <>,
};
BracedContractMembers: (OpenBrace, ContractMembers, CloseBrace) = {
    <_open_brace: OpenBrace>  <_members: ContractMembers>  <_close_brace: CloseBrace>  => {
        (_open_brace, _members, _close_brace)
    },
};
")
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
                                        EnumVariant(
                                            reference = ErrorDefinition,
                                            enabled = From("0.8.4")
                                        ),
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
                                    ),
                                    parser_options = ParserOptions(inline = false, public = false, verbatim = r#"
// State variable definitions have a conflict when used with function types, since some attributes
// can be both function type attributes and state variable attributes.
// For example in `function (uint a) internal internal foo;`, the first `internal` is a function type attribute,
// while the second `internal` is a state variable attribute.
//
// To disambiguate in these cases we need to count, everything from the second attribute onwards
// belongs to the state variable. This is very hard to do in LR(1) grammars, so we resort to letting
// the function type capture all compatible attributes, and then extract the trailing ones to use them in the state variable.
// 
// This is done by splitting the state variable rules into two cases, one where any type is allowed, except function types
// that do not specify a return; and one where only function types without return are allowed.
//
// Another issue comes from the `error` keyword, since it's not reserved it can also be used as an identifier.
// This conflicts with state variables since `error foo...` could be the beginning of either an
// error definition or a state variable definition.
//
// To solve this we match against state variables where the type is exactly `error` as a special case.
StateVariableDefinition: StateVariableDefinition = {
    // When allowing any type except function types without return, we can parse normally.
    // Note the `IdentifierPathNoError`, it avoids matching against `error` as an identifier.
    <_type_name: TypeName1<FunctionTypeInternalReturn, IndexAccessPath<IdentifierPathNoError>>>  <_attributes: StateVariableAttributes>  <_name: Identifier>  <_value: (StateVariableDefinitionValue)?>  <_semicolon: Semicolon>  => new_state_variable_definition(<>),

    // Special case for `error` type
    <l:@L> L_ErrorKeyword_Unreserved <r:@R>  <_attributes: StateVariableAttributes>  <_name: Identifier>  <_value: (StateVariableDefinitionValue)?>  <_semicolon: Semicolon> => {
        let identifier = new_identifier(l..r, source);
        let iap = new_index_access_path_from_identifier_path(new_identifier_path(vec![new_identifier_path_element_identifier(identifier)]));
        let type_name = new_type_name_index_access_path(iap);

        new_state_variable_definition(type_name, _attributes, _name, _value, _semicolon)
    },


    // If the function type has no return, then we don't directly parse state variable attributes, we only do it if
    // we see a special one (one that can be a state variable attribute but not a function type attribute).
    <_type_name: FunctionTypeInternalNoReturn> <special_attributes: (<SpecialStateVariableAttribute> <StateVariableAttributes>)?> <_name: Identifier>  <_value: (StateVariableDefinitionValue)?>  <_semicolon: Semicolon>  => {
        let (function_type, mut extra_attributes) = extract_extra_attributes(_type_name);
        if let Some(special_attributes) = special_attributes {
            extra_attributes.push(special_attributes.0);
            extra_attributes.extend(special_attributes.1.elements);
        }
        new_state_variable_definition(new_type_name_function_type(function_type), new_state_variable_attributes(extra_attributes), _name, _value, _semicolon)
    },
};

// Match an identifier path that, if it's a single element, is not `error`
IdentifierPathNoError: IdentifierPath = {
    // We either have any identifier with a tail (ie a period)
    <_head: Identifier>  <mut _tail: IdentifierPathTail>  => {
        _tail.insert(0, new_identifier_path_element_identifier(_head));
        new_identifier_path(_tail)
    },
    // or a single identifier that is not `error`
    <_head: SomeIdentifier<"ErrorKeyword_Unreserved">>  => new_identifier_path(vec![new_identifier_path_element_identifier(<>)]),
};

// These are the attributes that can appear in a state variable but not a function,
// they can work as a limit between these definitions.
SpecialStateVariableAttribute: StateVariableAttribute = {
        <_override_specifier: OverrideSpecifier>  => new_state_variable_attribute_override_specifier(<>),
        <_immutable_keyword: ImmutableKeyword>  => new_state_variable_attribute_immutable_keyword(<>),
        <_transient_keyword: TransientKeyword>  => new_state_variable_attribute_transient_keyword(<>),
};
"#)
                                ),
                                Struct(
                                    name = StateVariableDefinitionValue,
                                    fields =
                                        (equal = Required(Equal), value = Required(Expression))
                                ),
                                Repeated(
                                    name = StateVariableAttributes,
                                    reference = StateVariableAttribute,
                                    allow_empty = true,
                                    // We inline this to resolve a simple conflict, since some of the attributes
                                    // are unreserved (can be identifiers) the parser needs to wait before actually reducing.
                                    parser_options = ParserOptions(inline = true, public = false)
                                ),
                                Enum(
                                    name = StateVariableAttribute,
                                    variants = [
                                        EnumVariant(
                                            reference = OverrideSpecifier,
                                            enabled = From("0.6.0")
                                        ),
                                        EnumVariant(reference = ConstantKeyword),
                                        EnumVariant(reference = InternalKeyword),
                                        EnumVariant(reference = PrivateKeyword),
                                        EnumVariant(reference = PublicKeyword),
                                        EnumVariant(
                                            reference = ImmutableKeyword,
                                            enabled = From("0.6.5")
                                        ),
                                        EnumVariant(
                                            reference = TransientKeyword,
                                            enabled = From("0.8.27")
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
                                        EnumVariant(
                                            reference = OverrideSpecifier,
                                            enabled = From("0.6.0")
                                        ),
                                        EnumVariant(
                                            reference = ConstantKeyword,
                                            enabled = Till("0.5.0")
                                        ),
                                        EnumVariant(reference = ExternalKeyword),
                                        EnumVariant(reference = InternalKeyword),
                                        EnumVariant(reference = PayableKeyword),
                                        EnumVariant(reference = PrivateKeyword),
                                        EnumVariant(reference = PublicKeyword),
                                        EnumVariant(
                                            reference = PureKeyword,
                                            enabled = From("0.4.16")
                                        ),
                                        EnumVariant(
                                            reference = ViewKeyword,
                                            enabled = From("0.4.16")
                                        ),
                                        EnumVariant(
                                            reference = VirtualKeyword,
                                            enabled = From("0.6.0")
                                        )
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
                                        EnumVariant(reference = ExternalKeyword),
                                        EnumVariant(
                                            reference = InternalKeyword,
                                            enabled = Till("0.5.0")
                                        ),
                                        EnumVariant(reference = PayableKeyword),
                                        EnumVariant(
                                            reference = PrivateKeyword,
                                            enabled = Till("0.5.0")
                                        ),
                                        EnumVariant(
                                            reference = PublicKeyword,
                                            enabled = Till("0.5.0")
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
                                        EnumVariant(
                                            reference = OverrideSpecifier,
                                            enabled = From("0.6.0")
                                        ),
                                        EnumVariant(
                                            reference = VirtualKeyword,
                                            enabled = From("0.6.0")
                                        )
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
                                    ),
                                    parser_options = ParserOptions(inline = true, public = false)
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
                                    ],
                                    parser_options = ParserOptions(inline = false, public = false, verbatim = "
// TypeName has two peculiarities:
// 1. We need to parametrize the FunctionRule to allow StateVariableDefinition to disable FunctionTypes without returns.
//    Note that the ArrayTypeName doesn't respect this; most of these manual cases care about trailing constructs, as
//    soon as an extra `[]` is added, the ambiguities disappear.
// 2. A TypeName that can be represented as an `IndexAccessPath` can conflict with a `MemberAccess`, for example
//    `a.b[c]` can be either a member access over an identifier path, or array type, depending on what comes next.
//    This means we need to treat these constructs as IAPs for as long as possible, and only convert them to TypeNames
//    when it's certainly a type. `new_type_name_index_access_path` transforms an IAP into a TypeName.
//    However, since a IAP and an array type conflict, we need to make sure that array types are only matched against
//    base types that are not IAPs, hence the parametric IAPRule.
TypeName0<FunctionRule, IAPRule>: TypeName = {
    <_function_type: FunctionRule> => new_type_name_function_type(<>),
    <_mapping_type: MappingType>  => new_type_name_mapping_type(<>),
    <_index_access_path: IAPRule> => new_type_name_index_access_path(<>),
};
TypeName1<FunctionRule, IAPRule>: TypeName = {
    <_type_name: ArrayTypeName>  => new_type_name_array_type_name(<>),
    <type_name: TypeName0<FunctionRule, IAPRule>>  => <>,
};
TypeName: TypeName = {
    // A regular type can have any function type and an IAP
    <type_name: TypeName1<FunctionType, IndexAccessPath<IdentifierPath>>>  => <>,
};

#[inline]
ArrayTypeName: ArrayTypeName = {
    // The base expression shouldn't end in a trailing IAP, if it does (like `a.b[c]`) it will be
    // handled by `new_type_name_index_access_path` above
    <_type_name: TypeName1<FunctionType, NoIndexAccessPath>>  <_open_bracket: OpenBracket>  <_index: (Expression)?>  <_close_bracket: CloseBracket>  => new_array_type_name(<>),
};

// An empty rule to disable IAPs
NoIndexAccessPath: IndexAccessPath = {};
")
                                ),
                                Struct(
                                    name = FunctionType,
                                    fields = (
                                        function_keyword = Required(FunctionKeyword),
                                        parameters = Required(ParametersDeclaration),
                                        attributes = Required(FunctionTypeAttributes),
                                        returns = Optional(reference = ReturnsDeclaration)
                                    ),
                                    parser_options = ParserOptions(inline = false, public = false, verbatim = "
// The only reason to split FunctionType into two rules is to allow StateVariableDefinition
// to choose whether to allow FunctionTypes without returns or not.
// Note: This could be solved with macros, but is short enough to be explicit
FunctionType: FunctionType = {
    FunctionTypeInternalNoReturn => <>,
    FunctionTypeInternalReturn => <>,
};

FunctionTypeInternalNoReturn: FunctionType = {
    <_function_keyword: FunctionKeyword>  <_parameters: ParametersDeclaration>  <_attributes: FunctionTypeAttributes>   => new_function_type(_function_keyword, _parameters, _attributes, None),
};
FunctionTypeInternalReturn: FunctionType = {
    <_function_keyword: FunctionKeyword>  <_parameters: ParametersDeclaration>  <_attributes: FunctionTypeAttributes>  <_returns: ReturnsDeclaration>  => new_function_type(_function_keyword, _parameters, _attributes, Some(_returns)),
    
};
")
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
                                        EnumVariant(
                                            reference = ConstantKeyword,
                                            enabled = Till("0.5.0")
                                        ),
                                        EnumVariant(
                                            reference = PureKeyword,
                                            enabled = From("0.4.16")
                                        ),
                                        EnumVariant(
                                            reference = ViewKeyword,
                                            enabled = From("0.4.16")
                                        ),
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
                                        name = Optional(
                                            reference = Identifier,
                                            enabled = From("0.8.18")
                                        )
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
                                        name = Optional(
                                            reference = Identifier,
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
                                        EnumVariant(reference = BoolKeyword),
                                        EnumVariant(
                                            reference = ByteKeyword,
                                            enabled = Till("0.8.0")
                                        ),
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
                                        payable_keyword = Optional(
                                            reference = PayableKeyword,
                                            enabled = From("0.5.0")
                                        )
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
                                Repeated(
                                    name = Statements,
                                    reference = Statement,
                                    allow_empty = true
                                ),
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
                                        EnumVariant(
                                            reference = ThrowStatement,
                                            enabled = Till("0.5.0")
                                        ),
                                        EnumVariant(
                                            reference = EmitStatement,
                                            enabled = From("0.4.21")
                                        ),
                                        EnumVariant(
                                            reference = TryStatement,
                                            enabled = From("0.6.0")
                                        ),
                                        EnumVariant(
                                            reference = RevertStatement,
                                            enabled = From("0.8.4")
                                        ),
                                        EnumVariant(reference = AssemblyStatement),
                                        EnumVariant(reference = Block),
                                        EnumVariant(
                                            reference = UncheckedBlock,
                                            enabled = From("0.8.0")
                                        ),
                                        EnumVariant(reference = VariableDeclarationStatement),
                                        EnumVariant(reference = ExpressionStatement)
                                    ],
                                    parser_options = ParserOptions(inline = false, public = false, verbatim = r#"
// There's two issues with `Statement`:
//
// This is a common problem in grammars[1]. To not reinvent the wheel we follow advice from
// LALRPOP community[2].
// Briefly, we parametrize statements on whether they allow a trailing else or not, forcing `else`s to attach to
// the innermost `if`` possible.
//
// [1]: https://en.wikipedia.org/wiki/Dangling_else
// [2]: https://github.com/lalrpop/lalrpop/issues/67#issuecomment-188951041
//
// On top of that, the `revert` keyword is unreserved, meaning that a variable declaration like
// `revert x = ...;` is valid.
// From the perspective of the parser there's an ambiguity when looking at a statement starting with `revert x ...`
//
// To solve it we introduce a special `VariableDeclarationStatement` that handles this case specifically,
// since both this and `RevertStatement` are inlined, the parser doesn't need to reduce until it has seen the
// entire statement.
_Statement<TrailingElse>: Statement = {
    <_if_statement: IfStatement<TrailingElse>>  => new_statement_if_statement(<>),
    <_for_statement: ForStatement<TrailingElse>>  => new_statement_for_statement(<>),
    <_while_statement: WhileStatement<TrailingElse>>  => new_statement_while_statement(<>),
    <_do_while_statement: DoWhileStatement>  => new_statement_do_while_statement(<>),
    <_continue_statement: ContinueStatement>  => new_statement_continue_statement(<>),
    <_break_statement: BreakStatement>  => new_statement_break_statement(<>),
    <_return_statement: ReturnStatement>  => new_statement_return_statement(<>),
    <_emit_statement: EmitStatement>  => new_statement_emit_statement(<>),
    <_try_statement: TryStatement>  => new_statement_try_statement(<>),
    <_revert_statement: RevertStatement>  => new_statement_revert_statement(<>),
    <_assembly_statement: AssemblyStatement>  => new_statement_assembly_statement(<>),
    <_block: Block>  => new_statement_block(<>),
    <_unchecked_block: UncheckedBlock>  => new_statement_unchecked_block(<>),
    <_variable_declaration_statement: VariableDeclarationStatementSpecialRevert>  => new_statement_variable_declaration_statement(<>),
    <_expression_statement: ExpressionStatement>  => new_statement_expression_statement(<>),
};

// By default statements allow dangling `else`s
Statement = _Statement<"True">;

// A VariableDeclarationStatement that has a `revert` type as a special case, this allows
// to disambiguate between a revert statement and a variable declaration with type `revert`
//
// Note: They need to be inline together with `RevertStatement` to actually avoid shift/reduce conflicts
#[inline]
VariableDeclarationStatementSpecialRevert: VariableDeclarationStatement = {
    <_target: VariableDeclarationTargetSpecialRevert>  <_semicolon: Semicolon>  => new_variable_declaration_statement(<>),
};
#[inline]
VariableDeclarationTargetSpecialRevert: VariableDeclarationTarget = {
    <_single_typed_declaration: SingleTypedDeclarationSpecialRevert>  => new_variable_declaration_target_single_typed_declaration(<>),
    <_multi_typed_declaration: MultiTypedDeclaration>  => new_variable_declaration_target_multi_typed_declaration(<>),
};
#[inline]
SingleTypedDeclarationSpecialRevert: SingleTypedDeclaration = {
    <_declaration: VariableDeclarationSpecialRevert>  <_value: (VariableDeclarationValue)?>  => new_single_typed_declaration(<>),
};
#[inline]
VariableDeclarationSpecialRevert: VariableDeclaration = {
    // A regular type that is not `revert`
    //
    // Note: we're tempted to inline TypeNames, but they are recursive, that's why we extract the special case
    <_type_name: TypeName1<FunctionType, IndexAccessPath<IdentifierPathNoRevert>>>  <_storage_location: (StorageLocation)?>  <_name: Identifier>  => new_variable_declaration(<>),
    // The special `revert` type
    <l:@L> L_RevertKeyword_Unreserved <r:@R>  <_storage_location: (StorageLocation)?>  <_name: Identifier>  => {
        let identifier = new_identifier(l..r, source);
        let iap = new_index_access_path_from_identifier_path(new_identifier_path(vec![new_identifier_path_element_identifier(identifier)]));
        let type_name = new_type_name_index_access_path(iap);
        new_variable_declaration(type_name, _storage_location, _name)
    }
};

// An IdentifierPath that cannot be `revert`, used to disambiguate from the `revert` type
#[inline]
IdentifierPathNoRevert: IdentifierPath = {
    // We either have any identifier with a tail (ie a period)
    <_head: Identifier> <mut _tail: IdentifierPathTail>  => {
        _tail.insert(0, new_identifier_path_element_identifier(_head));
        new_identifier_path(_tail)
    },
    // or a single identifier that is not `revert`
    <_head: SomeIdentifier<"RevertKeyword_Unreserved">>  => new_identifier_path(vec![new_identifier_path_element_identifier(<>)]),
};
"#)
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
                                    name = VariableDeclaration,
                                    fields = (
                                        type_name = Required(TypeName),
                                        storage_location = Optional(reference = StorageLocation),
                                        name = Required(Identifier)
                                    )
                                ),
                                Struct(
                                    name = VariableDeclarationStatement,
                                    error_recovery = FieldsErrorRecovery(terminator = semicolon),
                                    fields = (
                                        target = Required(VariableDeclarationTarget),
                                        semicolon = Required(Semicolon)
                                    )
                                ),
                                Enum(
                                    name = VariableDeclarationTarget,
                                    variants = [
                                        EnumVariant(reference = SingleTypedDeclaration),
                                        EnumVariant(reference = MultiTypedDeclaration),
                                        EnumVariant(
                                            reference = UntypedDeclaration,
                                            enabled = Till("0.5.0")
                                        )
                                    ]
                                ),
                                Struct(
                                    name = SingleTypedDeclaration,
                                    fields = (
                                        declaration = Required(VariableDeclaration),
                                        value = Optional(reference = VariableDeclarationValue)
                                    )
                                ),
                                Struct(
                                    name = MultiTypedDeclaration,
                                    error_recovery = FieldsErrorRecovery(
                                        delimiters =
                                            FieldDelimiters(open = open_paren, close = close_paren)
                                    ),
                                    fields = (
                                        open_paren = Required(OpenParen),
                                        elements = Required(MultiTypedDeclarationElements),
                                        close_paren = Required(CloseParen),
                                        value = Required(VariableDeclarationValue)
                                    )
                                ),
                                Separated(
                                    name = MultiTypedDeclarationElements,
                                    reference = MultiTypedDeclarationElement,
                                    separator = Comma,
                                    parser_options = ParserOptions(inline = false, public = false, verbatim = r#"
// MultiTypedDeclaration conflict with tuple expression, for example, the following is ambiguous:
//
// |--------|  => Assignment expression
// |--|        => Tuple expression
// (,,) = foo;
// |--------|  => MultiTypedDeclaration
//
// In order to fix that, we give priority to assignment expressions, except an actual declaration (`uint bar`)
// is seen.
//
// Since they also share a prefix (`(,,,`) we need to have a common prefix rule to avoid reduce/reduce conflicts.
MultiTypedDeclarationElements: MultiTypedDeclarationElements = {
    <prefix: TuplePrefix> <differentiator: VariableDeclaration> <_typed_tuple_deconstruction_element: (Comma <Separated<Comma, <MultiTypedDeclarationElement>>>)?>  => {
        let mut elements = vec![new_multi_typed_declaration_element(None); prefix];
        elements.push(new_multi_typed_declaration_element(Some(differentiator)));
        elements.extend(_typed_tuple_deconstruction_element.unwrap_or(vec![]));
        new_multi_typed_declaration_elements(elements)
    },
    
};

// TuplePrefix counts how many leading commas we have in a tuple deconstruction or
// in a tuple expression, this helps avoid reduce/reduce conflicts
TuplePrefix: usize = {
    // Count how many commas we have at the start, each comma represents an unnamed element
    Comma  <_rest: TuplePrefix>  => 1 + _rest,
    => 0,
};
"#)
                                ),
                                Struct(
                                    name = MultiTypedDeclarationElement,
                                    fields = (member = Optional(reference = VariableDeclaration))
                                ),
                                Struct(
                                    name = UntypedDeclaration,
                                    enabled = Till("0.5.0"),
                                    fields = (
                                        var_keyword = Required(VarKeyword),
                                        names = Required(UntypedDeclarationNames),
                                        value = Required(VariableDeclarationValue)
                                    )
                                ),
                                Enum(
                                    name = UntypedDeclarationNames,
                                    enabled = Till("0.5.0"),
                                    variants = [
                                        EnumVariant(reference = Identifier),
                                        EnumVariant(reference = UntypedTupleDeclaration)
                                    ]
                                ),
                                Struct(
                                    name = UntypedTupleDeclaration,
                                    enabled = Till("0.5.0"),
                                    error_recovery = FieldsErrorRecovery(
                                        delimiters =
                                            FieldDelimiters(open = open_paren, close = close_paren)
                                    ),
                                    fields = (
                                        open_paren = Required(OpenParen),
                                        elements = Required(UntypedTupleDeclarationElements),
                                        close_paren = Required(CloseParen)
                                    )
                                ),
                                Separated(
                                    name = UntypedTupleDeclarationElements,
                                    reference = UntypedTupleDeclarationElement,
                                    separator = Comma,
                                    enabled = Till("0.5.0")
                                ),
                                Struct(
                                    name = UntypedTupleDeclarationElement,
                                    enabled = Till("0.5.0"),
                                    fields = (name = Optional(reference = Identifier))
                                ),
                                Struct(
                                    name = VariableDeclarationValue,
                                    fields = (
                                        equal = Required(Equal),
                                        expression = Required(Expression)
                                    )
                                ),
                                Enum(
                                    name = StorageLocation,
                                    variants = [
                                        EnumVariant(reference = MemoryKeyword),
                                        EnumVariant(reference = StorageKeyword),
                                        EnumVariant(
                                            reference = CallDataKeyword,
                                            enabled = From("0.5.0")
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
                                        if_keyword = Required(IfKeyword),
                                        open_paren = Required(OpenParen),
                                        condition = Required(Expression),
                                        close_paren = Required(CloseParen),
                                        body = Required(Statement),
                                        else_branch = Optional(reference = ElseBranch)
                                    ),
                                    parser_options = ParserOptions(inline = false, public = false, verbatim = r#"
// As explained in the `Statement` rule, this solves the dangling else problem
IfStatement<TrailingElse>: IfStatement = {
    // IfStatement only allows `if`s without an else if TrailingElse == "True"
    <_if_keyword: IfKeyword>  <_open_paren: OpenParen>  <_condition: Expression>  <_close_paren: CloseParen>  <_body: _Statement<"True">> if TrailingElse == "True"  => new_if_statement(<>, None),
    <_if_keyword: IfKeyword>  <_open_paren: OpenParen>  <_condition: Expression>  <_close_paren: CloseParen>  <_body: _Statement<"False">>  <_else_keyword: ElseKeyword>  <_else_branch: _Statement<TrailingElse>>  => new_if_statement(_if_keyword, _open_paren, _condition, _close_paren, _body, Some(new_else_branch(_else_keyword, _else_branch))),
};
"#)
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
                                    ),
                                    parser_options = ParserOptions(inline = false, public = false, verbatim = r#"
// As explained in the `Statement` rule, this solves the dangling else problem
//
// Since a `ForStatement` can have a trailing `Statement` we need to parametrize it as well
ForStatement<TrailingElse>: ForStatement = {
        <_for_keyword: ForKeyword>  <_open_paren: OpenParen>  <_initialization: ForStatementInitialization>  <_condition: ForStatementCondition>  <_iterator: (Expression)?>  <_close_paren: CloseParen>  <_body: _Statement<TrailingElse>>  => new_for_statement(<>),
};"#)
                                ),
                                Enum(
                                    name = ForStatementInitialization,
                                    variants = [
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
                                    ),
                                    parser_options = ParserOptions(inline = false, public = false, verbatim = r#"
// As explained in the `Statement` rule, this solves the dangling else problem
//
// Since a `WhileStatement` can have a trailing `Statement` we need to parametrize it as well
WhileStatement<TrailingElse>: WhileStatement = {
        <_while_keyword: WhileKeyword>  <_open_paren: OpenParen>  <_condition: Expression>  <_close_paren: CloseParen>  <_body: _Statement<TrailingElse>>  => new_while_statement(<>),
};"#)
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
                                    ),
                                    parser_options = ParserOptions(inline = false, public = false, verbatim = r#"
// A try statement conflicts with expressions since an expression can have named arguments (similar to a block)
// at the end. For example, if the parser sees `try foo { a` it doesn't know whether foo is an expression and it should
// start parsing a block (a reduce) or whether it should keep parsing a call options expression (a shift).
//
// We use expressions with a trailing block to solve this, and steal the block from there.
TryStatement: TryStatement = {
    // a `ReturnsDeclaration` acts as a disambiguator
    <_try_keyword: TryKeyword>  <_expression: Expression>  <_returns: ReturnsDeclaration>  <_body: Block>  <_catch_clauses: CatchClauses>  => {
        new_try_statement(_try_keyword, _expression, Some(_returns), _body, _catch_clauses)
    },
    <_try_keyword: TryKeyword>  <_expression: ExpressionTrailingBlock>   <_catch_clauses: CatchClauses>  => {
        let (expr, body) = _expression;
        new_try_statement(_try_keyword, expr, None, body, _catch_clauses)
    },
};

// An expression followed by a block
ExpressionTrailingBlock: (Expression, Block) = {
        <expression: Expression19<Block>>  => <>,
};
"#)
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
                                    ),
                                    // RevertStatement needs to be inline to disambiguate from VariableDeclarationStatement
                                    parser_options = ParserOptions(inline = true, public = false)

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
                                                    model = BinaryRightAssociative,
                                                    fields = (operator = Required(Equal))
                                                ),
                                                PrecedenceOperator(
                                                    model = BinaryRightAssociative,
                                                    fields = (operator = Required(BarEqual))
                                                ),
                                                PrecedenceOperator(
                                                    model = BinaryRightAssociative,
                                                    fields = (operator = Required(PlusEqual))
                                                ),
                                                PrecedenceOperator(
                                                    model = BinaryRightAssociative,
                                                    fields = (operator = Required(MinusEqual))
                                                ),
                                                PrecedenceOperator(
                                                    model = BinaryRightAssociative,
                                                    fields = (operator = Required(CaretEqual))
                                                ),
                                                PrecedenceOperator(
                                                    model = BinaryRightAssociative,
                                                    fields = (operator = Required(SlashEqual))
                                                ),
                                                PrecedenceOperator(
                                                    model = BinaryRightAssociative,
                                                    fields = (operator = Required(PercentEqual))
                                                ),
                                                PrecedenceOperator(
                                                    model = BinaryRightAssociative,
                                                    fields = (operator = Required(AsteriskEqual))
                                                ),
                                                PrecedenceOperator(
                                                    model = BinaryRightAssociative,
                                                    fields = (operator = Required(AmpersandEqual))
                                                ),
                                                PrecedenceOperator(
                                                    model = BinaryRightAssociative,
                                                    fields = (operator =
                                                        Required(LessThanLessThanEqual))
                                                ),
                                                PrecedenceOperator(
                                                    model = BinaryRightAssociative,
                                                    fields = (operator =
                                                        Required(GreaterThanGreaterThanEqual))
                                                ),
                                                PrecedenceOperator(
                                                    model = BinaryRightAssociative,
                                                    fields = (operator = Required(
                                                        GreaterThanGreaterThanGreaterThanEqual
                                                    ))
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
                                                    fields =
                                                        (operator = Required(GreaterThanEqual))
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
                                                    fields =
                                                        (operator = Required(LessThanLessThan))
                                                ),
                                                PrecedenceOperator(
                                                    model = BinaryLeftAssociative,
                                                    fields = (operator =
                                                        Required(GreaterThanGreaterThan))
                                                ),
                                                PrecedenceOperator(
                                                    model = BinaryLeftAssociative,
                                                    fields = (operator = Required(
                                                        GreaterThanGreaterThanGreaterThan
                                                    ))
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
                                                    fields =
                                                        (operator = Required(AsteriskAsterisk))
                                                ),
                                                // In '0.8.0', it became right-associative:
                                                PrecedenceOperator(
                                                    model = BinaryRightAssociative,
                                                    enabled = From("0.8.0"),
                                                    fields =
                                                        (operator = Required(AsteriskAsterisk))
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
                                                fields =
                                                    (arguments = Required(ArgumentsDeclaration))
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
                                                    member = Required(IdentifierPathElement)
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
                                    ],
                                    parser_options = ParserOptions(
                                        inline = false,
                                        public = true,
                                        verbatim = r#"
// Expression has a lot of tricky cases:
// 1. There's conflicts with `TypeName`, for example `a.b[c]` can be either a member access over an identifier path, or
//    an array type, depending on what comes next. As explained on the `TypeName` rule, we need to delay reduction of
//    these constructs as long as possible, only converting them to `Expression` at new_expression_index_access_path.
//    Note: That there's no `ElementaryType` rule, it's handled as an IAP
// 2. But this creates another problem, since we still want to have `MemberAccess` and `IndexAccess` as expressions,
//    but now they conflict with IAP; for example, `a.b` can be either a member access or an IAP, therefore we have to
//    disable member access and index access over IAPs that can be types, we do this by parametrizing the
//    `IndexAccessPathRule`
// 3. Also, new expressions are problematic, since they have a trailing `TypeName`, allowing this example to be parsed
//    in two different ambiguous ways:
//
//        |--------| => IndexAccessExpression
//        |-----|    => NewExpression
//            |-|    => TypeName
//        new foo[]
//            |---|  => TypeName (an array type)
//        |-------|  => NewExpression
//
//    We want to fix this by giving priority to the internal typename, therefore we parametrize over `NewExpressionRule`
//    disallowing `MemberAccess` and `IndexAccess` over new expressions.
// 4. `Expression` are often at a trailing position, for example in storage layout specifiers,
//    to facilitate fixing ambiguities in those cases, we allow `Expression` to be parametrized over a trailing element,
//    that will be matched against when possible, and lifted up through the recursion.
//    This seems to not have any effect, but it allows the parser to postpone reductions.
// 5. Finally, expressions have multiple precedence levels and associativity, we handle this explicitely here.
Expression0<IndexAccessPathRule, NewExpressionRule>: Expression = {
    // The Rule used here is parametric
    <_index_access_path: IndexAccessPathRule> => new_expression_index_access_path(<>),
    // The Rule used here is parametric
    <_new_expression: NewExpressionRule> => new_expression_new_expression(<>),
    <_tuple_expression: TupleExpression>  => new_expression_tuple_expression(<>),
    <_type_expression: TypeExpression>  => new_expression_type_expression(<>),
    <_array_expression: ArrayExpression>  => new_expression_array_expression(<>),
    <_hex_number_expression: HexNumberExpression>  => new_expression_hex_number_expression(<>),
    <_decimal_number_expression: DecimalNumberExpression>  => new_expression_decimal_number_expression(<>),
    <_string_expression: StringExpression>  => new_expression_string_expression(<>),
    <_payable_keyword: PayableKeyword>  => new_expression_payable_keyword(<>),
    <_this_keyword: ThisKeyword>  => new_expression_this_keyword(<>),
    <_super_keyword: SuperKeyword>  => new_expression_super_keyword(<>),
    <_true_keyword: TrueKeyword>  => new_expression_true_keyword(<>),
    <_false_keyword: FalseKeyword>  => new_expression_false_keyword(<>),
};

// An IAP that doesn't match anything
NoIndexAccessPath_Expr: IndexAccessPath = {};

// We simplifiy all these levels of expressions into a single one, there's no need
// for precedence here
Expression1<IndexAccessPathRule, NewExpressionRule>: Expression = {
    // When parsing an index acces expression, the sub expression shouldn't trail in an index access path
    // Nor should it trail on a NewExpression
    <_expression: Expression1<NoIndexAccessPath_Expr, NoNewExpression>>  <_open_bracket: OpenBracket>  <_start: (Expression)?>  <_end: (IndexAccessEnd)?>  <_close_bracket: CloseBracket>  => new_expression_index_access_expression(new_index_access_expression(<>)),
    // When parsing a member access expression, the sub expression shouldn't trail in a path
    // Nor should it trail on a NewExpression
    <_expression: Expression1<IndexAccessPath<NoIdentPath>, NoNewExpression>>  <_period: Period>  <_member: IdentifierPathElement>  => new_expression_member_access_expression(new_member_access_expression(<>)),

    // Both the braces and the arguments declaration serve as markers for disambiguation, therefore
    // resetting the parametric rules.
    <_expression: Expression1<IndexAccessPath<IdentifierPath>, NewExpression>>  <_open_brace: OpenBrace>  <_options: CallOptions>  <_close_brace: CloseBrace>  => new_expression_call_options_expression(new_call_options_expression(<>)),
    <_expression: Expression1<IndexAccessPath<IdentifierPath>, NewExpression>>  <_arguments: ArgumentsDeclaration>  => new_expression_function_call_expression(new_function_call_expression(<>)),

    <expression: Expression0<IndexAccessPathRule, NewExpressionRule>>  => <>,
};

// A Matcher for an empty NewExpression 
NoNewExpression: NewExpression = {};

// Tail is a rule identifying what comes after the expression, whatever is captured is added to the tuple result
Expression5<Tail>: (Expression, Tail) = {
    <_expression_prefix_expression_operator: Expression_PrefixExpression_Operator>  <_expression: Expression5<Tail>>  => {
        let (expr, tail) = _expression;
        (new_expression_prefix_expression(new_prefix_expression(_expression_prefix_expression_operator, expr)), tail)
    },
    
    // A tail can appear just after a postfix or primary expression
    <expression: Expression1<IndexAccessPath<IdentifierPath>, NewExpression>> <tail: Tail> => {
        (expression, tail)
    },
};
Expression6<Tail>: (Expression, Tail) = {
    // This is the only other postfix expression that can overwrite a trailing element
    // Note that the recursive call expects no tail at all
    <_expression: Expression6<EmptyTail>>  <_expression_postfix_expression_operator: Expression_PostfixExpression_Operator> <tail: Tail>  => {
        let (expr, _) = _expression;
        (new_expression_postfix_expression(new_postfix_expression(expr, _expression_postfix_expression_operator)), tail)
    },
    
    <expression: Expression5<Tail>>  => <>,
};
Expression7<Tail>: (Expression, Tail) = {
    // Note that only the right recursive rule matches a tail, the left recursive expects no tail
    <_expression: Expression6<EmptyTail>>  <_operator: Expression_ExponentiationExpression_Operator>  <_expression_2: Expression7<Tail>>  => {
        let (e, _) = _expression;
        let (e2, tail) = _expression_2;
        (new_expression_exponentiation_expression(new_exponentiation_expression(e, _operator, e2)), tail)
    },
    
    <expression: Expression6<Tail>>  => <>,
};
Expression8<Tail>: (Expression, Tail) = {
    <_expression: Expression8<EmptyTail>>  <_expression_multiplicative_expression_operator: Expression_MultiplicativeExpression_Operator>  <_expression_2: Expression7<Tail>>  => {
        let (e, _) = _expression;
        let (e2, tail) = _expression_2;
        (new_expression_multiplicative_expression(new_multiplicative_expression(e, _expression_multiplicative_expression_operator, e2)), tail)
    },
    
    <expression: Expression7<Tail>>  => <>,
};
Expression9<Tail>: (Expression, Tail) = {
    <_expression: Expression9<EmptyTail>>  <_expression_additive_expression_operator: Expression_AdditiveExpression_Operator>  <_expression_2: Expression8<Tail>>  => {
        let (e, _) = _expression;
        let (e2, tail) = _expression_2;
        (new_expression_additive_expression(new_additive_expression(e, _expression_additive_expression_operator, e2)), tail)
    },
    
    <expression: Expression8<Tail>>  => <>,
};
Expression10<Tail>: (Expression, Tail) = {
    <_expression: Expression10<EmptyTail>>  <_expression_shift_expression_operator: Expression_ShiftExpression_Operator>  <_expression_2: Expression9<Tail>>  => {
        let (e, _) = _expression;
        let (e2, tail) = _expression_2;
        (new_expression_shift_expression(new_shift_expression(e, _expression_shift_expression_operator, e2)), tail)
    },
    
    <expression: Expression9<Tail>>  => <>,
};
Expression11<Tail>: (Expression, Tail) = {
    <_expression: Expression11<EmptyTail>>  <_operator: Ampersand>  <_expression_2: Expression10<Tail>>  => {
        let (e, _) = _expression;
        let (e2, tail) = _expression_2;
        (new_expression_bitwise_and_expression(new_bitwise_and_expression(e, _operator, e2)), tail)
    },
    
    <expression: Expression10<Tail>>  => <>,
};
Expression12<Tail>: (Expression, Tail) = {
    <_expression: Expression12<EmptyTail>>  <_operator: Caret>  <_expression_2: Expression11<Tail>>  => {
        let (e, _) = _expression;
        let (e2, tail) = _expression_2;
        (new_expression_bitwise_xor_expression(new_bitwise_xor_expression(e, _operator, e2)), tail)
    },
    
    <expression: Expression11<Tail>>  => <>,
};
Expression13<Tail>: (Expression, Tail) = {
    <_expression: Expression13<EmptyTail>>  <_operator: Bar>  <_expression_2: Expression12<Tail>>  => {
        let (e, _) = _expression;
        let (e2, tail) = _expression_2;
        (new_expression_bitwise_or_expression(new_bitwise_or_expression(e, _operator, e2)), tail)
    },
    
    <expression: Expression12<Tail>>  => <>,
};
Expression14<Tail>: (Expression, Tail) = {
    <_expression: Expression14<EmptyTail>>  <_expression_inequality_expression_operator: Expression_InequalityExpression_Operator>  <_expression_2: Expression13<Tail>>  => {
        let (e, _) = _expression;
        let (e2, tail) = _expression_2;
        (new_expression_inequality_expression(new_inequality_expression(e, _expression_inequality_expression_operator, e2)), tail)
    },
    
    <expression: Expression13<Tail>>  => <>,
};
Expression15<Tail>: (Expression, Tail) = {
    <_expression: Expression15<EmptyTail>>  <_expression_equality_expression_operator: Expression_EqualityExpression_Operator>  <_expression_2: Expression14<Tail>>  => {
        let (e, _) = _expression;
        let (e2, tail) = _expression_2;
        (new_expression_equality_expression(new_equality_expression(e, _expression_equality_expression_operator, e2)), tail)
    },
    
    <expression: Expression14<Tail>>  => <>,
};
Expression16<Tail>: (Expression, Tail) = {
    <_expression: Expression16<EmptyTail>>  <_operator: AmpersandAmpersand>  <_expression_2: Expression15<Tail>>  => {
        let (e, _) = _expression;
        let (e2, tail) = _expression_2;
        (new_expression_and_expression(new_and_expression(e, _operator, e2)), tail)
    },
    
    <expression: Expression15<Tail>>  => <>,
};
Expression17<Tail>: (Expression, Tail) = {
    <_expression: Expression17<EmptyTail>>  <_operator: BarBar>  <_expression_2: Expression16<Tail>>  => {
        let (e, _) = _expression;
        let (e2, tail) = _expression_2;
        (new_expression_or_expression(new_or_expression(e, _operator, e2)), tail)
    },
    
    <expression: Expression16<Tail>>  => <>,
};
Expression18<Tail>: (Expression, Tail) = {
    <_expression: Expression17<EmptyTail>>  <_question_mark: QuestionMark>  <_true_expression: Expression18<EmptyTail>>  <_colon: Colon>  <_false_expression: Expression18<Tail>>  => {
        let (cond_expr, _) = _expression;
        let (true_expr, _) = _true_expression;
        let (false_expr, tail) = _false_expression;
        (new_expression_conditional_expression(new_conditional_expression(cond_expr, _question_mark, true_expr, _colon, false_expr)), tail)
    },
    
    <expression: Expression17<Tail>>  => <>,
};
Expression19<Tail>: (Expression, Tail) = {
    <_expression: Expression19<EmptyTail>>  <_expression_assignment_expression_operator: Expression_AssignmentExpression_Operator>  <_expression_2: Expression18<Tail>>  => {
        let (e, _) = _expression;
        let (e2, tail) = _expression_2;
        (new_expression_assignment_expression(new_assignment_expression(e, _expression_assignment_expression_operator, e2)), tail)
    },
    
    <expression: Expression18<Tail>>  => <>,
};

// Expression is public
pub Expression: Expression = {
    // By default, we expect no tail
    <expression: Expression19<EmptyTail>>  => expression.0,
};

// An empty tail is the default behaviour
EmptyTail: () = {
    () => (),
};

// The different operators are used like choices, and wrapped accordingly
Expression_PrefixExpression_Operator: Expression_PrefixExpression_Operator = {
    <_operator: PlusPlus>  => new_expression_prefix_expression_operator_plus_plus(<>),
    <_operator: MinusMinus>  => new_expression_prefix_expression_operator_minus_minus(<>),
    <_operator: Tilde>  => new_expression_prefix_expression_operator_tilde(<>),
    <_operator: Bang>  => new_expression_prefix_expression_operator_bang(<>),
    <_operator: Minus>  => new_expression_prefix_expression_operator_minus(<>),
    <_operator: DeleteKeyword>  => new_expression_prefix_expression_operator_delete_keyword(<>),
};
Expression_PostfixExpression_Operator: Expression_PostfixExpression_Operator = {
    <_operator: PlusPlus>  => new_expression_postfix_expression_operator_plus_plus(<>),
    <_operator: MinusMinus>  => new_expression_postfix_expression_operator_minus_minus(<>),
};
Expression_ExponentiationExpression_Operator: Expression_ExponentiationExpression_Operator = {
    <_operator: AsteriskAsterisk>  => new_expression_exponentiation_expression_operator_asterisk_asterisk(<>),
};
Expression_MultiplicativeExpression_Operator: Expression_MultiplicativeExpression_Operator = {
    <_operator: Asterisk>  => new_expression_multiplicative_expression_operator_asterisk(<>),
    <_operator: Slash>  => new_expression_multiplicative_expression_operator_slash(<>),
    <_operator: Percent>  => new_expression_multiplicative_expression_operator_percent(<>),
};
Expression_AdditiveExpression_Operator: Expression_AdditiveExpression_Operator = {
    <_operator: Plus>  => new_expression_additive_expression_operator_plus(<>),
    <_operator: Minus>  => new_expression_additive_expression_operator_minus(<>),
};
Expression_ShiftExpression_Operator: Expression_ShiftExpression_Operator = {
    <_operator: LessThanLessThan>  => new_expression_shift_expression_operator_less_than_less_than(<>),
    <_operator: GreaterThanGreaterThan>  => new_expression_shift_expression_operator_greater_than_greater_than(<>),
    <_operator: GreaterThanGreaterThanGreaterThan>  => new_expression_shift_expression_operator_greater_than_greater_than_greater_than(<>),
};
Expression_InequalityExpression_Operator: Expression_InequalityExpression_Operator = {
    <_operator: LessThan>  => new_expression_inequality_expression_operator_less_than(<>),
    <_operator: GreaterThan>  => new_expression_inequality_expression_operator_greater_than(<>),
    <_operator: LessThanEqual>  => new_expression_inequality_expression_operator_less_than_equal(<>),
    <_operator: GreaterThanEqual>  => new_expression_inequality_expression_operator_greater_than_equal(<>),
};
Expression_EqualityExpression_Operator: Expression_EqualityExpression_Operator = {
    <_operator: EqualEqual>  => new_expression_equality_expression_operator_equal_equal(<>),
    <_operator: BangEqual>  => new_expression_equality_expression_operator_bang_equal(<>),
};
Expression_AssignmentExpression_Operator: Expression_AssignmentExpression_Operator = {
    <_operator: Equal>  => new_expression_assignment_expression_operator_equal(<>),
    <_operator: BarEqual>  => new_expression_assignment_expression_operator_bar_equal(<>),
    <_operator: PlusEqual>  => new_expression_assignment_expression_operator_plus_equal(<>),
    <_operator: MinusEqual>  => new_expression_assignment_expression_operator_minus_equal(<>),
    <_operator: CaretEqual>  => new_expression_assignment_expression_operator_caret_equal(<>),
    <_operator: SlashEqual>  => new_expression_assignment_expression_operator_slash_equal(<>),
    <_operator: PercentEqual>  => new_expression_assignment_expression_operator_percent_equal(<>),
    <_operator: AsteriskEqual>  => new_expression_assignment_expression_operator_asterisk_equal(<>),
    <_operator: AmpersandEqual>  => new_expression_assignment_expression_operator_ampersand_equal(<>),
    <_operator: LessThanLessThanEqual>  => new_expression_assignment_expression_operator_less_than_less_than_equal(<>),
    <_operator: GreaterThanGreaterThanEqual>  => new_expression_assignment_expression_operator_greater_than_greater_than_equal(<>),
    <_operator: GreaterThanGreaterThanGreaterThanEqual>  => new_expression_assignment_expression_operator_greater_than_greater_than_greater_than_equal(<>),
};

// A rule matching en empty `IdentifierPath`
NoIdentPath: IdentifierPath = {};

// An Index Access Path that is parametric over the IdentifierPath rule used for member access and index access
IndexAccessPath<IdentPathRule>: IndexAccessPath = {
    // As before, we usually care about trailing constructs, so the brackets serve as markers to reset the parametric rule
    <iap: IndexAccessPath<IdentifierPath>> <_open_bracket: OpenBracket>  <_start: (Expression)?>  <_end: (IndexAccessEnd)?>  <_close_bracket: CloseBracket>  => index_access_path_add_index(<>),
    <IndexAccessPath1<IdentPathRule>>  => <>,
};
IndexAccessPath1<IdentPathRule>: IndexAccessPath = {
    <_identifier: IdentPathRule> => new_index_access_path_from_identifier_path(<>),
    <_elementary_type: ElementaryType>  => new_index_access_path_from_elementary_type(<>),
};
"#
                            )
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
                                        arguments = Required(NamedArgumentGroup),
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
                                    ),
                                    parser_options = ParserOptions(
                                        inline = false,
                                        public = true,
                                        verbatim = r#"
// We avoid function types entirely in new expressions, this is ok since they're not allowed
// in Solidity, but the error will be syntactic rather than semantic, which may be confusing.
//
// We do this to avoid the amibiguity of `try new function () returns (uint) ...`, where the returns clause may be
// parsed either as part of the function type or as part of a try statement.
NewExpression: NewExpression = {
    <_new_keyword: NewKeyword>  <_type_name: TypeName1<NoFunctionType, IndexAccessPath<IdentifierPath>>>  => new_new_expression(<>),
    
};

NoFunctionType: FunctionType = {};
"#)
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
                                    separator = Comma,
                                    parser_options = ParserOptions(
                                        inline = false,
                                        public = false,
                                        verbatim = r#"
// See the comment around TupleDeconstructionStatement for an explanation of this rule.
#[inline]
TupleValues: TupleValues = {
    <prefix: TuplePrefix> => {
        // We need to add an extra empty element here, since the trailing comma indicates
        // an additional empty tuple value.
        let elements = vec![new_tuple_value(None); prefix + 1];
        new_tuple_values(elements)
    },
    <prefix: TuplePrefix> <differentiator: Expression> <_tuple_value: (Comma <Separated<Comma, <TupleValue>>>)?>  => {
        let mut elements = vec![new_tuple_value(None); prefix];
        elements.push(new_tuple_value(Some(differentiator)));
        elements.extend(_tuple_value.unwrap_or(vec![]));
        new_tuple_values(elements)
    },
    
};
"#)
                                ),
                                Struct(
                                    name = TupleValue,
                                    fields = (expression = Optional(reference = Expression))
                                ),
                                Struct(
                                    name = ArrayExpression,
                                    error_recovery = FieldsErrorRecovery(
                                        delimiters = FieldDelimiters(
                                            open = open_bracket,
                                            close = close_bracket
                                        )
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
                                        unit = Optional(
                                            reference = NumberUnit,
                                            enabled = Till("0.5.0")
                                        )
                                    ),
                                    parser_options = ParserOptions(
                                        inline = false,
                                        public = false,
                                        verbatim = "
// This rule shouldn't be manual, but the node constructor takes an optional argument that is not
// enabled in 0.8.30, therefore we don't have it automatically generated
HexNumberExpression: HexNumberExpression = {
        <_literal: HexLiteral>  => new_hex_number_expression(<>, None),
};"
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
                                    definitions = [TokenDefinition(Sequence([
                                        Atom("0"),
                                        Choice([Atom("x"), Atom("X")]),
                                        OneOrMore(Fragment(HexCharacter)),
                                        ZeroOrMore(Sequence([
                                            Atom("_"),
                                            OneOrMore(Fragment(HexCharacter))
                                        ]))
                                    ]))]
                                ),
                                Token(
                                    name = DecimalLiteral,
                                    definitions = [
                                        TokenDefinition(Sequence([
                                            Fragment(DecimalDigits),
                                            Optional(Sequence([
                                                Atom("."),
                                                Optional(Fragment(DecimalDigits))
                                            ])),
                                            Optional(Fragment(DecimalExponent))
                                        ])),
                                        TokenDefinition(Sequence([
                                            Atom("."),
                                            Fragment(DecimalDigits),
                                            Optional(Fragment(DecimalExponent))
                                        ]))
                                    ]
                                ),
                                Fragment(
                                    name = DecimalDigits,
                                    scanner = Sequence([
                                        OneOrMore(Range(
                                            inclusive_start = '0',
                                            inclusive_end = '9'
                                        )),
                                        ZeroOrMore(Sequence([
                                            Atom("_"),
                                            OneOrMore(Range(
                                                inclusive_start = '0',
                                                inclusive_end = '9'
                                            ))
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
                                        EnumVariant(
                                            reference = GweiKeyword,
                                            enabled = From("0.6.11")
                                        ),
                                        // 1e-6 ETH
                                        EnumVariant(
                                            reference = SzaboKeyword,
                                            enabled = Till("0.7.0")
                                        ),
                                        // 1e-3 ETH
                                        EnumVariant(
                                            reference = FinneyKeyword,
                                            enabled = Till("0.7.0")
                                        ),
                                        // 1 ETH
                                        EnumVariant(reference = EtherKeyword),
                                        EnumVariant(reference = SecondsKeyword),
                                        EnumVariant(reference = MinutesKeyword),
                                        EnumVariant(reference = HoursKeyword),
                                        EnumVariant(reference = DaysKeyword),
                                        EnumVariant(reference = WeeksKeyword),
                                        EnumVariant(
                                            reference = YearsKeyword,
                                            enabled = Till("0.5.0")
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
                                        EnumVariant(
                                            reference = StringLiteral,
                                            enabled = Till("0.5.14")
                                        ),
                                        EnumVariant(
                                            reference = StringLiterals,
                                            enabled = From("0.5.14")
                                        ),
                                        EnumVariant(
                                            reference = HexStringLiteral,
                                            enabled = Till("0.5.14")
                                        ),
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
                                    definitions = [TokenDefinition(Sequence([
                                        Atom("'"),
                                        ZeroOrMore(Choice([
                                            Fragment(EscapeSequence),
                                            Not(['\'', '\\', '\r', '\n'])
                                        ])),
                                        Atom("'")
                                    ]))]
                                ),
                                Token(
                                    name = DoubleQuotedStringLiteral,
                                    definitions = [TokenDefinition(Sequence([
                                        Atom("\""),
                                        ZeroOrMore(Choice([
                                            Fragment(EscapeSequence),
                                            Not(['"', '\\', '\r', '\n'])
                                        ])),
                                        Atom("\"")
                                    ]))]
                                ),
                                Fragment(
                                    name = EscapeSequence,
                                    scanner = Sequence([
                                        Atom("\\"),
                                        Choice([
                                            Not(['x', 'u']),
                                            Fragment(HexByteEscape),
                                            Fragment(UnicodeEscape)
                                        ])
                                    ])
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
                                    definitions = [TokenDefinition(Sequence([
                                        Atom("hex'"),
                                        Optional(Fragment(HexStringContents)),
                                        Atom("'")
                                    ]))]
                                ),
                                Token(
                                    name = DoubleQuotedHexStringLiteral,
                                    definitions = [TokenDefinition(Sequence([
                                        Atom("hex\""),
                                        Optional(Fragment(HexStringContents)),
                                        Atom("\"")
                                    ]))]
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
                                    enabled = From("0.7.0"),
                                    definitions = [TokenDefinition(Sequence([
                                        Atom("unicode'"),
                                        ZeroOrMore(Choice([
                                            Fragment(UnicodeEscapeSequence),
                                            Not(['\'', '\\', '\r', '\n'])
                                        ])),
                                        Atom("'")
                                    ]))]
                                ),
                                Token(
                                    name = DoubleQuotedUnicodeStringLiteral,
                                    enabled = From("0.7.0"),
                                    definitions = [TokenDefinition(Sequence([
                                        Atom("unicode\""),
                                        ZeroOrMore(Choice([
                                            Fragment(UnicodeEscapeSequence),
                                            Not(['"', '\\', '\r', '\n'])
                                        ])),
                                        Atom("\"")
                                    ]))]
                                ),
                                Fragment(
                                    name = UnicodeEscapeSequence,
                                    enabled = From("0.7.0"),
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
                                    enabled = From("0.7.0"),
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
                                // Since an identifier path can include the reserved keyword `address` as parth of the path
                                // we use `IdentifierPathElement` to represent each element, instead of `Identifier`.
                                Separated(
                                    name = IdentifierPath,
                                    reference = IdentifierPathElement,
                                    separator = Period,
                                    parser_options = ParserOptions(inline = false, public = false, verbatim = r#"
// We need to force this to differentiate the first element from not being
// an `AddressKeyword`
IdentifierPath: IdentifierPath = {
    <_head: Identifier>  <mut _tail: (IdentifierPathTail)?>  => {
        match _tail {
            Some(mut tail) => {
                tail.insert(0, new_identifier_path_element_identifier(_head));
                new_identifier_path(tail)
            },
            None => new_identifier_path(vec![new_identifier_path_element_identifier(_head)]),
        }
    },
    
};
IdentifierPathTail: Vec<IdentifierPathElement> = {
    Period  <_elements: IdentifierPathTailElements>  => <>,
    
};
IdentifierPathTailElements: Vec<IdentifierPathElement> = {
    <_member_access_identifier: Separated<Period, <IdentifierPathElement>>>  => <>,
    
};
"#)

                                ),
                                Enum(
                                    // An element of an identifier path can be either an identifier or the reserved `address` keyword
                                    // Note: This is also used on `MemberAccessExpression`
                                    name = IdentifierPathElement,
                                    variants = [
                                        EnumVariant(reference = Identifier),
                                        EnumVariant(
                                            reference = AddressKeyword,
                                            enabled = From("0.6.0")
                                        )
                                    ]
                                ),
                                Token(
                                    name = Identifier,
                                    definitions = [TokenDefinition(Sequence([
                                        Fragment(IdentifierStart),
                                        ZeroOrMore(Fragment(IdentifierPart))
                                    ]))]
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
                )
            ]
        ),
        LexicalContext(
            name = Yul,
            identifier_token = YulIdentifier,
            sections = [Section(
                title = "Yul",
                topics = [
                    Topic(
                        title = "Yul Statements",
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
                                ),
                                // TODO(v2): Until the lexer can perform context switching, we ignore YulBlocks
                                parser_options = ParserOptions(inline = false, public = false, verbatim = r#"
YulBlock: YulBlock = {
    <_open_brace: OpenBrace>  !  <_close_brace: CloseBrace>  => {
        new_yul_block(_open_brace, new_yul_statements(vec![]), _close_brace)
    }
};
"#)
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
                                    EnumVariant(
                                        reference = YulLeaveStatement,
                                        enabled = From("0.6.0")
                                    ),
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
                                    EnumVariant(
                                        reference = YulColonAndEqual,
                                        enabled = Till("0.5.5")
                                    )
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
                        items = [
                            Precedence(
                                name = YulExpression,
                                precedence_expressions = [PrecedenceExpression(
                                    name = YulFunctionCallExpression,
                                    operators = [PrecedenceOperator(
                                        model = Postfix,
                                        error_recovery = FieldsErrorRecovery(
                                            delimiters = FieldDelimiters(
                                                open = open_paren,
                                                close = close_paren
                                            )
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
                                definitions = [TokenDefinition(Sequence([
                                    Fragment(IdentifierStart),
                                    ZeroOrMore(Choice([Fragment(IdentifierPart), Atom(".")]))
                                ]))]
                            ),
                            Enum(
                                name = YulLiteral,
                                variants = [
                                    EnumVariant(
                                        reference = YulTrueKeyword,
                                        enabled = From("0.6.2")
                                    ),
                                    EnumVariant(
                                        reference = YulFalseKeyword,
                                        enabled = From("0.6.2")
                                    ),
                                    EnumVariant(reference = YulDecimalLiteral),
                                    EnumVariant(reference = YulHexLiteral),
                                    EnumVariant(reference = HexStringLiteral),
                                    EnumVariant(reference = StringLiteral)
                                ]
                            ),
                            Token(
                                name = YulDecimalLiteral,
                                definitions = [TokenDefinition(Choice([
                                    Atom("0"),
                                    Sequence([
                                        Range(inclusive_start = '1', inclusive_end = '9'),
                                        ZeroOrMore(Range(
                                            inclusive_start = '0',
                                            inclusive_end = '9'
                                        ))
                                    ])
                                ]))]
                            ),
                            Token(
                                name = YulHexLiteral,
                                definitions = [TokenDefinition(Sequence([
                                    Atom("0x"),
                                    OneOrMore(Fragment(HexCharacter))
                                ]))]
                            )
                        ]
                    ),
                    Topic(
                        title = "Yul Keywords",
                        items = [
                            Keyword(
                                name = YulAbstractKeyword,
                                enabled = Never,
                                definitions = [KeywordDefinition(
                                    reserved = Till("0.7.1"),
                                    value = Atom("abstract")
                                )]
                            ),
                            Keyword(
                                name = YulAfterKeyword,
                                enabled = Never,
                                definitions = [KeywordDefinition(
                                    reserved = Till("0.7.1"),
                                    value = Atom("after")
                                )]
                            ),
                            Keyword(
                                name = YulAliasKeyword,
                                enabled = Never,
                                definitions = [KeywordDefinition(
                                    reserved = Range(from = "0.5.0", till = "0.7.1"),
                                    value = Atom("alias")
                                )]
                            ),
                            Keyword(
                                name = YulAnonymousKeyword,
                                enabled = Never,
                                definitions = [KeywordDefinition(
                                    reserved = Till("0.7.1"),
                                    value = Atom("anonymous")
                                )]
                            ),
                            Keyword(
                                name = YulApplyKeyword,
                                enabled = Never,
                                definitions = [KeywordDefinition(
                                    reserved = Range(from = "0.5.0", till = "0.7.1"),
                                    value = Atom("apply")
                                )]
                            ),
                            Keyword(
                                name = YulAsKeyword,
                                enabled = Never,
                                definitions = [KeywordDefinition(
                                    reserved = Till("0.7.1"),
                                    value = Atom("as")
                                )]
                            ),
                            Keyword(
                                name = YulAssemblyKeyword,
                                enabled = Never,
                                definitions = [KeywordDefinition(
                                    reserved = Till("0.7.1"),
                                    value = Atom("assembly")
                                )]
                            ),
                            Keyword(
                                name = YulAutoKeyword,
                                enabled = Never,
                                definitions = [KeywordDefinition(
                                    reserved = Range(from = "0.5.0", till = "0.7.1"),
                                    value = Atom("auto")
                                )]
                            ),
                            Keyword(
                                name = YulBoolKeyword,
                                enabled = Never,
                                definitions = [KeywordDefinition(
                                    reserved = Till("0.5.10"),
                                    value = Atom("bool")
                                )]
                            ),
                            Keyword(
                                name = YulBreakKeyword,
                                definitions = [KeywordDefinition(value = Atom("break"))]
                            ),
                            Keyword(
                                name = YulBytesKeyword,
                                enabled = Never,
                                definitions = [KeywordDefinition(
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
                                enabled = Never,
                                definitions = [KeywordDefinition(
                                    reserved = Range(from = "0.5.0", till = "0.7.1"),
                                    value = Atom("calldata")
                                )]
                            ),
                            Keyword(
                                name = YulCaseKeyword,
                                definitions = [KeywordDefinition(value = Atom("case"))]
                            ),
                            Keyword(
                                name = YulCatchKeyword,
                                enabled = Never,
                                definitions = [KeywordDefinition(
                                    reserved = Till("0.7.1"),
                                    value = Atom("catch")
                                )]
                            ),
                            Keyword(
                                name = YulConstantKeyword,
                                enabled = Never,
                                definitions = [KeywordDefinition(
                                    reserved = Till("0.7.1"),
                                    value = Atom("constant")
                                )]
                            ),
                            Keyword(
                                name = YulConstructorKeyword,
                                enabled = Never,
                                definitions = [KeywordDefinition(
                                    reserved = Range(from = "0.5.0", till = "0.7.1"),
                                    value = Atom("constructor")
                                )]
                            ),
                            Keyword(
                                name = YulContinueKeyword,
                                definitions = [KeywordDefinition(value = Atom("continue"))]
                            ),
                            Keyword(
                                name = YulContractKeyword,
                                enabled = Never,
                                definitions = [KeywordDefinition(
                                    reserved = Till("0.7.1"),
                                    value = Atom("contract")
                                )]
                            ),
                            Keyword(
                                name = YulCopyOfKeyword,
                                enabled = Never,
                                definitions = [KeywordDefinition(
                                    reserved = Range(from = "0.5.0", till = "0.7.1"),
                                    value = Atom("copyof")
                                )]
                            ),
                            Keyword(
                                name = YulDaysKeyword,
                                enabled = Never,
                                definitions = [KeywordDefinition(
                                    reserved = Till("0.7.1"),
                                    value = Atom("days")
                                )]
                            ),
                            Keyword(
                                name = YulDefaultKeyword,
                                definitions = [KeywordDefinition(value = Atom("default"))]
                            ),
                            Keyword(
                                name = YulDefineKeyword,
                                enabled = Never,
                                definitions = [KeywordDefinition(
                                    reserved = Range(from = "0.5.0", till = "0.7.1"),
                                    value = Atom("define")
                                )]
                            ),
                            Keyword(
                                name = YulDeleteKeyword,
                                enabled = Never,
                                definitions = [KeywordDefinition(
                                    reserved = Till("0.7.1"),
                                    value = Atom("delete")
                                )]
                            ),
                            Keyword(
                                name = YulDoKeyword,
                                enabled = Never,
                                definitions = [KeywordDefinition(
                                    reserved = Till("0.7.1"),
                                    value = Atom("do")
                                )]
                            ),
                            Keyword(
                                name = YulElseKeyword,
                                enabled = Never,
                                definitions = [KeywordDefinition(
                                    reserved = Till("0.7.1"),
                                    value = Atom("else")
                                )]
                            ),
                            Keyword(
                                name = YulEmitKeyword,
                                enabled = Never,
                                definitions = [KeywordDefinition(
                                    reserved = Range(from = "0.5.0", till = "0.7.1"),
                                    value = Atom("emit")
                                )]
                            ),
                            Keyword(
                                name = YulEnumKeyword,
                                enabled = Never,
                                definitions = [KeywordDefinition(
                                    reserved = Till("0.7.1"),
                                    value = Atom("enum")
                                )]
                            ),
                            Keyword(
                                name = YulEtherKeyword,
                                enabled = Never,
                                definitions = [KeywordDefinition(
                                    reserved = Till("0.7.1"),
                                    value = Atom("ether")
                                )]
                            ),
                            Keyword(
                                name = YulEventKeyword,
                                enabled = Never,
                                definitions = [KeywordDefinition(
                                    reserved = Till("0.7.1"),
                                    value = Atom("event")
                                )]
                            ),
                            Keyword(
                                name = YulExternalKeyword,
                                enabled = Never,
                                definitions = [KeywordDefinition(
                                    reserved = Till("0.7.1"),
                                    value = Atom("external")
                                )]
                            ),
                            Keyword(
                                name = YulFallbackKeyword,
                                enabled = Never,
                                definitions = [KeywordDefinition(
                                    reserved = Range(from = "0.6.0", till = "0.7.1"),
                                    value = Atom("fallback")
                                )]
                            ),
                            Keyword(
                                name = YulFalseKeyword,
                                enabled = From("0.6.2"),
                                definitions = [KeywordDefinition(value = Atom("false"))]
                            ),
                            Keyword(
                                name = YulFinalKeyword,
                                enabled = Never,
                                definitions = [KeywordDefinition(
                                    reserved = Till("0.7.1"),
                                    value = Atom("final")
                                )]
                            ),
                            Keyword(
                                name = YulFinneyKeyword,
                                enabled = Never,
                                definitions = [KeywordDefinition(
                                    reserved = Till("0.7.0"),
                                    value = Atom("finney")
                                )]
                            ),
                            Keyword(
                                name = YulFixedKeyword,
                                enabled = Never,
                                definitions = [
                                    KeywordDefinition(
                                        reserved = Till("0.7.1"),
                                        value = Atom("fixed")
                                    ),
                                    KeywordDefinition(
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
                                definitions = [KeywordDefinition(value = Atom("for"))]
                            ),
                            Keyword(
                                name = YulFunctionKeyword,
                                definitions = [KeywordDefinition(value = Atom("function"))]
                            ),
                            Keyword(
                                name = YulGweiKeyword,
                                enabled = Never,
                                definitions = [KeywordDefinition(
                                    reserved = Range(from = "0.7.0", till = "0.7.1"),
                                    value = Atom("gwei")
                                )]
                            ),
                            Keyword(
                                name = YulHexKeyword,
                                enabled = Never,
                                definitions = [KeywordDefinition(value = Atom("hex"))]
                            ),
                            Keyword(
                                name = YulHoursKeyword,
                                enabled = Never,
                                definitions = [KeywordDefinition(
                                    reserved = Till("0.7.1"),
                                    value = Atom("hours")
                                )]
                            ),
                            Keyword(
                                name = YulIfKeyword,
                                definitions = [KeywordDefinition(value = Atom("if"))]
                            ),
                            Keyword(
                                name = YulImmutableKeyword,
                                enabled = Never,
                                definitions = [KeywordDefinition(
                                    reserved = Range(from = "0.5.0", till = "0.7.1"),
                                    value = Atom("immutable")
                                )]
                            ),
                            Keyword(
                                name = YulImplementsKeyword,
                                enabled = Never,
                                definitions = [KeywordDefinition(
                                    reserved = Range(from = "0.5.0", till = "0.7.1"),
                                    value = Atom("implements")
                                )]
                            ),
                            Keyword(
                                name = YulImportKeyword,
                                enabled = Never,
                                definitions = [KeywordDefinition(
                                    reserved = Till("0.7.1"),
                                    value = Atom("import")
                                )]
                            ),
                            Keyword(
                                name = YulIndexedKeyword,
                                enabled = Never,
                                definitions = [KeywordDefinition(
                                    reserved = Till("0.7.1"),
                                    value = Atom("indexed")
                                )]
                            ),
                            Keyword(
                                name = YulInKeyword,
                                enabled = Never,
                                definitions = [KeywordDefinition(
                                    reserved = Till("0.6.8"),
                                    value = Atom("in")
                                )]
                            ),
                            Keyword(
                                name = YulInlineKeyword,
                                enabled = Never,
                                definitions = [KeywordDefinition(
                                    reserved = Till("0.7.1"),
                                    value = Atom("inline")
                                )]
                            ),
                            Keyword(
                                name = YulInterfaceKeyword,
                                enabled = Never,
                                definitions = [KeywordDefinition(
                                    reserved = Till("0.7.1"),
                                    value = Atom("interface")
                                )]
                            ),
                            Keyword(
                                name = YulInternalKeyword,
                                enabled = Never,
                                definitions = [KeywordDefinition(
                                    reserved = Till("0.7.1"),
                                    value = Atom("internal")
                                )]
                            ),
                            Keyword(
                                name = YulIntKeyword,
                                enabled = Never,
                                definitions = [KeywordDefinition(
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
                                enabled = Never,
                                definitions = [KeywordDefinition(
                                    reserved = Till("0.7.1"),
                                    value = Atom("is")
                                )]
                            ),
                            Keyword(
                                name = YulLeaveKeyword,
                                enabled = From("0.6.0"),
                                definitions = [KeywordDefinition(
                                    reserved = From("0.7.1"),
                                    value = Atom("leave")
                                )]
                            ),
                            Keyword(
                                name = YulLetKeyword,
                                definitions = [KeywordDefinition(value = Atom("let"))]
                            ),
                            Keyword(
                                name = YulLibraryKeyword,
                                enabled = Never,
                                definitions = [KeywordDefinition(
                                    reserved = Till("0.7.1"),
                                    value = Atom("library")
                                )]
                            ),
                            Keyword(
                                name = YulMacroKeyword,
                                enabled = Never,
                                definitions = [KeywordDefinition(
                                    reserved = Range(from = "0.5.0", till = "0.7.1"),
                                    value = Atom("macro")
                                )]
                            ),
                            Keyword(
                                name = YulMappingKeyword,
                                enabled = Never,
                                definitions = [KeywordDefinition(
                                    reserved = Till("0.7.1"),
                                    value = Atom("mapping")
                                )]
                            ),
                            Keyword(
                                name = YulMatchKeyword,
                                enabled = Never,
                                definitions = [KeywordDefinition(
                                    reserved = Till("0.7.1"),
                                    value = Atom("match")
                                )]
                            ),
                            Keyword(
                                name = YulMemoryKeyword,
                                enabled = Never,
                                definitions = [KeywordDefinition(
                                    reserved = Till("0.7.1"),
                                    value = Atom("memory")
                                )]
                            ),
                            Keyword(
                                name = YulMinutesKeyword,
                                enabled = Never,
                                definitions = [KeywordDefinition(
                                    reserved = Till("0.7.1"),
                                    value = Atom("minutes")
                                )]
                            ),
                            Keyword(
                                name = YulModifierKeyword,
                                enabled = Never,
                                definitions = [KeywordDefinition(
                                    reserved = Till("0.7.1"),
                                    value = Atom("modifier")
                                )]
                            ),
                            Keyword(
                                name = YulMutableKeyword,
                                enabled = Never,
                                definitions = [KeywordDefinition(
                                    reserved = Range(from = "0.5.0", till = "0.7.1"),
                                    value = Atom("mutable")
                                )]
                            ),
                            Keyword(
                                name = YulNewKeyword,
                                enabled = Never,
                                definitions = [KeywordDefinition(
                                    reserved = Till("0.7.1"),
                                    value = Atom("new")
                                )]
                            ),
                            Keyword(
                                name = YulNullKeyword,
                                enabled = Never,
                                definitions = [KeywordDefinition(
                                    reserved = Till("0.7.1"),
                                    value = Atom("null")
                                )]
                            ),
                            Keyword(
                                name = YulOfKeyword,
                                enabled = Never,
                                definitions = [KeywordDefinition(
                                    reserved = Till("0.7.1"),
                                    value = Atom("of")
                                )]
                            ),
                            Keyword(
                                name = YulOverrideKeyword,
                                enabled = Never,
                                definitions = [KeywordDefinition(
                                    reserved = Range(from = "0.5.0", till = "0.7.1"),
                                    value = Atom("override")
                                )]
                            ),
                            Keyword(
                                name = YulPartialKeyword,
                                enabled = Never,
                                definitions = [KeywordDefinition(
                                    reserved = Range(from = "0.5.0", till = "0.7.1"),
                                    value = Atom("partial")
                                )]
                            ),
                            Keyword(
                                name = YulPayableKeyword,
                                enabled = Never,
                                definitions = [KeywordDefinition(
                                    reserved = Till("0.7.1"),
                                    value = Atom("payable")
                                )]
                            ),
                            Keyword(
                                name = YulPragmaKeyword,
                                enabled = Never,
                                definitions = [KeywordDefinition(
                                    reserved = Till("0.7.1"),
                                    value = Atom("pragma")
                                )]
                            ),
                            Keyword(
                                name = YulPrivateKeyword,
                                enabled = Never,
                                definitions = [KeywordDefinition(
                                    reserved = Till("0.7.1"),
                                    value = Atom("private")
                                )]
                            ),
                            Keyword(
                                name = YulPromiseKeyword,
                                enabled = Never,
                                definitions = [KeywordDefinition(
                                    reserved = Range(from = "0.5.0", till = "0.7.1"),
                                    value = Atom("promise")
                                )]
                            ),
                            Keyword(
                                name = YulPublicKeyword,
                                enabled = Never,
                                definitions = [KeywordDefinition(
                                    reserved = Till("0.7.1"),
                                    value = Atom("public")
                                )]
                            ),
                            Keyword(
                                name = YulPureKeyword,
                                enabled = Never,
                                definitions = [KeywordDefinition(
                                    reserved = Till("0.7.1"),
                                    value = Atom("pure")
                                )]
                            ),
                            Keyword(
                                name = YulReceiveKeyword,
                                enabled = Never,
                                definitions = [KeywordDefinition(
                                    reserved = Range(from = "0.6.0", till = "0.7.1"),
                                    value = Atom("receive")
                                )]
                            ),
                            Keyword(
                                name = YulReferenceKeyword,
                                enabled = Never,
                                definitions = [KeywordDefinition(
                                    reserved = Range(from = "0.5.0", till = "0.7.1"),
                                    value = Atom("reference")
                                )]
                            ),
                            Keyword(
                                name = YulRelocatableKeyword,
                                enabled = Never,
                                definitions = [KeywordDefinition(
                                    reserved = Till("0.7.1"),
                                    value = Atom("relocatable")
                                )]
                            ),
                            Keyword(
                                name = YulReturnsKeyword,
                                enabled = Never,
                                definitions = [KeywordDefinition(
                                    reserved = Till("0.7.1"),
                                    value = Atom("returns")
                                )]
                            ),
                            Keyword(
                                name = YulSealedKeyword,
                                enabled = Never,
                                definitions = [KeywordDefinition(
                                    reserved = Range(from = "0.5.0", till = "0.7.1"),
                                    value = Atom("sealed")
                                )]
                            ),
                            Keyword(
                                name = YulSecondsKeyword,
                                enabled = Never,
                                definitions = [KeywordDefinition(
                                    reserved = Till("0.7.1"),
                                    value = Atom("seconds")
                                )]
                            ),
                            Keyword(
                                name = YulSizeOfKeyword,
                                enabled = Never,
                                definitions = [KeywordDefinition(
                                    reserved = Range(from = "0.5.0", till = "0.7.1"),
                                    value = Atom("sizeof")
                                )]
                            ),
                            Keyword(
                                name = YulStaticKeyword,
                                enabled = Never,
                                definitions = [KeywordDefinition(
                                    reserved = Till("0.7.1"),
                                    value = Atom("static")
                                )]
                            ),
                            Keyword(
                                name = YulStorageKeyword,
                                enabled = Never,
                                definitions = [KeywordDefinition(
                                    reserved = Till("0.7.1"),
                                    value = Atom("storage")
                                )]
                            ),
                            Keyword(
                                name = YulStringKeyword,
                                enabled = Never,
                                definitions = [KeywordDefinition(
                                    reserved = Till("0.7.1"),
                                    value = Atom("string")
                                )]
                            ),
                            Keyword(
                                name = YulStructKeyword,
                                enabled = Never,
                                definitions = [KeywordDefinition(
                                    reserved = Till("0.7.1"),
                                    value = Atom("struct")
                                )]
                            ),
                            Keyword(
                                name = YulSuperKeyword,
                                enabled = Never,
                                definitions = [KeywordDefinition(
                                    reserved = From("0.8.0"),
                                    value = Atom("super")
                                )]
                            ),
                            Keyword(
                                name = YulSupportsKeyword,
                                enabled = Never,
                                definitions = [KeywordDefinition(
                                    reserved = Range(from = "0.5.0", till = "0.7.1"),
                                    value = Atom("supports")
                                )]
                            ),
                            Keyword(
                                name = YulSwitchKeyword,
                                definitions = [KeywordDefinition(value = Atom("switch"))]
                            ),
                            Keyword(
                                name = YulSzaboKeyword,
                                enabled = Never,
                                definitions = [KeywordDefinition(
                                    reserved = Till("0.7.0"),
                                    value = Atom("szabo")
                                )]
                            ),
                            Keyword(
                                name = YulThisKeyword,
                                enabled = Never,
                                definitions = [KeywordDefinition(
                                    reserved = From("0.8.0"),
                                    value = Atom("this")
                                )]
                            ),
                            Keyword(
                                name = YulThrowKeyword,
                                enabled = Never,
                                definitions = [KeywordDefinition(
                                    reserved = Till("0.7.1"),
                                    value = Atom("throw")
                                )]
                            ),
                            Keyword(
                                name = YulTrueKeyword,
                                enabled = From("0.6.2"),
                                definitions = [KeywordDefinition(value = Atom("true"))]
                            ),
                            Keyword(
                                name = YulTryKeyword,
                                enabled = Never,
                                definitions = [KeywordDefinition(
                                    reserved = Till("0.7.1"),
                                    value = Atom("try")
                                )]
                            ),
                            Keyword(
                                name = YulTypeDefKeyword,
                                enabled = Never,
                                definitions = [KeywordDefinition(
                                    reserved = Range(from = "0.5.0", till = "0.7.1"),
                                    value = Atom("typedef")
                                )]
                            ),
                            Keyword(
                                name = YulTypeKeyword,
                                enabled = Never,
                                definitions = [KeywordDefinition(
                                    reserved = Till("0.7.1"),
                                    value = Atom("type")
                                )]
                            ),
                            Keyword(
                                name = YulTypeOfKeyword,
                                enabled = Never,
                                definitions = [KeywordDefinition(
                                    reserved = Till("0.7.1"),
                                    value = Atom("typeof")
                                )]
                            ),
                            Keyword(
                                name = YulUfixedKeyword,
                                enabled = Never,
                                definitions = [
                                    KeywordDefinition(
                                        reserved = Till("0.7.1"),
                                        value = Atom("ufixed")
                                    ),
                                    KeywordDefinition(
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
                                enabled = Never,
                                definitions = [KeywordDefinition(
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
                                enabled = Never,
                                definitions = [KeywordDefinition(
                                    reserved = Range(from = "0.5.0", till = "0.7.1"),
                                    value = Atom("unchecked")
                                )]
                            ),
                            Keyword(
                                name = YulUsingKeyword,
                                enabled = Never,
                                definitions = [KeywordDefinition(
                                    reserved = Till("0.7.1"),
                                    value = Atom("using")
                                )]
                            ),
                            Keyword(
                                name = YulVarKeyword,
                                enabled = Never,
                                definitions = [KeywordDefinition(
                                    reserved = Till("0.6.5"),
                                    value = Atom("var")
                                )]
                            ),
                            Keyword(
                                name = YulViewKeyword,
                                enabled = Never,
                                definitions = [KeywordDefinition(
                                    reserved = Till("0.7.1"),
                                    value = Atom("view")
                                )]
                            ),
                            Keyword(
                                name = YulVirtualKeyword,
                                enabled = Never,
                                definitions = [KeywordDefinition(
                                    reserved = Range(from = "0.6.0", till = "0.7.1"),
                                    value = Atom("virtual")
                                )]
                            ),
                            Keyword(
                                name = YulWeeksKeyword,
                                enabled = Never,
                                definitions = [KeywordDefinition(
                                    reserved = Till("0.7.1"),
                                    value = Atom("weeks")
                                )]
                            ),
                            Keyword(
                                name = YulWeiKeyword,
                                enabled = Never,
                                definitions = [KeywordDefinition(
                                    reserved = Till("0.7.1"),
                                    value = Atom("wei")
                                )]
                            ),
                            Keyword(
                                name = YulWhileKeyword,
                                enabled = Never,
                                definitions = [KeywordDefinition(
                                    reserved = Till("0.7.1"),
                                    value = Atom("while")
                                )]
                            ),
                            Keyword(
                                name = YulYearsKeyword,
                                enabled = Never,
                                definitions = [KeywordDefinition(
                                    reserved = Till("0.7.1"),
                                    value = Atom("years")
                                )]
                            )
                        ]
                    )
                ]
            )]
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
