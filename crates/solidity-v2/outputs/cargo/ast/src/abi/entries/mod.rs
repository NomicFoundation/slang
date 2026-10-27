mod serialize;

use std::fmt;
use std::sync::Arc;

use itertools::Either;
use slang_solidity_v2_common::files::FileId;
use slang_solidity_v2_common::nodes::NodeId;
use slang_solidity_v2_ir::ir;
use slang_solidity_v2_semantic::binder::Definition as SemanticDefinition;
use slang_solidity_v2_semantic::context::{AbiNameError, AbiTypeSpelling, SemanticContext};
use slang_solidity_v2_semantic::types::{
    ArraySliceType, ArrayType, FixedSizeArrayType, FunctionTypeMutability, StructType, Type, TypeId,
};

use crate::ast::{self, Definition};

/// Serializes as the JSON ABI, the array of its entries.
pub struct ContractAbi {
    node_id: NodeId,
    name: String,
    file_id: FileId,
    entries: Vec<Entry>,
    semantic: Arc<SemanticContext>,
}

impl ContractAbi {
    /// Builds the ABI with its entries in the order [`Self::entries`] describes.
    pub(crate) fn new(
        node_id: NodeId,
        name: String,
        file_id: FileId,
        mut entries: Vec<Entry>,
        semantic: &Arc<SemanticContext>,
    ) -> Self {
        entries.sort_by(|this, other| this.sort_key().cmp(&other.sort_key()));
        order_overloads_by_selector(&mut entries, semantic);
        Self {
            node_id,
            name,
            file_id,
            entries,
            semantic: Arc::clone(semantic),
        }
    }

    pub fn node_id(&self) -> NodeId {
        self.node_id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn file_id(&self) -> &FileId {
        &self.file_id
    }

    /// The entries as the JSON lists them: by `type`, then by name, with a function's overloads in
    /// ascending selector order.
    pub fn entries(&self) -> AbiEntries<'_> {
        AbiEntries {
            entries: &self.entries,
            semantic: &self.semantic,
        }
    }
}

pub type AbiMutability = FunctionTypeMutability;

#[derive(Clone, Debug)]
pub(crate) struct Constructor {
    pub(super) node_id: NodeId,
    pub(super) inputs: Vec<Parameter>,
    pub(super) state_mutability: AbiMutability,
}

#[derive(Clone, Debug)]
pub(crate) struct Error {
    pub(super) node_id: NodeId,
    pub(super) name: String,
    pub(super) inputs: Vec<Parameter>,
}

#[derive(Clone, Debug)]
pub(crate) struct Event {
    pub(super) node_id: NodeId,
    pub(super) name: String,
    pub(super) inputs: Vec<Parameter>,
    pub(super) anonymous: bool,
}

#[derive(Clone, Debug)]
pub(crate) struct Fallback {
    pub(super) node_id: NodeId,
    pub(super) state_mutability: AbiMutability,
}

#[derive(Clone, Debug)]
pub(crate) struct Function {
    pub(super) node_id: NodeId,
    pub(super) name: String,
    pub(super) inputs: Vec<Parameter>,
    pub(super) outputs: Vec<Parameter>,
    pub(super) state_mutability: AbiMutability,
    pub(super) spelling_policy: SpellingPolicy,
}

/// How an entry spells its parameters' types: by name in a library, as its selectors do, or
/// canonically otherwise. Each target, JSON or selector, then has its own [`AbiTypeSpelling`].
#[derive(Clone, Copy, Debug)]
pub(crate) enum SpellingPolicy {
    Canonical,
    Library,
}

impl SpellingPolicy {
    /// The policy of a function declared in `enclosing_definition`.
    pub(crate) fn of(enclosing_definition: Option<&Definition>) -> Self {
        match enclosing_definition {
            Some(Definition::Library(_)) => Self::Library,
            _ => Self::Canonical,
        }
    }

