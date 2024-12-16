// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

#[allow(clippy::needless_raw_string_hashes)]
#[allow(dead_code)] // TODO(#982): use to create the graph
pub const BINDING_RULES_SOURCE: &str = r#####"
    global ROOT_NODE
global FILE_PATH
global JUMP_TO_SCOPE_NODE

attribute node_definition = node     => type = "pop_symbol", node_symbol = node, is_definition
attribute node_reference = node      => type = "push_symbol", node_symbol = node, is_reference
attribute node_symbol = node         => symbol = (source-text node), source_node = node
attribute pop_symbol = symbol        => type = "pop_symbol", symbol = symbol
attribute push_symbol = symbol       => type = "push_symbol", symbol = symbol
attribute symbol_definition = symbol => type = "pop_symbol", symbol = symbol, is_definition
attribute symbol_reference = symbol  => type = "push_symbol", symbol = symbol, is_reference

attribute scoped_node_definition = node => type = "pop_scoped_symbol", node_symbol = node, is_definition
attribute scoped_node_reference  = node => type = "push_scoped_symbol", node_symbol = node, is_reference
attribute pop_scoped_symbol = symbol    => type = "pop_scoped_symbol", symbol = symbol
attribute push_scoped_symbol = symbol   => type = "push_scoped_symbol", symbol = symbol

;; Keeps a link to the enclosing contract definition to provide a parent for
;; method calls (to correctly resolve virtual methods)
inherit .enclosing_def

inherit .parent_scope
inherit .lexical_scope
inherit .extended_scope

; Used to resolve extension methods for `using for *` directives
; This is used as a minor optimization to avoid introducing new possible paths
; when there are no `using for *` directives in the contract.
inherit .star_extension


;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
;;; Source unit (aka .sol file)
;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;

@source_unit [SourceUnit] {
  ;; All lexical_scope nodes eventually connect to the file's root scope
  node @source_unit.lexical_scope

  ;; This provides all the exported symbols from the file
  node @source_unit.defs

  ;; Connect to ROOT_NODE to export our symbols
  node export
  edge ROOT_NODE -> export
  edge export -> @source_unit.defs

  if (is-system-file FILE_PATH) {
    ; If this is a system file (aka. built-ins), export everything through this
    ; special symbol (which is automatically imported below)
    attr (export) pop_symbol = "@@built-ins@@"

  } else {
    ; This is a user file, so we want to export under the file's path symbol
    attr (export) pop_symbol = FILE_PATH

    ; ... and also import the global built-ins
    node built_ins
    attr (built_ins) push_symbol = "@@built-ins@@"

    edge @source_unit.lexical_scope -> built_ins
    edge built_ins -> ROOT_NODE
  }

  let @source_unit.enclosing_def = #null

  ;; This defines a parent_scope at the source unit level (this attribute is
  ;; inherited) for contracts to resolve bases (both in inheritance lists and
  ;; override specifiers)
  let @source_unit.parent_scope = @source_unit.lexical_scope
  ; FIXME: we probably need to make extended scope different than lexical scope
  ; and push an extension scope on that path
  let @source_unit.extended_scope = @source_unit.lexical_scope

  ; We may jump to scope here to resolve using the extensions scope provided by
  ; contract/libraries that contain `using` directives
  edge @source_unit.lexical_scope -> JUMP_TO_SCOPE_NODE

  ; Provide a default star extension sink node that gets inherited. This is
  ; connected to from expressions, and those can potentially happen anywhere.
  node @source_unit.star_extension
}

;; Top-level definitions...
@source_unit [SourceUnit [SourceUnitMembers
    [SourceUnitMember @unit_member (
          [ContractDefinition]
        | [LibraryDefinition]
        | [InterfaceDefinition]
        | [StructDefinition]
        | [EnumDefinition]
        | [FunctionDefinition]
        | [ConstantDefinition]
        | [ErrorDefinition]
        | [UserDefinedValueTypeDefinition]
        | [EventDefinition]
    )]
]] {
  edge @source_unit.lexical_scope -> @unit_member.def
  edge @source_unit.defs -> @unit_member.def

  ; In the general case, the lexical scope of the definition connects directly
  ; to the source unit's
  edge @unit_member.lexical_scope -> @source_unit.lexical_scope
}

;; Definitions that need to resolve expressions do it through an extended_scope
@source_unit [SourceUnit [SourceUnitMembers
    [SourceUnitMember @unit_member (
          [FunctionDefinition]
        | [ConstantDefinition]
    )]
]] {
  edge @unit_member.extended_scope -> @source_unit.extended_scope
}


;; Special case for built-ins: we want to export all symbols in the contract:
;; functions, types and state variables. All built-in symbols are defined in an
;; internal contract named '%BuiltIns%' (renamed from '$BuiltIns$') so we need
;; to export all its members and type members directly as a source unit
;; definition.
;; __SLANG_SOLIDITY_BUILT_INS_CONTRACT_NAME__ keep in sync with built-ins generation.
@source_unit [SourceUnit [SourceUnitMembers
    [SourceUnitMember @contract [ContractDefinition name: ["%BuiltIns%"]]]
]] {
  if (is-system-file FILE_PATH) {
    edge @source_unit.defs -> @contract.instance
  }
}

@source_unit [SourceUnit [SourceUnitMembers [SourceUnitMember @using [UsingDirective]]]] {
  ; TODO: this is the hook for top-level extensions, but this should connect to
  ; an extensions scope that gets pushed to the scope stack, as in the case of
  ; contracts/libraries (defined further down below).
  edge @source_unit.lexical_scope -> @using.def
}

@source_unit [SourceUnit [SourceUnitMembers [SourceUnitMember
    @using [UsingDirective [GlobalKeyword]]
]]] {
  ; global using directives are exported by this source unit
  edge @source_unit.defs -> @using.def
}

;; Import connections to the source unit
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


;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
;;; Imports
;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;

[ImportClause
  [_
    path: [StringLiteral
      @path ([DoubleQuotedStringLiteral] | [SingleQuotedStringLiteral])
    ]
  ]
] {
  ;; This node represents the imported file and the @path.import node is used by
  ;; all subsequent import rules
  node @path.import

  let resolved_path = (resolve-path FILE_PATH @path)
  attr (@path.import) push_symbol = resolved_path

  edge @path.import -> ROOT_NODE
}

;;; `import <URI>`
@import [PathImport
  path: [StringLiteral
    @path ([DoubleQuotedStringLiteral] | [SingleQuotedStringLiteral])
  ]
] {
  ;; This is the "lexical" connection, which makes all symbols exported from the
  ;; imported source unit available for resolution globally at this' source unit
  ;; scope
  edge @import.defs -> @path.import
}

