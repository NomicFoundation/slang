use std::path::{Component, PathBuf};
use std::str::FromStr;

use anyhow::{Error, Result, bail};
use url::Url;

pub struct ImportResolver {
    pub import_remaps: Vec<ImportRemap>,
    pub source_maps: Vec<SourceMap>,
}

impl ImportResolver {
    /// Given an import path from a source file, traverse the remappings to find the real filename of
    /// the target source file.
    pub fn resolve_import(&self, source_id: &str, import_path: &str) -> Option<String> {
        let source_virtual_path = self.get_virtual_path(source_id)?;

        // solc resolves `./` and `../` against the importing file before it applies
        // remappings, so a `./=lib/` remapping never sees a relative import.
        let is_relative = import_path.starts_with("./") || import_path.starts_with("../");
        if !is_relative
            && let Some(remapped_import) = self.remap_import(&source_virtual_path, import_path)
            && let Some(source_id) = self.get_source_id(&remapped_import)
        {
            return Some(source_id);
        }

        if import_path.starts_with('@') {
            return self.get_source_id(import_path);
        }

        if path_is_url(import_path) {
            // URL imports don't need path resolution
            return self.get_source_id(import_path);
        }

        let source_is_url = path_is_url(&source_virtual_path);

        let resolved_path = if source_is_url {
            resolve_relative_url_import(&source_virtual_path, import_path).ok()?
        } else {
            resolve_relative_import(&source_virtual_path, import_path).ok()?
        };

        // solc applies remappings to the normalised path, so a remapping wins over a
        // source unit that happens to exist under the unremapped name.
        if !source_is_url
            && let Some(remapped_import) = self.remap_import(&source_virtual_path, &resolved_path)
            && let Some(source_id) = self.get_source_id(&remapped_import)
        {
            return Some(source_id);
        }

        self.get_source_id(&resolved_path).or_else(|| {
            if source_is_url {
                // Sometimes imports from URL-imports don't share the URL prefix
                self.get_source_id(import_path)
            } else {
                None
            }
        })
    }

    pub fn get_source_id(&self, virtual_path: &str) -> Option<String> {
        self.source_maps
            .iter()
            .find(|source| source.matches_virtual_path(virtual_path))
            .map(|source| source.source_id.clone())
    }

    pub fn get_virtual_path(&self, source_id: &str) -> Option<String> {
        self.source_maps
            .iter()
            .find(|source| source.matches_source_id(source_id))
            .map(|source| source.virtual_path.clone())
    }

    pub fn sources_count(&self) -> usize {
        self.source_maps.len()
    }

    fn remap_import(&self, source_virtual_path: &str, import_path: &str) -> Option<String> {
        self.import_remaps
            .iter()
            .filter(|remap| remap.matches(source_virtual_path, import_path))
            .reduce(|best, current| {
                if current.match_rank() >= best.match_rank() {
                    current
                } else {
                    best
                }
            })
            .map(|remap| import_path.replacen(&remap.prefix, &remap.target, 1))
    }
}

pub struct SourceMap {
    /// The actual filename for the source file, as found in the archive. This name can
    /// be used to read the content of a source file.
    pub source_id: String,
    /// The path to the source file in the contract's "virtual filesystem". This is the
    /// path to the source file as the contract was originally constructed. This value
    /// should be used when resolving imports to the real source files.
    pub virtual_path: String,
}

impl SourceMap {
    fn matches_virtual_path(&self, virtual_path: &str) -> bool {
        // Path resolution collapses `//`, including the one after a URL scheme, so
        // compare both sides collapsed.
        self.virtual_path == virtual_path
            || self.virtual_path.replace("//", "/") == virtual_path.replace("//", "/")
    }

    fn matches_source_id(&self, source_id: &str) -> bool {
        self.source_id == source_id
    }
}

pub struct ImportRemap {
    /// If provided, then this remap only applies to imports inside source files
    /// whose paths begin with this string.
    context: Option<String>,
    /// The prefix value which will be found in the import path and replaced by
    /// `target`.
    prefix: String,
    /// The target virtual path. Replacing `prefix` with `target` in the import
    /// path from a source file should give you a path that can be looked up
    /// in `Metadata::sources`.
    target: String,
}

