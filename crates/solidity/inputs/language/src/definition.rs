pub use solidity::SolidityDefinition;

codegen_language_macros::compile!(
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
                            repeated = NonTerminal(SourceUnitMember),
                            allow_empty = true
                        ),
                        Enum(
                            name = SourceUnitMember,
                            variants = [
                                Variant(
                                    name = Pragma,
                                    fields = (directive = Required(NonTerminal(PragmaDirective)))
                                ),
                                Variant(
                                    name = Import,
                                    fields = (directive = Required(NonTerminal(ImportDirective)))
                                ),
                                Variant(
                                    name = Contract,
                                    fields =
                                        (definition = Required(NonTerminal(ContractDefinition)))
                                ),
                                Variant(
                                    name = Interface,
                                    fields =
                                        (definition = Required(NonTerminal(InterfaceDefinition)))
                                ),
                                Variant(
                                    name = Library,
                                    fields =
                                        (definition = Required(NonTerminal(LibraryDefinition)))
                                ),
                                Variant(
                                    name = Struct,
                                    enabled_in = "0.6.0",
                                    fields = (definition = Required(NonTerminal(StructDefinition)))
                                ),
                                Variant(
                                    name = Enum,
                                    enabled_in = "0.6.0",
                                    fields = (definition = Required(NonTerminal(EnumDefinition)))
                                ),
                                Variant(
                                    name = Function,
                                    enabled_in = "0.7.1",
                                    fields =
                                        (definition = Required(NonTerminal(FunctionDefinition)))
                                ),
                                Variant(
                                    name = Constant,
                                    enabled_in = "0.7.4",
                                    fields =
                                        (definition = Required(NonTerminal(ConstantDefinition)))
                                ),
                                Variant(
                                    name = Error,
                                    enabled_in = "0.8.4",
                                    fields = (definition = Required(NonTerminal(ErrorDefinition)))
                                ),
                                Variant(
                                    name = UserDefinedValueType,
                                    enabled_in = "0.8.8",
                                    fields = (definition =
                                        Required(NonTerminal(UserDefinedValueTypeDefinition)))
                                ),
                                Variant(
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
                            fields = (
                                pragma_keyword = Required(Terminal([PragmaKeyword])),
                                pragma = Required(NonTerminal(Pragma)),
                                semicolon = Required(Terminal([Semicolon]))
                            )
                        ),
                        Enum(
                            name = Pragma,
                            variants = [
                                Variant(
                                    name = ABICoder,
                                    fields = (
                                        abicoder_keyword = Required(Terminal([AbicoderKeyword])),
                                        version = Required(Terminal([Identifier]))
                                    )
                                ),
                                Variant(
                                    name = Experimental,
                                    fields = (
                                        experimental_keyword =
                                            Required(Terminal([ExperimentalKeyword])),
                                        feature =
                                            Required(Terminal([AsciiStringLiteral, Identifier]))
                                    )
                                ),
                                Variant(
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
                            repeated = NonTerminal(VersionPragmaExpression)
                        ),
                        Precedence(
                            name = VersionPragmaExpression,
                            operators = [
                                Operator(
                                    expression_name = VersionPragmaBinaryExpression,
                                    model = BinaryLeftAssociative,
                                    fields = (operator = Required(Terminal([BarBar])))
                                ),
                                Operator(
                                    expression_name = VersionPragmaBinaryExpression,
                                    model = BinaryLeftAssociative,
                                    fields = (operator = Required(Terminal([Minus])))
                                ),
                                Operator(
                                    expression_name = VersionPragmaPrefixExpression,
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
                                )
                            ],
                            primary_expressions = [Variant(
                                name = VersionPragmaSpecifier,
                                fields =
                                    (specifier = Required(NonTerminal(VersionPragmaSpecifier)))
                            )]
                        ),
                        Separated(
                            name = VersionPragmaSpecifier,
                            separated = Terminal([VersionPragmaValue]),
                            separator = Terminal([Period])
                        ),
                        Token(
                            name = VersionPragmaValue,
                            definitions = [TokenDefinition(
                                scanner = OneOrMore(Choice([
                                    Range(inclusive_start = '0', exclusive_end = '9'),
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
                            fields = (
                                import_keyword = Required(Terminal([ImportKeyword])),
                                symbol = Required(NonTerminal(ImportSymbol)),
                                semicolon = Required(Terminal([Semicolon]))
                            )
                        ),
                        Enum(
                            name = ImportSymbol,
                            variants = [
                                Variant(
                                    name = Path,
                                    fields = (
                                        path = Required(Terminal([AsciiStringLiteral])),
                                        alias = Optional(reference = NonTerminal(ImportAlias))
                                    )
                                ),
                                Variant(
                                    name = Named,
                                    fields = (
                                        asterisk = Required(Terminal([Asterisk])),
                                        alias = Required(NonTerminal(ImportAlias)),
                                        from_keyword = Required(Terminal([FromKeyword])),
                                        path = Required(Terminal([AsciiStringLiteral]))
                                    )
                                ),
                                Variant(
                                    name = Deconstruction,
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
                            separated = NonTerminal(ImportDeconstructionField),
                            separator = Terminal([Comma]),
                            allow_empty = true
                        ),
                        Struct(
                            name = ImportDeconstructionField,
                            fields = (
                                name = Required(Terminal([Identifier])),
                                alias = Optional(reference = NonTerminal(ImportAlias))
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
                            fields = (
                                using_keyword = Required(Terminal([UsingKeyword])),
                                symbol = Required(NonTerminal(UsingSymbol)),
                                for_keyword = Required(Terminal([ForKeyword])),
                                target = Required(NonTerminal(UsingTarget)),
                                global_keyword = Optional(
                                    reference = Terminal([GlobalKeyword]),
                                    enabled_in = "0.8.13"
                                ),
                                semicolon = Required(Terminal([Semicolon]))
                            )
                        ),
                        Enum(
                            name = UsingSymbol,
                            variants = [
                                Variant(
                                    name = Path,
                                    fields = (path = Required(NonTerminal(IdentifierPath)))
                                ),
                                Variant(
                                    name = Deconstruction,
                                    enabled_in = "0.8.13",
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
                            separated = NonTerminal(UsingDeconstructionField),
                            separator = Terminal([Comma]),
                            enabled_in = "0.8.13",
                            allow_empty = true
                        ),
                        Struct(
                            name = UsingDeconstructionField,
                            enabled_in = "0.8.13",
                            fields = (
                                name = Required(NonTerminal(IdentifierPath)),
                                alias = Optional(
                                    reference = NonTerminal(UsingAlias),
                                    enabled_in = "0.8.19"
                                )
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
                                Variant(
                                    name = TypeName,
                                    fields = (type_name = Required(NonTerminal(TypeName)))
                                ),
                                Variant(
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
                            is_open_delimiter_for = CloseParen,
                            definitions = [TokenDefinition(scanner = Atom("("))]
                        ),
                        Token(
                            name = CloseParen,
                            is_close_delimiter_for = OpenParen,
                            definitions = [TokenDefinition(scanner = Atom(")"))]
                        ),
                        Token(
                            name = OpenBracket,
                            is_open_delimiter_for = CloseBracket,
                            definitions = [TokenDefinition(scanner = Atom("["))]
                        ),
                        Token(
                            name = CloseBracket,
                            is_close_delimiter_for = OpenBracket,
                            definitions = [TokenDefinition(scanner = Atom("]"))]
                        ),
                        Token(
                            name = OpenBrace,
                            is_open_delimiter_for = CloseBrace,
                            definitions = [TokenDefinition(scanner = Atom("{"))]
                        ),
                        Token(
                            name = CloseBrace,
                            is_close_delimiter_for = OpenBrace,
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
                            is_terminator = true,
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
                            fields = (
                                abstract_keyword = Optional(
                                    reference = Terminal([AbstractKeyword]),
                                    enabled_in = "0.6.0"
                                ),
                                contract_keyword = Required(Terminal([ContractKeyword])),
                                name = Required(Terminal([Identifier])),
                                inheritence =
                                    Optional(reference = NonTerminal(InheritanceSpecifier)),
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
                            separated = NonTerminal(InheritanceType),
                            separator = Terminal([Comma])
                        ),
                        Struct(
                            name = InheritanceType,
                            fields = (
                                type_name = Required(NonTerminal(IdentifierPath)),
                                arguments = Optional(reference = NonTerminal(ArgumentsDeclaration))
                            )
                        ),
                        Repeated(
                            name = ContractMembers,
                            repeated = NonTerminal(ContractMember),
                            allow_empty = true
                        ),
                        Enum(
                            name = ContractMember,
                            variants = [
                                Variant(
                                    name = Using,
                                    fields = (directive = Required(NonTerminal(UsingDirective)))
                                ),
                                Variant(
                                    name = Function,
                                    fields =
                                        (definition = Required(NonTerminal(FunctionDefinition)))
                                ),
                                Variant(
                                    name = Constructor,
                                    enabled_in = "0.4.22",
                                    fields =
                                        (definition = Required(NonTerminal(ConstructorDefinition)))
                                ),
                                Variant(
                                    name = ReceiveFunction,
                                    enabled_in = "0.6.0",
                                    fields = (definition =
                                        Required(NonTerminal(ReceiveFunctionDefinition)))
                                ),
                                Variant(
                                    name = FallbackFunction,
                                    enabled_in = "0.6.0",
                                    fields = (definition =
                                        Required(NonTerminal(FallbackFunctionDefinition)))
                                ),
                                Variant(
                                    name = UnnamedFunction,
                                    disabled_in = "0.6.0",
                                    fields = (definition =
                                        Required(NonTerminal(UnnamedFunctionDefinition)))
                                ),
                                Variant(
                                    name = Modifier,
                                    fields =
                                        (definition = Required(NonTerminal(ModifierDefinition)))
                                ),
                                Variant(
                                    name = Struct,
                                    fields = (definition = Required(NonTerminal(StructDefinition)))
                                ),
                                Variant(
                                    name = Enum,
                                    fields = (definition = Required(NonTerminal(EnumDefinition)))
                                ),
                                Variant(
                                    name = Event,
                                    fields = (definition = Required(NonTerminal(EventDefinition)))
                                ),
                                Variant(
                                    name = StateVariable,
                                    fields = (definition =
                                        Required(NonTerminal(StateVariableDefinition)))
                                ),
                                Variant(
                                    name = Error,
                                    enabled_in = "0.8.4",
                                    fields = (definition = Required(NonTerminal(ErrorDefinition)))
                                ),
                                Variant(
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
                            fields = (
                                interface_keyword = Required(Terminal([InterfaceKeyword])),
                                name = Required(Terminal([Identifier])),
                                inheritence =
                                    Optional(reference = NonTerminal(InheritanceSpecifier)),
                                open_brace = Required(Terminal([OpenBrace])),
                                members = Required(NonTerminal(InterfaceMembers)),
                                close_brace = Required(Terminal([CloseBrace]))
                            )
                        ),
                        Repeated(
                            name = InterfaceMembers,
                            repeated = NonTerminal(ContractMember),
                            allow_empty = true
                        )
                    ]
                ),
                Topic(
                    title = "Libraries",
                    items = [
                        Struct(
                            name = LibraryDefinition,
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
                            repeated = NonTerminal(ContractMember),
                            allow_empty = true
                        )
                    ]
                ),
                Topic(
                    title = "Structs",
                    items = [
                        Struct(
                            name = StructDefinition,
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
                            repeated = NonTerminal(StructMember),
                            allow_empty = true
                        ),
                        Struct(
                            name = StructMember,
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
                            separated = Terminal([Identifier]),
                            separator = Terminal([Comma]),
                            allow_empty = true
                        )
                    ]
                ),
                Topic(
                    title = "Constants",
                    items = [Struct(
                        name = ConstantDefinition,
                        enabled_in = "0.7.4",
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
                            fields = (
                                type_name = Required(NonTerminal(TypeName)),
                                attributes = Required(NonTerminal(StateVariableAttributes)),
                                name = Required(Terminal([Identifier])),
                                value =
                                    Optional(reference = NonTerminal(StateVariableDefinitionValue)),
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
                            repeated = NonTerminal(StateVariableAttribute),
                            allow_empty = true
                        ),
                        Enum(
                            name = StateVariableAttribute,
                            variants = [
                                Variant(
                                    name = Override,
                                    fields = (specifier = Required(NonTerminal(OverrideSpecifier)))
                                ),
                                Variant(
                                    name = Constant,
                                    fields = (keyword = Required(Terminal([ConstantKeyword])))
                                ),
                                Variant(
                                    name = Internal,
                                    fields = (keyword = Required(Terminal([InternalKeyword])))
                                ),
                                Variant(
                                    name = Private,
                                    fields = (keyword = Required(Terminal([PrivateKeyword])))
                                ),
                                Variant(
                                    name = Public,
                                    fields = (keyword = Required(Terminal([PublicKeyword])))
                                ),
                                Variant(
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
                                returns = Optional(reference = NonTerminal(ReturnsDeclaration)),
                                body = Required(NonTerminal(FunctionBody))
                            )
                        ),
                        Struct(
                            name = ParametersDeclaration,
                            fields = (
                                open_paren = Required(Terminal([OpenParen])),
                                parameters = Required(NonTerminal(Parameters)),
                                close_paren = Required(Terminal([CloseParen]))
                            )
                        ),
                        Separated(
                            name = Parameters,
                            separated = NonTerminal(Parameter),
                            separator = Terminal([Comma]),
                            allow_empty = true
                        ),
                        Struct(
                            name = Parameter,
                            fields = (
                                type_name = Required(NonTerminal(TypeName)),
                                storage_location =
                                    Optional(reference = NonTerminal(StorageLocation)),
                                name = Optional(reference = Terminal([Identifier]))
                            )
                        ),
                        Repeated(
                            name = FunctionAttributes,
                            repeated = NonTerminal(FunctionAttribute),
                            allow_empty = true
                        ),
                        Enum(
                            name = FunctionAttribute,
                            variants = [
                                Variant(
                                    name = Modifier,
                                    fields = (modifier = Required(NonTerminal(ModifierInvocation)))
                                ),
                                Variant(
                                    name = Override,
                                    fields = (specifier = Required(NonTerminal(OverrideSpecifier)))
                                ),
                                Variant(
                                    name = Constant,
                                    disabled_in = "0.5.0",
                                    fields = (keyword = Required(Terminal([ConstantKeyword])))
                                ),
                                Variant(
                                    name = External,
                                    fields = (keyword = Required(Terminal([ExternalKeyword])))
                                ),
                                Variant(
                                    name = Internal,
                                    fields = (keyword = Required(Terminal([InternalKeyword])))
                                ),
                                Variant(
                                    name = Payable,
                                    fields = (keyword = Required(Terminal([PayableKeyword])))
                                ),
                                Variant(
                                    name = Private,
                                    fields = (keyword = Required(Terminal([PrivateKeyword])))
                                ),
                                Variant(
                                    name = Public,
                                    fields = (keyword = Required(Terminal([PublicKeyword])))
                                ),
                                Variant(
                                    name = Pure,
                                    fields = (keyword = Required(Terminal([PureKeyword])))
                                ),
                                Variant(
                                    name = View,
                                    fields = (keyword = Required(Terminal([ViewKeyword])))
                                ),
                                Variant(
                                    name = Virtual,
                                    enabled_in = "0.6.0",
                                    fields = (keyword = Required(Terminal([VirtualKeyword])))
                                )
                            ]
                        ),
                        Struct(
                            name = OverrideSpecifier,
                            fields = (
                                override_keyword = Required(Terminal([OverrideKeyword])),
                                open_paren = Required(Terminal([OpenParen])),
                                overridden = Required(NonTerminal(OverridePaths)),
                                close_paren = Required(Terminal([CloseParen]))
                            )
                        ),
                        Separated(
                            name = OverridePaths,
                            separated = NonTerminal(IdentifierPath),
                            separator = Terminal([Comma])
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
                                Variant(
                                    name = Semicolon,
                                    fields = (semicolon = Required(Terminal([Semicolon])))
                                ),
                                Variant(
                                    name = Block,
                                    fields = (block = Required(NonTerminal(Block)))
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
                            repeated = NonTerminal(ConstructorAttribute),
                            enabled_in = "0.4.22",
                            allow_empty = true
                        ),
                        Enum(
                            name = ConstructorAttribute,
                            enabled_in = "0.4.22",
                            variants = [
                                Variant(
                                    name = Modifier,
                                    fields = (modifier = Required(NonTerminal(ModifierInvocation)))
                                ),
                                Variant(
                                    name = Override,
                                    fields = (specifier = Required(NonTerminal(OverrideSpecifier)))
                                ),
                                Variant(
                                    name = Payable,
                                    fields = (keyword = Required(Terminal([PayableKeyword])))
                                ),
                                Variant(
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
                            repeated = NonTerminal(UnnamedFunctionAttribute),
                            disabled_in = "0.6.0",
                            allow_empty = true
                        ),
                        Enum(
                            name = UnnamedFunctionAttribute,
                            disabled_in = "0.6.0",
                            variants = [
                                Variant(
                                    name = Modifier,
                                    fields = (modifier = Required(NonTerminal(ModifierInvocation)))
                                ),
                                Variant(
                                    name = Override,
                                    fields = (specifier = Required(NonTerminal(OverrideSpecifier)))
                                ),
                                Variant(
                                    name = External,
                                    fields = (keyword = Required(Terminal([ExternalKeyword])))
                                ),
                                Variant(
                                    name = Payable,
                                    fields = (keyword = Required(Terminal([PayableKeyword])))
                                ),
                                Variant(
                                    name = Pure,
                                    fields = (keyword = Required(Terminal([PureKeyword])))
                                ),
                                Variant(
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
                                returns = Optional(reference = NonTerminal(ReturnsDeclaration)),
                                body = Required(NonTerminal(FunctionBody))
                            )
                        ),
                        Repeated(
                            name = FallbackFunctionAttributes,
                            repeated = NonTerminal(FallbackFunctionAttribute),
                            enabled_in = "0.6.0",
                            allow_empty = true
                        ),
                        Enum(
                            name = FallbackFunctionAttribute,
                            enabled_in = "0.6.0",
                            variants = [
                                Variant(
                                    name = Modifier,
                                    fields = (modifier = Required(NonTerminal(ModifierInvocation)))
                                ),
                                Variant(
                                    name = Override,
                                    fields = (specifier = Required(NonTerminal(OverrideSpecifier)))
                                ),
                                Variant(
                                    name = External,
                                    fields = (keyword = Required(Terminal([ExternalKeyword])))
                                ),
                                Variant(
                                    name = Payable,
                                    fields = (keyword = Required(Terminal([PayableKeyword])))
                                ),
                                Variant(
                                    name = Pure,
                                    fields = (keyword = Required(Terminal([PureKeyword])))
                                ),
                                Variant(
                                    name = View,
                                    fields = (keyword = Required(Terminal([ViewKeyword])))
                                ),
                                Variant(
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
                            repeated = NonTerminal(ReceiveFunctionAttribute),
                            enabled_in = "0.6.0",
                            allow_empty = true
                        ),
                        Enum(
                            name = ReceiveFunctionAttribute,
                            enabled_in = "0.6.0",
                            variants = [
                                Variant(
                                    name = Modifier,
                                    fields = (modifier = Required(NonTerminal(ModifierInvocation)))
                                ),
                                Variant(
                                    name = Override,
                                    fields = (specifier = Required(NonTerminal(OverrideSpecifier)))
                                ),
                                Variant(
                                    name = External,
                                    fields = (keyword = Required(Terminal([ExternalKeyword])))
                                ),
                                Variant(
                                    name = Payable,
                                    fields = (keyword = Required(Terminal([PayableKeyword])))
                                ),
                                Variant(
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
                                parameters =
                                    Optional(reference = NonTerminal(ParametersDeclaration)),
                                attributes = Required(NonTerminal(ModifierAttributes)),
                                body = Required(NonTerminal(FunctionBody))
                            )
                        ),
                        Repeated(
                            name = ModifierAttributes,
                            repeated = NonTerminal(ModifierAttribute),
                            allow_empty = true
                        ),
                        Enum(
                            name = ModifierAttribute,
                            variants = [
                                Variant(
                                    name = Override,
                                    fields = (specifier = Required(NonTerminal(OverrideSpecifier)))
                                ),
                                Variant(
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
                                arguments = Optional(reference = NonTerminal(ArgumentsDeclaration))
                            )
                        )
                    ]
                ),
                Topic(
                    title = "Events",
                    items = [
                        Struct(
                            name = EventDefinition,
                            fields = (
                                event_keyword = Required(Terminal([EventKeyword])),
                                name = Required(Terminal([Identifier])),
                                parameters =
                                    Optional(reference = NonTerminal(EventParametersDeclaration)),
                                anonymous_keyword =
                                    Optional(reference = Terminal([AnonymousKeyword])),
                                semicolon = Required(Terminal([Semicolon]))
                            )
                        ),
                        Struct(
                            name = EventParametersDeclaration,
                            fields = (
                                open_paren = Required(Terminal([OpenParen])),
                                parameters = Required(NonTerminal(EventParameters)),
                                close_paren = Required(Terminal([CloseParen]))
                            )
                        ),
                        Separated(
                            name = EventParameters,
                            separated = NonTerminal(EventParameter),
                            separator = Terminal([Comma]),
                            allow_empty = true
                        ),
                        Struct(
                            name = EventParameter,
                            fields = (
                                type_name = Required(NonTerminal(TypeName)),
                                indexed_keyword = Optional(reference = Terminal([IndexedKeyword])),
                                name = Optional(reference = Terminal([Identifier]))
                            )
                        )
                    ]
                ),
                Topic(
                    title = "User Defined Value Types",
                    items = [Struct(
                        name = UserDefinedValueTypeDefinition,
                        enabled_in = "0.8.8",
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
                            fields = (
                                open_paren = Required(Terminal([OpenParen])),
                                parameters = Required(NonTerminal(ErrorParameters)),
                                close_paren = Required(Terminal([CloseParen]))
                            )
                        ),
                        Separated(
                            name = ErrorParameters,
                            separated = NonTerminal(ErrorParameter),
                            separator = Terminal([Comma]),
                            enabled_in = "0.8.4",
                            allow_empty = true
                        ),
                        Struct(
                            name = ErrorParameter,
                            enabled_in = "0.8.4",
                            fields = (
                                type_name = Required(NonTerminal(TypeName)),
                                name = Optional(reference = Terminal([Identifier]))
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
                            operators = [Operator(
                                expression_name = ArrayTypeName,
                                model = Postfix,
                                fields = (
                                    open_bracket = Required(Terminal([OpenBracket])),
                                    index = Optional(reference = NonTerminal(Expression)),
                                    close_bracket = Required(Terminal([CloseBracket]))
                                )
                            )],
                            primary_expressions = [
                                Variant(
                                    name = FunctionType,
                                    fields = (type_name = Required(NonTerminal(FunctionType)))
                                ),
                                Variant(
                                    name = MappingType,
                                    fields = (type_name = Required(NonTerminal(MappingType)))
                                ),
                                Variant(
                                    name = ElementaryType,
                                    fields = (type_name = Required(NonTerminal(ElementaryType)))
                                ),
                                Variant(
                                    name = IdentifierPath,
                                    fields = (type_name = Required(NonTerminal(IdentifierPath)))
                                )
                            ]
                        ),
                        Struct(
                            name = FunctionType,
                            fields = (
                                function_keyword = Required(Terminal([FunctionKeyword])),
                                parameters = Required(NonTerminal(ParametersDeclaration)),
                                attributes = Required(NonTerminal(FunctionTypeAttributes)),
                                returns = Optional(reference = NonTerminal(ReturnsDeclaration))
                            )
                        ),
                        Repeated(
                            name = FunctionTypeAttributes,
                            repeated = NonTerminal(FunctionTypeAttribute),
                            allow_empty = true
                        ),
                        Enum(
                            name = FunctionTypeAttribute,
                            variants = [
                                Variant(
                                    name = Internal,
                                    fields = (keyword = Required(Terminal([InternalKeyword])))
                                ),
                                Variant(
                                    name = External,
                                    fields = (keyword = Required(Terminal([ExternalKeyword])))
                                ),
                                Variant(
                                    name = Private,
                                    fields = (keyword = Required(Terminal([PrivateKeyword])))
                                ),
                                Variant(
                                    name = Public,
                                    fields = (keyword = Required(Terminal([PublicKeyword])))
                                ),
                                Variant(
                                    name = Pure,
                                    fields = (keyword = Required(Terminal([PureKeyword])))
                                ),
                                Variant(
                                    name = View,
                                    fields = (keyword = Required(Terminal([ViewKeyword])))
                                ),
                                Variant(
                                    name = Payable,
                                    fields = (keyword = Required(Terminal([PayableKeyword])))
                                )
                            ]
                        ),
                        Struct(
                            name = MappingType,
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
                                    reference = Terminal([Identifier]),
                                    enabled_in = "0.8.18"
                                )
                            )
                        ),
                        Enum(
                            name = MappingKeyType,
                            variants = [
                                Variant(
                                    name = ElementaryType,
                                    fields = (type_name = Required(NonTerminal(ElementaryType)))
                                ),
                                Variant(
                                    name = IdentifierPath,
                                    fields = (type_name = Required(NonTerminal(IdentifierPath)))
                                )
                            ]
                        ),
                        Struct(
                            name = MappingValue,
                            fields = (
                                type_name = Required(NonTerminal(TypeName)),
                                name = Optional(
                                    reference = Terminal([Identifier]),
                                    enabled_in = "0.8.18"
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
                                Variant(
                                    name = Bool,
                                    fields = (type_name = Required(Terminal([BoolKeyword])))
                                ),
                                Variant(
                                    name = Byte,
                                    disabled_in = "0.8.0",
                                    fields = (type_name = Required(Terminal([ByteKeyword])))
                                ),
                                Variant(
                                    name = String,
                                    fields = (type_name = Required(Terminal([StringKeyword])))
                                ),
                                Variant(
                                    name = Address,
                                    fields = (type_name = Required(NonTerminal(AddressType)))
                                ),
                                Variant(
                                    name = ByteArray,
                                    fields = (type_name = Required(Terminal([BytesKeyword])))
                                ),
                                Variant(
                                    name = SignedInteger,
                                    fields = (type_name = Required(Terminal([IntKeyword])))
                                ),
                                Variant(
                                    name = UnsignedInteger,
                                    fields = (type_name = Required(Terminal([UintKeyword])))
                                ),
                                Variant(
                                    name = SignedFixedPointNumber,
                                    fields = (type_name = Required(Terminal([FixedKeyword])))
                                ),
                                Variant(
                                    name = UnsignedFixedPointNumber,
                                    fields = (type_name = Required(Terminal([UfixedKeyword])))
                                )
                            ]
                        ),
                        Enum(
                            name = AddressType,
                            variants = [
                                Variant(
                                    name = Address,
                                    fields = (
                                        address_keyword = Required(Terminal([AddressKeyword])),
                                        payable_keyword =
                                            Optional(reference = Terminal([PayableKeyword]))
                                    )
                                ),
                                Variant(
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
                            fields = (
                                open_brace = Required(Terminal([OpenBrace])),
                                statements = Required(NonTerminal(Statements)),
                                close_brace = Required(Terminal([CloseBrace]))
                            )
                        ),
                        Repeated(
                            name = Statements,
                            repeated = NonTerminal(Statement),
                            allow_empty = true
                        ),
                        Enum(
                            name = Statement,
                            variants = [
                                Variant(
                                    name = TupleDeconstruction,
                                    fields = (statement =
                                        Required(NonTerminal(TupleDeconstructionStatement)))
                                ),
                                Variant(
                                    name = VariableDeclaration,
                                    fields = (statement =
                                        Required(NonTerminal(VariableDeclarationStatement)))
                                ),
                                Variant(
                                    name = If,
                                    fields = (statement = Required(NonTerminal(IfStatement)))
                                ),
                                Variant(
                                    name = For,
                                    fields = (statement = Required(NonTerminal(ForStatement)))
                                ),
                                Variant(
                                    name = While,
                                    fields = (statement = Required(NonTerminal(WhileStatement)))
                                ),
                                Variant(
                                    name = DoWhile,
                                    fields = (statement = Required(NonTerminal(DoWhileStatement)))
                                ),
                                Variant(
                                    name = Continue,
                                    fields = (statement = Required(NonTerminal(ContinueStatement)))
                                ),
                                Variant(
                                    name = Break,
                                    fields = (statement = Required(NonTerminal(BreakStatement)))
                                ),
                                Variant(
                                    name = Delete,
                                    fields = (statement = Required(NonTerminal(DeleteStatement)))
                                ),
                                Variant(
                                    name = Return,
                                    fields = (statement = Required(NonTerminal(ReturnStatement)))
                                ),
                                Variant(
                                    name = Throw,
                                    disabled_in = "0.5.0",
                                    fields = (statement = Required(NonTerminal(ThrowStatement)))
                                ),
                                Variant(
                                    name = Emit,
                                    enabled_in = "0.4.21",
                                    fields = (statement = Required(NonTerminal(EmitStatement)))
                                ),
                                Variant(
                                    name = Try,
                                    enabled_in = "0.6.0",
                                    fields = (statement = Required(NonTerminal(TryStatement)))
                                ),
                                Variant(
                                    name = Revert,
                                    enabled_in = "0.8.4",
                                    fields = (statement = Required(NonTerminal(RevertStatement)))
                                ),
                                Variant(
                                    name = Assembly,
                                    fields = (statement = Required(NonTerminal(AssemblyStatement)))
                                ),
                                Variant(
                                    name = Block,
                                    fields = (block = Required(NonTerminal(Block)))
                                ),
                                Variant(
                                    name = UncheckedBlock,
                                    enabled_in = "0.8.0",
                                    fields = (block = Required(NonTerminal(UncheckedBlock)))
                                ),
                                Variant(
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
                            separated = NonTerminal(TupleMemberDeconstruction),
                            separator = Terminal([Comma]),
                            allow_empty = true
                        ),
                        Struct(
                            name = TupleMemberDeconstruction,
                            fields = (member = Optional(reference = NonTerminal(TupleMember)))
                        ),
                        Enum(
                            name = TupleMember,
                            variants = [
                                Variant(
                                    name = Typed,
                                    fields = (
                                        type_name = Required(NonTerminal(TypeName)),
                                        storage_location =
                                            Optional(reference = NonTerminal(StorageLocation)),
                                        name = Required(Terminal([Identifier]))
                                    )
                                ),
                                Variant(
                                    name = Untyped,
                                    fields = (
                                        storage_location =
                                            Optional(reference = NonTerminal(StorageLocation)),
                                        name = Required(Terminal([Identifier]))
                                    )
                                )
                            ]
                        ),
                        Struct(
                            name = VariableDeclarationStatement,
                            fields = (
                                variable_type = Required(NonTerminal(VariableDeclarationType)),
                                storage_location =
                                    Optional(reference = NonTerminal(StorageLocation)),
                                name = Required(Terminal([Identifier])),
                                value = Optional(reference = NonTerminal(VariableDeclarationValue)),
                                semicolon = Required(Terminal([Semicolon]))
                            )
                        ),
                        Enum(
                            name = VariableDeclarationType,
                            variants = [
                                Variant(
                                    name = Typed,
                                    fields = (type_name = Required(NonTerminal(TypeName)))
                                ),
                                Variant(
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
                                Variant(
                                    name = Memory,
                                    fields = (keyword = Required(Terminal([MemoryKeyword])))
                                ),
                                Variant(
                                    name = Storage,
                                    fields = (keyword = Required(Terminal([StorageKeyword])))
                                ),
                                Variant(
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
                            fields = (
                                if_keyword = Required(Terminal([IfKeyword])),
                                open_paren = Required(Terminal([OpenParen])),
                                condition = Required(NonTerminal(Expression)),
                                close_paren = Required(Terminal([CloseParen])),
                                body = Required(NonTerminal(Statement)),
                                else_branch = Optional(reference = NonTerminal(ElseBranch))
                            )
                        ),
                        Struct(
                            name = ElseBranch,
                            fields = (
                                else_keyword = Optional(reference = Terminal([ElseKeyword])),
                                body = Optional(reference = NonTerminal(Statement))
                            )
                        ),
                        Struct(
                            name = ForStatement,
                            fields = (
                                for_keyword = Required(Terminal([ForKeyword])),
                                open_paren = Required(Terminal([OpenParen])),
                                initialization = Required(NonTerminal(ForInitialization)),
                                condition = Optional(reference = NonTerminal(Expression)),
                                semicolon = Required(Terminal([Semicolon])),
                                iterator = Optional(reference = NonTerminal(Expression)),
                                close_paren = Required(Terminal([CloseParen])),
                                body = Required(NonTerminal(Statement))
                            )
                        ),
                        Enum(
                            name = ForInitialization,
                            variants = [
                                Variant(
                                    name = Expression,
                                    fields =
                                        (statement = Required(NonTerminal(ExpressionStatement)))
                                ),
                                Variant(
                                    name = VariableDeclaration,
                                    fields = (statement =
                                        Required(NonTerminal(VariableDeclarationStatement)))
                                ),
                                Variant(
                                    name = TupleDeconstruction,
                                    fields = (statement =
                                        Required(NonTerminal(TupleDeconstructionStatement)))
                                ),
                                Variant(
                                    name = None,
                                    fields = (semicolon = Required(Terminal([Semicolon])))
                                )
                            ]
                        ),
                        Struct(
                            name = WhileStatement,
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
                            fields = (
                                continue_keyword = Required(Terminal([ContinueKeyword])),
                                semicolon = Required(Terminal([Semicolon]))
                            )
                        ),
                        Struct(
                            name = BreakStatement,
                            fields = (
                                break_keyword = Required(Terminal([BreakKeyword])),
                                semicolon = Required(Terminal([Semicolon]))
                            )
                        ),
                        Struct(
                            name = ReturnStatement,
                            fields = (
                                return_keyword = Required(Terminal([ReturnKeyword])),
                                expression = Optional(reference = NonTerminal(Expression)),
                                semicolon = Required(Terminal([Semicolon]))
                            )
                        ),
                        Struct(
                            name = EmitStatement,
                            enabled_in = "0.4.21",
                            fields = (
                                emit_keyword = Required(Terminal([EmitKeyword])),
                                event = Required(NonTerminal(IdentifierPath)),
                                arguments = Required(NonTerminal(ArgumentsDeclaration)),
                                semicolon = Required(Terminal([Semicolon]))
                            )
                        ),
                        Struct(
                            name = DeleteStatement,
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
                                returns = Optional(reference = NonTerminal(ReturnsDeclaration)),
                                body = Required(NonTerminal(Block)),
                                catch_clauses = Required(NonTerminal(CatchClauses))
                            )
                        ),
                        Repeated(
                            name = CatchClauses,
                            repeated = NonTerminal(CatchClause),
                            enabled_in = "0.6.0"
                        ),
                        Struct(
                            name = CatchClause,
                            enabled_in = "0.6.0",
                            fields = (
                                catch_keyword = Required(Terminal([CatchKeyword])),
                                error = Optional(reference = NonTerminal(CatchClauseError)),
                                body = Required(NonTerminal(Block))
                            )
                        ),
                        Struct(
                            name = CatchClauseError,
                            enabled_in = "0.6.0",
                            fields = (
                                name = Optional(reference = Terminal([Identifier])),
                                parameters = Required(NonTerminal(ParametersDeclaration))
                            )
                        ),
                        Struct(
                            name = RevertStatement,
                            enabled_in = "0.8.4",
                            fields = (
                                revert_keyword = Required(Terminal([RevertKeyword])),
                                error = Optional(reference = NonTerminal(IdentifierPath)),
                                arguments = Required(NonTerminal(ArgumentsDeclaration)),
                                semicolon = Required(Terminal([Semicolon]))
                            )
                        ),
                        Struct(
                            name = ThrowStatement,
                            disabled_in = "0.5.0",
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
                            operators = [
                                Operator(
                                    expression_name = BinaryExpression,
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
                                ),
                                Operator(
                                    expression_name = ConditionalExpression,
                                    model = Postfix,
                                    fields = (
                                        question_mark = Required(Terminal([QuestionMark])),
                                        true_expression = Required(NonTerminal(Expression)),
                                        colon = Required(Terminal([Colon])),
                                        false_expression = Required(NonTerminal(Expression))
                                    )
                                ),
                                Operator(
                                    expression_name = BinaryExpression,
                                    model = BinaryLeftAssociative,
                                    fields = (operator = Required(Terminal([BarBar])))
                                ),
                                Operator(
                                    expression_name = BinaryExpression,
                                    model = BinaryLeftAssociative,
                                    fields = (operator = Required(Terminal([AmpersandAmpersand])))
                                ),
                                Operator(
                                    expression_name = BinaryExpression,
                                    model = BinaryLeftAssociative,
                                    fields =
                                        (operator = Required(Terminal([EqualEqual, BangEqual])))
                                ),
                                Operator(
                                    expression_name = BinaryExpression,
                                    model = BinaryLeftAssociative,
                                    fields = (operator = Required(Terminal([
                                        LessThan,
                                        GreaterThan,
                                        LessThanEqual,
                                        GreaterThanEqual
                                    ])))
                                ),
                                Operator(
                                    expression_name = BinaryExpression,
                                    model = BinaryLeftAssociative,
                                    fields = (operator = Required(Terminal([Bar])))
                                ),
                                Operator(
                                    expression_name = BinaryExpression,
                                    model = BinaryLeftAssociative,
                                    fields = (operator = Required(Terminal([Caret])))
                                ),
                                Operator(
                                    expression_name = BinaryExpression,
                                    model = BinaryLeftAssociative,
                                    fields = (operator = Required(Terminal([Ampersand])))
                                ),
                                Operator(
                                    expression_name = BinaryExpression,
                                    model = BinaryLeftAssociative,
                                    fields = (operator = Required(Terminal([
                                        LessThanLessThan,
                                        GreaterThanGreaterThan,
                                        GreaterThanGreaterThanGreaterThan
                                    ])))
                                ),
                                Operator(
                                    expression_name = BinaryExpression,
                                    model = BinaryLeftAssociative,
                                    fields = (operator = Required(Terminal([Plus, Minus])))
                                ),
                                Operator(
                                    expression_name = BinaryExpression,
                                    model = BinaryLeftAssociative,
                                    fields =
                                        (operator = Required(Terminal([Asterisk, Slash, Percent])))
                                ),
                                Operator(
                                    // Before '0.6.0', it was left-associative:
                                    expression_name = BinaryExpression,
                                    model = BinaryLeftAssociative,
                                    disabled_in = "0.6.0",
                                    fields = (operator = Required(Terminal([AsteriskAsterisk])))
                                ),
                                Operator(
                                    // After '0.6.0', it became right-associative:
                                    expression_name = BinaryExpression,
                                    model = BinaryRightAssociative,
                                    enabled_in = "0.6.0",
                                    fields = (operator = Required(Terminal([AsteriskAsterisk])))
                                ),
                                Operator(
                                    expression_name = PostfixExpression,
                                    model = Postfix,
                                    fields =
                                        (operator = Required(Terminal([PlusPlus, MinusMinus])))
                                ),
                                Operator(
                                    // Before '0.5.0', 'Plus' was supported:
                                    expression_name = PrefixExpression,
                                    model = Prefix,
                                    disabled_in = "0.5.0",
                                    fields = (operator = Required(Terminal([
                                        PlusPlus, MinusMinus, Tilde, Bang, Minus, Plus
                                    ])))
                                ),
                                Operator(
                                    // After '0.5.0', 'Plus' was removed:
                                    expression_name = PrefixExpression,
                                    model = Prefix,
                                    enabled_in = "0.5.0",
                                    fields = (operator = Required(Terminal([
                                        PlusPlus, MinusMinus, Tilde, Bang, Minus
                                    ])))
                                ),
                                Operator(
                                    expression_name = FunctionCallExpression,
                                    model = Postfix,
                                    fields = (
                                        options = Required(NonTerminal(FunctionCallOptions)),
                                        arguments = Required(NonTerminal(ArgumentsDeclaration))
                                    )
                                ),
                                Operator(
                                    expression_name = MemberAccessExpression,
                                    model = Postfix,
                                    fields = (
                                        period = Required(Terminal([Period])),
                                        member = Required(Terminal([Identifier, AddressKeyword]))
                                    )
                                ),
                                Operator(
                                    expression_name = IndexAccessExpression,
                                    model = Postfix,
                                    fields = (
                                        open_bracket = Required(Terminal([OpenBracket])),
                                        start = Optional(reference = NonTerminal(Expression)),
                                        end = Optional(reference = NonTerminal(IndexAccessEnd)),
                                        close_bracket = Required(Terminal([CloseBracket]))
                                    )
                                )
                            ],
                            primary_expressions = [
                                Variant(
                                    name = NewExpression,
                                    fields = (expression = Required(NonTerminal(NewExpression)))
                                ),
                                Variant(
                                    name = TupleExpression,
                                    fields = (expression = Required(NonTerminal(TupleExpression)))
                                ),
                                Variant(
                                    name = TypeExpression,
                                    enabled_in = "0.5.3",
                                    fields = (expression = Required(NonTerminal(TypeExpression)))
                                ),
                                Variant(
                                    name = ArrayExpression,
                                    fields = (expression = Required(NonTerminal(ArrayExpression)))
                                ),
                                Variant(
                                    name = NumberExpression,
                                    fields = (expression = Required(NonTerminal(NumberExpression)))
                                ),
                                Variant(
                                    name = StringExpression,
                                    fields = (expression = Required(NonTerminal(StringExpression)))
                                ),
                                Variant(
                                    name = ElementaryType,
                                    fields = (expression = Required(NonTerminal(ElementaryType)))
                                ),
                                Variant(
                                    name = BooleanExpression,
                                    fields = (expression =
                                        Required(Terminal([TrueKeyword, FalseKeyword])))
                                ),
                                Variant(
                                    name = Identifier,
                                    fields = (expression = Required(Terminal([Identifier])))
                                )
                            ]
                        ),
                        Struct(
                            name = IndexAccessEnd,
                            fields = (
                                colon = Required(Terminal([Colon])),
                                end = Optional(reference = NonTerminal(Expression))
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
                                Variant(name = None, disabled_in = "0.6.2", fields = ()),
                                Variant(
                                    name = Multiple,
                                    enabled_in = "0.6.2",
                                    disabled_in = "0.8.0",
                                    fields = (options =
                                        Required(NonTerminal(NamedArgumentsDeclarations)))
                                ),
                                Variant(
                                    name = Single,
                                    enabled_in = "0.8.0",
                                    fields = (options = Optional(
                                        reference = NonTerminal(NamedArgumentsDeclaration)
                                    ))
                                )
                            ]
                        ),
                        Enum(
                            name = ArgumentsDeclaration,
                            variants = [
                                Variant(
                                    name = Positional,
                                    fields = (
                                        open_paren = Required(Terminal([OpenParen])),
                                        arguments = Required(NonTerminal(PositionalArguments)),
                                        close_paren = Required(Terminal([CloseParen]))
                                    )
                                ),
                                Variant(
                                    name = Named,
                                    fields = (
                                        open_paren = Required(Terminal([OpenParen])),
                                        arguments = Optional(
                                            reference = NonTerminal(NamedArgumentsDeclaration)
                                        ),
                                        close_paren = Required(Terminal([CloseParen]))
                                    )
                                )
                            ]
                        ),
                        Separated(
                            name = PositionalArguments,
                            separated = NonTerminal(Expression),
                            separator = Terminal([Comma]),
                            allow_empty = true
                        ),
                        Repeated(
                            name = NamedArgumentsDeclarations,
                            repeated = NonTerminal(NamedArgumentsDeclaration),
                            enabled_in = "0.6.2",
                            disabled_in = "0.8.0",
                            allow_empty = true
                        ),
                        Struct(
                            name = NamedArgumentsDeclaration,
                            fields = (
                                open_brace = Required(Terminal([OpenBrace])),
                                arguments = Required(NonTerminal(NamedArguments)),
                                close_brace = Required(Terminal([CloseBrace]))
                            )
                        ),
                        Separated(
                            name = NamedArguments,
                            separated = NonTerminal(NamedArgument),
                            separator = Terminal([Comma]),
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
                            fields = (
                                open_paren = Required(Terminal([OpenParen])),
                                items = Required(NonTerminal(TupleValues)),
                                close_paren = Required(Terminal([CloseParen]))
                            )
                        ),
                        Separated(
                            name = TupleValues,
                            separated = NonTerminal(TupleValue),
                            separator = Terminal([Comma])
                        ),
                        Struct(
                            name = TupleValue,
                            fields = (expression = Optional(reference = NonTerminal(Expression)))
                        ),
                        Struct(
                            name = ArrayExpression,
                            fields = (
                                open_bracket = Required(Terminal([OpenBracket])),
                                items = Required(NonTerminal(ArrayValues)),
                                close_bracket = Required(Terminal([CloseBracket]))
                            )
                        ),
                        Separated(
                            name = ArrayValues,
                            separated = NonTerminal(Expression),
                            separator = Terminal([Comma])
                        )
                    ]
                ),
                Topic(
                    title = "Numbers",
                    items = [
                        Enum(
                            name = NumberExpression,
                            variants = [
                                Variant(
                                    name = Hex,
                                    fields = (
                                        literal = Required(Terminal([HexLiteral])),
                                        unit = Optional(
                                            reference = NonTerminal(NumberUnit),
                                            disabled_in = "0.5.0"
                                        )
                                    )
                                ),
                                Variant(
                                    name = Decimal,
                                    fields = (
                                        literal = Required(Terminal([DecimalLiteral])),
                                        unit = Optional(reference = NonTerminal(NumberUnit))
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
                                OneOrMore(Range(inclusive_start = '0', exclusive_end = '9')),
                                ZeroOrMore(Sequence([
                                    Atom("_"),
                                    OneOrMore(Range(inclusive_start = '0', exclusive_end = '9'))
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
                                Variant(
                                    name = Wei, // 1e-18 ETH
                                    fields = (keyword = Required(Terminal([WeiKeyword])))
                                ),
                                Variant(
                                    name = Gwei, // 1e-9 ETH
                                    enabled_in = "0.6.11",
                                    fields = (keyword = Required(Terminal([GweiKeyword])))
                                ),
                                Variant(
                                    name = Szabo, // 1e-6 ETH
                                    disabled_in = "0.7.0",
                                    fields = (keyword = Required(Terminal([SzaboKeyword])))
                                ),
                                Variant(
                                    name = Finney, // 1e-3 ETH
                                    disabled_in = "0.7.0",
                                    fields = (keyword = Required(Terminal([FinneyKeyword])))
                                ),
                                Variant(
                                    name = Ether, // 1 ETH
                                    fields = (keyword = Required(Terminal([EtherKeyword])))
                                ),
                                Variant(
                                    name = Seconds,
                                    fields = (keyword = Required(Terminal([SecondsKeyword])))
                                ),
                                Variant(
                                    name = Minutes,
                                    fields = (keyword = Required(Terminal([MinutesKeyword])))
                                ),
                                Variant(
                                    name = Hours,
                                    fields = (keyword = Required(Terminal([HoursKeyword])))
                                ),
                                Variant(
                                    name = Days,
                                    fields = (keyword = Required(Terminal([DaysKeyword])))
                                ),
                                Variant(
                                    name = Weeks,
                                    fields = (keyword = Required(Terminal([WeeksKeyword])))
                                ),
                                Variant(
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
                                Variant(
                                    name = Hex,
                                    fields = (literals = Required(NonTerminal(HexStringLiterals)))
                                ),
                                Variant(
                                    name = Ascii,
                                    fields =
                                        (literals = Required(NonTerminal(AsciiStringLiterals)))
                                ),
                                Variant(
                                    name = Unicode,
                                    enabled_in = "0.7.0",
                                    fields =
                                        (literals = Required(NonTerminal(UnicodeStringLiterals)))
                                )
                            ]
                        ),
                        Repeated(
                            name = HexStringLiterals,
                            repeated = Terminal([HexStringLiteral])
                        ),
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
                                Range(inclusive_start = '0', exclusive_end = '9'),
                                Range(inclusive_start = 'a', exclusive_end = 'f'),
                                Range(inclusive_start = 'A', exclusive_end = 'F')
                            ])
                        ),
                        Repeated(
                            name = AsciiStringLiterals,
                            repeated = Terminal([AsciiStringLiteral])
                        ),
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
                                    Range(inclusive_start = ' ', exclusive_end = '&'),
                                    Range(inclusive_start = '(', exclusive_end = '['),
                                    Range(inclusive_start = ']', exclusive_end = '~')
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
                                    Range(inclusive_start = ' ', exclusive_end = '!'),
                                    Range(inclusive_start = '#', exclusive_end = '['),
                                    Range(inclusive_start = ']', exclusive_end = '~')
                                ])),
                                Atom("\"")
                            ])
                        ),
                        Repeated(
                            name = UnicodeStringLiterals,
                            repeated = Terminal([UnicodeStringLiteral]),
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
                            separated = Terminal([Identifier]),
                            separator = Terminal([Period])
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
                                Range(inclusive_start = 'a', exclusive_end = 'z'),
                                Range(inclusive_start = 'A', exclusive_end = 'Z')
                            ])
                        ),
                        Fragment(
                            name = IdentifierPart,
                            scanner = Choice([
                                Fragment(IdentifierStart),
                                Range(inclusive_start = '0', exclusive_end = '9')
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
                                label = Optional(reference = Terminal([AsciiStringLiteral])),
                                flags = Optional(reference = NonTerminal(AssemblyFlagsDeclaration)),
                                body = Required(NonTerminal(YulBlock))
                            )
                        ),
                        Struct(
                            name = AssemblyFlagsDeclaration,
                            fields = (
                                open_paren = Required(Terminal([OpenParen])),
                                flags = Required(NonTerminal(AssemblyFlags)),
                                close_paren = Required(Terminal([CloseParen]))
                            )
                        ),
                        Separated(
                            name = AssemblyFlags,
                            separated = Terminal([AsciiStringLiteral]),
                            separator = Terminal([Comma]),
                            allow_empty = true
                        ),
                        Struct(
                            name = YulBlock,
                            fields = (
                                open_paren = Required(Terminal([OpenParen])),
                                statements = Required(NonTerminal(YulStatements)),
                                close_paren = Required(Terminal([CloseParen]))
                            )
                        ),
                        Repeated(
                            name = YulStatements,
                            repeated = NonTerminal(YulStatement),
                            allow_empty = true
                        ),
                        Enum(
                            name = YulStatement,
                            variants = [
                                Variant(
                                    name = Block,
                                    fields = (block = Required(NonTerminal(YulBlock)))
                                ),
                                Variant(
                                    name = Function,
                                    fields =
                                        (definition = Required(NonTerminal(YulFunctionDefinition)))
                                ),
                                Variant(
                                    name = VariableDeclaration,
                                    fields = (statement =
                                        Required(NonTerminal(YulVariableDeclarationStatement)))
                                ),
                                Variant(
                                    name = Assignment,
                                    fields =
                                        (statement = Required(NonTerminal(YulAssignmentStatement)))
                                ),
                                Variant(
                                    name = If,
                                    fields = (statement = Required(NonTerminal(YulIfStatement)))
                                ),
                                Variant(
                                    name = For,
                                    fields = (statement = Required(NonTerminal(YulForStatement)))
                                ),
                                Variant(
                                    name = Switch,
                                    fields =
                                        (statement = Required(NonTerminal(YulSwitchStatement)))
                                ),
                                Variant(
                                    name = Leave,
                                    enabled_in = "0.6.0",
                                    fields = (statement = Required(NonTerminal(YulLeaveStatement)))
                                ),
                                Variant(
                                    name = Break,
                                    fields = (statement = Required(NonTerminal(YulBreakStatement)))
                                ),
                                Variant(
                                    name = Continue,
                                    fields =
                                        (statement = Required(NonTerminal(YulContinueStatement)))
                                ),
                                Variant(
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
                                returns = Optional(reference = NonTerminal(YulReturnsDeclaration)),
                                body = Required(NonTerminal(YulBlock))
                            )
                        ),
                        Struct(
                            name = YulParametersDeclaration,
                            fields = (
                                open_paren = Required(Terminal([OpenParen])),
                                parameters = Required(NonTerminal(YulParameters)),
                                close_paren = Required(Terminal([CloseParen]))
                            )
                        ),
                        Separated(
                            name = YulParameters,
                            separated = Terminal([YulIdentifier]),
                            separator = Terminal([Comma]),
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
                            separated = Terminal([YulIdentifier]),
                            separator = Terminal([Comma])
                        ),
                        Struct(
                            name = YulVariableDeclarationStatement,
                            fields = (
                                let_keyword = Required(Terminal([YulLetKeyword])),
                                names = Required(NonTerminal(YulIdentifierPaths)),
                                value =
                                    Optional(reference = NonTerminal(YulVariableDeclarationValue))
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
                        Repeated(name = YulSwitchCases, repeated = NonTerminal(YulSwitchCase)),
                        Enum(
                            name = YulSwitchCase,
                            variants = [
                                Variant(
                                    name = Default,
                                    fields = (
                                        default_keyword = Required(Terminal([YulDefaultKeyword])),
                                        body = Required(NonTerminal(YulBlock))
                                    )
                                ),
                                Variant(
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
                            operators = [Operator(
                                expression_name = YulFunctionCallExpression,
                                model = Postfix,
                                fields = (
                                    open_paren = Required(Terminal([OpenParen])),
                                    arguments = Required(NonTerminal(YulArguments)),
                                    close_paren = Required(Terminal([CloseParen]))
                                )
                            )],
                            primary_expressions = [
                                Variant(
                                    name = YulLiteral,
                                    fields = (expression = Required(NonTerminal(YulLiteral)))
                                ),
                                Variant(
                                    name = YulIdentifierPath,
                                    fields =
                                        (expression = Required(NonTerminal(YulIdentifierPath)))
                                )
                            ]
                        ),
                        Separated(
                            name = YulArguments,
                            separated = NonTerminal(YulExpression),
                            separator = Terminal([Comma]),
                            allow_empty = true
                        ),
                        Separated(
                            name = YulIdentifierPaths,
                            separated = NonTerminal(YulIdentifierPath),
                            separator = Terminal([Comma])
                        ),
                        Separated(
                            name = YulIdentifierPath,
                            separated = Terminal([YulIdentifier]),
                            separator = Terminal([Period])
                        ),
                        Token(
                            name = YulIdentifier,
                            definitions = [TokenDefinition(scanner = Fragment(RawIdentifier))]
                        ),
                        Enum(
                            name = YulLiteral,
                            variants = [
                                Variant(
                                    name = Boolean,
                                    fields = (literal =
                                        Required(Terminal([YulTrueKeyword, YulFalseKeyword])))
                                ),
                                Variant(
                                    name = Decimal,
                                    fields = (literal = Required(Terminal([YulDecimalLiteral])))
                                ),
                                Variant(
                                    name = Hex,
                                    fields = (literal = Required(Terminal([YulHexLiteral])))
                                ),
                                Variant(
                                    name = HexString,
                                    fields = (literal = Required(Terminal([HexStringLiteral])))
                                ),
                                Variant(
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
                                            Range(inclusive_start = '1', exclusive_end = '9'),
                                            ZeroOrMore(Range(
                                                inclusive_start = '0',
                                                exclusive_end = '9'
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
);
