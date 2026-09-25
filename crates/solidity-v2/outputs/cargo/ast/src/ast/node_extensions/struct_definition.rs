use slang_solidity_v2_common::collections::Map;
use slang_solidity_v2_common::nodes::NodeId;

use super::super::{Definition, StructDefinition, StructDefinitionStruct, Type};

impl StructDefinitionStruct {
    /// Whether the struct is one solc marks `recursive`, or one on a cycle closed by a
    /// function type among the structs solc leaves unmarked.
    pub fn is_recursive(&self) -> bool {
        self.reaches_cycle()
            || self
                .reachable(true, |named| !named.reaches_cycle())
                .contains_key(&self.node_id())
    }

    /// solc's `recursive`: whether a cycle of struct members is reachable through nested
    /// structs, arrays and mappings.
    fn reaches_cycle(&self) -> bool {
        self.reachable(false, |_| true).values().any(|named| {
            named
                .reachable(false, |_| true)
                .contains_key(&named.node_id())
        })
    }

    /// The structs reachable through the members' types, by node id, entering only those
    /// `entered` admits.
    fn reachable(
        &self,
        through_function_types: bool,
        entered: impl Fn(&StructDefinition) -> bool,
    ) -> Map<NodeId, StructDefinition> {
        let mut reached = Map::default();
        let mut pending = self.named_structs(through_function_types);
        while let Some(named) = pending.pop() {
            if reached.contains_key(&named.node_id()) || !entered(&named) {
                continue;
            }
            pending.extend(named.named_structs(through_function_types));
            reached.insert(named.node_id(), named);
        }
        reached
    }

    /// The structs the members name in their types.
    fn named_structs(&self, through_function_types: bool) -> Vec<StructDefinition> {
        self.members()
            .iter()
            .filter_map(|member| member.get_type())
            .flat_map(|member_type| Self::named_in(&member_type, through_function_types))
            .collect()
    }

    /// The structs a type names through arrays, mappings, tuples and, when
    /// `through_function_types`, function types.
    fn named_in(value: &Type, through_function_types: bool) -> Vec<StructDefinition> {
        match value {
            Type::Struct(struct_type) => {
                let Definition::Struct(definition) = struct_type.definition() else {
                    unreachable!("a struct type resolves to a struct definition");
                };
                vec![definition]
            }
            Type::Array(array) => Self::named_in(&array.element_type(), through_function_types),
            Type::FixedSizeArray(array) => {
                Self::named_in(&array.element_type(), through_function_types)
            }
            Type::Mapping(mapping) => Self::named_in(&mapping.value_type(), through_function_types),
            Type::Tuple(tuple) => tuple
                .types()
                .iter()
                .flat_map(|element| Self::named_in(element, through_function_types))
                .collect(),
            Type::Function(function) if through_function_types => function
                .parameter_types()
                .iter()
                .chain(std::iter::once(&function.return_type()))
                .flat_map(|parameter| Self::named_in(parameter, true))
                .collect(),
            Type::Address(_)
            | Type::ArraySlice(_)
            | Type::Boolean(_)
            | Type::ByteArray(_)
            | Type::Bytes(_)
            | Type::Contract(_)
            | Type::Enum(_)
            | Type::Error(_)
            | Type::Event(_)
            | Type::FixedPointNumber(_)
            | Type::Function(_)
            | Type::Integer(_)
            | Type::Interface(_)
            | Type::Library(_)
            | Type::Literal(_)
            | Type::MetaType(_)
            | Type::String(_)
            | Type::UserDefinedValue(_)
            | Type::UserMetaType(_)
            | Type::Void(_) => Vec::new(),
        }
    }
}
