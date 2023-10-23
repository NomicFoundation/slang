pub use solidity::SolidityDefinition;

codegen_language_macros::compile!(Language(
    name = Solidity,
    root_item = SourceUnit,
    leading_trivia = ZeroOrMore(Choice([
        Token(Whitespace),
        Token(EndOfLine),
        Token(SingleLineComment),
        Token(MultilineComment)
    ])),
    trailing_trivia = Sequence([
        Optional(Token(Whitespace)),
        Optional(Token(SingleLineComment)),
        Optional(Token(EndOfLine))
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
                                EnumVariant(
                                    name = Pragma,
                                    fields = (directive = Required(NonTerminal(PragmaDirective)))
                                ),
                                EnumVariant(
                                    name = Import,
                                    fields = (directive = Required(NonTerminal(ImportDirective)))
                                ),
                                EnumVariant(
                                    name = Contract,
                                    fields =
                                        (definition = Required(NonTerminal(ContractDefinition)))
                                ),
                                EnumVariant(
                                    name = Interface,
                                    fields =
                                        (definition = Required(NonTerminal(InterfaceDefinition)))
                                ),
                                EnumVariant(
                                    name = Library,
                                    fields =
                                        (definition = Required(NonTerminal(LibraryDefinition)))
                                ),
                                EnumVariant(
                                    name = Struct,
                                    enabled_in = "0.6.0",
                                    fields = (definition = Required(NonTerminal(StructDefinition)))
                                ),
                                EnumVariant(
                                    name = Enum,
                                    enabled_in = "0.6.0",
                                    fields = (definition = Required(NonTerminal(EnumDefinition)))
                                ),
                                EnumVariant(
                                    name = Function,
                                    enabled_in = "0.7.1",
                                    fields =
                                        (definition = Required(NonTerminal(FunctionDefinition)))
                                ),
                                EnumVariant(
                                    name = Constant,
                                    enabled_in = "0.7.4",
                                    fields =
                                        (definition = Required(NonTerminal(ConstantDefinition)))
                                ),
                                EnumVariant(
                                    name = Error,
                                    enabled_in = "0.8.4",
                                    fields = (definition = Required(NonTerminal(ErrorDefinition)))
                                ),
                                EnumVariant(
                                    name = UserDefinedValueType,
                                    enabled_in = "0.8.8",
                                    fields = (definition =
                                        Required(NonTerminal(UserDefinedValueTypeDefinition)))
                                ),
                                EnumVariant(
                                    name = Using,
                                    enabled_in = "0.8.13",
                                    fields = (directive = Required(NonTerminal(UsingDirective)))
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
                            error_recovery = [Terminator(semicolon)],
                            fields = (
                                pragma_keyword = Required(Terminal([PragmaKeyword])),
                                pragma = Required(NonTerminal(Pragma)),
                                semicolon = Required(Terminal([Semicolon]))
                            )
                        ),
                        Enum(
                            name = Pragma,
                            variants = [
                                EnumVariant(
                                    name = ABICoder,
                                    fields = (
                                        abicoder_keyword = Required(Terminal([AbicoderKeyword])),
                                        version = Required(Terminal([Identifier]))
                                    )
                                ),
                                EnumVariant(
                                    name = Experimental,
                                    fields = (
                                        experimental_keyword =
                                            Required(Terminal([ExperimentalKeyword])),
                                        feature =
                                            Required(Terminal([AsciiStringLiteral, Identifier]))
                                    )
                                ),
                                EnumVariant(
                                    name = Version,
                                    fields = (
                                        solidity_keyword = Required(Terminal([SolidityKeyword])),
                                        expressions =
                                            Required(NonTerminal(VersionPragmaExpressions))
                                    )
                                )
                            ]
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
                            error_recovery = [Terminator(semicolon)],
                            fields = (
                                import_keyword = Required(Terminal([ImportKeyword])),
                                symbol = Required(NonTerminal(ImportSymbol)),
                                semicolon = Required(Terminal([Semicolon]))
                            )
                        ),
                        Enum(
                            name = ImportSymbol,
                            variants = [
                                EnumVariant(
                                    name = Path,
                                    fields = (
                                        path = Required(Terminal([AsciiStringLiteral])),
                                        alias = Optional(kind = NonTerminal(ImportAlias))
                                    )
                                ),
                                EnumVariant(
                                    name = Named,
                                    fields = (
                                        asterisk = Required(Terminal([Asterisk])),
                                        alias = Required(NonTerminal(ImportAlias)),
                                        from_keyword = Required(Terminal([FromKeyword])),
                                        path = Required(Terminal([AsciiStringLiteral]))
                                    )
                                ),
                                EnumVariant(
                                    name = Deconstruction,
                                    error_recovery =
                                        [Delimiters(open = open_brace, close = close_brace)],
                                    fields = (
                                        open_brace = Required(Terminal([OpenBrace])),
                                        fields = Required(NonTerminal(ImportDeconstructionFields)),
                                        close_brace = Required(Terminal([CloseBrace])),
                                        from_keyword = Required(Terminal([FromKeyword])),
                                        path = Required(Terminal([AsciiStringLiteral]))
                                    )
                                )
                            ]
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
                            error_recovery = [Terminator(semicolon)],
                            fields = (
                                using_keyword = Required(Terminal([UsingKeyword])),
                                symbol = Required(NonTerminal(UsingSymbol)),
                                for_keyword = Required(Terminal([ForKeyword])),
                                target = Required(NonTerminal(UsingTarget)),
                                global_keyword = Optional(
                                    kind = Terminal([GlobalKeyword]),
                                    enabled_in = "0.8.13"
                                ),
                                semicolon = Required(Terminal([Semicolon]))
                            )
                        ),
                        Enum(
                            name = UsingSymbol,
                            variants = [
                                EnumVariant(
                                    name = Path,
                                    fields = (path = Required(NonTerminal(IdentifierPath)))
                                ),
                                EnumVariant(
                                    name = Deconstruction,
                                    enabled_in = "0.8.13",
                                    error_recovery =
                                        [Delimiters(open = open_brace, close = close_brace)],
                                    fields = (
                                        open_brace = Required(Terminal([OpenBrace])),
                                        fields = Required(NonTerminal(UsingDeconstructionFields)),
                                        close_brace = Required(Terminal([CloseBrace]))
                                    )
                                )
                            ]
                        ),
                        Separated(
                            name = UsingDeconstructionFields,
                            separated = UsingDeconstructionField,
                            separator = Comma,
                            enabled_in = "0.8.13",
                            allow_empty = true
                        ),
                        Struct(
                            name = UsingDeconstructionField,
                            enabled_in = "0.8.13",
                            fields = (
                                name = Required(NonTerminal(IdentifierPath)),
                                alias =
                                    Optional(kind = NonTerminal(UsingAlias), enabled_in = "0.8.19")
                            )
                        ),
                        Struct(
                            name = UsingAlias,
                            enabled_in = "0.8.19",
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
                                EnumVariant(
                                    name = TypeName,
                                    fields = (type_name = Required(NonTerminal(TypeName)))
                                ),
                                EnumVariant(
                                    name = Asterisk,
                                    fields = (asterisk = Required(Terminal([Asterisk])))
                                )
                            ]
                        )
                    ]
                ),
                Topic(
                    title = "Trivia",
                    items = [
                        Token(
                            name = Whitespace,
                            definitions = [TokenDefinition(
                                scanner = OneOrMore(Choice([Atom(" "), Atom("\t")]))
                            )]
                        ),
                        Token(
                            name = EndOfLine,
                            definitions = [TokenDefinition(
                                scanner = Sequence([Optional(Atom("\r")), Atom("\n")])
                            )]
                        ),
                        Token(
                            name = MultilineComment,
                            definitions = [TokenDefinition(
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
                            )]
                        ),
                        Token(
                            name = SingleLineComment,
                            definitions = [TokenDefinition(
                                scanner = Sequence([Atom("//"), ZeroOrMore(Not(['\r', '\n']))])
                            )]
                        )
                    ]
                ),
                Topic(
                    title = "Keywords",
                    items = [
                        Keyword(
                            name = AbicoderKeyword,
                            identifier = Identifier,
                            value = Atom("abicoder")
                        ),
                        Keyword(
                            name = AbstractKeyword,
                            identifier = Identifier,
                            enabled_in = "0.6.0",
                            value = Atom("abstract")
                        ),
                        Keyword(
                            name = AddressKeyword,
                            identifier = Identifier,
                            value = Atom("address")
                        ),
                        Keyword(
                            name = AfterKeyword,
                            identifier = Identifier,
                            disabled_in = "0.4.11",
                            value = Atom("after")
                        ),
                        Keyword(
                            name = AliasKeyword,
                            identifier = Identifier,
                            disabled_in = "0.4.11",
                            reserved_in = "0.5.0",
                            value = Atom("alias")
                        ),
                        Keyword(
                            name = AnonymousKeyword,
                            identifier = Identifier,
                            value = Atom("anonymous")
                        ),
                        Keyword(
                            name = ApplyKeyword,
                            identifier = Identifier,
                            disabled_in = "0.4.11",
                            reserved_in = "0.5.0",
                            value = Atom("apply")
                        ),
                        Keyword(
                            name = AsKeyword,
                            identifier = Identifier,
                            value = Atom("as")
                        ),
                        Keyword(
                            name = AssemblyKeyword,
                            identifier = Identifier,
                            value = Atom("assembly")
                        ),
                        Keyword(
                            name = AutoKeyword,
                            identifier = Identifier,
                            disabled_in = "0.4.11",
                            reserved_in = "0.5.0",
                            value = Atom("auto")
                        ),
                        Keyword(
                            name = BoolKeyword,
                            identifier = Identifier,
                            value = Atom("bool")
                        ),
                        Keyword(
                            name = BreakKeyword,
                            identifier = Identifier,
                            value = Atom("break")
                        ),
                        Keyword(
                            name = ByteKeyword,
                            identifier = Identifier,
                            disabled_in = "0.8.0",
                            value = Atom("byte")
                        ),
                        Keyword(
                            name = BytesKeyword,
                            identifier = Identifier,
                            value = Sequence([
                                Atom("bytes"),
                                Range(inclusive_start = 1, inclusive_end = 32, increment = 1)
                            ])
                        ),
                        Keyword(
                            name = CallDataKeyword,
                            identifier = Identifier,
                            enabled_in = "0.5.0",
                            value = Atom("calldata")
                        ),
                        Keyword(
                            name = CaseKeyword,
                            identifier = Identifier,
                            disabled_in = "0.4.11",
                            value = Atom("case")
                        ),
                        Keyword(
                            name = CatchKeyword,
                            identifier = Identifier,
                            enabled_in = "0.6.0",
                            value = Atom("catch")
                        ),
                        Keyword(
                            name = ConstantKeyword,
                            identifier = Identifier,
                            value = Atom("constant")
                        ),
                        Keyword(
                            name = ConstructorKeyword,
                            identifier = Identifier,
                            enabled_in = "0.4.22",
                            value = Atom("constructor")
                        ),
                        Keyword(
                            name = ContinueKeyword,
                            identifier = Identifier,
                            value = Atom("continue")
                        ),
                        Keyword(
                            name = ContractKeyword,
                            identifier = Identifier,
                            value = Atom("contract")
                        ),
                        Keyword(
                            name = CopyOfKeyword,
                            identifier = Identifier,
                            disabled_in = "0.4.11",
                            reserved_in = "0.5.0",
                            value = Atom("copyof")
                        ),
                        Keyword(
                            name = DaysKeyword,
                            identifier = Identifier,
                            value = Atom("days")
                        ),
                        Keyword(
                            name = DefaultKeyword,
                            identifier = Identifier,
                            disabled_in = "0.4.11",
                            value = Atom("default")
                        ),
                        Keyword(
                            name = DefineKeyword,
                            identifier = Identifier,
                            disabled_in = "0.4.11",
                            reserved_in = "0.5.0",
                            value = Atom("define")
                        ),
                        Keyword(
                            name = DeleteKeyword,
                            identifier = Identifier,
                            value = Atom("delete")
                        ),
                        Keyword(
                            name = DoKeyword,
                            identifier = Identifier,
                            value = Atom("do")
                        ),
                        Keyword(
                            name = ElseKeyword,
                            identifier = Identifier,
                            value = Atom("else")
                        ),
                        Keyword(
                            name = EmitKeyword,
                            identifier = Identifier,
                            enabled_in = "0.4.21",
                            value = Atom("emit")
                        ),
                        Keyword(
                            name = EnumKeyword,
                            identifier = Identifier,
                            value = Atom("enum")
                        ),
                        Keyword(
                            name = ErrorKeyword,
                            identifier = Identifier,
                            enabled_in = "0.8.4",
                            value = Atom("error")
                        ),
                        Keyword(
                            name = EtherKeyword,
                            identifier = Identifier,
                            value = Atom("ether")
                        ),
                        Keyword(
                            name = EventKeyword,
                            identifier = Identifier,
                            value = Atom("event")
                        ),
                        Keyword(
                            name = ExperimentalKeyword,
                            identifier = Identifier,
                            value = Atom("experimental")
                        ),
                        Keyword(
                            name = ExternalKeyword,
                            identifier = Identifier,
                            value = Atom("external")
                        ),
                        Keyword(
                            name = FallbackKeyword,
                            identifier = Identifier,
                            value = Atom("fallback")
                        ),
                        Keyword(
                            name = FalseKeyword,
                            identifier = Identifier,
                            value = Atom("false")
                        ),
                        Keyword(
                            name = FinalKeyword,
                            identifier = Identifier,
                            disabled_in = "0.4.11",
                            value = Atom("final")
                        ),
                        Keyword(
                            name = FinneyKeyword,
                            identifier = Identifier,
                            disabled_in = "0.7.0",
                            value = Atom("finney")
                        ),
                        Keyword(
                            name = FixedKeyword,
                            identifier = Identifier,
                            value = Sequence([
                                Atom("fixed"),
                                Optional(Sequence([
                                    Range(inclusive_start = 8, inclusive_end = 256, increment = 8),
                                    Atom("x"),
                                    Range(inclusive_start = 0, inclusive_end = 80, increment = 1)
                                ]))
                            ])
                        ),
                        Keyword(
                            name = ForKeyword,
                            identifier = Identifier,
                            value = Atom("for")
                        ),
                        Keyword(
                            name = FromKeyword,
                            identifier = Identifier,
                            value = Atom("from")
                        ),
                        Keyword(
                            name = FunctionKeyword,
                            identifier = Identifier,
                            value = Atom("function")
                        ),
                        Keyword(
                            name = GlobalKeyword,
                            identifier = Identifier,
                            enabled_in = "0.8.13",
                            value = Atom("global")
                        ),
                        Keyword(
                            name = GweiKeyword,
                            identifier = Identifier,
                            enabled_in = "0.6.11",
                            value = Atom("gwei")
                        ),
                        Keyword(
                            name = HexKeyword,
                            identifier = Identifier,
                            disabled_in = "0.4.11",
                            value = Atom("hex")
                        ),
                        Keyword(
                            name = HoursKeyword,
                            identifier = Identifier,
                            value = Atom("hours")
                        ),
                        Keyword(
                            name = IfKeyword,
                            identifier = Identifier,
                            value = Atom("if")
                        ),
                        Keyword(
                            name = ImmutableKeyword,
                            identifier = Identifier,
                            enabled_in = "0.6.5",
                            value = Atom("immutable")
                        ),
                        Keyword(
                            name = ImplementsKeyword,
                            identifier = Identifier,
                            disabled_in = "0.4.11",
                            reserved_in = "0.5.0",
                            value = Atom("implements")
                        ),
                        Keyword(
                            name = ImportKeyword,
                            identifier = Identifier,
                            value = Atom("import")
                        ),
                        Keyword(
                            name = IndexedKeyword,
                            identifier = Identifier,
                            value = Atom("indexed")
                        ),
                        Keyword(
                            name = InKeyword,
                            identifier = Identifier,
                            disabled_in = "0.4.11",
                            value = Atom("in")
                        ),
                        Keyword(
                            name = InlineKeyword,
                            identifier = Identifier,
                            disabled_in = "0.4.11",
                            value = Atom("inline")
                        ),
                        Keyword(
                            name = InterfaceKeyword,
                            identifier = Identifier,
                            value = Atom("interface")
                        ),
                        Keyword(
                            name = InternalKeyword,
                            identifier = Identifier,
                            value = Atom("internal")
                        ),
                        Keyword(
                            name = IntKeyword,
                            identifier = Identifier,
                            value = Sequence([
                                Atom("int"),
                                Optional(Range(
                                    inclusive_start = 8,
                                    inclusive_end = 256,
                                    increment = 8
                                ))
                            ])
                        ),
                        Keyword(
                            name = IsKeyword,
                            identifier = Identifier,
                            value = Atom("is")
                        ),
                        Keyword(
                            name = LetKeyword,
                            identifier = Identifier,
                            disabled_in = "0.4.11",
                            value = Atom("let")
                        ),
                        Keyword(
                            name = LibraryKeyword,
                            identifier = Identifier,
                            value = Atom("library")
                        ),
                        Keyword(
                            name = MacroKeyword,
                            identifier = Identifier,
                            disabled_in = "0.4.11",
                            reserved_in = "0.5.0",
                            value = Atom("macro")
                        ),
                        Keyword(
                            name = MappingKeyword,
                            identifier = Identifier,
                            value = Atom("mapping")
                        ),
                        Keyword(
                            name = MatchKeyword,
                            identifier = Identifier,
                            disabled_in = "0.4.11",
                            value = Atom("match")
                        ),
                        Keyword(
                            name = MemoryKeyword,
                            identifier = Identifier,
                            value = Atom("memory")
                        ),
                        Keyword(
                            name = MinutesKeyword,
                            identifier = Identifier,
                            value = Atom("minutes")
                        ),
                        Keyword(
                            name = ModifierKeyword,
                            identifier = Identifier,
                            value = Atom("modifier")
                        ),
                        Keyword(
                            name = MutableKeyword,
                            identifier = Identifier,
                            disabled_in = "0.4.11",
                            reserved_in = "0.5.0",
                            value = Atom("mutable")
                        ),
                        Keyword(
                            name = NewKeyword,
                            identifier = Identifier,
                            value = Atom("new")
                        ),
                        Keyword(
                            name = NullKeyword,
                            identifier = Identifier,
                            disabled_in = "0.4.11",
                            value = Atom("null")
                        ),
                        Keyword(
                            name = OfKeyword,
                            identifier = Identifier,
                            disabled_in = "0.4.11",
                            value = Atom("of")
                        ),
                        Keyword(
                            name = OverrideKeyword,
                            identifier = Identifier,
                            value = Atom("override")
                        ),
                        Keyword(
                            name = PartialKeyword,
                            identifier = Identifier,
                            disabled_in = "0.4.11",
                            reserved_in = "0.5.0",
                            value = Atom("partial")
                        ),
                        Keyword(
                            name = PayableKeyword,
                            identifier = Identifier,
                            value = Atom("payable")
                        ),
                        Keyword(
                            name = PragmaKeyword,
                            identifier = Identifier,
                            value = Atom("pragma")
                        ),
                        Keyword(
                            name = PrivateKeyword,
                            identifier = Identifier,
                            value = Atom("private")
                        ),
                        Keyword(
                            name = PromiseKeyword,
                            identifier = Identifier,
                            disabled_in = "0.4.11",
                            reserved_in = "0.5.0",
                            value = Atom("promise")
                        ),
                        Keyword(
                            name = PublicKeyword,
                            identifier = Identifier,
                            value = Atom("public")
                        ),
                        Keyword(
                            name = PureKeyword,
                            identifier = Identifier,
                            value = Atom("pure")
                        ),
                        Keyword(
                            name = ReceiveKeyword,
                            identifier = Identifier,
                            value = Atom("receive")
                        ),
                        Keyword(
                            name = ReferenceKeyword,
                            identifier = Identifier,
                            disabled_in = "0.4.11",
                            reserved_in = "0.5.0",
                            value = Atom("reference")
                        ),
                        Keyword(
                            name = RelocatableKeyword,
                            identifier = Identifier,
                            disabled_in = "0.4.11",
                            value = Atom("relocatable")
                        ),
                        Keyword(
                            name = ReturnKeyword,
                            identifier = Identifier,
                            value = Atom("return")
                        ),
                        Keyword(
                            name = ReturnsKeyword,
                            identifier = Identifier,
                            value = Atom("returns")
                        ),
                        Keyword(
                            name = RevertKeyword,
                            identifier = Identifier,
                            enabled_in = "0.8.4",
                            value = Atom("revert")
                        ),
                        Keyword(
                            name = SealedKeyword,
                            identifier = Identifier,
                            disabled_in = "0.4.11",
                            reserved_in = "0.5.0",
                            value = Atom("sealed")
                        ),
                        Keyword(
                            name = SecondsKeyword,
                            identifier = Identifier,
                            value = Atom("seconds")
                        ),
                        Keyword(
                            name = SizeOfKeyword,
                            identifier = Identifier,
                            disabled_in = "0.4.11",
                            reserved_in = "0.5.0",
                            value = Atom("sizeof")
                        ),
                        Keyword(
                            name = SolidityKeyword,
                            identifier = Identifier,
                            value = Atom("solidity")
                        ),
                        Keyword(
                            name = StaticKeyword,
                            identifier = Identifier,
                            disabled_in = "0.4.11",
                            value = Atom("static")
                        ),
                        Keyword(
                            name = StorageKeyword,
                            identifier = Identifier,
                            value = Atom("storage")
                        ),
                        Keyword(
                            name = StringKeyword,
                            identifier = Identifier,
                            value = Atom("string")
                        ),
                        Keyword(
                            name = StructKeyword,
                            identifier = Identifier,
                            value = Atom("struct")
                        ),
                        Keyword(
                            name = SupportsKeyword,
                            identifier = Identifier,
                            disabled_in = "0.4.11",
                            reserved_in = "0.5.0",
                            value = Atom("supports")
                        ),
                        Keyword(
                            name = SwitchKeyword,
                            identifier = Identifier,
                            disabled_in = "0.4.11",
                            value = Atom("switch")
                        ),
                        Keyword(
                            name = SzaboKeyword,
                            identifier = Identifier,
                            disabled_in = "0.7.0",
                            value = Atom("szabo")
                        ),
                        Keyword(
                            name = ThrowKeyword,
                            identifier = Identifier,
                            disabled_in = "0.5.0",
                            value = Atom("throw")
                        ),
                        Keyword(
                            name = TrueKeyword,
                            identifier = Identifier,
                            value = Atom("true")
                        ),
                        Keyword(
                            name = TryKeyword,
                            identifier = Identifier,
                            enabled_in = "0.6.0",
                            value = Atom("try")
                        ),
                        Keyword(
                            name = TypeDefKeyword,
                            identifier = Identifier,
                            disabled_in = "0.4.11",
                            reserved_in = "0.5.0",
                            value = Atom("typedef")
                        ),
                        Keyword(
                            name = TypeKeyword,
                            identifier = Identifier,
                            enabled_in = "0.5.3",
                            value = Atom("type")
                        ),
                        Keyword(
                            name = TypeOfKeyword,
                            identifier = Identifier,
                            disabled_in = "0.4.11",
                            value = Atom("typeof")
                        ),
                        Keyword(
                            name = UfixedKeyword,
                            identifier = Identifier,
                            value = Sequence([
                                Atom("ufixed"),
                                Optional(Sequence([
                                    Range(inclusive_start = 8, inclusive_end = 256, increment = 8),
                                    Atom("x"),
                                    Range(inclusive_start = 0, inclusive_end = 80, increment = 1)
                                ]))
                            ])
                        ),
                        Keyword(
                            name = UintKeyword,
                            identifier = Identifier,
                            value = Sequence([
                                Atom("uint"),
                                Optional(Range(
                                    inclusive_start = 8,
                                    inclusive_end = 256,
                                    increment = 8
                                ))
                            ])
                        ),
                        Keyword(
                            name = UncheckedKeyword,
                            identifier = Identifier,
                            enabled_in = "0.8.0",
                            value = Atom("unchecked")
                        ),
                        Keyword(
                            name = UsingKeyword,
                            identifier = Identifier,
                            value = Atom("using")
                        ),
                        Keyword(
                            name = VarKeyword,
                            identifier = Identifier,
                            disabled_in = "0.5.0",
                            value = Atom("var")
                        ),
                        Keyword(
                            name = ViewKeyword,
                            identifier = Identifier,
                            value = Atom("view")
                        ),
                        Keyword(
                            name = VirtualKeyword,
                            identifier = Identifier,
                            enabled_in = "0.6.0",
                            value = Atom("virtual")
                        ),
                        Keyword(
                            name = WeeksKeyword,
                            identifier = Identifier,
                            value = Atom("weeks")
                        ),
                        Keyword(
                            name = WeiKeyword,
                            identifier = Identifier,
                            value = Atom("wei")
                        ),
                        Keyword(
                            name = WhileKeyword,
                            identifier = Identifier,
                            value = Atom("while")
                        ),
                        Keyword(
                            name = YearsKeyword,
                            identifier = Identifier,
                            disabled_in = "0.5.0",
                            value = Atom("years")
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
                            error_recovery = [Delimiters(open = open_brace, close = close_brace)],
                            fields = (
                                abstract_keyword = Optional(
                                    kind = Terminal([AbstractKeyword]),
                                    enabled_in = "0.6.0"
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
                                EnumVariant(
                                    name = Using,
                                    fields = (directive = Required(NonTerminal(UsingDirective)))
                                ),
                                EnumVariant(
                                    name = Function,
                                    fields =
                                        (definition = Required(NonTerminal(FunctionDefinition)))
                                ),
                                EnumVariant(
                                    name = Constructor,
                                    enabled_in = "0.4.22",
                                    fields =
                                        (definition = Required(NonTerminal(ConstructorDefinition)))
                                ),
                                EnumVariant(
                                    name = ReceiveFunction,
                                    enabled_in = "0.6.0",
                                    fields = (definition =
                                        Required(NonTerminal(ReceiveFunctionDefinition)))
                                ),
                                EnumVariant(
                                    name = FallbackFunction,
                                    enabled_in = "0.6.0",
                                    fields = (definition =
                                        Required(NonTerminal(FallbackFunctionDefinition)))
                                ),
                                EnumVariant(
                                    name = UnnamedFunction,
                                    disabled_in = "0.6.0",
                                    fields = (definition =
                                        Required(NonTerminal(UnnamedFunctionDefinition)))
                                ),
                                EnumVariant(
                                    name = Modifier,
                                    fields =
                                        (definition = Required(NonTerminal(ModifierDefinition)))
                                ),
                                EnumVariant(
                                    name = Struct,
                                    fields = (definition = Required(NonTerminal(StructDefinition)))
                                ),
                                EnumVariant(
                                    name = Enum,
                                    fields = (definition = Required(NonTerminal(EnumDefinition)))
                                ),
                                EnumVariant(
                                    name = Event,
                                    fields = (definition = Required(NonTerminal(EventDefinition)))
                                ),
                                EnumVariant(
                                    name = StateVariable,
                                    fields = (definition =
                                        Required(NonTerminal(StateVariableDefinition)))
                                ),
                                EnumVariant(
                                    name = Error,
                                    enabled_in = "0.8.4",
                                    fields = (definition = Required(NonTerminal(ErrorDefinition)))
                                ),
                                EnumVariant(
                                    name = UserDefinedValueType,
                                    enabled_in = "0.8.8",
                                    fields = (definition =
                                        Required(NonTerminal(UserDefinedValueTypeDefinition)))
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
                            error_recovery = [Delimiters(open = open_brace, close = close_brace)],
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
                            error_recovery = [Delimiters(open = open_brace, close = close_brace)],
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
                            error_recovery = [Delimiters(open = open_brace, close = close_brace)],
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
                            error_recovery = [Terminator(semicolon)],
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
                            error_recovery = [Delimiters(open = open_brace, close = close_brace)],
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
                        enabled_in = "0.7.4",
                        error_recovery = [Terminator(semicolon)],
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
                            error_recovery = [Terminator(semicolon)],
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
                                EnumVariant(
                                    name = Override,
                                    fields = (specifier = Required(NonTerminal(OverrideSpecifier)))
                                ),
                                EnumVariant(
                                    name = Constant,
                                    fields = (keyword = Required(Terminal([ConstantKeyword])))
                                ),
                                EnumVariant(
                                    name = Internal,
                                    fields = (keyword = Required(Terminal([InternalKeyword])))
                                ),
                                EnumVariant(
                                    name = Private,
                                    fields = (keyword = Required(Terminal([PrivateKeyword])))
                                ),
                                EnumVariant(
                                    name = Public,
                                    fields = (keyword = Required(Terminal([PublicKeyword])))
                                ),
                                EnumVariant(
                                    name = Immutable,
                                    enabled_in = "0.6.5",
                                    fields = (keyword = Required(Terminal([ImmutableKeyword])))
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
                            error_recovery = [Delimiters(open = open_paren, close = close_paren)],
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
                                EnumVariant(
                                    name = Modifier,
                                    fields = (modifier = Required(NonTerminal(ModifierInvocation)))
                                ),
                                EnumVariant(
                                    name = Override,
                                    fields = (specifier = Required(NonTerminal(OverrideSpecifier)))
                                ),
                                EnumVariant(
                                    name = Constant,
                                    disabled_in = "0.5.0",
                                    fields = (keyword = Required(Terminal([ConstantKeyword])))
                                ),
                                EnumVariant(
                                    name = External,
                                    fields = (keyword = Required(Terminal([ExternalKeyword])))
                                ),
                                EnumVariant(
                                    name = Internal,
                                    fields = (keyword = Required(Terminal([InternalKeyword])))
                                ),
                                EnumVariant(
                                    name = Payable,
                                    fields = (keyword = Required(Terminal([PayableKeyword])))
                                ),
                                EnumVariant(
                                    name = Private,
                                    fields = (keyword = Required(Terminal([PrivateKeyword])))
                                ),
                                EnumVariant(
                                    name = Public,
                                    fields = (keyword = Required(Terminal([PublicKeyword])))
                                ),
                                EnumVariant(
                                    name = Pure,
                                    fields = (keyword = Required(Terminal([PureKeyword])))
                                ),
                                EnumVariant(
                                    name = View,
                                    fields = (keyword = Required(Terminal([ViewKeyword])))
                                ),
                                EnumVariant(
                                    name = Virtual,
                                    enabled_in = "0.6.0",
                                    fields = (keyword = Required(Terminal([VirtualKeyword])))
                                )
                            ]
                        ),
                        Struct(
                            name = OverrideSpecifier,
                            error_recovery = [Delimiters(open = open_paren, close = close_paren)],
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
                                EnumVariant(
                                    name = Block,
                                    fields = (block = Required(NonTerminal(Block)))
                                ),
                                EnumVariant(
                                    name = None,
                                    fields = (semicolon = Required(Terminal([Semicolon])))
                                )
                            ]
                        ),
                        Struct(
                            name = ConstructorDefinition,
                            enabled_in = "0.4.22",
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
                            enabled_in = "0.4.22",
                            allow_empty = true
                        ),
                        Enum(
                            name = ConstructorAttribute,
                            enabled_in = "0.4.22",
                            variants = [
                                EnumVariant(
                                    name = Modifier,
                                    fields = (modifier = Required(NonTerminal(ModifierInvocation)))
                                ),
                                EnumVariant(
                                    name = Override,
                                    fields = (specifier = Required(NonTerminal(OverrideSpecifier)))
                                ),
                                EnumVariant(
                                    name = Payable,
                                    fields = (keyword = Required(Terminal([PayableKeyword])))
                                ),
                                EnumVariant(
                                    name = Public,
                                    fields = (keyword = Required(Terminal([PublicKeyword])))
                                )
                            ]
                        ),
                        Struct(
                            name = UnnamedFunctionDefinition,
                            disabled_in = "0.6.0",
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
                            disabled_in = "0.6.0",
                            allow_empty = true
                        ),
                        Enum(
                            name = UnnamedFunctionAttribute,
                            disabled_in = "0.6.0",
                            variants = [
                                EnumVariant(
                                    name = Modifier,
                                    fields = (modifier = Required(NonTerminal(ModifierInvocation)))
                                ),
                                EnumVariant(
                                    name = Override,
                                    fields = (specifier = Required(NonTerminal(OverrideSpecifier)))
                                ),
                                EnumVariant(
                                    name = External,
                                    fields = (keyword = Required(Terminal([ExternalKeyword])))
                                ),
                                EnumVariant(
                                    name = Payable,
                                    fields = (keyword = Required(Terminal([PayableKeyword])))
                                ),
                                EnumVariant(
                                    name = Pure,
                                    fields = (keyword = Required(Terminal([PureKeyword])))
                                ),
                                EnumVariant(
                                    name = View,
                                    fields = (keyword = Required(Terminal([ViewKeyword])))
                                )
                            ]
                        ),
                        Struct(
                            name = FallbackFunctionDefinition,
                            enabled_in = "0.6.0",
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
                            enabled_in = "0.6.0",
                            allow_empty = true
                        ),
                        Enum(
                            name = FallbackFunctionAttribute,
                            enabled_in = "0.6.0",
                            variants = [
                                EnumVariant(
                                    name = Modifier,
                                    fields = (modifier = Required(NonTerminal(ModifierInvocation)))
                                ),
                                EnumVariant(
                                    name = Override,
                                    fields = (specifier = Required(NonTerminal(OverrideSpecifier)))
                                ),
                                EnumVariant(
                                    name = External,
                                    fields = (keyword = Required(Terminal([ExternalKeyword])))
                                ),
                                EnumVariant(
                                    name = Payable,
                                    fields = (keyword = Required(Terminal([PayableKeyword])))
                                ),
                                EnumVariant(
                                    name = Pure,
                                    fields = (keyword = Required(Terminal([PureKeyword])))
                                ),
                                EnumVariant(
                                    name = View,
                                    fields = (keyword = Required(Terminal([ViewKeyword])))
                                ),
                                EnumVariant(
                                    name = Virtual,
                                    fields = (keyword = Required(Terminal([VirtualKeyword])))
                                )
                            ]
                        ),
                        Struct(
                            name = ReceiveFunctionDefinition,
                            enabled_in = "0.6.0",
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
                            enabled_in = "0.6.0",
                            allow_empty = true
                        ),
                        Enum(
                            name = ReceiveFunctionAttribute,
                            enabled_in = "0.6.0",
                            variants = [
                                EnumVariant(
                                    name = Modifier,
                                    fields = (modifier = Required(NonTerminal(ModifierInvocation)))
                                ),
                                EnumVariant(
                                    name = Override,
                                    fields = (specifier = Required(NonTerminal(OverrideSpecifier)))
                                ),
                                EnumVariant(
                                    name = External,
                                    fields = (keyword = Required(Terminal([ExternalKeyword])))
                                ),
                                EnumVariant(
                                    name = Payable,
                                    fields = (keyword = Required(Terminal([PayableKeyword])))
                                ),
                                EnumVariant(
                                    name = Virtual,
                                    fields = (keyword = Required(Terminal([VirtualKeyword])))
                                )
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
                                EnumVariant(
                                    name = Override,
                                    fields = (specifier = Required(NonTerminal(OverrideSpecifier)))
                                ),
                                EnumVariant(
                                    name = Virtual,
                                    enabled_in = "0.6.0",
                                    fields = (keyword = Required(Terminal([VirtualKeyword])))
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
                            error_recovery = [Terminator(semicolon)],
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
                            error_recovery = [Delimiters(open = open_paren, close = close_paren)],
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
                        enabled_in = "0.8.8",
                        error_recovery = [Terminator(semicolon)],
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
                            enabled_in = "0.8.4",
                            error_recovery = [Terminator(semicolon)],
                            fields = (
                                error_keyword = Required(Terminal([ErrorKeyword])),
                                name = Required(Terminal([Identifier])),
                                members = Required(NonTerminal(ErrorParametersDeclaration)),
                                semicolon = Required(Terminal([Semicolon]))
                            )
                        ),
                        Struct(
                            name = ErrorParametersDeclaration,
                            enabled_in = "0.8.4",
                            error_recovery = [Delimiters(open = open_paren, close = close_paren)],
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
                            enabled_in = "0.8.4",
                            allow_empty = true
                        ),
                        Struct(
                            name = ErrorParameter,
                            enabled_in = "0.8.4",
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
                                    error_recovery =
                                        [Delimiters(open = open_bracket, close = close_bracket)],
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
                                EnumVariant(
                                    name = Internal,
                                    fields = (keyword = Required(Terminal([InternalKeyword])))
                                ),
                                EnumVariant(
                                    name = External,
                                    fields = (keyword = Required(Terminal([ExternalKeyword])))
                                ),
                                EnumVariant(
                                    name = Private,
                                    fields = (keyword = Required(Terminal([PrivateKeyword])))
                                ),
                                EnumVariant(
                                    name = Public,
                                    fields = (keyword = Required(Terminal([PublicKeyword])))
                                ),
                                EnumVariant(
                                    name = Pure,
                                    fields = (keyword = Required(Terminal([PureKeyword])))
                                ),
                                EnumVariant(
                                    name = View,
                                    fields = (keyword = Required(Terminal([ViewKeyword])))
                                ),
                                EnumVariant(
                                    name = Payable,
                                    fields = (keyword = Required(Terminal([PayableKeyword])))
                                )
                            ]
                        ),
                        Struct(
                            name = MappingType,
                            error_recovery = [Delimiters(open = open_paren, close = close_paren)],
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
                                name =
                                    Optional(kind = Terminal([Identifier]), enabled_in = "0.8.18")
                            )
                        ),
                        Enum(
                            name = MappingKeyType,
                            variants = [
                                EnumVariant(
                                    name = ElementaryType,
                                    fields = (type_name = Required(NonTerminal(ElementaryType)))
                                ),
                                EnumVariant(
                                    name = IdentifierPath,
                                    fields = (type_name = Required(NonTerminal(IdentifierPath)))
                                )
                            ]
                        ),
                        Struct(
                            name = MappingValue,
                            fields = (
                                type_name = Required(NonTerminal(TypeName)),
                                name =
                                    Optional(kind = Terminal([Identifier]), enabled_in = "0.8.18")
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
                                EnumVariant(
                                    name = Bool,
                                    fields = (type_name = Required(Terminal([BoolKeyword])))
                                ),
                                EnumVariant(
                                    name = Byte,
                                    disabled_in = "0.8.0",
                                    fields = (type_name = Required(Terminal([ByteKeyword])))
                                ),
                                EnumVariant(
                                    name = String,
                                    fields = (type_name = Required(Terminal([StringKeyword])))
                                ),
                                EnumVariant(
                                    name = Address,
                                    fields = (type_name = Required(NonTerminal(AddressType)))
                                ),
                                EnumVariant(
                                    name = ByteArray,
                                    fields = (type_name = Required(Terminal([BytesKeyword])))
                                ),
                                EnumVariant(
                                    name = SignedInteger,
                                    fields = (type_name = Required(Terminal([IntKeyword])))
                                ),
                                EnumVariant(
                                    name = UnsignedInteger,
                                    fields = (type_name = Required(Terminal([UintKeyword])))
                                ),
                                EnumVariant(
                                    name = SignedFixedPointNumber,
                                    fields = (type_name = Required(Terminal([FixedKeyword])))
                                ),
                                EnumVariant(
                                    name = UnsignedFixedPointNumber,
                                    fields = (type_name = Required(Terminal([UfixedKeyword])))
                                )
                            ]
                        ),
                        Enum(
                            name = AddressType,
                            variants = [
                                EnumVariant(
                                    name = Address,
                                    fields = (
                                        address_keyword = Required(Terminal([AddressKeyword])),
                                        payable_keyword =
                                            Optional(kind = Terminal([PayableKeyword]))
                                    )
                                ),
                                EnumVariant(
                                    name = Payable,
                                    fields =
                                        (payable_keyword = Required(Terminal([PayableKeyword])))
                                )
                            ]
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
                            error_recovery = [Delimiters(open = open_brace, close = close_brace)],
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
                                    fields = (statement =
                                        Required(NonTerminal(TupleDeconstructionStatement)))
                                ),
                                EnumVariant(
                                    name = VariableDeclaration,
                                    fields = (statement =
                                        Required(NonTerminal(VariableDeclarationStatement)))
                                ),
                                EnumVariant(
                                    name = If,
                                    fields = (statement = Required(NonTerminal(IfStatement)))
                                ),
                                EnumVariant(
                                    name = For,
                                    fields = (statement = Required(NonTerminal(ForStatement)))
                                ),
                                EnumVariant(
                                    name = While,
                                    fields = (statement = Required(NonTerminal(WhileStatement)))
                                ),
                                EnumVariant(
                                    name = DoWhile,
                                    fields = (statement = Required(NonTerminal(DoWhileStatement)))
                                ),
                                EnumVariant(
                                    name = Continue,
                                    fields = (statement = Required(NonTerminal(ContinueStatement)))
                                ),
                                EnumVariant(
                                    name = Break,
                                    fields = (statement = Required(NonTerminal(BreakStatement)))
                                ),
                                EnumVariant(
                                    name = Delete,
                                    fields = (statement = Required(NonTerminal(DeleteStatement)))
                                ),
                                EnumVariant(
                                    name = Return,
                                    fields = (statement = Required(NonTerminal(ReturnStatement)))
                                ),
                                EnumVariant(
                                    name = Throw,
                                    disabled_in = "0.5.0",
                                    fields = (statement = Required(NonTerminal(ThrowStatement)))
                                ),
                                EnumVariant(
                                    name = Emit,
                                    enabled_in = "0.4.21",
                                    fields = (statement = Required(NonTerminal(EmitStatement)))
                                ),
                                EnumVariant(
                                    name = Try,
                                    enabled_in = "0.6.0",
                                    fields = (statement = Required(NonTerminal(TryStatement)))
                                ),
                                EnumVariant(
                                    name = Revert,
                                    enabled_in = "0.8.4",
                                    fields = (statement = Required(NonTerminal(RevertStatement)))
                                ),
                                EnumVariant(
                                    name = Assembly,
                                    fields = (statement = Required(NonTerminal(AssemblyStatement)))
                                ),
                                EnumVariant(
                                    name = Block,
                                    fields = (block = Required(NonTerminal(Block)))
                                ),
                                EnumVariant(
                                    name = UncheckedBlock,
                                    enabled_in = "0.8.0",
                                    fields = (block = Required(NonTerminal(UncheckedBlock)))
                                ),
                                EnumVariant(
                                    name = Expression,
                                    fields =
                                        (statement = Required(NonTerminal(ExpressionStatement)))
                                )
                            ]
                        ),
                        Struct(
                            name = UncheckedBlock,
                            enabled_in = "0.8.0",
                            fields = (
                                unchecked_keyword = Required(Terminal([UncheckedKeyword])),
                                block = Required(NonTerminal(Block))
                            )
                        ),
                        Struct(
                            name = ExpressionStatement,
                            error_recovery = [Terminator(semicolon)],
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
                            error_recovery = [
                                Terminator(semicolon),
                                Delimiters(open = open_paren, close = close_paren)
                            ],
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
                                EnumVariant(
                                    name = Typed,
                                    fields = (
                                        type_name = Required(NonTerminal(TypeName)),
                                        storage_location =
                                            Optional(kind = NonTerminal(StorageLocation)),
                                        name = Required(Terminal([Identifier]))
                                    )
                                ),
                                EnumVariant(
                                    name = Untyped,
                                    fields = (
                                        storage_location =
                                            Optional(kind = NonTerminal(StorageLocation)),
                                        name = Required(Terminal([Identifier]))
                                    )
                                )
                            ]
                        ),
                        Struct(
                            name = VariableDeclarationStatement,
                            error_recovery = [Terminator(semicolon)],
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
                                EnumVariant(
                                    name = Typed,
                                    fields = (type_name = Required(NonTerminal(TypeName)))
                                ),
                                EnumVariant(
                                    name = Untyped,
                                    disabled_in = "0.5.0",
                                    fields = (type_name = Required(Terminal([VarKeyword])))
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
                                EnumVariant(
                                    name = Memory,
                                    fields = (keyword = Required(Terminal([MemoryKeyword])))
                                ),
                                EnumVariant(
                                    name = Storage,
                                    fields = (keyword = Required(Terminal([StorageKeyword])))
                                ),
                                EnumVariant(
                                    name = CallData,
                                    enabled_in = "0.5.0",
                                    fields = (keyword = Required(Terminal([CallDataKeyword])))
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
                            error_recovery = [Delimiters(open = open_paren, close = close_paren)],
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
                            error_recovery = [Delimiters(open = open_paren, close = close_paren)],
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
                                EnumVariant(
                                    name = Expression,
                                    fields =
                                        (statement = Required(NonTerminal(ExpressionStatement)))
                                ),
                                EnumVariant(
                                    name = VariableDeclaration,
                                    fields = (statement =
                                        Required(NonTerminal(VariableDeclarationStatement)))
                                ),
                                EnumVariant(
                                    name = TupleDeconstruction,
                                    fields = (statement =
                                        Required(NonTerminal(TupleDeconstructionStatement)))
                                ),
                                EnumVariant(
                                    name = None,
                                    fields = (semicolon = Required(Terminal([Semicolon])))
                                )
                            ]
                        ),
                        Enum(
                            name = ForStatementCondition,
                            variants = [
                                EnumVariant(
                                    name = Expression,
                                    fields =
                                        (statement = Required(NonTerminal(ExpressionStatement)))
                                ),
                                EnumVariant(
                                    name = None,
                                    fields = (semicolon = Required(Terminal([Semicolon])))
                                )
                            ]
                        ),
                        Struct(
                            name = WhileStatement,
                            error_recovery = [Delimiters(open = open_paren, close = close_paren)],
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
                            error_recovery = [
                                Terminator(semicolon),
                                Delimiters(open = open_paren, close = close_paren)
                            ],
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
                            error_recovery = [Terminator(semicolon)],
                            fields = (
                                continue_keyword = Required(Terminal([ContinueKeyword])),
                                semicolon = Required(Terminal([Semicolon]))
                            )
                        ),
                        Struct(
                            name = BreakStatement,
                            error_recovery = [Terminator(semicolon)],
                            fields = (
                                break_keyword = Required(Terminal([BreakKeyword])),
                                semicolon = Required(Terminal([Semicolon]))
                            )
                        ),
                        Struct(
                            name = ReturnStatement,
                            error_recovery = [Terminator(semicolon)],
                            fields = (
                                return_keyword = Required(Terminal([ReturnKeyword])),
                                expression = Optional(kind = NonTerminal(Expression)),
                                semicolon = Required(Terminal([Semicolon]))
                            )
                        ),
                        Struct(
                            name = EmitStatement,
                            enabled_in = "0.4.21",
                            error_recovery = [Terminator(semicolon)],
                            fields = (
                                emit_keyword = Required(Terminal([EmitKeyword])),
                                event = Required(NonTerminal(IdentifierPath)),
                                arguments = Required(NonTerminal(ArgumentsDeclaration)),
                                semicolon = Required(Terminal([Semicolon]))
                            )
                        ),
                        Struct(
                            name = DeleteStatement,
                            error_recovery = [Terminator(semicolon)],
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
                            enabled_in = "0.6.0",
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
                            enabled_in = "0.6.0"
                        ),
                        Struct(
                            name = CatchClause,
                            enabled_in = "0.6.0",
                            fields = (
                                catch_keyword = Required(Terminal([CatchKeyword])),
                                error = Optional(kind = NonTerminal(CatchClauseError)),
                                body = Required(NonTerminal(Block))
                            )
                        ),
                        Struct(
                            name = CatchClauseError,
                            enabled_in = "0.6.0",
                            fields = (
                                name = Optional(kind = Terminal([Identifier])),
                                parameters = Required(NonTerminal(ParametersDeclaration))
                            )
                        ),
                        Struct(
                            name = RevertStatement,
                            enabled_in = "0.8.4",
                            error_recovery = [Terminator(semicolon)],
                            fields = (
                                revert_keyword = Required(Terminal([RevertKeyword])),
                                error = Optional(kind = NonTerminal(IdentifierPath)),
                                arguments = Required(NonTerminal(ArgumentsDeclaration)),
                                semicolon = Required(Terminal([Semicolon]))
                            )
                        ),
                        Struct(
                            name = ThrowStatement,
                            disabled_in = "0.5.0",
                            error_recovery = [Terminator(semicolon)],
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
                                            disabled_in = "0.6.0",
                                            fields =
                                                (operator = Required(Terminal([AsteriskAsterisk])))
                                        ),
                                        // In '0.6.0', it became right-associative:
                                        PrecedenceOperator(
                                            model = BinaryRightAssociative,
                                            enabled_in = "0.6.0",
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
                                            disabled_in = "0.5.0",
                                            fields = (operator = Required(Terminal([
                                                PlusPlus, MinusMinus, Tilde, Bang, Minus, Plus
                                            ])))
                                        ),
                                        // In '0.5.0', 'Plus' was removed:
                                        PrecedenceOperator(
                                            model = Prefix,
                                            enabled_in = "0.5.0",
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
                                            options = Required(NonTerminal(FunctionCallOptions)),
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
                                        error_recovery = [Delimiters(
                                            open = open_bracket,
                                            close = close_bracket
                                        )],
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
                                    enabled_in = "0.5.3"
                                ),
                                PrimaryExpression(expression = ArrayExpression),
                                PrimaryExpression(expression = NumberExpression),
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
                            variants = [
                                EnumVariant(
                                    name = Multiple,
                                    enabled_in = "0.6.2",
                                    disabled_in = "0.8.0",
                                    fields = (options =
                                        Required(NonTerminal(NamedArgumentsDeclarations)))
                                ),
                                EnumVariant(
                                    name = Single,
                                    enabled_in = "0.8.0",
                                    fields = (options =
                                        Optional(kind = NonTerminal(NamedArgumentsDeclaration)))
                                ),
                                EnumVariant(name = None, disabled_in = "0.6.2", fields = ())
                            ]
                        ),
                        Enum(
                            name = ArgumentsDeclaration,
                            variants = [
                                EnumVariant(
                                    name = Positional,
                                    error_recovery =
                                        [Delimiters(open = open_paren, close = close_paren)],
                                    fields = (
                                        open_paren = Required(Terminal([OpenParen])),
                                        arguments = Required(NonTerminal(PositionalArguments)),
                                        close_paren = Required(Terminal([CloseParen]))
                                    )
                                ),
                                EnumVariant(
                                    name = Named,
                                    error_recovery =
                                        [Delimiters(open = open_paren, close = close_paren)],
                                    fields = (
                                        open_paren = Required(Terminal([OpenParen])),
                                        arguments =
                                            Optional(kind = NonTerminal(NamedArgumentsDeclaration)),
                                        close_paren = Required(Terminal([CloseParen]))
                                    )
                                )
                            ]
                        ),
                        Separated(
                            name = PositionalArguments,
                            separated = Expression,
                            separator = Comma,
                            allow_empty = true
                        ),
                        Repeated(
                            name = NamedArgumentsDeclarations,
                            repeated = NamedArgumentsDeclaration,
                            enabled_in = "0.6.2",
                            disabled_in = "0.8.0",
                            allow_empty = true
                        ),
                        Struct(
                            name = NamedArgumentsDeclaration,
                            error_recovery = [Delimiters(open = open_brace, close = close_brace)],
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
                            enabled_in = "0.5.3",
                            error_recovery = [Delimiters(open = open_paren, close = close_paren)],
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
                            error_recovery = [Delimiters(open = open_paren, close = close_paren)],
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
                            error_recovery =
                                [Delimiters(open = open_bracket, close = close_bracket)],
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
                        Enum(
                            name = NumberExpression,
                            variants = [
                                EnumVariant(
                                    name = Hex,
                                    fields = (
                                        literal = Required(Terminal([HexLiteral])),
                                        unit = Optional(
                                            kind = NonTerminal(NumberUnit),
                                            disabled_in = "0.5.0"
                                        )
                                    )
                                ),
                                EnumVariant(
                                    name = Decimal,
                                    fields = (
                                        literal = Required(Terminal([DecimalLiteral])),
                                        unit = Optional(kind = NonTerminal(NumberUnit))
                                    )
                                )
                            ]
                        ),
                        Token(
                            name = HexLiteral,
                            definitions = [
                                TokenDefinition(
                                    // Lowercase "0x" enabled in all versions:
                                    scanner = TrailingContext(
                                        scanner = Sequence([
                                            Atom("0x"),
                                            OneOrMore(Fragment(HexCharacter)),
                                            ZeroOrMore(Sequence([
                                                Atom("_"),
                                                OneOrMore(Fragment(HexCharacter))
                                            ]))
                                        ]),
                                        not_followed_by = Fragment(IdentifierPart)
                                    )
                                ),
                                TokenDefinition(
                                    // Uppercase "0X" only enabled before "0.5.0":
                                    disabled_in = "0.5.0",
                                    scanner = TrailingContext(
                                        scanner = Sequence([
                                            Atom("0X"),
                                            OneOrMore(Fragment(HexCharacter)),
                                            ZeroOrMore(Sequence([
                                                Atom("_"),
                                                OneOrMore(Fragment(HexCharacter))
                                            ]))
                                        ]),
                                        not_followed_by = Fragment(IdentifierPart)
                                    )
                                )
                            ]
                        ),
                        Token(
                            name = DecimalLiteral,
                            definitions = [
                                TokenDefinition(
                                    // An integer (without a dot or a fraction) is enabled in all versions:
                                    scanner = TrailingContext(
                                        scanner = Sequence([
                                            Fragment(DecimalDigits),
                                            Optional(Fragment(DecimalExponent))
                                        ]),
                                        not_followed_by = Fragment(IdentifierPart)
                                    )
                                ),
                                TokenDefinition(
                                    // An integer and a dot (without a fraction) is disabled in "0.5.0"
                                    disabled_in = "0.5.0",
                                    scanner = TrailingContext(
                                        scanner = Sequence([
                                            Fragment(DecimalDigits),
                                            Atom("."),
                                            Optional(Fragment(DecimalExponent))
                                        ]),
                                        not_followed_by = Fragment(IdentifierPart)
                                    )
                                ),
                                TokenDefinition(
                                    // A dot and a fraction (without an integer) is enabled in all versions:
                                    scanner = TrailingContext(
                                        scanner = Sequence([
                                            Atom("."),
                                            Fragment(DecimalDigits),
                                            Optional(Fragment(DecimalExponent))
                                        ]),
                                        not_followed_by = Fragment(IdentifierPart)
                                    )
                                ),
                                TokenDefinition(
                                    // An integer, a dot, and a fraction is enabled in all versions:
                                    scanner = TrailingContext(
                                        scanner = Sequence([
                                            Fragment(DecimalDigits),
                                            Atom("."),
                                            Fragment(DecimalDigits),
                                            Optional(Fragment(DecimalExponent))
                                        ]),
                                        not_followed_by = Fragment(IdentifierPart)
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
                                EnumVariant(
                                    name = Wei, // 1e-18 ETH
                                    fields = (keyword = Required(Terminal([WeiKeyword])))
                                ),
                                EnumVariant(
                                    name = Gwei, // 1e-9 ETH
                                    enabled_in = "0.6.11",
                                    fields = (keyword = Required(Terminal([GweiKeyword])))
                                ),
                                EnumVariant(
                                    name = Szabo, // 1e-6 ETH
                                    disabled_in = "0.7.0",
                                    fields = (keyword = Required(Terminal([SzaboKeyword])))
                                ),
                                EnumVariant(
                                    name = Finney, // 1e-3 ETH
                                    disabled_in = "0.7.0",
                                    fields = (keyword = Required(Terminal([FinneyKeyword])))
                                ),
                                EnumVariant(
                                    name = Ether, // 1 ETH
                                    fields = (keyword = Required(Terminal([EtherKeyword])))
                                ),
                                EnumVariant(
                                    name = Seconds,
                                    fields = (keyword = Required(Terminal([SecondsKeyword])))
                                ),
                                EnumVariant(
                                    name = Minutes,
                                    fields = (keyword = Required(Terminal([MinutesKeyword])))
                                ),
                                EnumVariant(
                                    name = Hours,
                                    fields = (keyword = Required(Terminal([HoursKeyword])))
                                ),
                                EnumVariant(
                                    name = Days,
                                    fields = (keyword = Required(Terminal([DaysKeyword])))
                                ),
                                EnumVariant(
                                    name = Weeks,
                                    fields = (keyword = Required(Terminal([WeeksKeyword])))
                                ),
                                EnumVariant(
                                    name = Years,
                                    disabled_in = "0.5.0",
                                    fields = (keyword = Required(Terminal([YearsKeyword])))
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
                                    name = Hex,
                                    fields = (literals = Required(NonTerminal(HexStringLiterals)))
                                ),
                                EnumVariant(
                                    name = Ascii,
                                    fields =
                                        (literals = Required(NonTerminal(AsciiStringLiterals)))
                                ),
                                EnumVariant(
                                    name = Unicode,
                                    enabled_in = "0.7.0",
                                    fields =
                                        (literals = Required(NonTerminal(UnicodeStringLiterals)))
                                )
                            ]
                        ),
                        Repeated(name = HexStringLiterals, repeated = HexStringLiteral),
                        Token(
                            name = HexStringLiteral,
                            definitions = [TokenDefinition(
                                scanner = TrailingContext(
                                    scanner = Choice([
                                        Fragment(SingleQuotedHexString),
                                        Fragment(DoubleQuotedHexString)
                                    ]),
                                    not_followed_by = Fragment(IdentifierPart)
                                )
                            )]
                        ),
                        Fragment(
                            name = SingleQuotedHexString,
                            scanner = Sequence([
                                Atom("hex'"),
                                Optional(Fragment(HexStringContents)),
                                Atom("'")
                            ])
                        ),
                        Fragment(
                            name = DoubleQuotedHexString,
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
                        Repeated(name = AsciiStringLiterals, repeated = AsciiStringLiteral),
                        Token(
                            name = AsciiStringLiteral,
                            definitions = [TokenDefinition(
                                scanner = TrailingContext(
                                    scanner = Choice([
                                        Fragment(SingleQuotedAsciiString),
                                        Fragment(DoubleQuotedAsciiString)
                                    ]),
                                    not_followed_by = Fragment(IdentifierPart)
                                )
                            )]
                        ),
                        Fragment(
                            name = SingleQuotedAsciiString,
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
                            name = DoubleQuotedAsciiString,
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
                            repeated = UnicodeStringLiteral,
                            enabled_in = "0.7.0"
                        ),
                        Token(
                            name = UnicodeStringLiteral,
                            definitions = [TokenDefinition(
                                enabled_in = "0.7.0",
                                scanner = TrailingContext(
                                    scanner = Choice([
                                        Fragment(SingleQuotedUnicodeString),
                                        Fragment(DoubleQuotedUnicodeString)
                                    ]),
                                    not_followed_by = Fragment(IdentifierPart)
                                )
                            )]
                        ),
                        Fragment(
                            name = SingleQuotedUnicodeString,
                            enabled_in = "0.7.0",
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
                            name = DoubleQuotedUnicodeString,
                            enabled_in = "0.7.0",
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
                                label = Optional(kind = Terminal([AsciiStringLiteral])),
                                flags = Optional(kind = NonTerminal(AssemblyFlagsDeclaration)),
                                body = Required(NonTerminal(YulBlock))
                            )
                        ),
                        Struct(
                            name = AssemblyFlagsDeclaration,
                            error_recovery = [Delimiters(open = open_paren, close = close_paren)],
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
                            error_recovery = [Delimiters(open = open_paren, close = close_paren)],
                            fields = (
                                open_paren = Required(Terminal([OpenParen])),
                                statements = Required(NonTerminal(YulStatements)),
                                close_paren = Required(Terminal([CloseParen]))
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
                                EnumVariant(
                                    name = Block,
                                    fields = (block = Required(NonTerminal(YulBlock)))
                                ),
                                EnumVariant(
                                    name = Function,
                                    fields =
                                        (definition = Required(NonTerminal(YulFunctionDefinition)))
                                ),
                                EnumVariant(
                                    name = VariableDeclaration,
                                    fields = (statement =
                                        Required(NonTerminal(YulVariableDeclarationStatement)))
                                ),
                                EnumVariant(
                                    name = Assignment,
                                    fields =
                                        (statement = Required(NonTerminal(YulAssignmentStatement)))
                                ),
                                EnumVariant(
                                    name = If,
                                    fields = (statement = Required(NonTerminal(YulIfStatement)))
                                ),
                                EnumVariant(
                                    name = For,
                                    fields = (statement = Required(NonTerminal(YulForStatement)))
                                ),
                                EnumVariant(
                                    name = Switch,
                                    fields =
                                        (statement = Required(NonTerminal(YulSwitchStatement)))
                                ),
                                EnumVariant(
                                    name = Leave,
                                    enabled_in = "0.6.0",
                                    fields = (statement = Required(NonTerminal(YulLeaveStatement)))
                                ),
                                EnumVariant(
                                    name = Break,
                                    fields = (statement = Required(NonTerminal(YulBreakStatement)))
                                ),
                                EnumVariant(
                                    name = Continue,
                                    fields =
                                        (statement = Required(NonTerminal(YulContinueStatement)))
                                ),
                                EnumVariant(
                                    name = Expression,
                                    fields = (expression = Required(NonTerminal(YulExpression)))
                                )
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
                            error_recovery = [Delimiters(open = open_paren, close = close_paren)],
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
                            enabled_in = "0.6.0",
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
                                EnumVariant(
                                    name = Default,
                                    fields = (
                                        default_keyword = Required(Terminal([YulDefaultKeyword])),
                                        body = Required(NonTerminal(YulBlock))
                                    )
                                ),
                                EnumVariant(
                                    name = Case,
                                    fields = (
                                        case_keyword = Required(Terminal([YulCaseKeyword])),
                                        value = Required(NonTerminal(YulLiteral)),
                                        body = Required(NonTerminal(YulBlock))
                                    )
                                )
                            ]
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
                                    error_recovery =
                                        [Delimiters(open = open_paren, close = close_paren)],
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
                                EnumVariant(
                                    name = True,
                                    fields = (literal = Required(Terminal([YulTrueKeyword])))
                                ),
                                EnumVariant(
                                    name = False,
                                    fields = (literal = Required(Terminal([YulFalseKeyword])))
                                ),
                                EnumVariant(
                                    name = Decimal,
                                    fields = (literal = Required(Terminal([YulDecimalLiteral])))
                                ),
                                EnumVariant(
                                    name = Hex,
                                    fields = (literal = Required(Terminal([YulHexLiteral])))
                                ),
                                EnumVariant(
                                    name = HexString,
                                    fields = (literal = Required(Terminal([HexStringLiteral])))
                                ),
                                EnumVariant(
                                    name = AsciiString,
                                    fields = (literal = Required(Terminal([AsciiStringLiteral])))
                                )
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
                                    not_followed_by = Fragment(IdentifierPart)
                                )
                            )]
                        ),
                        Token(
                            name = YulHexLiteral,
                            definitions = [TokenDefinition(
                                scanner = TrailingContext(
                                    scanner =
                                        Sequence([Atom("0x"), OneOrMore(Fragment(HexCharacter))]),
                                    not_followed_by = Fragment(IdentifierPart)
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
                            name = YulBreakKeyword,
                            identifier = YulIdentifier,
                            value = Atom("break")
                        ),
                        Keyword(
                            name = YulCaseKeyword,
                            identifier = YulIdentifier,
                            value = Atom("case")
                        ),
                        Keyword(
                            name = YulContinueKeyword,
                            identifier = YulIdentifier,
                            value = Atom("continue")
                        ),
                        Keyword(
                            name = YulDefaultKeyword,
                            identifier = YulIdentifier,
                            value = Atom("default")
                        ),
                        Keyword(
                            name = YulFalseKeyword,
                            identifier = YulIdentifier,
                            value = Atom("false")
                        ),
                        Keyword(
                            name = YulForKeyword,
                            identifier = YulIdentifier,
                            value = Atom("for")
                        ),
                        Keyword(
                            name = YulFunctionKeyword,
                            identifier = YulIdentifier,
                            value = Atom("function")
                        ),
                        Keyword(
                            name = YulHexKeyword,
                            identifier = YulIdentifier,
                            disabled_in = "0.4.11",
                            value = Atom("hex")
                        ),
                        Keyword(
                            name = YulIfKeyword,
                            identifier = YulIdentifier,
                            value = Atom("if")
                        ),
                        Keyword(
                            name = YulLeaveKeyword,
                            identifier = YulIdentifier,
                            enabled_in = "0.6.0",
                            value = Atom("leave")
                        ),
                        Keyword(
                            name = YulLetKeyword,
                            identifier = YulIdentifier,
                            value = Atom("let")
                        ),
                        Keyword(
                            name = YulSwitchKeyword,
                            identifier = YulIdentifier,
                            value = Atom("switch")
                        ),
                        Keyword(
                            name = YulTrueKeyword,
                            identifier = YulIdentifier,
                            value = Atom("true")
                        )
                    ]
                )
            ]
        )
    ]
));
