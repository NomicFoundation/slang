use language_v2_definition::model::{
    EnumItem, Field, Identifier, KeywordItem, OperatorModel, PrecedenceItem, RepeatedItem,
    SeparatedItem, StructItem, VersionSpecifier,
};
use semver::Version;
use serde::Serialize;

/// Newtype for the already generated Rust code, not to be confused with regular strings.
#[derive(Serialize, Debug, Clone)]
struct RustCode(String);

// TODO(v2): Support multiple versions
pub(crate) const VERSION: Version = Version::new(0, 8, 30);

#[derive(Clone, Debug, Serialize)]
#[serde(tag = "type", content = "content")]
/// An Item can be a manually written rule or a collection of items
pub(crate) enum LALRPOPItem {
    Verbatim(String),
    Items(Vec<LALRPOPItemInner>),
}

#[derive(Clone, Debug, Serialize)]
/// An `LALRPOPItemInner` represents a single LALRPOP rule, for example:
///
/// ```rust
/// ModifierAttribute: ModifierAttribute<'arena> = {
///     <_OverrideSpecifier: OverrideSpecifier>  => new_modifier_attribute_override_specifier(arena, <>),
///     <_VirtualKeyword: VirtualKeyword>  => new_modifier_attribute_virtual_keyword(arena, <>),
/// };
/// ```
pub(crate) struct LALRPOPItemInner {
    name: Identifier,
    producing_type: Identifier,
    options: Vec<LALRPOPOption>,
    pub inline: bool,
    pub pubb: bool,
}

#[derive(Clone, Debug, Serialize)]
/// An `LALRPOPOption` represents a single option within a rule, for example:
///
/// ```rust
///     <_OverrideSpecifier: OverrideSpecifier>  => new_modifier_attribute_override_specifier(arena, <>),
/// ```
struct LALRPOPOption {
    fields: Vec<LALRPOPField>,
    // If none, then the match is forwarded
    constructor: Option<RustCode>,
}

#[derive(Clone, Debug, Serialize)]
/// An `LALRPOPField` represents a single matching field within an option:
///
/// ```rust
///     <_OverrideSpecifier: OverrideSpecifier>
/// ```
struct LALRPOPField {
    capturing_name: Option<Identifier>,
    rule: RustCode,
}

fn is_enabled(enabled: &Option<VersionSpecifier>) -> bool {
    enabled.as_ref().is_none_or(|v| v.contains(&VERSION))
}

// Helper rules for LALRPOP matching rules

/// Given a rule `X`, return the optional matching rule `(X)?`
fn optional(rule: RustCode) -> RustCode {
    RustCode(format!("({})?", rule.0))
}

/// Given a rule `X`, return the, maybe empty, repeating rule `Rep<X>`
fn repeated(rule: RustCode, allow_empty: bool) -> RustCode {
    if allow_empty {
        RustCode(format!("Rep<{}>", rule.0))
    } else {
        RustCode(format!("RepOne<{}>", rule.0))
    }
}

/// Given rules `X` and `Y`, return the, maybe empty, separated rule `Sep<X, Y>`
fn separated(rule: RustCode, separator: RustCode, allow_empty: bool) -> RustCode {
    if allow_empty {
        RustCode(format!("Sep<{}, {}>", separator.0, rule.0))
    } else {
        RustCode(format!("SepOne<{}, {}>", separator.0, rule.0))
    }
}

/// anonymous capture inside LALRPOP macros, this is needed when capturing inside other rules
fn capture(rule: RustCode) -> RustCode {
    RustCode(format!("<{}>", rule.0))
}

/// Match against an identifier
fn simple_match(id: &Identifier) -> RustCode {
    RustCode(format!("{}", *id))
}

/// Get the constructor for `Identifier` `id`
///
/// Note: See the `slang_solidity_v2_cst::structured_cst::nodes` module for details
fn constructor(id: &Identifier) -> RustCode {
    RustCode(format!("new_{}", *id))
}

/// Get the constructor for the variant `variant` from the enum `id`
///
/// Note: See the `slang_solidity_v2_cst::structured_cst::nodes` module for details
fn variant_constructor(id: &Identifier, variant: &Identifier) -> RustCode {
    RustCode(format!("new_{}_{}", *id, *variant))
}

pub(crate) fn struct_item_to_lalrpop_items(item: &StructItem) -> Vec<LALRPOPItemInner> {
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

    vec![LALRPOPItemInner {
        name: item.name.clone(),
        producing_type: item.name.clone(),
        options: vec![option],
        inline: false,
        pubb: false,
    }]
}

pub(crate) fn enum_item_to_lalrpop_items(item: &EnumItem) -> Vec<LALRPOPItemInner> {
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

    vec![LALRPOPItemInner {
        name: item.name.clone(),
        producing_type: item.name.clone(),
        options,
        inline: false,
        pubb: false,
    }]
}

pub(crate) fn repeated_item_to_lalrpop_items(item: &RepeatedItem) -> Vec<LALRPOPItemInner> {
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

    vec![LALRPOPItemInner {
        name: item.name.clone(),
        producing_type: item.name.clone(),
        options: vec![option],
        inline: false,
        pubb: false,
    }]
}

