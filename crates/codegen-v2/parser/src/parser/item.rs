use language_v2_definition::model::{Field, Identifier, Item as LanguageItem};
use serde::Serialize;

/// An item in the parser, these correspond roughly with the building blocks of LALRPOP grammars.
#[derive(Clone, Debug, Serialize)]
pub(crate) enum Item {
    /// A named capture of an item.
    Capture { name: Identifier, item: Box<Item> },
    /// A reference to another item.
    Reference { name: Identifier },
    // TODO: Versioned(),
    /// An optional item.
    Optional { item: Box<Item> },
    /// An item repeated at least once.
    ///
    /// Note: There's no way to represent 0 or more times, one reason to do it could be
    /// performance, future work.
    OneOrMore { item: Box<Item> },
    /// A sequence of items.
    Sequence { items: Vec<Item> },
    /// A choice of items.
    Choice { items: Vec<Item> },
    // TODO(maybe): Precedence { item: Rc<PrecedenceItem> },
}

impl TryFrom<&LanguageItem> for Item {
    // We don't really care about errors, since some Items just shouldn't be represented in the parser,
    //
    // Maybe another abstraction is better.
    type Error = ();

    fn try_from(item: &LanguageItem) -> Result<Self, Self::Error> {
        // TODO: we're ignoring versions for now
        use Item::*;

        match item {
            LanguageItem::Struct { item } => {
                let items = item.fields.iter().map(|(_, field)| match field {
                    Field::Required { reference } => Reference {
                        name: reference.clone(),
                    },
                    Field::Optional { reference, .. } => Optional {
                        item: Box::new(Reference {
                            name: reference.clone(),
                        }),
                    },
                });

                Ok(Sequence {
                    items: items.collect(),
                })
            }
            LanguageItem::Enum { item } => {
                let items = item.variants.iter().map(|variant| Reference {
                    name: variant.reference.clone(),
                });

                Ok(Choice {
                    items: items.collect(),
                })
            }
            LanguageItem::Repeated { item } => {
                let one_or_more = OneOrMore {
                    item: Box::new(Reference {
                        name: item.reference.clone(),
                    }),
                };
                if item.allow_empty.unwrap_or_default() {
                    // TODO what is the default for allow_empty?
                    Ok(one_or_more)
                } else {
                    Ok(Optional {
                        item: Box::new(one_or_more),
                    })
                }
            }
            LanguageItem::Separated { item } => {
                let one_or_more = Optional {
                    item: Box::new(OneOrMore {
                        item: Box::new(Sequence {
                            items: vec![
                                Reference {
                                    name: item.reference.clone(),
                                },
                                Reference {
                                    name: item.separator.clone(),
                                },
                            ],
                        }),
                    }),
                };

                if item.allow_empty.unwrap_or_default() {
                    Ok(Sequence {
                        items: vec![
                            one_or_more,
                            Reference {
                                name: item.reference.clone(),
                            },
                        ],
                    })
                } else {
                    Ok(one_or_more)
                }
            }
            // TODO: No idea how to do this yet
            LanguageItem::Precedence { .. } => {
                dbg!("Not yet implemented");
                Err(())
            }
            // I don't think we care about fragments at all
            // ... but we'll see once versioning comes in place
            LanguageItem::Trivia { .. }
            | LanguageItem::Keyword { .. }
            | LanguageItem::Token { .. }
            | LanguageItem::Fragment { .. } => Err(()),
        }
    }
}
