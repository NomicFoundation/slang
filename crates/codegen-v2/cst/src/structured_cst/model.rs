//! This model abstracts away some details from our language definition in order to
//! facilitate generation of CST structures
//!
//! Note: This is a copy of the model in v1 (crates/codegen/generator/src/ir/model.rs) with
//! some small changes

// TODO(v2):
// - Collect the sizes of terminals and nonterminals, this should allow us to optimize space usage
//   in particular, terminals with size 1 can be represented as `()` (or anything 0 sized) and
//   terminals with size N can be represented with an enum

use std::collections::{BTreeMap, BTreeSet};

use language_v2_definition::model;
use serde::ser::SerializeMap;
use serde::Serialize;

use super::builder::StructuredCstModelBuilder;
use super::versioned_descendants::compute_has_versioned_descendant;

#[derive(Default, Serialize)]
pub struct StructuredCstModel {
    /// Terminal nodes.
    pub terminals: BTreeSet<model::Identifier>,

    /// Nonterminal nodes that are a fixed size group of potentially different nodes
    /// ie a struct
    pub sequences: BTreeMap<model::Identifier, Sequence>,

    /// Nonterminal nodes that are a choice between other nodes
    /// ie an enum
    pub choices: BTreeMap<model::Identifier, Choice>,

    /// Nonterminal nodes that are an unbounded collections of nodes of the same type
    /// ie a vector
    pub collections: BTreeMap<model::Identifier, Collection>,

    /// Whether each nonterminal has any version-gated descendants (transitively).
    pub has_versioned_descendant: BTreeMap<model::Identifier, bool>,
}

#[derive(Clone, Serialize)]
pub struct Sequence {
    pub fields: Vec<Field>,
    pub enabled: model::VersionSpecifier,
}

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum NodeType {
    Nonterminal(model::Identifier),
    Terminal(model::Identifier),
}

#[derive(Clone, Serialize)]
pub struct Field {
    pub label: model::Identifier,
    pub field_type: NodeType,
    pub is_optional: bool,
    pub enabled: model::VersionSpecifier,
}

#[allow(clippy::struct_field_names)]
#[derive(Clone, Serialize)]
pub struct Choice {
    pub variants: Vec<ChoiceVariant>,
    pub enabled: model::VersionSpecifier,
}

#[derive(Clone, Serialize)]
pub struct ChoiceVariant {
    pub variant_type: NodeType,
    pub enabled: model::VersionSpecifier,
}

#[derive(Clone, Serialize)]
pub struct Collection {
    pub item_type: NodeType,
    pub enabled: model::VersionSpecifier,
    /// Only `Separated` items have a `separator_type`
    pub separator_type: Option<NodeType>,
}

impl NodeType {
    pub fn as_identifier(&self) -> &model::Identifier {
        match self {
            NodeType::Nonterminal(identifier) | NodeType::Terminal(identifier) => identifier,
        }
    }

    pub fn is_terminal(&self) -> bool {
        matches!(self, Self::Terminal(_))
    }
}

impl PartialEq<model::Identifier> for NodeType {
    fn eq(&self, other: &model::Identifier) -> bool {
        match self {
            NodeType::Nonterminal(identifier) | NodeType::Terminal(identifier) => {
                identifier == other
            }
        }
    }
}

impl Serialize for NodeType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut map = serializer.serialize_map(Some(3))?;
        let (identifier, kind, is_terminal) = match self {
            NodeType::Nonterminal(identifier) => (identifier, "Nonterminal", false),
            NodeType::Terminal(identifier) => (identifier, "Terminal", true),
        };
        map.serialize_entry("name", identifier)?;
        map.serialize_entry("kind", kind)?;
        map.serialize_entry("is_terminal", &is_terminal)?;
        map.end()
    }
}

// Construction
impl StructuredCstModel {
    pub fn from_language(language: &model::Language) -> Self {
        let builder = StructuredCstModelBuilder::create(language);
        let has_versioned_descendant = compute_has_versioned_descendant(&builder);

        Self {
            terminals: builder.terminals,
            sequences: builder.sequences,
            choices: builder.choices,
            collections: builder.collections,
            has_versioned_descendant,
        }
    }
}
