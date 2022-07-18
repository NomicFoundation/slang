use indexmap::IndexMap;
use semver::Version;
use serde::{
    de::{self, MapAccess, Visitor},
    ser::SerializeMap,
    Deserialize, Serialize, Serializer,
};
use serde_yaml::Value;
use std::{
    cell::RefCell,
    collections::BTreeMap,
    fmt,
    path::PathBuf,
    rc::{Rc, Weak},
};

use crate::chumsky::combinator_tree::CombinatorTreeRoot;

#[derive(Clone, Debug)]
pub struct Grammar {
    pub manifest: Manifest,
    pub productions: IndexMap<String, Vec<ProductionRef>>,
}

impl Grammar {
    pub fn get_production(&self, name: &str) -> ProductionRef {
        // We can do this because the grammar has been validated
        for (_, v) in &self.productions {
            if let Some(production) = v.iter().find(|p| p.name == name) {
                return production.clone();
            }
        }
        panic!(
            "Cannot find {} production, should have been caught in validation pass",
            name
        )
    }
}

#[derive(Deserialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct Manifest {
    pub title: String,
    #[serde(rename = "rootProduction")]
    pub root_production: String,
    pub sections: Vec<Section>,
    pub versions: Option<Vec<String>>,
}

#[derive(Deserialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct Section {
    pub title: String,
    pub topics: Vec<Topic>,
}

#[derive(Deserialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct Topic {
    pub title: String,
    pub notes: Option<String>,
    pub definition: Option<String>,
}

#[derive(Clone, Debug)]
pub struct Production {
    pub name: String,
    pub kind: Option<ProductionKind>,
    pub title: Option<String>,
    pub versions: BTreeMap<Version, ExpressionRef>,
    pub combinator_tree: RefCell<CombinatorTreeRoot>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum ProductionKind {
    Rule,
    Token,
    ExpressionRule,
}

pub type ProductionRef = Rc<Production>;
pub type ProductionWeakRef = Weak<Production>;

impl Production {
    pub fn is_token(&self) -> bool {
        self.kind == Some(ProductionKind::Token)
    }

    fn serialize_in_map<S: Serializer>(&self, state: &mut S::SerializeMap) -> Result<(), S::Error> {
        state.serialize_entry("name", &self.name)?;
        if let Some(kind) = self.kind {
            state.serialize_entry("kind", &kind)?;
        }
        if let Some(title) = &self.title {
            state.serialize_entry("title", title)?;
        }
        // TODO: once `if let` can be combined with boolean guards, merge this
        if self.versions.len() == 1 {
            if let Some(entry) = self.versions.get(&Version::parse("0.0.0").unwrap()) {
                entry.serialize_in_map::<S>(state)?;
            } else {
                state.serialize_entry("versions", &self.versions)?;
            }
        } else {
            state.serialize_entry("versions", &self.versions)?;
        }
        Ok(())
    }
}

impl Serialize for Production {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_map(None)?;
        self.serialize_in_map::<S>(&mut state)?;
        state.end()
    }
}

impl<'de> Deserialize<'de> for Production {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Must be isomorphic with the const FIELDS below
        #[derive(Deserialize)]
        #[serde(field_identifier, rename_all = "camelCase")]
        enum Field {
            // Metadata
            Config,
            Kind,
            Name,
            Title,
            Versions,
            // EBNF
            Choice,
            DelimitedBy,
            Difference,
            End,
            Not,
            OneOrMore,
            Optional,
            Range,
            Reference,
            Repeat,
            Sequence,
            Terminal,
            ZeroOrMore,
        }

        struct ProductionVisitor;

        impl<'de> Visitor<'de> for ProductionVisitor {
            type Value = Production;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct Production")
            }

