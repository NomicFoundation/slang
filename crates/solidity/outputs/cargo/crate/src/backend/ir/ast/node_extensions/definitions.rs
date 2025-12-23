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
        self.semantic.references_binding_to(self.definition_id)
    }
}

macro_rules! define_references_method {
    ($type:ident) => {
        paste! {
            impl [<$type Struct>] {
                pub fn references(&self) -> Vec<Reference> {
                    self.semantic.references_binding_to(self.ir_node.node_id)
                }
            }
        }
    };
}

define_references_method!(ConstantDefinition);
define_references_method!(ContractDefinition);
define_references_method!(EnumDefinition);
define_references_method!(ErrorDefinition);
define_references_method!(EventDefinition);
define_references_method!(FunctionDefinition);
define_references_method!(ImportDeconstructionSymbol);
define_references_method!(InterfaceDefinition);
define_references_method!(LibraryDefinition);
define_references_method!(Parameter);
define_references_method!(PathImport);
define_references_method!(StateVariableDefinition);
define_references_method!(StructDefinition);
define_references_method!(StructMember);
define_references_method!(UserDefinedValueTypeDefinition);
define_references_method!(VariableDeclarationStatement);
define_references_method!(YulLabel);
define_references_method!(YulFunctionDefinition);
