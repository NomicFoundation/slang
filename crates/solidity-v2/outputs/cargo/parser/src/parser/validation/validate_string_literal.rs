use std::ops::Range;

use slang_solidity_v2_common::diagnostics::kinds::syntax::InvalidCharacterInString;

use crate::parser::GrammarCtx;

/// Reports a string literal, Solidity or Yul, holding a character outside of printable ASCII.
pub(crate) fn validate_string_literal(ctx: &mut GrammarCtx<'_>, range: Range<usize>) {
    if let Some(offsets) = first_non_printable_ascii(&ctx.source[range.clone()]) {
        ctx.diagnostics.push(
            ctx.file_id.to_owned(),
            range.start + offsets.start..range.start + offsets.end,
            InvalidCharacterInString,
        );
    }
}

/// The range, within a string literal's `text`, of its first character outside of printable
/// ASCII, leaving escape sequences aside: a line continuation holds a line break.
fn first_non_printable_ascii(text: &str) -> Option<Range<usize>> {
    // Fast path for valid strings
    if text.bytes().all(|byte| (b' '..=b'~').contains(&byte)) {
        return None;
    }

    let mut chars = text.char_indices().peekable();
    while let Some((offset, char)) = chars.next() {
        if char == '\\' {
            if let Some((_, '\r')) = chars.next() {
                chars.next_if(|&(_, next)| next == '\n');
            }
        } else if !(' '..='~').contains(&char) {
            return Some(offset..offset + char.len_utf8());
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::first_non_printable_ascii;

    #[test]
    fn first_non_printable_ascii_valid() {
        assert_eq!(
            first_non_printable_ascii(r#""hello, world! \n\t\\\"\x41\u00e9""#),
            None
        );
        // Line continuations: LF, CR, and CRLF.
        assert_eq!(first_non_printable_ascii("\"a\\\nb\\\rc\\\r\nd\""), None);
    }

    #[test]
    fn first_non_printable_ascii_invalid() {
        // `é` takes two bytes, starting right after the opening quote.
        assert_eq!(first_non_printable_ascii("\"é\""), Some(1..3));
        // Control characters.
        assert_eq!(first_non_printable_ascii("\"a\tb\""), Some(2..3));
        assert_eq!(first_non_printable_ascii("\"\x7f\""), Some(1..2));
        // A line continuation only holds one line break.
        assert_eq!(first_non_printable_ascii("\"a\\\n\nb\""), Some(4..5));
    }
}
