// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use std::rc::Rc;
use std::vec::Vec;

use crate::cst::{Cursor, TerminalNode};

//
// Sequences:
//

/**
 * This node represents a `SourceUnit` nonterminal, with the following structure:
 *
 * ```ebnf
 * SourceUnit = (* members: *) SourceUnitMembers;
 * ```
 */
pub type SourceUnit = Rc<SourceUnitStruct>;

#[derive(Debug)]
pub struct SourceUnitStruct {
    pub cursor: Cursor,
    pub members: SourceUnitMembers,
}

/**
 * This node represents a `PragmaDirective` nonterminal, with the following structure:
 *
 * ```ebnf
 * PragmaDirective = (* pragma_keyword: *) PRAGMA_KEYWORD
 *                   (* pragma: *) Pragma
 *                   (* semicolon: *) SEMICOLON;
 * ```
 */
pub type PragmaDirective = Rc<PragmaDirectiveStruct>;

#[derive(Debug)]
pub struct PragmaDirectiveStruct {
    pub cursor: Cursor,
    pub pragma_keyword: Rc<TerminalNode>,
    pub pragma: Pragma,
    pub semicolon: Rc<TerminalNode>,
}

/**
 * This node represents a `AbicoderPragma` nonterminal, with the following structure:
 *
 * ```ebnf
 * AbicoderPragma = (* abicoder_keyword: *) ABICODER_KEYWORD
 *                  (* version: *) IDENTIFIER;
 * ```
 */
pub type AbicoderPragma = Rc<AbicoderPragmaStruct>;

#[derive(Debug)]
pub struct AbicoderPragmaStruct {
    pub cursor: Cursor,
    pub abicoder_keyword: Rc<TerminalNode>,
    pub version: Rc<TerminalNode>,
}

/**
 * This node represents a `ExperimentalPragma` nonterminal, with the following structure:
 *
 * ```ebnf
 * ExperimentalPragma = (* experimental_keyword: *) EXPERIMENTAL_KEYWORD
 *                      (* feature: *) ExperimentalFeature;
 * ```
 */
pub type ExperimentalPragma = Rc<ExperimentalPragmaStruct>;

#[derive(Debug)]
pub struct ExperimentalPragmaStruct {
    pub cursor: Cursor,
    pub experimental_keyword: Rc<TerminalNode>,
    pub feature: ExperimentalFeature,
}

/**
 * This node represents a `VersionPragma` nonterminal, with the following structure:
 *
 * ```ebnf
 * VersionPragma = (* solidity_keyword: *) SOLIDITY_KEYWORD
 *                 (* sets: *) VersionExpressionSets;
 * ```
 */
pub type VersionPragma = Rc<VersionPragmaStruct>;

#[derive(Debug)]
pub struct VersionPragmaStruct {
    pub cursor: Cursor,
    pub solidity_keyword: Rc<TerminalNode>,
    pub sets: VersionExpressionSets,
}

/**
 * This node represents a `VersionRange` nonterminal, with the following structure:
 *
 * ```ebnf
 * VersionRange = (* start: *) VersionLiteral
 *                (* minus: *) MINUS
 *                (* end: *) VersionLiteral;
 * ```
 */
pub type VersionRange = Rc<VersionRangeStruct>;

#[derive(Debug)]
pub struct VersionRangeStruct {
    pub cursor: Cursor,
    pub start: VersionLiteral,
    pub minus: Rc<TerminalNode>,
    pub end: VersionLiteral,
}

/**
 * This node represents a `VersionTerm` nonterminal, with the following structure:
 *
 * ```ebnf
 * VersionTerm = (* operator: *) VersionOperator?
 *               (* literal: *) VersionLiteral;
 * ```
 */
pub type VersionTerm = Rc<VersionTermStruct>;

#[derive(Debug)]
pub struct VersionTermStruct {
    pub cursor: Cursor,
    pub operator: Option<VersionOperator>,
    pub literal: VersionLiteral,
}

/**
 * This node represents a `ImportDirective` nonterminal, with the following structure:
 *
 * ```ebnf
 * ImportDirective = (* import_keyword: *) IMPORT_KEYWORD
 *                   (* clause: *) ImportClause
 *                   (* semicolon: *) SEMICOLON;
 * ```
 */
pub type ImportDirective = Rc<ImportDirectiveStruct>;

#[derive(Debug)]
pub struct ImportDirectiveStruct {
    pub cursor: Cursor,
    pub import_keyword: Rc<TerminalNode>,
    pub clause: ImportClause,
    pub semicolon: Rc<TerminalNode>,
}

/**
 * This node represents a `PathImport` nonterminal, with the following structure:
 *
 * ```ebnf
 * PathImport = (* path: *) StringLiteral
 *              (* alias: *) ImportAlias?;
 * ```
 */
pub type PathImport = Rc<PathImportStruct>;

#[derive(Debug)]
pub struct PathImportStruct {
    pub cursor: Cursor,
    pub path: StringLiteral,
    pub alias: Option<ImportAlias>,
}

/**
 * This node represents a `NamedImport` nonterminal, with the following structure:
 *
 * ```ebnf
 * NamedImport = (* asterisk: *) ASTERISK
 *               (* alias: *) ImportAlias
 *               (* from_keyword: *) FROM_KEYWORD
 *               (* path: *) StringLiteral;
 * ```
 */
pub type NamedImport = Rc<NamedImportStruct>;

#[derive(Debug)]
pub struct NamedImportStruct {
    pub cursor: Cursor,
    pub asterisk: Rc<TerminalNode>,
    pub alias: ImportAlias,
    pub from_keyword: Rc<TerminalNode>,
    pub path: StringLiteral,
}

/**
 * This node represents a `ImportDeconstruction` nonterminal, with the following structure:
 *
 * ```ebnf
 * ImportDeconstruction = (* open_brace: *) OPEN_BRACE
 *                        (* symbols: *) ImportDeconstructionSymbols
 *                        (* close_brace: *) CLOSE_BRACE
 *                        (* from_keyword: *) FROM_KEYWORD
 *                        (* path: *) StringLiteral;
 * ```
 */
pub type ImportDeconstruction = Rc<ImportDeconstructionStruct>;

#[derive(Debug)]
pub struct ImportDeconstructionStruct {
    pub cursor: Cursor,
    pub open_brace: Rc<TerminalNode>,
    pub symbols: ImportDeconstructionSymbols,
    pub close_brace: Rc<TerminalNode>,
    pub from_keyword: Rc<TerminalNode>,
    pub path: StringLiteral,
}

/**
 * This node represents a `ImportDeconstructionSymbol` nonterminal, with the following structure:
 *
 * ```ebnf
 * ImportDeconstructionSymbol = (* name: *) IDENTIFIER
 *                              (* alias: *) ImportAlias?;
 * ```
 */
pub type ImportDeconstructionSymbol = Rc<ImportDeconstructionSymbolStruct>;

#[derive(Debug)]
pub struct ImportDeconstructionSymbolStruct {
    pub cursor: Cursor,
    pub name: Rc<TerminalNode>,
    pub alias: Option<ImportAlias>,
}

/**
 * This node represents a `ImportAlias` nonterminal, with the following structure:
 *
 * ```ebnf
 * ImportAlias = (* as_keyword: *) AS_KEYWORD
 *               (* identifier: *) IDENTIFIER;
 * ```
 */
pub type ImportAlias = Rc<ImportAliasStruct>;

#[derive(Debug)]
pub struct ImportAliasStruct {
    pub cursor: Cursor,
    pub as_keyword: Rc<TerminalNode>,
    pub identifier: Rc<TerminalNode>,
}

/**
 * This node represents a `UsingDirective` nonterminal, with the following structure:
 *
 * ```ebnf
 * UsingDirective = (* using_keyword: *) USING_KEYWORD
 *                  (* clause: *) UsingClause
 *                  (* for_keyword: *) FOR_KEYWORD
 *                  (* target: *) UsingTarget
 *                  (* global_keyword: *) GLOBAL_KEYWORD? (* Introduced in 0.8.13 *)
 *                  (* semicolon: *) SEMICOLON;
 * ```
 */
pub type UsingDirective = Rc<UsingDirectiveStruct>;

#[derive(Debug)]
pub struct UsingDirectiveStruct {
    pub cursor: Cursor,
    pub using_keyword: Rc<TerminalNode>,
    pub clause: UsingClause,
    pub for_keyword: Rc<TerminalNode>,
    pub target: UsingTarget,
    pub global_keyword: Option<Rc<TerminalNode>>,
    pub semicolon: Rc<TerminalNode>,
}

/**
 * This node represents a `UsingDeconstruction` nonterminal, with the following structure:
 *
 * ```ebnf
 * (* Introduced in 0.8.13 *)
 * UsingDeconstruction = (* open_brace: *) OPEN_BRACE
 *                       (* symbols: *) UsingDeconstructionSymbols
 *                       (* close_brace: *) CLOSE_BRACE;
 * ```
 */
pub type UsingDeconstruction = Rc<UsingDeconstructionStruct>;

#[derive(Debug)]
pub struct UsingDeconstructionStruct {
    pub cursor: Cursor,
    pub open_brace: Rc<TerminalNode>,
    pub symbols: UsingDeconstructionSymbols,
    pub close_brace: Rc<TerminalNode>,
}

/**
 * This node represents a `UsingDeconstructionSymbol` nonterminal, with the following structure:
 *
 * ```ebnf
 * (* Introduced in 0.8.13 *)
 * UsingDeconstructionSymbol = (* name: *) IdentifierPath
 *                             (* alias: *) UsingAlias?; (* Introduced in 0.8.19 *)
 * ```
 */
pub type UsingDeconstructionSymbol = Rc<UsingDeconstructionSymbolStruct>;

#[derive(Debug)]
pub struct UsingDeconstructionSymbolStruct {
    pub cursor: Cursor,
    pub name: IdentifierPath,
    pub alias: Option<UsingAlias>,
}

/**
 * This node represents a `UsingAlias` nonterminal, with the following structure:
 *
 * ```ebnf
 * (* Introduced in 0.8.19 *)
 * UsingAlias = (* as_keyword: *) AS_KEYWORD
 *              (* operator: *) UsingOperator;
 * ```
 */
pub type UsingAlias = Rc<UsingAliasStruct>;

#[derive(Debug)]
pub struct UsingAliasStruct {
    pub cursor: Cursor,
    pub as_keyword: Rc<TerminalNode>,
    pub operator: UsingOperator,
}

/**
 * This node represents a `ContractDefinition` nonterminal, with the following structure:
 *
 * ```ebnf
 * ContractDefinition = (* abstract_keyword: *) ABSTRACT_KEYWORD? (* Introduced in 0.6.0 *)
 *                      (* contract_keyword: *) CONTRACT_KEYWORD
 *                      (* name: *) IDENTIFIER
 *                      (* inheritance: *) InheritanceSpecifier?
 *                      (* open_brace: *) OPEN_BRACE
 *                      (* members: *) ContractMembers
 *                      (* close_brace: *) CLOSE_BRACE;
 * ```
 */
pub type ContractDefinition = Rc<ContractDefinitionStruct>;

#[derive(Debug)]
pub struct ContractDefinitionStruct {
    pub cursor: Cursor,
    pub abstract_keyword: Option<Rc<TerminalNode>>,
    pub contract_keyword: Rc<TerminalNode>,
    pub name: Rc<TerminalNode>,
    pub inheritance: Option<InheritanceSpecifier>,
    pub open_brace: Rc<TerminalNode>,
    pub members: ContractMembers,
    pub close_brace: Rc<TerminalNode>,
}

/**
 * This node represents a `InheritanceSpecifier` nonterminal, with the following structure:
 *
 * ```ebnf
 * InheritanceSpecifier = (* is_keyword: *) IS_KEYWORD
 *                        (* types: *) InheritanceTypes;
 * ```
 */
pub type InheritanceSpecifier = Rc<InheritanceSpecifierStruct>;

#[derive(Debug)]
pub struct InheritanceSpecifierStruct {
    pub cursor: Cursor,
    pub is_keyword: Rc<TerminalNode>,
    pub types: InheritanceTypes,
}

/**
 * This node represents a `InheritanceType` nonterminal, with the following structure:
 *
 * ```ebnf
 * InheritanceType = (* type_name: *) IdentifierPath
 *                   (* arguments: *) ArgumentsDeclaration?;
 * ```
 */
pub type InheritanceType = Rc<InheritanceTypeStruct>;

#[derive(Debug)]
pub struct InheritanceTypeStruct {
    pub cursor: Cursor,
    pub type_name: IdentifierPath,
    pub arguments: Option<ArgumentsDeclaration>,
}

/**
 * This node represents a `InterfaceDefinition` nonterminal, with the following structure:
 *
 * ```ebnf
 * InterfaceDefinition = (* interface_keyword: *) INTERFACE_KEYWORD
 *                       (* name: *) IDENTIFIER
 *                       (* inheritance: *) InheritanceSpecifier?
 *                       (* open_brace: *) OPEN_BRACE
 *                       (* members: *) InterfaceMembers
 *                       (* close_brace: *) CLOSE_BRACE;
 * ```
 */
pub type InterfaceDefinition = Rc<InterfaceDefinitionStruct>;

#[derive(Debug)]
pub struct InterfaceDefinitionStruct {
    pub cursor: Cursor,
    pub interface_keyword: Rc<TerminalNode>,
    pub name: Rc<TerminalNode>,
    pub inheritance: Option<InheritanceSpecifier>,
    pub open_brace: Rc<TerminalNode>,
    pub members: InterfaceMembers,
    pub close_brace: Rc<TerminalNode>,
}

/**
 * This node represents a `LibraryDefinition` nonterminal, with the following structure:
 *
 * ```ebnf
 * LibraryDefinition = (* library_keyword: *) LIBRARY_KEYWORD
 *                     (* name: *) IDENTIFIER
 *                     (* open_brace: *) OPEN_BRACE
 *                     (* members: *) LibraryMembers
 *                     (* close_brace: *) CLOSE_BRACE;
 * ```
 */
pub type LibraryDefinition = Rc<LibraryDefinitionStruct>;

#[derive(Debug)]
pub struct LibraryDefinitionStruct {
    pub cursor: Cursor,
    pub library_keyword: Rc<TerminalNode>,
    pub name: Rc<TerminalNode>,
    pub open_brace: Rc<TerminalNode>,
    pub members: LibraryMembers,
    pub close_brace: Rc<TerminalNode>,
}

/**
 * This node represents a `StructDefinition` nonterminal, with the following structure:
 *
 * ```ebnf
 * StructDefinition = (* struct_keyword: *) STRUCT_KEYWORD
 *                    (* name: *) IDENTIFIER
 *                    (* open_brace: *) OPEN_BRACE
 *                    (* members: *) StructMembers
 *                    (* close_brace: *) CLOSE_BRACE;
 * ```
 */
pub type StructDefinition = Rc<StructDefinitionStruct>;

#[derive(Debug)]
pub struct StructDefinitionStruct {
    pub cursor: Cursor,
    pub struct_keyword: Rc<TerminalNode>,
    pub name: Rc<TerminalNode>,
    pub open_brace: Rc<TerminalNode>,
    pub members: StructMembers,
    pub close_brace: Rc<TerminalNode>,
}

/**
 * This node represents a `StructMember` nonterminal, with the following structure:
 *
 * ```ebnf
 * StructMember = (* type_name: *) TypeName
 *                (* name: *) IDENTIFIER
 *                (* semicolon: *) SEMICOLON;
 * ```
 */
pub type StructMember = Rc<StructMemberStruct>;

#[derive(Debug)]
pub struct StructMemberStruct {
    pub cursor: Cursor,
    pub type_name: TypeName,
    pub name: Rc<TerminalNode>,
    pub semicolon: Rc<TerminalNode>,
}

/**
 * This node represents a `EnumDefinition` nonterminal, with the following structure:
 *
 * ```ebnf
 * EnumDefinition = (* enum_keyword: *) ENUM_KEYWORD
 *                  (* name: *) IDENTIFIER
 *                  (* open_brace: *) OPEN_BRACE
 *                  (* members: *) EnumMembers
 *                  (* close_brace: *) CLOSE_BRACE;
 * ```
 */
pub type EnumDefinition = Rc<EnumDefinitionStruct>;

#[derive(Debug)]
pub struct EnumDefinitionStruct {
    pub cursor: Cursor,
    pub enum_keyword: Rc<TerminalNode>,
    pub name: Rc<TerminalNode>,
    pub open_brace: Rc<TerminalNode>,
    pub members: EnumMembers,
    pub close_brace: Rc<TerminalNode>,
}

/**
 * This node represents a `ConstantDefinition` nonterminal, with the following structure:
 *
 * ```ebnf
 * (* Introduced in 0.7.4 *)
 * ConstantDefinition = (* type_name: *) TypeName
 *                      (* constant_keyword: *) CONSTANT_KEYWORD
 *                      (* name: *) IDENTIFIER
 *                      (* equal: *) EQUAL
 *                      (* value: *) Expression
 *                      (* semicolon: *) SEMICOLON;
 * ```
 */
pub type ConstantDefinition = Rc<ConstantDefinitionStruct>;