impl ImportRemap {
    /// `[context:]prefix=target`, as solc spells it. The context separator is the `:`
    /// before the `=`; one after it belongs to the target (`solmate/=D:/lib/solmate/src/`).
    pub fn new(remap_str: &str) -> Result<ImportRemap> {
        let Some((context_and_prefix, target)) = remap_str.split_once('=') else {
            bail!("{remap_str}: Could not separate prefix and target");
        };

        let (context, prefix) = match context_and_prefix.split_once(':') {
            Some((context, prefix)) => (Some(context).filter(|c| !c.is_empty()), prefix),
            None => (None, context_and_prefix),
        };

        Ok(ImportRemap {
            context: context.map(Into::into),
            prefix: prefix.into(),
            target: target.into(),
        })
    }

    /// Determine if `self` applies to the import `import_path` found in the file at `source_path`.
    fn matches(&self, source_path: &str, import_path: &str) -> bool {
        let context_matches = if let Some(context) = &self.context {
            source_path.starts_with(context)
        } else {
            true
        };

        context_matches && import_path.starts_with(&self.prefix)
    }

    /// Among the remappings that match, solc takes the longest context first, the
    /// longest prefix second, and the last one listed on a tie
    /// (`CompilerStack::applyRemapping`), so a short contextual remapping beats a
    /// long global one and a repeated prefix means its last target.
    fn match_rank(&self) -> (usize, usize) {
        (
            self.context.as_ref().map_or(0, |c| c.len()),
            self.prefix.len(),
        )
    }

    /// Sometimes contracts contain a remapping entry that, for whatever reason,
    /// is buggy. We can collect buggy remaps here so they're skipped during
    /// import path resolution.
    pub fn has_known_bug(&self) -> bool {
        // Ex Contract: 0x56D47372A66b3f640Bff83E745dE7D10f4B29075
        // Remapping list includes ":./=remappings.txt/"
        self.target.contains("remappings.txt")
    }
}

/// Resolve an import path that is relative to `source_path`.
fn resolve_relative_import(source_path: &str, import_path: &str) -> Result<String> {
    let source_file_path = PathBuf::from_str(source_path)?;
    let source_dir = source_file_path.parent().ok_or(Error::msg(format!(
        "Could not get directory of source file {source_path}"
    )))?;

    let import_path = PathBuf::from_str(import_path)?;

    let mut resolved_parts = vec![];

    // We're basically doing what `Path::canonicalize()` would do, but we can't use that because
    // the path must be a real path in the filesystem, and we're operating on "virtual" paths.
    // We check the first import path component to initialize `resolved_parts`
    let mut import_path_components = import_path.components();
    match import_path_components.next().unwrap() {
        // a/b.sol - an absolute path, so we can ignore `source_path`
        norm @ Component::Normal(_) => resolved_parts.push(norm),
        // /a/b.sol
        root @ Component::RootDir => resolved_parts.push(root),
        // ./a/b.sol - relative to `source_path`
        Component::CurDir => resolved_parts.extend(source_dir.components()),
        // ../a/b.sol - relative, but one dir above `source_dir`
        Component::ParentDir => {
            resolved_parts.extend(source_dir.components());
            resolved_parts.pop();
        }
        Component::Prefix(_) => {
            bail!("Found prefix component in import path, which is not supported")
        }
    }

    for component in import_path_components {
        match component {
            norm @ Component::Normal(_) => resolved_parts.push(norm),
            Component::ParentDir => {
                resolved_parts.pop();
            }
            Component::CurDir => {}
            invalid => bail!("Invalid path component found inside import path: {invalid:?}"),
        }
    }

    let resolved_import_path: PathBuf = resolved_parts.iter().collect();
    Ok(resolved_import_path.to_str().unwrap().into())
}

