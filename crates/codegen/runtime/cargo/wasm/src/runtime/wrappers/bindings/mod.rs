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
        BindingGraph, BindingLocation, BuiltInLocation, UserFileLocation,
    };

    /// TODO: This is a work-around for the fact that `metaslang_bindings` internals (handles, locators, etc...) are exposed.
    /// We should clean this when we finally publish `__experimental_bindings_api`.
    /// That means removing the types below, and using the original types instead.
    #[derive(Debug, Clone)]
    pub struct Definition {
        pub id: usize,
        pub name_location: BindingLocation,
        pub definiens_location: BindingLocation,
    }

    impl From<crate::rust_crate::bindings::Definition> for Definition {
        fn from(definition: crate::rust_crate::bindings::Definition) -> Self {
            Self {
                id: definition.id(),
                name_location: definition.name_location(),
                definiens_location: definition.definiens_location(),
            }
        }
    }

    /// TODO: This is a work-around for the fact that `metaslang_bindings` internals (handles, locators, etc...) are exposed.
    /// We should clean this when we finally publish `__experimental_bindings_api`.
    /// That means removing the types below, and using the original types instead.
    #[derive(Debug, Clone)]
    pub struct Reference {
        pub id: usize,
        pub location: BindingLocation,
        pub definitions: Vec<Definition>,
    }

    impl From<crate::rust_crate::bindings::Reference> for Reference {
        fn from(reference: crate::rust_crate::bindings::Reference) -> Self {
            Self {
                id: reference.id(),
                location: reference.location(),
                definitions: reference
                    .definitions()
                    .into_iter()
                    .map(Into::into)
                    .collect(),
            }
        }
    }
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
            .map(rust::Definition::from)
            .map(IntoFFI::_into_ffi)
    }

    fn reference_at(&self,  cursor: ffi::CursorBorrow<'_>) -> Option<ffi::Reference> {
        self._borrow_ffi()
            .reference_at(&cursor._borrow_ffi())
            .map(rust::Reference::from)
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
        self._borrow_ffi().id.try_into().unwrap()
    }

    fn name_location(&self) -> ffi::BindingLocation {
        self._borrow_ffi().name_location.clone()._into_ffi()
    }

    fn definiens_location(&self) -> ffi::BindingLocation {
        self._borrow_ffi().definiens_location.clone()._into_ffi()
    }
} }

//================================================
//
// resource reference
//
//================================================

define_wrapper! { Reference {
    fn id(&self) -> u32 {
        self._borrow_ffi().id.try_into().unwrap()
    }

    fn location(&self) -> ffi::BindingLocation {
        self._borrow_ffi().location.clone()._into_ffi()
    }

    fn definitions(&self) -> Vec<ffi::Definition> {
        self._borrow_ffi().definitions.iter().cloned().map(IntoFFI::_into_ffi).collect()
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
