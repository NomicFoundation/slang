use std::path::Path;

use anyhow::{Context, Result};
use slang_solidity_v2_common::evm_targets::EvmTarget;

/// A single semantic test, parsed from the `isoltest` file format into the set
/// of in-memory source files it comprises plus the compiler settings that
/// affect how `slang` should analyze it.
///
/// The `isoltest` format looks like:
///
/// ```text
/// <solidity source>
/// // ====
/// // EVMVersion: >=byzantium
/// // compileViaYul: also
/// // ----
/// // f() -> 1
/// ```
///
/// where the source region may itself be split into multiple named sources via
/// `==== Source: <name> ====` delimiters, and may pull in shared fixture files
/// via `==== ExternalSource: <path> ====` (or `<import name>=<path>`)
/// delimiters. The `// ====` block holds settings, and everything after
/// `// ----` is the (runtime) expectation, which we ignore.
pub struct IsolTestCase {
    /// All source files making up this test, keyed by their compilation file
    /// id (the source name `solc` would use).
    pub files: Vec<(String, String)>,
    /// Import remappings declared via `ExternalSource: <name>=<path>`, mapping
    /// the import string to the resolved file id.
    pub remappings: Vec<(String, String)>,
    /// The raw `EVMVersion` setting (e.g. `>=byzantium`), if present. Resolved
    /// to a concrete [`EvmTarget`] by the runner, which knows the language
    /// version being used (see [`resolve_evm_target`]).
    pub evm_version: Option<String>,
}

impl IsolTestCase {
    /// Parses the test file at `test_path` (an absolute path), loading any
    /// referenced `ExternalSource` fixtures relative to it.
    pub fn parse(test_path: &Path) -> Result<Self> {
        let contents = std::fs::read_to_string(test_path)
            .with_context(|| format!("Failed to read test file: {test_path:?}"))?;

        let (source_region, settings) = split_trailer(&contents);
        let evm_version = setting_value(&settings, "EVMVersion");

        // The file id used for the implicit (undelimited) source is the test's
        // own file name, matching how `solc` names a single-file input.
        let default_name = test_path
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("input.sol")
            .to_owned();

        let mut files = Vec::new();
        let mut remappings = Vec::new();
        parse_sources(
            source_region,
            &default_name,
            test_path,
            &mut files,
            &mut remappings,
        )?;

        Ok(Self {
            files,
            remappings,
            evm_version: evm_version.map(ToOwned::to_owned),
        })
    }
}

/// Splits the file into (source region, settings lines), where the settings are
/// the `// key: value` lines appearing after the source and before the `// ----`
/// expectation delimiter.
fn split_trailer(contents: &str) -> (&str, Vec<(&str, &str)>) {
    let mut source_end = contents.len();
    let mut settings = Vec::new();
    let mut in_trailer = false;

    for (offset, line) in line_offsets(contents) {
        let trimmed = line.trim_end();
        if !in_trailer {
            // The source region ends at the first settings (`// ====`) or
            // expectation (`// ----`) delimiter. Both are exact delimiter lines
            // in the `isoltest` format (never carrying trailing text).
            if trimmed == "// ====" || trimmed == "// ----" {
                source_end = offset;
                in_trailer = true;
            } else {
                continue;
            }
        }

        // Everything from `// ----` onward is the runtime expectation, ignored.
        if trimmed == "// ----" {
            break;
        }
        if trimmed == "// ====" {
            continue;
        }
        if let Some((key, value)) = parse_setting_line(trimmed) {
            settings.push((key, value));
        }
    }

    (&contents[..source_end], settings)
}

/// Parses a `// key: value` settings line, returning `None` for anything that
/// isn't shaped like a setting. The returned slices borrow from `line`.
fn parse_setting_line(line: &str) -> Option<(&str, &str)> {
    let rest = line.strip_prefix("//")?.trim();
    let (key, value) = rest.split_once(':')?;
    let key = key.trim();
    if key.is_empty() || key.contains(char::is_whitespace) {
        return None;
    }
    Some((key, value.trim()))
}

fn setting_value<'a>(settings: &[(&'a str, &'a str)], key: &str) -> Option<&'a str> {
    settings
        .iter()
        .find(|(name, _)| *name == key)
        .map(|(_, value)| *value)
}

