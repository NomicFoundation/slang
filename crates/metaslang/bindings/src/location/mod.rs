use metaslang_cst::cursor::Cursor;
use metaslang_cst::kinds::KindTypes;

#[derive(Debug, Clone)]
pub enum BindingLocation<KT: KindTypes + 'static> {
    UserFile(UserFileLocation<KT>),
    BuiltIn(BuiltInLocation),
}

impl<KT: KindTypes + 'static> BindingLocation<KT> {
    pub fn user_file(file_id: String, cursor: Cursor<KT>) -> Self {
        Self::UserFile(UserFileLocation { file_id, cursor })
    }

    pub fn built_in() -> Self {
        Self::BuiltIn(BuiltInLocation {})
    }
}

#[derive(Debug, Clone)]
pub struct UserFileLocation<KT: KindTypes + 'static> {
    file_id: String,
    cursor: Cursor<KT>,
}

impl<KT: KindTypes + 'static> UserFileLocation<KT> {
    pub fn file_id(&self) -> &str {
        &self.file_id
    }

    pub fn cursor(&self) -> &Cursor<KT> {
        &self.cursor
    }
}

#[derive(Debug, Clone)]
pub struct BuiltInLocation {
    // We are not exposing a `file_id` or a `cursor` here, because we don't expose the underlying `built_ins.sol` file yet.
    // Still, we are using a dedicated type here to make it easier to map to the corresponding WIT variant.
}
