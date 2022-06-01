use indexmap::IndexMap;
use semver::Version;
use serde::{
    de::{self, MapAccess, Visitor},
    ser::SerializeMap,
    Deserialize, Serialize, Serializer,
};
use serde_yaml::Value;
use std::{collections::BTreeMap, fmt, path::PathBuf, rc::Rc};

#[derive(Clone, Debug)]
pub struct Grammar {
    pub manifest: Manifest,
    pub productions: IndexMap<String, Vec<Production>>,
}

impl Grammar {
    pub fn get_production(&self, name: &str) -> Option<&Production> {
        for (_, v) in &self.productions {
            if let p @ Some(_) = v.iter().find(|p| p.name == name) {
                return p;
            }
        }
        return None;
    }
}

#[derive(Deserialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct Manifest {
    pub title: String,
    pub sections: Vec<Section>,
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

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Production {
    pub name: String,
    pub is_token: bool,
    pub title: Option<String>,
    pub versions: BTreeMap<Version, ExpressionRef>,
}

impl Production {
    fn serialize_in_map<S: Serializer>(&self, state: &mut S::SerializeMap) -> Result<(), S::Error> {
        state.serialize_entry("name", &self.name)?;
        if self.is_token {
            state.serialize_entry("isToken", &self.is_token)?;
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
        #[derive(Deserialize)]
        #[serde(field_identifier, rename_all = "camelCase")]
        enum Field {
            Name,
            IsToken,
            Title,
            Versions,
            Config,
            End,
            ZeroOrMore,
            OneOrMore,
            Repeat,
            Optional,
            Not,
            Choice,
            Sequence,
            Terminal,
            Reference,
            Difference,
            Range,
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
                let mut is_token = None;
                let mut title = None;
                let mut versions = None;
                let mut config = None;
                let mut ebnf = None;

                while let Some(key) = map.next_key()? {
                    match key {
                        Field::Name => {
                            if name.is_some() {
                                return Err(de::Error::duplicate_field("name"));
                            }
                        }
                        Field::IsToken => {
                            if is_token.is_some() {
                                return Err(de::Error::duplicate_field("is_token"));
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
                        Field::IsToken => is_token = Some(map.next_value()?),
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

                let is_token = is_token.unwrap_or_default();

                Ok(Production {
                    name,
                    is_token,
                    title,
                    versions,
                })
            }
        }

        const FIELDS: &'static [&'static str] = &[
            "name",
            "isToken",
            "title",
            "versions",
            "config",
            "end",
            "zeroOrMore",
            "oneOrMore",
            "repeat",
            "optional",
            "not",
            "choice",
            "sequence",
            "difference",
            "terminal",
            "reference",
            "range",
        ];
        deserializer.deserialize_struct("Production", FIELDS, ProductionVisitor)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
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
            EBNF::Sequence(..) => 3,
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
            Config,
            End,
            ZeroOrMore,
            OneOrMore,
            Repeat,
            Optional,
            Not,
            Choice,
            Sequence,
            Terminal,
            Reference,
            Difference,
            Range,
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
            "config",
            "end",
            "zeroOrMore",
            "oneOrMore",
            "repeat",
            "optional",
            "not",
            "choice",
            "sequence",
            "difference",
            "terminal",
            "reference",
            "range",
        ];
        deserializer.deserialize_struct("Expression", FIELDS, ExpressionVisitor)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EBNFDifference {
    pub minuend: ExpressionRef,
    pub subtrahend: ExpressionRef,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EBNFRange {
    pub from: char,
    pub to: char,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
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

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum EBNF {
    End,
    Repeat(EBNFRepeat),
    Not(ExpressionRef),
    Choice(Vec<ExpressionRef>),
    Sequence(Vec<ExpressionRef>),
    Terminal(String),
    Reference(String),
    Difference(EBNFDifference),
    Range(EBNFRange),
}

pub type ExpressionRef = Rc<Expression>;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct ExpressionConfig {
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub ignore: bool,
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub nomap: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub map: Option<String>,
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub unwrap: bool,
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub chain: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lookahead: Option<ExpressionRef>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prelude: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub preserve_token_structure: Option<bool>,
}

impl ExpressionConfig {
    pub fn is_default(&self) -> bool {
        *self == Default::default()
    }
}

impl Default for ExpressionConfig {
    fn default() -> Self {
        Self {
            ignore: false,
            nomap: false,
            map: None,
            unwrap: false,
            chain: false,
            lookahead: None,
            prelude: None,
            preserve_token_structure: None,
        }
    }
}

impl Grammar {
    pub fn from_manifest(manifest_path: &PathBuf) -> Self {
        let contents = std::fs::read(manifest_path).unwrap();
        let manifest_path_str = &manifest_path.to_str().unwrap();
        let manifest: Manifest = serde_yaml::from_slice(&contents).expect(manifest_path_str);

        let topics: IndexMap<String, Vec<Production>> = manifest
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

                    return Some((definition.clone(), rules));
                }
            })
            .collect();

        return Grammar {
            manifest,
            productions: topics,
        };
    }
}
