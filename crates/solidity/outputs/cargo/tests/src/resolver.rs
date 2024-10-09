use metaslang_bindings::{FileKind, PathResolver};

pub struct TestsPathResolver;

impl PathResolver for TestsPathResolver {
    fn resolve_path(&self, context_path: &FileKind, path_to_resolve: &str) -> FileKind {
        let FileKind::User(context_path) = context_path else {
            return FileKind::Unknown;
        };
        if is_relative_path(path_to_resolve) {
            // Relative import: the actual path will be computed using the
            // context path (ie. the path of the importing source unit)
            normalize_path(path_to_resolve, get_parent_path(context_path))
                .map_or(FileKind::Unknown, FileKind::User)
        } else {
            // Direct import: this path will be used as-is
            FileKind::User(path_to_resolve.to_string())
        }
    }
}

fn is_relative_path(path: &str) -> bool {
    path.starts_with("./") || path.starts_with("../")
}

fn get_parent_path(path: &str) -> &str {
    let sep_index = path.rfind('/').map_or(0usize, |index| index + 1);
    &path[..sep_index]
}

fn normalize_path(path: &str, base_path: &str) -> Option<String> {
    let mut normalized = base_path.to_string();
    let mut path = path;
    while let Some(index) = path.find('/') {
        match &path[..=index] {
            "/" | "./" => (),
            "../" => {
                if normalized.is_empty() {
                    return None;
                }
                let last_sep = normalized[..normalized.len() - 1]
                    .rfind('/')
                    .unwrap_or(0usize);
                normalized.drain(last_sep..);
            }
            other => normalized.push_str(other),
        }
        path = &path[index + 1..];
    }
    normalized.push_str(path);
    Some(normalized)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_parent_path() {
        assert_eq!("", get_parent_path(""));
        assert_eq!("", get_parent_path("foo.sol"));
        assert_eq!("bar/", get_parent_path("bar/foo.sol"));
    }

    #[test]
    fn test_normalize_path() {
        assert_eq!("foo.sol", normalize_path("foo.sol", "").unwrap());
        assert_eq!("bar/foo.sol", normalize_path("foo.sol", "bar/").unwrap());
        assert_eq!("bar/foo.sol", normalize_path("./foo.sol", "bar/").unwrap());
        assert_eq!("foo.sol", normalize_path("../foo.sol", "bar/").unwrap());
        assert_eq!("foo.sol", normalize_path("./../foo.sol", "bar/").unwrap());
        assert_eq!(
            "foo.sol",
            normalize_path("../../foo.sol", "bar/baz/").unwrap()
        );
        assert_eq!(
            "foo.sol",
            normalize_path(".././../foo.sol", "bar/baz/").unwrap()
        );
        assert_eq!(
            "baz/foo.sol",
            normalize_path("../baz/foo.sol", "bar/").unwrap()
        );
        assert!(normalize_path("../foo.sol", "").is_none());
        assert!(normalize_path("../../foo.sol", "bar/").is_none());
    }
}
