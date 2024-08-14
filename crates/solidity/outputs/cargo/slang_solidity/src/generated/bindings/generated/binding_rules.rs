// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

#[allow(clippy::needless_raw_string_hashes)]
#[allow(dead_code)] // TODO(#982): use to create the graph
pub const BINDING_RULES_SOURCE: &str = r#####"
    global ROOT_NODE
global FILE_PATH

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

  node export
  attr (export) pop_symbol = FILE_PATH
  edge ROOT_NODE -> export
  edge export -> @source_unit.defs
}

;; Definition entities that can appear at the source unit level
;; We define them individually to get better variable names when debugging
;; For all of the following:
;; - lexical_scope is the node that connect upwards for binding resolution
;; - def provides the definition entry for the entity (aka. the name)
;; - members is for internal use and it's where the nested definitions are found

@contract [ContractDefinition] {
  node @contract.lexical_scope
  node @contract.def
  node @contract.members
  node @contract.type_members

  edge @contract.lexical_scope -> @contract.members
  edge @contract.lexical_scope -> @contract.type_members
}

@interface [InterfaceDefinition] {
  node @interface.lexical_scope
  node @interface.def
  node @interface.members
  node @interface.type_members

  edge @interface.lexical_scope -> @interface.members
  edge @interface.lexical_scope -> @interface.type_members
}

@library [LibraryDefinition] {
  node @library.lexical_scope
  node @library.def
  node @library.members

  edge @library.lexical_scope -> @library.members
}

@struct [StructDefinition] {
  node @struct.lexical_scope
  node @struct.def
  node @struct.members
}

@enum [EnumDefinition] {
  node @enum.lexical_scope
  node @enum.def
  node @enum.members
}

@function [FunctionDefinition] {
  node @function.lexical_scope
  node @function.def
}

@constant [ConstantDefinition] {
  node @constant.lexical_scope
  node @constant.def
}

@error [ErrorDefinition] {
  node @error.lexical_scope
  node @error.def
}

@value_type [UserDefinedValueTypeDefinition] {
  node @value_type.lexical_scope
  node @value_type.def
}

@event [EventDefinition] {
  node @event.lexical_scope
  node @event.def
}

;; Top-level definitions...
@source_unit [SourceUnit [SourceUnitMembers
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
]] {
  edge @unit_member.lexical_scope -> @source_unit.lexical_scope

  ;; ... are available in the file's lexical scope
  edge @source_unit.lexical_scope -> @unit_member.def

  ;; ... and are exported in the file
  edge @source_unit.defs -> @unit_member.def
}


;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
;;; Imports
;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;

@source_unit [SourceUnit [SourceUnitMembers
     [SourceUnitMember [ImportDirective
         [ImportClause @import (
               [PathImport]
             | [NamedImport]
             | [ImportDeconstruction]
         )]
     ]]
]] {
   node @import.defs
   edge @source_unit.defs -> @import.defs
   edge @source_unit.lexical_scope -> @import.defs
}

[ImportClause [_ @path path: [StringLiteral]]] {
  ;; This node represents the imported file and the @path.import node is used by
  ;; all subsequent import rules
  node @path.import
  scan (source-text @path) {
    "^\\s*[\"'](.+)[\"']\\s*$" {
      let resolved_path = (resolve-path FILE_PATH $1)
      attr (@path.import) push_symbol = resolved_path
    }
    ;; TODO: if there are other cases possible, we should signal it as an error
  }
  edge @path.import -> ROOT_NODE
}


;;; `import <URI>`
@import [PathImport @path path: [StringLiteral] .] {
  ;; This is the "lexical" connection, which makes all symbols exported from the
  ;; imported source unit available for resolution globally at this' source unit
  ;; scope
  edge @import.defs -> @path.import
}

;;; `import <URI> as <IDENT>`
@import [PathImport
   @path path: [StringLiteral]
   alias: [ImportAlias @alias [Identifier]]
] {
  node def
  attr (def) node_definition = @alias
  attr (def) definiens_node = @import
  edge @import.defs -> def

  node member
  attr (member) pop_symbol = "."
  edge def -> member

  ;; Lexical connection, which makes the import available as a member through
  ;; the alias identifier
  edge member -> @path.import
}

