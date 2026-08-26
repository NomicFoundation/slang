//! Resolves imports between in-memory sources the way `solc` does for a
//! standard JSON input with no remappings:
//!
//! - An import path starting with `.` or `..` is *relative*: joined onto the
//!   importing source's name (minus its last segment) and normalized.
//! - Anything else is *direct*: it names a source verbatim and is not
//!   normalized, so a source named `C/../////D/d.sol` is importable.

use slang_solidity_v2_common::files::FileId;

/// Resolves `import_path` as written inside the source named `source_file_id`,
/// yielding the name of the source it refers to.
///
/// Always yields a name; whether the compilation has a source under that name
/// is a separate question, reported as a missing file.
pub fn resolve_import(source_file_id: &FileId, import_path: &str) -> FileId {
    if is_relative_path(import_path) {
        normalize_path(import_path, get_parent_path(source_file_id.as_str())).into()
    } else {
        import_path.into()
    }
}

fn is_relative_path(path: &str) -> bool {
    path.starts_with("./") || path.starts_with("../")
}

/// The directory part of `path`, including its trailing `/` (empty when `path`
/// has no directory part).
fn get_parent_path(path: &str) -> &str {
    let sep_index = path.rfind('/').map_or(0usize, |index| index + 1);
    &path[..sep_index]
}

