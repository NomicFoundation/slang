use codegen_grammar::*;

slang_grammar! {

    config = {

        name = "Solidity" ;

        versions = [
            "0.4.11" , "0.4.12" , "0.4.13" , "0.4.14" , "0.4.15" , "0.4.16" , "0.4.17" , "0.4.18" , "0.4.19" , "0.4.20" , "0.4.21" , "0.4.22" ,
            "0.4.23" , "0.4.24" , "0.4.25" , "0.4.26" ,

            "0.5.0" , "0.5.1" , "0.5.2" , "0.5.3" , "0.5.4" , "0.5.5" , "0.5.6" , "0.5.7" , "0.5.8" , "0.5.9" , "0.5.10" , "0.5.11" , "0.5.12" ,
            "0.5.13" , "0.5.14" , "0.5.15" , "0.5.16" , "0.5.17" ,

            "0.6.0" , "0.6.1" , "0.6.2" , "0.6.3" , "0.6.4" , "0.6.5" , "0.6.6" , "0.6.7" , "0.6.8" , "0.6.9" , "0.6.10" , "0.6.11" , "0.6.12" ,

            "0.7.0" , "0.7.1" , "0.7.2" , "0.7.3" , "0.7.4" , "0.7.5" , "0.7.6" ,

            "0.8.0" , "0.8.1" , "0.8.2" , "0.8.3" , "0.8.4" , "0.8.5" , "0.8.6" , "0.8.7" , "0.8.8" , "0.8.9" , "0.8.10" , "0.8.11" , "0.8.12" ,
            "0.8.13" , "0.8.14" , "0.8.15" , "0.8.16" , "0.8.17" , "0.8.18" , "0.8.19"
        ] ;

        leading trivia parser = LeadingTrivia ;
        trailing trivia parser = TrailingTrivia ;

    } ;

    /********************************************
     *         Parsers
     ********************************************/

    lexical context Default = {

        parser ABICoderPragma = (ABICoderKeyword Identifier) ;

        inline parser AddSubOperator = (Plus | Minus) ;

        parser AddressType = ((AddressKeyword (PayableKeyword ?)) | PayableKeyword) ;

        inline parser AndOperator = AmpersandAmpersand ;

        parser ArgumentsDeclaration = (((PositionalArgumentsList | NamedArgumentsDeclaration) ?) delimited_by OpenParen and CloseParen) ;

        parser ArrayExpression = (ArrayValuesList delimited_by OpenBracket and CloseBracket) ;

        inline parser ArrayTypeNameOperator = ((Expression ?) delimited_by OpenBracket and CloseBracket) ;

        parser ArrayValuesList = (Expression separated_by Comma) ;

        parser AsciiStringLiteralsList = (AsciiStringLiteral +) ;

        parser AssemblyFlagsList = (AsciiStringLiteral separated_by Comma) ;

        parser AssemblyStatement = (AssemblyKeyword (AsciiStringLiteral ?) ((AssemblyFlagsList delimited_by OpenParen and CloseParen) ?) YulBlock) ;

        inline parser AssignmentOperator = (
            Equal | BarEqual | PlusEqual | MinusEqual | CaretEqual | SlashEqual | PercentEqual | AsteriskEqual | AmpersandEqual |
            LessThanLessThanEqual | GreaterThanGreaterThanEqual | GreaterThanGreaterThanGreaterThanEqual
        ) ;

        inline parser BitwiseAndOperator = Ampersand ;

        inline parser BitwiseOrOperator = Bar ;

        inline parser BitwiseXOrOperator = Caret ;

        parser Block = ((StatementsList ?) delimited_by OpenBrace and CloseBrace) ;

        inline parser BooleanExpression = (TrueKeyword | FalseKeyword) ;

        parser BreakStatement = (BreakKeyword terminated_by Semicolon) ;

        parser CatchClause = { enabled from "0.6.0" (CatchKeyword (CatchClauseError ?) Block) } ;

        parser CatchClauseError = { enabled from "0.6.0" ((Identifier ?) ParametersDeclaration) } ;

        parser CatchClausesList = { enabled from "0.6.0" (CatchClause +) } ;

        inline parser ConditionalOperator = (QuestionMark Expression Colon Expression) ;

        parser ConstantDefinition = { enabled from "0.7.4" ((TypeName ConstantKeyword Identifier Equal Expression) terminated_by Semicolon) } ;

        inline parser ConstructorAttribute = { enabled from "0.4.22" (ModifierInvocation | InternalKeyword | PayableKeyword | PublicKeyword) } ;

        parser ConstructorAttributesList = { enabled from "0.4.22" (ConstructorAttribute +) } ;

        parser ConstructorDefinition = { enabled from "0.4.22" (ConstructorKeyword ParametersDeclaration (ConstructorAttributesList ?) Block) } ;

        parser ContinueStatement = (ContinueKeyword terminated_by Semicolon) ;

        parser ContractDefinition = (
            { enabled from "0.6.0"  (AbstractKeyword ?) } ContractKeyword Identifier (InheritanceSpecifier ?) ((ContractMembersList ?) delimited_by OpenBrace and CloseBrace)
        ) ;

        inline parser ContractMember = (
            UsingDirective | FunctionDefinition | ModifierDefinition | StructDefinition | EnumDefinition | EventDefinition |  StateVariableDefinition |
            { enabled from "0.4.22" ConstructorDefinition } |
            { enabled from "0.6.0" (FallbackFunctionDefinition | ReceiveFunctionDefinition) } |
            { disabled from "0.6.0" UnnamedFunctionDefinition } |
            { enabled from "0.8.4" ErrorDefinition } |
            { enabled from "0.8.8" UserDefinedValueTypeDefinition }
        ) ;

        parser ContractMembersList = (ContractMember +) ;

        inline parser ControlStatement = (
            IfStatement | ForStatement | WhileStatement | DoWhileStatement | ContinueStatement | BreakStatement | DeleteStatement | ReturnStatement | RevertStatement |
            { enabled from "0.4.21" EmitStatement} |
            { disabled from "0.5.0" ThrowStatement } |
            { enabled from "0.6.0" TryStatement}
        ) ;

        inline parser DataLocation = (
            MemoryKeyword | StorageKeyword |
            { enabled from "0.5.0" CalldataKeyword}
        ) ;

        parser DeconstructionImport = ((DeconstructionImportSymbolsList delimited_by OpenBrace and CloseBrace) FromKeyword AsciiStringLiteral) ;

        parser DeconstructionImportSymbol = (Identifier ((AsKeyword Identifier) ?)) ;

        parser DeconstructionImportSymbolsList = (DeconstructionImportSymbol separated_by Comma) ;

        parser DeleteStatement = ((DeleteKeyword Expression) terminated_by Semicolon) ;

        parser DoWhileStatement = ((DoKeyword Statement WhileKeyword (Expression delimited_by OpenParen and CloseParen)) terminated_by Semicolon) ;

        inline parser ElementaryType = (
            BoolKeyword | StringKeyword | AddressType | FixedBytesType | SignedIntegerType | UnsignedIntegerType | SignedFixedType | UnsignedFixedType |
            { disabled from "0.8.0" ByteKeyword}
        ) ;

        parser EmitStatement = { enabled from "0.4.21" ((EmitKeyword IdentifierPath ArgumentsDeclaration) terminated_by Semicolon) } ;

        parser EnumDefinition = (EnumKeyword Identifier ((IdentifiersList ?) delimited_by OpenBrace and CloseBrace)) ;

        inline parser EqualityComparisonOperator = (EqualEqual | BangEqual) ;

        parser ErrorDefinition = { enabled from "0.8.4" ((ErrorKeyword Identifier ((ErrorParametersList ?) delimited_by OpenParen and CloseParen)) terminated_by Semicolon) } ;

        parser ErrorParameter = { enabled from "0.8.4" (TypeName (Identifier ?)) } ;

        parser ErrorParametersList = { enabled from "0.8.4" (ErrorParameter separated_by Comma) } ;

        parser EventDefinition = ((EventKeyword Identifier ((EventParametersList ?) delimited_by OpenParen and CloseParen) (AnonymousKeyword ?)) terminated_by Semicolon) ;

        parser EventParameter = (TypeName (IndexedKeyword ?) (Identifier ?)) ;

        parser EventParametersList = (EventParameter separated_by Comma) ;

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
                { disabled from "0.6.0" left ExponentiationOperator as BinaryExpression },
                { enabled from "0.6.0" right ExponentiationOperator as BinaryExpression },
                postfix UnaryPostfixOperator as UnaryPostfixExpression,
                prefix UnaryPrefixOperator as UnaryPrefixExpression,
                postfix FunctionCallOperator as FunctionCallExpression,
                postfix MemberAccessOperator as MemberAccessExpression,
                postfix IndexAccessOperator as IndexAccessExpression
            ]
            with primary expression PrimaryExpression
        ) ;

        parser ExpressionStatement = (Expression terminated_by Semicolon) ;

        inline parser FallbackFunctionAttribute = { enabled from "0.6.0" (ModifierInvocation | OverrideSpecifier | ExternalKeyword | PayableKeyword | PureKeyword | ViewKeyword | VirtualKeyword) } ;

        parser FallbackFunctionAttributesList = { enabled from "0.6.0" (FallbackFunctionAttribute +) } ;

        parser FallbackFunctionDefinition = { enabled from "0.6.0" (FallbackKeyword ParametersDeclaration (FallbackFunctionAttributesList ?) (ReturnsDeclaration ?) (Semicolon | Block)) } ;

        parser ForStatement = (ForKeyword (((SimpleStatement | Semicolon) (ExpressionStatement | Semicolon) (Expression ?)) delimited_by OpenParen and CloseParen) Statement) ;

        inline parser FunctionAttribute = (
            ModifierInvocation | OverrideSpecifier | ExternalKeyword | InternalKeyword | PayableKeyword | PrivateKeyword | PublicKeyword | PureKeyword | ViewKeyword |
            { disabled from "0.5.0" ConstantKeyword } |
            { enabled from "0.6.0" VirtualKeyword }
         ) ;

        parser FunctionAttributesList = (FunctionAttribute +) ;

        inline parser FunctionCallOperator = (
            { enabled from "0.6.2" (FunctionCallOptions ?) }
            ArgumentsDeclaration
        ) ;

        parser FunctionCallOptions = (
            { enabled from "0.6.2" and disabled from "0.8.0" (NamedArgumentsDeclaration +) } |
            { enabled from "0.8.0"                           NamedArgumentsDeclaration }
        ) ;

        parser FunctionDefinition = (FunctionKeyword (Identifier | FallbackKeyword | ReceiveKeyword) ParametersDeclaration (FunctionAttributesList ?) (ReturnsDeclaration ?) (Semicolon | Block)) ;

        parser FunctionType = (FunctionKeyword ParametersDeclaration (FunctionTypeAttributesList ?) (ReturnsDeclaration ?)) ;

        inline parser FunctionTypeAttribute = (InternalKeyword | ExternalKeyword | PrivateKeyword | PublicKeyword | PureKeyword | ViewKeyword | PayableKeyword) ;

        parser FunctionTypeAttributesList = (FunctionTypeAttribute +) ;

        parser HexStringLiteralsList = (HexStringLiteral +) ;

        parser IdentifierPath = (Identifier separated_by Period) ;

        parser IdentifierPathsList = (IdentifierPath separated_by Comma) ;

        parser IdentifiersList = (Identifier separated_by Comma) ;

        parser IfStatement = (IfKeyword (Expression delimited_by OpenParen and CloseParen) Statement ((ElseKeyword Statement) ?)) ;

        parser ImportDirective = ((ImportKeyword (PathImport | NamedImport | DeconstructionImport)) terminated_by Semicolon) ;

        inline parser IndexAccessOperator = (((Expression ?) ((Colon (Expression ?)) ?)) delimited_by OpenBracket and CloseBracket) ;

        parser InheritanceSpecifier = (IsKeyword InheritanceTypesList) ;

        parser InheritanceType = (IdentifierPath (ArgumentsDeclaration ?)) ;

        parser InheritanceTypesList = (InheritanceType separated_by Comma) ;

        parser InterfaceDefinition = (InterfaceKeyword Identifier (InheritanceSpecifier ?) ((InterfaceMembersList ?) delimited_by OpenBrace and CloseBrace)) ;

        parser InterfaceMembersList = (ContractMember +) ;

        parser LibraryDefinition = (LibraryKeyword Identifier ((LibraryMembersList ?) delimited_by OpenBrace and CloseBrace)) ;

        parser LibraryMembersList = (ContractMember +) ;

        parser MappingKeyType = (
            (ElementaryType | IdentifierPath)
            { enabled from "0.8.18" (Identifier ?) }
        ) ;

        parser MappingType = (MappingKeyword ((MappingKeyType EqualGreaterThan MappingValueType) delimited_by OpenParen and CloseParen)) ;

        parser MappingValueType = (
            TypeName
            { enabled from "0.8.18" (Identifier ?) }
        ) ;

        inline parser MemberAccessOperator = (Period (Identifier | AddressKeyword)) ;

        inline parser ModifierAttribute = (
            OverrideSpecifier |
            { enabled from "0.6.0" VirtualKeyword }
        ) ;

        parser ModifierAttributesList = (ModifierAttribute +) ;

        parser ModifierDefinition = (ModifierKeyword Identifier (ParametersDeclaration ?) (ModifierAttributesList ?) (Semicolon | Block)) ;

        parser ModifierInvocation = (IdentifierPath (ArgumentsDeclaration ?)) ;

        inline parser MulDivModOperator = (Asterisk | Slash | Percent) ;

        parser NamedArgument = (Identifier Colon Expression) ;

        parser NamedArgumentsDeclaration = ((NamedArgumentsList ?) delimited_by OpenBrace and CloseBrace) ;

        parser NamedArgumentsList = (NamedArgument separated_by Comma) ;

        parser NamedImport = (Asterisk AsKeyword Identifier FromKeyword AsciiStringLiteral) ;

        parser NewExpression = (NewKeyword TypeName) ;

        inline parser NumberUnit = (
            DaysKeyword | EtherKeyword | HoursKeyword | MinutesKeyword | SecondsKeyword | WeeksKeyword | WeiKeyword |
            { disabled from "0.5.0" YearsKeyword } |
            { enabled from "0.6.11" GweiKeyword } |
            { disabled from "0.7.0" ( FinneyKeyword | SzaboKeyword) }
        ) ;

        parser NumericExpression = (
            (HexLiteral { disabled from "0.5.0" (NumberUnit ?) }) |
            (DecimalLiteral (NumberUnit ?))
        ) ;

        inline parser OrOperator = BarBar ;

        inline parser OrderComparisonOperator = (LessThan | GreaterThan | LessThanEqual | GreaterThanEqual) ;

        parser OverrideSpecifier = (OverrideKeyword (((IdentifierPathsList ?) delimited_by OpenParen and CloseParen) ?)) ;

        parser Parameter = (TypeName (DataLocation ?) (Identifier ?)) ;

        parser ParametersDeclaration = ((ParametersList ?) delimited_by OpenParen and CloseParen) ;

        parser ParametersList = (Parameter separated_by Comma) ;

        parser PathImport = (AsciiStringLiteral ((AsKeyword Identifier) ?)) ;

        parser PositionalArgumentsList = (Expression separated_by Comma) ;

        parser PragmaDirective = ((PragmaKeyword (ABICoderPragma | ExperimentalPragma | VersionPragma)) terminated_by Semicolon) ;

        inline parser PrimaryExpression = (
            NewExpression | TupleExpression | ArrayExpression | BooleanExpression | NumericExpression | StringExpression | ElementaryType | Identifier |
            { enabled from "0.5.3" TypeExpression }
        ) ;

        inline parser ReceiveFunctionAttribute = { enabled from "0.6.0" (ModifierInvocation | OverrideSpecifier | ExternalKeyword | PayableKeyword | VirtualKeyword) } ;

        parser ReceiveFunctionAttributesList = { enabled from "0.6.0" (ReceiveFunctionAttribute +) } ;

        parser ReceiveFunctionDefinition = { enabled from "0.6.0" (ReceiveKeyword ParametersDeclaration (ReceiveFunctionAttributesList ?) (Semicolon | Block)) } ;

        parser ReturnStatement = ((ReturnKeyword (Expression ?)) terminated_by Semicolon) ;

        parser ReturnsDeclaration = (ReturnsKeyword ParametersDeclaration) ;

        parser RevertStatement = ((RevertKeyword (IdentifierPath ?) ArgumentsDeclaration) terminated_by Semicolon) ;

        inline parser ShiftOperator = (LessThanLessThan | GreaterThanGreaterThan | GreaterThanGreaterThanGreaterThan) ;

        inline parser SimpleStatement = (ExpressionStatement | VariableDeclarationStatement | TupleDeconstructionStatement) ;

        parser SourceUnit = ((SourceUnitMembersList ?) (EndOfFileTrivia ?)) ;

        inline parser SourceUnitMember = (
              PragmaDirective | ImportDirective | ContractDefinition | InterfaceDefinition | LibraryDefinition |
              { enabled from "0.6.0" (StructDefinition | EnumDefinition) } |
              { enabled from "0.7.1" FunctionDefinition } |
              { enabled from "0.7.4" ConstantDefinition } |
              { enabled from "0.8.4" ErrorDefinition } |
              { enabled from "0.8.8" UserDefinedValueTypeDefinition } |
              { enabled from "0.8.13" UsingDirective }
        ) ;

        parser SourceUnitMembersList = (SourceUnitMember +) ;

        inline parser StateVariableAttribute = (
            OverrideSpecifier | ConstantKeyword | InternalKeyword | PrivateKeyword | PublicKeyword| { enabled from "0.6.5" ImmutableKeyword }
        ) ;

        parser StateVariableAttributesList = (StateVariableAttribute +) ;

        parser StateVariableDefinition = ((TypeName (StateVariableAttributesList ?) Identifier ((Equal Expression) ?)) terminated_by Semicolon) ;

        parser Statement = (
            SimpleStatement | ControlStatement | AssemblyStatement | Block | { enabled from "0.8.0" UncheckedBlock }
        ) ;

        parser StatementsList = (Statement +) ;

        inline parser StringExpression = (
            HexStringLiteralsList | AsciiStringLiteralsList | { enabled from "0.7.0" UnicodeStringLiteralsList }
        ) ;

        parser StructDefinition = (StructKeyword Identifier ((StructMembersList ?) delimited_by OpenBrace and CloseBrace)) ;

        parser StructMember = ((TypeName Identifier) terminated_by Semicolon) ;

        parser StructMembersList = (StructMember +) ;

        parser ThrowStatement = { disabled from "0.5.0" (ThrowKeyword terminated_by Semicolon) } ;

        parser TryStatement = { enabled from "0.6.0" (TryKeyword Expression (ReturnsDeclaration ?) Block CatchClausesList) } ;

        parser TupleDeconstructionStatement = ((((TupleMembersList ?) delimited_by OpenParen and CloseParen) Equal Expression) terminated_by Semicolon) ;

        parser TupleExpression = (TupleValuesList delimited_by OpenParen and CloseParen) ;

        parser TupleMember = (((TypeName (DataLocation ?) Identifier) | ((DataLocation ?) Identifier)) ?) ;

        parser TupleMembersList = (TupleMember separated_by Comma) ;

        parser TupleValuesList = ((Expression ?) separated_by Comma) ;

        parser TypeExpression = { enabled from "0.5.3" (TypeKeyword (TypeName delimited_by OpenParen and CloseParen)) } ;

        precedence parser TypeName = (
            [
                postfix ArrayTypeNameOperator as ArrayTypeName
            ]
            with primary expression (FunctionType | MappingType | ElementaryType | IdentifierPath)
        ) ;

        inline parser UnaryPostfixOperator = (PlusPlus | MinusMinus) ;

        inline parser UnaryPrefixOperator = (
            PlusPlus | MinusMinus | Tilde | Bang | Minus | { disabled from "0.5.0" Plus }
        ) ;

        parser UncheckedBlock = { enabled from "0.8.0" (UncheckedKeyword Block) } ;

        parser UnicodeStringLiteralsList = { enabled from "0.7.0" (UnicodeStringLiteral +) } ;

        inline parser UnnamedFunctionAttribute = { disabled from "0.6.0" (ModifierInvocation | OverrideSpecifier | ExternalKeyword | PayableKeyword | PureKeyword | ViewKeyword) } ;

        parser UnnamedFunctionAttributesList = { disabled from "0.6.0" (UnnamedFunctionAttribute +) } ;

        parser UnnamedFunctionDefinition = { disabled from "0.6.0" (FunctionKeyword ParametersDeclaration (UnnamedFunctionAttributesList ?) (Semicolon | Block)) } ;

        parser UserDefinedValueTypeDefinition = { enabled from "0.8.8" ((TypeKeyword Identifier IsKeyword ElementaryType) terminated_by Semicolon) } ;

        parser UsingDirective = ((UsingKeyword (UsingDirectivePath | UsingDirectiveDeconstruction) ForKeyword (Asterisk | TypeName) (GlobalKeyword ?)) terminated_by Semicolon) ;

        parser UsingDirectiveDeconstruction = (UsingDirectiveSymbolsList delimited_by OpenBrace and CloseBrace) ;

        inline parser UsingDirectiveOperator = {
            enabled from "0.8.19" (Ampersand | Asterisk | BangEqual | Bar | Caret | EqualEqual | GreaterThan | GreaterThanEqual | LessThan | LessThanEqual | Minus | Percent | Plus | Slash | Tilde)
        } ;

        parser UsingDirectivePath = IdentifierPath ;

        parser UsingDirectiveSymbol = (
            IdentifierPath { enabled from "0.8.19" ((AsKeyword UsingDirectiveOperator) ?) }
        ) ;

        parser UsingDirectiveSymbolsList = (UsingDirectiveSymbol separated_by Comma) ;

        parser VariableDeclaration = (
            ({ disabled from "0.5.0" VarKeyword } | TypeName) (DataLocation ?) Identifier
        ) ;

        parser VariableDeclarationStatement = ((VariableDeclaration ((Equal Expression) ?)) terminated_by Semicolon) ;

        parser WhileStatement = (WhileKeyword (Expression delimited_by OpenParen and CloseParen) Statement) ;

        trivia parser EndOfFileTrivia = ((Whitespace | EndOfLine | MultilineComment | SingleLineComment) +) ;

        trivia parser LeadingTrivia = ((Whitespace | EndOfLine | MultilineComment | SingleLineComment) +) ;

        trivia parser TrailingTrivia = ((Whitespace ?) (SingleLineComment ?) EndOfLine) ;

    } ;

    lexical context VersionPragma = {

        parser VersionPragma = (SolidityKeyword VersionPragmaExpressionsList) ;

        precedence parser VersionPragmaExpression = (
            [
                left VersionPragmaOrOperator as VersionPragmaBinaryExpression,
                left VersionPragmaRangeOperator  as VersionPragmaBinaryExpression,
                prefix VersionPragmaUnaryOperator as VersionPragmaUnaryExpression
            ]
            with primary expression VersionPragmaSpecifier
        ) ;

        parser VersionPragmaExpressionsList = (VersionPragmaExpression +) ;

        inline parser VersionPragmaOrOperator = BarBar ;

        inline parser VersionPragmaRangeOperator = Minus ;

        parser VersionPragmaSpecifier = (VersionPragmaValue separated_by Period) ;

        inline parser VersionPragmaUnaryOperator = (Caret | Tilde | Equal | LessThan | GreaterThan | LessThanEqual | GreaterThanEqual) ;

    } ;

    lexical context YulBlock = {

        parser YulAssignmentStatement = (YulIdentifierPathsList ColonEqual YulExpression) ;

        parser YulBlock = ((YulStatementsList ?) delimited_by OpenBrace and CloseBrace) ;

        parser YulBreakStatement = BreakKeyword ;

        parser YulContinueStatement = ContinueKeyword ;

        parser YulDeclarationStatement = (LetKeyword YulIdentifierPathsList ((ColonEqual YulExpression) ?)) ;

        precedence parser YulExpression = (
            [
                postfix YulFunctionCallOperator as YulFunctionCallExpression
            ]
            with primary expression (YulLiteral | YulIdentifierPath)
        ) ;

        parser YulExpressionsList = (YulExpression separated_by Comma) ;

        parser YulForStatement = (ForKeyword YulBlock YulExpression YulBlock YulBlock) ;

        inline parser YulFunctionCallOperator = ((YulExpressionsList ?) delimited_by OpenParen and CloseParen) ;

        parser YulFunctionDefinition = (FunctionKeyword YulIdentifier YulParametersDeclaration (YulReturnsDeclaration ?) YulBlock) ;

        parser YulIdentifierPath = (YulIdentifier separated_by Period) ;

        parser YulIdentifierPathsList = (YulIdentifierPath separated_by Comma) ;

        parser YulIdentifiersList = (YulIdentifier separated_by Comma) ;

        parser YulIfStatement = (IfKeyword YulExpression YulBlock) ;

        parser YulLeaveStatement = { enabled from "0.6.0" LeaveKeyword } ;

        inline parser YulLiteral = (TrueKeyword | FalseKeyword | YulHexLiteral | YulDecimalLiteral | HexStringLiteral | AsciiStringLiteral) ;

        parser YulParametersDeclaration = ((YulIdentifiersList ?) delimited_by OpenParen and CloseParen) ;

        parser YulReturnsDeclaration = (MinusGreaterThan YulIdentifiersList) ;

        parser YulStatement = (
            YulBlock | YulFunctionDefinition | YulDeclarationStatement | YulAssignmentStatement | YulIfStatement |
            YulForStatement | YulSwitchStatement | YulBreakStatement | YulContinueStatement | YulExpression |
            { enabled from "0.6.0" YulLeaveStatement }
        ) ;

        parser YulStatementsList = (YulStatement +) ;

        parser YulSwitchCase = ((DefaultKeyword | (CaseKeyword YulLiteral)) YulBlock) ;

        parser YulSwitchCasesList = (YulSwitchCase +) ;

        parser YulSwitchStatement = (SwitchKeyword YulExpression YulSwitchCasesList) ;

    } ;

    /********************************************
     *         Scanners
     ********************************************/

    scanner ABICoderKeyword = "abicoder" ;

    scanner AbstractKeyword = { enabled from "0.6.0" "abstract" } ;

    scanner AddressKeyword = "address" ;

    scanner Ampersand = '&' ;

    scanner AmpersandAmpersand = "&&" ;

    scanner AmpersandEqual = "&=" ;

    scanner AnonymousKeyword = "anonymous" ;

    scanner AsKeyword = "as" ;

    scanner AsciiEscape = ('\n' | '\r' | '"' | '\'' | '\\' | 'n' | 'r' | 't') ;

    scanner AsciiStringLiteral = (SingleQuotedAsciiStringLiteral | DoubleQuotedAsciiStringLiteral) ;

    scanner AssemblyKeyword = "assembly" ;

    scanner Asterisk = '*' ;

    scanner AsteriskAsterisk = "**" ;

    scanner AsteriskEqual = "*=" ;

    scanner Bang = '!' ;

    scanner BangEqual = "!=" ;

    scanner Bar = '|' ;

    scanner BarBar = "||" ;

    scanner BarEqual = "|=" ;

    scanner BoolKeyword = "bool" ;

    scanner BreakKeyword = "break" ;

    scanner ByteKeyword = { disabled from "0.8.0" "byte" } ;

    scanner CalldataKeyword = { enabled from "0.5.0" "calldata" } ;

    scanner Caret = '^' ;

    scanner CaretEqual = "^=" ;

    scanner CaseKeyword = "case" ;

    scanner CatchKeyword = { enabled from "0.6.0" "catch" } ;

    scanner CloseBrace = '}' ;

    scanner CloseBracket = ']' ;

    scanner CloseParen = ')' ;

    scanner Colon = ':' ;

    scanner ColonEqual = ":=" ;

    scanner Comma = ',' ;

    scanner ConstantKeyword = "constant" ;

    scanner ConstructorKeyword = { enabled from "0.4.22" "constructor" } ;

    scanner ContinueKeyword = "continue" ;

    scanner ContractKeyword = "contract" ;

    scanner DaysKeyword = "days" ;

    scanner DecimalDigits = ((('0' .. '9') +) (('_' (('0' .. '9') +)) *)) ;

    scanner DecimalExponent = (('E' | 'e') ('-' ?) DecimalDigits) ;

    scanner DecimalLiteral = (
        (
            (
                DecimalDigits
                (
                    (
                        '.'
                        (
                            { disabled from "0.5.0" (DecimalDigits ?)} |
                            { enabled from "0.5.0" DecimalDigits }
                        )
                    )
                ?)
            ) |
            (
                '.'
                DecimalDigits
            )
        )
        (DecimalExponent ?)
    ) ;

    scanner DefaultKeyword = "default" ;

    scanner DeleteKeyword = "delete" ;

    scanner DoKeyword = "do" ;

    scanner DoubleQuotedAsciiStringLiteral = ('"' ((EscapeSequence | ((' ' .. '!') | ('#' .. '[') | (']' .. '~'))) *) '"') ;

    scanner DoubleQuotedHexStringLiteral = ("hex" '"' (HexStringContents ?) '"') ;

    scanner DoubleQuotedUnicodeStringLiteral = { enabled from "0.7.0" ("unicode\"" ((EscapeSequence | (! "\n\r\"\\")) *) '"') } ;

    scanner ElseKeyword = "else" ;

    scanner EmitKeyword = { enabled from "0.4.21" "emit" } ;

    scanner EndOfLine = (('\r' ?) '\n') ;

    scanner EnumKeyword = "enum" ;

    scanner Equal = '=' ;

    scanner EqualEqual = "==" ;

    scanner EqualGreaterThan = "=>" ;

    scanner ErrorKeyword = { enabled from "0.8.4" "error" } ;

    scanner EscapeSequence = ('\\' (AsciiEscape | HexByteEscape | UnicodeEscape)) ;

    scanner EtherKeyword = "ether" ;

    scanner EventKeyword = "event" ;

    scanner ExperimentalKeyword = "experimental" ;

    scanner ExternalKeyword = "external" ;

    scanner FallbackKeyword = "fallback" ;

    scanner FalseKeyword = "false" ;

    scanner FinneyKeyword = { disabled from "0.7.0" "finney" } ;

    scanner FixedBytesType = ("bytes" FixedBytesTypeSize) ;

    scanner FixedBytesTypeSize = (
        "1" | "10" | "11" | "12" | "13" | "14" | "15" | "16" | "17" | "18" | "19" | "2" | "20" | "21" | "22" | "23" |
        "24" | "25" | "26" | "27" | "28" | "29" | "3" | "30" | "31" | "32" | "4" | "5" | "6" | "7" | "8" | "9"
    ) ;

    scanner FixedTypeSize = ((('0' .. '9') +) 'x' (('0' .. '9') +)) ;

    scanner ForKeyword = "for" ;

    scanner FromKeyword = "from" ;

    scanner FunctionKeyword = "function" ;

    scanner GlobalKeyword = "global" ;

    scanner GreaterThan = '>' ;

    scanner GreaterThanEqual = ">=" ;

    scanner GreaterThanGreaterThan = ">>" ;

    scanner GreaterThanGreaterThanEqual = ">>=" ;

    scanner GreaterThanGreaterThanGreaterThan = ">>>" ;

    scanner GreaterThanGreaterThanGreaterThanEqual = ">>>=" ;

    scanner GweiKeyword = { enabled from "0.6.11" "gwei" } ;

    scanner HexByteEscape = ('x' (HexCharacter) (HexCharacter)) ;

    scanner HexCharacter = (('0' .. '9') | ('A' .. 'F') | ('a' .. 'f')) ;

    scanner HexLiteral = (("0x" | {disabled from "0.5.0" "0X"}) ((HexCharacter) +) (('_' ((HexCharacter) +)) *)) ;

    scanner HexStringContents = ((HexCharacter) (HexCharacter) ((('_' ?) (HexCharacter) (HexCharacter)) *)) ;

    scanner HexStringLiteral = (SingleQuotedHexStringLiteral | DoubleQuotedHexStringLiteral) ;

    scanner HoursKeyword = "hours" ;

    scanner Identifier = RawIdentifier ;

    scanner IdentifierPart = (IdentifierStart | ('0' .. '9')) ;

    scanner IdentifierStart = ('$' | ('A' .. 'Z') | '_' | ('a' .. 'z')) ;

    scanner IfKeyword = "if" ;

    scanner ImmutableKeyword = { enabled from "0.6.5" "immutable" } ;

    scanner ImportKeyword = "import" ;

    scanner IndexedKeyword = "indexed" ;

    scanner IntegerTypeSize = (
        "104" | "112" | "120" | "128" | "136" | "144" | "152" | "16" | "160" | "168" | "176" | "184" | "192" | "200" | "208" |
        "216" | "224" | "232" | "24" | "240" | "248" | "256" | "32" | "40" | "48" | "56" | "64" | "72" | "8" | "80" | "88" | "96"
    ) ;

    scanner InterfaceKeyword = "interface" ;

    scanner InternalKeyword = "internal" ;

    scanner IsKeyword = "is" ;

    // scanner KeywordInAnyVersion = (FixedBytesType | SignedFixedType | UnsignedFixedType | SignedIntegerType | UnsignedIntegerType | AddressKeyword | AnonymousKeyword | AsKeyword | AssemblyKeyword | BoolKeyword | BreakKeyword | CaseKeyword | ConstantKeyword | ContinueKeyword | ContractKeyword | DaysKeyword | DefaultKeyword | DeleteKeyword | DoKeyword | ElseKeyword | EnumKeyword | EtherKeyword | EventKeyword | ExternalKeyword | FalseKeyword | ForKeyword | FunctionKeyword | HoursKeyword | IfKeyword | ImportKeyword | IndexedKeyword | InterfaceKeyword | InternalKeyword | IsKeyword | LetKeyword | LibraryKeyword | MappingKeyword | MemoryKeyword | MinutesKeyword | ModifierKeyword | NewKeyword | PayableKeyword | PragmaKeyword | PrivateKeyword | PublicKeyword | PureKeyword | ReturnKeyword | ReturnsKeyword | SecondsKeyword | StorageKeyword | StringKeyword | StructKeyword | SwitchKeyword | TrueKeyword | UsingKeyword | ViewKeyword | WeeksKeyword | WeiKeyword | WhileKeyword) ;

    // scanner KeywordInSomeVersion = (
    //     { disabled from "0.5.0" (ByteKeyword | FinneyKeyword | SzaboKeyword | ThrowKeyword | VarKeyword | YearsKeyword) } |
    //     { enabled from "0.5.0" and disabled from "0.6.0" (ByteKeyword | FinneyKeyword | SzaboKeyword | CalldataKeyword | ConstructorKeyword | EmitKeyword | OverrideKeyword | { enabled from "0.5.3" TypeKeyword }) } |
    //     { enabled from "0.6.0" and disabled from "0.6.5" (ByteKeyword | FinneyKeyword | SzaboKeyword | CalldataKeyword | ConstructorKeyword | EmitKeyword | OverrideKeyword | TypeKeyword | AbstractKeyword | CatchKeyword | FallbackKeyword | ReceiveKeyword | TryKeyword | VirtualKeyword) } |
    //     { enabled from "0.6.5" and disabled from "0.7.0" (ByteKeyword | FinneyKeyword | SzaboKeyword | CalldataKeyword | ConstructorKeyword | EmitKeyword | OverrideKeyword | TypeKeyword | AbstractKeyword | CatchKeyword | FallbackKeyword | ReceiveKeyword | TryKeyword | VirtualKeyword | ImmutableKeyword) } |
    //     { enabled from "0.7.0" and disabled from "0.8.0" (ByteKeyword | CalldataKeyword | ConstructorKeyword | EmitKeyword | OverrideKeyword | TypeKeyword | AbstractKeyword | CatchKeyword | FallbackKeyword | ReceiveKeyword | TryKeyword | VirtualKeyword | ImmutableKeyword | GweiKeyword) } |
    //     { enabled from "0.8.0" (CalldataKeyword | ConstructorKeyword | EmitKeyword | OverrideKeyword | TypeKeyword | AbstractKeyword | CatchKeyword | FallbackKeyword | ReceiveKeyword | TryKeyword | VirtualKeyword | ImmutableKeyword | GweiKeyword | UncheckedKeyword) }) ;

    scanner LeaveKeyword = { enabled from "0.6.0" "leave" } ;

    scanner LessThan = '<' ;

    scanner LessThanEqual = "<=" ;

    scanner LessThanLessThan = "<<" ;

    scanner LessThanLessThanEqual = "<<=" ;

    scanner LetKeyword = "let" ;

    scanner LibraryKeyword = "library" ;

    scanner MappingKeyword = "mapping" ;

    scanner MemoryKeyword = "memory" ;

    scanner Minus = '-' ;

    scanner MinusEqual = "-=" ;

    scanner MinusGreaterThan = "->" ;

    scanner MinusMinus = "--" ;

    scanner MinutesKeyword = "minutes" ;

    scanner ModifierKeyword = "modifier" ;

    scanner MultilineComment = ('/' '*' (((! '*') | ('*' (! '/'))) *) '*' '/') ;

    scanner NewKeyword = "new" ;

    scanner OpenBrace = '{' ;

    scanner OpenBracket = '[' ;

    scanner OpenParen = '(' ;

    scanner OverrideKeyword = "override" ;

    scanner PayableKeyword = "payable" ;

    scanner Percent = '%' ;

    scanner PercentEqual = "%=" ;

    scanner Period = '.' ;

    scanner Plus = '+' ;

    scanner PlusEqual = "+=" ;

    scanner PlusPlus = "++" ;

    scanner PragmaKeyword = "pragma" ;

    scanner PrivateKeyword = "private" ;

    scanner PublicKeyword = "public" ;

    scanner PureKeyword = "pure" ;

    scanner QuestionMark = '?' ;

    scanner RawIdentifier = (IdentifierStart (IdentifierPart *)) ;

    scanner ReceiveKeyword = "receive" ;

    // scanner ReservedWordInAnyVersion = ("abstract" | "after" | "byte" | "catch" | "final" | "finney" | "hex" | "in" | "inline" | "match" | "null" | "of" | "relocatable" | "static" | "szabo" | "throw" | "try" | "type" | "typeof" | "var" | "years") ;

    // scanner ReservedWordInSomeVersion = { enabled from "0.5.0" ("alias" | "apply" | "auto" | "copyof" | "define" | "implements" | "macro" | "mutable" | "partial" | "promise" | "reference" | "sealed" | "sizeof" | "supports" | "typedef") } ;

    scanner ReturnKeyword = "return" ;

    scanner ReturnsKeyword = "returns" ;

    scanner RevertKeyword = "revert" ;

    scanner SecondsKeyword = "seconds" ;

    scanner Semicolon = ';' ;

    scanner SignedFixedType = ("fixed" (FixedTypeSize ?)) ;

    scanner SignedIntegerType = ("int" (IntegerTypeSize ?)) ;

    scanner SingleLineComment = ("//" ((! "\n\r") *)) ;

    scanner SingleQuotedAsciiStringLiteral = ('\'' ((EscapeSequence | ((' ' .. '&') | ('(' .. '[') | (']' .. '~'))) *) '\'') ;

    scanner SingleQuotedHexStringLiteral = ("hex" '\'' (HexStringContents ?) '\'') ;

    scanner SingleQuotedUnicodeStringLiteral = { enabled from "0.7.0" ("unicode'" ((EscapeSequence | (! "\n\r'\\")) *) '\'') } ;

    scanner Slash = '/' ;

    scanner SlashEqual = "/=" ;

    scanner SolidityKeyword = "solidity" ;

    scanner StorageKeyword = "storage" ;

    scanner StringKeyword = "string" ;

    scanner StructKeyword = "struct" ;

    scanner SwitchKeyword = "switch" ;

    scanner SzaboKeyword = { disabled from "0.7.0" "szabo" } ;

    scanner ThrowKeyword = { disabled from "0.5.0" "throw" } ;

    scanner Tilde = '~' ;

    scanner TrueKeyword = "true" ;

    scanner TryKeyword = { enabled from "0.6.0" "try" } ;

    scanner TypeKeyword = { enabled from "0.5.3" "type" } ;

    scanner UncheckedKeyword = { enabled from "0.8.0" "unchecked" } ;

    scanner UnicodeEscape = ('u' (HexCharacter) (HexCharacter) (HexCharacter) (HexCharacter)) ;

    scanner UnicodeStringLiteral = { enabled from "0.7.0" (SingleQuotedUnicodeStringLiteral | DoubleQuotedUnicodeStringLiteral) } ;

    scanner UnsignedFixedType = ("ufixed" (FixedTypeSize ?)) ;

    scanner UnsignedIntegerType = ("uint" (IntegerTypeSize ?)) ;

    scanner UsingKeyword = "using" ;

    scanner VarKeyword = { disabled from "0.5.0" "var" } ;

    scanner VersionPragmaValue = (('*' | ('0' .. '9') | 'X' | 'x') +) ;

    scanner ViewKeyword = "view" ;

    scanner VirtualKeyword = { enabled from "0.6.0" "virtual" } ;

    scanner WeeksKeyword = "weeks" ;

    scanner WeiKeyword = "wei" ;

    scanner WhileKeyword = "while" ;

    scanner Whitespace = (('\t' | ' ') +) ;

    scanner YearsKeyword = { disabled from "0.5.0" "years" } ;

    scanner YulDecimalLiteral = ("0" | (('1' .. '9') (('0' .. '9') *))) ;
    scanner YulHexLiteral = ("0x" ((HexCharacter) +)) ;
    scanner YulIdentifier = ( RawIdentifier ) ;

    // scanner YulKeyword = ({ disabled from "0.6.0" (BreakKeyword | CaseKeyword | ContinueKeyword | DefaultKeyword | FalseKeyword | ForKeyword | FunctionKeyword | IfKeyword | LetKeyword | SwitchKeyword | TrueKeyword) } | { enabled from "0.6.0" (BreakKeyword | CaseKeyword | ContinueKeyword | DefaultKeyword | FalseKeyword | ForKeyword | FunctionKeyword | IfKeyword | LeaveKeyword | LetKeyword | SwitchKeyword | TrueKeyword) }) ;
    // scanner YulReservedWord = "hex" ;

}