    fn json(self) -> AbiTypeSpelling {
        match self {
            Self::Canonical => AbiTypeSpelling::Json,
            Self::Library => AbiTypeSpelling::LibraryJson,
        }
    }

    fn selector(self) -> AbiTypeSpelling {
        match self {
            Self::Canonical => AbiTypeSpelling::Selector,
            Self::Library => AbiTypeSpelling::LibrarySelector,
        }
    }
}

#[derive(Clone, Debug)]
pub(crate) struct Receive {
    pub(super) node_id: NodeId,
    pub(super) state_mutability: AbiMutability,
}

#[derive(Clone, Debug)]
pub(crate) enum Entry {
    Constructor(Constructor),
    Error(Error),
    Event(Event),
    Fallback(Fallback),
    Function(Function),
    Receive(Receive),
}

impl Entry {
    pub(crate) fn node_id(&self) -> NodeId {
        match self {
            Entry::Constructor(inner) => inner.node_id,
            Entry::Error(inner) => inner.node_id,
            Entry::Event(inner) => inner.node_id,
            Entry::Fallback(inner) => inner.node_id,
            Entry::Function(inner) => inner.node_id,
            Entry::Receive(inner) => inner.node_id,
        }
    }

    /// Sorts by `type` and then name, as solc lists entries; the node id breaks ties, until
    /// [`ContractAbi::new`] puts overloads in selector order.
    fn sort_key(&self) -> (u8, Option<&str>, NodeId) {
        match self {
            Entry::Constructor(inner) => (0, None, inner.node_id),
            Entry::Error(inner) => (1, Some(&inner.name), inner.node_id),
            Entry::Event(inner) => (2, Some(&inner.name), inner.node_id),
            Entry::Fallback(inner) => (3, None, inner.node_id),
            Entry::Function(inner) => (4, Some(&inner.name), inner.node_id),
            Entry::Receive(inner) => (5, None, inner.node_id),
        }
    }
}

/// Reorders each run of same-named functions in sorted `entries` by selector, hashing a selector
/// only for a function that has overloads.
fn order_overloads_by_selector(entries: &mut [Entry], semantic: &Arc<SemanticContext>) {
    for run in entries.chunk_by_mut(same_function_name) {
        if run.len() > 1 {
            run.sort_by_cached_key(|entry| overload_selector(entry, semantic));
        }
    }
}

fn same_function_name(this: &Entry, other: &Entry) -> bool {
    match (this, other) {
        (Entry::Function(this), Entry::Function(other)) => this.name == other.name,
        _ => false,
    }
}

fn overload_selector(entry: &Entry, semantic: &Arc<SemanticContext>) -> u32 {
    let Entry::Function(function) = entry else {
        unreachable!("only functions share a name");
    };
    let selector = match Definition::try_create(function.node_id, semantic) {
        Some(Definition::Function(definition)) => definition.compute_selector(),
        // A public state variable's getter can overload an inherited function.
        Some(Definition::StateVariable(definition)) => definition.compute_selector(),
        _ => unreachable!("an ABI function is a function or a state variable's getter"),
    };
    selector.expect("a function in the ABI is externally visible")
}

/// A parameter of an ABI entry. Its type's spellings are written from the interned `TypeId` when
/// they are asked for, rather than stored.
#[derive(Clone, Debug)]
pub(crate) struct Parameter {
    name: Option<String>,
    type_id: TypeId,
    indexed: bool,
}

impl Parameter {
    /// `None` when the type has no ABI representation, e.g. a mapping or a storage-only struct.
    pub(crate) fn new(
        name: Option<String>,
        type_id: TypeId,
        indexed: bool,
        semantic: &SemanticContext,
    ) -> Option<Self> {
        if !semantic.has_abi_type(type_id) {
            return None;
        }
        Some(Self {
            name,
            type_id,
            indexed,
        })
    }
}

/// The entries of a [`ContractAbi`].
#[derive(Clone, Copy)]
pub struct AbiEntries<'a> {
    entries: &'a [Entry],
    semantic: &'a Arc<SemanticContext>,
}

