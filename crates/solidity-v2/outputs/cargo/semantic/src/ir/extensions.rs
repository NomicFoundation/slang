use super::nodes::IdentifierPathElement;
use super::source::Source;

impl IdentifierPathElement {
    pub fn unparse<'a>(&self, source: &'a (impl Source + ?Sized)) -> &'a str {
        match self {
            IdentifierPathElement::Identifier(identifier) => identifier.unparse(source),
            IdentifierPathElement::AddressKeyword => "address",
        }
    }
}