            fn visit_map<V>(self, mut map: V) -> Result<Production, V::Error>
            where
                V: MapAccess<'de>,
            {
                let mut name = None;
                let mut title = None;
                let mut versions = None;
                let mut config = None;
                let mut ebnf = None;
                let mut kind = None;

                while let Some(key) = map.next_key()? {
                    match key {
                        Field::Name => {
                            if name.is_some() {
                                return Err(de::Error::duplicate_field("name"));
                            }
                        }
                        Field::Kind => {
                            if kind.is_some() {
                                return Err(de::Error::duplicate_field("kind"));
                            }
                        }
                        Field::Title => {
                            if title.is_some() {
                                return Err(de::Error::duplicate_field("title"));
                            }
                        }
                        Field::Versions => {
                            if versions.is_some() {
                                return Err(de::Error::duplicate_field("versions"));
                            }
                            if config.is_some() {
                                return Err(de::Error::duplicate_field("config excludes versions"));
                            }
                            if ebnf.is_some() {
                                return Err(de::Error::duplicate_field(
                                    "ebnf element excludes versions",
                                ));
                            }
                        }
                        Field::Config => {
                            if config.is_some() {
                                return Err(de::Error::duplicate_field("config"));
                            }
                            if versions.is_some() {
                                return Err(de::Error::duplicate_field("versions excludes config"));
                            }
                        }

                        Field::End
                        | Field::ZeroOrMore
                        | Field::OneOrMore
                        | Field::Repeat
                        | Field::Optional
                        | Field::Not
                        | Field::Choice
                        | Field::Sequence
                        | Field::Terminal
                        | Field::Reference
                        | Field::Difference
                        | Field::DelimitedBy
                        | Field::Range => {
                            if ebnf.is_some() {
                                return Err(de::Error::duplicate_field("ebnf element"));
                            }
                            if versions.is_some() {
                                return Err(de::Error::duplicate_field(
                                    "versions excludes ebnf element",
                                ));
                            }
                        }
                    }

                    match key {
                        Field::Name => name = Some(map.next_value()?),
                        Field::Kind => kind = Some(map.next_value()?),
                        Field::Title => title = Some(map.next_value()?),
                        Field::Versions => versions = Some(map.next_value()?),
                        Field::Config => config = Some(map.next_value()?),
                        Field::End => {
                            ebnf = Some(EBNF::End);
                            map.next_value()?
                        }
                        Field::ZeroOrMore | Field::OneOrMore | Field::Optional => {
                            #[derive(Deserialize)]
                            #[serde(deny_unknown_fields)]
                            struct V {
                                #[serde(flatten)]
                                expr: ExpressionRef,
                                #[serde(default)]
                                separator: Option<ExpressionRef>,
                            }
                            let v: V = map.next_value()?;
                            ebnf = Some(EBNF::Repeat(EBNFRepeat {
                                min: if let Field::OneOrMore = key { 1 } else { 0 },
                                max: if let Field::Optional = key {
                                    Some(1)
                                } else {
                                    None
                                },
                                expr: v.expr,
                                separator: v.separator,
                            }))
                        }
                        Field::Repeat => ebnf = Some(EBNF::Repeat(map.next_value()?)),
                        Field::Not => ebnf = Some(EBNF::Not(map.next_value()?)),
                        Field::Choice => ebnf = Some(EBNF::Choice(map.next_value()?)),
                        Field::Sequence => ebnf = Some(EBNF::Sequence(map.next_value()?)),
                        Field::Terminal => ebnf = Some(EBNF::Terminal(map.next_value()?)),
                        Field::Reference => ebnf = Some(EBNF::Reference(map.next_value()?)),
                        Field::DelimitedBy => ebnf = Some(EBNF::DelimitedBy(map.next_value()?)),
                        Field::Difference => ebnf = Some(EBNF::Difference(map.next_value()?)),
                        Field::Range => ebnf = Some(EBNF::Range(map.next_value()?)),
                    }
                }

                if name.is_none() {
                    return Err(de::Error::missing_field("name"));
                }
                let name = name.unwrap();

                if let Some(ebnf) = ebnf {
                    let config = config.unwrap_or_default();
                    versions = Some(BTreeMap::from([(
                        Version::parse("0.0.0").unwrap(),
                        Rc::new(Expression { config, ebnf }),
                    )]))
                }

                if versions.is_none() {
                    return Err(de::Error::missing_field("ebnf expression or versions"));
                }
                let versions = versions.unwrap();

                Ok(Production {
                    name,
                    kind,
                    title,
                    versions,
                    combinator_tree: Default::default(),
                })
            }
        }

        // Must be isomorphic with the enum Field above
        const FIELDS: &'static [&'static str] = &[
            // Metadata
            "config",
            "kind",
            "name",
            "title",
            "versions",
            // EBNF
            "choice",
            "delimitedBy",
            "difference",
            "end",
            "not",
            "oneOrMore",
            "optional",
            "range",
            "reference",
            "repeat",
            "sequence",
            "terminal",
            "zeroOrMore",
        ];
        deserializer.deserialize_struct("Production", FIELDS, ProductionVisitor)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Expression {
    pub config: ExpressionConfig,
    pub ebnf: EBNF,
}

impl Expression {
    pub fn precedence(&self) -> u8 {
        match self.ebnf {
            EBNF::End
            | EBNF::Repeat(..)
            | EBNF::Terminal(..)
            | EBNF::Reference(..)
            | EBNF::Range { .. } => 0,
            EBNF::Not(..) => 1,
            EBNF::Difference { .. } => 2,
            EBNF::DelimitedBy(..) | EBNF::Sequence(..) => 3,
            EBNF::Choice(..) => 4,
        }
    }

