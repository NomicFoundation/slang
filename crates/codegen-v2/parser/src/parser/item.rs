use language_v2_definition::model::{
    EnumItem, Field, Identifier, KeywordItem, OperatorModel, PrecedenceItem, RepeatedItem,
    SeparatedItem, StructItem,
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
    name: Option<Identifier>,
    prebuild: bool,
    attributes: RustCode,
    fields: Vec<LALRPOPField>,
    forward: bool,
}

#[derive(Clone, Debug, Serialize)]
struct LALRPOPField {
    capturing_name: Option<Identifier>,
    is_optional: bool,
    rule: RustCode,
    // TODO: we may need some mutable captures, maybe not
    // mutable: bool,
}

const VERSION: Version = Version::new(0, 8, 29);

fn optional(rule: RustCode) -> RustCode {
    RustCode(format!("({})?", rule.0))
}

fn repeated(rule: RustCode, allow_empty: bool) -> RustCode {
    if allow_empty {
        RustCode(format!("Rep<{}>", rule.0))
    } else {
        RustCode(format!("RepOne<{}>", rule.0))
    }
}

/// anonymous capture inside LALRPOP macros
fn capture(rule: RustCode) -> RustCode {
    RustCode(format!("<{}>", rule.0))
}

fn separated(rule: RustCode, separator: RustCode, allow_empty: bool) -> RustCode {
    // TODO is there a perf difference between this and `({} {})* {}`?
    // Hack, I'm capturing the repeating element
    if allow_empty {
        RustCode(format!("Sep<{}, {}>", separator.0, rule.0))
    } else {
        RustCode(format!("SepOne<{}, {}>", separator.0, rule.0))
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
            name: None,
            fields: fields.clone().collect(),
            forward: false,
            prebuild: false,
            attributes: RustCode("".to_owned()),
        };

        Ok(LALRPOPItem {
            name: item.name.clone(),
            producing_type: item.name.clone(),
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
                    name: Some(variant.reference.clone()),
                    fields: fields.clone(),
                    forward: false,
                    prebuild: false,
                    attributes: RustCode("".to_owned()),
                })
            })
            .collect();

        Ok(LALRPOPItem {
            name: item.name.clone(),
            producing_type: item.name.clone(),
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
            name: None,
            fields: fields.clone(),
            forward: false,
            prebuild: false,
            attributes: RustCode("".to_owned()),
        };

        Ok(LALRPOPItem {
            name: item.name.clone(),
            producing_type: item.name.clone(),
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
            name: None,
            fields: fields.clone(),
            forward: false,
            prebuild: false,
            attributes: RustCode("".to_owned()),
        };

        Ok(LALRPOPItem {
            name: item.name.clone(),
            producing_type: item.name.clone(),
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
                prebuild: false,
                name: None,
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
                forward: true,
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
            producing_type: item.name.clone(),
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
                name: Some(exp.reference.clone()),
                fields: vec![LALRPOPField {
                    is_optional: false,
                    capturing_name: Some(capturing_name.clone().into()),
                    rule: RustCode(exp.reference.clone().to_string()),
                }],
                forward: false,
                prebuild: false,
                attributes: RustCode("".to_owned()),
            })
        })
        .collect();

    let mut prec_counter = 0;

    ans.push(LALRPOPItem {
        name: format!("{}{}", item.name, prec_counter).into(),
        producing_type: item.name.clone(),
        options: primaries,
        inline: false,
    });

    for prec in item.precedence_expressions.iter().rev() {
        let prev_name = format!("{}{}", item.name, prec_counter);
        prec_counter += 1;
        let cur_name = format!("{}{}", item.name, prec_counter);

        // Pre-process fields for operators
        let mut pre_processed_fields = if prec.operators.len() == 1 {
            // If there's a single operator, we use its fields directly
            let op = &prec.operators[0];
            if op.enabled.clone().is_some_and(|v| !v.contains(&VERSION)) {
                vec![]
            } else {
                op.fields
                    .iter()
                    .filter_map(|(name, field)| field_to_lalrpop_field(&name, &field))
                    .collect::<Vec<_>>()
            }
        } else {
            // If there are multiple operators, we create a choice between them
            let options = prec
                .operators
                .iter()
                .filter_map(|op| {
                    if op.enabled.clone().is_some_and(|v| !v.contains(&VERSION)) {
                        return None;
                    }
                    assert!(
                        op.fields.len() == 1,
                        "Expected single field operators in multi-operator precedence expressions"
                    );
                    let a = op.fields.first().unwrap();
                    assert_eq!(
                        a.0,
                        &Identifier::from("operator"),
                        "Operator field must be labeled 'operator'"
                    );
                    if let Field::Required { reference } = a.1 {
                        Some(LALRPOPOption {
                            name: Some(reference.clone()),
                            fields: vec![field_to_lalrpop_field(a.0, a.1).unwrap()],
                            forward: false,
                            prebuild: false,
                            attributes: RustCode("".to_owned()),
                        })
                    } else {
                        panic!("Operator field must be required");
                    }
                })
                .collect();
            let ident = Identifier::from(format!("{}_{}_Operator", item.name, prec.name));

            ans.push(LALRPOPItem {
                name: ident.clone(),
                producing_type: ident.clone(),
                options,
                inline: false,
            });

            // And we reference it from the single field
            let capturing_name = format!("_{}", ident);
            vec![LALRPOPField {
                capturing_name: Some(capturing_name.clone().into()),
                is_optional: false,
                rule: RustCode(ident.clone().to_string()),
            }]
        };

        // Now, we always have a single option, since we're common eliminating some choices
        let option = {
            let capturing_name = format!("_{}", item.name);

            let fields = match prec.operators[0].model {
                OperatorModel::Prefix => {
                    let mut fields = pre_processed_fields;

                    fields.push(LALRPOPField {
                        is_optional: false,
                        capturing_name: Some(capturing_name.clone().into()),
                        rule: RustCode(cur_name.clone().to_string()),
                    });
                    fields
                }
                OperatorModel::Postfix => {
                    let mut fields = vec![LALRPOPField {
                        is_optional: false,
                        capturing_name: Some(capturing_name.clone().into()),
                        rule: RustCode(cur_name.clone().to_string()),
                    }];

                    fields.extend(pre_processed_fields);

                    fields
                }
                inner_op @ (OperatorModel::BinaryLeftAssociative
                | OperatorModel::BinaryRightAssociative) => {
                    let (left_capture, right_capture) = match inner_op {
                        OperatorModel::BinaryLeftAssociative => {
                            (cur_name.clone(), prev_name.clone())
                        }
                        OperatorModel::BinaryRightAssociative => {
                            (prev_name.clone(), cur_name.clone())
                        }
                        _ => unreachable!(),
                    };

                    vec![
                        LALRPOPField {
                            is_optional: false,
                            capturing_name: Some(format!("{capturing_name}").into()),
                            rule: RustCode(left_capture.clone().to_string()),
                        },
                        pre_processed_fields.remove(0),
                        LALRPOPField {
                            is_optional: false,
                            capturing_name: Some(format!("{capturing_name}_2").into()),
                            rule: RustCode(right_capture.clone().to_string()),
                        },
                    ]
                }
            };

            LALRPOPOption {
                name: Some(prec.name.clone()),
                fields: fields.clone(),
                forward: false,
                prebuild: true,
                attributes: RustCode("".to_owned()),
            }
        };

        // Add recursive case
        let options = vec![
            option,
            LALRPOPOption {
                name: None,
                prebuild: false,
                attributes: RustCode("".to_owned()),
                fields: vec![LALRPOPField {
                    capturing_name: Some(item.name.clone()),
                    is_optional: false,
                    rule: RustCode(format!("{}{}", item.name, prec_counter - 1)),
                }],
                forward: true,
            },
        ];

        ans.push(LALRPOPItem {
            name: format!("{}{}", item.name, prec_counter).into(),
            producing_type: item.name.clone(),
            options: options,
            inline: false,
        });
    }

    // Adding the entry case
    ans.push(LALRPOPItem {
        name: item.name.clone(),
        producing_type: item.name.clone(),
        options: vec![LALRPOPOption {
            name: None,
            prebuild: false,
            attributes: RustCode("".to_owned()),
            fields: vec![LALRPOPField {
                capturing_name: Some(item.name.clone()),
                is_optional: false,
                rule: RustCode(format!("{}{}", item.name, prec_counter)),
            }],
            forward: true,
        }],
        inline: false,
    });

    ans
}