/// Joins a relative `path` onto `base_path` and normalizes away `.` and `..`
/// segments.
///
/// Source names are `/`-separated whatever the host, so this works on `str`
/// rather than `std::path`, whose separators and prefixes vary by platform.
///
/// `..` past the root is dropped rather than rejected, as it is in `solc`:
/// `../../../../x/h.sol` from `dir/contract.sol` resolves to `x/h.sol`, and a
/// rooted base path loses its root the same way.
fn normalize_path(path: &str, base_path: &str) -> String {
    let mut rooted = base_path.starts_with('/');
    let mut segments: Vec<&str> = base_path
        .split('/')
        .filter(|part| !part.is_empty())
        .collect();

    for part in path.split('/') {
        match part {
            "" | "." => {}
            ".." => {
                if segments.pop().is_none() {
                    rooted = false;
                }
            }
            segment => segments.push(segment),
        }
    }

    let root = if rooted { "/" } else { "" };

    format!("{root}{}", segments.join("/"))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn resolve(source: &str, import: &str) -> String {
        resolve_import(&FileId::from(source), import)
            .as_str()
            .to_owned()
    }

    #[test]
    fn test_get_parent_path() {
        assert_eq!("", get_parent_path(""));
        assert_eq!("", get_parent_path("foo.sol"));
        assert_eq!("bar/", get_parent_path("bar/foo.sol"));
    }

    #[test]
    fn test_normalize_path() {
        assert_eq!("foo.sol", normalize_path("foo.sol", ""));
        assert_eq!("bar/foo.sol", normalize_path("foo.sol", "bar/"));
        assert_eq!("bar/foo.sol", normalize_path("./foo.sol", "bar/"));
        assert_eq!("foo.sol", normalize_path("../foo.sol", "bar/"));
        assert_eq!("foo.sol", normalize_path("./../foo.sol", "bar/"));
        assert_eq!("foo.sol", normalize_path("../../foo.sol", "bar/baz/"));
        assert_eq!("foo.sol", normalize_path(".././../foo.sol", "bar/baz/"));
        assert_eq!("baz/foo.sol", normalize_path("../baz/foo.sol", "bar/"));

        // `..` with nothing left to walk up yields the name anyway.
        assert_eq!("foo.sol", normalize_path("../foo.sol", ""));
        assert_eq!("foo.sol", normalize_path("../../foo.sol", "bar/"));

        // A rooted base path keeps its root until a `..` pops it.
        assert_eq!("/bar/foo.sol", normalize_path("./foo.sol", "/bar/"));
        assert_eq!("/foo.sol", normalize_path("../foo.sol", "/bar/"));
        assert_eq!("foo.sol", normalize_path("../foo.sol", "/"));
    }

    #[test]
    fn direct_imports_name_a_source_verbatim() {
        // However non-normalized they look, these are source names, not paths.
        assert_eq!("C/../////D/d.sol", resolve("main.sol", "C/../////D/d.sol"));
        assert_eq!(
            "_nonNormalizedPaths//a.sol",
            resolve("main.sol", "_nonNormalizedPaths//a.sol")
        );
        assert_eq!("/ExtSource.sol", resolve("main.sol", "/ExtSource.sol"));
        assert_eq!("a", resolve("main.sol", "a"));
        assert_eq!(
            "sub_external.sol",
            resolve("dir/deep/main.sol", "sub_external.sol")
        );
    }

    #[test]
    fn relative_imports_resolve_against_the_importing_source() {
        assert_eq!("a.sol", resolve("main.sol", "./a.sol"));
        assert_eq!("dir/a.sol", resolve("dir/main.sol", "./a.sol"));
        assert_eq!("b.sol", resolve("dir/main.sol", "../b.sol"));

        // Walking up from a nested directory keeps the path separated.
        assert_eq!("pre/b.sol", resolve("pre/dir/main.sol", "../b.sol"));
        assert_eq!("pre/x/b.sol", resolve("pre/dir/main.sol", "../x/b.sol"));
        assert_eq!("a/b.sol", resolve("a/b/c/main.sol", "../../b.sol"));

        // `.` and `..` mix freely within one path.
        assert_eq!("b.sol", resolve("dir/main.sol", "./../b.sol"));
        assert_eq!("a/b.sol", resolve("a/b/c/main.sol", ".././../b.sol"));
    }

    /// `..` above the root is dropped, so resolution still lands on a name.
    #[test]
    fn dot_dot_past_the_root_is_clamped() {
        assert_eq!("oops.sol", resolve("input.sol", "../../oops.sol"));
        assert_eq!("oops.sol", resolve("dir/input.sol", "../../oops.sol"));
        assert_eq!("oops.sol", resolve("dir/input.sol", "../../../../oops.sol"));
        assert_eq!(
            "x/h.sol",
            resolve("dir/contract.sol", "../../../../x/h.sol")
        );
    }

    /// A rooted source name keeps its root; `..` past it drops the root, as in
    /// `solc`.
    #[test]
    fn rooted_source_names_keep_their_root() {
        assert_eq!("/dir/a.sol", resolve("/dir/main.sol", "./a.sol"));
        assert_eq!("/dir/B/b.sol", resolve("/dir/main.sol", "./B/b.sol"));
        assert_eq!("/b.sol", resolve("/dir/main.sol", "../b.sol"));
        assert_eq!("/x.sol", resolve("/ExtSource.sol", "./x.sol"));

        // One `..` is enough: `solc` walks up with `parent_path()`, whose parent
        // of `/` is empty.
        assert_eq!("x.sol", resolve("/a.sol", "../x.sol"));
        assert_eq!("x.sol", resolve("/a.sol", "../../x.sol"));
        assert_eq!("x.sol", resolve("/dir/sub/main.sol", "../../../x.sol"));
    }

    /// The cases `semanticTests/externalSource/relative_imports.sol` covers.
    #[test]
    fn resolves_the_relative_imports_semantic_test() {
        let source = "_relativeImports/dir/contract.sol";

        assert_eq!("_relativeImports/dir/a.sol", resolve(source, "./a.sol"));
        assert_eq!("_relativeImports/dir/B/b.sol", resolve(source, "./B/b.sol"));
        assert_eq!("_relativeImports/c.sol", resolve(source, "../c.sol"));
        assert_eq!("_relativeImports/D/d.sol", resolve(source, "../D/d.sol"));
        assert_eq!(
            "_relativeImports/dir/G/g.sol",
            resolve(source, "./E/../F/../G/./g.sol")
        );

        assert_eq!(
            "_relativeImports/h.sol",
            resolve(source, "../../../../_relativeImports/h.sol")
        );

        assert_eq!(
            "_relativeImports/c.sol",
            resolve("_relativeImports/dir/B/b.sol", "../../c.sol")
        );
        assert_eq!(
            "_relativeImports/dir/B/b.sol",
            resolve("_relativeImports/dir/G/g.sol", "../B/b.sol")
        );
    }

    /// Sources named `./a.sol` and `../b.sol` exist in
    /// `semanticTests/externalSource/source_name_starting_with_dots.sol`; a
    /// relative import spelled the same way resolves against the importing
    /// source anyway.
    #[test]
    fn dot_prefixed_source_names_do_not_capture_relative_imports() {
        let source = "_sourceNameStartingWithDots/dir/contract.sol";

        assert_eq!(
            "_sourceNameStartingWithDots/dir/a.sol",
            resolve(source, "./a.sol")
        );
        assert_eq!(
            "_sourceNameStartingWithDots/b.sol",
            resolve(source, "../b.sol")
        );
    }
}
