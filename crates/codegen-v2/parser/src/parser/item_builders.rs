use language_v2_definition::model::{
    EnumItem, Field, Identifier, OperatorModel, PrecedenceItem, RepeatedItem, SeparatedItem,
    StructItem, VersionSpecifier,
};
use semver::Version;
use serde::Serialize;

/// Newtype for the already generated Rust code, not to be confused with regular strings.
#[derive(Serialize, Debug, Clone)]
struct RustCode(String);

// TODO(v2): Support multiple versions
// This is exported as it's currently being checked in various places
// _SLANG_V2_PARSER_VERSION_ (keep in sync)
pub const VERSION: Version = Version::new(0, 8, 30);

/// An `LALRPOPDerivedItem` represents a single LALRPOP rule, for example:
///
/// ```ignore
/// ModifierAttribute: ModifierAttribute = {
///     <_OverrideSpecifier: OverrideSpecifier>  => new_modifier_attribute_override_specifier(<>),
///     <_VirtualKeyword: VirtualKeyword>  => new_modifier_attribute_virtual_keyword(<>),
/// };
/// ```
#[derive(Clone, Debug, Serialize)]
pub(crate) struct LALRPOPDerivedItem {
    name: Identifier,
    producing_type: Identifier,
    options: Vec<LALRPOPDefinition>,
    pub inline: bool,
    pub public: bool,
}

impl LALRPOPDerivedItem {
    fn new(
        name: impl Into<Identifier>,
        producing_type: impl Into<Identifier>,
        options: Vec<LALRPOPDefinition>,
    ) -> Self {
        Self {
            name: name.into(),
            producing_type: producing_type.into(),
            options,
            inline: false,
            public: false,
        }
    }
}

/// An `LALRPOPDefinition` represents a single option within a rule, for example:
///
/// ```ignore
///     <_OverrideSpecifier: OverrideSpecifier>  => new_modifier_attribute_override_specifier(<>),
/// ```
#[derive(Clone, Debug, Serialize)]
struct LALRPOPDefinition {
    fields: Vec<LALRPOPField>,
    // If `None`, then the match is forwarded
    constructor: Option<RustCode>,
}

/// An `LALRPOPField` represents a single matching field within an option:
///
/// ```ignore
///     <_OverrideSpecifier: OverrideSpecifier>
/// ```
#[derive(Clone, Debug, Serialize)]
struct LALRPOPField {
    capturing_name: Identifier,
    rule: RustCode,
}

/// Checks if a given version specifier enables the supported version
fn is_enabled(enabled: Option<&VersionSpecifier>) -> bool {
    enabled
        .unwrap_or(&VersionSpecifier::default())
        .contains(&VERSION)
}

/// Helper rules for LALRPOP matching rules
mod lalrpop_rules {
    use language_v2_definition::model::Identifier;

    use super::RustCode;

    /// Given a rule `X`, return the optional matching rule `(X)?`
    pub(super) fn optional(rule: &RustCode) -> RustCode {
        RustCode(format!("({})?", rule.0))
    }

    /// Given a rule `X`, return the, maybe empty, repeating rule `RepeatedAllowEmpty<X>`
    pub(super) fn repeated(rule: &RustCode, allow_empty: bool) -> RustCode {
        if allow_empty {
            RustCode(format!("RepeatedAllowEmpty<{}>", rule.0))
        } else {
            RustCode(format!("Repeated<{}>", rule.0))
        }
    }

    /// Given rules `X` and `Y`, return the, maybe empty, separated rule `SeparatedAllowEmpty<X, Y>`
    pub(super) fn separated(rule: &RustCode, separator: &RustCode, allow_empty: bool) -> RustCode {
        if allow_empty {
            RustCode(format!("SeparatedAllowEmpty<{}, {}>", separator.0, rule.0))
        } else {
            RustCode(format!("Separated<{}, {}>", separator.0, rule.0))
        }
    }

