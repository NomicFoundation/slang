use itertools::Itertools;
use language_v2_definition::model::{
    EnumItem, Field, Identifier, Item as LanguageItem, KeywordItem, PrecedenceItem, RepeatedItem,
    SeparatedItem, StructItem,
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
    // attributes?
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

// /// An LALRPOP rule represents a single capturing rule in the grammar.
// ///
// /// For example `(<T> ",")*` would be represented by
// /// `ZeroOrMore(Sequence(vec![CapturingReference("T", true), Reference(",", false)]))`.
// #[derive(Clone, Debug, Serialize)]
// enum LALRPOPRule {
//     Reference(Identifier),
//     CapturingReference(Identifier),
//     Optional(Box<LALRPOPRule>),
//     OneOrMore(Box<LALRPOPRule>),
//     ZeroOrMore(Box<LALRPOPRule>),
//     Sequence(Vec<LALRPOPRule>),
// }

// impl From<&LALRPOPRule> for RustCode {
//     fn from(rule: &LALRPOPRule) -> Self {
//         match rule {
//             LALRPOPRule::Reference(identifier) => RustCode(identifier.to_string()),
//             LALRPOPRule::CapturingReference(identifier) => todo!(),
//             LALRPOPRule::Optional(lalrpoprule) => RustCode(format!(
//                 "({})?",
//                 Into::<RustCode>::into(lalrpoprule.as_ref()).0
//             )),
//             LALRPOPRule::OneOrMore(lalrpoprule) => todo!(),
//             LALRPOPRule::ZeroOrMore(lalrpoprule) => todo!(),
//             LALRPOPRule::Sequence(lalrpoprules) => todo!(),
//         }
//     }
// }

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
    if !allow_empty {
        optional(at_least_one)
    } else {
        at_least_one
    }
}

impl From<&StructItem> for LALRPOPItem {
    fn from(item: &StructItem) -> Self {
        let fields = item.fields.iter().filter_map(|(name, field)| {
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
        });

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
        };

        LALRPOPItem {
            name: item.name.clone(),
            producing_type: PRODUCING_TYPE.into(),
            options: vec![option],
        }
    }
}

impl From<&EnumItem> for LALRPOPItem {
    fn from(item: &EnumItem) -> Self {
        let options = item
            .variants
            .iter()
            .map(|variant| {
                let capturing_name = format!("_{}", variant.reference);
                LALRPOPOption {
                    fields: vec![LALRPOPField {
                        capturing_name: Some(capturing_name.clone().into()),
                        rule: RustCode(variant.reference.clone().to_string()),
                    }],
                    action: RustCode(format!("format!(\"Variant[{{:?}}]\", {capturing_name})")),
                }
            })
            .collect();

        LALRPOPItem {
            name: item.name.clone(),
            producing_type: PRODUCING_TYPE.into(),
            options,
        }
    }
}

impl From<&RepeatedItem> for LALRPOPItem {
    fn from(item: &RepeatedItem) -> Self {
        let capturing_name = format!("_{}", item.reference);

        let option = LALRPOPOption {
            fields: vec![LALRPOPField {
                capturing_name: Some(capturing_name.clone().into()),
                rule: repeated(
                    RustCode(item.reference.clone().to_string()),
                    item.allow_empty.unwrap_or(false),
                ),
            }],
            action: RustCode(format!("\"Repeated[{capturing_name}]\".to_string()")),
        };

        LALRPOPItem {
            name: item.name.clone(),
            producing_type: PRODUCING_TYPE.into(),
            options: vec![option],
        }
    }
}

impl From<&SeparatedItem> for LALRPOPItem {
    fn from(item: &SeparatedItem) -> Self {
        let capturing_name = format!("_{}", item.reference);
        let option = LALRPOPOption {
            fields: vec![LALRPOPField {
                capturing_name: Some(capturing_name.clone().into()),

                rule: separated(
                    // TODO: Capture internally
                    RustCode(item.reference.clone().to_string()),
                    RustCode(item.separator.clone().to_string()),
                    item.allow_empty.unwrap_or(false),
                ),
            }],
            action: RustCode(format!("\"Separated[{capturing_name}]\".to_string()")),
        };

        LALRPOPItem {
            name: item.name.clone(),
            producing_type: PRODUCING_TYPE.into(),
            options: vec![option],
        }
    }
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

impl From<&PrecedenceItem> for LALRPOPItem {
    fn from(item: &PrecedenceItem) -> Self {
        let primaries = item.primary_expressions.iter().map(|exp| {
            let capturing_name = format!("_{}", exp.reference);
            LALRPOPOption {
                fields: vec![LALRPOPField {
                    capturing_name: Some(capturing_name.clone().into()),
                    rule: RustCode(exp.reference.clone().to_string()),
                }],
                action: RustCode(format!("format!(\"Precedence[{{:?}}]\", {capturing_name})")),
            }
        });
        LALRPOPItem {
            name: item.name.clone(),
            producing_type: PRODUCING_TYPE.into(),
            options: primaries.collect(),
        }
    }
}

impl TryFrom<&LanguageItem> for LALRPOPItem {
    // TODO: add error recovery
    type Error = ();

    fn try_from(item: &LanguageItem) -> Result<Self, Self::Error> {
        // TODO: we're ignoring versions for now

        // TODO: use an actual type rather than string

        match item {
            LanguageItem::Struct { item } => Ok(item.as_ref().into()),
            LanguageItem::Enum { item } => Ok(item.as_ref().into()),
            LanguageItem::Repeated { item } => Ok(item.as_ref().into()),
            LanguageItem::Separated { item } => Ok(item.as_ref().into()),
            // TODO: No idea how to do this yet
            LanguageItem::Precedence { item } => Ok(item.as_ref().into()),
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