#[derive(Debug)]
pub struct ConstantDefinitionStruct {
    pub cursor: Cursor,
    pub type_name: TypeName,
    pub constant_keyword: Rc<TerminalNode>,
    pub name: Rc<TerminalNode>,
    pub equal: Rc<TerminalNode>,
    pub value: Expression,
    pub semicolon: Rc<TerminalNode>,
}

/**
 * This node represents a `StateVariableDefinition` nonterminal, with the following structure:
 *
 * ```ebnf
 * StateVariableDefinition = (* type_name: *) TypeName
 *                           (* attributes: *) StateVariableAttributes
 *                           (* name: *) IDENTIFIER
 *                           (* value: *) StateVariableDefinitionValue?
 *                           (* semicolon: *) SEMICOLON;
 * ```
 */
pub type StateVariableDefinition = Rc<StateVariableDefinitionStruct>;

#[derive(Debug)]
pub struct StateVariableDefinitionStruct {
    pub cursor: Cursor,
    pub type_name: TypeName,
    pub attributes: StateVariableAttributes,
    pub name: Rc<TerminalNode>,
    pub value: Option<StateVariableDefinitionValue>,
    pub semicolon: Rc<TerminalNode>,
}

/**
 * This node represents a `StateVariableDefinitionValue` nonterminal, with the following structure:
 *
 * ```ebnf
 * StateVariableDefinitionValue = (* equal: *) EQUAL
 *                                (* value: *) Expression;
 * ```
 */
pub type StateVariableDefinitionValue = Rc<StateVariableDefinitionValueStruct>;

#[derive(Debug)]
pub struct StateVariableDefinitionValueStruct {
    pub cursor: Cursor,
    pub equal: Rc<TerminalNode>,
    pub value: Expression,
}

/**
 * This node represents a `FunctionDefinition` nonterminal, with the following structure:
 *
 * ```ebnf
 * FunctionDefinition = (* function_keyword: *) FUNCTION_KEYWORD
 *                      (* name: *) FunctionName
 *                      (* parameters: *) ParametersDeclaration
 *                      (* attributes: *) FunctionAttributes
 *                      (* returns: *) ReturnsDeclaration?
 *                      (* body: *) FunctionBody;
 * ```
 */
pub type FunctionDefinition = Rc<FunctionDefinitionStruct>;

#[derive(Debug)]
pub struct FunctionDefinitionStruct {
    pub cursor: Cursor,
    pub function_keyword: Rc<TerminalNode>,
    pub name: FunctionName,
    pub parameters: ParametersDeclaration,
    pub attributes: FunctionAttributes,
    pub returns: Option<ReturnsDeclaration>,
    pub body: FunctionBody,
}

/**
 * This node represents a `ParametersDeclaration` nonterminal, with the following structure:
 *
 * ```ebnf
 * ParametersDeclaration = (* open_paren: *) OPEN_PAREN
 *                         (* parameters: *) Parameters
 *                         (* close_paren: *) CLOSE_PAREN;
 * ```
 */
pub type ParametersDeclaration = Rc<ParametersDeclarationStruct>;

#[derive(Debug)]
pub struct ParametersDeclarationStruct {
    pub cursor: Cursor,
    pub open_paren: Rc<TerminalNode>,
    pub parameters: Parameters,
    pub close_paren: Rc<TerminalNode>,
}

/**
 * This node represents a `Parameter` nonterminal, with the following structure:
 *
 * ```ebnf
 * Parameter = (* type_name: *) TypeName
 *             (* storage_location: *) StorageLocation?
 *             (* name: *) IDENTIFIER?;
 * ```
 */
pub type Parameter = Rc<ParameterStruct>;

#[derive(Debug)]
pub struct ParameterStruct {
    pub cursor: Cursor,
    pub type_name: TypeName,
    pub storage_location: Option<StorageLocation>,
    pub name: Option<Rc<TerminalNode>>,
}

/**
 * This node represents a `OverrideSpecifier` nonterminal, with the following structure:
 *
 * ```ebnf
 * (* Introduced in 0.6.0 *)
 * OverrideSpecifier = (* override_keyword: *) OVERRIDE_KEYWORD
 *                     (* overridden: *) OverridePathsDeclaration?;
 * ```
 */
pub type OverrideSpecifier = Rc<OverrideSpecifierStruct>;

#[derive(Debug)]
pub struct OverrideSpecifierStruct {
    pub cursor: Cursor,
    pub override_keyword: Rc<TerminalNode>,
    pub overridden: Option<OverridePathsDeclaration>,
}

/**
 * This node represents a `OverridePathsDeclaration` nonterminal, with the following structure:
 *
 * ```ebnf
 * (* Introduced in 0.6.0 *)
 * OverridePathsDeclaration = (* open_paren: *) OPEN_PAREN
 *                            (* paths: *) OverridePaths
 *                            (* close_paren: *) CLOSE_PAREN;
 * ```
 */
pub type OverridePathsDeclaration = Rc<OverridePathsDeclarationStruct>;

#[derive(Debug)]
pub struct OverridePathsDeclarationStruct {
    pub cursor: Cursor,
    pub open_paren: Rc<TerminalNode>,
    pub paths: OverridePaths,
    pub close_paren: Rc<TerminalNode>,
}

/**
 * This node represents a `ReturnsDeclaration` nonterminal, with the following structure:
 *
 * ```ebnf
 * ReturnsDeclaration = (* returns_keyword: *) RETURNS_KEYWORD
 *                      (* variables: *) ParametersDeclaration;
 * ```
 */
pub type ReturnsDeclaration = Rc<ReturnsDeclarationStruct>;

#[derive(Debug)]
pub struct ReturnsDeclarationStruct {
    pub cursor: Cursor,
    pub returns_keyword: Rc<TerminalNode>,
    pub variables: ParametersDeclaration,
}

/**
 * This node represents a `ConstructorDefinition` nonterminal, with the following structure:
 *
 * ```ebnf
 * (* Introduced in 0.4.22 *)
 * ConstructorDefinition = (* constructor_keyword: *) CONSTRUCTOR_KEYWORD
 *                         (* parameters: *) ParametersDeclaration
 *                         (* attributes: *) ConstructorAttributes
 *                         (* body: *) Block;
 * ```
 */
pub type ConstructorDefinition = Rc<ConstructorDefinitionStruct>;

#[derive(Debug)]
pub struct ConstructorDefinitionStruct {
    pub cursor: Cursor,
    pub constructor_keyword: Rc<TerminalNode>,
    pub parameters: ParametersDeclaration,
    pub attributes: ConstructorAttributes,
    pub body: Block,
}

/**
 * This node represents a `UnnamedFunctionDefinition` nonterminal, with the following structure:
 *
 * ```ebnf
 * (* Deprecated in 0.6.0 *)
 * UnnamedFunctionDefinition = (* function_keyword: *) FUNCTION_KEYWORD
 *                             (* parameters: *) ParametersDeclaration
 *                             (* attributes: *) UnnamedFunctionAttributes
 *                             (* body: *) FunctionBody;
 * ```
 */
pub type UnnamedFunctionDefinition = Rc<UnnamedFunctionDefinitionStruct>;

#[derive(Debug)]
pub struct UnnamedFunctionDefinitionStruct {
    pub cursor: Cursor,
    pub function_keyword: Rc<TerminalNode>,
    pub parameters: ParametersDeclaration,
    pub attributes: UnnamedFunctionAttributes,
    pub body: FunctionBody,
}

/**
 * This node represents a `FallbackFunctionDefinition` nonterminal, with the following structure:
 *
 * ```ebnf
 * (* Introduced in 0.6.0 *)
 * FallbackFunctionDefinition = (* fallback_keyword: *) FALLBACK_KEYWORD
 *                              (* parameters: *) ParametersDeclaration
 *                              (* attributes: *) FallbackFunctionAttributes
 *                              (* returns: *) ReturnsDeclaration?
 *                              (* body: *) FunctionBody;
 * ```
 */
pub type FallbackFunctionDefinition = Rc<FallbackFunctionDefinitionStruct>;

#[derive(Debug)]
pub struct FallbackFunctionDefinitionStruct {
    pub cursor: Cursor,
    pub fallback_keyword: Rc<TerminalNode>,
    pub parameters: ParametersDeclaration,
    pub attributes: FallbackFunctionAttributes,
    pub returns: Option<ReturnsDeclaration>,
    pub body: FunctionBody,
}

/**
 * This node represents a `ReceiveFunctionDefinition` nonterminal, with the following structure:
 *
 * ```ebnf
 * (* Introduced in 0.6.0 *)
 * ReceiveFunctionDefinition = (* receive_keyword: *) RECEIVE_KEYWORD
 *                             (* parameters: *) ParametersDeclaration
 *                             (* attributes: *) ReceiveFunctionAttributes
 *                             (* body: *) FunctionBody;
 * ```
 */
pub type ReceiveFunctionDefinition = Rc<ReceiveFunctionDefinitionStruct>;

#[derive(Debug)]
pub struct ReceiveFunctionDefinitionStruct {
    pub cursor: Cursor,
    pub receive_keyword: Rc<TerminalNode>,
    pub parameters: ParametersDeclaration,
    pub attributes: ReceiveFunctionAttributes,
    pub body: FunctionBody,
}

/**
 * This node represents a `ModifierDefinition` nonterminal, with the following structure:
 *
 * ```ebnf
 * ModifierDefinition = (* modifier_keyword: *) MODIFIER_KEYWORD
 *                      (* name: *) IDENTIFIER
 *                      (* parameters: *) ParametersDeclaration?
 *                      (* attributes: *) ModifierAttributes
 *                      (* body: *) FunctionBody;
 * ```
 */
pub type ModifierDefinition = Rc<ModifierDefinitionStruct>;

#[derive(Debug)]
pub struct ModifierDefinitionStruct {
    pub cursor: Cursor,
    pub modifier_keyword: Rc<TerminalNode>,
    pub name: Rc<TerminalNode>,
    pub parameters: Option<ParametersDeclaration>,
    pub attributes: ModifierAttributes,
    pub body: FunctionBody,
}

/**
 * This node represents a `ModifierInvocation` nonterminal, with the following structure:
 *
 * ```ebnf
 * ModifierInvocation = (* name: *) IdentifierPath
 *                      (* arguments: *) ArgumentsDeclaration?;
 * ```
 */
pub type ModifierInvocation = Rc<ModifierInvocationStruct>;

#[derive(Debug)]
pub struct ModifierInvocationStruct {
    pub cursor: Cursor,
    pub name: IdentifierPath,
    pub arguments: Option<ArgumentsDeclaration>,
}

/**
 * This node represents a `EventDefinition` nonterminal, with the following structure:
 *
 * ```ebnf
 * EventDefinition = (* event_keyword: *) EVENT_KEYWORD
 *                   (* name: *) IDENTIFIER
 *                   (* parameters: *) EventParametersDeclaration
 *                   (* anonymous_keyword: *) ANONYMOUS_KEYWORD?
 *                   (* semicolon: *) SEMICOLON;
 * ```
 */
pub type EventDefinition = Rc<EventDefinitionStruct>;

#[derive(Debug)]
pub struct EventDefinitionStruct {
    pub cursor: Cursor,
    pub event_keyword: Rc<TerminalNode>,
    pub name: Rc<TerminalNode>,
    pub parameters: EventParametersDeclaration,
    pub anonymous_keyword: Option<Rc<TerminalNode>>,
    pub semicolon: Rc<TerminalNode>,
}

/**
 * This node represents a `EventParametersDeclaration` nonterminal, with the following structure:
 *
 * ```ebnf
 * EventParametersDeclaration = (* open_paren: *) OPEN_PAREN
 *                              (* parameters: *) EventParameters
 *                              (* close_paren: *) CLOSE_PAREN;
 * ```
 */
pub type EventParametersDeclaration = Rc<EventParametersDeclarationStruct>;

#[derive(Debug)]
pub struct EventParametersDeclarationStruct {
    pub cursor: Cursor,
    pub open_paren: Rc<TerminalNode>,
    pub parameters: EventParameters,
    pub close_paren: Rc<TerminalNode>,
}

/**
 * This node represents a `EventParameter` nonterminal, with the following structure:
 *
 * ```ebnf
 * EventParameter = (* type_name: *) TypeName
 *                  (* indexed_keyword: *) INDEXED_KEYWORD?
 *                  (* name: *) IDENTIFIER?;
 * ```
 */
pub type EventParameter = Rc<EventParameterStruct>;

#[derive(Debug)]
pub struct EventParameterStruct {
    pub cursor: Cursor,
    pub type_name: TypeName,
    pub indexed_keyword: Option<Rc<TerminalNode>>,
    pub name: Option<Rc<TerminalNode>>,
}

/**
 * This node represents a `UserDefinedValueTypeDefinition` nonterminal, with the following structure:
 *
 * ```ebnf
 * (* Introduced in 0.8.8 *)
 * UserDefinedValueTypeDefinition = (* type_keyword: *) TYPE_KEYWORD
 *                                  (* name: *) IDENTIFIER
 *                                  (* is_keyword: *) IS_KEYWORD
 *                                  (* value_type: *) ElementaryType
 *                                  (* semicolon: *) SEMICOLON;
 * ```
 */
pub type UserDefinedValueTypeDefinition = Rc<UserDefinedValueTypeDefinitionStruct>;

#[derive(Debug)]
pub struct UserDefinedValueTypeDefinitionStruct {
    pub cursor: Cursor,
    pub type_keyword: Rc<TerminalNode>,
    pub name: Rc<TerminalNode>,
    pub is_keyword: Rc<TerminalNode>,
    pub value_type: ElementaryType,
    pub semicolon: Rc<TerminalNode>,
}

/**
 * This node represents a `ErrorDefinition` nonterminal, with the following structure:
 *
 * ```ebnf
 * (* Introduced in 0.8.4 *)
 * ErrorDefinition = (* error_keyword: *) ERROR_KEYWORD
 *                   (* name: *) IDENTIFIER
 *                   (* members: *) ErrorParametersDeclaration
 *                   (* semicolon: *) SEMICOLON;
 * ```
 */
pub type ErrorDefinition = Rc<ErrorDefinitionStruct>;

#[derive(Debug)]
pub struct ErrorDefinitionStruct {
    pub cursor: Cursor,
    pub error_keyword: Rc<TerminalNode>,
    pub name: Rc<TerminalNode>,
    pub members: ErrorParametersDeclaration,
    pub semicolon: Rc<TerminalNode>,
}

/**
 * This node represents a `ErrorParametersDeclaration` nonterminal, with the following structure:
 *
 * ```ebnf
 * (* Introduced in 0.8.4 *)
 * ErrorParametersDeclaration = (* open_paren: *) OPEN_PAREN
 *                              (* parameters: *) ErrorParameters
 *                              (* close_paren: *) CLOSE_PAREN;
 * ```
 */
pub type ErrorParametersDeclaration = Rc<ErrorParametersDeclarationStruct>;

#[derive(Debug)]
pub struct ErrorParametersDeclarationStruct {
    pub cursor: Cursor,
    pub open_paren: Rc<TerminalNode>,
    pub parameters: ErrorParameters,
    pub close_paren: Rc<TerminalNode>,
}

/**
 * This node represents a `ErrorParameter` nonterminal, with the following structure:
 *
 * ```ebnf
 * (* Introduced in 0.8.4 *)
 * ErrorParameter = (* type_name: *) TypeName
 *                  (* name: *) IDENTIFIER?;
 * ```
 */
pub type ErrorParameter = Rc<ErrorParameterStruct>;

#[derive(Debug)]
pub struct ErrorParameterStruct {
    pub cursor: Cursor,
    pub type_name: TypeName,
    pub name: Option<Rc<TerminalNode>>,
}

/**
 * This node represents a `ArrayTypeName` nonterminal, with the following structure:
 *
 * ```ebnf
 * (* Postfix unary operator *)
 * ArrayTypeName = (* operand: *) TypeName
 *                 (* open_bracket: *) OPEN_BRACKET
 *                 (* index: *) Expression?
 *                 (* close_bracket: *) CLOSE_BRACKET;
 * ```
 */
pub type ArrayTypeName = Rc<ArrayTypeNameStruct>;

#[derive(Debug)]
pub struct ArrayTypeNameStruct {
    pub cursor: Cursor,
    pub operand: TypeName,
    pub open_bracket: Rc<TerminalNode>,
    pub index: Option<Expression>,
    pub close_bracket: Rc<TerminalNode>,
}

/**
 * This node represents a `FunctionType` nonterminal, with the following structure:
 *
 * ```ebnf
 * FunctionType = (* function_keyword: *) FUNCTION_KEYWORD
 *                (* parameters: *) ParametersDeclaration
 *                (* attributes: *) FunctionTypeAttributes
 *                (* returns: *) ReturnsDeclaration?;
 * ```
 */
pub type FunctionType = Rc<FunctionTypeStruct>;

#[derive(Debug)]
pub struct FunctionTypeStruct {
    pub cursor: Cursor,
    pub function_keyword: Rc<TerminalNode>,
    pub parameters: ParametersDeclaration,
    pub attributes: FunctionTypeAttributes,
    pub returns: Option<ReturnsDeclaration>,
}

