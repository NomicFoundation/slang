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
                // The brace is unmatched when the assembly block is missing its opening brace
                // ('assembly }'), in which case it actually closes an enclosing Solidity
                // construct. Either way, it ends the Yul context.
                //
                // TODO(v2): Note that the lexeme is still reported as a 'YulCloseBrace', since
                // the context has already been morphed by the time we can tell the brace is
                // unmatched. That is harmless today, because the parser stops at the first
                // syntax error. Once error recovery is implemented, the parser will keep going
                // past it, and will need the brace to carry its Solidity kind to close the
                // enclosing construct. Fixing that requires looking ahead for the opening brace
                // before morphing to the Yul context.
                self.brace_depth = self.brace_depth.saturating_sub(1);
                if self.brace_depth == 0 {
                    self.context = self.context.clone().morph_to_solidity();
                }
            }

            _ => {}
        }

        Some(lexeme)
    }
}