;;; `import * as <IDENT> from <URI>`
@import [NamedImport
    alias: [ImportAlias @alias [Identifier]]
    @path path: [StringLiteral]
] {
  node def
  attr (def) node_definition = @alias
  attr (def) definiens_node = @import
  edge @import.defs -> def

  node member
  attr (member) pop_symbol = "."
  edge def -> member

  ;; Lexical connection, which makes the import available as a member through
  ;; the alias identifier
  edge member -> @path.import
}

;;; `import {<SYMBOL> [as <IDENT>] ...} from <PATH>`
@import [ImportDeconstruction
    symbols: [ImportDeconstructionSymbols @symbol [ImportDeconstructionSymbol]]
    @path path: [StringLiteral]
] {
  ;; We define these intermediate nodes for convenience only, to make the
  ;; queries simpler in the two rules below
  node @symbol.def
  edge @import.defs -> @symbol.def

  node @symbol.import
  edge @symbol.import -> @path.import
}

@symbol [ImportDeconstructionSymbol @name name: [Identifier] .] {
  node def
  attr (def) node_definition = @name
  attr (def) definiens_node = @symbol
  edge @symbol.def -> def

  node import
  attr (import) node_reference = @name
  edge def -> import

  edge import -> @symbol.import
}

@symbol [ImportDeconstructionSymbol
    @name name: [Identifier]
    alias: [ImportAlias @alias [Identifier]]
] {
  node def
  attr (def) node_definition = @alias
  attr (def) definiens_node = @symbol
  edge @symbol.def -> def

  node import
  attr (import) node_reference = @name
  edge def -> import

  edge import -> @symbol.import
}

;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
;;; Named definitions (contracts, functions, libraries, etc.)
;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;

@contract [ContractDefinition @name name: [Identifier]] {
  node def
  attr (def) node_definition = @name
  attr (def) definiens_node = @contract

  edge @contract.def -> def

  node type_def
  attr (type_def) pop_symbol = "@typeof"

  node member
  attr (member) pop_symbol = "."

  edge def -> type_def
  edge type_def -> member
  edge member -> @contract.members

  node type_member
  attr (type_member) pop_symbol = "."
  edge def -> type_member

  edge type_member -> @contract.type_members
}

@interface [InterfaceDefinition @name name: [Identifier]] {
  node def
  attr (def) node_definition = @name
  attr (def) definiens_node = @interface

  edge @interface.def -> def

  node type_def
  attr (type_def) pop_symbol = "@typeof"

  node member
  attr (member) pop_symbol = "."

  edge def -> type_def
  edge type_def -> member
  edge member -> @interface.members

  node type_member
  attr (type_member) pop_symbol = "."
  edge def -> type_member

  edge type_member -> @interface.type_members
}

@library [LibraryDefinition @name name: [Identifier]] {
  node def
  attr (def) node_definition = @name
  attr (def) definiens_node = @library

  edge @library.def -> def

  node member
  attr (member) pop_symbol = "."
  edge def -> member

  edge member -> @library.members
}

@function [FunctionDefinition name: [FunctionName @name [Identifier]]] {
  node def
  attr (def) node_definition = @name
  attr (def) definiens_node = @function

  edge @function.def -> def
}


;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
;;; Type names
;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;

@type_name [TypeName] {
  ;; This node should connect to the parent lexical scope to resolve the type
  node @type_name.type_ref

  ;; This represents the output of the type, ie. the node to which a variable
  ;; that is of this type should connect through a @typeof node
  node @type_name.output
}

@id_path [IdentifierPath] {
  node @id_path.left
  node @id_path.right
}

@type_name [TypeName @id_path [IdentifierPath]] {
  ;; For an identifier path used as a type, the left-most element is the one
  ;; that connects to the parent lexical scope, because the name resolution
  ;; starts at the left of the identifier.
  edge @id_path.left -> @type_name.type_ref

  ;; Conversely, the complete type is found at the right-most name, and that's
  ;; where users of this type should link to (eg. a variable declaration).
  edge @type_name.output -> @id_path.right
}

;; The identifier path constructs a path of nodes connected from right to left
[IdentifierPath @name [Identifier]] {
  node @name.ref
  attr (@name.ref) node_reference = @name
}

@id_path [IdentifierPath @name [Identifier] .] {
  edge @id_path.right -> @name.ref
}

[IdentifierPath @left_name [Identifier] . [Period] . @right_name [Identifier]] {
  node member
  attr (member) push_symbol = "."

  edge @right_name.ref -> member
  edge member -> @left_name.ref
}

@id_path [IdentifierPath . @name [Identifier]] {
  edge @name.ref -> @id_path.left
}