/**
 * This node represents a `MappingType` nonterminal, with the following structure:
 *
 * ```ebnf
 * MappingType = (* mapping_keyword: *) MAPPING_KEYWORD
 *               (* open_paren: *) OPEN_PAREN
 *               (* key_type: *) MappingKey
 *               (* equal_greater_than: *) EQUAL_GREATER_THAN
 *               (* value_type: *) MappingValue
 *               (* close_paren: *) CLOSE_PAREN;
 * ```
 */
pub type MappingType = Rc<MappingTypeStruct>;

#[derive(Debug)]
pub struct MappingTypeStruct {
    pub cursor: Cursor,
    pub mapping_keyword: Rc<TerminalNode>,
    pub open_paren: Rc<TerminalNode>,
    pub key_type: MappingKey,
    pub equal_greater_than: Rc<TerminalNode>,
    pub value_type: MappingValue,
    pub close_paren: Rc<TerminalNode>,
}

/**
 * This node represents a `MappingKey` nonterminal, with the following structure:
 *
 * ```ebnf
 * MappingKey = (* key_type: *) MappingKeyType
 *              (* name: *) IDENTIFIER?; (* Introduced in 0.8.18 *)
 * ```
 */
pub type MappingKey = Rc<MappingKeyStruct>;

#[derive(Debug)]
pub struct MappingKeyStruct {
    pub cursor: Cursor,
    pub key_type: MappingKeyType,
    pub name: Option<Rc<TerminalNode>>,
}

/**
 * This node represents a `MappingValue` nonterminal, with the following structure:
 *
 * ```ebnf
 * MappingValue = (* type_name: *) TypeName
 *                (* name: *) IDENTIFIER?; (* Introduced in 0.8.18 *)
 * ```
 */
pub type MappingValue = Rc<MappingValueStruct>;

#[derive(Debug)]
pub struct MappingValueStruct {
    pub cursor: Cursor,
    pub type_name: TypeName,
    pub name: Option<Rc<TerminalNode>>,
}

/**
 * This node represents a `AddressType` nonterminal, with the following structure:
 *
 * ```ebnf
 * AddressType = (* address_keyword: *) ADDRESS_KEYWORD
 *               (* payable_keyword: *) PAYABLE_KEYWORD?; (* Introduced in 0.5.0 *)
 * ```
 */
pub type AddressType = Rc<AddressTypeStruct>;

#[derive(Debug)]
pub struct AddressTypeStruct {
    pub cursor: Cursor,
    pub address_keyword: Rc<TerminalNode>,
    pub payable_keyword: Option<Rc<TerminalNode>>,
}

/**
 * This node represents a `Block` nonterminal, with the following structure:
 *
 * ```ebnf
 * Block = (* open_brace: *) OPEN_BRACE
 *         (* statements: *) Statements
 *         (* close_brace: *) CLOSE_BRACE;
 * ```
 */
pub type Block = Rc<BlockStruct>;

#[derive(Debug)]
pub struct BlockStruct {
    pub cursor: Cursor,
    pub open_brace: Rc<TerminalNode>,
    pub statements: Statements,
    pub close_brace: Rc<TerminalNode>,
}

/**
 * This node represents a `UncheckedBlock` nonterminal, with the following structure:
 *
 * ```ebnf
 * (* Introduced in 0.8.0 *)
 * UncheckedBlock = (* unchecked_keyword: *) UNCHECKED_KEYWORD
 *                  (* block: *) Block;
 * ```
 */
pub type UncheckedBlock = Rc<UncheckedBlockStruct>;

#[derive(Debug)]
pub struct UncheckedBlockStruct {
    pub cursor: Cursor,
    pub unchecked_keyword: Rc<TerminalNode>,
    pub block: Block,
}

/**
 * This node represents a `ExpressionStatement` nonterminal, with the following structure:
 *
 * ```ebnf
 * ExpressionStatement = (* expression: *) Expression
 *                       (* semicolon: *) SEMICOLON;
 * ```
 */
pub type ExpressionStatement = Rc<ExpressionStatementStruct>;

#[derive(Debug)]
pub struct ExpressionStatementStruct {
    pub cursor: Cursor,
    pub expression: Expression,
    pub semicolon: Rc<TerminalNode>,
}

/**
 * This node represents a `AssemblyStatement` nonterminal, with the following structure:
 *
 * ```ebnf
 * AssemblyStatement = (* assembly_keyword: *) ASSEMBLY_KEYWORD
 *                     (* label: *) StringLiteral?
 *                     (* flags: *) AssemblyFlagsDeclaration? (* Introduced in 0.8.13 *)
 *                     (* body: *) YulBlock;
 * ```
 */
pub type AssemblyStatement = Rc<AssemblyStatementStruct>;

#[derive(Debug)]
pub struct AssemblyStatementStruct {
    pub cursor: Cursor,
    pub assembly_keyword: Rc<TerminalNode>,
    pub label: Option<StringLiteral>,
    pub flags: Option<AssemblyFlagsDeclaration>,
    pub body: YulBlock,
}

/**
 * This node represents a `AssemblyFlagsDeclaration` nonterminal, with the following structure:
 *
 * ```ebnf
 * (* Introduced in 0.8.13 *)
 * AssemblyFlagsDeclaration = (* open_paren: *) OPEN_PAREN
 *                            (* flags: *) AssemblyFlags
 *                            (* close_paren: *) CLOSE_PAREN;
 * ```
 */
pub type AssemblyFlagsDeclaration = Rc<AssemblyFlagsDeclarationStruct>;

#[derive(Debug)]
pub struct AssemblyFlagsDeclarationStruct {
    pub cursor: Cursor,
    pub open_paren: Rc<TerminalNode>,
    pub flags: AssemblyFlags,
    pub close_paren: Rc<TerminalNode>,
}

/**
 * This node represents a `TupleDeconstructionStatement` nonterminal, with the following structure:
 *
 * ```ebnf
 * TupleDeconstructionStatement = (* var_keyword: *) VAR_KEYWORD? (* Deprecated in 0.5.0 *)
 *                                (* open_paren: *) OPEN_PAREN
 *                                (* elements: *) TupleDeconstructionElements
 *                                (* close_paren: *) CLOSE_PAREN
 *                                (* equal: *) EQUAL
 *                                (* expression: *) Expression
 *                                (* semicolon: *) SEMICOLON;
 * ```
 */
pub type TupleDeconstructionStatement = Rc<TupleDeconstructionStatementStruct>;

#[derive(Debug)]
pub struct TupleDeconstructionStatementStruct {
    pub cursor: Cursor,
    pub var_keyword: Option<Rc<TerminalNode>>,
    pub open_paren: Rc<TerminalNode>,
    pub elements: TupleDeconstructionElements,
    pub close_paren: Rc<TerminalNode>,
    pub equal: Rc<TerminalNode>,
    pub expression: Expression,
    pub semicolon: Rc<TerminalNode>,
}

/**
 * This node represents a `TupleDeconstructionElement` nonterminal, with the following structure:
 *
 * ```ebnf
 * TupleDeconstructionElement = (* member: *) TupleMember?;
 * ```
 */
pub type TupleDeconstructionElement = Rc<TupleDeconstructionElementStruct>;

#[derive(Debug)]
pub struct TupleDeconstructionElementStruct {
    pub cursor: Cursor,
    pub member: Option<TupleMember>,
}

/**
 * This node represents a `TypedTupleMember` nonterminal, with the following structure:
 *
 * ```ebnf
 * TypedTupleMember = (* type_name: *) TypeName
 *                    (* storage_location: *) StorageLocation?
 *                    (* name: *) IDENTIFIER;
 * ```
 */
pub type TypedTupleMember = Rc<TypedTupleMemberStruct>;

#[derive(Debug)]
pub struct TypedTupleMemberStruct {
    pub cursor: Cursor,
    pub type_name: TypeName,
    pub storage_location: Option<StorageLocation>,
    pub name: Rc<TerminalNode>,
}

/**
 * This node represents a `UntypedTupleMember` nonterminal, with the following structure:
 *
 * ```ebnf
 * UntypedTupleMember = (* storage_location: *) StorageLocation?
 *                      (* name: *) IDENTIFIER;
 * ```
 */
pub type UntypedTupleMember = Rc<UntypedTupleMemberStruct>;

#[derive(Debug)]
pub struct UntypedTupleMemberStruct {
    pub cursor: Cursor,
    pub storage_location: Option<StorageLocation>,
    pub name: Rc<TerminalNode>,
}

/**
 * This node represents a `VariableDeclarationStatement` nonterminal, with the following structure:
 *
 * ```ebnf
 * VariableDeclarationStatement = (* variable_type: *) VariableDeclarationType
 *                                (* storage_location: *) StorageLocation?
 *                                (* name: *) IDENTIFIER
 *                                (* value: *) VariableDeclarationValue?
 *                                (* semicolon: *) SEMICOLON;
 * ```
 */
pub type VariableDeclarationStatement = Rc<VariableDeclarationStatementStruct>;

#[derive(Debug)]
pub struct VariableDeclarationStatementStruct {
    pub cursor: Cursor,
    pub variable_type: VariableDeclarationType,
    pub storage_location: Option<StorageLocation>,
    pub name: Rc<TerminalNode>,
    pub value: Option<VariableDeclarationValue>,
    pub semicolon: Rc<TerminalNode>,
}

/**
 * This node represents a `VariableDeclarationValue` nonterminal, with the following structure:
 *
 * ```ebnf
 * VariableDeclarationValue = (* equal: *) EQUAL
 *                            (* expression: *) Expression;
 * ```
 */
pub type VariableDeclarationValue = Rc<VariableDeclarationValueStruct>;

#[derive(Debug)]
pub struct VariableDeclarationValueStruct {
    pub cursor: Cursor,
    pub equal: Rc<TerminalNode>,
    pub expression: Expression,
}

/**
 * This node represents a `IfStatement` nonterminal, with the following structure:
 *
 * ```ebnf
 * IfStatement = (* if_keyword: *) IF_KEYWORD
 *               (* open_paren: *) OPEN_PAREN
 *               (* condition: *) Expression
 *               (* close_paren: *) CLOSE_PAREN
 *               (* body: *) Statement
 *               (* else_branch: *) ElseBranch?;
 * ```
 */
pub type IfStatement = Rc<IfStatementStruct>;

#[derive(Debug)]
pub struct IfStatementStruct {
    pub cursor: Cursor,
    pub if_keyword: Rc<TerminalNode>,
    pub open_paren: Rc<TerminalNode>,
    pub condition: Expression,
    pub close_paren: Rc<TerminalNode>,
    pub body: Statement,
    pub else_branch: Option<ElseBranch>,
}

/**
 * This node represents a `ElseBranch` nonterminal, with the following structure:
 *
 * ```ebnf
 * ElseBranch = (* else_keyword: *) ELSE_KEYWORD
 *              (* body: *) Statement;
 * ```
 */
pub type ElseBranch = Rc<ElseBranchStruct>;

#[derive(Debug)]
pub struct ElseBranchStruct {
    pub cursor: Cursor,
    pub else_keyword: Rc<TerminalNode>,
    pub body: Statement,
}

/**
 * This node represents a `ForStatement` nonterminal, with the following structure:
 *
 * ```ebnf
 * ForStatement = (* for_keyword: *) FOR_KEYWORD
 *                (* open_paren: *) OPEN_PAREN
 *                (* initialization: *) ForStatementInitialization
 *                (* condition: *) ForStatementCondition
 *                (* iterator: *) Expression?
 *                (* close_paren: *) CLOSE_PAREN
 *                (* body: *) Statement;
 * ```
 */
pub type ForStatement = Rc<ForStatementStruct>;

#[derive(Debug)]
pub struct ForStatementStruct {
    pub cursor: Cursor,
    pub for_keyword: Rc<TerminalNode>,
    pub open_paren: Rc<TerminalNode>,
    pub initialization: ForStatementInitialization,
    pub condition: ForStatementCondition,
    pub iterator: Option<Expression>,
    pub close_paren: Rc<TerminalNode>,
    pub body: Statement,
}

/**
 * This node represents a `WhileStatement` nonterminal, with the following structure:
 *
 * ```ebnf
 * WhileStatement = (* while_keyword: *) WHILE_KEYWORD
 *                  (* open_paren: *) OPEN_PAREN
 *                  (* condition: *) Expression
 *                  (* close_paren: *) CLOSE_PAREN
 *                  (* body: *) Statement;
 * ```
 */
pub type WhileStatement = Rc<WhileStatementStruct>;

#[derive(Debug)]
pub struct WhileStatementStruct {
    pub cursor: Cursor,
    pub while_keyword: Rc<TerminalNode>,
    pub open_paren: Rc<TerminalNode>,
    pub condition: Expression,
    pub close_paren: Rc<TerminalNode>,
    pub body: Statement,
}

/**
 * This node represents a `DoWhileStatement` nonterminal, with the following structure:
 *
 * ```ebnf
 * DoWhileStatement = (* do_keyword: *) DO_KEYWORD
 *                    (* body: *) Statement
 *                    (* while_keyword: *) WHILE_KEYWORD
 *                    (* open_paren: *) OPEN_PAREN
 *                    (* condition: *) Expression
 *                    (* close_paren: *) CLOSE_PAREN
 *                    (* semicolon: *) SEMICOLON;
 * ```
 */
pub type DoWhileStatement = Rc<DoWhileStatementStruct>;

#[derive(Debug)]
pub struct DoWhileStatementStruct {
    pub cursor: Cursor,
    pub do_keyword: Rc<TerminalNode>,
    pub body: Statement,
    pub while_keyword: Rc<TerminalNode>,
    pub open_paren: Rc<TerminalNode>,
    pub condition: Expression,
    pub close_paren: Rc<TerminalNode>,
    pub semicolon: Rc<TerminalNode>,
}

/**
 * This node represents a `ContinueStatement` nonterminal, with the following structure:
 *
 * ```ebnf
 * ContinueStatement = (* continue_keyword: *) CONTINUE_KEYWORD
 *                     (* semicolon: *) SEMICOLON;
 * ```
 */
pub type ContinueStatement = Rc<ContinueStatementStruct>;

#[derive(Debug)]
pub struct ContinueStatementStruct {
    pub cursor: Cursor,
    pub continue_keyword: Rc<TerminalNode>,
    pub semicolon: Rc<TerminalNode>,
}

/**
 * This node represents a `BreakStatement` nonterminal, with the following structure:
 *
 * ```ebnf
 * BreakStatement = (* break_keyword: *) BREAK_KEYWORD
 *                  (* semicolon: *) SEMICOLON;
 * ```
 */
pub type BreakStatement = Rc<BreakStatementStruct>;

#[derive(Debug)]
pub struct BreakStatementStruct {
    pub cursor: Cursor,
    pub break_keyword: Rc<TerminalNode>,
    pub semicolon: Rc<TerminalNode>,
}

/**
 * This node represents a `ReturnStatement` nonterminal, with the following structure:
 *
 * ```ebnf
 * ReturnStatement = (* return_keyword: *) RETURN_KEYWORD
 *                   (* expression: *) Expression?
 *                   (* semicolon: *) SEMICOLON;
 * ```
 */
pub type ReturnStatement = Rc<ReturnStatementStruct>;

#[derive(Debug)]
pub struct ReturnStatementStruct {
    pub cursor: Cursor,
    pub return_keyword: Rc<TerminalNode>,
    pub expression: Option<Expression>,
    pub semicolon: Rc<TerminalNode>,
}

/**
 * This node represents a `EmitStatement` nonterminal, with the following structure:
 *
 * ```ebnf
 * (* Introduced in 0.4.21 *)
 * EmitStatement = (* emit_keyword: *) EMIT_KEYWORD
 *                 (* event: *) IdentifierPath
 *                 (* arguments: *) ArgumentsDeclaration
 *                 (* semicolon: *) SEMICOLON;
 * ```
 */
pub type EmitStatement = Rc<EmitStatementStruct>;

#[derive(Debug)]
pub struct EmitStatementStruct {
    pub cursor: Cursor,
    pub emit_keyword: Rc<TerminalNode>,
    pub event: IdentifierPath,
    pub arguments: ArgumentsDeclaration,
    pub semicolon: Rc<TerminalNode>,
}

/**
 * This node represents a `TryStatement` nonterminal, with the following structure:
 *
 * ```ebnf
 * (* Introduced in 0.6.0 *)
 * TryStatement = (* try_keyword: *) TRY_KEYWORD
 *                (* expression: *) Expression
 *                (* returns: *) ReturnsDeclaration?
 *                (* body: *) Block
 *                (* catch_clauses: *) CatchClauses;
 * ```
 */
pub type TryStatement = Rc<TryStatementStruct>;

#[derive(Debug)]
pub struct TryStatementStruct {
    pub cursor: Cursor,
    pub try_keyword: Rc<TerminalNode>,
    pub expression: Expression,
    pub returns: Option<ReturnsDeclaration>,
    pub body: Block,
    pub catch_clauses: CatchClauses,
}

/**
 * This node represents a `CatchClause` nonterminal, with the following structure:
 *
 * ```ebnf
 * (* Introduced in 0.6.0 *)
 * CatchClause = (* catch_keyword: *) CATCH_KEYWORD
 *               (* error: *) CatchClauseError?
 *               (* body: *) Block;
 * ```
 */
pub type CatchClause = Rc<CatchClauseStruct>;

