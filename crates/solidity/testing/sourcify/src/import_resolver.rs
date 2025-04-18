use std::path::{Component, PathBuf};
use std::str::FromStr;

use anyhow::{bail, Error, Result};

pub struct ImportResolver {
    pub remappings: Vec<ImportRemap>,
    pub sources: Vec<FileMapping>,
}

impl ImportResolver {
    /// Given an import path from a source file, traverse the remappings to find the real filename of
    /// the target source file.
    pub fn resolve_import(&self, source_name: &str, import_path: &str) -> Result<String> {
        let source_file_path = self.get_virtual_path(source_name)?;

        if let Some(remapped_import) = self.remap_import(&source_file_path, import_path) {
            // Paths that have been remapped don't need to go through path resolution.
            return self.get_real_name(&remapped_import);
        }

        if import_path.starts_with('@') {
            return self.get_real_name(import_path);
        }

        if path_is_url(import_path) {
            // URL imports don't need path resolution
            return self.get_real_name(import_path);
        }

        let source_is_url = path_is_url(&source_file_path);

        let resolved_path = if source_is_url {
            resolve_relative_url_import(&source_file_path, import_path)?
        } else {
            resolve_relative_import(&source_file_path, import_path)?
        };

        match self.get_real_name(&resolved_path) {
            e @ Err(_) => {
                // Try to recover from some corner cases
                if source_is_url {
                    // Sometimes imports from URL-imports don't share the URL prefix
                    self.get_real_name(import_path)
                } else if let Some(remapped_import) =
                    self.remap_import(&source_file_path, &resolved_path)
                {
                    // Sometimes relative paths still need to be remapped after being resolved
                    self.get_real_name(&remapped_import)
                } else {
                    // All other cases just return an error
                    e
                }
            }
            real_name => real_name,
        }
    }

    pub fn get_real_name(&self, virtual_path: &str) -> Result<String> {
        for source in &self.sources {
            // On rare occasions imports may have '//' in between directories. solc seems
            // to consider these interchangeable with '/'
            if source.virtual_path == virtual_path
                || source.virtual_path.replace("//", "/") == virtual_path
            {
                return Ok(source.real_name.clone());
            }
        }
        Err(Error::msg(format!(
            "Could not get real name for import {virtual_path}"
        )))
    }

    pub fn get_virtual_path(&self, real_name: &str) -> Result<String> {
        for source in &self.sources {
            if source.real_name == real_name {
                return Ok(source.virtual_path.clone());
            }
        }
        Err(Error::msg(format!(
            "Could not get virtual path for source file {real_name}"
        )))
    }

    fn remap_import(&self, source_name: &str, import_path: &str) -> Option<String> {
        let mut longest_match: Option<&ImportRemap> = None;
        for remap in &self.remappings {
            if !remap.matches_context(source_name) {
                continue;
            }

            if import_path.starts_with(&remap.prefix) {
                match longest_match {
                    Some(r) => {
                        if r.len() < remap.len() {
                            longest_match = Some(remap);
                        }
                    }
                    None => {
                        longest_match = Some(remap);
                    }
                }
            }
        }

        longest_match.map(|r| import_path.replacen(&r.prefix, &r.target, 1))
    }
}

/// Resolve an import path that is relative to `source_path`.
fn resolve_relative_import(source_path: &str, import_path: &str) -> Result<String> {
    let source_file_path = PathBuf::from_str(source_path)?;
    let source_dir = source_file_path.parent().ok_or(Error::msg(format!(
        "Could not get directory of source file {source_path}"
    )))?;

    let import_path = PathBuf::from_str(import_path)?;

    let source_dir_parts: Vec<_> = source_dir.components().collect();
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
        Component::CurDir => resolved_parts.extend(source_dir_parts),
        // ../a/b.sol - relative, but one dir above `source_dir`
        Component::ParentDir => {
            resolved_parts.extend(source_dir_parts);
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

pub struct FileMapping {
    /// The actual filename for the source file, as found in the archive. This name can
    /// be used to read the content of a source file.
    pub real_name: String,
    /// The path to the source file in the contract's "virtual filesystem". This is the
    /// path to the source file as the contract was originally constructed. This value
    /// should be used when resolving imports to the real source files.
    pub virtual_path: String,
}

pub struct ImportRemap {
    /// If provided, then this remap only applies to imports inside source files
    /// whose paths begin with this string.
    context: Option<String>,
    /// The prefix value which will be found in the import path and replaced by
    /// `target`.
    prefix: String,
    /// The "real" import path. Replacing `prefix` with `target` in the import
    /// path from a source file should give you a path that can be looked up
    /// in `Metadata::sources`.
    target: String,
}

impl ImportRemap {
    pub fn new(remap_str: &str) -> Result<ImportRemap> {
        if let Some((context, rest)) = remap_str.split_once(':') {
            if let Some((prefix, target)) = rest.split_once('=') {
                Ok(ImportRemap {
                    context: if context.is_empty() {
                        None
                    } else {
                        Some(context.into())
                    },
                    prefix: prefix.into(),
                    target: target.into(),
                })
            } else {
                bail!("{remap_str}: Could not separate prefix and target");
            }
        } else {
            bail!("{remap_str}: Could not separate context from mapping");
        }
    }

    fn len(&self) -> usize {
        self.context.as_ref().map_or(0, |c| c.len()) + self.prefix.len()
    }

    fn matches_context(&self, source_name: &str) -> bool {
        if let Some(context) = &self.context {
            source_name.starts_with(context)
        } else {
            true
        }
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
        let remappings: Vec<ImportRemap> = value.get("settings")
            .and_then(|settings| settings.get("remappings"))
            .and_then(|remappings| remappings.as_array())
            .map(|mappings| mappings
                .iter()
                .filter_map(|mapping| mapping.as_str())
                .filter_map(|m| ImportRemap::new(m).ok())
                .filter(|remap| remap.has_known_bug())
                .collect()
            )
            .ok_or(Error::msg("Could not get remappings from contract metadata"))?;

        let sources: Vec<FileMapping> = value.get("sources")
            .and_then(|sources| sources.as_object())
            .map(|sources| sources.iter()
                .filter_map(|(key, value)| value.get("keccak256")
                    .and_then(|k| k.as_str())
                    .map(|real_name| FileMapping{
                        real_name: real_name.into(),
                        virtual_path: key.clone(),
                    })
                )
                .collect()
            )
            .ok_or(Error::msg("Could not get sources from contract metadata"))?;

        Ok(ImportResolver{
            remappings,
            sources,
        })
    }
}

fn path_is_url(path: &str) -> bool {
    path.starts_with("http") || path.starts_with("ipfs")
}
