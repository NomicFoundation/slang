use crate::yaml::cst;

// These macros ensure field names are strongly typed, so that refactoring doesn't break them.
mod macros {
    #[macro_export]
    macro_rules! ast_value {
        ($syntax:expr, $value:expr, $key:ident, $type:ty) => {{
            let key = inflector::Inflector::to_camel_case(stringify!($key));
            let syntax = &$syntax.unwrap_field(&key).value;
            let value = $value.$key.to_owned();

            <$type>::new(syntax, value)
        }};

        ($syntax:expr, $value:expr, $key:ident) => {{
            crate::ast_value!($syntax, $value, $key, crate::validation::ast::Node<_>)
        }};
    }

    #[macro_export]
    macro_rules! ast_array {
        ($syntax:expr, $value:expr, $key:ident, $type:ty) => {{
            let key = inflector::Inflector::to_camel_case(stringify!($key));
            let syntax = &$syntax.unwrap_field(&key).value;
            let value = $value.$key;

            syntax.zip_array(value, |syntax, value| <$type>::new(syntax, value))
        }};

        ($syntax:expr, $value:expr, $key:ident) => {{
            crate::ast_array!($syntax, $value, $key, crate::validation::ast::Node<_>)
        }};
    }

    #[macro_export]
    macro_rules! ast_optional {
        ($syntax:expr, $value:expr, $key:ident, $type:ty) => {{
            $value.$key.and_then(|value| {
                let key = inflector::Inflector::to_camel_case(stringify!($key));
                let syntax = &$syntax.unwrap_field(&key).value;

                Some(<$type>::new(syntax, value))
            })
        }};

        ($syntax:expr, $value:expr, $key:ident) => {{
            crate::ast_optional!($syntax, $value, $key, crate::validation::ast::Node<_>)
        }};
    }
}

impl cst::Node {
    pub fn unwrap_field<'a>(&'a self, key: &str) -> &'a cst::NodeFieldRef {
        match self {
            Self::Object { fields, .. } => {
                return fields
                    .get(key)
                    .expect(&format!("Key '{key}' not found in object: {self:?}"));
            }
            _ => panic!("Unexpected node: {self:?}"),
        };
    }

    pub fn zip_array<TData, TResult, TMapper: Fn(&cst::NodeRef, TData) -> TResult>(
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
                    .map(|(syntax, value)| mapper(syntax, value))
                    .collect();
            }
            Self::Value { .. } => {
                // Yaml parser parses empty arrays as a value.
                assert_eq!(value.len(), 0);
                return vec![];
            }
            _ => panic!("Unexpected node: {self:?}"),
        };
    }
}
