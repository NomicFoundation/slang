use std::{collections::BTreeMap, fmt, rc::Rc};

use semver::Version;
use serde::{
    de::{self, MapAccess, Visitor},
    ser::SerializeMap,
    Deserialize, Serialize, Serializer,
};
use serde_yaml::Value;

pub type Grammar = Vec<Production>;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Production {
    pub name: String,
    #[serde(default)]
    #[serde(flatten)]
    pub expr: Option<ExpressionRef>,
    #[serde(default)]
    #[serde(skip_serializing_if = "BTreeMap::is_empty")]
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
            EBNF::Any => state.serialize_entry("any", &Value::Null),

            EBNF::ZeroOrMore(expr) => state.serialize_entry("zeroOrMore", &expr),
            EBNF::OneOrMore(expr) => state.serialize_entry("oneOrMore", &expr),
            EBNF::Optional(expr) => state.serialize_entry("optional", &expr),
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
            Any,
            ZeroOrMore,
            OneOrMore,
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
                        | Field::Any
                        | Field::ZeroOrMore
                        | Field::OneOrMore
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
                        Field::Any => ebnf = Some(EBNF::Any),
                        Field::ZeroOrMore => ebnf = Some(EBNF::ZeroOrMore(map.next_value()?)),
                        Field::OneOrMore => ebnf = Some(EBNF::OneOrMore(map.next_value()?)),
                        Field::Optional => ebnf = Some(EBNF::Optional(map.next_value()?)),
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
            "any",
            "zeroOrMore",
            "oneOrMore",
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

impl Expression {
    pub fn ref_from_ebnf(ebnf: EBNF) -> ExpressionRef {
        Rc::new(Self {
            config: Default::default(),
            ebnf,
        })
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

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum EBNF {
    End,
    Any,
    ZeroOrMore(ExpressionRef),
    OneOrMore(ExpressionRef),
    Optional(ExpressionRef),
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
    #[serde(default)]
    #[serde(skip_serializing_if = "std::ops::Not::not")]
    pub ignore: bool,
    #[serde(default)]
    #[serde(skip_serializing_if = "std::ops::Not::not")]
    pub nomap: bool,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub map: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "std::ops::Not::not")]
    pub unwrap: bool,
    #[serde(default)]
    #[serde(skip_serializing_if = "std::ops::Not::not")]
    pub chain: bool,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lookahead: Option<ExpressionRef>,
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
        }
    }
}