#[derive(Debug)]
pub struct CatchClauseStruct {
    pub cursor: Cursor,
    pub catch_keyword: Rc<TerminalNode>,
    pub error: Option<CatchClauseError>,
    pub body: Block,
}

/**
 * This node represents a `CatchClauseError` nonterminal, with the following structure:
 *
 * ```ebnf
 * (* Introduced in 0.6.0 *)
 * CatchClauseError = (* name: *) IDENTIFIER?
 *                    (* parameters: *) ParametersDeclaration;
 * ```
 */
pub type CatchClauseError = Rc<CatchClauseErrorStruct>;

#[derive(Debug)]
pub struct CatchClauseErrorStruct {
    pub cursor: Cursor,
    pub name: Option<Rc<TerminalNode>>,
    pub parameters: ParametersDeclaration,
}

/**
 * This node represents a `RevertStatement` nonterminal, with the following structure:
 *
 * ```ebnf
 * (* Introduced in 0.8.4 *)
 * RevertStatement = (* revert_keyword: *) REVERT_KEYWORD
 *                   (* error: *) IdentifierPath?
 *                   (* arguments: *) ArgumentsDeclaration
 *                   (* semicolon: *) SEMICOLON;
 * ```
 */
pub type RevertStatement = Rc<RevertStatementStruct>;

#[derive(Debug)]
pub struct RevertStatementStruct {
    pub cursor: Cursor,
    pub revert_keyword: Rc<TerminalNode>,
    pub error: Option<IdentifierPath>,
    pub arguments: ArgumentsDeclaration,
    pub semicolon: Rc<TerminalNode>,
}

/**
 * This node represents a `ThrowStatement` nonterminal, with the following structure:
 *
 * ```ebnf
 * (* Deprecated in 0.5.0 *)
 * ThrowStatement = (* throw_keyword: *) THROW_KEYWORD
 *                  (* semicolon: *) SEMICOLON;
 * ```
 */
pub type ThrowStatement = Rc<ThrowStatementStruct>;

#[derive(Debug)]
pub struct ThrowStatementStruct {
    pub cursor: Cursor,
    pub throw_keyword: Rc<TerminalNode>,
    pub semicolon: Rc<TerminalNode>,
}

/**
 * This node represents a `AssignmentExpression` nonterminal, with the following structure:
 *
 * ```ebnf
 * (* Left-associative binary operator *)
 * AssignmentExpression = (* left_operand: *) Expression
 *                        (* operator: *) EQUAL
 *                        (* right_operand: *) Expression;
 *
 * (* Left-associative binary operator *)
 * AssignmentExpression = (* left_operand: *) Expression
 *                        (* operator: *) BAR_EQUAL
 *                        (* right_operand: *) Expression;
 *
 * (* Left-associative binary operator *)
 * AssignmentExpression = (* left_operand: *) Expression
 *                        (* operator: *) PLUS_EQUAL
 *                        (* right_operand: *) Expression;
 *
 * (* Left-associative binary operator *)
 * AssignmentExpression = (* left_operand: *) Expression
 *                        (* operator: *) MINUS_EQUAL
 *                        (* right_operand: *) Expression;
 *
 * (* Left-associative binary operator *)
 * AssignmentExpression = (* left_operand: *) Expression
 *                        (* operator: *) CARET_EQUAL
 *                        (* right_operand: *) Expression;
 *
 * (* Left-associative binary operator *)
 * AssignmentExpression = (* left_operand: *) Expression
 *                        (* operator: *) SLASH_EQUAL
 *                        (* right_operand: *) Expression;
 *
 * (* Left-associative binary operator *)
 * AssignmentExpression = (* left_operand: *) Expression
 *                        (* operator: *) PERCENT_EQUAL
 *                        (* right_operand: *) Expression;
 *
 * (* Left-associative binary operator *)
 * AssignmentExpression = (* left_operand: *) Expression
 *                        (* operator: *) ASTERISK_EQUAL
 *                        (* right_operand: *) Expression;
 *
 * (* Left-associative binary operator *)
 * AssignmentExpression = (* left_operand: *) Expression
 *                        (* operator: *) AMPERSAND_EQUAL
 *                        (* right_operand: *) Expression;
 *
 * (* Left-associative binary operator *)
 * AssignmentExpression = (* left_operand: *) Expression
 *                        (* operator: *) LESS_THAN_LESS_THAN_EQUAL
 *                        (* right_operand: *) Expression;
 *
 * (* Left-associative binary operator *)
 * AssignmentExpression = (* left_operand: *) Expression
 *                        (* operator: *) GREATER_THAN_GREATER_THAN_EQUAL
 *                        (* right_operand: *) Expression;
 *
 * (* Left-associative binary operator *)
 * AssignmentExpression = (* left_operand: *) Expression
 *                        (* operator: *) GREATER_THAN_GREATER_THAN_GREATER_THAN_EQUAL
 *                        (* right_operand: *) Expression;
 * ```
 */
pub type AssignmentExpression = Rc<AssignmentExpressionStruct>;

#[derive(Debug)]
pub struct AssignmentExpressionStruct {
    pub cursor: Cursor,
    pub left_operand: Expression,
    pub operator: Rc<TerminalNode>,
    pub right_operand: Expression,
}

/**
 * This node represents a `ConditionalExpression` nonterminal, with the following structure:
 *
 * ```ebnf
 * (* Postfix unary operator *)
 * ConditionalExpression = (* operand: *) Expression
 *                         (* question_mark: *) QUESTION_MARK
 *                         (* true_expression: *) Expression
 *                         (* colon: *) COLON
 *                         (* false_expression: *) Expression;
 * ```
 */
pub type ConditionalExpression = Rc<ConditionalExpressionStruct>;

#[derive(Debug)]
pub struct ConditionalExpressionStruct {
    pub cursor: Cursor,
    pub operand: Expression,
    pub question_mark: Rc<TerminalNode>,
    pub true_expression: Expression,
    pub colon: Rc<TerminalNode>,
    pub false_expression: Expression,
}

/**
 * This node represents a `OrExpression` nonterminal, with the following structure:
 *
 * ```ebnf
 * (* Left-associative binary operator *)
 * OrExpression = (* left_operand: *) Expression
 *                (* operator: *) BAR_BAR
 *                (* right_operand: *) Expression;
 * ```
 */
pub type OrExpression = Rc<OrExpressionStruct>;

#[derive(Debug)]
pub struct OrExpressionStruct {
    pub cursor: Cursor,
    pub left_operand: Expression,
    pub operator: Rc<TerminalNode>,
    pub right_operand: Expression,
}

/**
 * This node represents a `AndExpression` nonterminal, with the following structure:
 *
 * ```ebnf
 * (* Left-associative binary operator *)
 * AndExpression = (* left_operand: *) Expression
 *                 (* operator: *) AMPERSAND_AMPERSAND
 *                 (* right_operand: *) Expression;
 * ```
 */
pub type AndExpression = Rc<AndExpressionStruct>;

#[derive(Debug)]
pub struct AndExpressionStruct {
    pub cursor: Cursor,
    pub left_operand: Expression,
    pub operator: Rc<TerminalNode>,
    pub right_operand: Expression,
}

/**
 * This node represents a `EqualityExpression` nonterminal, with the following structure:
 *
 * ```ebnf
 * (* Left-associative binary operator *)
 * EqualityExpression = (* left_operand: *) Expression
 *                      (* operator: *) EQUAL_EQUAL
 *                      (* right_operand: *) Expression;
 *
 * (* Left-associative binary operator *)
 * EqualityExpression = (* left_operand: *) Expression
 *                      (* operator: *) BANG_EQUAL
 *                      (* right_operand: *) Expression;
 * ```
 */
pub type EqualityExpression = Rc<EqualityExpressionStruct>;

#[derive(Debug)]
pub struct EqualityExpressionStruct {
    pub cursor: Cursor,
    pub left_operand: Expression,
    pub operator: Rc<TerminalNode>,
    pub right_operand: Expression,
}

/**
 * This node represents a `InequalityExpression` nonterminal, with the following structure:
 *
 * ```ebnf
 * (* Left-associative binary operator *)
 * InequalityExpression = (* left_operand: *) Expression
 *                        (* operator: *) LESS_THAN
 *                        (* right_operand: *) Expression;
 *
 * (* Left-associative binary operator *)
 * InequalityExpression = (* left_operand: *) Expression
 *                        (* operator: *) GREATER_THAN
 *                        (* right_operand: *) Expression;
 *
 * (* Left-associative binary operator *)
 * InequalityExpression = (* left_operand: *) Expression
 *                        (* operator: *) LESS_THAN_EQUAL
 *                        (* right_operand: *) Expression;
 *
 * (* Left-associative binary operator *)
 * InequalityExpression = (* left_operand: *) Expression
 *                        (* operator: *) GREATER_THAN_EQUAL
 *                        (* right_operand: *) Expression;
 * ```
 */
pub type InequalityExpression = Rc<InequalityExpressionStruct>;

#[derive(Debug)]
pub struct InequalityExpressionStruct {
    pub cursor: Cursor,
    pub left_operand: Expression,
    pub operator: Rc<TerminalNode>,
    pub right_operand: Expression,
}

/**
 * This node represents a `BitwiseOrExpression` nonterminal, with the following structure:
 *
 * ```ebnf
 * (* Left-associative binary operator *)
 * BitwiseOrExpression = (* left_operand: *) Expression
 *                       (* operator: *) BAR
 *                       (* right_operand: *) Expression;
 * ```
 */
pub type BitwiseOrExpression = Rc<BitwiseOrExpressionStruct>;

#[derive(Debug)]
pub struct BitwiseOrExpressionStruct {
    pub cursor: Cursor,
    pub left_operand: Expression,
    pub operator: Rc<TerminalNode>,
    pub right_operand: Expression,
}

/**
 * This node represents a `BitwiseXorExpression` nonterminal, with the following structure:
 *
 * ```ebnf
 * (* Left-associative binary operator *)
 * BitwiseXorExpression = (* left_operand: *) Expression
 *                        (* operator: *) CARET
 *                        (* right_operand: *) Expression;
 * ```
 */
pub type BitwiseXorExpression = Rc<BitwiseXorExpressionStruct>;

#[derive(Debug)]
pub struct BitwiseXorExpressionStruct {
    pub cursor: Cursor,
    pub left_operand: Expression,
    pub operator: Rc<TerminalNode>,
    pub right_operand: Expression,
}

/**
 * This node represents a `BitwiseAndExpression` nonterminal, with the following structure:
 *
 * ```ebnf
 * (* Left-associative binary operator *)
 * BitwiseAndExpression = (* left_operand: *) Expression
 *                        (* operator: *) AMPERSAND
 *                        (* right_operand: *) Expression;
 * ```
 */
pub type BitwiseAndExpression = Rc<BitwiseAndExpressionStruct>;

#[derive(Debug)]
pub struct BitwiseAndExpressionStruct {
    pub cursor: Cursor,
    pub left_operand: Expression,
    pub operator: Rc<TerminalNode>,
    pub right_operand: Expression,
}

/**
 * This node represents a `ShiftExpression` nonterminal, with the following structure:
 *
 * ```ebnf
 * (* Left-associative binary operator *)
 * ShiftExpression = (* left_operand: *) Expression
 *                   (* operator: *) LESS_THAN_LESS_THAN
 *                   (* right_operand: *) Expression;
 *
 * (* Left-associative binary operator *)
 * ShiftExpression = (* left_operand: *) Expression
 *                   (* operator: *) GREATER_THAN_GREATER_THAN
 *                   (* right_operand: *) Expression;
 *
 * (* Left-associative binary operator *)
 * ShiftExpression = (* left_operand: *) Expression
 *                   (* operator: *) GREATER_THAN_GREATER_THAN_GREATER_THAN
 *                   (* right_operand: *) Expression;
 * ```
 */
pub type ShiftExpression = Rc<ShiftExpressionStruct>;

#[derive(Debug)]
pub struct ShiftExpressionStruct {
    pub cursor: Cursor,
    pub left_operand: Expression,
    pub operator: Rc<TerminalNode>,
    pub right_operand: Expression,
}

/**
 * This node represents a `AdditiveExpression` nonterminal, with the following structure:
 *
 * ```ebnf
 * (* Left-associative binary operator *)
 * AdditiveExpression = (* left_operand: *) Expression
 *                      (* operator: *) PLUS
 *                      (* right_operand: *) Expression;
 *
 * (* Left-associative binary operator *)
 * AdditiveExpression = (* left_operand: *) Expression
 *                      (* operator: *) MINUS
 *                      (* right_operand: *) Expression;
 * ```
 */
pub type AdditiveExpression = Rc<AdditiveExpressionStruct>;

#[derive(Debug)]
pub struct AdditiveExpressionStruct {
    pub cursor: Cursor,
    pub left_operand: Expression,
    pub operator: Rc<TerminalNode>,
    pub right_operand: Expression,
}

/**
 * This node represents a `MultiplicativeExpression` nonterminal, with the following structure:
 *
 * ```ebnf
 * (* Left-associative binary operator *)
 * MultiplicativeExpression = (* left_operand: *) Expression
 *                            (* operator: *) ASTERISK
 *                            (* right_operand: *) Expression;
 *
 * (* Left-associative binary operator *)
 * MultiplicativeExpression = (* left_operand: *) Expression
 *                            (* operator: *) SLASH
 *                            (* right_operand: *) Expression;
 *
 * (* Left-associative binary operator *)
 * MultiplicativeExpression = (* left_operand: *) Expression
 *                            (* operator: *) PERCENT
 *                            (* right_operand: *) Expression;
 * ```
 */
pub type MultiplicativeExpression = Rc<MultiplicativeExpressionStruct>;

#[derive(Debug)]
pub struct MultiplicativeExpressionStruct {
    pub cursor: Cursor,
    pub left_operand: Expression,
    pub operator: Rc<TerminalNode>,
    pub right_operand: Expression,
}

/**
 * This node represents a `ExponentiationExpression` nonterminal, with the following structure:
 *
 * ```ebnf
 * (* Left-associative binary operator *)
 * (* Deprecated in 0.8.0 *)
 * ExponentiationExpression = (* left_operand: *) Expression
 *                            (* operator: *) ASTERISK_ASTERISK
 *                            (* right_operand: *) Expression;
 *
 * (* Right-associative binary operator *)
 * (* Introduced in 0.8.0 *)
 * ExponentiationExpression = (* left_operand: *) Expression
 *                            (* operator: *) ASTERISK_ASTERISK
 *                            (* right_operand: *) Expression;
 * ```
 */
pub type ExponentiationExpression = Rc<ExponentiationExpressionStruct>;

#[derive(Debug)]
pub struct ExponentiationExpressionStruct {
    pub cursor: Cursor,
    pub left_operand: Expression,
    pub operator: Rc<TerminalNode>,
    pub right_operand: Expression,
}

/**
 * This node represents a `PostfixExpression` nonterminal, with the following structure:
 *
 * ```ebnf
 * (* Postfix unary operator *)
 * PostfixExpression = (* operand: *) Expression
 *                     (* operator: *) PLUS_PLUS;
 *
 * (* Postfix unary operator *)
 * PostfixExpression = (* operand: *) Expression
 *                     (* operator: *) MINUS_MINUS;
 * ```
 */
pub type PostfixExpression = Rc<PostfixExpressionStruct>;

#[derive(Debug)]
pub struct PostfixExpressionStruct {
    pub cursor: Cursor,
    pub operand: Expression,
    pub operator: Rc<TerminalNode>,
}

/**
 * This node represents a `PrefixExpression` nonterminal, with the following structure:
 *
 * ```ebnf
 * (* Prefix unary operator *)
 * PrefixExpression = (* operator: *) PLUS_PLUS
 *                    (* operand: *) Expression;
 *
 * (* Prefix unary operator *)
 * PrefixExpression = (* operator: *) MINUS_MINUS
 *                    (* operand: *) Expression;
 *
 * (* Prefix unary operator *)
 * PrefixExpression = (* operator: *) TILDE
 *                    (* operand: *) Expression;
 *
 * (* Prefix unary operator *)
 * PrefixExpression = (* operator: *) BANG
 *                    (* operand: *) Expression;
 *
 * (* Prefix unary operator *)
 * PrefixExpression = (* operator: *) MINUS
 *                    (* operand: *) Expression;
 *
 * (* Prefix unary operator *)
 * (* Deprecated in 0.5.0 *)
 * PrefixExpression = (* operator: *) PLUS
 *                    (* operand: *) Expression;
 *
 * (* Prefix unary operator *)
 * PrefixExpression = (* operator: *) DELETE_KEYWORD
 *                    (* operand: *) Expression;
 * ```
 */
pub type PrefixExpression = Rc<PrefixExpressionStruct>;

#[derive(Debug)]
pub struct PrefixExpressionStruct {
    pub cursor: Cursor,
    pub operator: Rc<TerminalNode>,
    pub operand: Expression,
}

/**
 * This node represents a `FunctionCallExpression` nonterminal, with the following structure:
 *
 * ```ebnf
 * (* Postfix unary operator *)
 * FunctionCallExpression = (* operand: *) Expression
 *                          (* arguments: *) ArgumentsDeclaration;
 * ```
 */
pub type FunctionCallExpression = Rc<FunctionCallExpressionStruct>;

