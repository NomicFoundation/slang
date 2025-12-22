use std::rc::Rc;

use paste::paste;

use super::super::nodes::{
    ConstantDefinition, ConstantDefinitionStruct, ContractDefinition, ContractDefinitionStruct,
    EnumDefinition, EnumDefinitionStruct, ErrorDefinition, ErrorDefinitionStruct, EventDefinition,
    EventDefinitionStruct, FunctionDefinition, FunctionDefinitionStruct,
    ImportDeconstructionSymbol, ImportDeconstructionSymbolStruct, InterfaceDefinition,
    InterfaceDefinitionStruct, LibraryDefinition, LibraryDefinitionStruct, Parameter,
    ParameterStruct, PathImport, PathImportStruct, StateVariableDefinition,
    StateVariableDefinitionStruct, StructDefinition, StructDefinitionStruct, StructMember,
    StructMemberStruct, UserDefinedValueTypeDefinition, UserDefinedValueTypeDefinitionStruct,
    VariableDeclarationStatement, VariableDeclarationStatementStruct, YulFunctionDefinition,
    YulFunctionDefinitionStruct, YulLabel, YulLabelStruct,
};
use super::{Identifier, IdentifierStruct, Reference};
use crate::backend::{binder, SemanticAnalysis};
use crate::cst::NodeId;

pub type Definition = Rc<DefinitionStruct>;

pub struct DefinitionStruct {
    definition_id: NodeId,
    semantic: Rc<SemanticAnalysis>,
}

macro_rules! define_variant {
    ($variant:ident) => {
        paste! {
            pub fn [<is_ $variant:snake:lower>](&self) -> bool {
                matches!(self.internal_definition(), binder::Definition::$variant(_))
            }

            pub fn [<as_ $variant:snake:lower>](&self) -> Option<[<$variant Definition>]> {
                if let binder::Definition::$variant(variant) = self.internal_definition() {
                    Some(Rc::new([<$variant DefinitionStruct>]::create(&variant.ir_node, &self.semantic)))
                } else {
                    None
                }
            }
        }
    };
    ($variant:ident, $deftype:ident) => {
        paste! {
            pub fn [<is_ $variant:snake:lower>](&self) -> bool {
                matches!(self.internal_definition(), binder::Definition::$variant(_))
            }

            pub fn [<as_ $variant:snake:lower>](&self) -> Option<$deftype> {
                if let binder::Definition::$variant(variant) = self.internal_definition() {
                    Some(Rc::new([<$deftype Struct>]::create(&variant.ir_node, &self.semantic)))
                } else {
                    None
                }
            }
        }
    };
}

impl DefinitionStruct {
    pub(crate) fn create(definition_id: NodeId, semantic: &Rc<SemanticAnalysis>) -> Self {
        assert!(semantic
            .binder()
            .find_definition_by_id(definition_id)
            .is_some());
        Self {
            definition_id,
            semantic: Rc::clone(semantic),
        }
    }

    fn internal_definition(&self) -> &binder::Definition {
        self.semantic
            .binder()
            .find_definition_by_id(self.definition_id)
            .unwrap()
    }

    define_variant!(Constant);
    define_variant!(Contract);
    define_variant!(Enum);
    define_variant!(EnumMember, Identifier);
    define_variant!(Error);
    define_variant!(Event);
    define_variant!(Function);
    define_variant!(Import, PathImport);
    define_variant!(ImportedSymbol, ImportDeconstructionSymbol);
    define_variant!(Interface);
    define_variant!(Library);
    define_variant!(Modifier, FunctionDefinition);
    define_variant!(Parameter, Parameter);
    define_variant!(StateVariable);
    define_variant!(Struct);
    define_variant!(StructMember, StructMember);
    define_variant!(TypeParameter, Parameter);
    define_variant!(UserDefinedValueType);
    define_variant!(Variable, VariableDeclarationStatement);
    define_variant!(YulFunction);
    define_variant!(YulLabel, YulLabel);
    define_variant!(YulParameter, Identifier);
    define_variant!(YulVariable, Identifier);

    pub fn references(&self) -> Vec<Reference> {
        self.semantic
            .binder()
            .get_references_by_definition_id(self.definition_id)
            .iter()
            .filter_map(|node_id| {
                self.semantic
                    .binder()
                    .find_reference_by_identifier_node_id(*node_id)
                    .and_then(|reference| {
                        Reference::try_create(&reference.identifier, &self.semantic)
                    })
            })
            .collect()
    }
}

macro_rules! definition_to_references {
    ($type:ident) => {
        paste! {
            impl [<$type Struct>] {
                pub fn references(&self) -> Vec<Reference> {
                    self.semantic
                        .binder()
                        .get_references_by_definition_id(self.ir_node.node_id)
                        .iter()
                        .filter_map(|node_id| {
                            self.semantic
                                .binder()
                                .find_reference_by_identifier_node_id(*node_id)
                                .and_then(|reference| {
                                    Reference::try_create(&reference.identifier, &self.semantic)
                                })
                        })
                        .collect()
                }
            }
        }
    };
}

definition_to_references!(ConstantDefinition);
definition_to_references!(ContractDefinition);
definition_to_references!(EnumDefinition);
definition_to_references!(ErrorDefinition);
definition_to_references!(EventDefinition);
definition_to_references!(FunctionDefinition);
definition_to_references!(ImportDeconstructionSymbol);
definition_to_references!(InterfaceDefinition);
definition_to_references!(LibraryDefinition);
definition_to_references!(Parameter);
definition_to_references!(PathImport);
definition_to_references!(StateVariableDefinition);
definition_to_references!(StructDefinition);
definition_to_references!(StructMember);
definition_to_references!(UserDefinedValueTypeDefinition);
definition_to_references!(VariableDeclarationStatement);
definition_to_references!(YulLabel);
definition_to_references!(YulFunctionDefinition);
