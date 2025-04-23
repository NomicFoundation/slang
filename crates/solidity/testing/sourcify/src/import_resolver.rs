use std::path::{Component, PathBuf};
use std::str::FromStr;

use anyhow::{bail, Error, Result};

pub struct ImportResolver {
    import_remaps: Vec<ImportRemap>,
    source_maps: Vec<SourceMap>,
}

impl ImportResolver {
    /// Given an import path from a source file, traverse the remappings to find the real filename of
    /// the target source file.
    pub fn resolve_import(&self, source_id: &str, import_path: &str) -> Option<String> {
        let source_virtual_path = self.get_virtual_path(source_id)?;

        if let Some(remapped_import) = self.remap_import(&source_virtual_path, import_path) {
            // Paths that have been remapped don't need to go through path resolution.
            return self.get_source_id(&remapped_import);
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

        self.get_source_id(&resolved_path).or_else(|| {
            if source_is_url {
                // Sometimes imports from URL-imports don't share the URL prefix
                self.get_source_id(import_path)
            } else if let Some(remapped_import) =
                self.remap_import(&source_virtual_path, &resolved_path)
            {
                // Sometimes relative paths still need to be remapped after being resolved
                self.get_source_id(&remapped_import)
            } else {
                // All other cases just say we couldn't resolve anything
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
            .reduce(|longest, current| {
                if current.match_len() > longest.match_len() {
                    current
                } else {
                    longest
                }
            })
            .map(|remap| import_path.replacen(&remap.prefix, &remap.target, 1))
    }
}

struct SourceMap {
    /// The actual filename for the source file, as found in the archive. This name can
    /// be used to read the content of a source file.
    source_id: String,
    /// The path to the source file in the contract's "virtual filesystem". This is the
    /// path to the source file as the contract was originally constructed. This value
    /// should be used when resolving imports to the real source files.
    virtual_path: String,
}

impl SourceMap {
    fn matches_virtual_path(&self, virtual_path: &str) -> bool {
        self.virtual_path == virtual_path || self.virtual_path.replace("//", "/") == virtual_path
    }

    fn matches_source_id(&self, source_id: &str) -> bool {
        self.source_id == source_id
    }
}

struct ImportRemap {
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
    fn new(remap_str: &str) -> Result<ImportRemap> {
        let Some((context, rest)) = remap_str.split_once(':') else {
            bail!("{remap_str}: Could not separate context from mapping");
        };

        let Some((prefix, target)) = rest.split_once('=') else {
            bail!("{remap_str}: Could not separate prefix and target");
        };

        Ok(ImportRemap {
            context: if context.is_empty() {
                None
            } else {
                Some(context.into())
            },
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

    /// The `match_size` is the length of the remap context + the length of the remap
    /// prefix. This is used to compare `ImportRemap`s: if a source file + import path combo matches
    /// two or more `ImportRemap`s, then it should use the one with the biggest match, since
    /// that one will be the most specific.
    fn match_len(&self) -> usize {
        self.context.as_ref().map(|c| c.len()).unwrap_or(0) + self.prefix.len()
    }

    /// Sometimes contracts contain a remapping entry that, for whatever reason,
    /// is buggy. We can collect buggy remaps here so they're skipped during
    /// import path resolution.
    pub fn has_known_bug(&self) -> bool {
        // Ex Contract: 0x56D47372A66b3f640Bff83E745dE7D10f4B29075
        // Remapping list includes ":./=remappings.txt/"
        if self.target.contains("remappings.txt") {
            return true;
        }

        false
    }
}

impl TryFrom<serde_json::Value> for ImportResolver {
    type Error = anyhow::Error;

    fn try_from(value: serde_json::Value) -> Result<ImportResolver, Self::Error> {
        let import_remaps: Vec<ImportRemap> = value
            .get("settings")
            .and_then(|settings| settings.get("remappings"))
            .and_then(|remappings| remappings.as_array())
            .ok_or(Error::msg(
                "Could not find settings.remappings array entry.",
            ))
            .map(|mappings| {
                mappings
                    .iter()
                    .filter_map(|mapping| mapping.as_str())
                    .filter_map(|m| ImportRemap::new(m).ok())
                    .filter(|remap| !remap.has_known_bug())
                    .collect()
            })?;

        let source_maps: Vec<SourceMap> = value
            .get("sources")
            .and_then(|sources| sources.as_object())
            .ok_or(Error::msg(
                "Could not get sources entry in contract metadata.",
            ))
            .map(|sources| {
                sources
                    .iter()
                    .filter_map(|(key, value)| {
                        value
                            .get("keccak256")
                            .and_then(|k| k.as_str())
                            .map(|source_id| SourceMap {
                                source_id: source_id.into(),
                                virtual_path: key.clone(),
                            })
                    })
                    .collect()
            })?;

        Ok(ImportResolver {
            import_remaps,
            source_maps,
        })
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
    let (proto, rest) = source_path
        .split_once("://")
        .ok_or(Error::msg("Cannot parse url"))?;
    let (host, path) = rest
        .split_once('/')
        .ok_or(Error::msg("Cannot parse path"))?;

    let resolved_path = resolve_relative_import(path, import_path)?;

    let mut res = String::new();
    res.push_str(proto);
    res.push_str("://");
    res.push_str(host);
    res.push('/');
    res.push_str(&resolved_path);

    Ok(res)
}

fn path_is_url(path: &str) -> bool {
    path.starts_with("http") || path.starts_with("ipfs")
}
