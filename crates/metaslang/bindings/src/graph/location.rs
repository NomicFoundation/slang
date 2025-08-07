use metaslang_cst::cursor::Cursor;
use metaslang_cst::kinds::KindTypes;

/// Represents a location in the binding graph.
#[derive(Debug, Clone)]
pub enum BindingLocation<KT: KindTypes + 'static> {
    /// Represents a location in a user file.
    UserFile(UserFileLocation<KT>),
    /// Represents a location in the built-in definitions.
    BuiltIn(BuiltInLocation),
}

impl<KT: KindTypes + 'static> BindingLocation<KT> {
    /// Creates a new `BindingLocation` for a user file.
    pub fn user_file(file_id: String, cursor: Cursor<KT>) -> Self {
        Self::UserFile(UserFileLocation { file_id, cursor })
    }

    /// Creates a new `BindingLocation` for built-in definitions.
    pub fn built_in() -> Self {
        Self::BuiltIn(BuiltInLocation {})
    }
}

/// Represents a location in a user file.
#[derive(Debug, Clone)]
pub struct UserFileLocation<KT: KindTypes + 'static> {
    file_id: String,
    cursor: Cursor<KT>,
}

impl<KT: KindTypes + 'static> UserFileLocation<KT> {
    /// Returns the file ID of the user file location.
    pub fn file_id(&self) -> &str {
        &self.file_id
    }

    /// Returns the cursor pointing to the location in the user file.
    pub fn cursor(&self) -> &Cursor<KT> {
        &self.cursor
    }
}

/// Represents a built-in location in the binding graph.
#[derive(Debug, Clone)]
pub struct BuiltInLocation {
    // We are not exposing a `file_id` or a `cursor` here, because we don't expose the underlying `built_ins.sol` file yet.
    // Still, we are using a dedicated type here to make it easier to map to the corresponding WIT variant.
}