;;; `import <URI> as <IDENT>`
@import [PathImport
  path: [StringLiteral
    @path ([DoubleQuotedStringLiteral] | [SingleQuotedStringLiteral])
  ]
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
  path: [StringLiteral
    @path ([DoubleQuotedStringLiteral] | [SingleQuotedStringLiteral])
  ]
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
  path: [StringLiteral
    @path ([DoubleQuotedStringLiteral] | [SingleQuotedStringLiteral])
  ]
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
;;; Common inheritance rules (apply to contracts and interfaces)
;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;

@specifier [InheritanceSpecifier [InheritanceTypes
    [InheritanceType @type_name [IdentifierPath]]
]] {
  ;; This should point to the enclosing contract or interface definition
  let heir = @specifier.heir

  ;; Resolve base names through the parent scope of our heir (contract or
  ;; interface), aka the source unit
  edge @type_name.push_end -> heir.parent_scope

  ; Access instance members of the inherited contract/interface, from the
  ; instance scope of the inheriting contract/interface
  node instance
  attr (instance) push_symbol = "@instance"
  edge heir.instance -> instance
  edge instance -> @type_name.push_begin

  ; Base members can also be accessed (from the instance scope) qualified with
  ; the base name (eg. `Base.something`)
  node member_pop
  attr (member_pop) pop_symbol = "."
  edge heir.instance -> @type_name.pop_begin
  edge @type_name.pop_end -> member_pop
  edge member_pop -> instance

  ; Base namespace-like members (ie. enums, structs, etc) are also accessible as
  ; our own namespace members
  node ns_member
  attr (ns_member) push_symbol = "."
  edge heir.ns -> ns_member
  edge ns_member -> @type_name.push_begin

  if (version-matches "< 0.7.0") {
    ; `using` directives are inherited in Solidity < 0.7.0, so connect them to
    ; our own extensions scope
    node extensions_push_guard
    attr (extensions_push_guard) push_symbol = "@extensions"
    edge heir.extensions -> extensions_push_guard
    edge extensions_push_guard -> @type_name.push_begin
  }
}

;; The next couple of rules setup a `.parent_refs` attribute to use in the
;; resolution algorithm to perform linearisation of a contract hierarchy.

;; NOTE: we use anchors here to prevent the query engine from returning all the
;; sublists of possible parents
@specifier [InheritanceSpecifier [InheritanceTypes . @parents [_]+ .]] {
  var parent_refs = []
  for parent in @parents {
    if (eq (node-type parent) "InheritanceType") {
      ;; this is intentionally reversed because of how Solidity linearised the contract bases
      set parent_refs = (concat [parent.ref] parent_refs)
    }
  }
  let @specifier.parent_refs = parent_refs
}

@parent [InheritanceType @type_name [IdentifierPath]] {
  let @parent.ref = @type_name.push_begin
}


;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
;;; Contracts
;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;

@contract [ContractDefinition @name name: [Identifier]] {
  node @contract.lexical_scope
  node @contract.extended_scope
  node @contract.extensions
  node @contract.def
  node @contract.members
  node @contract.ns
  node @contract.modifiers
  node @contract.instance

  attr (@contract.def) node_definition = @name
  attr (@contract.def) definiens_node = @contract

  edge @contract.lexical_scope -> @contract.instance

  ; Instance scope can also see members and our namespace definitions
  edge @contract.instance -> @contract.members
  edge @contract.instance -> @contract.ns

  let @contract.enclosing_def = @contract.def

  ;; External "instance" scope access: either member access through a variable
  ;; of the contract's type, or through calling (which happens on `new`
  ;; invocations or casting). These should access only externally accessible
  ;; members, such as functions and public variables.
  node member
  attr (member) pop_symbol = "."
  edge member -> @contract.instance

  node type_def
  attr (type_def) pop_symbol = "@typeof"
  edge @contract.def -> type_def
  edge type_def -> member

  node call
  attr (call) pop_symbol = "()"
  edge @contract.def -> call
  edge call -> member

  ;; "namespace" scope access
  node ns_member
  attr (ns_member) pop_symbol = "."
  edge @contract.def -> ns_member
  edge ns_member -> @contract.ns

  ; Finally there's an @instance guarded path used by derived contracts to
  ; access instance accessible members
  node instance
  attr (instance) pop_symbol = "@instance"
  edge @contract.def -> instance
  edge instance -> @contract.instance

  ; "this" keyword is available in our lexical scope and can access any
  ; externally available member
  node this
  attr (this) pop_symbol = "this"
  edge @contract.lexical_scope -> this
  edge this -> member

  ;; Modifiers are available as a contract type members through a special '@modifier' guard
  node modifier
  attr (modifier) pop_symbol = "@modifier"
  edge @contract.ns -> modifier
  edge modifier -> @contract.modifiers

  ; There may be attached functions to our type. For the general case of
  ; variables of our type, that's already handled via normal lexical scope
  ; resolution. But for casting/`new` invocations that we resolve through the
  ; `()` guard above, we need to explicitly jump to the extension scope from
  ; here to attempt resolving the attached function. We cannot jump back to the
  ; parent scope because that would create a cycle in the graph.
  node push_typeof
  attr (push_typeof) push_symbol = "@typeof"
  node push_name
  attr (push_name) push_symbol = (source-text @name)
  edge call -> push_typeof
  edge push_typeof -> push_name
  edge push_name -> JUMP_TO_SCOPE_NODE

  if (version-matches "< 0.5.0") {
    ; For Solidity < 0.5.0 `this` also acts like an `address`
    node address_ref
    attr (address_ref) push_symbol = "%address"
    node address_typeof
    attr (address_typeof) push_symbol = "@typeof"
    edge this -> address_typeof
    edge address_typeof -> address_ref
    edge address_ref -> @contract.lexical_scope
  }

  ; This is the connection point to resolve attached functions by `using for *`
  node @contract.star_extension
  attr (@contract.star_extension) push_symbol = "@*"

  if (version-matches "< 0.7.0") {
    ; Expose extensions through an `@extensions` guard on Solidity < 0.7.0 so
    ; that they can be accessed from sub contract extension scopes
    node extensions_guard
    attr (extensions_guard) pop_symbol = "@extensions"
    edge @contract.def -> extensions_guard
    edge extensions_guard -> @contract.extensions

    ; Since using directives are inherited, we need to *always* connect the push
    ; extensions to the extended scope, regardless of whether this contract
    ; contains any `using` directive.
    edge @contract.extended_scope -> @contract.push_extensions

    ; For Solidity < 0.7.0 using directives are inherited, so we need to connect
    ; always For newer versions, this connection only happens when there is a
    ; `using for *` directive in the contract (see rule below)
    edge @contract.star_extension -> @contract.extended_scope
  }

  ; Path to resolve the built-in type for type() expressions
  node type
  attr (type) pop_symbol = "@type"
  node type_contract_type
  attr (type_contract_type) push_symbol = "%ContractTypeType"
  edge @contract.def -> type
  edge type -> type_contract_type
  edge type_contract_type -> @contract.parent_scope

  ; The following defines the connection nodes the resolution algorithm uses
  ; *only when setting a compilation context/target*.

  ; This attribute defines the sink of edges added from base contracts when
  ; setting this contract as the compilation context, and should provide access
  ; to anything that can be reached through `super`. The instance scope is a bit
  ; too broad, but `.members` is too narrow as it doesn't allow navigation to
  ; parent contracts (and from the base we need to be able to reach all
  ; contracts in the hierarchy).
  attr (@contract.def) export_node = @contract.instance

  ; This node will eventually connect to the contract's members being compiled
  ; and grants access to definitions in that contract and all its parents
  ; (recursively). It only makes sense if `super` is defined (ie. if we have
  ; parents), but we define it here to be able to use it in the declaration of
  ; import nodes. This is the dual of the export_node above.
  node @contract.super_import
  attr (@contract.super_import) pop_symbol = "."

  ; This defines the source side of edges added to base contracts when setting
  ; a contract as compilation context; this allows this contract (a base) to
  ; access virtual methods in any sub-contract defined in the hierarchy (both
  ; with and without `super`, hence the two connection points).
  attr (@contract.def) import_nodes = [@contract.lexical_scope, @contract.super_import]
}

@contract [ContractDefinition @specifier [InheritanceSpecifier]] {
  ; The `.heir` scoped variable allows the rules for `InheritanceSpecifier`
  ; above to connect the instance scope of this contract to the parents.
  let @specifier.heir = @contract
  attr (@contract.def) parents = @specifier.parent_refs

  ; The rest of these statements deal with defining and connecting the `super`
  ; keyword path.

  ; `super_scope` is where we hook all references to our parent contracts
  node @contract.super_scope

  ; Define "super" in the lexical scope
  node @contract.super
  attr (@contract.super) pop_symbol = "super"
  edge @contract.lexical_scope -> @contract.super

  ; This connects `super` to exported scopes from all contracts in the hierarchy
  ; when setting a contract compilation target (see more detailed description
  ; above on the definition of the `super_import` node).
  edge @contract.super -> @contract.super_import

  ; Then connect it through an `@instance` guard to the parent contracts through
  ; `super_scope`. This allows "instance"-like access to members of parents
  ; through `super`.
  node super_instance
  attr (super_instance) push_symbol = "@instance"
  edge @contract.super_import -> super_instance
  edge super_instance -> @contract.super_scope
}

@contract [ContractDefinition [InheritanceSpecifier [InheritanceTypes
    [InheritanceType @type_name [IdentifierPath]]
]]] {
  ;; The base contract defs are directly accesible through our super scope
  edge @contract.super_scope -> @type_name.push_begin
}

; Pure definitions that cannot contain expressions
@contract [ContractDefinition [ContractMembers
    [ContractMember @member (
          [EnumDefinition]
        | [StructDefinition]
        | [EventDefinition]
        | [ErrorDefinition]
        | [UserDefinedValueTypeDefinition]
    )]
]] {
  edge @member.lexical_scope -> @contract.lexical_scope
}

; Definitions that can contain expressions need two scopes:
; - normal lexical scope for resolving types
; - extended scope (extended by using directives) for resolving expressions
@contract [ContractDefinition [ContractMembers
    [ContractMember @member (
          [FunctionDefinition]
        | [ConstructorDefinition]
        | [ModifierDefinition]
        | [FallbackFunctionDefinition]
        | [ReceiveFunctionDefinition]
        | [UnnamedFunctionDefinition]
        | [StateVariableDefinition]
    )]
]] {
  edge @member.lexical_scope -> @contract.lexical_scope
  edge @member.extended_scope -> @contract.extended_scope
}

