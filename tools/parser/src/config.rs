use std::collections::{BTreeSet, HashMap};

use chumsky::Parser;
use either::Either;
use yaml_rust::Yaml;

use crate::{parser::create_expression_parser, tree_builder::ExpressionRef, util::print_errors};

pub struct Configuration {
    // BTreeSet to ensure repeatability when regenerating
    parsers: BTreeSet<String>,
    productions: HashMap<String, ConfigData>,
}

impl Configuration {
    pub fn from(annotations: Vec<Yaml>) -> Self {
        let mut result = Self {
            parsers: Default::default(),
            productions: Default::default(),
        };

        for a in annotations {
            for p in a["parsers"].as_vec().unwrap() {
                result.parsers.insert(p.as_str().unwrap().to_owned());
            }
            for (name, a) in a["productions"].as_hash().unwrap() {
                let config = result
                    .productions
                    .entry(name.as_str().unwrap().to_owned())
                    .or_default();
                config.update_from(a);
            }
        }

        result
    }

    pub fn parsers(&self) -> &BTreeSet<String> {
        &self.parsers
    }

    pub fn production<'a>(&'a self, name: &String) -> ExpressionConfig<'a> {
        ExpressionConfig::from_root(self, self.productions.get(name))
    }
}

pub struct ExpressionConfig<'a> {
    parent: Either<&'a ExpressionConfig<'a>, &'a Configuration>,
    data: Option<&'a ConfigData>,
}

impl<'a> ExpressionConfig<'a> {
    fn from_root(parent: &'a Configuration, data: Option<&'a ConfigData>) -> Self {
        Self {
            parent: Either::Right(parent),
            data,
        }
    }

    fn from_child(parent: &'a ExpressionConfig, data: Option<&'a ConfigData>) -> Self {
        Self {
            parent: Either::Left(parent),
            data,
        }
    }

    pub fn empty(&'a self) -> ExpressionConfig<'a> {
        match self.parent {
            Either::Left(parent) => parent.empty(),
            Either::Right(configuration) => Self::from_root(configuration, None),
        }
    }

    pub fn production(&'a self, name: &String) -> ExpressionConfig<'a> {
        match self.parent {
            Either::Left(parent) => parent.production(name),
            Either::Right(configuration) => {
                Self::from_root(configuration, configuration.productions.get(name))
            }
        }
    }

    pub fn get(&'a self, index: usize, name: Option<&String>) -> ExpressionConfig<'a> {
        Self::from_child(
            self,
            self.data.and_then(|d| {
                d.children
                    .get(&index.to_string())
                    .or_else(|| name.map(|n| d.children.get(n)).flatten())
            }),
        )
    }

    pub fn ignore(&self) -> bool {
        self.data.map(|d| d.ignore).unwrap_or(false) || {
            match self.parent {
                Either::Left(parent) => parent.ignore(),
                Either::Right(_) => false,
            }
        }
    }

    pub fn nomap(&self) -> bool {
        self.data.map(|d| d.nomap).unwrap_or(false)
    }

    pub fn unwrap(&self) -> bool {
        self.data.map(|d| d.unwrap).unwrap_or(false)
    }

    pub fn chain(&self) -> bool {
        self.data.map(|d| d.chain).unwrap_or(false)
    }

    pub fn map(&self) -> Option<String> {
        self.data.map(|d| d.map.clone()).flatten()
    }

    pub fn lookahead(&self) -> Option<ExpressionRef> {
        self.data.map(|d| d.lookahead.clone()).flatten()
    }
}

struct ConfigData {
    pub ignore: bool,
    pub nomap: bool,
    pub map: Option<String>,
    pub unwrap: bool,
    pub chain: bool,
    pub lookahead: Option<ExpressionRef>,

    children: HashMap<String, ConfigData>,
}

impl ConfigData {
    fn update_from(&mut self, annotations: &Yaml) {
        for (name, a) in annotations.as_hash().unwrap() {
            if let Some(name) = name
                .as_str()
                .map(|s| s.to_owned())
                .or_else(|| name.as_i64().map(|i| i.to_string()))
            {
                match name.as_str() {
                    "ignore" => self.ignore = true,
                    "nomap" => self.nomap = true,
                    "unwrap" => self.unwrap = true,
                    "chain" => self.chain = true,
                    "map" => self.map = Some(a.as_str().unwrap().to_owned()),
                    "lookahead" => {
                        let source = a.as_str().unwrap();
                        let (expr, errs) = create_expression_parser().parse_recovery(source);
                        self.lookahead = expr;
                        print_errors(errs, source);
                    }
                    _ => {
                        let name = if name.starts_with("r#") {
                            name[2..].to_owned()
                        } else {
                            name
                        };
                        let config = self.children.entry(name).or_default();
                        config.update_from(a);
                    }
                }
            }
        }
    }
}

impl Default for ConfigData {
    fn default() -> Self {
        Self {
            ignore: false,
            nomap: false,
            map: None,
            unwrap: false,
            chain: false,
            lookahead: None,
            children: Default::default(),
        }
    }
}