pub(crate) fn separated_item_to_lalrpop_items(item: &SeparatedItem) -> Vec<LALRPOPItemInner> {
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

    vec![LALRPOPItemInner {
        name: item.name.clone(),
        producing_type: item.name.clone(),
        options: vec![option],
        inline: false,
        pubb: false,
    }]
}

// TODO: This is unused, but we may need it
#[allow(dead_code)]
pub(crate) fn keyword_item_to_lalrpop_items(item: &KeywordItem) -> Vec<LALRPOPItemInner> {
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

    vec![LALRPOPItemInner {
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

/// Given a precedence item we produce multiple rules to match it, and a single entry rule
/// with `item.name` as its name.
///
/// Since this is the most complex rule we explain, all other rules are easy compares to it:
///
/// A PrecedenceItem contains a collection of primary expressions (ie the base of the recursion)
/// and a, sorted, collection of precedence expressions.
/// Each precedence expression contains a collection of operators, where each of them has
/// an `OperatorModel` (Prefix, Postfix, BinaryLeftAssociative, BinaryRightAssociative)
/// and some named fields.
///
/// TODO(v2): We're assuming that Precedence Items follow a strict shape, in particular that Binary Operators
/// have a single required field called `operator` and that prefix and postfix operator with mutiple
/// operators follow the same rule.
pub(crate) fn precedence_item_to_lalrpop_items(item: &PrecedenceItem) -> Vec<LALRPOPItemInner> {
    if !is_enabled(&item.enabled) {
        return vec![];
    }
    // The final items
    let mut ans = Vec::new();

    // A counter, for all the rules.
    let mut prec_counter = 0;

    ans.push(collect_primaries(item, prec_counter));

    for prec in item.precedence_expressions.iter().rev() {
        let prev_name = format!("{}{}", item.name, prec_counter);
        prec_counter += 1;
        let cur_name = format!("{}{}", item.name, prec_counter);

        // Pre-process fields for operators
        let mut pre_processed_fields = if prec.operators.len() == 1 {
            // If there's a single operator, we use its fields directly
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
            // If there are multiple operators, we create a choice between them as a new rule, for example:
            // ```rust
            // Expression_PostfixExpression_Operator: Expression_PostfixExpression_Operator<'arena> = {
            //    <_operator: PlusPlus>  => new_expression_postfix_expression_operator_plus_plus(arena, <>),
            //    <_operator: MinusMinus>  => new_expression_postfix_expression_operator_minus_minus(arena, <>),
            //};
            // ```
            // And the pre_processed_fields refer to the new rule alone (ie `Expression_PostfixExpression_Operator`)
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

            ans.push(LALRPOPItemInner {
                name: operator_ident.clone(),
                producing_type: operator_ident.clone(),
                options,
                inline: false,
                pubb: false,
            });

            // And then the single field references this new rule
            let capturing_name = format!("_{}", operator_ident);
            vec![LALRPOPField {
                capturing_name: Some(capturing_name.clone().into()),
                rule: simple_match(&operator_ident),
            }]
        };

        // Now, we always have a single option, since alternatives are tucked away in their own rule
        let option = {
            let capturing_name = format!("_{}", item.name);

            // The fields of this option depend on the operaot model, within a given precedence expression
            // all operators should have the same model, therefore we match against the first one
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

            // At this point we create a rule with the single option and the processed fields,
            // accounting also for associativity
            ans.push(LALRPOPItemInner {
                name: prec.name.clone(),
                producing_type: prec.name.clone(),
                options: vec![LALRPOPOption {
                    fields: fields,
                    constructor: Some(constructor(&prec.name)),
                }],
                inline: true,
                pubb: false,
            });

            // And we reference it from the single option
            LALRPOPOption {
                fields: vec![LALRPOPField {
                    capturing_name: Some(capturing_name.clone().into()),
                    rule: simple_match(&prec.name),
                }],
                constructor: Some(variant_constructor(&item.name, &prec.name)),
            }
        };

        // Add the recursive case to the options
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

        // And append the given item
        ans.push(LALRPOPItemInner {
            name: format!("{}{}", item.name, prec_counter).into(),
            producing_type: item.name.clone(),
            options: options,
            inline: false,
            pubb: false,
        });
    }

    // The entry case points to the latest item
    ans.push(LALRPOPItemInner {
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

/// Given a prec_counter and a PrecedenceItem, create an `LALRPOPItemInner` parsing the primary expressions.
///
/// For each primary expression create an alternative matching against its reference, and constructing
/// an instance of the resulting variant.
///
/// For example:
/// ```rust
/// YulExpression0: YulExpression<'arena> = {
///     <_YulLiteral: YulLiteral>  => new_yul_expression_yul_literal(arena, <>),
///     <_YulPath: YulPath>  => new_yul_expression_yul_path(arena, <>),
/// };
/// ```
fn collect_primaries(item: &PrecedenceItem, prec_counter: i32) -> LALRPOPItemInner {
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

    LALRPOPItemInner {
        name: format!("{}{}", item.name, prec_counter).into(),
        producing_type: item.name.clone(),
        options: primaries,
        inline: false,
        pubb: false,
    }
}