#[derive(Debug)]
pub struct FunctionCallExpressionStruct {
    pub cursor: Cursor,
    pub operand: Expression,
    pub arguments: ArgumentsDeclaration,
}

/**
 * This node represents a `CallOptionsExpression` nonterminal, with the following structure:
 *
 * ```ebnf
 * (* Postfix unary operator *)
 * (* Introduced in 0.6.2 *)
 * CallOptionsExpression = (* operand: *) Expression
 *                         (* open_brace: *) OPEN_BRACE
 *                         (* options: *) CallOptions
 *                         (* close_brace: *) CLOSE_BRACE;
 * ```
 */
pub type CallOptionsExpression = Rc<CallOptionsExpressionStruct>;

#[derive(Debug)]
pub struct CallOptionsExpressionStruct {
    pub cursor: Cursor,
    pub operand: Expression,
    pub open_brace: Rc<TerminalNode>,
    pub options: CallOptions,
    pub close_brace: Rc<TerminalNode>,
}

/**
 * This node represents a `MemberAccessExpression` nonterminal, with the following structure:
 *
 * ```ebnf
 * (* Postfix unary operator *)
 * MemberAccessExpression = (* operand: *) Expression
 *                          (* period: *) PERIOD
 *                          (* member: *) IDENTIFIER;
 * ```
 */
pub type MemberAccessExpression = Rc<MemberAccessExpressionStruct>;

#[derive(Debug)]
pub struct MemberAccessExpressionStruct {
    pub cursor: Cursor,
    pub operand: Expression,
    pub period: Rc<TerminalNode>,
    pub member: Rc<TerminalNode>,
}

/**
 * This node represents a `IndexAccessExpression` nonterminal, with the following structure:
 *
 * ```ebnf
 * (* Postfix unary operator *)
 * IndexAccessExpression = (* operand: *) Expression
 *                         (* open_bracket: *) OPEN_BRACKET
 *                         (* start: *) Expression?
 *                         (* end: *) IndexAccessEnd?
 *                         (* close_bracket: *) CLOSE_BRACKET;
 * ```
 */
pub type IndexAccessExpression = Rc<IndexAccessExpressionStruct>;

#[derive(Debug)]
pub struct IndexAccessExpressionStruct {
    pub cursor: Cursor,
    pub operand: Expression,
    pub open_bracket: Rc<TerminalNode>,
    pub start: Option<Expression>,
    pub end: Option<IndexAccessEnd>,
    pub close_bracket: Rc<TerminalNode>,
}

/**
 * This node represents a `IndexAccessEnd` nonterminal, with the following structure:
 *
 * ```ebnf
 * IndexAccessEnd = (* colon: *) COLON
 *                  (* end: *) Expression?;
 * ```
 */
pub type IndexAccessEnd = Rc<IndexAccessEndStruct>;

#[derive(Debug)]
pub struct IndexAccessEndStruct {
    pub cursor: Cursor,
    pub colon: Rc<TerminalNode>,
    pub end: Option<Expression>,
}

/**
 * This node represents a `PositionalArgumentsDeclaration` nonterminal, with the following structure:
 *
 * ```ebnf
 * PositionalArgumentsDeclaration = (* open_paren: *) OPEN_PAREN
 *                                  (* arguments: *) PositionalArguments
 *                                  (* close_paren: *) CLOSE_PAREN;
 * ```
 */
pub type PositionalArgumentsDeclaration = Rc<PositionalArgumentsDeclarationStruct>;

#[derive(Debug)]
pub struct PositionalArgumentsDeclarationStruct {
    pub cursor: Cursor,
    pub open_paren: Rc<TerminalNode>,
    pub arguments: PositionalArguments,
    pub close_paren: Rc<TerminalNode>,
}

/**
 * This node represents a `NamedArgumentsDeclaration` nonterminal, with the following structure:
 *
 * ```ebnf
 * NamedArgumentsDeclaration = (* open_paren: *) OPEN_PAREN
 *                             (* arguments: *) NamedArgumentGroup?
 *                             (* close_paren: *) CLOSE_PAREN;
 * ```
 */
pub type NamedArgumentsDeclaration = Rc<NamedArgumentsDeclarationStruct>;

#[derive(Debug)]
pub struct NamedArgumentsDeclarationStruct {
    pub cursor: Cursor,
    pub open_paren: Rc<TerminalNode>,
    pub arguments: Option<NamedArgumentGroup>,
    pub close_paren: Rc<TerminalNode>,
}

/**
 * This node represents a `NamedArgumentGroup` nonterminal, with the following structure:
 *
 * ```ebnf
 * NamedArgumentGroup = (* open_brace: *) OPEN_BRACE
 *                      (* arguments: *) NamedArguments
 *                      (* close_brace: *) CLOSE_BRACE;
 * ```
 */
pub type NamedArgumentGroup = Rc<NamedArgumentGroupStruct>;

#[derive(Debug)]
pub struct NamedArgumentGroupStruct {
    pub cursor: Cursor,
    pub open_brace: Rc<TerminalNode>,
    pub arguments: NamedArguments,
    pub close_brace: Rc<TerminalNode>,
}

/**
 * This node represents a `NamedArgument` nonterminal, with the following structure:
 *
 * ```ebnf
 * NamedArgument = (* name: *) IDENTIFIER
 *                 (* colon: *) COLON
 *                 (* value: *) Expression;
 * ```
 */
pub type NamedArgument = Rc<NamedArgumentStruct>;

#[derive(Debug)]
pub struct NamedArgumentStruct {
    pub cursor: Cursor,
    pub name: Rc<TerminalNode>,
    pub colon: Rc<TerminalNode>,
    pub value: Expression,
}

/**
 * This node represents a `TypeExpression` nonterminal, with the following structure:
 *
 * ```ebnf
 * (* Introduced in 0.5.3 *)
 * TypeExpression = (* type_keyword: *) TYPE_KEYWORD
 *                  (* open_paren: *) OPEN_PAREN
 *                  (* type_name: *) TypeName
 *                  (* close_paren: *) CLOSE_PAREN;
 * ```
 */
pub type TypeExpression = Rc<TypeExpressionStruct>;

#[derive(Debug)]
pub struct TypeExpressionStruct {
    pub cursor: Cursor,
    pub type_keyword: Rc<TerminalNode>,
    pub open_paren: Rc<TerminalNode>,
    pub type_name: TypeName,
    pub close_paren: Rc<TerminalNode>,
}

/**
 * This node represents a `NewExpression` nonterminal, with the following structure:
 *
 * ```ebnf
 * NewExpression = (* new_keyword: *) NEW_KEYWORD
 *                 (* type_name: *) TypeName;
 * ```
 */
pub type NewExpression = Rc<NewExpressionStruct>;

#[derive(Debug)]
pub struct NewExpressionStruct {
    pub cursor: Cursor,
    pub new_keyword: Rc<TerminalNode>,
    pub type_name: TypeName,
}

/**
 * This node represents a `TupleExpression` nonterminal, with the following structure:
 *
 * ```ebnf
 * TupleExpression = (* open_paren: *) OPEN_PAREN
 *                   (* items: *) TupleValues
 *                   (* close_paren: *) CLOSE_PAREN;
 * ```
 */
pub type TupleExpression = Rc<TupleExpressionStruct>;

#[derive(Debug)]
pub struct TupleExpressionStruct {
    pub cursor: Cursor,
    pub open_paren: Rc<TerminalNode>,
    pub items: TupleValues,
    pub close_paren: Rc<TerminalNode>,
}

/**
 * This node represents a `TupleValue` nonterminal, with the following structure:
 *
 * ```ebnf
 * TupleValue = (* expression: *) Expression?;
 * ```
 */
pub type TupleValue = Rc<TupleValueStruct>;

#[derive(Debug)]
pub struct TupleValueStruct {
    pub cursor: Cursor,
    pub expression: Option<Expression>,
}

/**
 * This node represents a `ArrayExpression` nonterminal, with the following structure:
 *
 * ```ebnf
 * ArrayExpression = (* open_bracket: *) OPEN_BRACKET
 *                   (* items: *) ArrayValues
 *                   (* close_bracket: *) CLOSE_BRACKET;
 * ```
 */
pub type ArrayExpression = Rc<ArrayExpressionStruct>;

#[derive(Debug)]
pub struct ArrayExpressionStruct {
    pub cursor: Cursor,
    pub open_bracket: Rc<TerminalNode>,
    pub items: ArrayValues,
    pub close_bracket: Rc<TerminalNode>,
}

/**
 * This node represents a `HexNumberExpression` nonterminal, with the following structure:
 *
 * ```ebnf
 * HexNumberExpression = (* literal: *) HEX_LITERAL
 *                       (* unit: *) NumberUnit?; (* Deprecated in 0.5.0 *)
 * ```
 */
pub type HexNumberExpression = Rc<HexNumberExpressionStruct>;

#[derive(Debug)]
pub struct HexNumberExpressionStruct {
    pub cursor: Cursor,
    pub literal: Rc<TerminalNode>,
    pub unit: Option<NumberUnit>,
}

/**
 * This node represents a `DecimalNumberExpression` nonterminal, with the following structure:
 *
 * ```ebnf
 * DecimalNumberExpression = (* literal: *) DECIMAL_LITERAL
 *                           (* unit: *) NumberUnit?;
 * ```
 */
pub type DecimalNumberExpression = Rc<DecimalNumberExpressionStruct>;

#[derive(Debug)]
pub struct DecimalNumberExpressionStruct {
    pub cursor: Cursor,
    pub literal: Rc<TerminalNode>,
    pub unit: Option<NumberUnit>,
}

/**
 * This node represents a `YulBlock` nonterminal, with the following structure:
 *
 * ```ebnf
 * YulBlock = (* open_brace: *) OPEN_BRACE
 *            (* statements: *) YulStatements
 *            (* close_brace: *) CLOSE_BRACE;
 * ```
 */
pub type YulBlock = Rc<YulBlockStruct>;

#[derive(Debug)]
pub struct YulBlockStruct {
    pub cursor: Cursor,
    pub open_brace: Rc<TerminalNode>,
    pub statements: YulStatements,
    pub close_brace: Rc<TerminalNode>,
}

/**
 * This node represents a `YulFunctionDefinition` nonterminal, with the following structure:
 *
 * ```ebnf
 * YulFunctionDefinition = (* function_keyword: *) YUL_FUNCTION_KEYWORD
 *                         (* name: *) YUL_IDENTIFIER
 *                         (* parameters: *) YulParametersDeclaration
 *                         (* returns: *) YulReturnsDeclaration?
 *                         (* body: *) YulBlock;
 * ```
 */
pub type YulFunctionDefinition = Rc<YulFunctionDefinitionStruct>;

#[derive(Debug)]
pub struct YulFunctionDefinitionStruct {
    pub cursor: Cursor,
    pub function_keyword: Rc<TerminalNode>,
    pub name: Rc<TerminalNode>,
    pub parameters: YulParametersDeclaration,
    pub returns: Option<YulReturnsDeclaration>,
    pub body: YulBlock,
}

/**
 * This node represents a `YulParametersDeclaration` nonterminal, with the following structure:
 *
 * ```ebnf
 * YulParametersDeclaration = (* open_paren: *) OPEN_PAREN
 *                            (* parameters: *) YulParameters
 *                            (* close_paren: *) CLOSE_PAREN;
 * ```
 */
pub type YulParametersDeclaration = Rc<YulParametersDeclarationStruct>;

#[derive(Debug)]
pub struct YulParametersDeclarationStruct {
    pub cursor: Cursor,
    pub open_paren: Rc<TerminalNode>,
    pub parameters: YulParameters,
    pub close_paren: Rc<TerminalNode>,
}

/**
 * This node represents a `YulReturnsDeclaration` nonterminal, with the following structure:
 *
 * ```ebnf
 * YulReturnsDeclaration = (* minus_greater_than: *) MINUS_GREATER_THAN
 *                         (* variables: *) YulVariableNames;
 * ```
 */
pub type YulReturnsDeclaration = Rc<YulReturnsDeclarationStruct>;

#[derive(Debug)]
pub struct YulReturnsDeclarationStruct {
    pub cursor: Cursor,
    pub minus_greater_than: Rc<TerminalNode>,
    pub variables: YulVariableNames,
}

/**
 * This node represents a `YulVariableDeclarationStatement` nonterminal, with the following structure:
 *
 * ```ebnf
 * YulVariableDeclarationStatement = (* let_keyword: *) YUL_LET_KEYWORD
 *                                   (* variables: *) YulVariableNames
 *                                   (* value: *) YulVariableDeclarationValue?;
 * ```
 */
pub type YulVariableDeclarationStatement = Rc<YulVariableDeclarationStatementStruct>;

#[derive(Debug)]
pub struct YulVariableDeclarationStatementStruct {
    pub cursor: Cursor,
    pub let_keyword: Rc<TerminalNode>,
    pub variables: YulVariableNames,
    pub value: Option<YulVariableDeclarationValue>,
}

/**
 * This node represents a `YulVariableDeclarationValue` nonterminal, with the following structure:
 *
 * ```ebnf
 * YulVariableDeclarationValue = (* assignment: *) YulAssignmentOperator
 *                               (* expression: *) YulExpression;
 * ```
 */
pub type YulVariableDeclarationValue = Rc<YulVariableDeclarationValueStruct>;

#[derive(Debug)]
pub struct YulVariableDeclarationValueStruct {
    pub cursor: Cursor,
    pub assignment: YulAssignmentOperator,
    pub expression: YulExpression,
}

/**
 * This node represents a `YulVariableAssignmentStatement` nonterminal, with the following structure:
 *
 * ```ebnf
 * YulVariableAssignmentStatement = (* variables: *) YulPaths
 *                                  (* assignment: *) YulAssignmentOperator
 *                                  (* expression: *) YulExpression;
 * ```
 */
pub type YulVariableAssignmentStatement = Rc<YulVariableAssignmentStatementStruct>;

#[derive(Debug)]
pub struct YulVariableAssignmentStatementStruct {
    pub cursor: Cursor,
    pub variables: YulPaths,
    pub assignment: YulAssignmentOperator,
    pub expression: YulExpression,
}

/**
 * This node represents a `YulColonAndEqual` nonterminal, with the following structure:
 *
 * ```ebnf
 * (* Deprecated in 0.5.5 *)
 * YulColonAndEqual = (* colon: *) COLON
 *                    (* equal: *) EQUAL;
 * ```
 */
pub type YulColonAndEqual = Rc<YulColonAndEqualStruct>;

#[derive(Debug)]
pub struct YulColonAndEqualStruct {
    pub cursor: Cursor,
    pub colon: Rc<TerminalNode>,
    pub equal: Rc<TerminalNode>,
}

/**
 * This node represents a `YulStackAssignmentStatement` nonterminal, with the following structure:
 *
 * ```ebnf
 * (* Deprecated in 0.5.0 *)
 * YulStackAssignmentStatement = (* assignment: *) YulStackAssignmentOperator
 *                               (* variable: *) YUL_IDENTIFIER;
 * ```
 */
pub type YulStackAssignmentStatement = Rc<YulStackAssignmentStatementStruct>;

#[derive(Debug)]
pub struct YulStackAssignmentStatementStruct {
    pub cursor: Cursor,
    pub assignment: YulStackAssignmentOperator,
    pub variable: Rc<TerminalNode>,
}

/**
 * This node represents a `YulEqualAndColon` nonterminal, with the following structure:
 *
 * ```ebnf
 * (* Deprecated in 0.5.0 *)
 * YulEqualAndColon = (* equal: *) EQUAL
 *                    (* colon: *) COLON;
 * ```
 */
pub type YulEqualAndColon = Rc<YulEqualAndColonStruct>;

#[derive(Debug)]
pub struct YulEqualAndColonStruct {
    pub cursor: Cursor,
    pub equal: Rc<TerminalNode>,
    pub colon: Rc<TerminalNode>,
}

/**
 * This node represents a `YulIfStatement` nonterminal, with the following structure:
 *
 * ```ebnf
 * YulIfStatement = (* if_keyword: *) YUL_IF_KEYWORD
 *                  (* condition: *) YulExpression
 *                  (* body: *) YulBlock;
 * ```
 */
pub type YulIfStatement = Rc<YulIfStatementStruct>;

#[derive(Debug)]
pub struct YulIfStatementStruct {
    pub cursor: Cursor,
    pub if_keyword: Rc<TerminalNode>,
    pub condition: YulExpression,
    pub body: YulBlock,
}

/**
 * This node represents a `YulForStatement` nonterminal, with the following structure:
 *
 * ```ebnf
 * YulForStatement = (* for_keyword: *) YUL_FOR_KEYWORD
 *                   (* initialization: *) YulBlock
 *                   (* condition: *) YulExpression
 *                   (* iterator: *) YulBlock
 *                   (* body: *) YulBlock;
 * ```
 */
pub type YulForStatement = Rc<YulForStatementStruct>;

#[derive(Debug)]
pub struct YulForStatementStruct {
    pub cursor: Cursor,
    pub for_keyword: Rc<TerminalNode>,
    pub initialization: YulBlock,
    pub condition: YulExpression,
    pub iterator: YulBlock,
    pub body: YulBlock,
}

