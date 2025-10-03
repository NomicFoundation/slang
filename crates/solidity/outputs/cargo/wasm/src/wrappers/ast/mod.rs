#[path = "selectors.generated.rs"]
mod selectors;

use crate::utils::IntoFFI;

mod ffi {
    pub use crate::bindgen::exports::nomic_foundation::slang::ast::{Guest, GuestSelectors};
    pub use crate::bindgen::exports::nomic_foundation::slang::cst::{Node, NonterminalNodeBorrow};
}

mod rust {}

impl ffi::Guest for crate::World {
    type Selectors = SelectorsWrapper;
}

//================================================
//
// resource selectors
//
//================================================

pub struct SelectorsWrapper;

impl ffi::GuestSelectors for SelectorsWrapper {
    fn sequence(node: ffi::NonterminalNodeBorrow<'_>) -> Result<Vec<Option<ffi::Node>>, String> {
        Ok(selectors::select_sequence(node._borrow_ffi())?
            .into_iter()
            .map(|opt| opt.map(|node| node._into_ffi()))
            .collect())
    }

    fn choice(node: ffi::NonterminalNodeBorrow<'_>) -> Result<ffi::Node, String> {
        Ok(selectors::select_choice(node._borrow_ffi())?._into_ffi())
    }

    fn repeated(node: ffi::NonterminalNodeBorrow<'_>) -> Result<Vec<ffi::Node>, String> {
        Ok(selectors::select_repeated(node._borrow_ffi())?
            .into_iter()
            .map(|node| node._into_ffi())
            .collect())
    }

    fn separated(node: ffi::NonterminalNodeBorrow<'_>) -> Result<Vec<Vec<ffi::Node>>, String> {
        Ok(selectors::select_separated(node._borrow_ffi())?
            .into_iter()
            .map(|vec| vec.into_iter().map(|node| node._into_ffi()).collect())
            .collect())
    }
}