    /// Anonymously capture a rule `X` into `<X>`
    pub(super) fn capture(rule: &RustCode) -> RustCode {
        RustCode(format!("<{}>", rule.0))
    }

    /// Match against an identifier
    pub(super) fn simple_match(id: &Identifier) -> RustCode {
        RustCode(format!("{}", *id))
    }
}

/// Helper methods to get constructors for nodes
///
/// Note: See the `slang_solidity_v2_cst::structured_cst::nodes` module for details
mod node_constructors {
    use language_v2_definition::model::Identifier;

    use super::RustCode;
    /// Get the constructor for `Identifier` `id`
    pub(super) fn constructor(id: &Identifier) -> RustCode {
        RustCode(format!("new_{}", *id))
    }

    /// Get the constructor for the variant `variant` from the enum `id`
    pub(super) fn variant_constructor(id: &Identifier, variant: &Identifier) -> RustCode {
        RustCode(format!("new_{}_{}", *id, *variant))
    }
}

/// A struct item is converted into a single LALRPOP item with a single option with one or more fields.
///
/// ```ignore
/// ImportDirective: ImportDirective = {
///     <_import_keyword: ImportKeyword>  <_clause: ImportClause>  <_semicolon: Semicolon>  => new_import_directive(<>),
/// };
///```
pub(crate) fn struct_item_to_lalrpop_items(item: &StructItem) -> Vec<LALRPOPDerivedItem> {
    use node_constructors::constructor;

    if !is_enabled(item.enabled.as_ref()) {
        return vec![];
    }
    let fields = item
        .fields
        .iter()
        .filter_map(|(name, field)| field_to_lalrpop_field(name, field));

    let option = LALRPOPDefinition {
        fields: fields.collect(),
        constructor: Some(constructor(&item.name)),
    };

    vec![LALRPOPDerivedItem::new(
        item.name.clone(),
        item.name.clone(),
        vec![option],
    )]
}

/// An enum item is converted into a single LALRPOP item with multiple options, one per variant.
/// Each option has a single field matching the variant reference.
///
/// ```ignore
/// ImportClause: ImportClause = {
///     <_PathImport: PathImport>  => new_import_clause_path_import(<>),
///     <_NamedImport: NamedImport>  => new_import_clause_named_import(<>),
///     <_ImportDeconstruction: ImportDeconstruction>  => new_import_clause_import_deconstruction(<>),
/// };
/// ```
pub(crate) fn enum_item_to_lalrpop_items(item: &EnumItem) -> Vec<LALRPOPDerivedItem> {
    use lalrpop_rules::simple_match;
    use node_constructors::variant_constructor;

    if !is_enabled(item.enabled.as_ref()) {
        return vec![];
    }
    let options = item
        .variants
        .iter()
        .filter(|variant| is_enabled(variant.enabled.as_ref()))
        .map(|variant| {
            let capturing_name = format!("{}", variant.reference);
            let fields = vec![LALRPOPField {
                capturing_name: capturing_name.clone().into(),
                rule: simple_match(&variant.reference),
            }];
            LALRPOPDefinition {
                fields: fields.clone(),
                constructor: Some(variant_constructor(&item.name, &variant.reference)),
            }
        })
        .collect();

    vec![LALRPOPDerivedItem::new(
        item.name.clone(),
        item.name.clone(),
        options,
    )]
}

/// A repeated item is converted into a single LALRPOP item with a single option with a single field, that is
/// either `RepeatedAllowEmpty<...>` or `Repeated<...>` based on the `allow_empty` flag.
///
/// ```ignore
/// ContractMembers: ContractMembers = {
///     <_ContractMember: RepeatedAllowEmpty<<ContractMember>>>  => new_contract_members(<>),
/// };
/// ```
pub(crate) fn repeated_item_to_lalrpop_items(item: &RepeatedItem) -> Vec<LALRPOPDerivedItem> {
    use lalrpop_rules::{capture, repeated};
    use node_constructors::constructor;

    if !is_enabled(item.enabled.as_ref()) {
        return vec![];
    }
    let capturing_name = format!("{}", item.reference);
    let fields = vec![LALRPOPField {
        capturing_name: capturing_name.clone().into(),
        rule: repeated(
            &capture(&RustCode(item.reference.clone().to_string())),
            item.allow_empty.unwrap_or(false),
        ),
    }];
    let option = LALRPOPDefinition {
        fields: fields.clone(),
        constructor: Some(constructor(&item.name)),
    };

    vec![LALRPOPDerivedItem::new(
        item.name.clone(),
        item.name.clone(),
        vec![option],
    )]
}