@contract [ContractDefinition [ContractMembers
    [ContractMember @using [UsingDirective]]
]] {
  ; Hook the using definition in the extensions scope
  edge @contract.extensions -> @using.def

  ; Connect the extensions push path (this can happen multiple times if there
  ; are multiple `using` directives in the contract, but that's allowed by the
  ; graph builder).
  edge @contract.extended_scope -> @contract.push_extensions
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
  ; These definition go into the "namespace" scope and are accessible externally
  ; via qualified naming (eg. `Contract.MyStruct`)
  edge @contract.ns -> @member.def
}

@contract [ContractDefinition [ContractMembers
    [ContractMember @state_var [StateVariableDefinition]]
]] {
  ; State variables are available to derived contracts.
  ; TODO: this also exposes private state variables to derived contracts, but we
  ; can't easily filter them because we don't have negative assertions in our
  ; query language (we would need to modify this query for anything *not*
  ; containing a `PrivateKeyword` node)
  edge @contract.instance -> @state_var.def
}

;; Public state variables are also exposed as external member functions
@contract [ContractDefinition [ContractMembers
    [ContractMember @state_var [StateVariableDefinition
        [StateVariableAttributes [StateVariableAttribute [PublicKeyword]]]
    ]]
]] {
  edge @contract.members -> @state_var.def
}

@contract [ContractDefinition [ContractMembers
    [ContractMember @function [FunctionDefinition]]
]] {
  ;; Contract functions are also accessible for an instance of the contract
  edge @contract.members -> @function.def

  ;; This may prioritize this definition (when there are multiple options)
  ;; according to the C3 linerisation ordering
  attr (@function.def) tag = "c3"
  attr (@function.def) parents = [@contract.def]
}

@contract [ContractDefinition [ContractMembers
    [ContractMember @function [FunctionDefinition
        [FunctionAttributes [FunctionAttribute ([ExternalKeyword] | [PublicKeyword])]]
    ]]
]] {
  ; Public or external functions are also accessible through the contract type
  ; (to retrieve their `.selector` for example)
  edge @contract.ns -> @function.def
}

@contract [ContractDefinition members: [ContractMembers
    [ContractMember @modifier [ModifierDefinition]]
]] {
  ; Modifiers live in their own special scope
  edge @contract.modifiers -> @modifier.def

  ;; This may prioritize this definition (when there are multiple options)
  ;; according to the C3 linerisation ordering
  attr (@modifier.def) tag = "c3"
  attr (@modifier.def) parents = [@contract.def]
}

@contract [ContractDefinition [ContractMembers [ContractMember
    [UsingDirective [UsingTarget [Asterisk]]]
]]] {
  ; Connect the star extension node to the resolution extended scope if there is
  ; a `using for *` directive in the contract
  edge @contract.star_extension -> @contract.extended_scope
}

; This applies to both state variables and function definitions
@override [OverrideSpecifier [OverridePathsDeclaration [OverridePaths
    @base_ident [IdentifierPath]
]]] {
  ;; Resolve overriden bases when listed in the function or modifiers modifiers
  edge @base_ident.push_end -> @override.parent_scope
}


;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
;;; Interfaces
;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;

@interface [InterfaceDefinition @name name: [Identifier]] {
  node @interface.lexical_scope
  node @interface.def
  node @interface.members
  node @interface.ns
  node @interface.instance

  attr (@interface.def) node_definition = @name
  attr (@interface.def) definiens_node = @interface

  edge @interface.lexical_scope -> @interface.instance

  ; Interfaces don't contain expressions (or `using` directives), so the
  ; extended scope is the same as the lexical scope
  let @interface.extended_scope = @interface.lexical_scope
  ; The extensions node is required for the inheritance rules, but not used in interfaces
  let @interface.extensions = (node)

  edge @interface.instance -> @interface.members
  edge @interface.instance -> @interface.ns

  ;; External "instance" like access path, to access members of a variable of
  ;; the interface's type or through a casting call.
  node member
  attr (member) pop_symbol = "."
  edge member -> @interface.instance

  node typeof
  attr (typeof) pop_symbol = "@typeof"
  edge @interface.def -> typeof
  edge typeof -> member

  node call
  attr (call) pop_symbol = "()"
  edge @interface.def -> call
  edge call -> member

  ; From a call we may need to resolve using the extensions scope, in case there's
  ; a `using` directive on our type. This path ends up jumping to scope just to
  ; handle that case.
  node push_typeof
  attr (push_typeof) push_symbol = "@typeof"
  node push_name
  attr (push_name) push_symbol = (source-text @name)
  edge call -> push_typeof
  edge push_typeof -> push_name
  edge push_name -> JUMP_TO_SCOPE_NODE

  ;; "namespace" like access path
  node ns_member
  attr (ns_member) pop_symbol = "."
  edge @interface.def -> ns_member
  edge ns_member -> @interface.ns

  ; Finally there's guarded `@instance` path used by derived contracts to access
  ; instance accessible members
  node instance
  attr (instance) pop_symbol = "@instance"
  edge @interface.def -> instance
  edge instance -> @interface.instance

  ; Path to resolve the built-in type for type() expressions
  node type
  attr (type) pop_symbol = "@type"
  node type_interface_type
  attr (type_interface_type) push_symbol = "%InterfaceTypeType"
  edge @interface.def -> type
  edge type -> type_interface_type
  edge type_interface_type -> @interface.parent_scope
}

@interface [InterfaceDefinition @specifier [InheritanceSpecifier]] {
  let @specifier.heir = @interface
  attr (@interface.def) parents = @specifier.parent_refs
}

@interface [InterfaceDefinition [InterfaceMembers
    [ContractMember @member (
          [EnumDefinition]
        | [FunctionDefinition]
        | [StructDefinition]
        | [EventDefinition]
        | [ErrorDefinition]
        | [UserDefinedValueTypeDefinition]
    )]
]] {
  edge @member.lexical_scope -> @interface.lexical_scope
  edge @interface.ns -> @member.def
}

;; Allow references (eg. variables of the interface type) to the interface to
;; access functions
@interface [InterfaceDefinition members: [InterfaceMembers
    item: [ContractMember @function variant: [FunctionDefinition]]
]] {
  edge @interface.members -> @function.def
  edge @function.extended_scope -> @interface.extended_scope
}

[InterfaceDefinition [InterfaceMembers [ContractMember @using [UsingDirective]]]] {
  ; using directives are not allowed in interfaces, but the grammar allows them
  ; so we need to create an artificial node here to connect to created edges from
  ; the instance nodes
  let @using.lexical_scope = (node)
}


;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
;;; Libraries
;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;

@library [LibraryDefinition @name name: [Identifier]] {
  node @library.lexical_scope
  node @library.extended_scope
  node @library.extensions
  node @library.def
  node @library.ns
  node @library.modifiers

  attr (@library.def) node_definition = @name
  attr (@library.def) definiens_node = @library

  edge @library.lexical_scope -> @library.ns

  node member
  attr (member) pop_symbol = "."
  edge @library.def -> member
  edge member -> @library.ns

  ; Access to modifiers is guarded by a @modifier symbol
  node modifier
  attr (modifier) pop_symbol = "@modifier"
  edge @library.ns -> modifier
  edge modifier -> @library.modifiers

  ; Path to resolve the built-in type for type() expressions (same as contracts)
  node type
  attr (type) pop_symbol = "@type"
  node type_library_type
  attr (type_library_type) push_symbol = "%ContractTypeType"
  edge @library.def -> type
  edge type -> type_library_type
  edge type_library_type -> @library.lexical_scope

  ; This is the connection point to resolve attached functions by `using for *`
  node @library.star_extension
  attr (@library.star_extension) push_symbol = "@*"
}

@library [LibraryDefinition [LibraryMembers
    [ContractMember @member (
          [EnumDefinition]
        | [StructDefinition]
        | [EventDefinition]
        | [ErrorDefinition]
        | [UserDefinedValueTypeDefinition]
    )]
]] {
  edge @member.lexical_scope -> @library.lexical_scope
  edge @library.ns -> @member.def
}

@library [LibraryDefinition [LibraryMembers
    [ContractMember @member (
          [FunctionDefinition]
        | [StateVariableDefinition [StateVariableAttributes [StateVariableAttribute [ConstantKeyword]]]]
    )]
]] {
  edge @member.lexical_scope -> @library.lexical_scope
  edge @member.extended_scope -> @library.extended_scope
  edge @library.ns -> @member.def
}

@library [LibraryDefinition [LibraryMembers
    [ContractMember @modifier [ModifierDefinition]]
]] {
  edge @library.modifiers -> @modifier.def
  edge @modifier.lexical_scope -> @library.lexical_scope
  edge @modifier.extended_scope -> @library.extended_scope
}

