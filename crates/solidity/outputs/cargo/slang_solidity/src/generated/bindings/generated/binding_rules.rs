// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

#[allow(clippy::needless_raw_string_hashes)]
#[allow(dead_code)] // TODO(#982): use to create the graph
pub const BINDING_RULES_SOURCE: &str = r#####"
    attribute node_definition = node     => type = "pop_symbol", node_symbol = node, is_definition
attribute node_reference = node      => type = "push_symbol", node_symbol = node, is_reference
attribute node_symbol = node         => symbol = (source-text node), source_node = node
attribute pop_symbol = symbol        => type = "pop_symbol", symbol = symbol
attribute push_symbol = symbol       => type = "push_symbol", symbol = symbol
attribute symbol_definition = symbol => type = "pop_symbol", symbol = symbol, is_definition
attribute symbol_reference = symbol  => type = "push_symbol", symbol = symbol, is_reference

;; Generalities
;; - we will define two nodes for all meaningful CST nodes
;;   - a lexical_scope node which will connect "upwards" towards the root of the CST
;;   - a defs node to access the definitions reachable from each node (usually connecting "downwards")
;; - the pair will not be created for every CST node, as there is a lot of redundancy in the tree
;; - identifier nodes that are part of the definition of an artifact
;;   will create graph nodes with the node_definition attributes
;; - identifier nodes that are references will create graph nodes with the node_reference attributes


;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
;;; Source unit (aka .sol file)
;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;

@source_unit [SourceUnit] {
  node @source_unit.lexical_scope
  node @source_unit.defs
}


;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
;;; Contract definitions
;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;

@contract [ContractDefinition] {
  node @contract.lexical_scope
  node @contract.defs
}

@contract [ContractDefinition ... @name name: [Identifier] ...] {
  node def
  attr (def) node_definition = @name

  edge @contract.lexical_scope -> def
  edge @contract.defs -> def
}

;; Connect the contract to its containing source unit
@source_unit [SourceUnit ... [SourceUnitMembers
    ...
    [SourceUnitMember @contract [ContractDefinition]]
    ...
] ...] {
  edge @source_unit.defs -> @contract.defs
  edge @contract.lexical_scope -> @source_unit.lexical_scope
}


;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
;;; Interface definitions
;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;

@interface [InterfaceDefinition] {
  node @interface.lexical_scope
  node @interface.defs
}

@interface [InterfaceDefinition ... @name name: [Identifier] ...] {
  node def
  attr (def) node_definition = @name

  edge @interface.lexical_scope -> def
  edge @interface.defs -> def
}

;; Connect the interface to its containing source unit
@source_unit [SourceUnit [SourceUnitMembers
    ...
    [SourceUnitMember @interface [InterfaceDefinition]]
    ...
]] {
  edge @source_unit.defs -> @interface.defs
  edge @interface.lexical_scope -> @source_unit.lexical_scope
}


;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
;;; Library definitions
;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;

@library [LibraryDefinition] {
  node @library.lexical_scope
  node @library.defs
}

@library [LibraryDefinition ... @name name: [Identifier] ...] {
  node def
  attr (def) node_definition = @name

  edge @library.lexical_scope -> def
  edge @library.defs -> def
}

;; Connect the library to its containing source unit
@source_unit [SourceUnit [SourceUnitMembers
    ...
    [SourceUnitMember @library [LibraryDefinition]]
    ...
]] {
  edge @source_unit.defs -> @library.defs
  edge @library.lexical_scope -> @source_unit.lexical_scope
}


;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
;;; Function definitions
;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;

@function [FunctionDefinition] {
  node @function.lexical_scope
  node @function.defs
}

@function [FunctionDefinition ... name: [FunctionName ... @name [Identifier] ...] ...] {
  node def
  attr (def) node_definition = @name

  edge @function.lexical_scope -> def
  edge @function.defs -> def
}

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

;; Connect the parameters to the functions they belong to
@function [FunctionDefinition
    ...
    parameters: [_ ... parameters: [Parameters ... @param item: [Parameter] ...] ...]
    ...
] {
  edge @function.lexical_scope -> @param.defs
  edge @function.defs -> @param.defs
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
  edge @unit_member.lexical_scope -> @function.defs
  edge @unit_member.defs -> @function.defs
  edge @function.lexical_scope -> @unit_member.lexical_scope
}

@body [FunctionBody] {
  node @body.lexical_scope
  node @body.defs
}

;; Connect the function body to the function definition
@function [FunctionDefinition ... @body body: [FunctionBody] ...] {
  edge @body.lexical_scope -> @function.lexical_scope
  edge @function.defs -> @body.defs
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

@block [Block ... statements: [_ ... @stmt [Statement]...] ...] {
  edge @stmt.lexical_scope -> @block.lexical_scope
  edge @block.defs -> @stmt.defs
}

@body [FunctionBody @block variant: [Block]] {
  edge @block.lexical_scope -> @body.lexical_scope
  edge @body.defs -> @block.defs
}


;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
;;; Declaration Statements
;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;

@stmt [Statement [VariableDeclarationStatement ... @name name: [Identifier] ...]] {
  node def
  attr (def) node_definition = @name

  edge @stmt.lexical_scope -> def
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

  edge @state_var.lexical_scope -> def
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
  edge @unit_member.lexical_scope -> @state_var.defs
  edge @unit_member.defs -> @state_var.defs
  edge @state_var.lexical_scope -> @unit_member.lexical_scope
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
  edge @unit_member.defs -> @struct.defs
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
