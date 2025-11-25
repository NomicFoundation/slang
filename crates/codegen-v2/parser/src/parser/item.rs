use itertools::Itertools;
use language_v2_definition::model::{
    EnumItem, Field, Identifier, Item as LanguageItem, KeywordItem, OperatorModel, PrecedenceItem,
    RepeatedItem, SeparatedItem, StructItem,
};
use semver::Version;
use serde::Serialize;

/// Newtype for the already generated Rust code, not to be confused with regular strings.
#[derive(Serialize, Debug, Clone)]
struct RustCode(String);

#[derive(Clone, Debug, Serialize)]
pub(crate) struct LALRPOPItem {
    name: Identifier,
    producing_type: Identifier,
    options: Vec<LALRPOPOption>,
}

#[derive(Clone, Debug, Serialize)]
struct LALRPOPOption {
    attributes: RustCode,
    fields: Vec<LALRPOPField>,
    action: RustCode,
}

#[derive(Clone, Debug, Serialize)]
struct LALRPOPField {
    capturing_name: Option<Identifier>,
    rule: RustCode,
    // TODO: we may need some mutable captures, maybe not
    // mutable: bool,
}

const PRODUCING_TYPE: &str = "String";

const VERSION: Version = Version::new(0, 8, 29);

fn optional(rule: RustCode) -> RustCode {
    RustCode(format!("({})?", rule.0))
}

fn repeated(rule: RustCode, allow_empty: bool) -> RustCode {
    if allow_empty {
        RustCode(format!("({})*", rule.0))
    } else {
        RustCode(format!("({})+", rule.0))
    }
}

fn separated(rule: RustCode, separator: RustCode, allow_empty: bool) -> RustCode {
    // TODO is there a perf difference between this and `({} {})* {}`?
    let at_least_one = RustCode(format!("({} ({} {})*)", rule.0, separator.0, rule.0));
    if allow_empty {
        at_least_one
    } else {
        optional(at_least_one)
    }
}

impl TryFrom<&StructItem> for LALRPOPItem {
    fn try_from(item: &StructItem) -> Result<Self, Self::Error> {
        if item.enabled.clone().is_some_and(|v| !v.contains(&VERSION)) {
            return Err(());
        }
        let fields = item
            .fields
            .iter()
            .filter_map(|(name, field)| field_to_lalrpop_field(&name, &field));

        let option = LALRPOPOption {
            fields: fields.clone().collect(),
            action: RustCode(format!(
                "format!(\"Struct({})[{}]\", {})",
                item.name,
                fields
                    .clone()
                    .filter_map(|field| { field.capturing_name.map(|_| "{:?}".to_string()) })
                    .join(", "),
                fields
                    .into_iter()
                    .filter_map(|field| { field.capturing_name.map(|name| name.to_string()) })
                    .join(", ")
            )),
            attributes: RustCode("".to_owned()),
        };

        Ok(LALRPOPItem {
            name: item.name.clone(),
            producing_type: PRODUCING_TYPE.into(),
            options: vec![option],
        })
    }

    type Error = ();
}

impl TryFrom<&EnumItem> for LALRPOPItem {
    fn try_from(item: &EnumItem) -> Result<Self, Self::Error> {
        if item.enabled.clone().is_some_and(|v| !v.contains(&VERSION)) {
            return Err(());
        }
        let options = item
            .variants
            .iter()
            .filter_map(|variant| {
                if variant
                    .enabled
                    .clone()
                    .is_some_and(|v| !v.contains(&VERSION))
                {
                    return None;
                }
                let capturing_name = format!("_{}", variant.reference);
                Some(LALRPOPOption {
                    fields: vec![LALRPOPField {
                        capturing_name: Some(capturing_name.clone().into()),
                        rule: RustCode(variant.reference.clone().to_string()),
                    }],
                    action: RustCode(format!(
                        "format!(\"Variant({})[{{:?}}]\", {capturing_name})",
                        item.name
                    )),
                    attributes: RustCode("".to_owned()),
                })
            })
            .collect();

        Ok(LALRPOPItem {
            name: item.name.clone(),
            producing_type: PRODUCING_TYPE.into(),
            options,
        })
    }
    type Error = ();
}