    fn serialize_in_map<S: Serializer>(&self, state: &mut S::SerializeMap) -> Result<(), S::Error> {
        match &self.ebnf {
            // Ugly - serde has no way of emitting a map entry with
            // no value, which is valid in YAML. So this ends up as `end: ~`
            EBNF::End => state.serialize_entry("end", &Value::Null),

            EBNF::Repeat(EBNFRepeat {
                min: min @ 0,
                max: max @ None,
                expr,
                separator,
            })
            | EBNF::Repeat(EBNFRepeat {
                min: min @ 0,
                max: max @ Some(1),
                expr,
                separator,
            })
            | EBNF::Repeat(EBNFRepeat {
                min: min @ 1,
                max: max @ None,
                expr,
                separator,
            }) => {
                #[derive(Serialize)]
                struct V<'a> {
                    #[serde(flatten)]
                    expr: &'a ExpressionRef,
                    #[serde(skip_serializing_if = "Option::is_none")]
                    separator: &'a Option<ExpressionRef>,
                }
                let v = V { expr, separator };
                state.serialize_entry(
                    if *min == 1 {
                        "oneOrMore"
                    } else if let Some(1) = max {
                        "optional"
                    } else {
                        "zeroOrMore"
                    },
                    &v,
                )
            }
            EBNF::Repeat(repeat) => state.serialize_entry("repeat", &repeat),
            EBNF::Not(expr) => state.serialize_entry("not", &expr),
            EBNF::Choice(exprs) => state.serialize_entry("choice", &exprs),
            EBNF::Sequence(exprs) => state.serialize_entry("sequence", &exprs),
            EBNF::Terminal(string) => state.serialize_entry("terminal", &string),
            EBNF::Reference(name) => state.serialize_entry("reference", &name),
            EBNF::DelimitedBy(delimited_by) => state.serialize_entry("delimitedBy", &delimited_by),
            EBNF::Difference(difference) => state.serialize_entry("difference", &difference),
            EBNF::Range(range) => state.serialize_entry("range", &range),
        }?;
        if !self.config.is_default() {
            state.serialize_entry("config", &self.config)?;
        };
        Ok(())
    }
}

impl Serialize for Expression {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_map(None)?;
        self.serialize_in_map::<S>(&mut state)?;
        state.end()
    }
}

impl<'de> Deserialize<'de> for Expression {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(Deserialize, Debug)]
        #[serde(field_identifier, rename_all = "camelCase")]
        enum Field {
            Choice,
            Config,
            DelimitedBy,
            Difference,
            End,
            Not,
            OneOrMore,
            Optional,
            Range,
            Reference,
            Repeat,
            Sequence,
            Terminal,
            ZeroOrMore,
        }

        struct ExpressionVisitor;

        impl<'de> Visitor<'de> for ExpressionVisitor {
            type Value = Expression;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct Expression")
            }

            fn visit_map<V>(self, mut map: V) -> Result<Expression, V::Error>
            where
                V: MapAccess<'de>,
            {
                let mut config = None;
                let mut ebnf = None;
                while let Some(key) = map.next_key()? {
                    match key {
                        Field::Config => {
                            if config.is_some() {
                                return Err(de::Error::duplicate_field("config"));
                            }
                        }
                        Field::End
                        | Field::ZeroOrMore
                        | Field::OneOrMore
                        | Field::Repeat
                        | Field::Optional
                        | Field::Not
                        | Field::Choice
                        | Field::Sequence
                        | Field::Terminal
                        | Field::Reference
                        | Field::Difference
                        | Field::DelimitedBy
                        | Field::Range => {
                            if ebnf.is_some() {
                                return Err(de::Error::duplicate_field("ebnf element"));
                            }
                        }
                    }
                    match key {
                        Field::Config => config = Some(map.next_value()?),
                        Field::End => {
                            ebnf = Some(EBNF::End);
                            map.next_value()?
                        }
                        Field::ZeroOrMore | Field::OneOrMore | Field::Optional => {
                            #[derive(Deserialize)]
                            #[serde(deny_unknown_fields)]
                            struct V {
                                #[serde(flatten)]
                                expr: ExpressionRef,
                                #[serde(default)]
                                separator: Option<ExpressionRef>,
                            }
                            let v: V = map.next_value()?;
                            ebnf = Some(EBNF::Repeat(EBNFRepeat {
                                min: if let Field::OneOrMore = key { 1 } else { 0 },
                                max: if let Field::Optional = key {
                                    Some(1)
                                } else {
                                    None
                                },
                                expr: v.expr,
                                separator: v.separator,
                            }))
                        }
                        Field::Repeat => ebnf = Some(EBNF::Repeat(map.next_value()?)),
                        Field::Not => ebnf = Some(EBNF::Not(map.next_value()?)),
                        Field::Choice => ebnf = Some(EBNF::Choice(map.next_value()?)),
                        Field::Sequence => ebnf = Some(EBNF::Sequence(map.next_value()?)),
                        Field::Terminal => ebnf = Some(EBNF::Terminal(map.next_value()?)),
                        Field::Reference => ebnf = Some(EBNF::Reference(map.next_value()?)),
                        Field::DelimitedBy => ebnf = Some(EBNF::DelimitedBy(map.next_value()?)),
                        Field::Difference => ebnf = Some(EBNF::Difference(map.next_value()?)),
                        Field::Range => ebnf = Some(EBNF::Range(map.next_value()?)),
                    }
                }
                let config = config.unwrap_or_default();
                let ebnf = ebnf.ok_or_else(|| de::Error::missing_field("ebnf element"))?;
                Ok(Expression { config, ebnf })
            }
        }

        const FIELDS: &'static [&'static str] = &[
            "choice",
            "config",
            "delimitedBy",
            "difference",
            "end",
            "not",
            "oneOrMore",
            "optional",
            "range",
            "reference",
            "repeat",
            "sequence",
            "terminal",
            "zeroOrMore",
        ];
        deserializer.deserialize_struct("Expression", FIELDS, ExpressionVisitor)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Hash)]
