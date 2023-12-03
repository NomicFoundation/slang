use std::{path::PathBuf, rc::Rc};

pub type LocationRef = Rc<Location>;

pub enum Location {
    Root { file_path: PathBuf },
    Field { parent: LocationRef, field: String },
    Index { parent: LocationRef, index: usize },
}

impl Location {
    pub(super) fn root(file_path: PathBuf) -> LocationRef {
        LocationRef::new(Location::Root { file_path })
    }

    pub(super) fn field<S: Into<String>>(self: &LocationRef, field: S) -> LocationRef {
        LocationRef::new(Location::Field {
            parent: self.to_owned(),
            field: field.into(),
        })
    }

    pub(super) fn index(self: &LocationRef, index: usize) -> LocationRef {
        LocationRef::new(Location::Index {
            parent: self.to_owned(),
            index,
        })
    }

    pub(super) fn flatten(self: &LocationRef) -> (PathBuf, Vec<LocationRef>) {
        let mut path = Vec::new();
        let mut current = self;

        let file_path = loop {
            match current.as_ref() {
                Location::Root { file_path } => {
                    break file_path.to_owned();
                }
                Location::Index { parent, .. } | Location::Field { parent, .. } => {
                    path.push(current.to_owned());
                    current = parent;
                }
            }
        };

        (file_path, path)
    }
}