impl TryFrom<&RepeatedItem> for LALRPOPItem {
    fn try_from(item: &RepeatedItem) -> Result<Self, Self::Error> {
        if item.enabled.clone().is_some_and(|v| !v.contains(&VERSION)) {
            return Err(());
        }
        let capturing_name = format!("_{}", item.reference);

        let option = LALRPOPOption {
            fields: vec![LALRPOPField {
                capturing_name: Some(capturing_name.clone().into()),
                rule: repeated(
                    RustCode(item.reference.clone().to_string()),
                    item.allow_empty.unwrap_or(false),
                ),
            }],
            action: RustCode(format!(
                "\"Repeated({})[{capturing_name}]\".to_string()",
                item.name
            )),
            attributes: RustCode("".to_owned()),
        };

        Ok(LALRPOPItem {
            name: item.name.clone(),
            producing_type: PRODUCING_TYPE.into(),
            options: vec![option],
        })
    }

    type Error = ();
}

impl TryFrom<&SeparatedItem> for LALRPOPItem {
    fn try_from(item: &SeparatedItem) -> Result<Self, Self::Error> {
        if item.enabled.clone().is_some_and(|v| !v.contains(&VERSION)) {
            return Err(());
        }
        let capturing_name = format!("_{}", item.reference);
        let option = LALRPOPOption {
            fields: vec![LALRPOPField {
                capturing_name: Some(capturing_name.clone().into()),

                rule: separated(
                    // TODO: Capture internally
                    RustCode(item.reference.clone().to_string()),
                    RustCode(item.separator.clone().to_string()),
                    item.allow_empty.unwrap_or(true),
                ),
            }],
            action: RustCode(format!(
                "\"Separated({})[{capturing_name}]\".to_string()",
                item.name
            )),
            attributes: RustCode("".to_owned()),
        };

        Ok(LALRPOPItem {
            name: item.name.clone(),
            producing_type: PRODUCING_TYPE.into(),
            options: vec![option],
        })
    }
    type Error = ();
}

impl From<&KeywordItem> for LALRPOPItem {
    fn from(item: &KeywordItem) -> Self {
        let capturing_name = format!("_{}", item.name);
        let keyword_option = |reserved: bool| {
            LALRPOPOption {
                fields: vec![LALRPOPField {
                    // Sometimes you want to capture and parse
                    capturing_name: Some(capturing_name.clone().into()),
                    rule: RustCode(format!(
                        "{}_{}",
                        item.name.clone(),
                        if reserved { "Reserved" } else { "Unreserved" }
                    )),
                }],
                action: RustCode(format!("\"Keyword[{capturing_name}]\".to_string()")),
                attributes: RustCode("".to_owned()),
            }
        };
        let mut options = vec![];
        if item.definitions.iter().any(|def| {
            def.reserved
                .clone()
                .is_none_or(|rng| rng.contains(&VERSION))
        }) {
            options.push(keyword_option(true));
        }
        if item.definitions.iter().any(|def| {
            def.reserved
                .clone()
                .is_some_and(|rng| !rng.contains(&VERSION))
        }) {
            options.push(keyword_option(false));
        }

        LALRPOPItem {
            name: item.name.clone(),
            producing_type: PRODUCING_TYPE.into(),
            options,
        }
    }
}

fn field_to_lalrpop_field(name: &Identifier, field: &Field) -> Option<LALRPOPField> {
    let capturing_name = format!("_{}", name);
    match field {
        Field::Required { reference } => Some(LALRPOPField {
            capturing_name: Some(capturing_name.clone().into()),
            rule: RustCode(reference.clone().to_string()),
        }),
        Field::Optional { reference, enabled }
            if enabled.as_ref().is_none_or(|x| x.contains(&VERSION)) =>
        {
            Some(LALRPOPField {
                capturing_name: Some(capturing_name.clone().into()),
                rule: optional(RustCode(reference.clone().to_string())),
            })
        }
        _ => None,
    }
}