/// Resolve an import from a source file which was imported using a URL. These need a bit of special handling
/// because the resolved path needs to also be a URL.
fn resolve_relative_url_import(source_path: &str, import_path: &str) -> Result<String> {
    let url = Url::parse(source_path)?;

    let path = url.path();
    let path = path.strip_prefix('/').unwrap_or(path);

    let resolved_path = resolve_relative_import(path, import_path)?;

    let port = url
        .port()
        .map(|port| format!(":{port}"))
        .unwrap_or_default();
    Ok(format!(
        "{scheme}://{host}{port}/{resolved_path}",
        scheme = url.scheme(),
        host = url.host_str().unwrap(),
    ))
}

fn path_is_url(path: &str) -> bool {
    Url::parse(path).is_ok_and(|url| url.is_special())
}

#[cfg(test)]
mod test {
    use super::{ImportRemap, ImportResolver, SourceMap};

    #[test]
    fn absolute_import() {
        let resolver = new_resolver(
            &[],
            &[("src/main.sol", "entry"), ("src/a/other.sol", "target")],
        );
        test_import(&resolver, "entry", "src/a/other.sol", "target");
    }

    #[test]
    fn a_drive_letter_in_the_target_is_not_a_context() {
        let resolver = new_resolver(
            &["solmate/=D:/lib/solmate/src/"],
            &[
                ("src/main.sol", "entry"),
                ("D:/lib/solmate/src/utils/Math.sol", "target"),
            ],
        );
        test_import(&resolver, "entry", "solmate/utils/Math.sol", "target");
    }

    #[test]
    fn a_relative_import_resolves_before_remappings_apply() {
        let resolver = new_resolver(
            &["./=lib/"],
            &[
                ("lib/contracts/eip/ERC1155.sol", "entry"),
                ("lib/contracts/eip/interface/IERC1155.sol", "target"),
            ],
        );
        test_import(&resolver, "entry", "./interface/IERC1155.sol", "target");
    }

    #[test]
    fn a_contextual_remapping_beats_a_longer_global_one() {
        let resolver = new_resolver(
            &[
                "@openzeppelin/contracts-upgradeable/=lib/oz-upgradeable/contracts/",
                "lib/t-rex/:@openzeppelin/=lib/t-rex/node_modules/@openzeppelin/",
            ],
            &[
                ("lib/t-rex/contracts/Module.sol", "entry"),
                (
                    "lib/t-rex/node_modules/@openzeppelin/contracts-upgradeable/proxy/utils/Initializable.sol",
                    "target",
                ),
                (
                    "lib/oz-upgradeable/contracts/proxy/utils/Initializable.sol",
                    "other",
                ),
            ],
        );
        test_import(
            &resolver,
            "entry",
            "@openzeppelin/contracts-upgradeable/proxy/utils/Initializable.sol",
            "target",
        );
    }

    #[test]
    fn a_relative_import_from_a_url_with_a_double_slash() {
        let resolver = new_resolver(
            &[],
            &[
                (
                    "https://github.com/oz/blob/v5//contracts/token/ERC1155.sol",
                    "entry",
                ),
                (
                    "https://github.com/oz/blob/v5//contracts/token/IERC1155.sol",
                    "target",
                ),
            ],
        );
        test_import(&resolver, "entry", "./IERC1155.sol", "target");
    }

    #[test]
    fn the_last_of_two_equal_remappings_wins() {
        let resolver = new_resolver(
            &[
                "@openzeppelin/=node_modules/@openzeppelin/contracts/",
                "@openzeppelin/=node_modules/@openzeppelin/",
            ],
            &[
                ("src/A.sol", "entry"),
                (
                    "node_modules/@openzeppelin/contracts/token/IERC20.sol",
                    "target",
                ),
            ],
        );
        test_import(
            &resolver,
            "entry",
            "@openzeppelin/contracts/token/IERC20.sol",
            "target",
        );
    }

    #[test]
    fn a_relative_import_from_a_url_keeps_the_port() {
        let resolver = new_resolver(
            &[],
            &[
                ("http://47.99.87.207:8080/token/ERC20.sol", "entry"),
                ("http://47.99.87.207:8080/token/IERC20.sol", "target"),
            ],
        );
        test_import(&resolver, "entry", "./IERC20.sol", "target");
    }