/**
 * This node represents a `YulSwitchStatement` nonterminal, with the following structure:
 *
 * ```ebnf
 * YulSwitchStatement = (* switch_keyword: *) YUL_SWITCH_KEYWORD
 *                      (* expression: *) YulExpression
 *                      (* cases: *) YulSwitchCases;
 * ```
 */
pub type YulSwitchStatement = Rc<YulSwitchStatementStruct>;

#[derive(Debug)]
pub struct YulSwitchStatementStruct {
    pub cursor: Cursor,
    pub switch_keyword: Rc<TerminalNode>,
    pub expression: YulExpression,
    pub cases: YulSwitchCases,
}

/**
 * This node represents a `YulDefaultCase` nonterminal, with the following structure:
 *
 * ```ebnf
 * YulDefaultCase = (* default_keyword: *) YUL_DEFAULT_KEYWORD
 *                  (* body: *) YulBlock;
 * ```
 */
pub type YulDefaultCase = Rc<YulDefaultCaseStruct>;

#[derive(Debug)]
pub struct YulDefaultCaseStruct {
    pub cursor: Cursor,
    pub default_keyword: Rc<TerminalNode>,
    pub body: YulBlock,
}

/**
 * This node represents a `YulValueCase` nonterminal, with the following structure:
 *
 * ```ebnf
 * YulValueCase = (* case_keyword: *) YUL_CASE_KEYWORD
 *                (* value: *) YulLiteral
 *                (* body: *) YulBlock;
 * ```
 */
pub type YulValueCase = Rc<YulValueCaseStruct>;

#[derive(Debug)]
pub struct YulValueCaseStruct {
    pub cursor: Cursor,
    pub case_keyword: Rc<TerminalNode>,
    pub value: YulLiteral,
    pub body: YulBlock,
}

/**
 * This node represents a `YulLeaveStatement` nonterminal, with the following structure:
 *
 * ```ebnf
 * (* Introduced in 0.6.0 *)
 * YulLeaveStatement = (* leave_keyword: *) YUL_LEAVE_KEYWORD;
 * ```
 */
pub type YulLeaveStatement = Rc<YulLeaveStatementStruct>;

#[derive(Debug)]
pub struct YulLeaveStatementStruct {
    pub cursor: Cursor,
    pub leave_keyword: Rc<TerminalNode>,
}

/**
 * This node represents a `YulBreakStatement` nonterminal, with the following structure:
 *
 * ```ebnf
 * YulBreakStatement = (* break_keyword: *) YUL_BREAK_KEYWORD;
 * ```
 */
pub type YulBreakStatement = Rc<YulBreakStatementStruct>;

#[derive(Debug)]
pub struct YulBreakStatementStruct {
    pub cursor: Cursor,
    pub break_keyword: Rc<TerminalNode>,
}

/**
 * This node represents a `YulContinueStatement` nonterminal, with the following structure:
 *
 * ```ebnf
 * YulContinueStatement = (* continue_keyword: *) YUL_CONTINUE_KEYWORD;
 * ```
 */
pub type YulContinueStatement = Rc<YulContinueStatementStruct>;

#[derive(Debug)]
pub struct YulContinueStatementStruct {
    pub cursor: Cursor,
    pub continue_keyword: Rc<TerminalNode>,
}

/**
 * This node represents a `YulLabel` nonterminal, with the following structure:
 *
 * ```ebnf
 * (* Deprecated in 0.5.0 *)
 * YulLabel = (* label: *) YUL_IDENTIFIER
 *            (* colon: *) COLON;
 * ```
 */
pub type YulLabel = Rc<YulLabelStruct>;

#[derive(Debug)]
pub struct YulLabelStruct {
    pub cursor: Cursor,
    pub label: Rc<TerminalNode>,
    pub colon: Rc<TerminalNode>,
}

/**
 * This node represents a `YulFunctionCallExpression` nonterminal, with the following structure:
 *
 * ```ebnf
 * (* Postfix unary operator *)
 * YulFunctionCallExpression = (* operand: *) YulExpression
 *                             (* open_paren: *) OPEN_PAREN
 *                             (* arguments: *) YulArguments
 *                             (* close_paren: *) CLOSE_PAREN;
 * ```
 */
pub type YulFunctionCallExpression = Rc<YulFunctionCallExpressionStruct>;

#[derive(Debug)]
pub struct YulFunctionCallExpressionStruct {
    pub cursor: Cursor,
    pub operand: YulExpression,
    pub open_paren: Rc<TerminalNode>,
    pub arguments: YulArguments,
    pub close_paren: Rc<TerminalNode>,
}

//
// Choices:
//

/**
 * This node represents a `SourceUnitMember` nonterminal, with the following structure:
 *
 * ```ebnf
 * SourceUnitMember = (* variant: *) PragmaDirective
 *                  | (* variant: *) ImportDirective
 *                  | (* variant: *) ContractDefinition
 *                  | (* variant: *) InterfaceDefinition
 *                  | (* variant: *) LibraryDefinition
 *                  | (* variant: *) StructDefinition (* Introduced in 0.6.0 *)
 *                  | (* variant: *) EnumDefinition (* Introduced in 0.6.0 *)
 *                  | (* variant: *) FunctionDefinition (* Introduced in 0.7.1 *)
 *                  | (* variant: *) ErrorDefinition (* Introduced in 0.8.4 *)
 *                  | (* variant: *) UserDefinedValueTypeDefinition (* Introduced in 0.8.8 *)
 *                  | (* variant: *) UsingDirective (* Introduced in 0.8.13 *)
 *                  | (* variant: *) EventDefinition (* Introduced in 0.8.22 *)
 *                  | (* variant: *) ConstantDefinition; (* Introduced in 0.7.4 *)
 * ```
 */

#[derive(Debug)]
pub enum SourceUnitMember {
    PragmaDirective(PragmaDirective),
    ImportDirective(ImportDirective),
    ContractDefinition(ContractDefinition),
    InterfaceDefinition(InterfaceDefinition),
    LibraryDefinition(LibraryDefinition),
    StructDefinition(StructDefinition),
    EnumDefinition(EnumDefinition),
    FunctionDefinition(FunctionDefinition),
    ErrorDefinition(ErrorDefinition),
    UserDefinedValueTypeDefinition(UserDefinedValueTypeDefinition),
    UsingDirective(UsingDirective),
    EventDefinition(EventDefinition),
    ConstantDefinition(ConstantDefinition),
}

/**
 * This node represents a `Pragma` nonterminal, with the following structure:
 *
 * ```ebnf
 * Pragma = (* variant: *) AbicoderPragma
 *        | (* variant: *) ExperimentalPragma
 *        | (* variant: *) VersionPragma;
 * ```
 */

#[derive(Debug)]
pub enum Pragma {
    AbicoderPragma(AbicoderPragma),
    ExperimentalPragma(ExperimentalPragma),
    VersionPragma(VersionPragma),
}

/**
 * This node represents a `ExperimentalFeature` nonterminal, with the following structure:
 *
 * ```ebnf
 * ExperimentalFeature = (* variant: *) IDENTIFIER
 *                     | (* variant: *) StringLiteral;
 * ```
 */

#[derive(Debug)]
pub enum ExperimentalFeature {
    StringLiteral(StringLiteral),
    TerminalNode(Rc<TerminalNode>),
}

/**
 * This node represents a `VersionExpression` nonterminal, with the following structure:
 *
 * ```ebnf
 * VersionExpression = (* variant: *) VersionRange
 *                   | (* variant: *) VersionTerm;
 * ```
 */

#[derive(Debug)]
pub enum VersionExpression {
    VersionRange(VersionRange),
    VersionTerm(VersionTerm),
}

/**
 * This node represents a `VersionOperator` nonterminal, with the following structure:
 *
 * ```ebnf
 * VersionOperator = (* variant: *) CARET
 *                 | (* variant: *) TILDE
 *                 | (* variant: *) EQUAL
 *                 | (* variant: *) LESS_THAN
 *                 | (* variant: *) GREATER_THAN
 *                 | (* variant: *) LESS_THAN_EQUAL
 *                 | (* variant: *) GREATER_THAN_EQUAL;
 * ```
 */

#[derive(Debug)]
pub struct VersionOperator(pub Rc<TerminalNode>);

/**
 * This node represents a `VersionLiteral` nonterminal, with the following structure:
 *
 * ```ebnf
 * VersionLiteral = (* variant: *) SimpleVersionLiteral
 *                | (* variant: *) SINGLE_QUOTED_VERSION_LITERAL
 *                | (* variant: *) DOUBLE_QUOTED_VERSION_LITERAL;
 * ```
 */

#[derive(Debug)]
pub enum VersionLiteral {
    SimpleVersionLiteral(SimpleVersionLiteral),
    TerminalNode(Rc<TerminalNode>),
}

/**
 * This node represents a `ImportClause` nonterminal, with the following structure:
 *
 * ```ebnf
 * ImportClause = (* variant: *) PathImport
 *              | (* variant: *) NamedImport
 *              | (* variant: *) ImportDeconstruction;
 * ```
 */

#[derive(Debug)]
pub enum ImportClause {
    PathImport(PathImport),
    NamedImport(NamedImport),
    ImportDeconstruction(ImportDeconstruction),
}

/**
 * This node represents a `UsingClause` nonterminal, with the following structure:
 *
 * ```ebnf
 * UsingClause = (* variant: *) IdentifierPath
 *             | (* variant: *) UsingDeconstruction; (* Introduced in 0.8.13 *)
 * ```
 */

#[derive(Debug)]
pub enum UsingClause {
    IdentifierPath(IdentifierPath),
    UsingDeconstruction(UsingDeconstruction),
}

/**
 * This node represents a `UsingOperator` nonterminal, with the following structure:
 *
 * ```ebnf
 * (* Introduced in 0.8.19 *)
 * UsingOperator = (* variant: *) AMPERSAND
 *               | (* variant: *) ASTERISK
 *               | (* variant: *) BANG_EQUAL
 *               | (* variant: *) BAR
 *               | (* variant: *) CARET
 *               | (* variant: *) EQUAL_EQUAL
 *               | (* variant: *) GREATER_THAN
 *               | (* variant: *) GREATER_THAN_EQUAL
 *               | (* variant: *) LESS_THAN
 *               | (* variant: *) LESS_THAN_EQUAL
 *               | (* variant: *) MINUS
 *               | (* variant: *) PERCENT
 *               | (* variant: *) PLUS
 *               | (* variant: *) SLASH
 *               | (* variant: *) TILDE;
 * ```
 */

#[derive(Debug)]
pub struct UsingOperator(pub Rc<TerminalNode>);

/**
 * This node represents a `UsingTarget` nonterminal, with the following structure:
 *
 * ```ebnf
 * UsingTarget = (* variant: *) TypeName
 *             | (* variant: *) ASTERISK;
 * ```
 */

#[derive(Debug)]
pub enum UsingTarget {
    TypeName(TypeName),
    TerminalNode(Rc<TerminalNode>),
}

/**
 * This node represents a `ContractMember` nonterminal, with the following structure:
 *
 * ```ebnf
 * ContractMember = (* variant: *) UsingDirective
 *                | (* variant: *) FunctionDefinition
 *                | (* variant: *) ConstructorDefinition (* Introduced in 0.4.22 *)
 *                | (* variant: *) ReceiveFunctionDefinition (* Introduced in 0.6.0 *)
 *                | (* variant: *) FallbackFunctionDefinition (* Introduced in 0.6.0 *)
 *                | (* variant: *) UnnamedFunctionDefinition (* Deprecated in 0.6.0 *)
 *                | (* variant: *) ModifierDefinition
 *                | (* variant: *) StructDefinition
 *                | (* variant: *) EnumDefinition
 *                | (* variant: *) EventDefinition
 *                | (* variant: *) ErrorDefinition (* Introduced in 0.8.4 *)
 *                | (* variant: *) UserDefinedValueTypeDefinition (* Introduced in 0.8.8 *)
 *                | (* variant: *) StateVariableDefinition;
 * ```
 */

#[derive(Debug)]
pub enum ContractMember {
    UsingDirective(UsingDirective),
    FunctionDefinition(FunctionDefinition),
    ConstructorDefinition(ConstructorDefinition),
    ReceiveFunctionDefinition(ReceiveFunctionDefinition),
    FallbackFunctionDefinition(FallbackFunctionDefinition),
    UnnamedFunctionDefinition(UnnamedFunctionDefinition),
    ModifierDefinition(ModifierDefinition),
    StructDefinition(StructDefinition),
    EnumDefinition(EnumDefinition),
    EventDefinition(EventDefinition),
    ErrorDefinition(ErrorDefinition),
    UserDefinedValueTypeDefinition(UserDefinedValueTypeDefinition),
    StateVariableDefinition(StateVariableDefinition),
}

/**
 * This node represents a `StateVariableAttribute` nonterminal, with the following structure:
 *
 * ```ebnf
 * StateVariableAttribute = (* variant: *) OverrideSpecifier (* Introduced in 0.6.0 *)
 *                        | (* variant: *) CONSTANT_KEYWORD
 *                        | (* variant: *) INTERNAL_KEYWORD
 *                        | (* variant: *) PRIVATE_KEYWORD
 *                        | (* variant: *) PUBLIC_KEYWORD
 *                        | (* variant: *) IMMUTABLE_KEYWORD (* Introduced in 0.6.5 *)
 *                        | (* variant: *) TRANSIENT_KEYWORD; (* Introduced in 0.8.27 *)
 * ```
 */

#[derive(Debug)]
pub enum StateVariableAttribute {
    OverrideSpecifier(OverrideSpecifier),
    TerminalNode(Rc<TerminalNode>),
}

/**
 * This node represents a `FunctionName` nonterminal, with the following structure:
 *
 * ```ebnf
 * FunctionName = (* variant: *) IDENTIFIER
 *              | (* variant: *) FALLBACK_KEYWORD
 *              | (* variant: *) RECEIVE_KEYWORD;
 * ```
 */

#[derive(Debug)]
pub struct FunctionName(pub Rc<TerminalNode>);

/**
 * This node represents a `FunctionAttribute` nonterminal, with the following structure:
 *
 * ```ebnf
 * FunctionAttribute = (* variant: *) ModifierInvocation
 *                   | (* variant: *) OverrideSpecifier (* Introduced in 0.6.0 *)
 *                   | (* variant: *) CONSTANT_KEYWORD (* Deprecated in 0.5.0 *)
 *                   | (* variant: *) EXTERNAL_KEYWORD
 *                   | (* variant: *) INTERNAL_KEYWORD
 *                   | (* variant: *) PAYABLE_KEYWORD
 *                   | (* variant: *) PRIVATE_KEYWORD
 *                   | (* variant: *) PUBLIC_KEYWORD
 *                   | (* variant: *) PURE_KEYWORD (* Introduced in 0.4.16 *)
 *                   | (* variant: *) VIEW_KEYWORD (* Introduced in 0.4.16 *)
 *                   | (* variant: *) VIRTUAL_KEYWORD; (* Introduced in 0.6.0 *)
 * ```
 */

#[derive(Debug)]
pub enum FunctionAttribute {
    ModifierInvocation(ModifierInvocation),
    OverrideSpecifier(OverrideSpecifier),
    TerminalNode(Rc<TerminalNode>),
}

/**
 * This node represents a `FunctionBody` nonterminal, with the following structure:
 *
 * ```ebnf
 * FunctionBody = (* variant: *) Block
 *              | (* variant: *) SEMICOLON;
 * ```
 */

#[derive(Debug)]
pub enum FunctionBody {
    Block(Block),
    TerminalNode(Rc<TerminalNode>),
}

/**
 * This node represents a `ConstructorAttribute` nonterminal, with the following structure:
 *
 * ```ebnf
 * (* Introduced in 0.4.22 *)
 * ConstructorAttribute = (* variant: *) ModifierInvocation
 *                      | (* variant: *) INTERNAL_KEYWORD
 *                      | (* variant: *) OVERRIDE_KEYWORD (* Introduced in 0.6.0 and deprecated in 0.6.7. *)
 *                      | (* variant: *) PAYABLE_KEYWORD
 *                      | (* variant: *) PUBLIC_KEYWORD
 *                      | (* variant: *) VIRTUAL_KEYWORD; (* Introduced in 0.6.0 and deprecated in 0.6.7. *)
 * ```
 */

#[derive(Debug)]
pub enum ConstructorAttribute {
    ModifierInvocation(ModifierInvocation),
    TerminalNode(Rc<TerminalNode>),
}

/**
 * This node represents a `UnnamedFunctionAttribute` nonterminal, with the following structure:
 *
 * ```ebnf
 * (* Deprecated in 0.6.0 *)
 * UnnamedFunctionAttribute = (* variant: *) ModifierInvocation
 *                          | (* variant: *) CONSTANT_KEYWORD (* Deprecated in 0.5.0 *)
 *                          | (* variant: *) EXTERNAL_KEYWORD
 *                          | (* variant: *) INTERNAL_KEYWORD (* Deprecated in 0.5.0 *)
 *                          | (* variant: *) PAYABLE_KEYWORD
 *                          | (* variant: *) PRIVATE_KEYWORD (* Deprecated in 0.5.0 *)
 *                          | (* variant: *) PUBLIC_KEYWORD (* Deprecated in 0.5.0 *)
 *                          | (* variant: *) PURE_KEYWORD (* Introduced in 0.4.16 and deprecated in 0.6.0. *)
 *                          | (* variant: *) VIEW_KEYWORD; (* Introduced in 0.4.16 and deprecated in 0.6.0. *)
 * ```
 */