#[serde(deny_unknown_fields)]
pub struct EBNFDifference {
    pub minuend: ExpressionRef,
    pub subtrahend: ExpressionRef,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Hash)]
#[serde(deny_unknown_fields)]
pub struct EBNFRange {
    pub from: char,
    pub to: char,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Hash)]
#[serde(deny_unknown_fields)]
pub struct EBNFRepeat {
    #[serde(default)] // TODO: skip_serializing_if is_zero
    pub min: usize,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max: Option<usize>,
    #[serde(flatten)]
    pub expr: ExpressionRef,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub separator: Option<ExpressionRef>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Hash)]
#[serde(deny_unknown_fields)]
pub struct EBNFDelimitedBy {
    pub open: String,
    #[serde(flatten)]
    pub expr: ExpressionRef,
    pub close: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum EBNF {
    End,
    Repeat(EBNFRepeat),
    Not(ExpressionRef),
    DelimitedBy(EBNFDelimitedBy),
    Choice(Vec<ExpressionRef>),
    Sequence(Vec<ExpressionRef>),
    Terminal(String),
    Reference(String),
    Difference(EBNFDifference),
    Range(EBNFRange),
}

pub type ExpressionRef = Rc<Expression>;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Hash)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct ExpressionConfig {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lookahead: Option<ExpressionRef>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prelude: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub associativity: Option<ExpressionAssociativity>,
}

impl ExpressionConfig {
    pub fn is_default(&self) -> bool {
        *self == Default::default()
    }
}

impl Default for ExpressionConfig {
    fn default() -> Self {
        Self {
            name: None,
            lookahead: None,
            prelude: None,
            associativity: None,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Hash)]
pub enum ExpressionAssociativity {
    Left,
    Right,
}

impl Grammar {
    pub fn from_manifest(manifest_path: &PathBuf) -> Self {
        let contents = std::fs::read(manifest_path).unwrap();
        let manifest_path_str = &manifest_path.to_str().unwrap();
        let manifest: Manifest = serde_yaml::from_slice(&contents).expect(manifest_path_str);

        let productions: IndexMap<String, Vec<ProductionRef>> = manifest
            .sections
            .iter()
            .flat_map(|section| &section.topics)
            .filter_map(|topic| match &topic.definition {
                None => None,
                Some(definition) => {
                    let topic_path = manifest_path.parent().unwrap().join(definition);
                    let topic_path_str = topic_path.to_str().unwrap();

                    let contents = std::fs::read(&topic_path).expect(topic_path_str);
                    let rules: Vec<Production> =
                        serde_yaml::from_slice(&contents).expect(topic_path_str);
                    let rules: Vec<ProductionRef> = rules.into_iter().map(|p| Rc::new(p)).collect();

                    return Some((definition.clone(), rules));
                }
            })
            .collect();

        let mut grammar = Grammar {
            manifest,
            productions,
        };

        grammar.post_initialize();

        grammar
    }

    fn post_initialize(&mut self) {
        self.create_combinator_trees();
    }
}
