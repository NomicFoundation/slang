// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

#[allow(clippy::needless_raw_string_hashes)]
#[allow(dead_code)] // TODO(#982): use to create the graph
pub const BINDING_RULES_SOURCE: &str = r#####"
    global ROOT_NODE

attribute node_definition = node     => type = "pop_symbol", node_symbol = node, is_definition
attribute node_reference = node      => type = "push_symbol", node_symbol = node, is_reference
attribute node_symbol = node         => symbol = (source-text node), source_node = node
attribute pop_symbol = symbol        => type = "pop_symbol", symbol = symbol
attribute push_symbol = symbol       => type = "push_symbol", symbol = symbol
attribute symbol_definition = symbol => type = "pop_symbol", symbol = symbol, is_definition
attribute symbol_reference = symbol  => type = "push_symbol", symbol = symbol, is_reference


;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
;;; Source unit (aka .sol file)
;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;

@source_unit [SourceUnit] {
  node @source_unit.lexical_scope
  node @source_unit.defs
}

@definition (
      [ContractDefinition]
    | [InterfaceDefinition]
    | [LibraryDefinition]
    | [StructDefinition]
    | [EnumDefinition]
    | [FunctionDefinition]
    | [ConstantDefinition]
    | [ErrorDefinition]
    | [UserDefinedValueTypeDefinition]
    | [EventDefinition]
) {
  ;; All these nodes provide named definitions
  node @definition.lexical_scope
  node @definition.defs
}

;; Top-level definitions
@source_unit [SourceUnit ... [SourceUnitMembers
    ...
    [SourceUnitMember @unit_member (
          [ContractDefinition]
        | [InterfaceDefinition]
        | [LibraryDefinition]
        | [StructDefinition]
        | [EnumDefinition]
        | [FunctionDefinition]
        | [ConstantDefinition]
        | [ErrorDefinition]
        | [UserDefinedValueTypeDefinition]
        | [EventDefinition]
    )]
    ...
] ...] {
  edge @unit_member.lexical_scope -> @source_unit.lexical_scope

  ;; ... are available in the lexical scope
  edge @source_unit.lexical_scope -> @unit_member.defs

  ;; ... and are exported in the file
  edge @source_unit.defs -> @unit_member.defs
}


;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
;;; Named definitions (contracts, functions, libraries, etc.)
;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;

@contract [ContractDefinition ... @name name: [Identifier] ...] {
  node def
  attr (def) node_definition = @name

  edge @contract.defs -> def
}

@interface [InterfaceDefinition ... @name name: [Identifier] ...] {
  node def
  attr (def) node_definition = @name

  edge @interface.defs -> def
}

@library [LibraryDefinition ... @name name: [Identifier] ...] {
  node def
  attr (def) node_definition = @name

  edge @library.defs -> def
}

@function [FunctionDefinition ... name: [FunctionName ... @name [Identifier] ...] ...] {
  node def
  attr (def) node_definition = @name

  edge @function.defs -> def
}

;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
;;; Functions
;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;

@param [Parameter] {
  node @param.lexical_scope
  node @param.defs
}

@param [Parameter ... @name [Identifier]] {
  node def
  attr (def) node_definition = @name

  edge @param.lexical_scope -> def
  edge @param.defs -> def
}

@function [FunctionDefinition
    ...
    parameters: [_ ... parameters: [Parameters ... @param item: [Parameter] ...] ...]
    ...
] {
  ;; Parameters are available in the function scope
  edge @function.lexical_scope -> @param.defs
  attr (@function.lexical_scope -> @param.defs) precedence = 1
}

;; Connect the function to the contract/interface/library they belong to
[SourceUnitMember @unit_member variant: [_
    ...
    members: [_
        ...
        item: [_ @function variant: [FunctionDefinition]]
        ...
    ]
    ...
  ]
] {
  edge @function.lexical_scope -> @unit_member.lexical_scope
}

@body [FunctionBody] {
  node @body.lexical_scope
}

;; Connect the function body to the function definition
@function [FunctionDefinition ... @body body: [FunctionBody] ...] {
  edge @body.lexical_scope -> @function.lexical_scope
}