#[derive(Debug)]
pub enum UnnamedFunctionAttribute {
    ModifierInvocation(ModifierInvocation),
    TerminalNode(Rc<TerminalNode>),
}

/**
 * This node represents a `FallbackFunctionAttribute` nonterminal, with the following structure:
 *
 * ```ebnf
 * (* Introduced in 0.6.0 *)
 * FallbackFunctionAttribute = (* variant: *) ModifierInvocation
 *                           | (* variant: *) OverrideSpecifier
 *                           | (* variant: *) EXTERNAL_KEYWORD
 *                           | (* variant: *) PAYABLE_KEYWORD
 *                           | (* variant: *) PURE_KEYWORD
 *                           | (* variant: *) VIEW_KEYWORD
 *                           | (* variant: *) VIRTUAL_KEYWORD;
 * ```
 */

#[derive(Debug)]
pub enum FallbackFunctionAttribute {
    ModifierInvocation(ModifierInvocation),
    OverrideSpecifier(OverrideSpecifier),
    TerminalNode(Rc<TerminalNode>),
}

/**
 * This node represents a `ReceiveFunctionAttribute` nonterminal, with the following structure:
 *
 * ```ebnf
 * (* Introduced in 0.6.0 *)
 * ReceiveFunctionAttribute = (* variant: *) ModifierInvocation
 *                          | (* variant: *) OverrideSpecifier
 *                          | (* variant: *) EXTERNAL_KEYWORD
 *                          | (* variant: *) PAYABLE_KEYWORD
 *                          | (* variant: *) VIRTUAL_KEYWORD;
 * ```
 */

#[derive(Debug)]
pub enum ReceiveFunctionAttribute {
    ModifierInvocation(ModifierInvocation),
    OverrideSpecifier(OverrideSpecifier),
    TerminalNode(Rc<TerminalNode>),
}

/**
 * This node represents a `ModifierAttribute` nonterminal, with the following structure:
 *
 * ```ebnf
 * ModifierAttribute = (* variant: *) OverrideSpecifier (* Introduced in 0.6.0 *)
 *                   | (* variant: *) VIRTUAL_KEYWORD; (* Introduced in 0.6.0 *)
 * ```
 */

#[derive(Debug)]
pub enum ModifierAttribute {
    OverrideSpecifier(OverrideSpecifier),
    TerminalNode(Rc<TerminalNode>),
}

/**
 * This node represents a `TypeName` nonterminal, with the following structure:
 *
 * ```ebnf
 * TypeName = (* variant: *) ArrayTypeName
 *          | (* variant: *) FunctionType
 *          | (* variant: *) MappingType
 *          | (* variant: *) ElementaryType
 *          | (* variant: *) IdentifierPath;
 * ```
 */

#[derive(Debug)]
pub enum TypeName {
    ArrayTypeName(ArrayTypeName),
    FunctionType(FunctionType),
    MappingType(MappingType),
    ElementaryType(ElementaryType),
    IdentifierPath(IdentifierPath),
}

/**
 * This node represents a `FunctionTypeAttribute` nonterminal, with the following structure:
 *
 * ```ebnf
 * FunctionTypeAttribute = (* variant: *) INTERNAL_KEYWORD
 *                       | (* variant: *) EXTERNAL_KEYWORD
 *                       | (* variant: *) PRIVATE_KEYWORD
 *                       | (* variant: *) PUBLIC_KEYWORD
 *                       | (* variant: *) CONSTANT_KEYWORD (* Deprecated in 0.5.0 *)
 *                       | (* variant: *) PURE_KEYWORD (* Introduced in 0.4.16 *)
 *                       | (* variant: *) VIEW_KEYWORD (* Introduced in 0.4.16 *)
 *                       | (* variant: *) PAYABLE_KEYWORD;
 * ```
 */

#[derive(Debug)]
pub struct FunctionTypeAttribute(pub Rc<TerminalNode>);

/**
 * This node represents a `MappingKeyType` nonterminal, with the following structure:
 *
 * ```ebnf
 * MappingKeyType = (* variant: *) ElementaryType
 *                | (* variant: *) IdentifierPath;
 * ```
 */

#[derive(Debug)]
pub enum MappingKeyType {
    ElementaryType(ElementaryType),
    IdentifierPath(IdentifierPath),
}

/**
 * This node represents a `ElementaryType` nonterminal, with the following structure:
 *
 * ```ebnf
 * ElementaryType = (* variant: *) BOOL_KEYWORD
 *                | (* variant: *) BYTE_KEYWORD (* Deprecated in 0.8.0 *)
 *                | (* variant: *) STRING_KEYWORD
 *                | (* variant: *) AddressType
 *                | (* variant: *) BYTES_KEYWORD
 *                | (* variant: *) INT_KEYWORD
 *                | (* variant: *) UINT_KEYWORD
 *                | (* variant: *) FIXED_KEYWORD
 *                | (* variant: *) UFIXED_KEYWORD;
 * ```
 */

#[derive(Debug)]
pub enum ElementaryType {
    AddressType(AddressType),
    TerminalNode(Rc<TerminalNode>),
}

/**
 * This node represents a `Statement` nonterminal, with the following structure:
 *
 * ```ebnf
 * Statement = (* variant: *) IfStatement
 *           | (* variant: *) ForStatement
 *           | (* variant: *) WhileStatement
 *           | (* variant: *) DoWhileStatement
 *           | (* variant: *) ContinueStatement
 *           | (* variant: *) BreakStatement
 *           | (* variant: *) ReturnStatement
 *           | (* variant: *) ThrowStatement (* Deprecated in 0.5.0 *)
 *           | (* variant: *) EmitStatement (* Introduced in 0.4.21 *)
 *           | (* variant: *) TryStatement (* Introduced in 0.6.0 *)
 *           | (* variant: *) RevertStatement (* Introduced in 0.8.4 *)
 *           | (* variant: *) AssemblyStatement
 *           | (* variant: *) Block
 *           | (* variant: *) UncheckedBlock (* Introduced in 0.8.0 *)
 *           | (* variant: *) TupleDeconstructionStatement
 *           | (* variant: *) VariableDeclarationStatement
 *           | (* variant: *) ExpressionStatement;
 * ```
 */

#[derive(Debug)]
pub enum Statement {
    IfStatement(IfStatement),
    ForStatement(ForStatement),
    WhileStatement(WhileStatement),
    DoWhileStatement(DoWhileStatement),
    ContinueStatement(ContinueStatement),
    BreakStatement(BreakStatement),
    ReturnStatement(ReturnStatement),
    ThrowStatement(ThrowStatement),
    EmitStatement(EmitStatement),
    TryStatement(TryStatement),
    RevertStatement(RevertStatement),
    AssemblyStatement(AssemblyStatement),
    Block(Block),
    UncheckedBlock(UncheckedBlock),
    TupleDeconstructionStatement(TupleDeconstructionStatement),
    VariableDeclarationStatement(VariableDeclarationStatement),
    ExpressionStatement(ExpressionStatement),
}

/**
 * This node represents a `TupleMember` nonterminal, with the following structure:
 *
 * ```ebnf
 * TupleMember = (* variant: *) TypedTupleMember
 *             | (* variant: *) UntypedTupleMember;
 * ```
 */

#[derive(Debug)]
pub enum TupleMember {
    TypedTupleMember(TypedTupleMember),
    UntypedTupleMember(UntypedTupleMember),
}

/**
 * This node represents a `VariableDeclarationType` nonterminal, with the following structure:
 *
 * ```ebnf
 * VariableDeclarationType = (* variant: *) TypeName
 *                         | (* variant: *) VAR_KEYWORD; (* Deprecated in 0.5.0 *)
 * ```
 */

#[derive(Debug)]
pub enum VariableDeclarationType {
    TypeName(TypeName),
    TerminalNode(Rc<TerminalNode>),
}

/**
 * This node represents a `StorageLocation` nonterminal, with the following structure:
 *
 * ```ebnf
 * StorageLocation = (* variant: *) MEMORY_KEYWORD
 *                 | (* variant: *) STORAGE_KEYWORD
 *                 | (* variant: *) CALL_DATA_KEYWORD; (* Introduced in 0.5.0 *)
 * ```
 */

#[derive(Debug)]
pub struct StorageLocation(pub Rc<TerminalNode>);

/**
 * This node represents a `ForStatementInitialization` nonterminal, with the following structure:
 *
 * ```ebnf
 * ForStatementInitialization = (* variant: *) TupleDeconstructionStatement
 *                            | (* variant: *) VariableDeclarationStatement
 *                            | (* variant: *) ExpressionStatement
 *                            | (* variant: *) SEMICOLON;
 * ```
 */

#[derive(Debug)]
pub enum ForStatementInitialization {
    TupleDeconstructionStatement(TupleDeconstructionStatement),
    VariableDeclarationStatement(VariableDeclarationStatement),
    ExpressionStatement(ExpressionStatement),
    TerminalNode(Rc<TerminalNode>),
}

/**
 * This node represents a `ForStatementCondition` nonterminal, with the following structure:
 *
 * ```ebnf
 * ForStatementCondition = (* variant: *) ExpressionStatement
 *                       | (* variant: *) SEMICOLON;
 * ```
 */

#[derive(Debug)]
pub enum ForStatementCondition {
    ExpressionStatement(ExpressionStatement),
    TerminalNode(Rc<TerminalNode>),
}

/**
 * This node represents a `Expression` nonterminal, with the following structure:
 *
 * ```ebnf
 * Expression = (* variant: *) AssignmentExpression
 *            | (* variant: *) ConditionalExpression
 *            | (* variant: *) OrExpression
 *            | (* variant: *) AndExpression
 *            | (* variant: *) EqualityExpression
 *            | (* variant: *) InequalityExpression
 *            | (* variant: *) BitwiseOrExpression
 *            | (* variant: *) BitwiseXorExpression
 *            | (* variant: *) BitwiseAndExpression
 *            | (* variant: *) ShiftExpression
 *            | (* variant: *) AdditiveExpression
 *            | (* variant: *) MultiplicativeExpression
 *            | (* variant: *) ExponentiationExpression
 *            | (* variant: *) PostfixExpression
 *            | (* variant: *) PrefixExpression
 *            | (* variant: *) FunctionCallExpression
 *            | (* variant: *) CallOptionsExpression
 *            | (* variant: *) MemberAccessExpression
 *            | (* variant: *) IndexAccessExpression
 *            | (* variant: *) NewExpression
 *            | (* variant: *) TupleExpression
 *            | (* variant: *) TypeExpression (* Introduced in 0.5.3 *)
 *            | (* variant: *) ArrayExpression
 *            | (* variant: *) HexNumberExpression
 *            | (* variant: *) DecimalNumberExpression
 *            | (* variant: *) StringExpression
 *            | (* variant: *) ElementaryType
 *            | (* variant: *) PAYABLE_KEYWORD (* Introduced in 0.6.0 *)
 *            | (* variant: *) THIS_KEYWORD
 *            | (* variant: *) SUPER_KEYWORD
 *            | (* variant: *) TRUE_KEYWORD
 *            | (* variant: *) FALSE_KEYWORD
 *            | (* variant: *) IDENTIFIER;
 * ```
 */

#[derive(Debug)]
pub enum Expression {
    AssignmentExpression(AssignmentExpression),
    ConditionalExpression(ConditionalExpression),
    OrExpression(OrExpression),
    AndExpression(AndExpression),
    EqualityExpression(EqualityExpression),
    InequalityExpression(InequalityExpression),
    BitwiseOrExpression(BitwiseOrExpression),
    BitwiseXorExpression(BitwiseXorExpression),
    BitwiseAndExpression(BitwiseAndExpression),
    ShiftExpression(ShiftExpression),
    AdditiveExpression(AdditiveExpression),
    MultiplicativeExpression(MultiplicativeExpression),
    ExponentiationExpression(ExponentiationExpression),
    PostfixExpression(PostfixExpression),
    PrefixExpression(PrefixExpression),
    FunctionCallExpression(FunctionCallExpression),
    CallOptionsExpression(CallOptionsExpression),
    MemberAccessExpression(MemberAccessExpression),
    IndexAccessExpression(IndexAccessExpression),
    NewExpression(NewExpression),
    TupleExpression(TupleExpression),
    TypeExpression(TypeExpression),
    ArrayExpression(ArrayExpression),
    HexNumberExpression(HexNumberExpression),
    DecimalNumberExpression(DecimalNumberExpression),
    StringExpression(StringExpression),
    ElementaryType(ElementaryType),
    TerminalNode(Rc<TerminalNode>),
}

/**
 * This node represents a `ArgumentsDeclaration` nonterminal, with the following structure:
 *
 * ```ebnf
 * ArgumentsDeclaration = (* variant: *) PositionalArgumentsDeclaration
 *                      | (* variant: *) NamedArgumentsDeclaration;
 * ```
 */

#[derive(Debug)]
pub enum ArgumentsDeclaration {
    PositionalArgumentsDeclaration(PositionalArgumentsDeclaration),
    NamedArgumentsDeclaration(NamedArgumentsDeclaration),
}

/**
 * This node represents a `NumberUnit` nonterminal, with the following structure:
 *
 * ```ebnf
 * NumberUnit = (* variant: *) WEI_KEYWORD
 *            | (* variant: *) GWEI_KEYWORD (* Introduced in 0.6.11 *)
 *            | (* variant: *) SZABO_KEYWORD (* Deprecated in 0.7.0 *)
 *            | (* variant: *) FINNEY_KEYWORD (* Deprecated in 0.7.0 *)
 *            | (* variant: *) ETHER_KEYWORD
 *            | (* variant: *) SECONDS_KEYWORD
 *            | (* variant: *) MINUTES_KEYWORD
 *            | (* variant: *) HOURS_KEYWORD
 *            | (* variant: *) DAYS_KEYWORD
 *            | (* variant: *) WEEKS_KEYWORD
 *            | (* variant: *) YEARS_KEYWORD; (* Deprecated in 0.5.0 *)
 * ```
 */

#[derive(Debug)]
pub struct NumberUnit(pub Rc<TerminalNode>);

/**
 * This node represents a `StringExpression` nonterminal, with the following structure:
 *
 * ```ebnf
 * StringExpression = (* variant: *) StringLiteral (* Deprecated in 0.5.14 *)
 *                  | (* variant: *) StringLiterals (* Introduced in 0.5.14 *)
 *                  | (* variant: *) HexStringLiteral (* Deprecated in 0.5.14 *)
 *                  | (* variant: *) HexStringLiterals (* Introduced in 0.5.14 *)
 *                  | (* variant: *) UnicodeStringLiterals; (* Introduced in 0.7.0 *)
 * ```
 */

#[derive(Debug)]
pub enum StringExpression {
    StringLiteral(StringLiteral),
    StringLiterals(StringLiterals),
    HexStringLiteral(HexStringLiteral),
    HexStringLiterals(HexStringLiterals),
    UnicodeStringLiterals(UnicodeStringLiterals),
}

/**
 * This node represents a `StringLiteral` nonterminal, with the following structure:
 *
 * ```ebnf
 * StringLiteral = (* variant: *) SINGLE_QUOTED_STRING_LITERAL
 *               | (* variant: *) DOUBLE_QUOTED_STRING_LITERAL;
 * ```
 */

#[derive(Debug)]
pub struct StringLiteral(pub Rc<TerminalNode>);

/**
 * This node represents a `HexStringLiteral` nonterminal, with the following structure:
 *
 * ```ebnf
 * HexStringLiteral = (* variant: *) SINGLE_QUOTED_HEX_STRING_LITERAL
 *                  | (* variant: *) DOUBLE_QUOTED_HEX_STRING_LITERAL;
 * ```
 */

#[derive(Debug)]
pub struct HexStringLiteral(pub Rc<TerminalNode>);

/**
 * This node represents a `UnicodeStringLiteral` nonterminal, with the following structure:
 *
 * ```ebnf
 * (* Introduced in 0.7.0 *)
 * UnicodeStringLiteral = (* variant: *) SINGLE_QUOTED_UNICODE_STRING_LITERAL
 *                      | (* variant: *) DOUBLE_QUOTED_UNICODE_STRING_LITERAL;
 * ```
 */

#[derive(Debug)]
pub struct UnicodeStringLiteral(pub Rc<TerminalNode>);

/**
 * This node represents a `YulStatement` nonterminal, with the following structure:
 *
 * ```ebnf
 * YulStatement = (* variant: *) YulBlock
 *              | (* variant: *) YulFunctionDefinition
 *              | (* variant: *) YulStackAssignmentStatement (* Deprecated in 0.5.0 *)
 *              | (* variant: *) YulIfStatement
 *              | (* variant: *) YulForStatement
 *              | (* variant: *) YulSwitchStatement
 *              | (* variant: *) YulLeaveStatement (* Introduced in 0.6.0 *)
 *              | (* variant: *) YulBreakStatement
 *              | (* variant: *) YulContinueStatement
 *              | (* variant: *) YulVariableAssignmentStatement
 *              | (* variant: *) YulLabel (* Deprecated in 0.5.0 *)
 *              | (* variant: *) YulVariableDeclarationStatement
 *              | (* variant: *) YulExpression;
 * ```
 */

