use std::{fs, path::PathBuf};

use quote::quote;

use super::rustfmt::rustfmt;
use crate::schema::*;

pub struct Context {
    pub no_default_map: bool,
    pub box_non_tokens: bool,
}

impl Grammar {
    pub fn generate_chumsky(&self, context: &Context, output_path: &PathBuf) {
        let mut preludes: Vec<String> = vec![];
        let mut parsers: Vec<String> = vec![];
        self.productions.iter().flat_map(|(_, p)| p).for_each(|p| {
            let expr = p.expression_to_generate();
            if let Some(prelude) = expr.config.prelude.clone() {
                preludes.push(prelude);
                parsers.push(p.generate_chumsky(self, context))
            }
        });

        fs::write(
            output_path,
            rustfmt(
                vec![
                    quote!(
                        use chumsky::prelude::*;
                        use chumsky::Parser;

                        pub type ErrorType = Simple<char>;
                    )
                    .to_string(),
                    preludes.join("\n\n"),
                    parsers.join("\n\n"),
                ]
                .join("\n\n"),
            ),
        )
        .expect("Unable to write to parser file");
    }
}