    #[test]
    fn a_remapping_applies_to_the_normalised_relative_path_first() {
        let resolver = new_resolver(
            &["axelar/=lib/axelar/"],
            &[
                ("axelar/executable/Executable.sol", "entry"),
                ("axelar/interfaces/IGateway.sol", "unremapped"),
                ("lib/axelar/interfaces/IGateway.sol", "target"),
            ],
        );
        test_import(&resolver, "entry", "../interfaces/IGateway.sol", "target");
    }

    #[test]
    fn relative_import() {
        let resolver = new_resolver(
            &[],
            &[("src/main.sol", "target"), ("src/a/other.sol", "entry")],
        );
        test_import(&resolver, "entry", "../main.sol", "target");
    }

    #[test]
    fn remapped_import() {
        let resolver = new_resolver(
            &[":xd=src/a"],
            &[("src/main.sol", "entry"), ("src/a/other.sol", "target")],
        );
        test_import(&resolver, "entry", "xd/other.sol", "target");
    }

    #[test]
    fn remapped_import_with_context() {
        let resolver = new_resolver(
            &["src/b:xd=src/a"],
            &[("src/a/other.sol", "target"), ("src/b/extra.sol", "entry")],
        );
        test_import(&resolver, "entry", "xd/other.sol", "target");
    }

    #[test]
    fn url_import() {
        let resolver = new_resolver(
            &[],
            &[
                ("src/main.sol", "entry"),
                ("https://github.com/org/project/main.sol", "target"),
            ],
        );
        test_import(
            &resolver,
            "entry",
            "https://github.com/org/project/main.sol",
            "target",
        );
    }

    #[test]
    fn relative_import_from_url_source() {
        let resolver = new_resolver(
            &[],
            &[
                ("https://github.com/org/project/main.sol", "target"),
                ("https://github.com/org/project/folder/other.sol", "entry"),
            ],
        );
        test_import(&resolver, "entry", "../main.sol", "target");
    }

    #[test]
    fn remap_with_relative_path() {
        // If a remap target contains a relative path, then that path is not expanded/resolved
        // when sourcify creates the sources list. Instead, it's used verbatim.
        let resolver = new_resolver(
            &[":xd=../folder"],
            &[("main.sol", "entry"), ("../folder/file.sol", "target")],
        );
        test_import(&resolver, "entry", "xd/file.sol", "target");
    }

    #[test]
    fn dueling_remaps() {
        let resolver = new_resolver(
            &["a:@org=node_modules/@org", "b:@org=node_modules/@org-v2"],
            &[
                ("a/file.sol", "entry_a"),
                ("b/file.sol", "entry_b"),
                ("node_modules/@org/main.sol", "target_a"),
                ("node_modules/@org-v2/main.sol", "target_b"),
            ],
        );
        test_import(&resolver, "entry_a", "@org/main.sol", "target_a");
        test_import(&resolver, "entry_b", "@org/main.sol", "target_b");
    }

    #[test]
    fn url_source_fallback() {
        // Sometimes, imports found in a URL source file don't share the URL prefix.
        // We can fallback on the import path to resolve the source file.
        let resolver = new_resolver(
            &[],
            &[
                ("https://github.com/org/project/main.sol", "entry"),
                ("file.sol", "target"),
            ],
        );
        test_import(&resolver, "entry", "file.sol", "target");
    }

    fn test_import(
        resolver: &ImportResolver,
        source_id: &str,
        import_path: &str,
        expected_id: &str,
    ) {
        let resolved_id = resolver
            .resolve_import(source_id, import_path)
            .expect("Could not resolve import");
        assert_eq!(resolved_id, expected_id);
    }

    fn new_resolver(remap_strs: &[&str], sources: &[(&str, &str)]) -> ImportResolver {
        let import_remaps: Vec<_> = remap_strs
            .iter()
            .flat_map(|s| ImportRemap::new(s))
            .collect();

        let source_maps: Vec<_> = sources
            .iter()
            .map(|(path, id)| SourceMap {
                source_id: id.to_owned().into(),
                virtual_path: path.to_owned().into(),
            })
            .collect();

        ImportResolver {
            import_remaps,
            source_maps,
        }
    }
}