@library [LibraryDefinition [LibraryMembers
    [ContractMember @using [UsingDirective]]
]] {
  ; Expose the using directive from the extensions scope
  edge @library.extensions -> @using.def

  ; Connect the extensions push path
  edge @library.extended_scope -> @library.push_extensions
}

@library [LibraryDefinition [LibraryMembers [ContractMember
    [UsingDirective [UsingTarget [Asterisk]]]
]]] {
  ; Connect the star extension node to the resolution extended scope if there is
  ; a `using for *` directive in the library
  edge @library.star_extension -> @library.extended_scope
}

;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
;;; Extensions scope rules
;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;

;; For contracts (and libraries) navigating to the source unit lexical scope
;; *also* needs to (optionally) propagate an extensions scope to be able to
;; correctly bind `using` attached functions.
@contract_or_library ([ContractDefinition] | [LibraryDefinition]) {
  ; The `.extended_scope` resolution scope used by function bodies and other
  ; expressions. From there we need to (optionally) push the extensions scope
  ; (ie. `using` directives definitions) to the scope stack.
  ; We will connect the path to push the extensions *if* the contract/library
  ; has a `using` directive. Also, the extended scope links to the lexical scope
  ; of the contract/library directly, regardless of whether there is a `using`
  ; directive or not.
  ; TODO: if we had a query negation operator to detect when there is no `using`
  ; directive, could we avoid connecting directly when there are extensions?
  edge @contract_or_library.extended_scope -> @contract_or_library.lexical_scope

  ; The .extensions node is where `using` directives will hook the definitions
  attr (@contract_or_library.extensions) is_exported

  ; Now we define the path to push the .extensions scope into the scope stack.
  ; We connect this to the extended scope only when there are extensions in the
  ; contract/library.
  node @contract_or_library.push_extensions
  attr (@contract_or_library.push_extensions) push_scoped_symbol = "@extend"
  attr (@contract_or_library.push_extensions) scope = @contract_or_library.extensions
  node drop_scopes
  attr (drop_scopes) type = "drop_scopes"
  node pop_extensions
  attr (pop_extensions) pop_scoped_symbol = "@extend"

  edge @contract_or_library.push_extensions -> drop_scopes
  edge drop_scopes -> pop_extensions
  edge pop_extensions -> @contract_or_library.lexical_scope
}


;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
;;; Using directives
;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;

;; The UsingDirective node requires the enclosing context to setup an
;; .extended_scope scoped variable for it to resolve both targets and subjects.
;; The resolution connects to the extended scope in order to (potentially) push
;; the same extension scope again, to resolve chained calls that all make use of
;; attached functions.

@using [UsingDirective] {
  ; This node acts as a definition in the sense that provides an entry point
  ; that pops the target type and pushes the library/functions to attach to the
  ; target type
  node @using.def

  ; This internal node connects the definition side of the clause to the target
  ; for resolution, and allows handling the multiple cases of `using` syntax
  ; easily
  node @using.clause
}

@using [UsingDirective [UsingClause @id_path [IdentifierPath]]] {
  ; resolve the library to be used in the directive
  edge @id_path.push_end -> @using.extended_scope

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
  edge @id_path.push_end -> @using.extended_scope

  node dot
  attr (dot) pop_symbol = "."
  node last_identifier
  attr (last_identifier) pop_symbol = (source-text @id_path.rightmost_identifier)

  edge @using.clause -> dot
  edge dot -> last_identifier
  edge last_identifier -> @id_path.push_begin
}

@using [UsingDirective [UsingTarget @type_name [TypeName]]] {
  ; pop the type symbols to connect to the attached function (via @using.clause)
  node typeof
  attr (typeof) pop_symbol = "@typeof"
  node cast
  attr (cast) pop_symbol = "()"

  ; We connect to all_pop_begin to be able to resolve both qualified and
  ; unqualified instances of the target type
  edge @using.def -> @type_name.all_pop_begin
  edge @type_name.pop_end -> typeof
  edge typeof -> @using.clause
  edge @type_name.pop_end -> cast
  edge cast -> @using.clause

  ; resolve the target type of the directive on the extended scope so the
  ; extension scope gets re-pushed
  edge @type_name.type_ref -> @using.extended_scope
}

[ContractMember @using [UsingDirective [UsingTarget [Asterisk]]]] {
  ; using X for * is only allowed inside contracts
  node star
  attr (star) pop_symbol = "@*"
  edge @using.def -> star
  edge star -> @using.clause
}


;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
;;; Type names
;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;

;; TypeName nodes should define these scoped variables:
;;
;; - @type_name.type_ref represents the node in the graph where we're ready to
;;   resolve the type, and thus should generally be connected to a (lexical)
;;   scope node (source node, outside edges connect *from* here).
;;
;; - @type_name.output represents the other end of the type and corresponds to a
;;   state where the type has already been resolved so we can, for example
;;   resolve its members (sink node, outside edges connect *to* here).
;;
;; - @type_name.pop_begin, @type_name.pop_end are used in a definition context,
;;   ie. when we need to pop the type name symbol(s) from the symbol stack.
;;   Additionally, @type_name.all_pop_begin links to each symbol in a typename
;;   (ie. in an identifier path typename), which allows referring to a type both
;;   qualified and unqualified.

@type_name [TypeName @elementary [ElementaryType]] {
  let @type_name.type_ref = @elementary.ref
  let @type_name.output = @elementary.ref
  let @type_name.pop_begin = @elementary.pop
  let @type_name.pop_end = @elementary.pop
  let @type_name.all_pop_begin = @elementary.pop
}

@type_name [TypeName @id_path [IdentifierPath]] {
  ;; For an identifier path used as a type, the left-most element is the one
  ;; that connects to the parent lexical scope, because the name resolution
  ;; starts at the left of the identifier.
  let @type_name.type_ref = @id_path.push_end

  ;; Conversely, the complete type is found at the right-most name, and that's
  ;; where users of this type should link to (eg. a variable declaration).
  let @type_name.output = @id_path.push_begin

  let @type_name.pop_begin = @id_path.pop_begin
  let @type_name.pop_end = @id_path.pop_end
  let @type_name.all_pop_begin = @id_path.all_pop_begin
}

@type_name [TypeName @type_variant ([ArrayTypeName] | [FunctionType])] {
  let @type_name.type_ref = @type_variant.lexical_scope
  let @type_name.output = @type_variant.output
  let @type_name.pop_begin = @type_variant.pop_begin
  let @type_name.pop_end = @type_variant.pop_end
  let @type_name.all_pop_begin = @type_variant.pop_begin
}

@type_name [TypeName @mapping [MappingType]] {
  let @type_name.type_ref = @mapping.lexical_scope
  let @type_name.output = @mapping.output
  let @type_name.pop_begin = @mapping.pop_begin
  let @type_name.pop_end = @mapping.pop_end
  let @type_name.all_pop_begin = @mapping.pop_begin
}


;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
;;; Elementary types
;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;

@elementary [ElementaryType] {
  node @elementary.ref
  attr (@elementary.ref) type = "push_symbol"
  attr (@elementary.ref) source_node = @elementary, symbol = @elementary.symbol

  node @elementary.pop
  attr (@elementary.pop) pop_symbol = @elementary.symbol

  ; These variables are a bit redundant, but necessary to easily use elementary
  ; types as mapping keys
  let @elementary.pop_begin = @elementary.pop
  let @elementary.pop_end = @elementary.pop
  let @elementary.all_pop_begin = @elementary.pop

  let @elementary.push_begin = @elementary.ref
  let @elementary.push_end = @elementary.ref
}

@elementary [ElementaryType [AddressType]] {
  let @elementary.symbol = "%address"
}

@elementary [ElementaryType [BoolKeyword]] {
  let @elementary.symbol = "%bool"
}

@elementary [ElementaryType [ByteKeyword]] {
  let @elementary.symbol = "%byte"
}

@elementary [ElementaryType @keyword [BytesKeyword]] {
  let @elementary.symbol = (format "%{}" (source-text @keyword))
}

@elementary [ElementaryType [StringKeyword]] {
  let @elementary.symbol = "%string"
}

@elementary [ElementaryType @keyword [IntKeyword]] {
  let symbol = (source-text @keyword)
  if (eq symbol "int") {
    let @elementary.symbol = "%int256"
  } else {
    let @elementary.symbol = (format "%{}" symbol)
  }
}

@elementary [ElementaryType @keyword [UintKeyword]] {
  let symbol = (source-text @keyword)
  if (eq symbol "uint") {
    let @elementary.symbol = "%uint256"
  } else {
    let @elementary.symbol = (format "%{}" symbol)
  }
}

@elementary [ElementaryType @keyword [FixedKeyword]] {
  let symbol = (source-text @keyword)
  if (eq symbol "fixed") {
    let @elementary.symbol = "%fixed128x18"
  } else {
    let @elementary.symbol = (format "%{}" symbol)
  }
}

