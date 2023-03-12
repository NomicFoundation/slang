/// Maps token names to pygments color classes:
/// https://squidfunk.github.io/mkdocs-material/reference/code-blocks/#custom-syntax-theme
#[derive(Debug)]
pub enum TokenKind {
    Comment,
    Constant,
    Keyword,
    Operator,
    String,
}

pub trait EBNFWriter {
    /*
     * Statements:
     */

    fn write_prelude(&mut self) {}
    fn write_postlude(&mut self) {}

    fn write_line_start(&mut self);
    fn write_line_end(&mut self);

    /*
     * Productions:
     */

    fn write_global_definition(&mut self, name: &str);

    fn write_local_definition(&mut self, parent_name: &str, name: &str);

    fn write_global_reference(&mut self, name: &str);

    fn write_local_reference(&mut self, parent_name: &str, name: &str);

    /*
     * Tokens:
     */

    fn write_token(&mut self, kind: TokenKind, value: &str);

    fn write_comment(&mut self, value: &str) {
        self.write_token(TokenKind::Comment, value)
    }

    fn write_constant(&mut self, value: &str) {
        self.write_token(TokenKind::Constant, value)
    }

    fn write_keyword(&mut self, value: &str) {
        self.write_token(TokenKind::Keyword, value)
    }

    fn write_operator(&mut self, value: &str) {
        self.write_token(TokenKind::Operator, value)
    }

    fn write_string(&mut self, value: &str) {
        self.write_token(TokenKind::String, &format_string_literal(value))
    }
}

pub trait EBNFWritable<W: EBNFWriter> {
    fn write_ebnf(&self, name: &str, writer: &mut W);
}

fn format_string_literal(value: &str) -> String {
    let delimiter = if value.len() == 1 {
        if value.contains("'") && !value.contains('"') {
            '"'
        } else {
            '\''
        }
    } else {
        if value.contains('"') && !value.contains("'") {
            '\''
        } else {
            '"'
        }
    };

    let formatted: String = value
        .chars()
        .map(|c| {
            if c == '\'' || c == '\\' {
                format!("\\{c}")
            } else if c.is_ascii_graphic() || c == '¬' || c == '…' || c == '«' || c == '»' {
                c.to_string()
            } else {
                c.escape_unicode().to_string()
            }
        })
        .collect();

    return format!("{delimiter}{formatted}{delimiter}");
}
