use language_v2_definition::model::{
    EnumItem, Field, Identifier, KeywordItem, OperatorModel, PrecedenceItem, RepeatedItem,
    SeparatedItem, StructItem, VersionSpecifier,
};
use semver::Version;
use serde::Serialize;

/// Newtype for the already generated Rust code, not to be confused with regular strings.
#[derive(Serialize, Debug, Clone)]
struct RustCode(String);

#[derive(Clone, Debug, Serialize)]
pub(crate) struct LALRPOPItem {
    pub name: Identifier,
    pub producing_type: Identifier,
    pub options: Vec<LALRPOPOption>,
    pub inline: bool,
    pub pubb: bool,
}

#[derive(Clone, Debug, Serialize)]
struct LALRPOPOption {
    fields: Vec<LALRPOPField>,
    // If none, then the match is forwarded
    constructor: Option<RustCode>,
}

#[derive(Clone, Debug, Serialize)]
struct LALRPOPField {
    capturing_name: Option<Identifier>,
    rule: RustCode,
}

// Fixing ourselves to this version for now
pub const VERSION: Version = Version::new(0, 8, 30);

fn is_enabled(enabled: &Option<VersionSpecifier>) -> bool {
    enabled.as_ref().is_none_or(|v| v.contains(&VERSION))
}

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
    if allow_empty {
        RustCode(format!("Sep<{}, {}>", separator.0, rule.0))
    } else {
        RustCode(format!("SepOne<{}, {}>", separator.0, rule.0))
    }
}

fn simple_match(id: &Identifier) -> RustCode {
    RustCode(format!("{}", *id))
}

// TODO transform to snake case?
fn constructor(id: &Identifier) -> RustCode {
    RustCode(format!("new_{}", *id))
}

fn variant_constructor(id: &Identifier, variant: &Identifier) -> RustCode {
    RustCode(format!("new_{}_{}", *id, *variant))
}

pub(crate) fn struct_item_to_lalrpop_items(item: &StructItem) -> Vec<LALRPOPItem> {
    if !is_enabled(&item.enabled) {
        return vec![];
    }
    let fields = item
        .fields
        .iter()
        .filter_map(|(name, field)| field_to_lalrpop_field(&name, &field));

    let option = LALRPOPOption {
        fields: fields.clone().collect(),
        constructor: Some(constructor(&item.name)),
    };

    vec![LALRPOPItem {
        name: item.name.clone(),
        producing_type: item.name.clone(),
        options: vec![option],
        inline: false,
        pubb: false,
    }]
}

pub(crate) fn enum_item_to_lalrpop_items(item: &EnumItem) -> Vec<LALRPOPItem> {
    if !is_enabled(&item.enabled) {
        return vec![];
    }
    let options = item
        .variants
        .iter()
        .filter(|variant| is_enabled(&variant.enabled))
        .map(|variant| {
            let capturing_name = format!("_{}", variant.reference);
            let fields = vec![LALRPOPField {
                capturing_name: Some(capturing_name.clone().into()),
                rule: simple_match(&variant.reference),
            }];
            LALRPOPOption {
                fields: fields.clone(),
                constructor: Some(variant_constructor(&item.name, &variant.reference)),
            }
        })
        .collect();

    vec![LALRPOPItem {
        name: item.name.clone(),
        producing_type: item.name.clone(),
        options,
        inline: false,
        pubb: false,
    }]
}

pub(crate) fn repeated_item_to_lalrpop_items(item: &RepeatedItem) -> Vec<LALRPOPItem> {
    if !is_enabled(&item.enabled) {
        return vec![];
    }
    let capturing_name = format!("_{}", item.reference);
    let fields = vec![LALRPOPField {
        capturing_name: Some(capturing_name.clone().into()),
        rule: repeated(
            capture(RustCode(item.reference.clone().to_string())),
            item.allow_empty.unwrap_or(false),
        ),
    }];
    let option = LALRPOPOption {
        fields: fields.clone(),
        constructor: Some(constructor(&item.name)),
    };

    vec![LALRPOPItem {
        name: item.name.clone(),
        producing_type: item.name.clone(),
        options: vec![option],
        inline: false,
        pubb: false,
    }]
}

