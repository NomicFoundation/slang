use crate::wasm_crate::utils::{define_rc_wrapper, define_wrapper, IntoFFI};

mod ffi {
    pub use crate::wasm_crate::bindgen::exports::nomic_foundation::slang::bindings::{
        BindingGraph, BindingGraphBorrow, BindingLocation, BuiltInLocation, BuiltInLocationBorrow,
        CursorBorrow, Definition, DefinitionBorrow, Guest, GuestBindingGraph, GuestBuiltInLocation,
        GuestDefinition, GuestReference, GuestUserFileLocation, Reference, ReferenceBorrow,
        UserFileLocation, UserFileLocationBorrow,
    };
    pub use crate::wasm_crate::bindgen::exports::nomic_foundation::slang::cst::Cursor;
}

mod rust {
    pub use crate::rust_crate::bindings::{
        BindingGraph, BindingLocation, BuiltInLocation, Definition, Reference, UserFileLocation,
    };
}

impl ffi::Guest for crate::wasm_crate::World {
    type BindingGraph = BindingGraphWrapper;

    type Definition = DefinitionWrapper;
    type Reference = ReferenceWrapper;

    type UserFileLocation = UserFileLocationWrapper;
    type BuiltInLocation = BuiltInLocationWrapper;
}

//================================================
//
// resource binding-graph
//
//================================================

define_rc_wrapper! { BindingGraph {
    fn definition_at(&self, cursor: ffi::CursorBorrow<'_>) -> Option<ffi::Definition> {
        self._borrow_ffi()
            .definition_at(&cursor._borrow_ffi())
            .map(IntoFFI::_into_ffi)
    }

    fn reference_at(&self,  cursor: ffi::CursorBorrow<'_>) -> Option<ffi::Reference> {
        self._borrow_ffi()
            .reference_at(&cursor._borrow_ffi())
            .map(IntoFFI::_into_ffi)
    }
} }

//================================================
//
// resource definition
//
//================================================

define_wrapper! { Definition {
    fn id(&self) -> u32 {
        self._borrow_ffi().id().try_into().unwrap()
    }

    fn name_location(&self) -> ffi::BindingLocation {
        self._borrow_ffi().name_location()._into_ffi()
    }

    fn definiens_location(&self) -> ffi::BindingLocation {
        self._borrow_ffi().definiens_location()._into_ffi()
    }

    fn references(&self) -> Vec<ffi::Reference> {
        self._borrow_ffi().references().iter().cloned().map(IntoFFI::_into_ffi).collect()
    }
} }

//================================================
//
// resource reference
//
//================================================

define_wrapper! { Reference {
    fn id(&self) -> u32 {
        self._borrow_ffi().id().try_into().unwrap()
    }

    fn location(&self) -> ffi::BindingLocation {
        self._borrow_ffi().location().clone()._into_ffi()
    }

    fn definitions(&self) -> Vec<ffi::Definition> {
        self._borrow_ffi().definitions().iter().cloned().map(IntoFFI::_into_ffi).collect()
    }
} }

//================================================
//
// variant binding-location
//
//================================================

impl IntoFFI<ffi::BindingLocation> for rust::BindingLocation {
    #[inline]
    fn _into_ffi(self) -> ffi::BindingLocation {
        match self {
            Self::BuiltIn(location) => ffi::BindingLocation::BuiltIn(location._into_ffi()),
            Self::UserFile(location) => ffi::BindingLocation::UserFile(location._into_ffi()),
        }
    }
}

//================================================
//
// resource user-file-location
//
//================================================

define_wrapper! { UserFileLocation {
    fn file_id(&self) -> String {
        self._borrow_ffi().file_id().to_owned()
    }

    fn cursor(&self) -> ffi::Cursor {
        self._borrow_ffi().cursor().to_owned()._into_ffi()
    }
} }

//================================================
//
// resource built-in-location
//
//================================================

define_wrapper! { BuiltInLocation {
} }
