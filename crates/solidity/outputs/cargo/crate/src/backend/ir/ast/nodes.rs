#![allow(unused)]
use std::rc::Rc;

use paste::paste;

use super::input as input_ir;
use crate::backend::{binder, SemanticAnalysis};
use crate::cst::{NodeId, TerminalKind, TerminalNode};

pub type Identifier = Rc<IdentifierStruct>;

pub struct IdentifierStruct {
    ir_node: Rc<TerminalNode>,
    semantic: Rc<SemanticAnalysis>,
}

impl IdentifierStruct {
    fn create(ir_node: &Rc<TerminalNode>, semantic: &Rc<SemanticAnalysis>) -> Self {
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
}

pub type IdentifierPath = Rc<IdentifierPathStruct>;

pub struct IdentifierPathStruct {
    ir_nodes: Vec<Rc<TerminalNode>>,
    semantic: Rc<SemanticAnalysis>,
}

impl IdentifierPathStruct {
    fn create(ir_nodes: &[Rc<TerminalNode>], semantic: &Rc<SemanticAnalysis>) -> Self {
        Self {
            ir_nodes: ir_nodes.to_vec(),
            semantic: Rc::clone(semantic),
        }
    }

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

    pub fn parts(&self) -> impl Iterator<Item = Identifier> + use<'_> {
        self.ir_nodes
            .iter()
            .map(|ir_node| Rc::new(IdentifierStruct::create(ir_node, &self.semantic)))
    }
}

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

include!("ast_nodes.generated.rs");

impl SourceUnitStruct {
    pub fn file_id(&self) -> String {
        self.semantic
            .node_id_to_file_id(self.ir_node.node_id)
            .unwrap()
    }

    pub fn contracts(&self) -> impl Iterator<Item = ContractDefinition> + use<'_> {
        self.members()
            .filter_map(|member| member.as_contract_definition())
    }
}