/// Parses the source region into named files, following `Source` and
/// `ExternalSource` delimiters.
fn parse_sources(
    region: &str,
    default_name: &str,
    test_path: &Path,
    files: &mut Vec<(String, String)>,
    remappings: &mut Vec<(String, String)>,
) -> Result<()> {
    let test_dir = test_path.parent().unwrap_or(Path::new("."));

    // The current source starts out named after the file itself, so any content
    // before the first `Source:` delimiter (including a whole undelimited file)
    // becomes the implicit default source — matching how `solc` names a
    // single-file input.
    let mut current_name = default_name.to_owned();
    let mut current_content = String::new();

    // Emit the accumulated source, unless it's empty — e.g. the region before
    // the first `Source:` delimiter, which is just whitespace and isn't a real
    // source. `mem::take` leaves `content` empty for the next source.
    let flush = |name: &str, content: &mut String, files: &mut Vec<(String, String)>| {
        if content.trim().is_empty() {
            content.clear();
        } else {
            files.push((name.to_owned(), std::mem::take(content)));
        }
    };

    for line in region.lines() {
        let trimmed = line.trim();
        if let Some(name) = parse_source_delimiter(trimmed) {
            flush(&current_name, &mut current_content, files);
            current_name = name.to_owned();
        } else if let Some(spec) = parse_external_source_delimiter(trimmed) {
            load_external_source(spec, test_dir, files, remappings)?;
        } else {
            current_content.push_str(line);
            current_content.push('\n');
        }
    }

    flush(&current_name, &mut current_content, files);

    Ok(())
}

fn parse_source_delimiter(line: &str) -> Option<&str> {
    let inner = line.strip_prefix("==== Source:")?.strip_suffix("====")?;
    Some(inner.trim())
}

fn parse_external_source_delimiter(line: &str) -> Option<&str> {
    let inner = line
        .strip_prefix("==== ExternalSource:")?
        .strip_suffix("====")?;
    Some(inner.trim())
}

/// Loads a fixture referenced by an `ExternalSource` directive. The spec is
/// either a bare `<path>` or an `<import name>=<path>` remapping. Paths are
/// resolved relative to the test file's directory.
fn load_external_source(
    spec: &str,
    test_dir: &Path,
    files: &mut Vec<(String, String)>,
    remappings: &mut Vec<(String, String)>,
) -> Result<()> {
    let (import_name, relative_path) = match spec.split_once('=') {
        Some((name, path)) => (Some(name.trim()), path.trim()),
        None => (None, spec),
    };

    let disk_path = test_dir.join(relative_path);
    let content = std::fs::read_to_string(&disk_path).with_context(|| {
        format!("Failed to read external source {relative_path:?} (at {disk_path:?})")
    })?;

    // The file id within the compilation is the path as written in the
    // directive, so that relative imports inside the fixture resolve correctly
    // and other sources can import it by that path.
    if !files.iter().any(|(id, _)| id == relative_path) {
        files.push((relative_path.to_owned(), content));
    }

    if let Some(import_name) = import_name {
        remappings.push((import_name.to_owned(), relative_path.to_owned()));
    }

    Ok(())
}

/// Resolves the `EVMVersion` setting (e.g. `>=byzantium`, `<constantinople`,
/// `istanbul`) to a single concrete [`EvmTarget`] to analyze the test at.
///
/// `default` is the EVM target `solc` of the language version being tested would
/// pick on its own (the runner passes the version's default). We prefer it
/// whenever it satisfies the constraint, so a lower-bound-only setting like
/// `>=byzantium` stays at the version's real default rather than jumping to the
/// newest target ever — which is a combination that `solc` of that version never
/// ran, and would spuriously enable target-gated builtins. Only when the default
/// doesn't satisfy the constraint (e.g. a hard `<constantinople` upper bound on a
/// newer default) do we pick the *nearest* satisfying target to the default.
///
/// When no setting is present, or the constraint can't be parsed or satisfied by
/// any supported target, `default` is used.
pub fn resolve_evm_target(setting: Option<&str>, default: EvmTarget) -> EvmTarget {
    let Some(setting) = setting else {
        return default;
    };

    let setting = setting.trim();
    let (op, name) = if let Some(name) = setting.strip_prefix(">=") {
        (">=", name)
    } else if let Some(name) = setting.strip_prefix("<=") {
        ("<=", name)
    } else if let Some(name) = setting.strip_prefix('>') {
        (">", name)
    } else if let Some(name) = setting.strip_prefix('<') {
        ("<", name)
    } else if let Some(name) = setting.strip_prefix('=') {
        ("=", name)
    } else {
        ("=", setting)
    };

    let Some(bound) = parse_evm_target_name(name.trim()) else {
        return default;
    };

    let satisfies = |target: EvmTarget| match op {
        ">=" => target >= bound,
        ">" => target > bound,
        "<=" => target <= bound,
        "<" => target < bound,
        _ => target == bound,
    };

    // Prefer the version's own default target when it already satisfies the
    // constraint.
    if satisfies(default) {
        return default;
    }

    // Otherwise pick the supported target satisfying the constraint that sits
    // closest to the default (the nearest above for an unmet lower bound, the
    // nearest below for an unmet upper bound). Falls back to `default` when
    // nothing satisfies it (e.g. an unsatisfiable `>osaka`).
    let index_of = |target: EvmTarget| EvmTarget::ALL.iter().position(|t| *t == target);
    let default_index = index_of(default);
    EvmTarget::ALL
        .iter()
        .copied()
        .filter(|target| satisfies(*target))
        .min_by_key(|target| match (index_of(*target), default_index) {
            (Some(i), Some(d)) => i.abs_diff(d),
            _ => usize::MAX,
        })
        .unwrap_or(default)
}