;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
;;; Functions
;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;

@param [Parameter] {
  node @param.lexical_scope
  node @param.def
}

@param [Parameter @type_name [TypeName]] {
  edge @type_name.type_ref -> @param.lexical_scope
}

@param [Parameter @type_name [TypeName] @name [Identifier]] {
  node def
  attr (def) node_definition = @name
  attr (def) definiens_node = @param

  edge @param.def -> def

  node typeof
  attr (typeof) push_symbol = "@typeof"

  edge def -> typeof
  edge typeof -> @type_name.output
}

@function [FunctionDefinition parameters: [ParametersDeclaration
    [Parameters @param item: [Parameter]]
]] {
  edge @param.lexical_scope -> @function.lexical_scope

  ;; Input parameters are available in the function scope
  edge @function.lexical_scope -> @param.def
  attr (@function.lexical_scope -> @param.def) precedence = 1
}

@function [FunctionDefinition returns: [ReturnsDeclaration
    [ParametersDeclaration [Parameters @param item: [Parameter]]]
]] {
  edge @param.lexical_scope -> @function.lexical_scope

  ;; Return parameters are available in the function scope
  edge @function.lexical_scope -> @param.def
  attr (@function.lexical_scope -> @param.def) precedence = 1
}

;; Connect function's lexical scope with the enclosing
;; contract/interface/library, and make the function itself available in the
;; enclosing contract/interface/library scope.
;; NB. free-functions (ie. those defined at the file's level) are already
;; covered above

@contract [ContractDefinition members: [ContractMembers
    item: [ContractMember @function variant: [FunctionDefinition]]
]] {
  edge @function.lexical_scope -> @contract.lexical_scope
  edge @contract.members -> @function.def
}

@interface [InterfaceDefinition members: [InterfaceMembers
    item: [ContractMember @function variant: [FunctionDefinition]]
]] {
  edge @function.lexical_scope -> @interface.lexical_scope
  edge @interface.members -> @function.def
}

@library [LibraryDefinition members: [LibraryMembers
    item: [ContractMember @function variant: [FunctionDefinition]]
]] {
  edge @function.lexical_scope -> @library.lexical_scope
  edge @library.members -> @function.def
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

  if (version-matches ">= 0.5.0") {
    ;; For Solidity >= 0.5.0, definitions are immediately available in the
    ;; statement scope. For < 0.5.0 this is also true, but resolved through the
    ;; enclosing block's lexical scope.
    edge @stmt.lexical_scope -> @stmt.defs
    attr (@stmt.lexical_scope -> @stmt.defs) precedence = 1
  }
}

;; The first statement in a block
@block [Block [Statements . @stmt [Statement]]] {
  if (version-matches ">= 0.5.0") {
    edge @stmt.lexical_scope -> @block.lexical_scope
  }
}

;; Two consecutive statements
[Statements @left_stmt [Statement] . @right_stmt [Statement]] {
  if (version-matches ">= 0.5.0") {
    edge @right_stmt.lexical_scope -> @left_stmt.lexical_scope
  }
}

@block [Block [Statements @stmt [Statement]]] {
  ;; Hoist statement definitions for Solidity < 0.5.0
  if (version-matches "< 0.5.0") {
    ;; definitions are carried over to the block
    edge @block.defs -> @stmt.defs

    ;; resolution happens in the context of the block
    edge @stmt.lexical_scope -> @block.lexical_scope

    ;; and the statement definitions are available block's scope
    edge @block.lexical_scope -> @stmt.defs
    attr (@block.lexical_scope -> @stmt.defs) precedence = 1
  }
}

;; Statements of type block
@stmt [Statement @block variant: [Block]] {
  edge @block.lexical_scope -> @stmt.lexical_scope

  ;; Hoist block definitions (< 0.5.0)
  if (version-matches "< 0.5.0") {
    edge @stmt.defs -> @block.defs
  }
}

;; Connect the function body's block lexical scope to the function
@function [FunctionDefinition [FunctionBody @block [Block]]] {
  edge @block.lexical_scope -> @function.lexical_scope
}


;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
;;; Statements
;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;

;; In the general case for statements the structure is [Statement [StmtVariant]]
;; and we will define the scoped nodes .lexical_scope and (possibly) .defs in
;; the Statement CST node.
;;
;; For expression statements, variable and tuple declarations we defined
;; separately from the enclosing statement to be able to use them in for
;; initialization and condition clauses directly. Also, because we intend to
;; reuse them, all of them must have both a .lexical_scope and .defs scoped
;; nodes.

@expr_stmt [ExpressionStatement] {
  node @expr_stmt.lexical_scope
  node @expr_stmt.defs
}

@stmt [Statement @expr_stmt [ExpressionStatement]] {
  edge @expr_stmt.lexical_scope -> @stmt.lexical_scope
}

@var_decl [VariableDeclarationStatement] {
  node @var_decl.lexical_scope
  node @var_decl.defs
}

@stmt [Statement @var_decl [VariableDeclarationStatement]] {
  edge @var_decl.lexical_scope -> @stmt.lexical_scope
  edge @stmt.defs -> @var_decl.defs
}

@tuple_decon [TupleDeconstructionStatement] {
  node @tuple_decon.lexical_scope
  node @tuple_decon.defs
}

@stmt [Statement @tuple_decon [TupleDeconstructionStatement]] {
  edge @tuple_decon.lexical_scope -> @stmt.lexical_scope
  edge @stmt.defs -> @tuple_decon.defs
}


;;; Variable declaration and tuple deconstruction statements introduce new definitionns

@var_decl [VariableDeclarationStatement
    [VariableDeclarationType @var_type [TypeName]]
    @name name: [Identifier]
] {
  node def
  attr (def) node_definition = @name
  attr (def) definiens_node = @var_decl

  edge @var_decl.defs -> def
  edge @var_type.type_ref -> @var_decl.lexical_scope

  node typeof
  attr (typeof) push_symbol = "@typeof"

  edge def -> typeof
  edge typeof -> @var_type.output
}

@tuple_decon [TupleDeconstructionStatement [TupleDeconstructionElements
    [TupleDeconstructionElement
        @tuple_member [TupleMember variant: [UntypedTupleMember
            @name name: [Identifier]]
        ]
    ]
]] {
  node def
  attr (def) node_definition = @name
  attr (def) definiens_node = @tuple_member

  edge @tuple_decon.defs -> def
}

@tuple_decon [TupleDeconstructionStatement [TupleDeconstructionElements
    [TupleDeconstructionElement
        @tuple_member [TupleMember variant: [TypedTupleMember
            @member_type type_name: [TypeName]
            @name name: [Identifier]]
        ]
    ]
]] {
  node def
  attr (def) node_definition = @name
  attr (def) definiens_node = @tuple_member

  edge @tuple_decon.defs -> def
  edge @member_type.type_ref -> @tuple_decon.lexical_scope

  node typeof
  attr (typeof) push_symbol = "@typeof"

  edge def -> typeof
  edge typeof -> @member_type.output
}


;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
;;; Control statements
;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;

;; If conditionals

@stmt [Statement [IfStatement @body body: [Statement]]] {
  edge @body.lexical_scope -> @stmt.lexical_scope
  if (version-matches "< 0.5.0") {
    edge @stmt.defs -> @body.defs
  }
}

@stmt [Statement [IfStatement else_branch: [ElseBranch @else_body body: [Statement]]]] {
  edge @else_body.lexical_scope -> @stmt.lexical_scope
  if (version-matches "< 0.5.0") {
    edge @stmt.defs -> @else_body.defs
  }
}

;; For loops

@stmt [Statement [ForStatement @body body: [Statement]]] {
  node @stmt.init_defs

  edge @body.lexical_scope -> @stmt.lexical_scope
  edge @body.lexical_scope -> @stmt.init_defs
  if (version-matches "< 0.5.0") {
    edge @stmt.defs -> @body.defs
    edge @stmt.defs -> @stmt.init_defs
  }
}

@stmt [Statement [ForStatement
    initialization: [ForStatementInitialization
        @init_stmt ([ExpressionStatement]
                  | [VariableDeclarationStatement]
                  | [TupleDeconstructionStatement])
    ]
]] {
  edge @init_stmt.lexical_scope -> @stmt.lexical_scope
  edge @stmt.init_defs -> @init_stmt.defs
}

@stmt [Statement [ForStatement
    condition: [ForStatementCondition @cond_stmt [ExpressionStatement]]
]] {
  edge @cond_stmt.lexical_scope -> @stmt.lexical_scope
  edge @cond_stmt.lexical_scope -> @stmt.init_defs
}

;; While loops

@stmt [Statement [WhileStatement @body body: [Statement]]] {
  edge @body.lexical_scope -> @stmt.lexical_scope
  if (version-matches "< 0.5.0") {
    edge @stmt.defs -> @body.defs
  }
}

;; Do-while loops

@stmt [Statement [DoWhileStatement @body body: [Statement]]] {
  edge @body.lexical_scope -> @stmt.lexical_scope
  if (version-matches "< 0.5.0") {
    edge @stmt.defs -> @body.defs
  }
}


;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
;;; State Variables
;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;

@state_var [StateVariableDefinition] {
  node @state_var.lexical_scope
  node @state_var.def
}

@state_var [StateVariableDefinition
    @type_name type_name: [TypeName]
    @name name: [Identifier]
] {
  node def
  attr (def) node_definition = @name
  attr (def) definiens_node = @state_var

  edge @state_var.def -> def
  edge @type_name.type_ref -> @state_var.lexical_scope

  node typeof
  attr (typeof) push_symbol = "@typeof"

  edge def -> typeof
  edge typeof -> @type_name.output
}

;; NB. Even though the grammar allows it, state variables can only be declared
;; inside contracts, and not interfaces or libraries. So, we will only bind
;; contract state variables.
@contract [ContractDefinition members: [ContractMembers
    item: [ContractMember @state_var variant: [StateVariableDefinition]]
]] {
  edge @state_var.lexical_scope -> @contract.lexical_scope
  edge @contract.lexical_scope -> @state_var.def
}


;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
;;; Enum definitions
;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;

@enum [EnumDefinition @name name: [Identifier]] {
  node def
  attr (def) node_definition = @name
  attr (def) definiens_node = @enum

  edge @enum.def -> def

  node member
  attr (member) pop_symbol = "."

  edge def -> member
  edge member -> @enum.members
}

@enum [EnumDefinition
    members: [EnumMembers @item [Identifier]]
] {
  node def
  attr (def) node_definition = @item
  attr (def) definiens_node = @item

  edge @enum.members -> def
}

;; Make the enum available to the enclosing contract/interface/library.
;; NB. top-level enums (ie. those defined at the file's level) are already
;; covered above

@contract [ContractDefinition members: [ContractMembers
    item: [ContractMember @enum variant: [EnumDefinition]]
]] {
  edge @contract.type_members -> @enum.def
}

@interface [InterfaceDefinition members: [InterfaceMembers
    item: [ContractMember @enum variant: [EnumDefinition]]
]] {
  edge @interface.type_members -> @enum.def
}

@library [LibraryDefinition members: [LibraryMembers
    item: [ContractMember @enum variant: [EnumDefinition]]
]] {
  edge @library.members -> @enum.def
}


;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
;;; Structure definitions
;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;

@struct [StructDefinition @name name: [Identifier]] {
  node def
  attr (def) node_definition = @name
  attr (def) definiens_node = @struct

  edge @struct.def -> def

  node type_def
  attr (type_def) pop_symbol = "@typeof"

  node member
  attr (member) pop_symbol = "."

  edge def -> type_def
  edge type_def -> member
  edge member -> @struct.members
}

@struct [StructDefinition [StructMembers @member item: [StructMember]]] {
  node @member.lexical_scope
  edge @member.lexical_scope -> @struct.lexical_scope
}

@struct [StructDefinition [StructMembers
    @member item: [StructMember @type_name [TypeName] @name name: [Identifier]]
]] {
  node def
  attr (def) node_definition = @name
  attr (def) definiens_node = @member

  edge @struct.members -> def

  edge @type_name.type_ref -> @member.lexical_scope

  node typeof
  attr (typeof) push_symbol = "@typeof"

  edge def -> typeof
  edge typeof -> @type_name.output
}

;; Make the struct available to the enclosing contract/interface/library.
;; NB. top-level enums (ie. those defined at the file's level) are already
;; covered above

@contract [ContractDefinition members: [ContractMembers
    item: [ContractMember @struct variant: [StructDefinition]]
]] {
  edge @struct.lexical_scope -> @contract.lexical_scope
  edge @contract.type_members -> @struct.def
}

@interface [InterfaceDefinition members: [InterfaceMembers
    item: [ContractMember @struct variant: [StructDefinition]]
]] {
  edge @struct.lexical_scope -> @interface.lexical_scope
  edge @interface.type_members -> @struct.def
}

@library [LibraryDefinition members: [LibraryMembers
    item: [ContractMember @struct variant: [StructDefinition]]
]] {
  edge @struct.lexical_scope -> @library.lexical_scope
  edge @library.members -> @struct.def
}


;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
;;; Expressions
;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;

@expr [Expression] {
  node @expr.lexical_scope
  ;; this is an output scope for use in member access
  node @expr.output
}

;; General case for nested expressions
@expr [Expression variant: [_ @child [Expression]]] {
  edge @child.lexical_scope -> @expr.lexical_scope
}

;; Expressions statements
@expr_stmt [ExpressionStatement @expr [Expression]] {
  edge @expr.lexical_scope -> @expr_stmt.lexical_scope
}

;; Expressions used for variable declarations
@var_decl [VariableDeclarationStatement
    value: [VariableDeclarationValue @expr [Expression]]
] {
  edge @expr.lexical_scope -> @var_decl.lexical_scope
}

;; Expressions used for tuple deconstruction statements
@tuple_decon [TupleDeconstructionStatement
    @expr expression: [Expression]
] {
  edge @expr.lexical_scope -> @tuple_decon.lexical_scope
}

;; Expressions as if conditions
@stmt [Statement [IfStatement @condition condition: [Expression]]] {
  edge @condition.lexical_scope -> @stmt.lexical_scope
}

;; Expressions in return statements
@stmt [Statement [ReturnStatement @expr [Expression]]] {
  edge @expr.lexical_scope -> @stmt.lexical_scope
}

;; Expressions in for iterations
@stmt [Statement [ForStatement
    @iter_expr iterator: [Expression]
]] {
  edge @iter_expr.lexical_scope -> @stmt.lexical_scope
  edge @iter_expr.lexical_scope -> @stmt.init_defs
}

;; Expressions in while conditions
@stmt [Statement [WhileStatement
    @condition condition: [Expression]
]] {
  edge @condition.lexical_scope -> @stmt.lexical_scope
}

;; Expressions in do-while conditions
@stmt [Statement [DoWhileStatement
    @condition condition: [Expression]
]] {
  edge @condition.lexical_scope -> @stmt.lexical_scope
}

;; Expressions used for state variable declarations
@state_var [StateVariableDefinition
    value: [StateVariableDefinitionValue @expr [Expression]]
] {
  edge @expr.lexical_scope -> @state_var.lexical_scope
}

;; Tuple expressions
@tuple_expr [Expression [TupleExpression
    items: [TupleValues [TupleValue @expr [Expression]]]
]] {
  edge @expr.lexical_scope -> @tuple_expr.lexical_scope
}


;;; Identifier expressions

@expr [Expression @name variant: [Identifier]] {
  node ref
  attr (ref) node_reference = @name

  edge ref -> @expr.lexical_scope
  edge @expr.output -> ref
}


;;; Member access expressions

;; TODO: implement variant for `.address` member
@expr [Expression [MemberAccessExpression
    @operand operand: [Expression]
    @name member: [Identifier]
]] {
  node ref
  attr (ref) node_reference = @name

  node member
  attr (member) push_symbol = "."

  edge ref -> member
  edge member -> @operand.output

  edge @expr.output -> ref
}

;; TODO: implement `.output` link for other expression variants


;;; Function call expressions

@args [ArgumentsDeclaration] {
  node @args.lexical_scope
}

@args [ArgumentsDeclaration [PositionalArgumentsDeclaration
    [PositionalArguments @argument [Expression]]
]] {
  edge @argument.lexical_scope -> @args.lexical_scope
}

@named_arg [NamedArgument @name [Identifier] [Colon] @value [Expression]] {
  node @named_arg.lexical_scope

  edge @value.lexical_scope -> @named_arg.lexical_scope

  node ref
  attr (ref) node_reference = @name
  ;; TODO: bind the named argument to the parameters definition of the function
  ;; (is this possible given that function references are expressions?)
}

@args [ArgumentsDeclaration [NamedArgumentsDeclaration
    [NamedArgumentGroup [NamedArguments @argument [NamedArgument]]]
]] {
  edge @argument.lexical_scope -> @args.lexical_scope
}

@funcall [Expression [FunctionCallExpression @args [ArgumentsDeclaration]]] {
  edge @args.lexical_scope -> @funcall.lexical_scope
}


;;; Type expressions

@type_expr [Expression [TypeExpression @type [TypeName]]] {
  edge @type.type_ref -> @type_expr.lexical_scope
}



"#####;