pub(crate) fn separated_item_to_lalrpop_items(item: &SeparatedItem) -> Vec<LALRPOPItem> {
    if !is_enabled(&item.enabled) {
        return vec![];
    }
    let capturing_name = format!("_{}", item.reference);
    let fields = vec![LALRPOPField {
        capturing_name: Some(capturing_name.clone().into()),
        rule: separated(
            capture(RustCode(item.reference.clone().to_string())),
            // Don't capture separator
            RustCode(item.separator.clone().to_string()),
            item.allow_empty.unwrap_or(false),
        ),
    }];
    let option = LALRPOPOption {
        fields: fields.clone(),
        constructor: Some(constructor(&item.name)),
    };

    vec![LALRPOPItem {
        name: item.name.clone(),
        producing_type: item.name.clone(),
        options: vec![option],
        inline: false,
        pubb: false,
    }]
}

pub(crate) fn keyword_item_to_lalrpop_items(item: &KeywordItem) -> Vec<LALRPOPItem> {
    let capturing_name = format!("_{}", item.name);
    let keyword_option = |reserved: bool| LALRPOPOption {
        fields: vec![LALRPOPField {
            capturing_name: Some(capturing_name.clone().into()),
            rule: RustCode(format!(
                "{}_{}",
                item.name.clone(),
                if reserved { "Reserved" } else { "Unreserved" }
            )),
        }],
        constructor: None,
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

    vec![LALRPOPItem {
        name: item.name.clone(),
        producing_type: item.name.clone(),
        options,
        inline: true,
        pubb: false,
    }]
}

fn field_to_lalrpop_field(name: &Identifier, field: &Field) -> Option<LALRPOPField> {
    let capturing_name = format!("_{}", name);
    match field {
        Field::Required { reference } => Some(LALRPOPField {
            capturing_name: Some(capturing_name.clone().into()),
            rule: simple_match(&reference),
        }),
        Field::Optional { reference, enabled } if is_enabled(enabled) => Some(LALRPOPField {
            capturing_name: Some(capturing_name.clone().into()),
            rule: optional(simple_match(&reference)),
        }),
        _ => None,
    }
}

pub(crate) fn precedence_item_to_lalrpop_items(item: &PrecedenceItem) -> Vec<LALRPOPItem> {
    if !is_enabled(&item.enabled) {
        return vec![];
    }
    let mut ans = Vec::new();

    let primaries: Vec<LALRPOPOption> = item
        .primary_expressions
        .iter()
        .filter(|exp| is_enabled(&exp.enabled))
        .map(|exp| {
            let capturing_name = format!("_{}", exp.reference);
            LALRPOPOption {
                fields: vec![LALRPOPField {
                    capturing_name: Some(capturing_name.clone().into()),
                    rule: simple_match(&exp.reference),
                }],
                constructor: Some(variant_constructor(&item.name, &exp.reference)),
            }
        })
        .collect();

    let mut prec_counter = 0;

    ans.push(LALRPOPItem {
        name: format!("{}{}", item.name, prec_counter).into(),
        producing_type: item.name.clone(),
        options: primaries,
        inline: false,
        pubb: false,
    });

    for prec in item.precedence_expressions.iter().rev() {
        let prev_name = format!("{}{}", item.name, prec_counter);
        prec_counter += 1;
        let cur_name = format!("{}{}", item.name, prec_counter);

        // Pre-process fields for operators
        let mut pre_processed_fields = if prec.operators.len() == 1 {
            // If there's a single operator, we use its fields directly, there may be many fields
            let op = &prec.operators[0];
            if is_enabled(&op.enabled) {
                op.fields
                    .iter()
                    .filter_map(|(name, field)| field_to_lalrpop_field(&name, &field))
                    .collect()
            } else {
                vec![]
            }
        } else {
            // If there are multiple operators, we create a choice between them
            let operator_ident = Identifier::from(format!("{}_{}_Operator", item.name, prec.name));
            let options = prec
                .operators
                .iter()
                .filter(|op| is_enabled(&op.enabled))
                .map(|op| {
                    assert!(
                        op.fields.len() == 1,
                        "Expected single field operators in multi-operator precedence expressions"
                    );
                    let (identifier, field) = op.fields.first().unwrap();
                    assert_eq!(
                        identifier,
                        &Identifier::from("operator"),
                        "Operator field must be labeled 'operator'"
                    );
                    if let Field::Required { reference } = field {
                        LALRPOPOption {
                            fields: vec![field_to_lalrpop_field(identifier, field).unwrap()],
                            constructor: Some(variant_constructor(&operator_ident, &reference)),
                        }
                    } else {
                        panic!("Operator field must be required");
                    }
                })
                .collect();

            ans.push(LALRPOPItem {
                name: operator_ident.clone(),
                producing_type: operator_ident.clone(),
                options,
                inline: false,
                pubb: false,
            });

            // And the single field now references this choice
            let capturing_name = format!("_{}", operator_ident);
            vec![LALRPOPField {
                capturing_name: Some(capturing_name.clone().into()),
                rule: simple_match(&operator_ident),
            }]
        };

        // Now, we always have a single option, since we're common eliminating some choices
        let option = {
            let capturing_name = format!("_{}", item.name);

            let fields = match prec.operators[0].model {
                OperatorModel::Prefix => {
                    let mut fields = pre_processed_fields;

                    fields.push(LALRPOPField {
                        capturing_name: Some(capturing_name.clone().into()),
                        rule: RustCode(cur_name.clone().to_string()),
                    });
                    fields
                }
                OperatorModel::Postfix => {
                    let mut fields = vec![LALRPOPField {
                        capturing_name: Some(capturing_name.clone().into()),
                        rule: RustCode(cur_name.clone().to_string()),
                    }];

                    fields.extend(pre_processed_fields);

                    fields
                }
                inner_op @ (OperatorModel::BinaryLeftAssociative
                | OperatorModel::BinaryRightAssociative) => {
                    let (left_capture, right_capture) = match inner_op {
                        OperatorModel::BinaryLeftAssociative => (cur_name, prev_name),
                        OperatorModel::BinaryRightAssociative => (prev_name, cur_name),
                        _ => unreachable!(),
                    };

                    vec![
                        LALRPOPField {
                            capturing_name: Some(format!("{capturing_name}").into()),
                            rule: RustCode(left_capture),
                        },
                        pre_processed_fields.remove(0),
                        LALRPOPField {
                            capturing_name: Some(format!("{capturing_name}_2").into()),
                            rule: RustCode(right_capture),
                        },
                    ]
                }
            };

            ans.push(LALRPOPItem {
                name: prec.name.clone(),
                producing_type: prec.name.clone(),
                options: vec![LALRPOPOption {
                    fields: fields,
                    constructor: Some(constructor(&prec.name)),
                }],
                inline: true,
                pubb: false,
            });

            // And we reference it from the single field
            LALRPOPOption {
                fields: vec![LALRPOPField {
                    capturing_name: Some(capturing_name.clone().into()),
                    rule: simple_match(&prec.name),
                }],
                constructor: Some(variant_constructor(&item.name, &prec.name)),
            }
        };

        // Add recursive case
        let options = vec![
            option,
            LALRPOPOption {
                fields: vec![LALRPOPField {
                    capturing_name: Some(item.name.clone()),
                    rule: RustCode(format!("{}{}", item.name, prec_counter - 1)),
                }],
                constructor: None,
            },
        ];

        ans.push(LALRPOPItem {
            name: format!("{}{}", item.name, prec_counter).into(),
            producing_type: item.name.clone(),
            options: options,
            inline: false,
            pubb: false,
        });
    }

    // Adding the entry case
    ans.push(LALRPOPItem {
        name: item.name.clone(),
        producing_type: item.name.clone(),
        options: vec![LALRPOPOption {
            fields: vec![LALRPOPField {
                capturing_name: Some(item.name.clone()),
                rule: RustCode(format!("{}{}", item.name, prec_counter)),
            }],
            constructor: None,
        }],
        inline: false,
        pubb: false,
    });

    ans
}