impl<'a> AbiEntries<'a> {
    pub fn iter(&self) -> impl Iterator<Item = AbiEntry<'a>> + use<'a> {
        let semantic = self.semantic;
        self.entries
            .iter()
            .map(move |entry| AbiEntry::new(entry, semantic))
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
}

/// An entry of a [`ContractAbi`].
#[derive(Clone, Copy, Debug)]
pub enum AbiEntry<'a> {
    Constructor(AbiConstructor<'a>),
    Error(AbiError<'a>),
    Event(AbiEvent<'a>),
    Fallback(AbiFallback<'a>),
    Function(AbiFunction<'a>),
    Receive(AbiReceive<'a>),
}

impl<'a> AbiEntry<'a> {
    fn new(entry: &'a Entry, semantic: &'a Arc<SemanticContext>) -> Self {
        match entry {
            Entry::Constructor(constructor) => Self::Constructor(AbiConstructor {
                constructor,
                semantic,
            }),
            Entry::Error(error) => Self::Error(AbiError { error, semantic }),
            Entry::Event(event) => Self::Event(AbiEvent { event, semantic }),
            Entry::Fallback(fallback) => Self::Fallback(AbiFallback { fallback }),
            Entry::Function(function) => Self::Function(AbiFunction { function, semantic }),
            Entry::Receive(receive) => Self::Receive(AbiReceive { receive }),
        }
    }

    pub fn node_id(&self) -> NodeId {
        match self {
            AbiEntry::Constructor(inner) => inner.node_id(),
            AbiEntry::Error(inner) => inner.node_id(),
            AbiEntry::Event(inner) => inner.node_id(),
            AbiEntry::Fallback(inner) => inner.node_id(),
            AbiEntry::Function(inner) => inner.node_id(),
            AbiEntry::Receive(inner) => inner.node_id(),
        }
    }
}

#[derive(Clone, Copy)]
pub struct AbiConstructor<'a> {
    constructor: &'a Constructor,
    semantic: &'a Arc<SemanticContext>,
}

impl<'a> AbiConstructor<'a> {
    pub fn node_id(&self) -> NodeId {
        self.constructor.node_id
    }

    pub fn inputs(&self) -> AbiParameters<'a> {
        AbiParameters::of_entry(
            &self.constructor.inputs,
            SpellingPolicy::Canonical,
            false,
            self.semantic,
        )
    }

    pub fn state_mutability(&self) -> &'a AbiMutability {
        &self.constructor.state_mutability
    }
}

#[derive(Clone, Copy)]
pub struct AbiError<'a> {
    error: &'a Error,
    semantic: &'a Arc<SemanticContext>,
}

impl<'a> AbiError<'a> {
    pub fn node_id(&self) -> NodeId {
        self.error.node_id
    }

    pub fn name(&self) -> &'a str {
        &self.error.name
    }

    pub fn inputs(&self) -> AbiParameters<'a> {
        AbiParameters::of_entry(
            &self.error.inputs,
            SpellingPolicy::Canonical,
            false,
            self.semantic,
        )
    }
}

#[derive(Clone, Copy)]
pub struct AbiEvent<'a> {
    event: &'a Event,
    semantic: &'a Arc<SemanticContext>,
}

impl<'a> AbiEvent<'a> {
    pub fn node_id(&self) -> NodeId {
        self.event.node_id
    }

    pub fn name(&self) -> &'a str {
        &self.event.name
    }

    pub fn inputs(&self) -> AbiParameters<'a> {
        AbiParameters::of_entry(
            &self.event.inputs,
            SpellingPolicy::Canonical,
            true,
            self.semantic,
        )
    }

    pub fn anonymous(&self) -> bool {
        self.event.anonymous
    }
}

#[derive(Clone, Copy)]
pub struct AbiFallback<'a> {
    fallback: &'a Fallback,
}

impl<'a> AbiFallback<'a> {
    pub fn node_id(&self) -> NodeId {
        self.fallback.node_id
    }