@elementary [ElementaryType @keyword [UfixedKeyword]] {
  let symbol = (source-text @keyword)
  if (eq symbol "ufixed") {
    let @elementary.symbol = "%ufixed128x18"
  } else {
    let @elementary.symbol = (format "%{}" symbol)
  }
}


;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
;;; Mappings
;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;

@mapping [MappingType
    [MappingKey [MappingKeyType @key_type ([IdentifierPath] | [ElementaryType])]]
    [MappingValue @value_type [TypeName]]
] {
  node @mapping.lexical_scope
  node @mapping.output

  ; Define the pushing path of the mapping type
  ;   ValueType <- top of the symbol stack
  ;   KeyType
  ;   %mapping <- bottom of the symbol stack
  node mapping
  attr (mapping) push_symbol = "%Mapping"
  edge @mapping.output -> mapping
  edge mapping -> @key_type.push_begin
  edge @key_type.push_end -> @value_type.output

  ; Both key and value types need to be resolved
  edge @value_type.type_ref -> @mapping.lexical_scope
  edge @key_type.push_end -> @mapping.lexical_scope

  ; The mapping's type exposes the `[]` operator that returns the value type.

  node typeof_input
  attr (typeof_input) pop_symbol = "@typeof"
  edge @mapping.output -> typeof_input

  node typeof_output
  attr (typeof_output) push_symbol = "@typeof"
  edge typeof_output -> @value_type.output

  node index
  attr (index) pop_symbol = "[]"
  edge typeof_input -> index
  edge index -> typeof_output

  ; Special case for mapping public state variables: they can be called
  ; like a function with a key, and it's effectively the same as indexing it.
  node getter_call
  attr (getter_call) pop_symbol = "@as_getter"
  edge typeof_input -> getter_call
  edge getter_call -> typeof_output

  ; Now we define the "definition" route (aka. the pop route), to use in `using` directives only
  ; This is the reverse of the pushing path above (to the `.output` node)
  node pop_mapping
  attr (pop_mapping) pop_symbol = "%Mapping"

  let @mapping.pop_begin = @value_type.pop_begin
  edge @value_type.pop_end -> @key_type.pop_begin
  edge @key_type.pop_end -> pop_mapping
  let @mapping.pop_end = pop_mapping
}


;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
;;; Arrays types
;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;

@array [ArrayTypeName] {
  node @array.lexical_scope
  node @array.output
}

@array [ArrayTypeName [TypeName] index: [Expression]] {
  let @array.type_symbol = "%FixedArray"
}

@array [ArrayTypeName [OpenBracket] . [CloseBracket]] {
  let @array.type_symbol = "%Array"
}

@array [ArrayTypeName @type_name [TypeName]] {
  ; Define the pushing path of the array type
  ;   ValueType <- top of the symbol stack
  ;   %array / %arrayFixed <- bottom of the symbol stack
  node array
  attr (array) push_symbol = @array.type_symbol
  edge @array.output -> array
  edge array -> @type_name.output

  ; Resolve the value type itself
  edge @type_name.type_ref -> @array.lexical_scope
  ; And also the "type erased" array type so we can resolve built-in members
  edge array -> @array.lexical_scope

  ; Define the path to resolve index access (aka the `[]` operator)

  node typeof_input
  attr (typeof_input) pop_symbol = "@typeof"
  edge @array.output -> typeof_input

  node typeof_output
  attr (typeof_output) push_symbol = "@typeof"
  edge typeof_output -> @type_name.output

  node index
  attr (index) pop_symbol = "[]"
  edge typeof_input -> index
  edge index -> typeof_output

  ; Special case for public state variables of type array: they can be called
  ; like a function with an index, and it's effectively the same as indexing the
  ; array.
  node getter_call
  attr (getter_call) pop_symbol = "@as_getter"
  edge typeof_input -> getter_call
  edge getter_call -> typeof_output

  ; Define the special `.push()` built-in that returns the element type (for Solidity >= 0.6.0)
  if (version-matches ">= 0.6.0") {
    node built_in_member
    attr (built_in_member) pop_symbol = "."
    node push_built_in
    attr (push_built_in) pop_symbol = "push"
    node built_in_call
    attr (built_in_call) pop_symbol = "()"

    edge typeof_input -> built_in_member
    edge built_in_member -> push_built_in
    edge push_built_in -> built_in_call
    edge built_in_call -> typeof_output
  }

  ; Now we define the "definition" route (aka. the pop route), to use in `using` directives only
  ; This is essentially the reverse of the second path above
  node pop_array
  attr (pop_array) pop_symbol = @array.type_symbol

  let @array.pop_begin = @type_name.pop_begin
  edge @type_name.pop_end -> pop_array
  let @array.pop_end = pop_array
}


;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
;;; Function types
;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;

@ftype [FunctionType @attrs [FunctionTypeAttributes]] {
  ; Compute the built-in type of the function
  ; %functionExternal provides access to .selector and .address
  var type_symbol = "%Function"
  scan (source-text @attrs) {
    "external" {
      set type_symbol = "%ExternalFunction"
    }
  }

  node @ftype.lexical_scope
  node @ftype.output

  ; This path pushes the function type to the symbol stack
  ; TODO: add parameter and return types to distinguish between different function types
  node function_type
  attr (function_type) push_symbol = type_symbol

  edge @ftype.output -> function_type
  edge function_type -> @ftype.lexical_scope

  ; the pop path for the using directive
  node pop_function_type
  attr (pop_function_type) pop_symbol = type_symbol

  let @ftype.pop_begin = pop_function_type
  let @ftype.pop_end = pop_function_type
}

@ftype [FunctionType @params [ParametersDeclaration]] {
  edge @params.lexical_scope -> @ftype.lexical_scope
}

@ftype [FunctionType [ReturnsDeclaration @return_params [ParametersDeclaration]]] {
  edge @return_params.lexical_scope -> @ftype.lexical_scope
}

