use std::rc::Rc;

use semver::Version;

use crate::utils::{define_rc_wrapper, define_refcell_wrapper, IntoFFI};

mod ffi {
    pub use crate::bindgen::exports::nomic_foundation::slang::bindings::BindingGraph;
    pub use crate::bindgen::exports::nomic_foundation::slang::compilation::{
        AddFileResponse, CompilationUnit, CompilationUnitBorrow, CursorBorrow, File, FileBorrow,
        Guest, GuestCompilationUnit, GuestFile, GuestInternalCompilationBuilder,
        InternalCompilationBuilder, InternalCompilationBuilderBorrow,
    };
    pub use crate::bindgen::exports::nomic_foundation::slang::cst::{Cursor, NonterminalNode};
    pub use crate::bindgen::exports::nomic_foundation::slang::parser::ParseError;
}

mod rust {
    pub use slang_solidity::compilation::{
        AddFileResponse, CompilationUnit, File, InternalCompilationBuilder,
    };
}

impl ffi::Guest for crate::World {
    type InternalCompilationBuilder = InternalCompilationBuilderWrapper;
    type CompilationUnit = CompilationUnitWrapper;
    type File = FileWrapper;
}

//================================================
//
// resource internal-compilation-builder
//
//================================================

define_refcell_wrapper! { InternalCompilationBuilder {
    fn create(language_version: String) -> Result<ffi::InternalCompilationBuilder, String> {
        let language_version = Version::parse(&language_version).map_err(|e| e.to_string())?;

        rust::InternalCompilationBuilder::create(language_version)
            .map(IntoFFI::_into_ffi)
            .map_err(|e| e.to_string())
    }

    fn add_file(&self, id: String, contents: String) -> ffi::AddFileResponse {
        self._borrow_mut_ffi()
            .add_file(id, &contents)
            ._into_ffi()
    }

    fn resolve_import(&self, source_file_id: String, import_path: ffi::CursorBorrow<'_>, destination_file_id: String) -> Result<(), String> {
        self._borrow_mut_ffi()
            .resolve_import(&source_file_id, &import_path._borrow_ffi(), destination_file_id)
            .map_err(|e| e.to_string())
    }

    fn build(&self) -> ffi::CompilationUnit {
        Rc::new(self._borrow_ffi().build())._into_ffi()
    }
} }

//================================================
//
// record add-file-response
//
//================================================

impl IntoFFI<ffi::AddFileResponse> for rust::AddFileResponse {
    #[inline]
    fn _into_ffi(self) -> ffi::AddFileResponse {
        let Self { import_paths } = self;

        ffi::AddFileResponse {
            import_paths: import_paths.into_iter().map(IntoFFI::_into_ffi).collect(),
        }
    }
}

//================================================
//
// resource compilation-unit
//
//================================================

define_rc_wrapper! { CompilationUnit {
    fn language_version(&self) -> String {
        self._borrow_ffi().language_version().to_string()
    }

    fn files(&self) -> Vec<ffi::File> {
        self._borrow_ffi().files().into_iter().map(IntoFFI::_into_ffi).collect()
    }

     fn file(&self, id: String) -> Option<ffi::File> {
        self._borrow_ffi().file(&id).map(IntoFFI::_into_ffi)
    }

    fn binding_graph(&self) -> ffi::BindingGraph {
        Rc::clone(self._borrow_ffi().binding_graph())._into_ffi()
    }
} }

//================================================
//
// resource file
//
//================================================

define_rc_wrapper! { File {
    fn id(&self) -> String {
        self._borrow_ffi().id().to_owned()
    }

    fn tree(&self) -> ffi::NonterminalNode {
        self._borrow_ffi().tree().to_owned()._into_ffi()
    }

    fn errors(&self) -> Vec<ffi::ParseError> {
        self._borrow_ffi().errors().iter().map(|e| e.clone()._into_ffi()).collect()
    }

    fn create_tree_cursor(&self) -> ffi::Cursor {
        self._borrow_ffi().create_tree_cursor()._into_ffi()
    }
} }