    pub fn state_mutability(&self) -> &'a AbiMutability {
        &self.fallback.state_mutability
    }
}

#[derive(Clone, Copy)]
pub struct AbiFunction<'a> {
    function: &'a Function,
    semantic: &'a Arc<SemanticContext>,
}

impl<'a> AbiFunction<'a> {
    pub fn node_id(&self) -> NodeId {
        self.function.node_id
    }

    pub fn name(&self) -> &'a str {
        &self.function.name
    }

    pub fn inputs(&self) -> AbiParameters<'a> {
        AbiParameters::of_entry(
            &self.function.inputs,
            self.function.spelling_policy,
            false,
            self.semantic,
        )
    }

    pub fn outputs(&self) -> AbiParameters<'a> {
        AbiParameters::of_entry(
            &self.function.outputs,
            self.function.spelling_policy,
            false,
            self.semantic,
        )
    }

    pub fn state_mutability(&self) -> &'a AbiMutability {
        &self.function.state_mutability
    }
}

#[derive(Clone, Copy)]
pub struct AbiReceive<'a> {
    receive: &'a Receive,
}

impl<'a> AbiReceive<'a> {
    pub fn node_id(&self) -> NodeId {
        self.receive.node_id
    }

    pub fn state_mutability(&self) -> &'a AbiMutability {
        &self.receive.state_mutability
    }
}

/// The parameters of an ABI entry, or the components of a `tuple` parameter.
#[derive(Clone, Copy)]
pub struct AbiParameters<'a> {
    source: ParameterSource<'a>,
    spelling_policy: SpellingPolicy,
    semantic: &'a Arc<SemanticContext>,
}

#[derive(Clone, Copy)]
enum ParameterSource<'a> {
    /// An entry's parameters. Only an event's inputs are marked `indexed` in the JSON.
    Entry {
        parameters: &'a [Parameter],
        with_indexed: bool,
    },
    /// The members of the struct behind a `tuple`.
    Members(&'a [ir::StructMember]),
}

