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

;; Keeps a link to the enclosing contract definition to provide a parent for
;; method calls (to correctly resolve virtual methods)
inherit .enclosing_def

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

  let @source_unit.enclosing_def = #null
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
  edge @source_unit.lexical_scope -> @unit_member.def
  edge @source_unit.defs -> @unit_member.def
}

@source_unit [SourceUnit [SourceUnitMembers [SourceUnitMember @using [UsingDirective]]]] {
  let @using.lexical_scope = @source_unit.lexical_scope
  edge @source_unit.lexical_scope -> @using.def
  ; FIXME: if the using directive is global, its def should also be linked to @source_unit.defs
}

;; ... and imports
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

;; Contracts need access to the parent scope to resolve bases. This is purely
;; for convenience, as contracts can only appear in SourceUnits so we could
;; potentially connect this directly when connecting to the base contract
;; identifiers (but that would make the query longer)
@source_unit [SourceUnit [SourceUnitMembers
    [SourceUnitMember @contract [ContractDefinition]]
]] {
  let @contract.parent_scope = @source_unit.lexical_scope
}


;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
;;; Imports
;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;

[ImportClause [_ @path path: [StringLiteral]]] {
  ;; This node represents the imported file and the @path.import node is used by
  ;; all subsequent import rules
  node @path.import
  scan (source-text @path) {
    "^\\s*[\"'](.+)[\"']\\s*$" {
      let resolved_path = (resolve-path FILE_PATH $1)
      attr (@path.import) push_symbol = resolved_path
    }
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
  attr (def) tag = "alias"  ; deprioritize this definition
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
  attr (def) tag = "alias"  ; deprioritize this definition
  edge @symbol.def -> def

  node import
  attr (import) node_reference = @name
  edge def -> import

  edge import -> @symbol.import
}


;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
;;; Contracts
;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;

@contract [ContractDefinition] {
  node @contract.lexical_scope
  node @contract.super_scope
  node @contract.def
  node @contract.members
  node @contract.type_members
  node @contract.modifiers

  edge @contract.lexical_scope -> @contract.members
  edge @contract.lexical_scope -> @contract.type_members
  edge @contract.lexical_scope -> @contract.modifiers

  let @contract.enclosing_def = @contract.def
}

@contract [ContractDefinition @name name: [Identifier]] {
  attr (@contract.def) node_definition = @name
  attr (@contract.def) definiens_node = @contract

  ;; "instance" like access path
  ;; we have two distinct paths: @typeof -> . for accesses to variables of the contract's type
  ;; and () -> . for accesses through a `new` invocation
  node member
  attr (member) pop_symbol = "."
  edge member -> @contract.members

  node type_def
  attr (type_def) pop_symbol = "@typeof"
  edge @contract.def -> type_def
  edge type_def -> member

  node call
  attr (call) pop_symbol = "()"
  edge @contract.def -> call
  edge call -> member

  ;; "namespace" like access path
  node type_member
  attr (type_member) pop_symbol = "."
  edge @contract.def -> type_member
  edge type_member -> @contract.type_members

  ;; Define "super" effectively as if it was a state variable of a type connected by our super_scope
  ;; super_scope will later connect to the base contract defs directly
  node super
  attr (super) pop_symbol = "super"

  node super_typeof
  attr (super_typeof) push_symbol = "@typeof"

  edge super -> super_typeof
  edge super_typeof -> @contract.super_scope

  ;; Finally make "super" available in the contract's lexical scope for function bodies to use
  edge @contract.lexical_scope -> super

  ;; This defines the sink of edges added from base contracts when setting this
  ;; contract as the compilation context
  attr (@contract.def) export_node = @contract.members

  ;; This node will eventually connect to the contract's members being compiled
  ;; and grants access to definitions in that contract and all its parents
  ;; (recursively)
  node super_import
  attr (super_import) pop_symbol = "."
  edge super -> super_import

  ;; This defines the source side of edges added to base contracts when setting
  ;; a contract as compilation context; this allows this contract (a base) to
  ;; access virtual methods in any sub-contract defined in the hierarchy
  attr (@contract.def) import_nodes = [@contract.lexical_scope, super_import]
}

@contract [ContractDefinition [InheritanceSpecifier [InheritanceTypes
    [InheritanceType @type_name [IdentifierPath]]
]]] {
  ;; Resolve contract bases names through the parent scope of the contract (aka
  ;; the source unit)
  edge @type_name.push_end -> @contract.parent_scope

  ;; Make base members accesible as our own members
  node member
  attr (member) push_symbol = "."

  node typeof
  attr (typeof) push_symbol = "@typeof"

  edge @contract.members -> member
  edge member -> typeof
  edge typeof -> @type_name.push_begin

  ;; Make base contract defs (eg. enums and structs) accessible as our own
  node type_member
  attr (type_member) push_symbol = "."

  edge @contract.type_members -> type_member
  edge type_member -> @type_name.push_begin

  ;; The base contract defs are directly accesible through our special super scope
  edge @contract.super_scope -> @type_name.push_begin
}

@parent [InheritanceType @type_name [IdentifierPath]] {
  let @parent.ref = @type_name.push_begin
}

;; NOTE: we use anchors here to prevent the query engine from returning all the
;; sublists of possible parents
@contract [ContractDefinition [InheritanceSpecifier
    [InheritanceTypes . @parents [_]+ .]
]] {
  var parent_refs = []
  for parent in @parents {
    if (eq (node-type parent) "InheritanceType") {
      ;; this is intentionally reversed because of how Solidity linearised the contract bases
      set parent_refs = (concat [parent.ref] parent_refs)
    }
  }
  attr (@contract.def) parents = parent_refs
}

@contract [ContractDefinition [ContractMembers
    [ContractMember @member (
          [EnumDefinition]
        | [StructDefinition]
        | [EventDefinition]
        | [ErrorDefinition]
        | [UserDefinedValueTypeDefinition]
        | [FunctionDefinition]
        | [StateVariableDefinition]
        | [ModifierDefinition]
        | [FallbackFunctionDefinition]
        | [ReceiveFunctionDefinition]
    )]
]] {
  edge @member.lexical_scope -> @contract.lexical_scope
}

@contract [ContractDefinition [ContractMembers
    [ContractMember @using [UsingDirective]]
]] {
  let @using.lexical_scope = @contract.lexical_scope
  edge @contract.lexical_scope -> @using.def
}

@contract [ContractDefinition [ContractMembers
    [ContractMember @member (
          [EnumDefinition]
        | [StructDefinition]
        | [EventDefinition]
        | [ErrorDefinition]
        | [UserDefinedValueTypeDefinition]
    )]
]] {
  edge @contract.type_members -> @member.def
}

@contract [ContractDefinition [ContractMembers
    [ContractMember @member (
          [FunctionDefinition]
        | [StateVariableDefinition]
        | [ModifierDefinition]
    )]
]] {
  edge @contract.lexical_scope -> @member.def
}

@contract [ContractDefinition members: [ContractMembers
    item: [ContractMember @function variant: [FunctionDefinition]]
]] {
  ;; Contract functions are also accessible for an instance of the contract
  edge @contract.members -> @function.def

  ;; This may prioritize this definition (when there are multiple options)
  ;; according to the C3 linerisation ordering
  attr (@function.def) tag = "c3"
  attr (@function.def) parents = [@contract.def]
}

@contract [ContractDefinition members: [ContractMembers
    item: [ContractMember @modifier variant: [ModifierDefinition]]
]] {
  edge @contract.modifiers -> @modifier.def
}

@contract [ContractDefinition [ContractMembers [ContractMember
    [FunctionDefinition [FunctionAttributes [FunctionAttribute
        [OverrideSpecifier [OverridePathsDeclaration [OverridePaths
            @base_ident [IdentifierPath]
        ]]]
    ]]]
]]] {
  ;; Resolve overriden bases when listed in the function modifiers
  edge @base_ident.push_end -> @contract.parent_scope
}


;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
;;; Interfaces
;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;

@interface [InterfaceDefinition] {
  node @interface.lexical_scope
  node @interface.def
  node @interface.members
  node @interface.type_members

  edge @interface.lexical_scope -> @interface.members
  edge @interface.lexical_scope -> @interface.type_members
}

@interface [InterfaceDefinition @name name: [Identifier]] {
  node def
  attr (def) node_definition = @name
  attr (def) definiens_node = @interface

  edge @interface.def -> def

  ;; "instance" like access path
  node type_def
  attr (type_def) pop_symbol = "@typeof"
  node member
  attr (member) pop_symbol = "."
  edge def -> type_def
  edge type_def -> member
  edge member -> @interface.members

  ;; "namespace" like access path
  node type_member
  attr (type_member) pop_symbol = "."
  edge def -> type_member
  edge type_member -> @interface.type_members
}

@interface [InterfaceDefinition [InterfaceMembers
    [ContractMember @member (
          [EnumDefinition]
        | [StructDefinition]
        | [EventDefinition]
        | [ErrorDefinition]
        | [UserDefinedValueTypeDefinition]
    )]
]] {
  edge @member.lexical_scope -> @interface.lexical_scope
  edge @interface.type_members -> @member.def
}

@interface [InterfaceDefinition members: [InterfaceMembers
    item: [ContractMember @function variant: [FunctionDefinition]]
]] {
  edge @function.lexical_scope -> @interface.lexical_scope
  edge @interface.members -> @function.def
}

[InterfaceDefinition [InterfaceMembers [ContractMember @using [UsingDirective]]]] {
  ; using directives are not allowed in interfaces, but the grammar allows them
  ; so we need to create an artificial node here to connect to created edges from
  ; the internal nodes
  let @using.lexical_scope = (node)
}


;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
;;; Libraries
;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;

@library [LibraryDefinition] {
  node @library.lexical_scope
  node @library.def
  node @library.members

  edge @library.lexical_scope -> @library.members
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

@library [LibraryDefinition [LibraryMembers
    [ContractMember @member (
          [FunctionDefinition]
        | [EnumDefinition]
        | [StructDefinition]
        | [EventDefinition]
        | [ErrorDefinition]
        | [UserDefinedValueTypeDefinition]
    )]
]] {
  edge @member.lexical_scope -> @library.lexical_scope
  edge @library.members -> @member.def
}

@library [LibraryDefinition [LibraryMembers
    [ContractMember @using [UsingDirective]]
]] {
  let @using.lexical_scope = @library.lexical_scope
  edge @library.lexical_scope -> @using.def
}


;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
;;; Using directives
;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;

@using [UsingDirective] {
  ; this node acts as a definition in the sense that provides an entry point
  ; that pops the target type and pushes the library/functions to attach to the
  ; target type
  node @using.def

  ; this node connects the other end of the popping path starting at .def and
  ; resolves for the library/functions in the directive
  node @using.clause
}

@using [UsingDirective [UsingClause @id_path [IdentifierPath]]] {
  ; resolve the library to be used in the directive
  edge @id_path.push_end -> @using.lexical_scope

  ; because we're using the whole library, we don't need to "consume" the
  ; attached function (as when using the deconstruction syntax), but we still
  ; need to verify that we're only using this path when resolving a function
  ; access to the target type, not the target type itself
  node dot_guard_pop
  attr (dot_guard_pop) pop_symbol = "."
  node dot_guard_push
  attr (dot_guard_push) push_symbol = "."

  edge @using.clause -> dot_guard_pop
  edge dot_guard_pop -> dot_guard_push
  edge dot_guard_push -> @id_path.push_begin
}

@using [UsingDirective [UsingClause [UsingDeconstruction
    [UsingDeconstructionSymbols [UsingDeconstructionSymbol
        @id_path [IdentifierPath]
    ]]
]]] {
  ; resolve the function to be used in the directive
  edge @id_path.push_end -> @using.lexical_scope

  node dot
  attr (dot) pop_symbol = "."
  node last_identifier
  attr (last_identifier) pop_symbol = (source-text @id_path.rightmost_identifier)

  edge @using.clause -> dot
  edge dot -> last_identifier
  edge last_identifier -> @id_path.push_begin
}

@using [UsingDirective [UsingTarget @type_name [TypeName]]] {
  ; resolve the target type of the directive
  edge @type_name.type_ref -> @using.lexical_scope
}

@using [UsingDirective [UsingTarget [TypeName @id_path [IdentifierPath]]]] {
  node typeof
  attr (typeof) pop_symbol = "@typeof"

  edge @using.def -> @id_path.pop_begin
  edge @id_path.pop_end -> typeof
  edge typeof -> @using.clause
}

; FIXME: handle array target types


;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
;;; Type names
;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;

;; TypeName nodes should define two scoped variables:
;;
;; - @type_name.type_ref represents the node in the graph where we're ready to
;;   resolve the type, and thus should generally be connected to a (lexical)
;;   scope node (outgoing node).
;;
;; - @type_name.output represents the other end of the type and corresponds to a
;;   state where the type has already been resolved so we can, for example
;;   resolve its members (incoming node).

@type_name [TypeName @elementary [ElementaryType]] {
  let @type_name.type_ref = @elementary.ref
  let @type_name.output = @elementary.ref
}

@type_name [TypeName @id_path [IdentifierPath]] {
  ;; For an identifier path used as a type, the left-most element is the one
  ;; that connects to the parent lexical scope, because the name resolution
  ;; starts at the left of the identifier.
  let @type_name.type_ref = @id_path.push_end

  ;; Conversely, the complete type is found at the right-most name, and that's
  ;; where users of this type should link to (eg. a variable declaration).
  let @type_name.output = @id_path.push_begin
}

@type_name [TypeName @mapping [MappingType]] {
  let @type_name.type_ref = @mapping.lexical_scope
  let @type_name.output = @mapping.output
}

@type_name [TypeName @array [ArrayTypeName]] {
  let @type_name.type_ref = @array.lexical_scope
  let @type_name.output = @array.output
}

@type_name [TypeName @ftype [FunctionType]] {
  let @type_name.type_ref = @ftype.lexical_scope
  let @type_name.output = @ftype.output
}


;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
;;; Elementary types
;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;

@elementary [ElementaryType] {
  node @elementary.ref
  attr (@elementary.ref) type = "push_symbol"
  attr (@elementary.ref) source_node = @elementary, symbol = @elementary.symbol
}

@elementary [ElementaryType variant: [AddressType @address [AddressKeyword]]] {
  let @elementary.symbol = (format "@builtInType@{}" (source-text @address))
}

@elementary [ElementaryType @keyword (
      [BoolKeyword]
    | [ByteKeyword]
    | [BytesKeyword]
    | [StringKeyword]
    | [IntKeyword]
    | [UintKeyword]
    | [FixedKeyword]
    | [UfixedKeyword]
)] {
  let @elementary.symbol = (format "@builtInType@{}" (source-text @keyword))
}


;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
;;; Mappings
;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;

@mapping [MappingType] {
  node @mapping.lexical_scope
  node @mapping.output
}

@mapping [MappingType [MappingKey [MappingKeyType @key_ident [IdentifierPath]]]] {
  edge @key_ident.push_end -> @mapping.lexical_scope
}

@mapping [MappingType [MappingValue @value_type [TypeName]]] {
  edge @value_type.type_ref -> @mapping.lexical_scope

  node typeof_input
  attr (typeof_input) pop_symbol = "@typeof"

  node index
  attr (index) pop_symbol = "[]"

  node typeof_output
  attr (typeof_output) push_symbol = "@typeof"

  ;; The mapping's type exposes the `[]` operator that returns the value type
  edge @mapping.output -> typeof_input
  edge typeof_input -> index
  edge index -> typeof_output
  edge typeof_output -> @value_type.output
}


;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
;;; Arrays types
;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;

@array [ArrayTypeName] {
  node @array.lexical_scope
  node @array.output
}

@array [ArrayTypeName @type_name [TypeName]] {
  edge @type_name.type_ref -> @array.lexical_scope

  node typeof_input
  attr (typeof_input) pop_symbol = "@typeof"

  node index
  attr (index) pop_symbol = "[]"

  node typeof_output
  attr (typeof_output) push_symbol = "@typeof"

  ;; The array type exposes the `[]` operator that returns the value type
  edge @array.output -> typeof_input
  edge typeof_input -> index
  edge index -> typeof_output
  edge typeof_output -> @type_name.output
}

@array [ArrayTypeName @size index: [Expression]] {
  edge @size.lexical_scope -> @array.lexical_scope
}


;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
;;; Function types
;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;

@ftype [FunctionType] {
  node @ftype.lexical_scope
  node @ftype.output
}

@ftype [FunctionType @params [ParametersDeclaration]] {
  edge @params.lexical_scope -> @ftype.lexical_scope
}

@ftype [FunctionType [ReturnsDeclaration @return_params [ParametersDeclaration]]] {
  edge @return_params.lexical_scope -> @ftype.lexical_scope
}



;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
;;; Identifier Paths (aka. references to custom types)
;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;

;; The identifier path builds two graph paths:
;;
;; - From right to left, pushing the identifiers and acting as a "reference".
;;   This path begins at @id_path.push_begin and ends at @id_path.push_end.
;;
;; - From left to right, popping the identifiers (used as a definition sink in
;;   using directives). This path begins at @id_path.pop_begin and ends at
;;   @id_path.pop_end.
;;
;; Additionally the IdentifierPath defines another scoped variable
;; @id_path.rightmost_identifier which corresponds to the identifier in the last
;; position in the path, from left to right. Useful for the using directive to
;; be able to pop the name of the attached function.

[IdentifierPath @name [Identifier]] {
  node @name.ref
  attr (@name.ref) node_reference = @name

  node @name.pop
  attr (@name.pop) pop_symbol = (source-text @name)
}

@id_path [IdentifierPath @name [Identifier] .] {
  let @id_path.rightmost_identifier = @name

  let @id_path.push_begin = @name.ref
  let @id_path.pop_end = @name.pop
}

[IdentifierPath @left_name [Identifier] . [Period] . @right_name [Identifier]] {
  node ref_member
  attr (ref_member) push_symbol = "."

  edge @right_name.ref -> ref_member
  edge ref_member -> @left_name.ref

  node pop_member
  attr (pop_member) pop_symbol = "."

  edge @left_name.pop -> pop_member
  edge pop_member -> @right_name.pop
}

@id_path [IdentifierPath . @name [Identifier]] {
  let @id_path.push_end = @name.ref
  let @id_path.pop_begin = @name.pop
}


;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
;;; Function, parameter declarations and modifiers
;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;

@param [Parameter @type_name [TypeName]] {
  node @param.lexical_scope
  node @param.def

  edge @type_name.type_ref -> @param.lexical_scope

  node @param.typeof
  attr (@param.typeof) push_symbol = "@typeof"
  edge @param.typeof -> @type_name.output
}

@param [Parameter @name [Identifier]] {
  attr (@param.def) node_definition = @name
  attr (@param.def) definiens_node = @param

  edge @param.def -> @param.typeof
}

@params [ParametersDeclaration] {
  node @params.lexical_scope
  node @params.defs

  ;; This scope can be used to resolve named argument calls
  node @params.names
  attr (@params.names) pop_symbol = "@param_names"
  edge @params.names -> @params.defs
}

@params [ParametersDeclaration [Parameters @param item: [Parameter]]] {
  edge @param.lexical_scope -> @params.lexical_scope
  edge @params.defs -> @param.def
}

@function [FunctionDefinition] {
  node @function.lexical_scope
  node @function.def
}

@function [FunctionDefinition name: [FunctionName @name [Identifier]]] {
  attr (@function.def) node_definition = @name
  attr (@function.def) definiens_node = @function
}

@function [FunctionDefinition @params parameters: [ParametersDeclaration]] {
  edge @params.lexical_scope -> @function.lexical_scope

  ;; Input parameters are available in the function scope
  edge @function.lexical_scope -> @params.defs
  ;; ... and shadow other declarations
  attr (@function.lexical_scope -> @params.defs) precedence = 1

  ;; Connect to paramaters for named argument resolution
  edge @function.def -> @params.names
}

@function [FunctionDefinition returns: [ReturnsDeclaration
    @return_params [ParametersDeclaration]
]] {
  edge @return_params.lexical_scope -> @function.lexical_scope

  ;; Return parameters are available in the function scope
  edge @function.lexical_scope -> @return_params.defs
  ;; ... and shadow other declarations
  attr (@function.lexical_scope -> @return_params.defs) precedence = 1
}

;; Only functions that return a single value have an actual return type
;; since tuples are not actual types in Solidity
@function [FunctionDefinition returns: [ReturnsDeclaration
    [ParametersDeclaration [Parameters . @param [Parameter] .]]
]] {
  node call
  attr (call) pop_symbol = "()"

  edge @function.def -> call
  edge call -> @param.typeof
}

;; Connect the function body's block lexical scope to the function
@function [FunctionDefinition [FunctionBody @block [Block]]] {
  edge @block.lexical_scope -> @function.lexical_scope
}

@function [FunctionDefinition [FunctionAttributes item: [FunctionAttribute
    @modifier [ModifierInvocation]
]]] {
  edge @modifier.lexical_scope -> @function.lexical_scope
}

@modifier [ModifierInvocation @name [IdentifierPath]] {
  node @modifier.lexical_scope

  edge @name.push_end -> @modifier.lexical_scope
}

@modifier [ModifierInvocation @args [ArgumentsDeclaration]] {
  edge @args.lexical_scope -> @modifier.lexical_scope
}


;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
;;; Fallback and receive functions
;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;

@fallback [FallbackFunctionDefinition] {
  node @fallback.lexical_scope
}

@fallback [FallbackFunctionDefinition @params parameters: [ParametersDeclaration]] {
  edge @params.lexical_scope -> @fallback.lexical_scope

  ;; Input parameters are available in the fallback function scope
  edge @fallback.lexical_scope -> @params.defs
  attr (@fallback.lexical_scope -> @params.defs) precedence = 1
}

@fallback [FallbackFunctionDefinition returns: [ReturnsDeclaration
    @return_params [ParametersDeclaration]
]] {
  edge @return_params.lexical_scope -> @fallback.lexical_scope

  ;; Return parameters are available in the fallback function scope
  edge @fallback.lexical_scope -> @return_params.defs
  attr (@fallback.lexical_scope -> @return_params.defs) precedence = 1
}

@fallback [FallbackFunctionDefinition [FunctionBody @block [Block]]] {
  edge @block.lexical_scope -> @fallback.lexical_scope
}

@fallback [FallbackFunctionDefinition [FallbackFunctionAttributes
    item: [FallbackFunctionAttribute @modifier [ModifierInvocation]]
]] {
  edge @modifier.lexical_scope -> @fallback.lexical_scope
}

@receive [ReceiveFunctionDefinition] {
  node @receive.lexical_scope
}

@receive [ReceiveFunctionDefinition [FunctionBody @block [Block]]] {
  edge @block.lexical_scope -> @receive.lexical_scope
}

@receive [ReceiveFunctionDefinition [ReceiveFunctionAttributes
    item: [ReceiveFunctionAttribute @modifier [ModifierInvocation]]
]] {
  edge @modifier.lexical_scope -> @receive.lexical_scope
}


;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
;;; Function modifiers
;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;

@modifier [ModifierDefinition
    @name name: [Identifier]
    body: [FunctionBody @body [Block]]
] {
  node @modifier.def
  node @modifier.lexical_scope

  node def
  attr (def) node_definition = @name
  attr (def) definiens_node = @modifier

  edge @modifier.def -> def
  edge @body.lexical_scope -> @modifier.lexical_scope
}

@modifier [ModifierDefinition @params [ParametersDeclaration]] {
  edge @params.lexical_scope -> @modifier.lexical_scope

  ;; Input parameters are available in the modifier scope
  edge @modifier.lexical_scope -> @params.defs
  attr (@modifier.lexical_scope -> @params.defs) precedence = 1
}


;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
;;; Blocks and generic statements
;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;

@block [Block] {
  node @block.lexical_scope
  node @block.defs
}

;; The first statement in a block
@block [Block [Statements . @stmt [Statement]]] {
  if (version-matches ">= 0.5.0") {
    edge @stmt.lexical_scope -> @block.lexical_scope
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
    ;; ... shadowing declarations in enclosing scopes
    attr (@block.lexical_scope -> @stmt.defs) precedence = 1
  }
}

;; Two consecutive statements
[Statements @left_stmt [Statement] . @right_stmt [Statement]] {
  if (version-matches ">= 0.5.0") {
    edge @right_stmt.lexical_scope -> @left_stmt.lexical_scope
  }
}

@stmt [Statement] {
  node @stmt.lexical_scope
  node @stmt.defs

  if (version-matches ">= 0.5.0") {
    ;; For Solidity >= 0.5.0, definitions are immediately available in the
    ;; statement scope. For < 0.5.0 this is also true, but resolved through the
    ;; enclosing block's lexical scope.
    edge @stmt.lexical_scope -> @stmt.defs
    ;; Statement definitions shadow other declarations in its scope
    attr (@stmt.lexical_scope -> @stmt.defs) precedence = 1
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


;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
;;; Expressions & declaration statements
;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;

;; In general for statements the structure is [Statement [StmtVariant]] and we
;; will define the scoped nodes .lexical_scope and (possibly) .defs in the
;; Statement CST node, skipping scoped nodes in the variant of the statement.
;;
;; For expression statements, variable and tuple declarations we define them
;; separately from the enclosing statement to be able to use them in `for`
;; initialization and condition clauses directly. Also, because we intend to
;; reuse them, all of them must have both a .lexical_scope and .defs scoped
;; nodes (even though .defs doesn't make sense for ExpressionStatement)

@stmt [Statement @expr_stmt [ExpressionStatement]] {
  edge @expr_stmt.lexical_scope -> @stmt.lexical_scope
}

@expr_stmt [ExpressionStatement] {
  node @expr_stmt.lexical_scope
  node @expr_stmt.defs
}

@expr_stmt [ExpressionStatement @expr [Expression]] {
  edge @expr.lexical_scope -> @expr_stmt.lexical_scope
}


;;; Variable declaration statements

@stmt [Statement @var_decl [VariableDeclarationStatement]] {
  edge @var_decl.lexical_scope -> @stmt.lexical_scope
  edge @stmt.defs -> @var_decl.defs
}

@var_decl [VariableDeclarationStatement] {
  node @var_decl.lexical_scope
  node @var_decl.defs
}

@var_decl [VariableDeclarationStatement
    value: [VariableDeclarationValue @expr [Expression]]
] {
  edge @expr.lexical_scope -> @var_decl.lexical_scope
}

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


;;; Tuple deconstruction statements

@stmt [Statement @tuple_decon [TupleDeconstructionStatement]] {
  edge @tuple_decon.lexical_scope -> @stmt.lexical_scope
  edge @stmt.defs -> @tuple_decon.defs
}

@tuple_decon [TupleDeconstructionStatement] {
  node @tuple_decon.lexical_scope
  node @tuple_decon.defs
}

@tuple_decon [TupleDeconstructionStatement
    @expr expression: [Expression]
] {
  edge @expr.lexical_scope -> @tuple_decon.lexical_scope
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

@stmt [Statement [IfStatement @condition condition: [Expression]]] {
  edge @condition.lexical_scope -> @stmt.lexical_scope
}

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

@stmt [Statement [ForStatement @iter_expr iterator: [Expression]]] {
  edge @iter_expr.lexical_scope -> @stmt.lexical_scope
  edge @iter_expr.lexical_scope -> @stmt.init_defs
}

@stmt [Statement [ForStatement @body body: [Statement]]] {
  node @stmt.init_defs

  edge @body.lexical_scope -> @stmt.lexical_scope
  edge @body.lexical_scope -> @stmt.init_defs
  if (version-matches "< 0.5.0") {
    edge @stmt.defs -> @body.defs
    edge @stmt.defs -> @stmt.init_defs
  }
}

;; While loops

@stmt [Statement [WhileStatement @condition condition: [Expression]]] {
  edge @condition.lexical_scope -> @stmt.lexical_scope
}

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

@stmt [Statement [DoWhileStatement @condition condition: [Expression]]] {
  edge @condition.lexical_scope -> @stmt.lexical_scope
}


;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
;;; Error handling
;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;

;;; Try-catch statements

@stmt [Statement [TryStatement @expr expression: [Expression]]] {
  edge @expr.lexical_scope -> @stmt.lexical_scope
}

@stmt [Statement [TryStatement @body body: [Block]]] {
  edge @body.lexical_scope -> @stmt.lexical_scope
}

@stmt [Statement [TryStatement
    [ReturnsDeclaration @return_params [ParametersDeclaration]]
    @body body: [Block]
]] {
  edge @return_params.lexical_scope -> @stmt.lexical_scope
  edge @body.lexical_scope -> @return_params.defs
  ;; Similar to functions, return params shadow other declarations
  attr (@body.lexical_scope -> @return_params.defs) precedence = 1
}

@stmt [Statement [TryStatement [CatchClauses [CatchClause
    @body body: [Block]
]]]] {
  edge @body.lexical_scope -> @stmt.lexical_scope
}

@stmt [Statement [TryStatement [CatchClauses [CatchClause
    [CatchClauseError @catch_params parameters: [ParametersDeclaration]]
    @body body: [Block]
]]]] {
  edge @catch_params.lexical_scope -> @stmt.lexical_scope
  edge @body.lexical_scope -> @catch_params.defs
  ;; Similar to functions, catch params shadow other declarations
  attr (@body.lexical_scope -> @catch_params.defs) precedence = 1
}


;;; Revert statements

@stmt [Statement [RevertStatement @error_ident [IdentifierPath]]] {
  edge @error_ident.push_end -> @stmt.lexical_scope
}

@stmt [Statement [RevertStatement @args [ArgumentsDeclaration]]] {
  edge @args.lexical_scope -> @stmt.lexical_scope
}

[Statement [RevertStatement
    @error_ident [IdentifierPath]
    @args [ArgumentsDeclaration]
]] {
  edge @args.refs -> @error_ident.push_begin
}


;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
;;; Other statements
;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;

;;; Return
@stmt [Statement [ReturnStatement @expr [Expression]]] {
  edge @expr.lexical_scope -> @stmt.lexical_scope
}

;;; Emit
@stmt [Statement [EmitStatement
    @event_ident [IdentifierPath]
    @args [ArgumentsDeclaration]
]] {
  edge @event_ident.push_end -> @stmt.lexical_scope
  edge @args.lexical_scope -> @stmt.lexical_scope
  edge @args.refs -> @event_ident.push_begin
}

;;; Unchecked
@stmt [Statement [UncheckedBlock @block block: [Block]]] {
  edge @block.lexical_scope -> @stmt.lexical_scope
}

;;; Assembly
@stmt [Statement [AssemblyStatement @body body: [YulBlock]]] {
  edge @body.lexical_scope -> @stmt.lexical_scope
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

@state_var [StateVariableDefinition
    value: [StateVariableDefinitionValue @expr [Expression]]
] {
  edge @expr.lexical_scope -> @state_var.lexical_scope
}


;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
;;; Enum definitions
;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;

@enum [EnumDefinition] {
  node @enum.lexical_scope
  node @enum.def
  node @enum.members
}

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


;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
;;; Structure definitions
;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;

@struct [StructDefinition] {
  node @struct.lexical_scope
  node @struct.def
  node @struct.members
}

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


;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
;;; Event definitions
;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;

@event [EventDefinition @name name: [Identifier]] {
  node @event.lexical_scope
  node @event.def

  attr (@event.def) node_definition = @name
  attr (@event.def) definiens_node = @event

  node @event.params
  attr (@event.params) pop_symbol = "@param_names"
  edge @event.def -> @event.params
}

@event [EventDefinition [EventParametersDeclaration [EventParameters
    [EventParameter @type_name type_name: [TypeName]]
]]] {
  edge @type_name.type_ref -> @event.lexical_scope
}

@event [EventDefinition [EventParametersDeclaration [EventParameters
    @param [EventParameter
        @name name: [Identifier]
    ]
]]] {
  node def
  attr (def) node_definition = @name
  attr (def) definiens_node = @param

  edge @event.params -> def
}


;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
;;; Error definitions
;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;

@error [ErrorDefinition @name name: [Identifier]] {
  node @error.lexical_scope
  node @error.def

  attr (@error.def) node_definition = @name
  attr (@error.def) definiens_node = @error

  node @error.params
  attr (@error.params) pop_symbol = "@param_names"
  edge @error.def -> @error.params
}

@error [ErrorDefinition [ErrorParametersDeclaration [ErrorParameters
    [ErrorParameter @type_name type_name: [TypeName]]
]]] {
    edge @type_name.type_ref -> @error.lexical_scope
}

@error [ErrorDefinition [ErrorParametersDeclaration [ErrorParameters
    @param [ErrorParameter
        @name name: [Identifier]
    ]
]]] {
  node def
  attr (def) node_definition = @name
  attr (def) definiens_node = @param

  edge @error.params -> def
}


;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
;;; Other named definitions
;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;

@constant [ConstantDefinition] {
  node @constant.lexical_scope
  node @constant.def
}

@constant [ConstantDefinition
    @type_name type_name: [TypeName]
    @name name: [Identifier]
    @value value: [Expression]
] {
  node def
  attr (def) node_definition = @name
  attr (def) definiens_node = @constant

  edge @constant.def -> def

  edge @value.lexical_scope -> @constant.lexical_scope
  edge @type_name.type_ref -> @constant.lexical_scope
}

@user_type [UserDefinedValueTypeDefinition] {
  node @user_type.lexical_scope
  node @user_type.def
}

@user_type [UserDefinedValueTypeDefinition @name [Identifier]] {
  node def
  attr (def) node_definition = @name
  attr (def) definiens_node = @user_type

  edge @user_type.def -> def
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

;; Tuple expressions
@tuple_expr [Expression [TupleExpression
    items: [TupleValues [TupleValue @expr [Expression]]]
]] {
  edge @expr.lexical_scope -> @tuple_expr.lexical_scope
}

;; Identifier expressions
@expr [Expression @name variant: [Identifier]] {
  node ref
  attr (ref) node_reference = @name
  attr (ref) parents = [@expr.enclosing_def]

  edge ref -> @expr.lexical_scope
  edge @expr.output -> ref
}

;; Member access expressions
@expr [Expression [MemberAccessExpression
    @operand operand: [Expression]
    @name member: [Identifier]
]] {
  node @name.ref
  attr (@name.ref) node_reference = @name
  attr (@name.ref) parents = [@expr.enclosing_def]

  node member
  attr (member) push_symbol = "."

  edge @name.ref -> member
  edge member -> @operand.output

  edge @expr.output -> @name.ref
}

;; Special case: member accesses to `super` are tagged with "super" to rank
;; virtual methods correctly
[MemberAccessExpression
    operand: [Expression ["super"]]
    @name member: [Identifier]
] {
  attr (@name.ref) tag = "super"
}

;; Index access expressions
@expr [Expression [IndexAccessExpression
    @operand operand: [Expression]
]] {
  node index
  attr (index) push_symbol = "[]"

  edge @expr.output -> index
  edge index -> @operand.output
}

;; Type expressions
@type_expr [Expression [TypeExpression @type [TypeName]]] {
  edge @type.type_ref -> @type_expr.lexical_scope
}

;; New expressions

@new_expr [Expression [NewExpression @type [TypeName]]] {
  edge @type.type_ref -> @new_expr.lexical_scope
  edge @new_expr.output -> @type.output
}


;;; Function call expressions

@args [ArgumentsDeclaration] {
  node @args.lexical_scope

  node @args.refs
  attr (@args.refs) push_symbol = "@param_names"
}

@args [ArgumentsDeclaration [PositionalArgumentsDeclaration
    [PositionalArguments @argument [Expression]]
]] {
  edge @argument.lexical_scope -> @args.lexical_scope
}

@named_arg [NamedArgument @name [Identifier] [Colon] @value [Expression]] {
  node @named_arg.lexical_scope

  edge @value.lexical_scope -> @named_arg.lexical_scope

  node @named_arg.ref
  attr (@named_arg.ref) node_reference = @name
}

@args [ArgumentsDeclaration [NamedArgumentsDeclaration
    [NamedArgumentGroup [NamedArguments @argument [NamedArgument]]]
]] {
  edge @argument.lexical_scope -> @args.lexical_scope
  edge @argument.ref -> @args.refs
}

@funcall [Expression [FunctionCallExpression
    @operand [Expression]
    @args [ArgumentsDeclaration]
]] {
  edge @args.lexical_scope -> @funcall.lexical_scope

  ;; Connect to the output of the function name to be able to resolve named arguments
  edge @args.refs -> @operand.output

  node call
  attr (call) push_symbol = "()"

  edge @funcall.output -> call
  edge call -> @operand.output
}


;;; Call options

@expr [Expression [CallOptionsExpression options: [CallOptions @named_arg [NamedArgument]]]] {
  edge @named_arg.lexical_scope -> @expr.lexical_scope
}


;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
;;; Yul
;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;

;;; Blocks and statements

@block [YulBlock] {
  node @block.lexical_scope
  ; Variables defined in this block (only used to forward the init block
  ; declarations in a for statement)
  node @block.variable_defs
  ; Function definitions accessible from the block (ie. defined in the block, or
  ; accessible in the enclosing parent block)
  node @block.function_defs

  edge @block.lexical_scope -> @block.function_defs
}

@block [YulBlock [YulStatements . @stmt [YulStatement]]] {
  edge @stmt.lexical_scope -> @block.lexical_scope
}

@block [YulBlock [YulStatements @stmt [YulStatement]]] {
  edge @stmt.function_scope -> @block.function_defs
  edge @block.variable_defs -> @stmt.defs
}

[YulStatements @left_stmt [YulStatement] . @right_stmt [YulStatement]] {
  edge @right_stmt.lexical_scope -> @left_stmt.lexical_scope
  ; variable declaration are accessible from the next statement
  edge @right_stmt.lexical_scope -> @left_stmt.defs
}

@stmt [YulStatement] {
  node @stmt.lexical_scope
  node @stmt.defs
  ;; Functions visible in this scope (to propagate to inner function
  ;; definitions, since the lexical scope is not accessible inside a function
  ;; body)
  node @stmt.function_scope
}

;;; Blocks as statements

@stmt [YulStatement @block variant: [YulBlock]] {
  edge @block.lexical_scope -> @stmt.lexical_scope
  edge @block.function_defs -> @stmt.function_scope
}

;;; Expression as statements

@stmt [YulStatement @expr_stmt [YulExpression]] {
  edge @expr_stmt.lexical_scope -> @stmt.lexical_scope
}

;;; Variable declarations

@stmt [YulStatement @var_decl [YulVariableDeclarationStatement]] {
  edge @var_decl.lexical_scope -> @stmt.lexical_scope
  edge @stmt.defs -> @var_decl.defs
}

@var_decl [YulVariableDeclarationStatement] {
  node @var_decl.lexical_scope
  node @var_decl.defs
}

@var_decl [YulVariableDeclarationStatement [YulVariableNames @name [YulIdentifier]]] {
  node def
  attr (def) node_definition = @name
  attr (def) definiens_node = @var_decl

  edge @var_decl.defs -> def
}

@var_decl [YulVariableDeclarationStatement [YulVariableDeclarationValue
    @value [YulExpression]
]] {
  edge @value.lexical_scope -> @var_decl.lexical_scope
}

;;; Variable assignments

@stmt [YulStatement @var_assign [YulVariableAssignmentStatement]] {
  edge @var_assign.lexical_scope -> @stmt.lexical_scope
}

@var_assign [YulVariableAssignmentStatement] {
  node @var_assign.lexical_scope
}

@var_assign [YulVariableAssignmentStatement [YulPaths @path [YulPath]]] {
  edge @path.lexical_scope -> @var_assign.lexical_scope
}

@var_assign [YulVariableAssignmentStatement @expr expression: [YulExpression]] {
  edge @expr.lexical_scope -> @var_assign.lexical_scope
}

;;; Function definitions

@block [YulBlock [YulStatements [YulStatement @fundef [YulFunctionDefinition]]]] {
  ;; Function definitions are hoisted in the enclosing block
  edge @block.function_defs -> @fundef.def
  ;; The only definitions available in the function's lexical scope (other than
  ;; parameters) are functions (ie. the body of the function doesn't have access
  ;; to any outside variables)
  edge @fundef.lexical_scope -> @block.function_defs
}

@fundef [YulFunctionDefinition
    @name name: [YulIdentifier]
    @body body: [YulBlock]
] {
  node @fundef.lexical_scope
  node @fundef.def

  node def
  attr (def) node_definition = @name
  attr (def) definiens_node = @fundef

  edge @fundef.def -> def
  edge @body.lexical_scope -> @fundef.lexical_scope
}

@fundef [YulFunctionDefinition [YulParametersDeclaration [YulParameters
    @param [YulIdentifier]
]]] {
  node def
  attr (def) node_definition = @param
  attr (def) definiens_node = @param

  edge @fundef.lexical_scope -> def
}

@fundef [YulFunctionDefinition [YulReturnsDeclaration [YulVariableNames
    @return_param [YulIdentifier]
]]] {
  node def
  attr (def) node_definition = @return_param
  attr (def) definiens_node = @return_param

  edge @fundef.lexical_scope -> def
}

;;; Stack assignment (Solidity < 0.5.0)

@stmt [YulStatement [YulStackAssignmentStatement @name [YulIdentifier]]] {
  node ref
  attr (ref) node_reference = @name

  edge ref -> @stmt.lexical_scope
}

;;; If statements

@stmt [YulStatement [YulIfStatement
    @condition condition: [YulExpression]
    @body body: [YulBlock]
]] {
  edge @condition.lexical_scope -> @stmt.lexical_scope
  edge @body.lexical_scope -> @stmt.lexical_scope
  edge @body.function_defs -> @stmt.function_scope
}

;;; Switch statements

@stmt [YulStatement [YulSwitchStatement
    @expr expression: [YulExpression]
]] {
  edge @expr.lexical_scope -> @stmt.lexical_scope
}

@stmt [YulStatement [YulSwitchStatement [YulSwitchCases [YulSwitchCase
    [_ @body body: [YulBlock]]
]]]] {
  edge @body.lexical_scope -> @stmt.lexical_scope
  edge @body.function_defs -> @stmt.function_scope
}

;;; For statements

@stmt [YulStatement [YulForStatement
    @init initialization: [YulBlock]
    @cond condition: [YulExpression]
    @iter iterator: [YulBlock]
    @body body: [YulBlock]
]] {
  edge @init.lexical_scope -> @stmt.lexical_scope
  edge @cond.lexical_scope -> @stmt.lexical_scope
  edge @iter.lexical_scope -> @stmt.lexical_scope
  edge @body.lexical_scope -> @stmt.lexical_scope

  edge @cond.lexical_scope -> @init.variable_defs
  edge @iter.lexical_scope -> @init.variable_defs
  edge @body.lexical_scope -> @init.variable_defs
}

;;; Label statements (Solidity < 0.5.0)

@block [YulBlock [YulStatements [YulStatement @label [YulLabel @name label: [YulIdentifier]]]]] {
  node def
  attr (def) node_definition = @name
  attr (def) definiens_node = @label

  ; Labels are hoisted to the beginning of the block
  edge @block.lexical_scope -> def
}

;;; Expressions

@expr [YulExpression] {
  node @expr.lexical_scope
}

@expr [YulExpression @path [YulPath]] {
  edge @path.lexical_scope -> @expr.lexical_scope
}

@path [YulPath . [YulPathComponent @name [YulIdentifier]]] {
  node @path.lexical_scope

  node ref
  attr (ref) node_reference = @name

  edge ref -> @path.lexical_scope
}

@expr [YulExpression @funcall [YulFunctionCallExpression]] {
  edge @funcall.lexical_scope -> @expr.lexical_scope
}

@funcall [YulFunctionCallExpression
  @operand operand: [YulExpression]
  @args arguments: [YulArguments]
] {
  node @funcall.lexical_scope

  edge @operand.lexical_scope -> @funcall.lexical_scope
  edge @args.lexical_scope -> @funcall.lexical_scope
}

@args [YulArguments] {
  node @args.lexical_scope
}

@args [YulArguments @arg [YulExpression]] {
  edge @arg.lexical_scope -> @args.lexical_scope
}

"#####;
