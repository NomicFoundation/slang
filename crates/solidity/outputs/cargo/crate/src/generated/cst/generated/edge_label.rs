// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

#[repr(u8)]
#[derive(
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    serde::Serialize,
    strum_macros::AsRefStr,
    strum_macros::Display,
    strum_macros::EnumString,
    strum_macros::IntoStaticStr,
)]
#[strum(serialize_all = "snake_case")]
#[derive(Clone, Copy)]
/// Represents the different types of relationships between nodes in the syntax tree.
pub enum EdgeLabel {
    /// Represents a child node with the label `root`.
    Root,
    /// Represents a child node with the label `unrecognized`.
    Unrecognized,
    /// Represents a child node with the label `missing`.
    Missing,
    /// Represents a child node with the label `item`.
    Item,
    /// Represents a child node with the label `variant`.
    Variant,
    /// Represents a child node with the label `separator`.
    Separator,
    /// Represents a child node with the label `operand`.
    Operand,
    /// Represents a child node with the label `left_operand`.
    LeftOperand,
    /// Represents a child node with the label `right_operand`.
    RightOperand,
    /// Represents a child node with the label `leading_trivia`.
    LeadingTrivia,
    /// Represents a child node with the label `trailing_trivia`.
    TrailingTrivia,

    /// Represents a child node with the label `abicoder_keyword`.
    AbicoderKeyword,
    /// Represents a child node with the label `abstract_keyword`.
    AbstractKeyword,
    /// Represents a child node with the label `address_keyword`.
    AddressKeyword,
    /// Represents a child node with the label `alias`.
    Alias,
    /// Represents a child node with the label `anonymous_keyword`.
    AnonymousKeyword,
    /// Represents a child node with the label `arguments`.
    Arguments,
    /// Represents a child node with the label `as_keyword`.
    AsKeyword,
    /// Represents a child node with the label `assembly_keyword`.
    AssemblyKeyword,
    /// Represents a child node with the label `assignment`.
    Assignment,
    /// Represents a child node with the label `asterisk`.
    Asterisk,
    /// Represents a child node with the label `at_keyword`.
    AtKeyword,
    /// Represents a child node with the label `attributes`.
    Attributes,
    /// Represents a child node with the label `block`.
    Block,
    /// Represents a child node with the label `body`.
    Body,
    /// Represents a child node with the label `break_keyword`.
    BreakKeyword,
    /// Represents a child node with the label `case_keyword`.
    CaseKeyword,
    /// Represents a child node with the label `cases`.
    Cases,
    /// Represents a child node with the label `catch_clauses`.
    CatchClauses,
    /// Represents a child node with the label `catch_keyword`.
    CatchKeyword,
    /// Represents a child node with the label `clause`.
    Clause,
    /// Represents a child node with the label `close_brace`.
    CloseBrace,
    /// Represents a child node with the label `close_bracket`.
    CloseBracket,
    /// Represents a child node with the label `close_paren`.
    CloseParen,
    /// Represents a child node with the label `colon`.
    Colon,
    /// Represents a child node with the label `condition`.
    Condition,
    /// Represents a child node with the label `constant_keyword`.
    ConstantKeyword,
    /// Represents a child node with the label `constructor_keyword`.
    ConstructorKeyword,
    /// Represents a child node with the label `continue_keyword`.
    ContinueKeyword,
    /// Represents a child node with the label `contract_keyword`.
    ContractKeyword,
    /// Represents a child node with the label `default_keyword`.
    DefaultKeyword,
    /// Represents a child node with the label `do_keyword`.
    DoKeyword,
    /// Represents a child node with the label `elements`.
    Elements,
    /// Represents a child node with the label `else_branch`.
    ElseBranch,
    /// Represents a child node with the label `else_keyword`.
    ElseKeyword,
    /// Represents a child node with the label `emit_keyword`.
    EmitKeyword,
    /// Represents a child node with the label `end`.
    End,
    /// Represents a child node with the label `enum_keyword`.
    EnumKeyword,
    /// Represents a child node with the label `equal`.
    Equal,
    /// Represents a child node with the label `equal_greater_than`.
    EqualGreaterThan,
    /// Represents a child node with the label `error`.
    Error,
    /// Represents a child node with the label `error_keyword`.
    ErrorKeyword,
    /// Represents a child node with the label `event`.
    Event,
    /// Represents a child node with the label `event_keyword`.
    EventKeyword,
    /// Represents a child node with the label `experimental_keyword`.
    ExperimentalKeyword,
    /// Represents a child node with the label `expression`.
    Expression,
    /// Represents a child node with the label `fallback_keyword`.
    FallbackKeyword,
    /// Represents a child node with the label `false_expression`.
    FalseExpression,
    /// Represents a child node with the label `feature`.
    Feature,
    /// Represents a child node with the label `flags`.
    Flags,
    /// Represents a child node with the label `for_keyword`.
    ForKeyword,
    /// Represents a child node with the label `from_keyword`.
    FromKeyword,
    /// Represents a child node with the label `function_keyword`.
    FunctionKeyword,
    /// Represents a child node with the label `global_keyword`.
    GlobalKeyword,
    /// Represents a child node with the label `identifier`.
    Identifier,
    /// Represents a child node with the label `if_keyword`.
    IfKeyword,
    /// Represents a child node with the label `import_keyword`.
    ImportKeyword,
    /// Represents a child node with the label `index`.
    Index,
    /// Represents a child node with the label `indexed_keyword`.
    IndexedKeyword,
    /// Represents a child node with the label `inheritance`.
    Inheritance,
    /// Represents a child node with the label `initialization`.
    Initialization,
    /// Represents a child node with the label `interface_keyword`.
    InterfaceKeyword,
    /// Represents a child node with the label `is_keyword`.
    IsKeyword,
    /// Represents a child node with the label `items`.
    Items,
    /// Represents a child node with the label `iterator`.
    Iterator,
    /// Represents a child node with the label `key_type`.
    KeyType,
    /// Represents a child node with the label `label`.
    Label,
    /// Represents a child node with the label `layout_keyword`.
    LayoutKeyword,
    /// Represents a child node with the label `leave_keyword`.
    LeaveKeyword,
    /// Represents a child node with the label `let_keyword`.
    LetKeyword,
    /// Represents a child node with the label `library_keyword`.
    LibraryKeyword,
    /// Represents a child node with the label `literal`.
    Literal,
    /// Represents a child node with the label `mapping_keyword`.
    MappingKeyword,
    /// Represents a child node with the label `member`.
    Member,
    /// Represents a child node with the label `members`.
    Members,
    /// Represents a child node with the label `minus`.
    Minus,
    /// Represents a child node with the label `minus_greater_than`.
    MinusGreaterThan,
    /// Represents a child node with the label `modifier_keyword`.
    ModifierKeyword,
    /// Represents a child node with the label `name`.
    Name,
    /// Represents a child node with the label `new_keyword`.
    NewKeyword,
    /// Represents a child node with the label `open_brace`.
    OpenBrace,
    /// Represents a child node with the label `open_bracket`.
    OpenBracket,
    /// Represents a child node with the label `open_paren`.
    OpenParen,
    /// Represents a child node with the label `operator`.
    Operator,
    /// Represents a child node with the label `options`.
    Options,
    /// Represents a child node with the label `overridden`.
    Overridden,
    /// Represents a child node with the label `override_keyword`.
    OverrideKeyword,
    /// Represents a child node with the label `parameters`.
    Parameters,
    /// Represents a child node with the label `path`.
    Path,
    /// Represents a child node with the label `paths`.
    Paths,
    /// Represents a child node with the label `payable_keyword`.
    PayableKeyword,
    /// Represents a child node with the label `period`.
    Period,
    /// Represents a child node with the label `pragma`.
    Pragma,
    /// Represents a child node with the label `pragma_keyword`.
    PragmaKeyword,
    /// Represents a child node with the label `question_mark`.
    QuestionMark,
    /// Represents a child node with the label `receive_keyword`.
    ReceiveKeyword,
    /// Represents a child node with the label `return_keyword`.
    ReturnKeyword,
    /// Represents a child node with the label `returns`.
    Returns,
    /// Represents a child node with the label `returns_keyword`.
    ReturnsKeyword,
    /// Represents a child node with the label `revert_keyword`.
    RevertKeyword,
    /// Represents a child node with the label `semicolon`.
    Semicolon,
    /// Represents a child node with the label `sets`.
    Sets,
    /// Represents a child node with the label `solidity_keyword`.
    SolidityKeyword,
    /// Represents a child node with the label `specifiers`.
    Specifiers,
    /// Represents a child node with the label `start`.
    Start,
    /// Represents a child node with the label `statements`.
    Statements,
    /// Represents a child node with the label `storage_location`.
    StorageLocation,
    /// Represents a child node with the label `struct_keyword`.
    StructKeyword,
    /// Represents a child node with the label `switch_keyword`.
    SwitchKeyword,
    /// Represents a child node with the label `symbols`.
    Symbols,
    /// Represents a child node with the label `target`.
    Target,
    /// Represents a child node with the label `throw_keyword`.
    ThrowKeyword,
    /// Represents a child node with the label `true_expression`.
    TrueExpression,
    /// Represents a child node with the label `try_keyword`.
    TryKeyword,
    /// Represents a child node with the label `type_keyword`.
    TypeKeyword,
    /// Represents a child node with the label `type_name`.
    TypeName,
    /// Represents a child node with the label `types`.
    Types,
    /// Represents a child node with the label `unchecked_keyword`.
    UncheckedKeyword,
    /// Represents a child node with the label `unit`.
    Unit,
    /// Represents a child node with the label `using_keyword`.
    UsingKeyword,
    /// Represents a child node with the label `value`.
    Value,
    /// Represents a child node with the label `value_type`.
    ValueType,
    /// Represents a child node with the label `var_keyword`.
    VarKeyword,
    /// Represents a child node with the label `variable`.
    Variable,
    /// Represents a child node with the label `variable_type`.
    VariableType,
    /// Represents a child node with the label `variables`.
    Variables,
    /// Represents a child node with the label `version`.
    Version,
    /// Represents a child node with the label `while_keyword`.
    WhileKeyword,
}

impl crate::cst::EdgeLabelExtensions for EdgeLabel {}

impl Default for EdgeLabel {
    fn default() -> Self {
        Self::Root
    }
}