impl<'a> AbiParameters<'a> {
    fn of_entry(
        parameters: &'a [Parameter],
        spelling_policy: SpellingPolicy,
        with_indexed: bool,
        semantic: &'a Arc<SemanticContext>,
    ) -> Self {
        Self {
            source: ParameterSource::Entry {
                parameters,
                with_indexed,
            },
            spelling_policy,
            semantic,
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = AbiParameter<'a>> + use<'a> {
        let (spelling_policy, semantic) = (self.spelling_policy, self.semantic);
        match self.source {
            ParameterSource::Entry {
                parameters,
                with_indexed,
            } => Either::Left(parameters.iter().map(move |parameter| AbiParameter {
                name: parameter.name.as_deref(),
                type_id: parameter.type_id,
                indexed: with_indexed.then_some(parameter.indexed),
                spelling_policy,
                semantic,
            })),
            ParameterSource::Members(members) => Either::Right(members.iter().map(move |member| {
                AbiParameter {
                    name: Some(member.name.unparse()),
                    type_id: semantic
                        .binder()
                        .node_typing(member.id())
                        .as_type_id()
                        .expect("a struct member in the ABI is typed"),
                    indexed: None,
                    spelling_policy,
                    semantic,
                }
            })),
        }
    }

    pub fn len(&self) -> usize {
        match self.source {
            ParameterSource::Entry { parameters, .. } => parameters.len(),
            ParameterSource::Members(members) => members.len(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

/// A parameter of an ABI entry, or a component of a `tuple` parameter.
#[derive(Clone, Copy)]
pub struct AbiParameter<'a> {
    name: Option<&'a str>,
    type_id: TypeId,
    /// `Some` only on an event's input, the only parameter the JSON marks `indexed`.
    indexed: Option<bool>,
    spelling_policy: SpellingPolicy,
    semantic: &'a Arc<SemanticContext>,
}

impl<'a> AbiParameter<'a> {
    pub fn name(&self) -> Option<&'a str> {
        self.name
    }

    pub fn type_id(&self) -> TypeId {
        self.type_id
    }

    /// The Solidity type behind the parameter.
    pub fn get_type(&self) -> ast::Type {
        ast::Type::create(self.type_id, self.semantic)
    }

    pub fn indexed(&self) -> bool {
        self.indexed.unwrap_or(false)
    }

    /// The JSON ABI `type`, e.g. `uint256[]`, or `tuple` for a struct whose members are its
    /// [`Self::components`].
    pub fn json_type_name(&self) -> String {
        self.type_name(self.spelling_policy.json())
    }

    /// The type as the entry's selector signature spells it, e.g. `(uint256,bool)[]` for an array
    /// of structs, or `L.S[]` in a library, which names user-defined types.
    pub fn selector_type_name(&self) -> String {
        self.type_name(self.spelling_policy.selector())
    }

    /// The JSON ABI `internalType`, which names the type as declared, e.g. `struct C.S[]`.
    pub fn internal_type_name(&self) -> String {
        self.semantic.type_abi_internal_name(self.type_id)
    }

    /// The members of the struct behind a `tuple` type, through any arrays, in declaration order.
    /// `None` when the type is not a `tuple`.
    pub fn components(&self) -> Option<AbiParameters<'a>> {
        let struct_id = struct_behind(self.semantic, self.type_id)?;
        let Some(SemanticDefinition::Struct(definition)) =
            self.semantic.binder().find_definition_by_id(struct_id)
        else {
            unreachable!("a struct type resolves to a struct definition");
        };
        Some(AbiParameters {
            source: ParameterSource::Members(&definition.ir_node.members),
            spelling_policy: self.spelling_policy,
            semantic: self.semantic,
        })
    }

    fn type_name(&self, spelling: AbiTypeSpelling) -> String {
        let mut name = String::new();
        self.write_type_name(spelling, &mut name)
            .expect("writing to a String never fails");
        name
    }

    fn write_type_name(&self, spelling: AbiTypeSpelling, out: &mut impl fmt::Write) -> fmt::Result {
        self.semantic
            .write_type_abi_name(self.type_id, spelling, out)
            .map_err(|error| match error {
                AbiNameError::Write(error) => error,
                // The parameter was checked to have an ABI type when it was built.
                error => unreachable!("a parameter in the ABI has an ABI type: {error}"),
            })
    }
}

/// The struct behind a `tuple` type, through any arrays.
fn struct_behind(semantic: &SemanticContext, type_id: TypeId) -> Option<NodeId> {
    match semantic.types().get_type_by_id(type_id) {
        Type::Array(ArrayType { element_type, .. })
        | Type::FixedSizeArray(FixedSizeArrayType { element_type, .. }) => {
            struct_behind(semantic, *element_type)
        }
        Type::ArraySlice(ArraySliceType { array_type_id }) => {
            struct_behind(semantic, *array_type_id)
        }
        Type::Struct(StructType { definition_id, .. }) => Some(*definition_id),
        _ => None,
    }
}

impl fmt::Debug for AbiEntries<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_list().entries(self.iter()).finish()
    }
}

impl fmt::Debug for AbiConstructor<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.constructor.fmt(f)
    }
}

impl fmt::Debug for AbiError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.error.fmt(f)
    }
}

impl fmt::Debug for AbiEvent<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.event.fmt(f)
    }
}

impl fmt::Debug for AbiFallback<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.fallback.fmt(f)
    }
}

impl fmt::Debug for AbiFunction<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.function.fmt(f)
    }
}

impl fmt::Debug for AbiReceive<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.receive.fmt(f)
    }
}

impl fmt::Debug for AbiParameters<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_list().entries(self.iter()).finish()
    }
}

impl fmt::Debug for AbiParameter<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("AbiParameter")
            .field("name", &self.name)
            .field("type_id", &self.type_id)
            .field("indexed", &self.indexed)
            .finish_non_exhaustive()
    }
}
