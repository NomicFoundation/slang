#![allow(unused)]
use std::rc::Rc;

use paste::paste;

use super::nodes::{
    ConstantDefinition, ConstantDefinitionStruct, ContractDefinition, ContractDefinitionStruct,
    EnumDefinition, EnumDefinitionStruct, ErrorDefinition, ErrorDefinitionStruct, EventDefinition,
    EventDefinitionStruct, FunctionDefinition, FunctionDefinitionStruct, IdentifierPathStruct,
    ImportDeconstructionSymbol, ImportDeconstructionSymbolStruct, InterfaceDefinition,
    InterfaceDefinitionStruct, LibraryDefinition, LibraryDefinitionStruct, Parameter,
    ParameterStruct, PathImport, PathImportStruct, SourceUnitStruct, StateVariableDefinition,
    StateVariableDefinitionStruct, StructDefinition, StructDefinitionStruct, StructMember,
    StructMemberStruct, UserDefinedValueTypeDefinition, UserDefinedValueTypeDefinitionStruct,
    VariableDeclarationStatement, VariableDeclarationStatementStruct, YulFunctionDefinition,
    YulFunctionDefinitionStruct, YulLabel, YulLabelStruct,
};
use crate::backend::{binder, SemanticAnalysis};
use crate::cst::{NodeId, TerminalKind, TerminalNode};

pub type Identifier = Rc<IdentifierStruct>;

pub struct IdentifierStruct {
    ir_node: Rc<TerminalNode>,
    semantic: Rc<SemanticAnalysis>,
}

impl IdentifierStruct {
    pub(crate) fn create(ir_node: &Rc<TerminalNode>, semantic: &Rc<SemanticAnalysis>) -> Self {
        assert!(ir_node.kind == TerminalKind::Identifier);
        Self {
            ir_node: Rc::clone(ir_node),
            semantic: Rc::clone(semantic),
        }
    }

    pub fn unparse(&self) -> String {
        self.ir_node.unparse()
    }

    pub fn resolve_to_definition(&self) -> Option<Definition> {
        let reference = self
            .semantic
            .binder()
            .find_reference_by_identifier_node_id(self.ir_node.id())?;
        let definition_id = reference.resolution.as_definition_id()?;
        Some(Rc::new(DefinitionStruct::create(
            definition_id,
            &self.semantic,
        )))
    }

    pub fn is_definition(&self) -> bool {
        self.semantic
            .binder()
            .find_definition_by_identifier_node_id(self.ir_node.id())
            .is_some()
    }

    pub fn is_reference(&self) -> bool {
        self.semantic
            .binder()
            .find_reference_by_identifier_node_id(self.ir_node.id())
            .is_some()
    }
}

pub type YulIdentifierStruct = IdentifierStruct;
pub type YulIdentifier = Rc<YulIdentifierStruct>;

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
}

impl SourceUnitStruct {
    pub fn file_id(&self) -> String {
        self.semantic
            .node_id_to_file_id(self.ir_node.node_id)
            .unwrap()
    }

    pub fn contracts(&self) -> Vec<ContractDefinition> {
        self.members()
            .iter()
            .filter_map(|member| member.as_contract_definition())
            .collect()
    }
}

pub enum ContractBase {
    Contract(ContractDefinition),
    Interface(InterfaceDefinition),
}

impl ContractBase {
    fn from_definition(definition: &Definition) -> Option<Self> {
        definition
            .as_contract()
            .map(Self::Contract)
            .or_else(|| definition.as_interface().map(Self::Interface))
    }
}

impl ContractDefinitionStruct {
    pub fn direct_bases(&self) -> Vec<ContractBase> {
        self.inheritance_types()
            .iter()
            .filter_map(|inheritance_type| {
                let base = inheritance_type.type_name().resolve_to_definition()?;
                ContractBase::from_definition(&base)
            })
            .collect()
    }

    pub fn linearised_bases(&self) -> Vec<ContractBase> {
        let Some(base_node_ids) = self
            .semantic
            .binder()
            .get_linearised_bases(self.ir_node.node_id)
        else {
            // TODO(validation): once we have validation implemented, this
            // branch should not be reachable, or we should generate an error
            // while building the `SemanticAnalysis`.
            return Vec::new();
        };
        base_node_ids
            .iter()
            .map(|node_id| {
                let base_definition = Rc::new(DefinitionStruct::create(*node_id, &self.semantic));
                ContractBase::from_definition(&base_definition)
                    .expect("Linearised base is either a contract or interface")
            })
            .collect()
    }
}

impl IdentifierPathStruct {
    pub fn unparse(&self) -> String {
        self.ir_nodes
            .iter()
            .map(|ir_node| ir_node.unparse())
            .collect::<Vec<_>>()
            .join(".")
    }

    pub fn resolve_to_definition(&self) -> Option<Definition> {
        let ir_node = self.ir_nodes.last()?;
        let reference = self
            .semantic
            .binder()
            .find_reference_by_identifier_node_id(ir_node.id())?;
        let definition_id = reference.resolution.as_definition_id()?;
        Some(Rc::new(DefinitionStruct::create(
            definition_id,
            &self.semantic,
        )))
    }
}
