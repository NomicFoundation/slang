use std::collections::{HashMap, HashSet};

use chumsky::Parser;
use yaml_rust::Yaml;

use crate::{
    generated_parser::create_expression_parser, tree_builder::ExpressionRef, util::print_errors,
};

pub struct Configuration {
    parsers: HashSet<String>,
    productions: HashMap<String, ExpressionConfig>,
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

    pub fn parsers(&self) -> &HashSet<String> {
        &self.parsers
    }

    pub fn production(&self, name: &String) -> Option<&ExpressionConfig> {
        self.productions.get(name)
    }
}

pub struct ExpressionConfig {
    pub ignore: bool,
    pub nomap: bool,
    pub map: Option<String>,
    pub unwrap: bool,
    pub chain: bool,
    pub lookahead: Option<ExpressionRef>,

    children: HashMap<String, ExpressionConfig>,
}

impl ExpressionConfig {
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

    pub fn get(&self, index: usize, name: Option<&String>) -> Option<&ExpressionConfig> {
        self.children
            .get(&index.to_string())
            .or_else(|| name.map(|n| self.children.get(n)).flatten())
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
            children: Default::default(),
        }
    }
}
