use crate::yaml::cst;

impl cst::Node {
    pub fn field<'a>(&'a self, key: &str) -> &'a cst::NodeFieldRef {
        match self {
            Self::Object { fields, .. } => {
                return fields.get(key).expect(&format!(
                    "Key '{key}' not found in object at {:?}",
                    self.range()
                ));
            }
            _ => panic!("Unexpected node: {self:?}"),
        };
    }

    pub fn get<'a>(&'a self, key: &str) -> &'a cst::NodeRef {
        return &self.field(key).value;
    }

    pub fn zip<TData, TResult, TMapper: Fn(&cst::NodeRef, TData) -> TResult>(
        &self,
        value: Vec<TData>,
        mapper: TMapper,
    ) -> Vec<TResult> {
        match self {
            Self::Array { items, .. } => {
                assert_eq!(items.len(), value.len());
                return items
                    .into_iter()
                    .zip(value)
                    .map(|(cst_node, value)| mapper(cst_node, value))
                    .collect();
            }
            Self::Value { .. } => {
                // Yaml parser parses empty arrays as a value.
                assert_eq!(value.len(), 0);
                return vec![];
            }
            _ => panic!("Unexpected node at {:?}", self.range()),
        };
    }
}
