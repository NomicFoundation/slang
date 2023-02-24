#[derive(Debug)]
#[allow(non_camel_case_types)]
pub enum TokenKind {
    comment,
    constant,
    keyword,
    operator,
    string,
}

pub trait EBNFWriter {
    fn write_token(&mut self, kind: TokenKind, value: &str);
    fn write_line_break(&mut self);

    fn write_production_definition(&mut self, production_name: &str);
    fn write_production_reference(&mut self, production_name: &str);
    fn write_local_production_reference(&mut self, _parent_name: &str, production_name: &str) {
        self.write_production_reference(production_name)
    }

    fn write_prelude(&mut self) {}
    fn write_postlude(&mut self) {}
    fn write_line_start(&mut self) {}

    fn write_comment(&mut self, value: &str) {
        self.write_token(TokenKind::comment, value)
    }
    fn write_constant(&mut self, value: &str) {
        self.write_token(TokenKind::constant, value)
    }
    fn write_keyword(&mut self, value: &str) {
        self.write_token(TokenKind::keyword, value)
    }
    fn write_operator(&mut self, value: &str) {
        self.write_token(TokenKind::operator, value)
    }
    fn write_string(&mut self, value: &str) {
        self.write_token(TokenKind::string, &Self::format_string_literal(value))
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
                    format!("\\{}", c)
                } else if c.is_ascii_graphic() || c == '¬' || c == '…' || c == '«' || c == '»'
                {
                    c.to_string()
                } else {
                    c.escape_unicode().to_string()
                }
            })
            .collect();

        return format!("{}{}{}", delimiter, formatted, delimiter);
    }
}

pub trait EBNFWritable<W: EBNFWriter> {
    fn write_ebnf(&self, name: &str, writer: &mut W);
}