@ftype [FunctionType [ReturnsDeclaration
    [ParametersDeclaration [Parameters . @param [Parameter] .]]
]] {
  ; Variables of a function type type can be "called" and resolve to the type of
  ; the return parameter. This is only valid if the function returns a single
  ; value.
  node typeof
  attr (typeof) pop_symbol = "@typeof"

  node call
  attr (call) pop_symbol = "()"

  edge @ftype.output -> typeof
  edge typeof -> call
  edge call -> @param.typeof
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
;;   NOTE: most of the time, and unless this identifier path is the target of a
;;   using directive this second path will not be used and will form a
;;   disconnected graph component. We currently have no way of determining when
;;   this path is necessary, so we always construct it.
;;
;; Additionally the IdentifierPath defines another scoped variable
;; @id_path.rightmost_identifier which corresponds to the identifier in the last
;; position in the path, from left to right. This is used in the using directive
;; rules to be able to pop the name of the attached function.

@id_path [IdentifierPath] {
  ; This node connects to all parts of the path, for popping. This allows to
  ; connect at any point of the path. Useful for `using` directives when the
  ; target type is fully qualified but we want to resolve for the unqualified
  ; name.
  node @id_path.all_pop_begin
}

@id_path [IdentifierPath @name [Identifier]] {
  node @name.ref
  attr (@name.ref) node_reference = @name
  attr (@name.ref) parents = [@id_path.enclosing_def]

  node @name.pop
  attr (@name.pop) pop_symbol = (source-text @name)

  edge @id_path.all_pop_begin -> @name.pop
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

@function [FunctionDefinition @attrs [FunctionAttributes]] {
  var type_symbol = "%Function"
  scan (source-text @attrs) {
    "\\b(public|external)\\b" {
      set type_symbol = "%ExternalFunction"
    }
  }

  node @function.lexical_scope
  node @function.extended_scope
  node @function.def

  ; this path from the function definition to the scope allows attaching
  ; functions to this function's type
  node typeof
  attr (typeof) push_symbol = "@typeof"
  node type_function
  attr (type_function) push_symbol = type_symbol
  edge @function.def -> typeof
  edge typeof -> type_function
  edge type_function -> @function.lexical_scope
}

@function [FunctionDefinition name: [FunctionName @name [Identifier]]] {
  attr (@function.def) node_definition = @name
  attr (@function.def) definiens_node = @function
}

@function [FunctionDefinition @params parameters: [ParametersDeclaration]] {
  edge @params.lexical_scope -> @function.extended_scope

  ;; Input parameters are available in the function scope
  edge @function.extended_scope -> @params.defs
  ;; ... and shadow other declarations
  attr (@function.extended_scope -> @params.defs) precedence = 1

  ;; Connect to paramaters for named argument resolution
  edge @function.def -> @params.names
}

@function [FunctionDefinition returns: [ReturnsDeclaration
    @return_params [ParametersDeclaration]
]] {
  edge @return_params.lexical_scope -> @function.extended_scope

  ;; Return parameters are available in the function scope
  edge @function.extended_scope -> @return_params.defs
  ;; ... and shadow other declarations
  attr (@function.extended_scope -> @return_params.defs) precedence = 1
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
  edge @block.lexical_scope -> @function.extended_scope
}

@function [FunctionDefinition [FunctionAttributes item: [FunctionAttribute
    @modifier [ModifierInvocation]
]]] {
  edge @modifier.lexical_scope -> @function.extended_scope
}

@modifier [ModifierInvocation @name [IdentifierPath]] {
  node @modifier.lexical_scope

  node modifier
  attr (modifier) push_symbol = "@modifier"

  edge @name.push_end -> modifier
  edge modifier -> @modifier.lexical_scope

  ; This allows resolving @name in the more general scope in constructors (since
  ; calling a parent constructor is parsed as a modifier invocation)
  let @modifier.identifier = @name.push_end
}

@modifier [ModifierInvocation @args [ArgumentsDeclaration]] {
  edge @args.lexical_scope -> @modifier.lexical_scope
}

;;; Unnamed functions (deprecated)
@unnamed_function [UnnamedFunctionDefinition] {
  node @unnamed_function.lexical_scope
  node @unnamed_function.extended_scope
}

@unnamed_function [UnnamedFunctionDefinition @params parameters: [ParametersDeclaration]] {
  edge @params.lexical_scope -> @unnamed_function.extended_scope

  edge @unnamed_function.extended_scope -> @params.defs
  attr (@unnamed_function.extended_scope -> @params.defs) precedence = 1
}

@unnamed_function [UnnamedFunctionDefinition [FunctionBody @block [Block]]] {
  edge @block.lexical_scope -> @unnamed_function.extended_scope
}

@unnamed_function [UnnamedFunctionDefinition
    [UnnamedFunctionAttributes [UnnamedFunctionAttribute @modifier [ModifierInvocation]]]
] {
  edge @modifier.lexical_scope -> @unnamed_function.lexical_scope
}



;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
;;; Constructors
;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;

@constructor [ConstructorDefinition] {
  node @constructor.lexical_scope
  node @constructor.extended_scope
  node @constructor.def
}

@constructor [ConstructorDefinition @params parameters: [ParametersDeclaration]] {
  edge @params.lexical_scope -> @constructor.extended_scope

  ;; Input parameters are available in the constructor scope
  edge @constructor.extended_scope -> @params.defs
  ;; ... and shadow other declarations
  attr (@constructor.extended_scope -> @params.defs) precedence = 1

  ;; Connect to paramaters for named argument resolution
  edge @constructor.def -> @params.names
}

;; Connect the constructor body's block lexical scope to the constructor
@constructor [ConstructorDefinition @block [Block]] {
  edge @block.lexical_scope -> @constructor.extended_scope
}

@constructor [ConstructorDefinition [ConstructorAttributes item: [ConstructorAttribute
    @modifier [ModifierInvocation]
]]] {
  edge @modifier.lexical_scope -> @constructor.extended_scope
  edge @modifier.identifier -> @constructor.lexical_scope
}

@contract [ContractDefinition [ContractMembers [ContractMember
    @constructor [ConstructorDefinition]
]]] {
  ;; This link allows calling a constructor with the named parameters syntax
  edge @contract.def -> @constructor.def
}

;; Solidity < 0.5.0 constructors
;; They were declared as functions of the contract's name

@contract [ContractDefinition
    @contract_name [Identifier]
    [ContractMembers [ContractMember [FunctionDefinition
        [FunctionName @function_name [Identifier]]
        @params [ParametersDeclaration]
    ]]]
] {
  if (version-matches "< 0.5.0") {
    if (eq (source-text @contract_name) (source-text @function_name)) {
      ; Connect to paramaters for named argument resolution
      edge @contract.def -> @params.names
    }
  }
}

[ContractDefinition
    @contract_name [Identifier]
    [ContractMembers [ContractMember @function [FunctionDefinition
        [FunctionName @function_name [Identifier]]
        [FunctionAttributes [FunctionAttribute @modifier [ModifierInvocation]]]
    ]]]
] {
  if (version-matches "< 0.5.0") {
    if (eq (source-text @contract_name) (source-text @function_name)) {
      ; Parent constructor calls are parsed as modifier invocations
      edge @modifier.identifier -> @function.lexical_scope
    }
  }
}


;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
;;; Fallback and receive functions
;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;

@fallback [FallbackFunctionDefinition] {
  node @fallback.lexical_scope
  node @fallback.extended_scope
}

@fallback [FallbackFunctionDefinition @params parameters: [ParametersDeclaration]] {
  edge @params.lexical_scope -> @fallback.extended_scope

  ;; Input parameters are available in the fallback function scope
  edge @fallback.extended_scope -> @params.defs
  attr (@fallback.extended_scope -> @params.defs) precedence = 1
}

@fallback [FallbackFunctionDefinition returns: [ReturnsDeclaration
    @return_params [ParametersDeclaration]
]] {
  edge @return_params.lexical_scope -> @fallback.lexical_scope

  ;; Return parameters are available in the fallback function scope
  edge @fallback.extended_scope -> @return_params.defs
  attr (@fallback.extended_scope -> @return_params.defs) precedence = 1
}

@fallback [FallbackFunctionDefinition [FunctionBody @block [Block]]] {
  edge @block.lexical_scope -> @fallback.extended_scope
}

@fallback [FallbackFunctionDefinition [FallbackFunctionAttributes
    item: [FallbackFunctionAttribute @modifier [ModifierInvocation]]
]] {
  edge @modifier.lexical_scope -> @fallback.lexical_scope
}

@receive [ReceiveFunctionDefinition] {
  node @receive.lexical_scope
  node @receive.extended_scope
}

@receive [ReceiveFunctionDefinition [FunctionBody @block [Block]]] {
  edge @block.lexical_scope -> @receive.extended_scope
}

@receive [ReceiveFunctionDefinition [ReceiveFunctionAttributes
    item: [ReceiveFunctionAttribute @modifier [ModifierInvocation]]
]] {
  edge @modifier.lexical_scope -> @receive.lexical_scope
}


;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
;;; Function modifiers
;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;

@modifier [ModifierDefinition] {
  node @modifier.def
  node @modifier.lexical_scope
  node @modifier.extended_scope
}

@modifier [ModifierDefinition
    @name name: [Identifier]
    body: [FunctionBody @body [Block]]
] {
  attr (@modifier.def) node_definition = @name
  attr (@modifier.def) definiens_node = @modifier

  edge @body.lexical_scope -> @modifier.extended_scope

  ; Special case: bind the place holder statement `_` to the built-in
  ; `%placeholder`. This only happens in the body of a modifier.
  node placeholder_pop
  attr (placeholder_pop) pop_symbol = "_"
  node placeholder_ref
  attr (placeholder_ref) push_symbol = "%placeholder"

  edge @body.lexical_scope -> placeholder_pop
  edge placeholder_pop -> placeholder_ref
  edge placeholder_ref -> @modifier.lexical_scope
}

@modifier [ModifierDefinition @params [ParametersDeclaration]] {
  edge @params.lexical_scope -> @modifier.extended_scope

  ;; Input parameters are available in the modifier scope
  edge @modifier.extended_scope -> @params.defs
  attr (@modifier.extended_scope -> @params.defs) precedence = 1
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
}


;;; Variable declaration statements

@stmt [Statement @var_decl [VariableDeclarationStatement]] {
  edge @var_decl.lexical_scope -> @stmt.lexical_scope
  edge @stmt.defs -> @var_decl.def
}

@var_decl [VariableDeclarationStatement] {
  node @var_decl.lexical_scope
  node @var_decl.def
}

@var_decl [VariableDeclarationStatement
    [VariableDeclarationType @var_type [TypeName]]
    @name name: [Identifier]
] {
  attr (@var_decl.def) node_definition = @name
  attr (@var_decl.def) definiens_node = @var_decl

  edge @var_type.type_ref -> @var_decl.lexical_scope

  node typeof
  attr (typeof) push_symbol = "@typeof"

  edge @var_decl.def -> typeof
  edge typeof -> @var_type.output
}

@var_decl [VariableDeclarationStatement
    [VariableDeclarationType [VarKeyword]]
    @name name: [Identifier]
] {
  attr (@var_decl.def) node_definition = @name
  attr (@var_decl.def) definiens_node = @var_decl
}

@var_decl [VariableDeclarationStatement
    [VariableDeclarationType [VarKeyword]]
    [VariableDeclarationValue @value [Expression]]
] {
  edge @var_decl.def -> @value.output
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

@stmt [Statement [ForStatement
    initialization: [ForStatementInitialization @init_stmt [ExpressionStatement]]
]] {
  edge @init_stmt.lexical_scope -> @stmt.lexical_scope
}

@stmt [Statement [ForStatement
    initialization: [ForStatementInitialization @init_stmt [VariableDeclarationStatement]]
]] {
  edge @init_stmt.lexical_scope -> @stmt.lexical_scope
  edge @stmt.init_defs -> @init_stmt.def
}

@stmt [Statement [ForStatement
    initialization: [ForStatementInitialization @init_stmt [TupleDeconstructionStatement]]
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
  ; for the iterator expression we need an independent scope node that can
  ; connect to both the for-statement *and* the definitions in the init
  ; expression
  node @iter_expr.lexical_scope
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
;;; Error handling
;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;

;;; Try-catch statements

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
  node @state_var.extended_scope
  node @state_var.def
}

@state_var [StateVariableDefinition
    @type_name type_name: [TypeName]
    @name name: [Identifier]
] {
  attr (@state_var.def) node_definition = @name
  attr (@state_var.def) definiens_node = @state_var

  edge @type_name.type_ref -> @state_var.lexical_scope

  node @state_var.typeof
  attr (@state_var.typeof) push_symbol = "@typeof"

  edge @state_var.def -> @state_var.typeof
  edge @state_var.typeof -> @type_name.output
}

@state_var [StateVariableDefinition
    [StateVariableAttributes [StateVariableAttribute [PublicKeyword]]]
] {
  ; Public state variables are used as functions when invoked from an external contract
  node call
  attr (call) pop_symbol = "()"

  ; In the general case using the getter can bind to the state variable's type
  edge @state_var.def -> call
  edge call -> @state_var.typeof

  ; Some complex types generate special getters (ie. arrays and mappings index
  ; their contents, structs flatten most of their fields and return a tuple)
  node getter
  attr (getter) push_symbol = "@as_getter"
  edge call -> getter
  edge getter -> @state_var.typeof
}

@state_var [StateVariableDefinition
    [StateVariableDefinitionValue @value [Expression]]
] {
  let @value.lexical_scope = @state_var.extended_scope
}


;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
;;; Enum definitions
;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;

@enum [EnumDefinition @name name: [Identifier]] {
  node @enum.lexical_scope
  node @enum.def
  node @enum.members

  attr (@enum.def) node_definition = @name
  attr (@enum.def) definiens_node = @enum

  node member
  attr (member) pop_symbol = "."

  edge @enum.def -> member
  edge member -> @enum.members

  ; Path to resolve the built-in type for enums (which is the same as for integer types)
  node type
  attr (type) pop_symbol = "@type"
  node type_enum_type
  attr (type_enum_type) push_symbol = "%IntTypeType"
  edge @enum.def -> type
  edge type -> type_enum_type
  edge type_enum_type -> @enum.lexical_scope
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

@struct [StructDefinition @name name: [Identifier]] {
  node @struct.lexical_scope
  node @struct.def
  node @struct.members

  attr (@struct.def) node_definition = @name
  attr (@struct.def) definiens_node = @struct

  ; Now connect normally to the struct members
  node @struct.typeof
  attr (@struct.typeof) pop_symbol = "@typeof"
  node member
  attr (member) pop_symbol = "."
  edge @struct.def -> @struct.typeof
  edge @struct.typeof -> member
  edge member -> @struct.members

  ; Bind member names when using construction with named arguments
  node param_names
  attr (param_names) pop_symbol = "@param_names"
  edge @struct.def -> param_names
  edge param_names -> @struct.members

  ; Used as a function call (ie. casting), should bind to itself
  node call
  attr (call) pop_symbol = "()"
  edge @struct.def -> call
  edge call -> member
}

@struct [StructDefinition [StructMembers
    @member item: [StructMember @type_name [TypeName] @name name: [Identifier]]
]] {
  node @member.def
  attr (@member.def) node_definition = @name
  attr (@member.def) definiens_node = @member

  edge @struct.members -> @member.def

  edge @type_name.type_ref -> @struct.lexical_scope

  node @member.typeof
  attr (@member.typeof) push_symbol = "@typeof"

  edge @member.def -> @member.typeof
  edge @member.typeof -> @type_name.output
}

@struct [StructDefinition [StructMembers . @first_member [StructMember]]] {
  ; As a public getter result, the value returned is a tuple with all our fields flattened
  ; We only care about the first member for name binding, since tuples are not real types
  node getter_call
  attr (getter_call) pop_symbol = "@as_getter"
  edge @struct.typeof -> getter_call
  edge getter_call -> @first_member.typeof
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

  ; Bind to built-in errorType for accessing built-in member `.selector`
  node typeof
  attr (typeof) push_symbol = "@typeof"
  node error_type
  attr (error_type) push_symbol = "%ErrorType"
  edge @error.def -> typeof
  edge typeof -> error_type
  edge error_type -> @error.lexical_scope
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
] {
  node def
  attr (def) node_definition = @name
  attr (def) definiens_node = @constant

  edge @constant.def -> def

  edge @type_name.type_ref -> @constant.lexical_scope
}

@user_type [UserDefinedValueTypeDefinition @name [Identifier] @value_type [ElementaryType]] {
  node @user_type.lexical_scope
  node @user_type.def

  attr (@user_type.def) node_definition = @name
  attr (@user_type.def) definiens_node = @user_type

  ; Provide member resolution through the built-in `%userTypeType`
  ; Because the built-in is defined as a struct, we need to push an extra `@typeof`
  node member_guard
  attr (member_guard) pop_symbol = "."
  node member
  attr (member) push_symbol = "."
  node typeof
  attr (typeof) push_symbol = "@typeof"
  node user_type_type
  attr (user_type_type) push_symbol = "%UserDefinedValueType"

  edge @user_type.def -> member_guard
  edge member_guard -> member
  edge member -> typeof
  edge typeof -> user_type_type
  edge user_type_type -> @user_type.lexical_scope

  ; Hard-code built-in functions `wrap` and `unwrap` in order to be able to
  ; resolve their return types
  node wrap
  attr (wrap) pop_symbol = "wrap"
  node wrap_call
  attr (wrap_call) pop_symbol = "()"
  node wrap_typeof
  attr (wrap_typeof) push_symbol = "@typeof"

  edge member_guard -> wrap
  edge wrap -> wrap_call
  edge wrap_call -> wrap_typeof
  edge wrap_typeof -> @value_type.ref
  edge @value_type.ref -> @user_type.lexical_scope

  node unwrap
  attr (unwrap) pop_symbol = "unwrap"
  node unwrap_call
  attr (unwrap_call) pop_symbol = "()"
  node unwrap_typeof
  attr (unwrap_typeof) push_symbol = "@typeof"
  node type_ref
  attr (type_ref) push_symbol = (source-text @name)

  edge member_guard -> unwrap
  edge unwrap -> unwrap_call
  edge unwrap_call -> unwrap_typeof
  edge unwrap_typeof -> type_ref
  edge type_ref -> @user_type.lexical_scope
}


;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
;;; Expressions
;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;

;; Expressions have two important scoped variables:
;; - @expr.lexical_scope should be set by the enclosing node to provide a scope
;;   for name resolution
;; - @expr.output is a node provided by the expression and represents the output
;;   of the expression for chaining eg. with a member access

@expr [Expression] {
  ;; this is an output scope for use in member access (and other uses)
  node @expr.output
}

;; Identifier expressions
@expr [Expression @name [Identifier]] {
  node ref
  attr (ref) node_reference = @name
  attr (ref) parents = [@expr.enclosing_def]

  edge ref -> @expr.lexical_scope
  edge @expr.output -> ref
}

@expr [Expression @keyword ([ThisKeyword] | [SuperKeyword])] {
  ; This is almost equivalent to the above rule, except it doesn't generate a reference
  node keyword
  attr (keyword) push_symbol = (source-text @keyword)
  edge keyword -> @expr.lexical_scope
  edge @expr.output -> keyword
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

  ; Shortcut path for expressions inside contracts with using X for * directives
  edge member -> @expr.star_extension
}

;; Special case: member accesses to `super` are tagged with "super" to rank
;; virtual methods correctly
[MemberAccessExpression
    operand: [Expression [SuperKeyword]]
    @name member: [Identifier]
] {
  attr (@name.ref) tag = "super"
}

;; Elementary types used as expressions (eg. for type casting, or for built-ins like `string.concat`)
@expr [Expression @type [ElementaryType]] {
  edge @expr.output -> @type.ref
  edge @type.ref -> @expr.lexical_scope

  ; Elementary types can also be used for casting; instead of defining built-in
  ; struct for each available elementary type, we define a special path here
  node call
  attr (call) pop_symbol = "()"
  node typeof
  attr (typeof) push_symbol = "@typeof"
  edge @expr.output -> call
  edge call -> typeof
  edge typeof -> @type.ref
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

@type_expr [Expression [TypeExpression [TypeName [ElementaryType ([IntKeyword] | [UintKeyword])]]]] {
  ; For integer types the type's type is fixed
  node typeof
  attr (typeof) push_symbol = "@typeof"
  node type
  attr (type) push_symbol = "%IntTypeType"

  edge @type_expr.output -> typeof
  edge typeof -> type
  edge type -> @type_expr.lexical_scope
}

@type_expr [Expression [TypeExpression [TypeName @id_path [IdentifierPath]]]] {
  ; For other identifiers, resolve it through a pseudo-member `%type`
  node typeof
  attr (typeof) push_symbol = "@typeof"
  node type
  attr (type) push_symbol = "@type"

  edge @type_expr.output -> typeof
  edge typeof -> type
  edge type -> @id_path.push_begin
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

@named_arg [NamedArgument @name [Identifier] [Colon] [Expression]] {
  node @named_arg.lexical_scope

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

@expr [Expression [CallOptionsExpression @operand [Expression] @options [CallOptions]]] {
  edge @expr.output -> @operand.output

  node @options.refs
  attr (@options.refs) push_symbol = "@param_names"

  node call_options
  attr (call_options) push_symbol = "%CallOptions"

  edge @options.refs -> call_options
  edge call_options -> @expr.lexical_scope
}

@expr [Expression [CallOptionsExpression
    @options [CallOptions @named_arg [NamedArgument]]
]] {
  edge @named_arg.lexical_scope -> @expr.lexical_scope
  edge @named_arg.ref -> @options.refs
}


;;; Payable
; These work like `address`, should they should bind to `%address`
@expr [Expression [PayableKeyword]] {
  node ref
  attr (ref) push_symbol = "%address"

  edge ref -> @expr.lexical_scope
  edge @expr.output -> ref
}


;;; Tuple expressions

; Parenthesized expressions are parsed as tuples of a single value
@expr [Expression [TupleExpression [TupleValues . [TupleValue @operand [Expression]] .]]] {
  edge @expr.output -> @operand.output
}

;;; Arithmetic, bitwise & logical operators, etc

; Bind to the left operand only: assignment expressions
@expr [Expression [_
    @left_operand left_operand: [Expression]
    (
          [Equal]
        | [BarEqual]
        | [PlusEqual]
        | [MinusEqual]
        | [CaretEqual]
        | [SlashEqual]
        | [PercentEqual]
        | [AsteriskEqual]
        | [AmpersandEqual]
        | [LessThanLessThanEqual]
        | [GreaterThanGreaterThanEqual]
        | [GreaterThanGreaterThanGreaterThanEqual]
    )
]] {
  edge @expr.output -> @left_operand.output
}

; Unary operators postfix
@expr [Expression [_
    @operand operand: [Expression]
    ([PlusPlus] | [MinusMinus])
]] {
  edge @expr.output -> @operand.output
}

; Unary operators prefix
@expr [Expression [_
    ([PlusPlus] | [MinusMinus] | [Tilde] | [Bang] | [Minus] | [Plus])
    @operand operand: [Expression]
]] {
  edge @expr.output -> @operand.output
}

; Bind to both operands: logical and/or, arithmetic, bit-wise expressions
@expr [Expression [_
    @left_operand left_operand: [Expression]
    (
          [BarBar]
        | [AmpersandAmpersand]

        | [Plus]
        | [Minus]
        | [Asterisk]
        | [Slash]
        | [Percent]
        | [AsteriskAsterisk]

        | [Bar]
        | [Caret]
        | [Ampersand]

        | [LessThanLessThan]
        | [GreaterThanGreaterThan]
        | [GreaterThanGreaterThanGreaterThan]
    )
    @right_operand right_operand: [Expression]
]] {
  edge @expr.output -> @left_operand.output
  edge @expr.output -> @right_operand.output
}

; Comparison operators bind to bool type
@expr [Expression [_
    (
          [EqualEqual]
        | [BangEqual]
        | [LessThan]
        | [GreaterThan]
        | [LessThanEqual]
        | [GreaterThanEqual]
    )
]] {
  node typeof
  attr (typeof) push_symbol = "@typeof"
  node bool
  attr (bool) push_symbol = "%bool"
  edge @expr.output -> typeof
  edge typeof -> bool
  edge bool -> @expr.lexical_scope
}

; Ternary conditional expression binds to both branches
@expr [Expression [ConditionalExpression
    @true_expression true_expression: [Expression]
    @false_expression false_expression: [Expression]
]] {
  edge @expr.output -> @true_expression.output
  edge @expr.output -> @false_expression.output
}


;;; Literal Address Expressions
@expr [Expression [HexNumberExpression @hex_literal [HexLiteral]]] {
  scan (source-text @hex_literal) {
    "0x[0-9a-fA-F]{40}" {
      ; Treat it as a valid address
      node typeof
      attr (typeof) push_symbol = "@typeof"
      node address
      attr (address) push_symbol = "%address"
      edge @expr.output -> typeof
      edge typeof -> address
      edge address -> @expr.lexical_scope
    }
  }
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
  ; Exception: but outside constants *are* available, so we provide a guarded
  ; access to the parent lexical scope. This guard will be popped to link to
  ; available constants.
  node yul_function_guard
  attr (yul_function_guard) push_symbol = "@in_yul_function"
  edge @fundef.lexical_scope -> yul_function_guard
  edge yul_function_guard -> @block.lexical_scope
}

;; Constants need to be available inside Yul functions. This is an exception
;; since no other external identifiers are, so the path is guarded. We create a
;; scope in the source unit, contracts and libraries, and guard it from the
;; lexical scope, so we can link constant definitions here. See the dual path in
;; the rule above.
@constant_container ([SourceUnit] | [ContractDefinition] | [LibraryDefinition]) {
  node @constant_container.yul_functions_guarded_scope
  attr (@constant_container.yul_functions_guarded_scope) pop_symbol = "@in_yul_function"
  edge @constant_container.lexical_scope -> @constant_container.yul_functions_guarded_scope
}

;; Make top-level constants available inside Yul functions
@source_unit [SourceUnit [SourceUnitMembers [SourceUnitMember @constant [ConstantDefinition]]]] {
  edge @source_unit.yul_functions_guarded_scope -> @constant.def
}

;; Ditto for contracts, interfaces and libraries
@contract [_ members: [_ [ContractMember
    @constant [StateVariableDefinition
        [StateVariableAttributes [StateVariableAttribute [ConstantKeyword]]]
    ]
]]] {
  edge @contract.yul_functions_guarded_scope -> @constant.def
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

@path [YulPath] {
  node @path.lexical_scope
}

@path [YulPath . @name [YulIdentifier]] {
  node ref
  attr (ref) node_reference = @name

  edge ref -> @path.lexical_scope

  if (version-matches "< 0.7.0") {
    ; Before Solidity 0.7.0 storage variables' `.offset` and `.slot` were
    ; accessed by suffixing the name with `_offset` and `_slot`
    scan (source-text @name) {
      "^(.*)_(slot|offset|length)$" {
        let symbol = $0
        let without_suffix = $1
        let suffix = $2

        ; We bind the whole symbol to the built-in field for the known cases
        node pop_ref
        attr (pop_ref) pop_symbol = symbol
        node push_suffixless
        attr (push_suffixless) push_symbol = suffix
        node member_of
        attr (member_of) push_symbol = "."
        node typeof
        attr (typeof) push_symbol = "@typeof"
        node yul_external
        attr (yul_external) push_symbol = "%YulExternal"

        edge ref -> pop_ref
        edge pop_ref -> push_suffixless
        edge push_suffixless -> member_of
        edge member_of -> typeof
        edge typeof -> yul_external
        edge yul_external -> @path.lexical_scope
      }
    }
  }
}

@path [YulPath [Period] @member [YulIdentifier] .] {
  ; Yul variable members only apply to external variables and hence are
  ; automatically bound to a special %YulExternal built-in
  node ref
  attr (ref) node_reference = @member
  node member_of
  attr (member_of) push_symbol = "."
  node typeof
  attr (typeof) push_symbol = "@typeof"
  node yul_external
  attr (yul_external) push_symbol = "%YulExternal"

  edge ref -> member_of
  edge member_of -> typeof
  edge typeof -> yul_external
  edge yul_external -> @path.lexical_scope
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