/// A separated item is converted into a single LALRPOP item with a single option with a single field, that is
/// `SeparatedAllowEmpty<...>` or `Separated<...>` based on the `allow_empty` flag.
///
/// ```ignore
/// ImportDeconstructionSymbols: ImportDeconstructionSymbols = {
///    <_ImportDeconstructionSymbol: Separated<Comma, <ImportDeconstructionSymbol>>>  => new_import_deconstruction_symbols(<>),
/// };
/// ```
pub(crate) fn separated_item_to_lalrpop_items(item: &SeparatedItem) -> Vec<LALRPOPDerivedItem> {
    use lalrpop_rules::{capture, separated};
    use node_constructors::constructor;

    if !is_enabled(item.enabled.as_ref()) {
        return vec![];
    }
    let capturing_name = format!("{}", item.reference);
    let fields = vec![LALRPOPField {
        capturing_name: capturing_name.clone().into(),
        rule: separated(
            &capture(&RustCode(item.reference.clone().to_string())),
            // Don't capture separator
            &RustCode(item.separator.clone().to_string()),
            item.allow_empty.unwrap_or(false),
        ),
    }];
    let option = LALRPOPDefinition {
        fields: fields.clone(),
        constructor: Some(constructor(&item.name)),
    };

    vec![LALRPOPDerivedItem::new(
        item.name.clone(),
        item.name.clone(),
        vec![option],
    )]
}

