use slang_solidity_v2_common::versions::LanguageVersion;

use crate::lexer::contexts::{ContextExtras, ContextWrapper};
use crate::lexer::lexemes::{Lexeme, LexemeKind};

pub struct Lexer<'source> {
    context: ContextWrapper<'source>,
    brace_depth: usize,
}

impl<'source> Lexer<'source> {
    pub fn new(source: &'source str, language_version: LanguageVersion) -> Self {
        let extras = ContextExtras { language_version };

        let context = ContextWrapper::new(source, extras);

        Self {
            context,
            brace_depth: 0,
        }
    }

    pub fn next_lexeme(&mut self) -> Option<Lexeme> {
        let lexeme = self.context.next_lexeme()?;

        match lexeme.kind {
            LexemeKind::PragmaKeyword_Reserved => {
                self.context = self.context.clone().morph_to_pragma();
            }
            LexemeKind::PragmaSemicolon => {
                self.context = self.context.clone().morph_to_solidity();
            }

            LexemeKind::AssemblyKeyword_Reserved => {
                self.brace_depth = 0;
                self.context = self.context.clone().morph_to_yul();
            }
            LexemeKind::YulOpenBrace => {
                self.brace_depth += 1;
            }
            LexemeKind::YulCloseBrace => {
                self.brace_depth -= 1;
                if self.brace_depth == 0 {
                    self.context = self.context.clone().morph_to_solidity();
                }
            }

            _ => {}
        }

        Some(lexeme)
    }
}