#[derive(Debug)]
pub enum YulStatement {
    YulBlock(YulBlock),
    YulFunctionDefinition(YulFunctionDefinition),
    YulStackAssignmentStatement(YulStackAssignmentStatement),
    YulIfStatement(YulIfStatement),
    YulForStatement(YulForStatement),
    YulSwitchStatement(YulSwitchStatement),
    YulLeaveStatement(YulLeaveStatement),
    YulBreakStatement(YulBreakStatement),
    YulContinueStatement(YulContinueStatement),
    YulVariableAssignmentStatement(YulVariableAssignmentStatement),
    YulLabel(YulLabel),
    YulVariableDeclarationStatement(YulVariableDeclarationStatement),
    YulExpression(YulExpression),
}

/**
 * This node represents a `YulAssignmentOperator` nonterminal, with the following structure:
 *
 * ```ebnf
 * YulAssignmentOperator = (* variant: *) COLON_EQUAL
 *                       | (* variant: *) YulColonAndEqual; (* Deprecated in 0.5.5 *)
 * ```
 */

#[derive(Debug)]
pub enum YulAssignmentOperator {
    YulColonAndEqual(YulColonAndEqual),
    TerminalNode(Rc<TerminalNode>),
}

/**
 * This node represents a `YulStackAssignmentOperator` nonterminal, with the following structure:
 *
 * ```ebnf
 * (* Deprecated in 0.5.0 *)
 * YulStackAssignmentOperator = (* variant: *) EQUAL_COLON
 *                            | (* variant: *) YulEqualAndColon;
 * ```
 */

#[derive(Debug)]
pub enum YulStackAssignmentOperator {
    YulEqualAndColon(YulEqualAndColon),
    TerminalNode(Rc<TerminalNode>),
}

/**
 * This node represents a `YulSwitchCase` nonterminal, with the following structure:
 *
 * ```ebnf
 * YulSwitchCase = (* variant: *) YulDefaultCase
 *               | (* variant: *) YulValueCase;
 * ```
 */

#[derive(Debug)]
pub enum YulSwitchCase {
    YulDefaultCase(YulDefaultCase),
    YulValueCase(YulValueCase),
}

/**
 * This node represents a `YulExpression` nonterminal, with the following structure:
 *
 * ```ebnf
 * YulExpression = (* variant: *) YulFunctionCallExpression
 *               | (* variant: *) YulLiteral
 *               | (* variant: *) YulPath;
 * ```
 */

#[derive(Debug)]
pub enum YulExpression {
    YulFunctionCallExpression(YulFunctionCallExpression),
    YulLiteral(YulLiteral),
    YulPath(YulPath),
}

/**
 * This node represents a `YulLiteral` nonterminal, with the following structure:
 *
 * ```ebnf
 * YulLiteral = (* variant: *) YUL_TRUE_KEYWORD (* Introduced in 0.6.2 *)
 *            | (* variant: *) YUL_FALSE_KEYWORD (* Introduced in 0.6.2 *)
 *            | (* variant: *) YUL_DECIMAL_LITERAL
 *            | (* variant: *) YUL_HEX_LITERAL
 *            | (* variant: *) HexStringLiteral
 *            | (* variant: *) StringLiteral;
 * ```
 */

#[derive(Debug)]
pub enum YulLiteral {
    HexStringLiteral(HexStringLiteral),
    StringLiteral(StringLiteral),
    TerminalNode(Rc<TerminalNode>),
}

//
// Repeated:
//

/**
 * This node represents a `SourceUnitMembers` nonterminal, with the following structure:
 *
 * ```ebnf
 * SourceUnitMembers = (* item: *) SourceUnitMember*;
 * ```
 */
pub type SourceUnitMembers = Vec<SourceUnitMember>;

/**
 * This node represents a `VersionExpressionSet` nonterminal, with the following structure:
 *
 * ```ebnf
 * VersionExpressionSet = (* item: *) VersionExpression+;
 * ```
 */
pub type VersionExpressionSet = Vec<VersionExpression>;

/**
 * This node represents a `ContractMembers` nonterminal, with the following structure:
 *
 * ```ebnf
 * ContractMembers = (* item: *) ContractMember*;
 * ```
 */
pub type ContractMembers = Vec<ContractMember>;

/**
 * This node represents a `InterfaceMembers` nonterminal, with the following structure:
 *
 * ```ebnf
 * InterfaceMembers = (* item: *) ContractMember*;
 * ```
 */
pub type InterfaceMembers = Vec<ContractMember>;

/**
 * This node represents a `LibraryMembers` nonterminal, with the following structure:
 *
 * ```ebnf
 * LibraryMembers = (* item: *) ContractMember*;
 * ```
 */
pub type LibraryMembers = Vec<ContractMember>;

/**
 * This node represents a `StructMembers` nonterminal, with the following structure:
 *
 * ```ebnf
 * StructMembers = (* item: *) StructMember*;
 * ```
 */
pub type StructMembers = Vec<StructMember>;

/**
 * This node represents a `StateVariableAttributes` nonterminal, with the following structure:
 *
 * ```ebnf
 * StateVariableAttributes = (* item: *) StateVariableAttribute*;
 * ```
 */
pub type StateVariableAttributes = Vec<StateVariableAttribute>;

/**
 * This node represents a `FunctionAttributes` nonterminal, with the following structure:
 *
 * ```ebnf
 * FunctionAttributes = (* item: *) FunctionAttribute*;
 * ```
 */
pub type FunctionAttributes = Vec<FunctionAttribute>;

/**
 * This node represents a `ConstructorAttributes` nonterminal, with the following structure:
 *
 * ```ebnf
 * (* Introduced in 0.4.22 *)
 * ConstructorAttributes = (* item: *) ConstructorAttribute*;
 * ```
 */
pub type ConstructorAttributes = Vec<ConstructorAttribute>;

/**
 * This node represents a `UnnamedFunctionAttributes` nonterminal, with the following structure:
 *
 * ```ebnf
 * (* Deprecated in 0.6.0 *)
 * UnnamedFunctionAttributes = (* item: *) UnnamedFunctionAttribute*;
 * ```
 */
pub type UnnamedFunctionAttributes = Vec<UnnamedFunctionAttribute>;

/**
 * This node represents a `FallbackFunctionAttributes` nonterminal, with the following structure:
 *
 * ```ebnf
 * (* Introduced in 0.6.0 *)
 * FallbackFunctionAttributes = (* item: *) FallbackFunctionAttribute*;
 * ```
 */
pub type FallbackFunctionAttributes = Vec<FallbackFunctionAttribute>;

/**
 * This node represents a `ReceiveFunctionAttributes` nonterminal, with the following structure:
 *
 * ```ebnf
 * (* Introduced in 0.6.0 *)
 * ReceiveFunctionAttributes = (* item: *) ReceiveFunctionAttribute*;
 * ```
 */
pub type ReceiveFunctionAttributes = Vec<ReceiveFunctionAttribute>;

/**
 * This node represents a `ModifierAttributes` nonterminal, with the following structure:
 *
 * ```ebnf
 * ModifierAttributes = (* item: *) ModifierAttribute*;
 * ```
 */
pub type ModifierAttributes = Vec<ModifierAttribute>;

/**
 * This node represents a `FunctionTypeAttributes` nonterminal, with the following structure:
 *
 * ```ebnf
 * FunctionTypeAttributes = (* item: *) FunctionTypeAttribute*;
 * ```
 */
pub type FunctionTypeAttributes = Vec<FunctionTypeAttribute>;

/**
 * This node represents a `Statements` nonterminal, with the following structure:
 *
 * ```ebnf
 * Statements = (* item: *) Statement*;
 * ```
 */
pub type Statements = Vec<Statement>;

/**
 * This node represents a `CatchClauses` nonterminal, with the following structure:
 *
 * ```ebnf
 * (* Introduced in 0.6.0 *)
 * CatchClauses = (* item: *) CatchClause+;
 * ```
 */
pub type CatchClauses = Vec<CatchClause>;

/**
 * This node represents a `StringLiterals` nonterminal, with the following structure:
 *
 * ```ebnf
 * (* Introduced in 0.5.14 *)
 * StringLiterals = (* item: *) StringLiteral+;
 * ```
 */
pub type StringLiterals = Vec<StringLiteral>;

/**
 * This node represents a `HexStringLiterals` nonterminal, with the following structure:
 *
 * ```ebnf
 * (* Introduced in 0.5.14 *)
 * HexStringLiterals = (* item: *) HexStringLiteral+;
 * ```
 */
pub type HexStringLiterals = Vec<HexStringLiteral>;

/**
 * This node represents a `UnicodeStringLiterals` nonterminal, with the following structure:
 *
 * ```ebnf
 * (* Introduced in 0.7.0 *)
 * UnicodeStringLiterals = (* item: *) UnicodeStringLiteral+;
 * ```
 */
pub type UnicodeStringLiterals = Vec<UnicodeStringLiteral>;

/**
 * This node represents a `YulStatements` nonterminal, with the following structure:
 *
 * ```ebnf
 * YulStatements = (* item: *) YulStatement*;
 * ```
 */
pub type YulStatements = Vec<YulStatement>;

/**
 * This node represents a `YulSwitchCases` nonterminal, with the following structure:
 *
 * ```ebnf
 * YulSwitchCases = (* item: *) YulSwitchCase+;
 * ```
 */
pub type YulSwitchCases = Vec<YulSwitchCase>;

//
// Separated:
//

/**
 * This node represents a `VersionExpressionSets` nonterminal, with the following structure:
 *
 * ```ebnf
 * VersionExpressionSets = (* item: *) VersionExpressionSet ((* separator: *) BAR_BAR (* item: *) VersionExpressionSet)*;
 * ```
 */
pub type VersionExpressionSets = Vec<VersionExpressionSet>;

/**
 * This node represents a `SimpleVersionLiteral` nonterminal, with the following structure:
 *
 * ```ebnf
 * SimpleVersionLiteral = (* item: *) VERSION_SPECIFIER ((* separator: *) PERIOD (* item: *) VERSION_SPECIFIER)*;
 * ```
 */
pub type SimpleVersionLiteral = Vec<Rc<TerminalNode>>;

/**
 * This node represents a `ImportDeconstructionSymbols` nonterminal, with the following structure:
 *
 * ```ebnf
 * ImportDeconstructionSymbols = (* item: *) ImportDeconstructionSymbol ((* separator: *) COMMA (* item: *) ImportDeconstructionSymbol)*;
 * ```
 */
pub type ImportDeconstructionSymbols = Vec<ImportDeconstructionSymbol>;

/**
 * This node represents a `UsingDeconstructionSymbols` nonterminal, with the following structure:
 *
 * ```ebnf
 * (* Introduced in 0.8.13 *)
 * UsingDeconstructionSymbols = (* item: *) UsingDeconstructionSymbol ((* separator: *) COMMA (* item: *) UsingDeconstructionSymbol)*;
 * ```
 */
pub type UsingDeconstructionSymbols = Vec<UsingDeconstructionSymbol>;

/**
 * This node represents a `InheritanceTypes` nonterminal, with the following structure:
 *
 * ```ebnf
 * InheritanceTypes = (* item: *) InheritanceType ((* separator: *) COMMA (* item: *) InheritanceType)*;
 * ```
 */
pub type InheritanceTypes = Vec<InheritanceType>;

/**
 * This node represents a `EnumMembers` nonterminal, with the following structure:
 *
 * ```ebnf
 * EnumMembers = ((* item: *) IDENTIFIER ((* separator: *) COMMA (* item: *) IDENTIFIER)*)?;
 * ```
 */
pub type EnumMembers = Vec<Rc<TerminalNode>>;

/**
 * This node represents a `Parameters` nonterminal, with the following structure:
 *
 * ```ebnf
 * Parameters = ((* item: *) Parameter ((* separator: *) COMMA (* item: *) Parameter)*)?;
 * ```
 */
pub type Parameters = Vec<Parameter>;

/**
 * This node represents a `OverridePaths` nonterminal, with the following structure:
 *
 * ```ebnf
 * (* Introduced in 0.6.0 *)
 * OverridePaths = (* item: *) IdentifierPath ((* separator: *) COMMA (* item: *) IdentifierPath)*;
 * ```
 */
pub type OverridePaths = Vec<IdentifierPath>;

/**
 * This node represents a `EventParameters` nonterminal, with the following structure:
 *
 * ```ebnf
 * EventParameters = ((* item: *) EventParameter ((* separator: *) COMMA (* item: *) EventParameter)*)?;
 * ```
 */
pub type EventParameters = Vec<EventParameter>;

/**
 * This node represents a `ErrorParameters` nonterminal, with the following structure:
 *
 * ```ebnf
 * (* Introduced in 0.8.4 *)
 * ErrorParameters = ((* item: *) ErrorParameter ((* separator: *) COMMA (* item: *) ErrorParameter)*)?;
 * ```
 */
pub type ErrorParameters = Vec<ErrorParameter>;

/**
 * This node represents a `AssemblyFlags` nonterminal, with the following structure:
 *
 * ```ebnf
 * (* Introduced in 0.8.13 *)
 * AssemblyFlags = (* item: *) StringLiteral ((* separator: *) COMMA (* item: *) StringLiteral)*;
 * ```
 */
pub type AssemblyFlags = Vec<StringLiteral>;

/**
 * This node represents a `TupleDeconstructionElements` nonterminal, with the following structure:
 *
 * ```ebnf
 * TupleDeconstructionElements = (* item: *) TupleDeconstructionElement ((* separator: *) COMMA (* item: *) TupleDeconstructionElement)*;
 * ```
 */
pub type TupleDeconstructionElements = Vec<TupleDeconstructionElement>;

/**
 * This node represents a `PositionalArguments` nonterminal, with the following structure:
 *
 * ```ebnf
 * PositionalArguments = ((* item: *) Expression ((* separator: *) COMMA (* item: *) Expression)*)?;
 * ```
 */
pub type PositionalArguments = Vec<Expression>;

/**
 * This node represents a `NamedArguments` nonterminal, with the following structure:
 *
 * ```ebnf
 * NamedArguments = ((* item: *) NamedArgument ((* separator: *) COMMA (* item: *) NamedArgument)*)?;
 * ```
 */
pub type NamedArguments = Vec<NamedArgument>;

/**
 * This node represents a `CallOptions` nonterminal, with the following structure:
 *
 * ```ebnf
 * (* Introduced in 0.6.2 *)
 * CallOptions = (* item: *) NamedArgument ((* separator: *) COMMA (* item: *) NamedArgument)*;
 * ```
 */
pub type CallOptions = Vec<NamedArgument>;

/**
 * This node represents a `TupleValues` nonterminal, with the following structure:
 *
 * ```ebnf
 * TupleValues = (* item: *) TupleValue ((* separator: *) COMMA (* item: *) TupleValue)*;
 * ```
 */
pub type TupleValues = Vec<TupleValue>;

/**
 * This node represents a `ArrayValues` nonterminal, with the following structure:
 *
 * ```ebnf
 * ArrayValues = (* item: *) Expression ((* separator: *) COMMA (* item: *) Expression)*;
 * ```
 */
pub type ArrayValues = Vec<Expression>;

/**
 * This node represents a `IdentifierPath` nonterminal, with the following structure:
 *
 * ```ebnf
 * IdentifierPath = (* item: *) IDENTIFIER ((* separator: *) PERIOD (* item: *) IDENTIFIER)*;
 * ```
 */
pub type IdentifierPath = Vec<Rc<TerminalNode>>;

/**
 * This node represents a `YulParameters` nonterminal, with the following structure:
 *
 * ```ebnf
 * YulParameters = ((* item: *) YUL_IDENTIFIER ((* separator: *) COMMA (* item: *) YUL_IDENTIFIER)*)?;
 * ```
 */
pub type YulParameters = Vec<Rc<TerminalNode>>;

/**
 * This node represents a `YulVariableNames` nonterminal, with the following structure:
 *
 * ```ebnf
 * YulVariableNames = (* item: *) YUL_IDENTIFIER ((* separator: *) COMMA (* item: *) YUL_IDENTIFIER)*;
 * ```
 */
pub type YulVariableNames = Vec<Rc<TerminalNode>>;

/**
 * This node represents a `YulArguments` nonterminal, with the following structure:
 *
 * ```ebnf
 * YulArguments = ((* item: *) YulExpression ((* separator: *) COMMA (* item: *) YulExpression)*)?;
 * ```
 */
pub type YulArguments = Vec<YulExpression>;

/**
 * This node represents a `YulPaths` nonterminal, with the following structure:
 *
 * ```ebnf
 * YulPaths = (* item: *) YulPath ((* separator: *) COMMA (* item: *) YulPath)*;
 * ```
 */
pub type YulPaths = Vec<YulPath>;

/**
 * This node represents a `YulPath` nonterminal, with the following structure:
 *
 * ```ebnf
 * YulPath = (* item: *) YUL_IDENTIFIER ((* separator: *) PERIOD (* item: *) YUL_IDENTIFIER)*;
 * ```
 */
pub type YulPath = Vec<Rc<TerminalNode>>;