/// Given a precedence item we produce multiple rules to match it, and a single entry rule
/// with `item.name` as its name.
///
/// Since this is the most complex rule we explain it in detail, all other rules are easy compared to it:
///
/// A `PrecedenceItem` contains a collection of primary expressions (ie the base of the recursion)
/// and a, sorted, collection of precedence expressions.
/// Each precedence expression contains a collection of operators, where each of them has
/// an `OperatorModel` (Prefix, Postfix, `BinaryLeftAssociative`, `BinaryRightAssociative`)
/// and some named fields.
/// In the case multiple operators are defined within a precedence expression, we wrap them in their own rule,
/// basically converting this into syntactic sugar for using an enum item.
///
/// TODO(v2): We're assuming that Precedence Items follow a strict shape, in particular that Binary Operators
/// have a single required field called `operator` and that prefix and postfix operator with mutiple
/// operators follow the same rule.
/// Ideally we should simplify the model to enforce this constraint, or be more relaxed in the code here.
pub(crate) fn precedence_item_to_lalrpop_items(item: &PrecedenceItem) -> Vec<LALRPOPDerivedItem> {
    use lalrpop_rules::simple_match;
    use node_constructors::{constructor, variant_constructor};

    if !is_enabled(item.enabled.as_ref()) {
        return vec![];
    }
    // The final items
    let mut ans = Vec::new();

    // A counter, tracking the precedence levels
    let mut prec_counter = 0;

    // Collect all the primary expressions
    ans.push(collect_primaries(item, prec_counter));

    for prec in item.precedence_expressions.iter().rev() {
        let prev_name = format!("{}{}", item.name, prec_counter);
        prec_counter += 1;
        let cur_name = format!("{}{}", item.name, prec_counter);

        // Pre-process fields for operators
        let pre_processed_fields = if prec.operators.len() == 1 {
            // If there's a single operator, we use its fields directly
            let op = &prec.operators[0];
            if is_enabled(op.enabled.as_ref()) {
                op.fields
                    .iter()
                    .filter_map(|(name, field)| field_to_lalrpop_field(name, field))
                    .collect()
            } else {
                vec![]
            }
        } else {
            // If there are multiple operators, we create a choice between them as a new rule.
            // And the pre_processed_fields refer to the new rule alone (ie `Expression_PostfixExpression_Operator`)
            //
            // This better structures the grammar, following the implicit rule where every action (to the right of `=>`)
            // is a single constructor call
            let x = precedence_operator_to_lalrpop_item(item, prec);
            let operator_ident = &x.name;

            // And then the single field references this new rule
            let capturing_name = format!("{operator_ident}");
            let fields = vec![LALRPOPField {
                capturing_name: capturing_name.clone().into(),
                rule: simple_match(operator_ident),
            }];
            ans.push(x);
            fields
        };

        // Now, we always have a single option, since alternatives are tucked away in their own rule
        let option = {
            let capturing_name = format!("{}", item.name);

            // The fields of this option depend on the operaot model, within a given precedence expression
            // all operators should have the same model, therefore we match against the first one
            let fields = operator_model_to_fields(
                prec.operators[0].model,
                pre_processed_fields,
                &capturing_name,
                cur_name,
                prev_name,
            );

            // At this point we create a rule with the single option and the processed fields,
            // accounting also for associativity
            ans.push(LALRPOPDerivedItem {
                name: prec.name.clone(),
                producing_type: prec.name.clone(),
                options: vec![LALRPOPDefinition {
                    fields,
                    constructor: Some(constructor(&prec.name)),
                }],
                inline: true,
                public: false,
            });

            // And we reference it from the single option
            LALRPOPDefinition {
                fields: vec![LALRPOPField {
                    capturing_name: capturing_name.clone().into(),
                    rule: simple_match(&prec.name),
                }],
                constructor: Some(variant_constructor(&item.name, &prec.name)),
            }
        };

        // Add the recursive case to the options
        let options = vec![
            option,
            LALRPOPDefinition {
                fields: vec![LALRPOPField {
                    capturing_name: item.name.clone(),
                    rule: RustCode(format!("{}{}", item.name, prec_counter - 1)),
                }],
                constructor: None,
            },
        ];

        // And append the given item
        ans.push(LALRPOPDerivedItem::new(
            format!("{}{}", item.name, prec_counter),
            item.name.clone(),
            options,
        ));
    }

    // The entry case points to the latest item
    ans.push(LALRPOPDerivedItem::new(
        item.name.clone(),
        item.name.clone(),
        vec![LALRPOPDefinition {
            fields: vec![LALRPOPField {
                capturing_name: item.name.clone(),
                rule: RustCode(format!("{}{}", item.name, prec_counter)),
            }],
            constructor: None,
        }],
    ));

    ans
}

/// Given an operator model, process the fields accordingly
fn operator_model_to_fields(
    operator_model: OperatorModel,
    mut pre_processed_fields: Vec<LALRPOPField>,
    capturing_name: &String,
    cur_name: String,
    prev_name: String,
) -> Vec<LALRPOPField> {
    match operator_model {
        OperatorModel::Prefix => {
            let mut fields = pre_processed_fields;

            fields.push(LALRPOPField {
                capturing_name: capturing_name.clone().into(),
                rule: RustCode(cur_name.clone().to_string()),
            });
            fields
        }
        OperatorModel::Postfix => {
            let mut fields = vec![LALRPOPField {
                capturing_name: capturing_name.clone().into(),
                rule: RustCode(cur_name.clone().to_string()),
            }];

            fields.extend(pre_processed_fields);

            fields
        }
        inner_op @ (OperatorModel::BinaryLeftAssociative
        | OperatorModel::BinaryRightAssociative) => {
            // The associativity determines the order of captures
            let (left_capture, right_capture) = match inner_op {
                OperatorModel::BinaryLeftAssociative => (cur_name, prev_name),
                OperatorModel::BinaryRightAssociative => (prev_name, cur_name),
                _ => unreachable!(),
            };

            vec![
                LALRPOPField {
                    capturing_name: capturing_name.to_string().into(),
                    rule: RustCode(left_capture),
                },
                // We're assuming a single field is used here
                pre_processed_fields.remove(0),
                LALRPOPField {
                    capturing_name: format!("{capturing_name}_2").into(),
                    rule: RustCode(right_capture),
                },
            ]
        }
    }
}

