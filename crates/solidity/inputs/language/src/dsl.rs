use codegen_grammar::*;

slang_grammar! {

    config = {

        name = "Solidity" ;

        versions = [
            "0.4.11" , "0.4.12" , "0.4.13" , "0.4.14" , "0.4.15" , "0.4.16" , "0.4.17" , "0.4.18" , "0.4.19" , "0.4.20" ,
            "0.4.21" , "0.4.22" , "0.4.23" , "0.4.24" , "0.4.25" , "0.4.26" ,

            "0.5.0" , "0.5.1" , "0.5.2" , "0.5.3" , "0.5.4" , "0.5.5" , "0.5.6" , "0.5.7" , "0.5.8" , "0.5.9" , "0.5.10" ,
            "0.5.11" , "0.5.12" , "0.5.13" , "0.5.14" , "0.5.15" , "0.5.16" , "0.5.17" ,

            "0.6.0" , "0.6.1" , "0.6.2" , "0.6.3" , "0.6.4" , "0.6.5" , "0.6.6" , "0.6.7" , "0.6.8" , "0.6.9" , "0.6.10" ,
            "0.6.11" , "0.6.12" ,

            "0.7.0" , "0.7.1" , "0.7.2" , "0.7.3" , "0.7.4" , "0.7.5" , "0.7.6" ,

            "0.8.0" , "0.8.1" , "0.8.2" , "0.8.3" , "0.8.4" , "0.8.5" , "0.8.6" , "0.8.7" , "0.8.8" , "0.8.9" , "0.8.10" ,
            "0.8.11" , "0.8.12" , "0.8.13" , "0.8.14" , "0.8.15" , "0.8.16" , "0.8.17" , "0.8.18" , "0.8.19" , "0.8.20" ,
            "0.8.21" , "0.8.22"
        ] ;

        leading trivia parser = LeadingTrivia ;
        trailing trivia parser = TrailingTrivia ;

    } ;

    /********************************************
     *         Parsers
     ********************************************/

    lexical context Default = {

        inline parser SolidityKeywordsOverAllVersions = (
              AbicoderKeyword
            | AbstractKeyword
            | AddressKeyword
            | AfterKeyword
            | AnonymousKeyword
            | AsKeyword
            | AssemblyKeyword
            | BoolKeyword
            | BreakKeyword
            | ByteKeyword
            | CaseKeyword
            | CatchKeyword
            | ConstantKeyword
            | ContinueKeyword
            | ContractKeyword
            | DaysKeyword
            | DefaultKeyword
            | DeleteKeyword
            | DoKeyword
            | ElseKeyword
            | EnumKeyword
            | EtherKeyword
            | EventKeyword
            | ExperimentalKeyword
            | ExternalKeyword
            | FallbackKeyword
            | FalseKeyword
            | FinalKeyword
            | FinneyKeyword
            | ForKeyword
            | FromKeyword
            | FunctionKeyword
            | HexKeyword
            | HoursKeyword
            | IfKeyword
            | ImportKeyword
            | IndexedKeyword
            | InKeyword
            | InlineKeyword
            | InterfaceKeyword
            | InternalKeyword
            | IsKeyword
            | LetKeyword
            | LibraryKeyword
            | MappingKeyword
            | MatchKeyword
            | MemoryKeyword
            | MinutesKeyword
            | ModifierKeyword
            | NewKeyword
            | NullKeyword
            | OfKeyword
            | OverrideKeyword
            | PayableKeyword
            | PragmaKeyword
            | PrivateKeyword
            | PublicKeyword
            | PureKeyword
            | ReceiveKeyword
            | RelocatableKeyword
            | ReturnKeyword
            | ReturnsKeyword
            | SecondsKeyword
            | SolidityKeyword
            | StaticKeyword
            | StorageKeyword
            | StringKeyword
            | StructKeyword
            | SwitchKeyword
            | SzaboKeyword
            | ThrowKeyword
            | TrueKeyword
            | TypeofKeyword
            | UsingKeyword
            | VarKeyword
            | ViewKeyword
            | WeeksKeyword
            | WeiKeyword
            | WhileKeyword
            | YearsKeyword

            // Introduced in 0.4.21
            | EmitKeyword

            // Introduced in 0.4.22
            | ConstructorKeyword

            // introduced in 0.5.0
            | AliasKeyword
            | ApplyKeyword
            | AutoKeyword
            | CalldataKeyword
            | CopyofKeyword
            | DefineKeyword
            | ImplementsKeyword
            | MacroKeyword
            | MutableKeyword
            | PartialKeyword
            | PromiseKeyword
            | ReferenceKeyword
            | SealedKeyword
            | SizeofKeyword
            | SupportsKeyword
            | TypedefKeyword

            // Introduced in 0.5.3
            | TypeKeyword

            // Introduced in 0.6.0
            | LeaveKeyword
            | TryKeyword
            | VirtualKeyword

            // Introduced in 0.6.5
            | ImmutableKeyword

            // Introduced in 0.6.11
            | GweiKeyword

            // Introduced in 0.8.0
            | UncheckedKeyword

            // Introduced in 0.8.4
            | ErrorKeyword
            | RevertKeyword

            // Introduced in 0.8.13
            | GlobalKeyword
        ) ;

        parser ABICoderPragma = (AbicoderKeyword Identifier) ;

        inline parser AddSubOperator = (Plus | Minus) ;

        parser AddressType = ((AddressKeyword (PayableKeyword ?)) | PayableKeyword) ;

        inline parser AndOperator = AmpersandAmpersand ;

        parser ArgumentsDeclaration = (((PositionalArguments | NamedArgumentsDeclaration) ?) delimited by OpenParen and CloseParen) ;

        parser ArrayExpression = (ArrayValues delimited by OpenBracket and CloseBracket) ;

        inline parser ArrayTypeNameOperator = ((Expression ?) delimited by OpenBracket and CloseBracket) ;

        parser ArrayValues = (Expression separated by Comma) ;

        parser AsciiStringLiterals = (AsciiStringLiteral +) ;

        parser AssemblyFlags = (AsciiStringLiteral separated by Comma) ;

        parser AssemblyStatement = (AssemblyKeyword (AsciiStringLiteral ?) (AssemblyFlagsDeclaration ?) YulBlock) ;

        parser AssemblyFlagsDeclaration = (AssemblyFlags delimited by OpenParen and CloseParen) ;

        inline parser AssignmentOperator = (
            Equal | BarEqual | PlusEqual | MinusEqual | CaretEqual | SlashEqual | PercentEqual | AsteriskEqual | AmpersandEqual |
            LessThanLessThanEqual | GreaterThanGreaterThanEqual | GreaterThanGreaterThanGreaterThanEqual
        ) ;

        inline parser BitwiseAndOperator = Ampersand ;

        inline parser BitwiseOrOperator = Bar ;

        inline parser BitwiseXOrOperator = Caret ;

        parser Block = ((Statements ?) delimited by OpenBrace and CloseBrace) ;

        inline parser BooleanExpression = (TrueKeyword | FalseKeyword) ;

        parser BreakStatement = (BreakKeyword terminated by Semicolon) ;

        parser CatchClause = { introduced in "0.6.0" (CatchKeyword (CatchClauseError ?) Block) } ;

        parser CatchClauseError = { introduced in "0.6.0" ((Identifier ?) ParametersDeclaration) } ;

        parser CatchClauses = { introduced in "0.6.0" (CatchClause +) } ;

        inline parser ConditionalOperator = (QuestionMark Expression Colon Expression) ;

        parser ConstantDefinition = { introduced in "0.7.4" ((TypeName ConstantKeyword Identifier Equal Expression) terminated by Semicolon) } ;

        inline parser ConstructorAttribute = { introduced in "0.4.22" (ModifierInvocation | InternalKeyword | PayableKeyword | PublicKeyword) } ;

        parser ConstructorAttributes = { introduced in "0.4.22" (ConstructorAttribute +) } ;

        parser ConstructorDefinition = { introduced in "0.4.22" (ConstructorKeyword ParametersDeclaration (ConstructorAttributes ?) Block) } ;

        parser ContinueStatement = (ContinueKeyword terminated by Semicolon) ;

        parser ContractDefinition = (
            { introduced in "0.6.0" (AbstractKeyword ?) } ContractKeyword Identifier (InheritanceSpecifier ?) ((ContractMembers ?) delimited by OpenBrace and CloseBrace)
        ) ;

        inline parser ContractMember = (
            UsingDirective | FunctionDefinition | ModifierDefinition | StructDefinition | EnumDefinition | EventDefinition |  StateVariableDefinition |
            { introduced in "0.4.22" ConstructorDefinition } |
            { introduced in "0.6.0"  (FallbackFunctionDefinition | ReceiveFunctionDefinition) } |
            { removed in "0.6.0"     UnnamedFunctionDefinition } |
            { introduced in "0.8.4"  ErrorDefinition } |
            { introduced in "0.8.8"  UserDefinedValueTypeDefinition }
        ) ;

        parser ContractMembers = (ContractMember +) ;

        inline parser ControlStatement = (
            IfStatement | ForStatement | WhileStatement | DoWhileStatement | ContinueStatement | BreakStatement | DeleteStatement | ReturnStatement |
            { introduced in "0.4.21" EmitStatement } |
            { removed in "0.5.0"     ThrowStatement } |
            { introduced in "0.6.0"  TryStatement } |
            { introduced in "0.8.4"  RevertStatement }
        ) ;

        inline parser DataLocation = (
            MemoryKeyword | StorageKeyword |
            { introduced in "0.5.0" CalldataKeyword}
        ) ;

        parser ImportDeconstruction = ((ImportDeconstructionSymbols delimited by OpenBrace and CloseBrace) FromKeyword AsciiStringLiteral) ;

        parser ImportDeconstructionSymbol = (Identifier ((AsKeyword Identifier) ?)) ;

        parser ImportDeconstructionSymbols = (ImportDeconstructionSymbol separated by Comma) ;

        parser DeleteStatement = ((DeleteKeyword Expression) terminated by Semicolon) ;

        parser DoWhileStatement = ((DoKeyword Statement WhileKeyword (Expression delimited by OpenParen and CloseParen)) terminated by Semicolon) ;

        inline parser ElementaryType = (
            BoolKeyword | StringKeyword | AddressType | BytesKeyword | IntKeyword | UintKeyword | FixedKeyword | UfixedKeyword |
            { removed in "0.8.0" ByteKeyword}
        ) ;

        parser EmitStatement = { introduced in "0.4.21" ((EmitKeyword IdentifierPath ArgumentsDeclaration) terminated by Semicolon) } ;

        parser EnumDefinition = (EnumKeyword Identifier ((EnumMembers ?) delimited by OpenBrace and CloseBrace)) ;

        inline parser EqualityComparisonOperator = (EqualEqual | BangEqual) ;

        parser ErrorDefinition = { introduced in "0.8.4" ((ErrorKeyword Identifier ErrorParametersDeclaration) terminated by Semicolon) } ;

        parser ErrorParametersDeclaration = { introduced in "0.8.4" ((ErrorParameters ?) delimited by OpenParen and CloseParen) } ;

        parser ErrorParameter = { introduced in "0.8.4" (TypeName (Identifier ?)) } ;

        parser ErrorParameters = { introduced in "0.8.4" (ErrorParameter separated by Comma) } ;

        parser EventDefinition = ((EventKeyword Identifier EventParametersDeclaration (AnonymousKeyword ?)) terminated by Semicolon) ;

        parser EventParametersDeclaration = ((EventParameters ?) delimited by OpenParen and CloseParen);

        parser EventParameter = (TypeName (IndexedKeyword ?) (Identifier ?)) ;

        parser EventParameters = (EventParameter separated by Comma) ;

        parser ExperimentalPragma = (ExperimentalKeyword (AsciiStringLiteral | Identifier)) ;

        inline parser ExponentiationOperator = AsteriskAsterisk ;

        precedence parser Expression = (
            [
                left AssignmentOperator as BinaryExpression,
                postfix ConditionalOperator as ConditionalExpression,
                left OrOperator as BinaryExpression,
                left AndOperator as BinaryExpression,
                left EqualityComparisonOperator as BinaryExpression,
                left OrderComparisonOperator as BinaryExpression,
                left BitwiseOrOperator as BinaryExpression,
                left BitwiseXOrOperator as BinaryExpression,
                left BitwiseAndOperator as BinaryExpression,
                left ShiftOperator as BinaryExpression,
                left AddSubOperator as BinaryExpression,
                left MulDivModOperator as BinaryExpression,
                { removed in "0.6.0"    left  ExponentiationOperator as BinaryExpression },
                { introduced in "0.6.0" right ExponentiationOperator as BinaryExpression },
                postfix UnaryPostfixOperator as UnaryPostfixExpression,
                prefix UnaryPrefixOperator as UnaryPrefixExpression,
                postfix FunctionCallOperator as FunctionCallExpression,
                postfix MemberAccessOperator as MemberAccessExpression,
                postfix IndexAccessOperator as IndexAccessExpression
            ]
            with primary expression PrimaryExpression
        ) ;

        parser ExpressionStatement = (Expression terminated by Semicolon) ;

        inline parser FallbackFunctionAttribute = { introduced in "0.6.0" (ModifierInvocation | OverrideSpecifier | ExternalKeyword | PayableKeyword | PureKeyword | ViewKeyword | VirtualKeyword) } ;

        parser FallbackFunctionAttributes = { introduced in "0.6.0" (FallbackFunctionAttribute +) } ;

        parser FallbackFunctionDefinition = { introduced in "0.6.0" (FallbackKeyword ParametersDeclaration (FallbackFunctionAttributes ?) (ReturnsDeclaration ?) (Semicolon | Block)) } ;

        parser ForStatement = (ForKeyword (((SimpleStatement | Semicolon) (ExpressionStatement | Semicolon) (Expression ?)) delimited by OpenParen and CloseParen) Statement) ;

        inline parser FunctionAttribute = (
            ModifierInvocation | OverrideSpecifier | ExternalKeyword | InternalKeyword | PayableKeyword | PrivateKeyword | PublicKeyword | PureKeyword | ViewKeyword |
            { removed in "0.5.0"    ConstantKeyword } |
            { introduced in "0.6.0" VirtualKeyword }
         ) ;

        parser FunctionAttributes = (FunctionAttribute +) ;

        inline parser FunctionCallOperator = (
            { introduced in "0.6.2" (FunctionCallOptions ?) }
            ArgumentsDeclaration
        ) ;

        parser FunctionCallOptions = (
            { introduced in "0.6.2" and removed in "0.8.0" (NamedArgumentsDeclaration +) } |
            { introduced in "0.8.0"                        NamedArgumentsDeclaration     }
        ) ;

        parser FunctionDefinition = (FunctionKeyword (Identifier | FallbackKeyword | ReceiveKeyword) ParametersDeclaration (FunctionAttributes ?) (ReturnsDeclaration ?) (Semicolon | Block)) ;

        parser FunctionType = (FunctionKeyword ParametersDeclaration (FunctionTypeAttributes ?) (ReturnsDeclaration ?)) ;

        inline parser FunctionTypeAttribute = (InternalKeyword | ExternalKeyword | PrivateKeyword | PublicKeyword | PureKeyword | ViewKeyword | PayableKeyword) ;

        parser FunctionTypeAttributes = (FunctionTypeAttribute +) ;

        parser HexStringLiterals = (HexStringLiteral +) ;

        parser IdentifierPath = (Identifier separated by Period) ;

        parser OverridePaths = (IdentifierPath separated by Comma) ;

        parser EnumMembers = (Identifier separated by Comma) ;

        parser IfStatement = (IfKeyword (Expression delimited by OpenParen and CloseParen) Statement ((ElseKeyword Statement) ?)) ;

        parser ImportDirective = ((ImportKeyword (PathImport | NamedImport | ImportDeconstruction)) terminated by Semicolon) ;

        inline parser IndexAccessOperator = (((Expression ?) ((Colon (Expression ?)) ?)) delimited by OpenBracket and CloseBracket) ;

        parser InheritanceSpecifier = (IsKeyword InheritanceTypes) ;

        parser InheritanceType = (IdentifierPath (ArgumentsDeclaration ?)) ;

        parser InheritanceTypes = (InheritanceType separated by Comma) ;

        parser InterfaceDefinition = (InterfaceKeyword Identifier (InheritanceSpecifier ?) ((InterfaceMembers ?) delimited by OpenBrace and CloseBrace)) ;

        parser InterfaceMembers = (ContractMember +) ;

        parser LibraryDefinition = (LibraryKeyword Identifier ((LibraryMembers ?) delimited by OpenBrace and CloseBrace)) ;

        parser LibraryMembers = (ContractMember +) ;

        parser MappingKeyType = (
            (ElementaryType | IdentifierPath)
            { introduced in "0.8.18" (Identifier ?) }
        ) ;

        parser MappingType = (MappingKeyword ((MappingKeyType EqualGreaterThan MappingValueType) delimited by OpenParen and CloseParen)) ;

        parser MappingValueType = (
            TypeName
            { introduced in "0.8.18" (Identifier ?) }
        ) ;

        inline parser MemberAccessOperator = (Period (Identifier | AddressKeyword)) ;

        inline parser ModifierAttribute = (
            OverrideSpecifier |
            { introduced in "0.6.0" VirtualKeyword }
        ) ;

        parser ModifierAttributes = (ModifierAttribute +) ;

        parser ModifierDefinition = (ModifierKeyword Identifier (ParametersDeclaration ?) (ModifierAttributes ?) (Semicolon | Block)) ;

        parser ModifierInvocation = (IdentifierPath (ArgumentsDeclaration ?)) ;

        inline parser MulDivModOperator = (Asterisk | Slash | Percent) ;

        parser NamedArgument = (Identifier Colon Expression) ;

        parser NamedArgumentsDeclaration = ((NamedArguments ?) delimited by OpenBrace and CloseBrace) ;

        parser NamedArguments = (NamedArgument separated by Comma) ;

        parser NamedImport = (Asterisk AsKeyword Identifier FromKeyword AsciiStringLiteral) ;

        parser NewExpression = (NewKeyword TypeName) ;

        inline parser NumberUnit = (
            DaysKeyword | EtherKeyword | HoursKeyword | MinutesKeyword | SecondsKeyword | WeeksKeyword | WeiKeyword |
            { removed in "0.5.0"     YearsKeyword } |
            { introduced in "0.6.11" GweiKeyword } |
            { removed in "0.7.0"     (FinneyKeyword | SzaboKeyword) }
        ) ;

        inline parser NumericExpression = ( DecimalNumberExpression | HexNumberExpression ) ;

        parser DecimalNumberExpression = (DecimalLiteral (NumberUnit ?)) ;
        parser HexNumberExpression = (HexLiteral { removed in "0.5.0" (NumberUnit ?) }) ;

        inline parser OrOperator = BarBar ;

        inline parser OrderComparisonOperator = (LessThan | GreaterThan | LessThanEqual | GreaterThanEqual) ;

        parser OverrideSpecifier = (OverrideKeyword (((OverridePaths ?) delimited by OpenParen and CloseParen) ?)) ;

        parser Parameter = (TypeName (DataLocation ?) (Identifier ?)) ;

        parser ParametersDeclaration = ((Parameters ?) delimited by OpenParen and CloseParen) ;

        parser Parameters = (Parameter separated by Comma) ;

        parser PathImport = (AsciiStringLiteral ((AsKeyword Identifier) ?)) ;

        parser PositionalArguments = (Expression separated by Comma) ;

        parser PragmaDirective = ((PragmaKeyword (ABICoderPragma | ExperimentalPragma | VersionPragma)) terminated by Semicolon) ;

        inline parser PrimaryExpression = (
            NewExpression | TupleExpression | ArrayExpression | BooleanExpression | NumericExpression | StringExpression | ElementaryType | Identifier |
            { introduced in "0.5.3" TypeExpression }
        ) ;

        inline parser ReceiveFunctionAttribute = { introduced in "0.6.0" (ModifierInvocation | OverrideSpecifier | ExternalKeyword | PayableKeyword | VirtualKeyword) } ;

        parser ReceiveFunctionAttributes = { introduced in "0.6.0" (ReceiveFunctionAttribute +) } ;

        parser ReceiveFunctionDefinition = { introduced in "0.6.0" (ReceiveKeyword ParametersDeclaration (ReceiveFunctionAttributes ?) (Semicolon | Block)) } ;

        parser ReturnStatement = ((ReturnKeyword (Expression ?)) terminated by Semicolon) ;

        parser ReturnsDeclaration = (ReturnsKeyword ParametersDeclaration) ;

        parser RevertStatement = ((RevertKeyword (IdentifierPath ?) ArgumentsDeclaration) terminated by Semicolon) ;

        inline parser ShiftOperator = (LessThanLessThan | GreaterThanGreaterThan | GreaterThanGreaterThanGreaterThan) ;

        inline parser SimpleStatement = (ExpressionStatement | VariableDeclarationStatement | TupleDeconstructionStatement) ;

        parser SourceUnit = ((SourceUnitMembers ?) (EndOfFileTrivia ?)) ;

        inline parser SourceUnitMember = (
              PragmaDirective | ImportDirective | ContractDefinition | InterfaceDefinition | LibraryDefinition |
              { introduced in "0.6.0"  (StructDefinition | EnumDefinition) } |
              { introduced in "0.7.1"  FunctionDefinition } |
              { introduced in "0.7.4"  ConstantDefinition } |
              { introduced in "0.8.4"  ErrorDefinition } |
              { introduced in "0.8.8"  UserDefinedValueTypeDefinition } |
              { introduced in "0.8.13" UsingDirective } |
              { introduced in "0.8.22" EventDefinition }
        ) ;

        parser SourceUnitMembers = (SourceUnitMember +) ;

        inline parser StateVariableAttribute = (
            OverrideSpecifier | ConstantKeyword | InternalKeyword | PrivateKeyword | PublicKeyword| { introduced in "0.6.5" ImmutableKeyword }
        ) ;

        parser StateVariableAttributes = (StateVariableAttribute +) ;

        parser StateVariableDefinition = ((TypeName (StateVariableAttributes ?) Identifier ((Equal Expression) ?)) terminated by Semicolon) ;

        inline parser Statement = (
            SimpleStatement | ControlStatement | AssemblyStatement | Block | { introduced in "0.8.0" UncheckedBlock }
        ) ;

        parser Statements = (Statement +) ;

        inline parser StringExpression = (
            HexStringLiterals | AsciiStringLiterals | { introduced in "0.7.0" UnicodeStringLiterals }
        ) ;

        parser StructDefinition = (StructKeyword Identifier ((StructMembers ?) delimited by OpenBrace and CloseBrace)) ;

        parser StructMember = ((TypeName Identifier) terminated by Semicolon) ;

        parser StructMembers = (StructMember +) ;

        parser ThrowStatement = { removed in "0.5.0" (ThrowKeyword terminated by Semicolon) } ;

        parser TryStatement = { introduced in "0.6.0" (TryKeyword Expression (ReturnsDeclaration ?) Block CatchClauses) } ;

        parser TupleDeconstructionStatement = (((TupleDeconstructionElements delimited by OpenParen and CloseParen) Equal Expression) terminated by Semicolon) ;

        parser TupleExpression = (TupleValues delimited by OpenParen and CloseParen) ;

        parser TupleDeconstructionElement = ((TypedTupleMember | UntypedTupleMember) ?) ;

        parser TupleDeconstructionElements = (TupleDeconstructionElement separated by Comma) ;

        parser TupleValues = ((Expression ?) separated by Comma) ;

        parser TypeExpression = { introduced in "0.5.3" (TypeKeyword (TypeName delimited by OpenParen and CloseParen)) } ;

        parser TypedTupleMember = (TypeName (DataLocation ?) Identifier) ;

        precedence parser TypeName = (
            [
                postfix ArrayTypeNameOperator as ArrayTypeName
            ]
            with primary expression (FunctionType | MappingType | ElementaryType | IdentifierPath)
        ) ;

        inline parser UnaryPostfixOperator = (PlusPlus | MinusMinus) ;

        inline parser UnaryPrefixOperator = (
            PlusPlus | MinusMinus | Tilde | Bang | Minus | { removed in "0.5.0" Plus }
        ) ;

        parser UncheckedBlock = { introduced in "0.8.0" (UncheckedKeyword Block) } ;

        parser UnicodeStringLiterals = { introduced in "0.7.0" (UnicodeStringLiteral +) } ;

        inline parser UnnamedFunctionAttribute = { removed in "0.6.0" (ModifierInvocation | OverrideSpecifier | ExternalKeyword | PayableKeyword | PureKeyword | ViewKeyword) } ;

        parser UnnamedFunctionAttributes = { removed in "0.6.0" (UnnamedFunctionAttribute +) } ;

        parser UnnamedFunctionDefinition = { removed in "0.6.0" (FunctionKeyword ParametersDeclaration (UnnamedFunctionAttributes ?) (Semicolon | Block)) } ;

        parser UntypedTupleMember = ((DataLocation ?) Identifier) ;

        parser UserDefinedValueTypeDefinition = { introduced in "0.8.8" ((TypeKeyword Identifier IsKeyword ElementaryType) terminated by Semicolon) } ;

        parser UsingDirective = ((UsingKeyword (IdentifierPath | { introduced in "0.8.13" UsingDeconstruction }) ForKeyword (Asterisk | TypeName) ({ introduced in "0.8.13" GlobalKeyword } ?)) terminated by Semicolon) ;

        parser UsingDeconstruction = { introduced in "0.8.13" (UsingDeconstructionSymbols delimited by OpenBrace and CloseBrace) };

        inline parser UsingDirectiveOperator = {
            introduced in "0.8.19" (Ampersand | Asterisk | BangEqual | Bar | Caret | EqualEqual | GreaterThan | GreaterThanEqual | LessThan | LessThanEqual | Minus | Percent | Plus | Slash | Tilde)
        } ;

        parser UsingDeconstructionSymbol = {
            introduced in "0.8.13" (IdentifierPath { introduced in "0.8.19" ((AsKeyword UsingDirectiveOperator) ?) } )
        } ;

        parser UsingDeconstructionSymbols = {
            introduced in "0.8.13"  (UsingDeconstructionSymbol separated by Comma)
         } ;

        parser VariableDeclaration = (VariableDeclarationType (DataLocation ?) Identifier) ;

        inline parser VariableDeclarationType = ({ removed in "0.5.0" VarKeyword } | TypeName) ;

        parser VariableDeclarationStatement = ((VariableDeclaration ((Equal Expression) ?)) terminated by Semicolon) ;

        parser WhileStatement = (WhileKeyword (Expression delimited by OpenParen and CloseParen) Statement) ;

        trivia parser EndOfFileTrivia = ((Whitespace | EndOfLine | MultilineComment | SingleLineComment) +) ;

        trivia parser LeadingTrivia = ((Whitespace | EndOfLine | MultilineComment | SingleLineComment) +) ;

        trivia parser TrailingTrivia = ((Whitespace ?) (SingleLineComment ?) EndOfLine) ;

    } ;

    lexical context VersionPragma = {

        parser VersionPragma = (SolidityKeyword VersionPragmaExpressions) ;

        precedence parser VersionPragmaExpression = (
            [
                left VersionPragmaOrOperator as VersionPragmaBinaryExpression,
                left VersionPragmaRangeOperator  as VersionPragmaBinaryExpression,
                prefix VersionPragmaUnaryOperator as VersionPragmaUnaryExpression
            ]
            with primary expression VersionPragmaSpecifier
        ) ;

        parser VersionPragmaExpressions = (VersionPragmaExpression +) ;

        inline parser VersionPragmaOrOperator = BarBar ;

        inline parser VersionPragmaRangeOperator = Minus ;

        parser VersionPragmaSpecifier = (VersionPragmaValue separated by Period) ;

        inline parser VersionPragmaUnaryOperator = (Caret | Tilde | Equal | LessThan | GreaterThan | LessThanEqual | GreaterThanEqual) ;

    } ;

    lexical context YulBlock = {

        inline parser YulKeywordsOverAllVersions = (
              BreakKeyword
            | CaseKeyword
            | ContinueKeyword
            | DefaultKeyword
            | FalseKeyword
            | ForKeyword
            | FunctionKeyword
            | HexKeyword
            | IfKeyword
            | LetKeyword
            | SwitchKeyword
            | TrueKeyword

            // Introduced in 0.6.0
            | LeaveKeyword
        ) ;

        parser YulAssignmentStatement = (YulIdentifierPaths ColonEqual YulExpression) ;

        parser YulBlock = ((YulStatements ?) delimited by OpenBrace and CloseBrace) ;

        parser YulBreakStatement = BreakKeyword ;

        parser YulContinueStatement = ContinueKeyword ;

        parser YulDeclarationStatement = (LetKeyword YulIdentifierPaths ((ColonEqual YulExpression) ?)) ;

        precedence parser YulExpression = (
            [
                postfix YulFunctionCallOperator as YulFunctionCallExpression
            ]
            with primary expression (YulLiteral | YulIdentifierPath)
        ) ;

        parser YulArguments = (YulExpression separated by Comma) ;

        parser YulForStatement = (ForKeyword YulBlock YulExpression YulBlock YulBlock) ;

        inline parser YulFunctionCallOperator = ((YulArguments ?) delimited by OpenParen and CloseParen) ;

        parser YulFunctionDefinition = (FunctionKeyword YulIdentifier YulParametersDeclaration (YulReturnsDeclaration ?) YulBlock) ;

        parser YulIdentifierPath = (YulIdentifier separated by Period) ;

        parser YulIdentifierPaths = (YulIdentifierPath separated by Comma) ;

        parser YulIdentifiers = (YulIdentifier separated by Comma) ;

        parser YulIfStatement = (IfKeyword YulExpression YulBlock) ;

        parser YulLeaveStatement = { introduced in "0.6.0" LeaveKeyword } ;

        inline parser YulLiteral = (TrueKeyword | FalseKeyword | YulHexLiteral | YulDecimalLiteral | HexStringLiteral | AsciiStringLiteral) ;

        parser YulParametersDeclaration = ((YulIdentifiers ?) delimited by OpenParen and CloseParen) ;

        parser YulReturnsDeclaration = (MinusGreaterThan YulIdentifiers) ;

        inline parser YulStatement = (
            YulBlock | YulFunctionDefinition | YulDeclarationStatement | YulAssignmentStatement | YulIfStatement |
            YulForStatement | YulSwitchStatement | { introduced in "0.6.0" YulLeaveStatement } |
            YulBreakStatement | YulContinueStatement | YulExpression
        ) ;

        parser YulStatements = (YulStatement +) ;

        parser YulSwitchCase = ((DefaultKeyword | (CaseKeyword YulLiteral)) YulBlock) ;

        parser YulSwitchCases = (YulSwitchCase +) ;

        parser YulSwitchStatement = (SwitchKeyword YulExpression YulSwitchCases) ;

    } ;

    /********************************************
     *         Scanners
     ********************************************/

    // Trivia

    scanner EndOfLine = (('\r' ?) '\n') ;
    scanner Whitespace = (('\t' | ' ') +) ;
    scanner SingleLineComment = ("//" ((! "\n\r") *)) ;
    scanner MultilineComment = ('/' '*' (((! '*') | ('*' not followed by '/')) *) '*' '/') ;

    // Delimiters

    scanner OpenBrace = '{' ;
    scanner CloseBrace = '}' ;

    scanner OpenBracket = '[' ;
    scanner CloseBracket = ']' ;

    scanner OpenParen = '(' ;
    scanner CloseParen = ')' ;

    // Punctuation

    scanner Colon = ':' ;
    scanner Comma = ',' ;
    scanner MinusGreaterThan = "->" ;
    scanner Period = '.' ;
    scanner QuestionMark = '?' ;
    scanner Semicolon = ';' ;

    // Operators

    scanner Ampersand = '&' ;
    scanner AmpersandAmpersand = "&&" ;
    scanner Asterisk = '*' ;
    scanner AsteriskAsterisk = "**" ;
    scanner Bang = '!' ;
    scanner Bar = '|' ;
    scanner BarBar = "||" ;
    scanner Caret = '^' ;
    scanner GreaterThan = '>' ;
    scanner GreaterThanEqual = ">=" ;
    scanner GreaterThanGreaterThan = ">>" ;
    scanner GreaterThanGreaterThanGreaterThan = ">>>" ;
    scanner EqualEqual = "==" ;
    scanner EqualGreaterThan = "=>" ;
    scanner LessThan = '<' ;
    scanner LessThanEqual = "<=" ;
    scanner LessThanLessThan = "<<" ;
    scanner Minus = '-' ;
    scanner MinusMinus = "--" ;
    scanner Percent = '%' ;
    scanner Plus = '+' ;
    scanner PlusPlus = "++" ;
    scanner Slash = '/' ;
    scanner Tilde = '~' ;

    // Assignment Operators

    scanner AmpersandEqual = "&=" ;
    scanner AsteriskEqual = "*=" ;
    scanner BangEqual = "!=" ;
    scanner BarEqual = "|=" ;
    scanner CaretEqual = "^=" ;
    scanner ColonEqual = ":=" ;
    scanner GreaterThanGreaterThanEqual = ">>=" ;
    scanner GreaterThanGreaterThanGreaterThanEqual = ">>>=" ;
    scanner Equal = '=' ;
    scanner LessThanLessThanEqual = "<<=" ;
    scanner MinusEqual = "-=" ;
    scanner PercentEqual = "%=" ;
    scanner PlusEqual = "+=" ;
    scanner SlashEqual = "/=" ;

    // Identifiers

    scanner Identifier = RawIdentifier ;
    scanner YulIdentifier = ( RawIdentifier ) ;

    scanner IdentifierPart = (IdentifierStart | ('0' .. '9')) ;
    scanner IdentifierStart = ('$' | ('A' .. 'Z') | '_' | ('a' .. 'z')) ;
    scanner RawIdentifier = (IdentifierStart (IdentifierPart *)) ;

    // Constructed Identifiers (Typenames)

    scanner BytesKeyword =        ("bytes"  FixedBytesTypeSize) ;
    scanner FixedKeyword =        ("fixed"  (FixedTypeSize ?)) ;
    scanner IntKeyword =          ("int"    (IntegerTypeSize ?)) ;
    scanner UfixedKeyword =   ("ufixed" (FixedTypeSize ?)) ;
    scanner UintKeyword = ("uint"   (IntegerTypeSize ?)) ;

    scanner FixedBytesTypeSize = (
        "1"  | "2"  | "3"  | "4"  | "5"  | "6"  | "7"  | "8"  |
        "9"  | "10" | "11" | "12" | "13" | "14" | "15" | "16" |
        "17" | "18" | "19" | "20" | "21" | "22" | "23" | "24" |
        "25" | "26" | "27" | "28" | "29" | "30" | "31" | "32"
    ) ;
    scanner FixedTypeSize = ((('0' .. '9') +) 'x' (('0' .. '9') +)) ;
    scanner IntegerTypeSize = (
        "8"   | "16"  | "24"  | "32"  | "40"  | "48"  | "56"  | "64"  |
        "72"  | "80"  | "88"  | "96"  | "104" | "112" | "120" | "128" |
        "136" | "144" | "152" | "160" | "168" | "176" | "184" | "192" |
        "200" | "208" | "216" | "224" | "232" | "240" | "248" | "256"
    ) ;

    // Literal Building Blocks

    scanner DecimalDigit = ('0' .. '9') ;
    scanner HexCharacter = (DecimalDigit | ('A' .. 'F') | ('a' .. 'f')) ;
    scanner AsciiCharacterWithoutDoubleQuoteOrBackslash = ((' ' .. '!') | ('#' .. '[') | (']' .. '~')) ;
    scanner AsciiCharacterWithoutSingleQuoteOrBackslash = ((' ' .. '&') | ('(' .. '[') | (']' .. '~')) ;

    scanner EscapeSequence = ('\\' (AsciiEscape | HexByteEscape | UnicodeEscape)) ;
    scanner AsciiEscape = ('\n' | '\r' | '"' | '\'' | '\\' | 'n' | 'r' | 't') ;
    scanner HexByteEscape = ('x' HexCharacter HexCharacter) ;
    scanner UnicodeEscape = ('u' HexCharacter HexCharacter HexCharacter HexCharacter) ;

    // Ascii String Literals

    scanner AsciiStringLiteral = (SingleQuotedAsciiStringLiteral | DoubleQuotedAsciiStringLiteral) ;
    scanner DoubleQuotedAsciiStringLiteral = ("\"" ((EscapeSequence | AsciiCharacterWithoutDoubleQuoteOrBackslash) *) "\"") ;
    scanner SingleQuotedAsciiStringLiteral = ("\'" ((EscapeSequence | AsciiCharacterWithoutSingleQuoteOrBackslash) *) "\'") ;

    // Hex String Literals

    scanner HexStringLiteral = (SingleQuotedHexStringLiteral | DoubleQuotedHexStringLiteral) ;
    scanner DoubleQuotedHexStringLiteral = ("hex\"" (HexStringContents ?) "\"") ;
    scanner SingleQuotedHexStringLiteral = ("hex\'" (HexStringContents ?) "\'") ;
    scanner HexStringContents = (HexCharacter HexCharacter ((('_' ?) HexCharacter HexCharacter) *)) ;

    // Unicode String Literals

    scanner UnicodeStringLiteral = {
        introduced in "0.7.0" (SingleQuotedUnicodeStringLiteral | DoubleQuotedUnicodeStringLiteral)
    } ;
    scanner DoubleQuotedUnicodeStringLiteral = { introduced in "0.7.0" ("unicode\"" ((EscapeSequence | (! "\n\r\"\\")) *) "\"") } ;
    scanner SingleQuotedUnicodeStringLiteral = { introduced in "0.7.0" ("unicode\'" ((EscapeSequence | (! "\n\r\'\\")) *) "\'") } ;

    // Numeric Literals

    scanner DecimalLiteral = (
        (
            (
                (DecimalDigits not followed by '.') |
                { removed in "0.5.0" ((DecimalDigits '.') not followed by DecimalDigits) } |
                ('.' DecimalDigits) |
                (DecimalDigits '.' DecimalDigits)
            )
            (DecimalExponent ?)
        ) not followed by IdentifierStart
    ) ;
    scanner DecimalExponent = (('E' | 'e') ('-' ?) DecimalDigits) ;
    scanner DecimalDigits = ((DecimalDigit +) (('_' (DecimalDigit +)) *)) ;
    scanner HexLiteral = (
        (("0x" | { removed in "0.5.0" "0X" }) (HexCharacter +) (('_' (HexCharacter +)) *))
        not followed by IdentifierStart
    ) ;

    scanner YulDecimalLiteral = (
        ("0" | (('1' .. '9') (DecimalDigit *)))
        not followed by IdentifierStart
    ) ;
    scanner YulHexLiteral = (
        ("0x" (HexCharacter +))
        not followed by IdentifierStart
    ) ;

    // Pragma Literals

    scanner VersionPragmaValue = (('*' | ('0' .. '9') | 'X' | 'x') +) ;

    // Keywords

    scanner AbicoderKeyword = "abicoder" ;
    scanner AbstractKeyword = "abstract" ;
    scanner AddressKeyword = "address" ;
    scanner AfterKeyword = "after" ;
    scanner AnonymousKeyword = "anonymous" ;
    scanner AsKeyword = "as" ;
    scanner AssemblyKeyword = "assembly" ;
    scanner BoolKeyword = "bool" ;
    scanner BreakKeyword = "break" ;
    scanner ByteKeyword = "byte" ;
    scanner CaseKeyword = "case" ;
    scanner CatchKeyword = "catch" ;
    scanner ConstantKeyword = "constant" ;
    scanner ContinueKeyword = "continue" ;
    scanner ContractKeyword = "contract" ;
    scanner DaysKeyword = "days" ;
    scanner DefaultKeyword = "default" ;
    scanner DeleteKeyword = "delete" ;
    scanner DoKeyword = "do" ;
    scanner ElseKeyword = "else" ;
    scanner EnumKeyword = "enum" ;
    scanner EtherKeyword = "ether" ;
    scanner EventKeyword = "event" ;
    scanner ExperimentalKeyword = "experimental" ;
    scanner ExternalKeyword = "external" ;
    scanner FalseKeyword = "false" ;
    scanner FinalKeyword = "final" ;
    scanner ForKeyword = "for" ;
    scanner FromKeyword = "from" ;
    scanner FunctionKeyword = "function" ;
    scanner HexKeyword = "hex" ;
    scanner HoursKeyword = "hours" ;
    scanner IfKeyword = "if" ;
    scanner ImportKeyword = "import" ;
    scanner IndexedKeyword = "indexed" ;
    scanner InKeyword = "in" ;
    scanner InlineKeyword = "inline" ;
    scanner InterfaceKeyword = "interface" ;
    scanner InternalKeyword = "internal" ;
    scanner IsKeyword = "is" ;
    scanner LetKeyword = "let" ;
    scanner LibraryKeyword = "library" ;
    scanner MappingKeyword = "mapping" ;
    scanner MatchKeyword =  "match" ;
    scanner MemoryKeyword = "memory" ;
    scanner MinutesKeyword = "minutes" ;
    scanner ModifierKeyword = "modifier" ;
    scanner NewKeyword = "new" ;
    scanner NullKeyword = "null" ;
    scanner OfKeyword = "of" ;
    scanner PayableKeyword = "payable" ;
    scanner PragmaKeyword = "pragma" ;
    scanner PrivateKeyword = "private" ;
    scanner PublicKeyword = "public" ;
    scanner PureKeyword = "pure" ;
    scanner RelocatableKeyword = "relocatable" ;
    scanner ReturnKeyword = "return" ;
    scanner ReturnsKeyword = "returns" ;
    scanner SecondsKeyword = "seconds" ;
    scanner SolidityKeyword = "solidity" ;
    scanner StaticKeyword = "static" ;
    scanner StorageKeyword = "storage" ;
    scanner StringKeyword = "string" ;
    scanner StructKeyword = "struct" ;
    scanner SwitchKeyword = "switch" ;
    scanner ThrowKeyword = "throw" ;
    scanner TrueKeyword = "true" ;
    scanner TypeKeyword = "type";
    scanner TypeofKeyword = "typeof" ;
    scanner UsingKeyword = "using" ;
    scanner VarKeyword = "var" ;
    scanner ViewKeyword = "view" ;
    scanner WeeksKeyword = "weeks" ;
    scanner WeiKeyword = "wei" ;
    scanner WhileKeyword = "while" ;
    scanner YearsKeyword = "years" ;

    // Always reserved but used since 0.6.0
    scanner TryKeyword = "try" ;

    // introduced in 0.4.21
    // WRONG, it is both a keyword AND identifier for some versions.
    scanner EmitKeyword = { introduced in "0.4.21" "emit" } ;

    // Introduced in 0.4.22
    scanner ConstructorKeyword = { introduced in "0.4.22" "constructor" } ;

    // introduced in 0.5.0
    scanner AliasKeyword =      { introduced in "0.5.0" "alias" } ;
    scanner ApplyKeyword =      { introduced in "0.5.0" "apply" } ;
    scanner AutoKeyword =       { introduced in "0.5.0" "auto" } ;
    scanner CalldataKeyword =   { introduced in "0.5.0" "calldata" } ;
    scanner CopyofKeyword =     { introduced in "0.5.0" "copyof" } ;
    scanner DefineKeyword =     { introduced in "0.5.0" "define" } ;
    scanner ImplementsKeyword = { introduced in "0.5.0" "implements" } ;
    scanner MacroKeyword =      { introduced in "0.5.0" "macro" } ;
    scanner MutableKeyword =    { introduced in "0.5.0" "mutable" } ;
    scanner PartialKeyword =    { introduced in "0.5.0" "partial" } ;
    scanner PromiseKeyword =    { introduced in "0.5.0" "promise" } ;
    scanner ReferenceKeyword =  { introduced in "0.5.0" "reference" } ;
    scanner SealedKeyword =     { introduced in "0.5.0" "sealed" } ;
    scanner SizeofKeyword =     { introduced in "0.5.0" "sizeof" } ;
    scanner SupportsKeyword =   { introduced in "0.5.0" "supports" } ;
    scanner TypedefKeyword =    { introduced in "0.5.0" "typedef" } ;

    // Reserved since 0.5.0 and used since 0.8.0
    scanner UncheckedKeyword = { introduced in "0.5.0" "unchecked" } ;
    // Reserved since 0.5.0 and used since 0.6.5
    scanner ImmutableKeyword = { introduced in "0.5.0" "immutable" } ;
    // Reserved since 0.5.0 and used since 0.6.0
    scanner OverrideKeyword =  { introduced in "0.5.0" "override" } ;

    // Introduced in 0.6.0
    scanner FallbackKeyword = { introduced in "0.6.0" "fallback" } ;
    scanner ReceiveKeyword =  { introduced in "0.6.0" "receive" } ;
    scanner LeaveKeyword =    { introduced in "0.6.0" "leave" } ; // warning: used in yul
    scanner VirtualKeyword =  { introduced in "0.6.0" "virtual" } ;

    // Introduced in 0.6.11
    scanner GweiKeyword = { introduced in "0.6.11" "gwei" } ;

    // Removed in 0.7.0
    scanner FinneyKeyword = { removed in "0.7.0" "finney" } ;
    scanner SzaboKeyword = { removed in "0.7.0" "szabo" } ;

    // Introduced in 0.8.4
    scanner ErrorKeyword =  { introduced in "0.8.4" "error" } ;
    scanner RevertKeyword = { introduced in "0.8.4" "revert" } ;

    // Introduced in 0.8.13
    scanner GlobalKeyword = { introduced in "0.8.13" "global" } ;
}
