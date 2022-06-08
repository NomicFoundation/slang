use std::fmt::Write;

use super::named_character::NamedCharacter;

pub trait NamedString {
    fn slang_name(&self) -> String;
}

impl NamedString for String {
    fn slang_name(&self) -> String {
        let mut result = String::new();
        let mut last_was_named = None;
        for c in self.chars() {
            if let Some(name) = c.slang_name() {
                if last_was_named != None {
                    write!(&mut result, "_").unwrap();
                }
                write!(&mut result, "{}", name).unwrap();
                last_was_named = Some(true)
            } else {
                if last_was_named == Some(true) {
                    write!(&mut result, "_").unwrap();
                }
                write!(&mut result, "{}", c).unwrap();
                last_was_named = Some(false)
            }
        }
        result
    }
}
