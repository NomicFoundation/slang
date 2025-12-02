use std::iter::once;

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
    pub name: Identifier,
    producing_type: Identifier,
    options: Vec<LALRPOPOption>,
    pub inline: bool,
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
    is_optional: bool,
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

/// anonymous capture inside LALRPOP macros
fn capture(rule: RustCode) -> RustCode {
    RustCode(format!("<{}>", rule.0))
}

fn separated(rule: RustCode, separator: RustCode, allow_empty: bool) -> RustCode {
    // TODO is there a perf difference between this and `({} {})* {}`?
    // Hack, I'm capturing the repeating element
    let at_least_one = RustCode(format!("({} <({} {})*>)", rule.0, separator.0, rule.0));
    if allow_empty {
        optional(at_least_one)
    } else {
        at_least_one
    }
}

fn to_action(
    item_kind: &str,
    item_name: &str,
    captures: impl Iterator<Item = String>,
    actions: impl Iterator<Item = String>,
) -> RustCode {
    RustCode(format!(
        r#"format!("{{{{ \"item\": \"{}\", \"name\": \"{}\", {} }}}}", {})"#,
        item_kind,
        item_name,
        captures.map(|c| format!(r#"\"{}\": {{}}"#, c)).join(", "),
        actions.map(|c| c.to_string()).join(", ")
    ))
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
            action: to_action(
                "Struct",
                &item.name,
                fields
                    .clone()
                    .into_iter()
                    .filter_map(|field| field.capturing_name.map(|name| name.to_string())),
                fields
                    .into_iter()
                    .filter_map(|field| match field.capturing_name {
                        Some(name) if field.is_optional => {
                            Some(format!("{}.unwrap_or(\"\\\"\\\"\".into())", name))
                        }
                        Some(name) => Some(name.to_string()),
                        None => None,
                    }),
            ),
            attributes: RustCode("".to_owned()),
        };

        Ok(LALRPOPItem {
            name: item.name.clone(),
            producing_type: PRODUCING_TYPE.into(),
            options: vec![option],
            inline: false,
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
                let fields = vec![LALRPOPField {
                    is_optional: false,
                    capturing_name: Some(capturing_name.clone().into()),
                    rule: RustCode(variant.reference.clone().to_string()),
                }];
                Some(LALRPOPOption {
                    fields: fields.clone(),
                    action: to_action(
                        "Variant",
                        &item.name,
                        once(capturing_name.clone()),
                        once(capturing_name),
                    ),
                    attributes: RustCode("".to_owned()),
                })
            })
            .collect();

        Ok(LALRPOPItem {
            name: item.name.clone(),
            producing_type: PRODUCING_TYPE.into(),
            options,
            inline: false,
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
        let fields = vec![LALRPOPField {
            is_optional: false,
            capturing_name: Some(capturing_name.clone().into()),
            rule: repeated(
                capture(RustCode(item.reference.clone().to_string())),
                item.allow_empty.unwrap_or(false),
            ),
        }];
        let option = LALRPOPOption {
            fields: fields.clone(),
            action: to_action(
                "Repeated",
                &item.name,
                once(capturing_name.clone()),
                once(format!(
                    "format!(\"[{{}}]\", {}.join(\", \"))",
                    capturing_name
                )),
            ),
            attributes: RustCode("".to_owned()),
        };

        Ok(LALRPOPItem {
            name: item.name.clone(),
            producing_type: PRODUCING_TYPE.into(),
            options: vec![option],
            inline: false,
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
        let fields = vec![LALRPOPField {
            is_optional: false,
            capturing_name: Some(capturing_name.clone().into()),

            rule: separated(
                // Don't capture separator
                capture(RustCode(item.reference.clone().to_string())),
                RustCode(item.separator.clone().to_string()),
                item.allow_empty.unwrap_or(false),
            ),
        }];
        let option = LALRPOPOption {
            fields: fields.clone(),
            action: to_action(
                "Separated",
                &item.name,
                once(capturing_name.clone()),
                if item.allow_empty.unwrap_or(false) {
                    // If it's allowed to be empty, then we have an Option<(String, Vec<String>)>
                    // TODO: This is not ideal, we should make something smarter later on
                    once(format!(
                        "{capturing_name}.map(|x| format!(\"[{{}}, {{}}]\", x.0, x.1.iter().join(\", \"))).unwrap_or(\"[]\".into())"
                    ))
                } else {
                    // If it's not allowed to be empty, then we have an (String, Vec<String>)
                    once(format!(
                        "format!(\"[{{}}, {{}}]\", {capturing_name}.0, {capturing_name}.1.join(\", \"))"
                    ))
                },
            ),
            attributes: RustCode("".to_owned()),
        };

        Ok(LALRPOPItem {
            name: item.name.clone(),
            producing_type: PRODUCING_TYPE.into(),
            options: vec![option],
            inline: false,
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
                    is_optional: false,
                    // Sometimes you want to capture and parse
                    capturing_name: Some(capturing_name.clone().into()),
                    rule: RustCode(format!(
                        "{}_{}",
                        item.name.clone(),
                        if reserved { "Reserved" } else { "Unreserved" }
                    )),
                }],
                action: to_action(
                    "Keyword",
                    &item.name,
                    once(capturing_name.clone()),
                    once(format!("format!(\"{{}}\", {})", capturing_name.clone())),
                ),
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
            inline: true,
        }
    }
}

fn field_to_lalrpop_field(name: &Identifier, field: &Field) -> Option<LALRPOPField> {
    let capturing_name = format!("_{}", name);
    match field {
        Field::Required { reference } => Some(LALRPOPField {
            capturing_name: Some(capturing_name.clone().into()),
            is_optional: false,
            rule: RustCode(reference.clone().to_string()),
        }),
        Field::Optional { reference, enabled }
            if enabled.as_ref().is_none_or(|x| x.contains(&VERSION)) =>
        {
            Some(LALRPOPField {
                capturing_name: Some(capturing_name.clone().into()),
                is_optional: true,
                rule: optional(RustCode(reference.clone().to_string())),
            })
        }
        _ => None,
    }
}

// TODO: Improve this function
#[allow(clippy::too_many_lines)]
pub(crate) fn precedence_item_to_lalrpop_items(item: &PrecedenceItem) -> Vec<LALRPOPItem> {
    if item.enabled.clone().is_some_and(|v| !v.contains(&VERSION)) {
        return vec![];
    }
    let mut ans = Vec::new();

    let primaries: Vec<LALRPOPOption> = item
        .primary_expressions
        .iter()
        .filter_map(|exp| {
            if exp.enabled.clone().is_some_and(|v| !v.contains(&VERSION)) {
                return None;
            }
            let capturing_name = format!("_{}", exp.reference);
            Some(LALRPOPOption {
                fields: vec![LALRPOPField {
                    is_optional: false,
                    capturing_name: Some(capturing_name.clone().into()),
                    rule: RustCode(exp.reference.clone().to_string()),
                }],
                action: to_action(
                    "Precedence-primary",
                    &item.name,
                    once(capturing_name.clone()),
                    once(capturing_name.clone()),
                ),
                attributes: RustCode("".to_owned()),
            })
        })
        .collect();

    let mut prec_counter = 0;

    ans.push(LALRPOPItem {
        name: format!("{}{}", item.name, prec_counter).into(),
        producing_type: PRODUCING_TYPE.into(),
        options: primaries,
        inline: false,
    });

    for prec in &item.precedence_expressions {
        let prev_name = format!("{}{}", item.name, prec_counter);
        prec_counter += 1;
        let cur_name = format!("{}{}", item.name, prec_counter);

        let mut options: Vec<LALRPOPOption> = prec
            .operators
            .iter()
            .filter_map(move |op| {
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
                            is_optional: false,
                            capturing_name: Some(capturing_name.clone().into()),
                            rule: RustCode(cur_name.clone().to_string()),
                        });
                        Some(fields)
                    }
                    OperatorModel::Postfix => {
                        let mut fields = vec![LALRPOPField {
                            is_optional: false,
                            capturing_name: Some(capturing_name.clone().into()),
                            rule: RustCode(cur_name.clone().to_string()),
                        }];

                        let extra_fields = op
                            .fields
                            .iter()
                            .filter_map(|(name, field)| field_to_lalrpop_field(&name, &field));

                        fields.extend(extra_fields);

                        Some(fields)
                    }
                    inner_op @ (OperatorModel::BinaryLeftAssociative
                    | OperatorModel::BinaryRightAssociative) => {
                        let op_ident: Identifier = "operator".into();

                        debug_assert!(op.fields.len() == 1);
                        debug_assert!(op.fields.contains_key(&op_ident));

                        let operator =
                            field_to_lalrpop_field(&op_ident, op.fields.get(&op_ident).unwrap());

                        let (left_capture, right_capture) = match inner_op {
                            OperatorModel::BinaryLeftAssociative => {
                                (cur_name.clone(), prev_name.clone())
                            }
                            OperatorModel::BinaryRightAssociative => {
                                (prev_name.clone(), cur_name.clone())
                            }
                            _ => unreachable!(),
                        };

                        operator.map(|op| {
                            vec![
                                LALRPOPField {
                                    is_optional: false,
                                    capturing_name: Some(format!("{capturing_name}").into()),
                                    rule: RustCode(left_capture.clone().to_string()),
                                },
                                op,
                                LALRPOPField {
                                    is_optional: false,
                                    capturing_name: Some(format!("{capturing_name}_2").into()),
                                    rule: RustCode(right_capture.clone().to_string()),
                                },
                            ]
                        })
                    }
                };

                fields.map(|fields| LALRPOPOption {
                    fields: fields.clone(),
                    action: to_action(
                        "Precedence-Operator",
                        &item.name,
                        fields
                            .clone()
                            .into_iter()
                            .filter_map(|field| field.capturing_name.map(|name| name.to_string())),
                        fields
                            .into_iter()
                            .filter_map(|field| match field.capturing_name {
                                Some(name) if field.is_optional => {
                                    Some(format!("{}.unwrap_or(\"\\\"\\\"\".into())", name))
                                }
                                Some(name) => Some(name.to_string()),
                                None => None,
                            }),
                    ),
                    attributes: RustCode("".to_owned()),
                })
            })
            .collect();

        // Add next case
        options.push(LALRPOPOption {
            attributes: RustCode("".to_owned()),
            fields: vec![LALRPOPField {
                capturing_name: Some(item.name.clone()),
                is_optional: false,
                rule: RustCode(format!("{}{}", item.name, prec_counter - 1)),
            }],
            action: RustCode("<>".to_owned()),
        });

        ans.push(LALRPOPItem {
            name: format!("{}{}", item.name, prec_counter).into(),
            producing_type: PRODUCING_TYPE.into(),
            options: options,
            inline: false,
        });
    }

    // Adding the entry case
    ans.push(LALRPOPItem {
        name: item.name.clone(),
        producing_type: PRODUCING_TYPE.into(),
        options: vec![LALRPOPOption {
            attributes: RustCode("".to_owned()),
            fields: vec![LALRPOPField {
                capturing_name: Some(item.name.clone()),
                is_optional: false,
                rule: RustCode(format!("{}{}", item.name, prec_counter)),
            }],
            action: RustCode("<>".to_owned()),
        }],
        inline: false,
    });

    ans
}
