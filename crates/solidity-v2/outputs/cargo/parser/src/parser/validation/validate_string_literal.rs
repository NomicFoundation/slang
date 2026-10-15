use std::ops::Range;

use slang_solidity_v2_common::diagnostics::kinds::syntax::{
    InvalidCharacterInString, InvalidEscapeSequence, SyntaxDiagnosticKind,
};

use crate::parser::GrammarCtx;

/// Reports the first problem of a string literal, Solidity or Yul: a character outside of
/// printable ASCII, or an invalid escape sequence.
pub(crate) fn validate_string_literal(ctx: &mut GrammarCtx<'_>, range: Range<usize>) {
    report_first_problem(ctx, range, true);
}

/// Reports the first invalid escape sequence of a `unicode` string literal, which can hold
/// any character.
pub(crate) fn validate_unicode_string_literal(ctx: &mut GrammarCtx<'_>, range: Range<usize>) {
    report_first_problem(ctx, range, false);
}

fn report_first_problem(ctx: &mut GrammarCtx<'_>, range: Range<usize>, ascii_only: bool) {
    if let Some((offsets, diagnostic)) = first_problem(&ctx.source[range.clone()], ascii_only) {
        ctx.diagnostics.push(
            ctx.file_id.to_owned(),
            range.start + offsets.start..range.start + offsets.end,
            diagnostic,
        );
    }
}

/// The diagnostic for the first problem in a string literal's `text`, and where it is within
/// it: a character outside of printable ASCII when `ascii_only`, or an invalid escape sequence,
/// spanning from its backslash to the last character read for it.
fn first_problem(text: &str, ascii_only: bool) -> Option<(Range<usize>, SyntaxDiagnosticKind)> {
    // Fast path for valid strings
    if text
        .bytes()
        .all(|byte| byte != b'\\' && (!ascii_only || (b' '..=b'~').contains(&byte)))
    {
        return None;
    }

    let mut chars = text.char_indices().peekable();
    while let Some((offset, char)) = chars.next() {
        if char == '\\' {
            // The lexer only ends a string at its quote, so a backslash is never last.
            let Some((next_offset, next)) = chars.next() else {
                break;
            };
            let mut end = next_offset + next.len_utf8();
            let digits = match next {
                // Valid escape sequences
                '\'' | '"' | '\\' | 'n' | 'r' | 't' | '\n' => continue,
                '\r' => {
                    chars.next_if(|&(_, next)| next == '\n');
                    continue;
                }
                // `\x` or `\u` require extra digits
                'x' => 2,
                'u' => 4,
                // Invalid escape
                _ => return Some((offset..end, InvalidEscapeSequence.into())),
            };
            for _ in 0..digits {
                match chars.next_if(|(_, next)| next.is_ascii_hexdigit()) {
                    Some((digit_offset, _)) => end = digit_offset + 1,
                    None => return Some((offset..end, InvalidEscapeSequence.into())),
                }
            }
        } else if ascii_only && !(' '..='~').contains(&char) {
            return Some((
                offset..offset + char.len_utf8(),
                InvalidCharacterInString.into(),
            ));
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use slang_solidity_v2_common::diagnostics::kinds::syntax::{
        InvalidCharacterInString, InvalidEscapeSequence,
    };

    use super::first_problem;

    #[test]
    fn first_problem_valid() {
        assert_eq!(
            first_problem(r#""hello, world! \n\t\\\"\x41\u00e9""#, true),
            None
        );
        // Line continuations: LF, CR, and CRLF.
        assert_eq!(first_problem("\"a\\\nb\\\rc\\\r\nd\"", true), None);
        // `unicode` string literals can hold any character.
        assert_eq!(first_problem("\"é\\u00e9\"", false), None);
    }

    #[test]
    fn first_problem_invalid() {
        // `é` takes two bytes, starting right after the opening quote.
        assert_eq!(
            first_problem("\"é\"", true),
            Some((1..3, InvalidCharacterInString.into()))
        );
        // Control characters.
        assert_eq!(
            first_problem("\"a\tb\"", true),
            Some((2..3, InvalidCharacterInString.into()))
        );
        assert_eq!(
            first_problem("\"\x7f\"", true),
            Some((1..2, InvalidCharacterInString.into()))
        );
        // A line continuation only holds one line break.
        assert_eq!(
            first_problem("\"a\\\n\nb\"", true),
            Some((4..5, InvalidCharacterInString.into()))
        );
        // Unknown escape, spanning the backslash and the character after it.
        assert_eq!(
            first_problem("\"a\\qb\"", true),
            Some((2..4, InvalidEscapeSequence.into()))
        );
        // Short hex escape, spanning up to its last hex digit.
        assert_eq!(
            first_problem("\"\\x4\"", true),
            Some((1..4, InvalidEscapeSequence.into()))
        );
        // `unicode` string literals still report invalid escapes.
        assert_eq!(
            first_problem("\"é\\q\"", false),
            Some((3..5, InvalidEscapeSequence.into()))
        );
    }
}