/// Given a `PrecedenceItem` and a `PrecedenceExpression`, create an `LALRPOPDerivedItem` parsing
/// the operators within the precedence expression.
///
/// For example:
/// ```ignore
/// Expression_PostfixExpression_Operator: Expression_PostfixExpression_Operator = {
///    <_operator: PlusPlus>  => new_expression_postfix_expression_operator_plus_plus(<>),
///    <_operator: MinusMinus>  => new_expression_postfix_expression_operator_minus_minus(<>),
///};
/// ```
fn precedence_operator_to_lalrpop_item(
    item: &PrecedenceItem,
    prec: &std::rc::Rc<language_v2_definition::model::PrecedenceExpression>,
) -> LALRPOPDerivedItem {
    use node_constructors::variant_constructor;

    let operator_ident = Identifier::from(format!("{}_{}_Operator", item.name, prec.name));
    let options = prec
        .operators
        .iter()
        .filter(|op| is_enabled(op.enabled.as_ref()))
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
                LALRPOPDefinition {
                    fields: vec![field_to_lalrpop_field(identifier, field).unwrap()],
                    constructor: Some(variant_constructor(&operator_ident, reference)),
                }
            } else {
                panic!("Operator field must be required");
            }
        })
        .collect();

    LALRPOPDerivedItem::new(operator_ident.clone(), operator_ident.clone(), options)
}

/// Given a `prec_counter` and a `PrecedenceItem`, create an `LALRPOPDerivedItem` parsing the primary expressions.
///
/// For each primary expression create an alternative matching against its reference, and constructing
/// an instance of the resulting variant.
///
/// For example:
/// ```ignore
/// YulExpression0: YulExpression = {
///     <_YulLiteral: YulLiteral>  => new_yul_expression_yul_literal(<>),
///     <_YulPath: YulPath>  => new_yul_expression_yul_path(<>),
/// };
/// ```
fn collect_primaries(item: &PrecedenceItem, prec_counter: i32) -> LALRPOPDerivedItem {
    use lalrpop_rules::simple_match;
    use node_constructors::variant_constructor;

    let primaries: Vec<LALRPOPDefinition> = item
        .primary_expressions
        .iter()
        .filter(|exp| is_enabled(exp.enabled.as_ref()))
        .map(|exp| {
            let capturing_name = format!("{}", exp.reference);
            LALRPOPDefinition {
                fields: vec![LALRPOPField {
                    capturing_name: capturing_name.into(),
                    rule: simple_match(&exp.reference),
                }],
                constructor: Some(variant_constructor(&item.name, &exp.reference)),
            }
        })
        .collect();

    LALRPOPDerivedItem::new(
        format!("{}{}", item.name, prec_counter),
        item.name.clone(),
        primaries,
    )
}

/// Given a field, produce an `LALRPOPField` if the field is enabled
fn field_to_lalrpop_field(name: &Identifier, field: &Field) -> Option<LALRPOPField> {
    use lalrpop_rules::{optional, simple_match};

    let capturing_name = format!("_{name}");
    match field {
        Field::Required { reference } => Some(LALRPOPField {
            capturing_name: capturing_name.clone().into(),
            rule: simple_match(reference),
        }),
        Field::Optional { reference, enabled } if is_enabled(enabled.as_ref()) => {
            Some(LALRPOPField {
                capturing_name: capturing_name.clone().into(),
                rule: optional(&simple_match(reference)),
            })
        }
        Field::Optional { .. } => None,
    }
}