// TODO: Improve this function
#[allow(clippy::too_many_lines)]
impl TryFrom<&PrecedenceItem> for LALRPOPItem {
    fn try_from(item: &PrecedenceItem) -> Result<Self, Self::Error> {
        if item.enabled.clone().is_some_and(|v| !v.contains(&VERSION)) {
            return Err(());
        }
        let primaries = item.primary_expressions.iter().filter_map(|exp| {
            if exp.enabled.clone().is_some_and(|v| !v.contains(&VERSION)) {
                return None;
            }
            let capturing_name = format!("_{}", exp.reference);
            Some(LALRPOPOption {
                fields: vec![LALRPOPField {
                    capturing_name: Some(capturing_name.clone().into()),
                    rule: RustCode(exp.reference.clone().to_string()),
                }],
                action: RustCode(format!("format!(\"Precedence[{{:?}}]\", {capturing_name})")),
                attributes: RustCode("#[precedence(level=\"0\")]".to_owned()),
            })
        });

        let mut prec_counter = 0;

        let mut precedence_expressions: Vec<LALRPOPOption> = item
            .precedence_expressions
            .iter()
            .flat_map(|prec| {
                prec_counter += 1;

                prec.operators.iter().filter_map(move |op| {
                    if op.enabled.clone().is_some_and(|v| !v.contains(&VERSION)) {
                        return None;
                    }
                    let capturing_name = format!("_{}", item.name);
                    let fields = match op.model {
                        OperatorModel::Prefix => {
                            let mut fields = op
                                .fields
                                .iter()
                                .filter_map(|(name, field)| field_to_lalrpop_field(&name, &field))
                                .collect::<Vec<_>>();

                            fields.push(LALRPOPField {
                                capturing_name: Some(capturing_name.clone().into()),
                                rule: RustCode(item.name.clone().to_string()),
                            });
                            Some(fields)
                        }
                        OperatorModel::Postfix => {
                            let mut fields = vec![LALRPOPField {
                                capturing_name: Some(capturing_name.clone().into()),
                                rule: RustCode(item.name.clone().to_string()),
                            }];

                            let mut extra_fields = op
                                .fields
                                .iter()
                                .filter_map(|(name, field)| field_to_lalrpop_field(&name, &field));

                            fields.extend(extra_fields);

                            Some(fields)
                        }
                        OperatorModel::BinaryLeftAssociative
                        | OperatorModel::BinaryRightAssociative => {
                            let op_ident: Identifier = "operator".into();

                            debug_assert!(op.fields.len() == 1);
                            debug_assert!(op.fields.contains_key(&op_ident));

                            let operator = field_to_lalrpop_field(
                                &op_ident,
                                op.fields.get(&op_ident).unwrap(),
                            );

                            operator.map(|op| {
                                vec![
                                    LALRPOPField {
                                        capturing_name: Some(format!("{capturing_name}_1").into()),
                                        rule: RustCode(item.name.clone().to_string()),
                                    },
                                    op,
                                    LALRPOPField {
                                        capturing_name: Some(format!("{capturing_name}_2").into()),
                                        rule: RustCode(item.name.clone().to_string()),
                                    },
                                ]
                            })
                        }
                    };

                    let attributes = match op.model {
                        OperatorModel::Prefix | OperatorModel::Postfix => {
                            RustCode(format!("#[precedence(level=\"{}\")]", prec_counter))
                        }
                        OperatorModel::BinaryLeftAssociative => RustCode(format!(
                            "#[precedence(level=\"{prec_counter}\")] #[assoc(side=\"left\")]"
                        )),
                        OperatorModel::BinaryRightAssociative => RustCode(format!(
                            "#[precedence(level=\"{prec_counter}\")] #[assoc(side=\"right\")]"
                        )),
                    };

                    fields.map(|fields| LALRPOPOption {
                        fields,
                        action: RustCode(format!(
                            "format!(\"Precedence({})[{{:?}}]\", {capturing_name})",
                            prec.name
                        )),
                        attributes,
                    })
                })
            })
            .collect();

        precedence_expressions.extend(primaries);

        // TODO add operators
        Ok(LALRPOPItem {
            name: item.name.clone(),
            producing_type: PRODUCING_TYPE.into(),
            options: precedence_expressions,
        })
    }

    type Error = ();
}

impl TryFrom<&LanguageItem> for LALRPOPItem {
    // TODO: add error recovery
    type Error = ();

    fn try_from(item: &LanguageItem) -> Result<Self, Self::Error> {
        // TODO: we're ignoring versions for now

        // TODO: use an actual type rather than string

        match item {
            LanguageItem::Struct { item } => item.as_ref().try_into(),
            LanguageItem::Enum { item } => item.as_ref().try_into(),
            LanguageItem::Repeated { item } => item.as_ref().try_into(),
            LanguageItem::Separated { item } => item.as_ref().try_into(),
            // TODO: No idea how to do this yet
            LanguageItem::Precedence { item } => item.as_ref().try_into(),
            // I don't think we care about fragments at all
            // ... but we'll see once versioning comes in place

            // Actually, trivia, keyword, and token should translate to references
            LanguageItem::Keyword { item } => Ok(item.as_ref().into()),
            LanguageItem::Trivia { .. }
            | LanguageItem::Token { .. }
            | LanguageItem::Fragment { .. } => Err(()),
        }
    }
}
