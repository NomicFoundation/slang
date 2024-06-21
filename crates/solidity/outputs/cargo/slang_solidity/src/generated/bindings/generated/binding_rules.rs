// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

#[allow(clippy::needless_raw_string_hashes)]
#[allow(dead_code)] // TODO(#982): use to create the graph
pub const BINDING_RULES_SOURCE: &str = r#####"
    global ROOT_NODE
global VERSION

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
  ;; All lexical_scope nodes eventually connect to the file's root scope
  node @source_unit.lexical_scope

  ;; This provides all the exported symbols from the file
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
  ;; All these CST nodes provide named definitions
  node @definition.lexical_scope
  node @definition.defs
}

;; Top-level definitions...
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

  ;; ... are available in the file's lexical scope
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

  edge @param.defs -> def
}

@function [FunctionDefinition ... parameters: [ParametersDeclaration
    ...
    [Parameters ... @param item: [Parameter] ...]
    ...
] ...] {
  edge @param.lexical_scope -> @function.lexical_scope

  ;; Input parameters are available in the function scope
  edge @function.lexical_scope -> @param.defs
  attr (@function.lexical_scope -> @param.defs) precedence = 1
}

@function [FunctionDefinition ... returns: [ReturnsDeclaration
    ...
    [ParametersDeclaration ... [Parameters ... @param item: [Parameter] ...] ...]
    ...
] ...] {
  edge @param.lexical_scope -> @function.lexical_scope

  ;; Return parameters are available in the function scope
  edge @function.lexical_scope -> @param.defs
  attr (@function.lexical_scope -> @param.defs) precedence = 1
}

;; Connect function's lexical scope with the enclosing
;; contract/interface/library, and make the function itself available in the
;; enclosing contract/interface/library scope.
;; NB. free-functions (ie. those defined at the file's level) are already
;; covered above

@contract [ContractDefinition ... members: [ContractMembers
    ...
    item: [ContractMember @function variant: [FunctionDefinition]]
    ...
] ...] {
  edge @function.lexical_scope -> @contract.lexical_scope
  edge @contract.lexical_scope -> @function.defs
}

@interface [InterfaceDefinition ... members: [InterfaceMembers
    ...
    item: [ContractMember @function variant: [FunctionDefinition]]
    ...
] ...] {
  edge @function.lexical_scope -> @interface.lexical_scope
  edge @interface.lexical_scope -> @function.defs
}

@library [LibraryDefinition ... members: [LibraryMembers
    ...
    item: [ContractMember @function variant: [FunctionDefinition]]
    ...
] ...] {
  edge @function.lexical_scope -> @library.lexical_scope
  edge @library.lexical_scope -> @function.defs
}



;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
;;; Blocks
;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;

@block [Block] {
  node @block.lexical_scope
  node @block.defs
}

@stmt [Statement] {
  node @stmt.lexical_scope
  node @stmt.defs
}

;; FIXME: For C99 scoping, we should link each statement's lexical_scope to the
;; previous lexical_scope, and to the statement's defs (to make the def
;; available to the statement itself).
;; Only the first statement in the block connects to the block's lexical_scope.

@block [Block ... [Statements ... @stmt [Statement]...] ...] {
  edge @stmt.lexical_scope -> @block.lexical_scope

  ;; FIXME: doesn't work for C99 scoping
  edge @block.lexical_scope -> @stmt.defs
  attr (@block.lexical_scope -> @stmt.defs) precedence = 1

  ;; Hoist statement definitions (< 0.5.0)
  if (version-matches VERSION "< 0.5.0") {
    edge @block.defs -> @stmt.defs
  }
}

@stmt [Statement @block variant: [Block]] {
  edge @block.lexical_scope -> @stmt.lexical_scope

  ;; Hoist block definitions (< 0.5.0)
  edge @stmt.defs -> @block.defs
}

;; Connect the function body's block lexical scope to the function
@function [FunctionDefinition ... [FunctionBody @block [Block]] ...] {
  edge @block.lexical_scope -> @function.lexical_scope
}


;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
;;; Declaration Statements introducing variables
;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;

@stmt [Statement [VariableDeclarationStatement ... @name name: [Identifier] ...]] {
  node def
  attr (def) node_definition = @name

  edge @stmt.defs -> def
}

@stmt [Statement [TupleDeconstructionStatement ... [TupleDeconstructionElements
    ...
    [TupleDeconstructionElement [TupleMember variant: [_ ... @name name: [Identifier]]]]
    ...
] ...]] {
  node def
  attr (def) node_definition = @name

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

;; NB. Even though the grammar allows it, state variables can only be declared
;; inside contracts, and not interfaces or libraries. So, we will only bind
;; contract state variables.
@contract [ContractDefinition ... members: [ContractMembers
    ...
    item: [ContractMember @state_var variant: [StateVariableDefinition]]
    ...
] ...] {
  edge @state_var.lexical_scope -> @contract.lexical_scope
  edge @contract.lexical_scope -> @state_var.defs
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

  edge @struct.defs -> def
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

;; TODO: connect lexical scope of structs to the rest of the source unit and
;; make them available in the parent scope (either top-level, or
;; contract/interface/library).


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

;; Expressions used for variable declarations
@stmt [Statement ... variant: [VariableDeclarationStatement
    ...
    value: [VariableDeclarationValue ... @expr [Expression] ...]
    ...
] ...] {
  edge @expr.lexical_scope -> @stmt.lexical_scope
}


;;; Member access expressions

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