/// Maps an `EVMVersion` name (as written by `solc`, e.g. `tangerineWhistle`) to
/// an [`EvmTarget`], case-insensitively.
pub fn parse_evm_target_name(name: &str) -> Option<EvmTarget> {
    EvmTarget::ALL
        .iter()
        .copied()
        .find(|target| target.to_string().eq_ignore_ascii_case(name))
}

/// Yields `(byte offset, line)` for each line in `contents`.
fn line_offsets(contents: &str) -> impl Iterator<Item = (usize, &str)> {
    contents.lines().scan(0usize, |offset, line| {
        let start = *offset;
        // +1 accounts for the `\n` stripped by `lines()`. This slightly
        // over-counts on `\r\n` inputs, but semantic tests use `\n`, and we
        // only use the offset to slice at line starts.
        *offset = start + line.len() + 1;
        Some((start, line))
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn splits_settings_from_source() {
        let contents = "\
contract C {}
// ====
// EVMVersion: >=byzantium
// compileViaYul: also
// ----
// f() -> 1
";
        let (source, settings) = split_trailer(contents);
        assert_eq!(source.trim(), "contract C {}");
        assert_eq!(setting_value(&settings, "EVMVersion"), Some(">=byzantium"));
        assert_eq!(setting_value(&settings, "compileViaYul"), Some("also"));
        assert_eq!(setting_value(&settings, "missing"), None);
    }

    #[test]
    fn source_without_trailer_is_all_source() {
        let contents = "pragma solidity >=0.4.0;\ncontract C {}\n";
        let (source, settings) = split_trailer(contents);
        assert_eq!(source, contents);
        assert!(settings.is_empty());
    }

    #[test]
    fn resolves_evm_version_constraints() {
        // No setting: the version's default target is used as-is.
        assert_eq!(
            resolve_evm_target(None, EvmTarget::Istanbul),
            EvmTarget::Istanbul
        );

        // A lower bound the default already satisfies keeps the *version's*
        // default rather than jumping to the newest target ever — the crux of
        // the version-aware fix (a `>=byzantium` test at an Istanbul-default
        // version runs at Istanbul, not Amsterdam).
        assert_eq!(
            resolve_evm_target(Some(">=byzantium"), EvmTarget::Istanbul),
            EvmTarget::Istanbul
        );

        // If the default is too old for the lower bound, pick the nearest
        // satisfying target above it.
        assert_eq!(
            resolve_evm_target(Some(">=cancun"), EvmTarget::Istanbul),
            EvmTarget::Cancun
        );

        // Upper bounds the default violates resolve to the nearest satisfying
        // target below it.
        assert_eq!(
            resolve_evm_target(Some("<constantinople"), EvmTarget::Osaka),
            EvmTarget::Byzantium
        );
        assert_eq!(
            resolve_evm_target(Some("<=constantinople"), EvmTarget::Osaka),
            EvmTarget::Constantinople
        );

        // Exact constraints resolve to that target regardless of the default.
        assert_eq!(
            resolve_evm_target(Some("=istanbul"), EvmTarget::Osaka),
            EvmTarget::Istanbul
        );
        assert_eq!(
            resolve_evm_target(Some("istanbul"), EvmTarget::Osaka),
            EvmTarget::Istanbul
        );

        // An unsatisfiable constraint (nothing newer than the latest target)
        // falls back to the default rather than picking a violating target.
        assert_eq!(
            resolve_evm_target(Some(">amsterdam"), EvmTarget::Osaka),
            EvmTarget::Osaka
        );

        // Unknown names fall back to the default target.
        assert_eq!(
            resolve_evm_target(Some("nonsense"), EvmTarget::Osaka),
            EvmTarget::Osaka
        );
    }

    #[test]
    fn splits_multiple_named_sources() {
        let region = "\
==== Source: A ====
contract A {}
==== Source: B ====
import \"A\";
contract B is A {}
";
        let mut files = Vec::new();
        let mut remappings = Vec::new();
        parse_sources(
            region,
            "input.sol",
            Path::new("/tmp/test.sol"),
            &mut files,
            &mut remappings,
        )
        .unwrap();

        assert_eq!(files.len(), 2);
        assert_eq!(files[0].0, "A");
        assert!(files[0].1.contains("contract A"));
        assert_eq!(files[1].0, "B");
        assert!(files[1].1.contains("contract B is A"));
        assert!(remappings.is_empty());
    }

    #[test]
    fn implicit_single_source_uses_default_name() {
        let region = "contract C {}\n";
        let mut files = Vec::new();
        let mut remappings = Vec::new();
        parse_sources(
            region,
            "erc20.sol",
            Path::new("/tmp/erc20.sol"),
            &mut files,
            &mut remappings,
        )
        .unwrap();

        assert_eq!(files.len(), 1);
        assert_eq!(files[0].0, "erc20.sol");
    }
}