;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
;;; Blocks
;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;

@block [Block] {
  node @block.lexical_scope
  node @block.defs
  edge @block.lexical_scope -> @block.defs
}

@stmt [Statement] {
  node @stmt.lexical_scope
  node @stmt.defs
  edge @stmt.lexical_scope -> @stmt.defs
}

@block [Block ... statements: [_ ... @stmt [Statement]...] ...] {
  edge @stmt.lexical_scope -> @block.lexical_scope
  edge @block.defs -> @stmt.defs
}

@body [FunctionBody @block variant: [Block]] {
  edge @block.lexical_scope -> @body.lexical_scope
}


;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
;;; Declaration Statements
;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;

@stmt [Statement [VariableDeclarationStatement ... @name name: [Identifier] ...]] {
  node def
  attr (def) node_definition = @name

  edge @stmt.defs -> def
}

@stmt [Statement [TupleDeconstructionStatement
    ...
    [TupleDeconstructionElements
        ...
        item: [_ member: [_ variant: [_ ... @name name: [Identifier]]]]
        ...
    ]
    ...
]] {
  node def
  attr (def) node_definition = @name

  edge @stmt.lexical_scope -> def
  edge @stmt.defs -> def
}


;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
;;; State Variables
;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;

@state_var [StateVariableDefinition] {
  node @state_var.lexical_scope
  node @state_var.defs
}

@state_var [StateVariableDefinition ... @name name: [Identifier] ...] {
  node def
  attr (def) node_definition = @name

  edge @state_var.defs -> def
}

[SourceUnitMember @unit_member variant: [_
    ...
    members: [_
        ...
        item: [_ @state_var variant: [StateVariableDefinition]]
        ...
    ]
    ...
  ]
] {
  edge @state_var.lexical_scope -> @unit_member.lexical_scope
  edge @unit_member.lexical_scope -> @state_var.defs
}

;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
;;; Structure definitions
;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;

@struct [StructDefinition] {
  node @struct.lexical_scope
  node @struct.defs
}

@struct [StructDefinition ... @name name: [Identifier] ...] {
  node def
  attr (def) node_definition = @name

  edge @struct.lexical_scope -> def
  edge @struct.defs -> def
}

;; Connect the struct to the contract/interface/library they belong to
[SourceUnitMember @unit_member variant: [_
    ...
    members: [_
        ...
        item: [_ @struct variant: [StructDefinition]]
        ...
    ]
    ...
  ]
] {
  edge @unit_member.lexical_scope -> @struct.defs
}

@member [StructMember] {
  node @member.lexical_scope
  node @member.defs
}

@member item: [StructMember ... @name name: [Identifier] ...] {
  node member
  attr (member) pop_symbol = "."
  edge @member.defs -> member

  node def
  attr (def) node_definition = @name
  edge member -> def
}

;; TODO: missing connection between the struct field declarations and the struct


;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
;;; Expressions
;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;

@expr [Expression] {
  node @expr.lexical_scope
}

@expr [Expression ... @name variant: [Identifier]] {
  node ref
  attr (ref) node_reference = @name

  edge ref -> @expr.lexical_scope
}

@expr [Expression ... variant: [_ ... @child [Expression] ...] ...] {
  edge @child.lexical_scope -> @expr.lexical_scope
}

@stmt [Statement ... variant: [_ ... @expr [Expression] ...] ...] {
  edge @expr.lexical_scope -> @stmt.lexical_scope
}

@stmt [Statement ... variant: [VariableDeclarationStatement
    ...
    value: [VariableDeclarationValue ... @expr [Expression] ...]
    ...
] ...] {
  edge @expr.lexical_scope -> @stmt.lexical_scope
}

@member [MemberAccess] {
  node @member.lexical_scope
}

@member [MemberAccess @name [Identifier]] {
  node ref
  attr (ref) node_reference = @name

  edge ref -> @member.lexical_scope
}

[MemberAccessExpression
    ...
    @expr operand: [Expression]
    ...
    @member member: [MemberAccess]
    ...
] {
  node member
  attr (member) push_symbol = "."

  edge @member.lexical_scope -> member
  edge member -> @expr.lexical_scope
}

"#####;
