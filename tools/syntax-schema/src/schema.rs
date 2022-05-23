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
    pub definition: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Production {
    pub name: String,
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub is_token: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,

    // TODO: use Either for the next two fields,
    // but it will require custom ser/de methods.
    // Or, use "0.0.0" and elide on serializing, which
    // is logically consistent.
    #[serde(flatten, default, skip_serializing_if = "Option::is_none")]
    pub expr: Option<ExpressionRef>,
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub versions: BTreeMap<Version, ExpressionRef>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Expression {
    pub config: ExpressionConfig,
    pub ebnf: EBNF,
}

impl Serialize for Expression {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_map(None)?;
        match &self.ebnf {
            // These two are ugly - serde has no way of emitting a map entry with
            // no value, which is valid in YAML. So these end up as e.g. `end: ~`
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
        state.end()
    }
}

impl<'de> Deserialize<'de> for Expression {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(Deserialize)]
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
                        Field::End => ebnf = Some(EBNF::End),
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
#[serde(deny_unknown_fields)]
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
}

impl ExpressionConfig {
    pub fn is_default(&self) -> bool {
        !self.ignore
            && !self.nomap
            && self.map.is_none()
            && !self.unwrap
            && !self.chain
            && self.lookahead.is_none()
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
            .map(|topic| {
                let topic_path = manifest_path.parent().unwrap().join(&topic.definition);
                let topic_path_str = topic_path.to_str().unwrap();

                let contents = std::fs::read(&topic_path).expect(topic_path_str);
                let rules: Vec<Production> =
                    serde_yaml::from_slice(&contents).expect(topic_path_str);

                return (topic.definition.clone(), rules);
            })
            .collect();

        return Grammar {
            manifest,
            productions: topics,
        };
    }
}
