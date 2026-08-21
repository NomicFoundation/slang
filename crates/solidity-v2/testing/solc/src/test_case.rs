use std::path::Path;

use anyhow::{Context, Result, bail};
use infra_utils::solc::parse_evm_target_name;
use slang_solidity_v2_common::collections::OrderedMap;
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
/// via `==== ExternalSource: <path> ====` (or `<source name>=<path>`)
/// delimiters. The `// ====` block holds settings, and everything after
/// `// ----` is the (runtime) expectation, which we ignore.
pub struct IsolTestCase {
    /// All source files making up this test, keyed by their compilation file
    /// id (the source name `solc` would use), in declaration order.
    pub files: OrderedMap<String, String>,
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

        let files = parse_sources(source_region, &default_name, test_path)?;

        Ok(Self {
            files,
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
) -> Result<OrderedMap<String, String>> {
    let test_dir = test_path.parent().unwrap_or(Path::new("."));

    let mut files = OrderedMap::default();

    // The current source starts out named after the file itself, so any content
    // before the first `Source:` delimiter (including a whole undelimited file)
    // becomes the implicit default source — matching how `solc` names a
    // single-file input.
    let mut current_name = default_name.to_owned();
    let mut current_content = String::new();

    for line in region.lines() {
        let trimmed = line.trim();
        if let Some(name) = parse_source_delimiter(trimmed) {
            flush(&mut files, &current_name, &mut current_content)?;
            // Reject the new name up front, as `isoltest` does when it reads the
            // delimiter, so a redefinition is caught even when its body is empty
            // and would never reach `flush`.
            ensure_undefined(&files, name)?;
            current_name = name.to_owned();
        } else if let Some(spec) = parse_external_source_delimiter(trimmed) {
            load_external_source(spec, test_dir, &mut files)?;
        } else {
            current_content.push_str(line);
            current_content.push('\n');
        }
    }

    flush(&mut files, &current_name, &mut current_content)?;

    Ok(files)
}

/// Emits the accumulated source, unless it's empty — e.g. the region before the
/// first `Source:` delimiter, which is just whitespace and isn't a real source.
/// `mem::take` leaves `content` empty for the next source.
fn flush(files: &mut OrderedMap<String, String>, name: &str, content: &mut String) -> Result<()> {
    if content.trim().is_empty() {
        content.clear();

        return Ok(());
    }

    push_source(files, name, std::mem::take(content))
}

/// Registers `content` under the source name `name`.
///
/// A test that defines the same source name twice is malformed, and `isoltest`
/// refuses to run it ("Multiple definitions of test source"). We reject it too.
fn push_source(files: &mut OrderedMap<String, String>, name: &str, content: String) -> Result<()> {
    ensure_undefined(files, name)?;
    files.insert(name.to_owned(), content);

    Ok(())
}

fn ensure_undefined(files: &OrderedMap<String, String>, name: &str) -> Result<()> {
    if files.contains_key(name) {
        bail!("multiple definitions of test source {name:?}");
    }

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

/// Loads a fixture referenced by an `ExternalSource` directive, either a bare
/// `<path>` or `<source name>=<path>`. The disk path is relative to the test
/// file's directory, and the spec splits on the *first* `=`, since a fixture's
/// file name may itself contain one (`a=_external/external.sol=sol`).
///
/// The left-hand side is a source *name*, not an import remapping — `isoltest`
/// stores it as `sources[externalSourceName]` and declares no remappings — so it
/// must not pre-empt relative resolution.
fn load_external_source(
    spec: &str,
    test_dir: &Path,
    files: &mut OrderedMap<String, String>,
) -> Result<()> {
    let (source_name, relative_path) = match spec.split_once('=') {
        Some((name, path)) => (name.trim(), path.trim()),
        None => (spec, spec),
    };

    let disk_path = test_dir.join(relative_path);
    let content = std::fs::read_to_string(&disk_path).with_context(|| {
        format!("Failed to read external source {relative_path:?} (at {disk_path:?})")
    })?;

    push_source(files, source_name, content)
}

/// `isoltest`'s placeholder for an EVM version that has not been released yet,
/// written in place of a target name (e.g. `EVMVersion: =@future`).
pub const FUTURE_EVM_VERSION: &str = "@future";

/// A successfully understood `EVMVersion` setting.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ParsedTarget {
    /// The test asks for [`FUTURE_EVM_VERSION`], an EVM version that hasn't
    /// been released yet, so there is no target to analyze it at.
    FutureSpec,
    /// The concrete target to analyze the test at.
    Target(EvmTarget),
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
/// When no setting is present, `default` is used.
///
/// [`ParsedTarget::FutureSpec`] means the test asks for an EVM version `slang`
/// has no target for at all. For now this only covers the [`FUTURE_EVM_VERSION`]
/// isoltest marker.
///
/// Anything else *is* an error: silently falling back on a setting we failed to
/// understand would analyze the test at the wrong target and quietly bake the
/// result into the baseline, so a setting `solc` accepts but we don't is treated
/// as a gap in *this* code.
pub fn resolve_evm_target(setting: Option<&str>, default: EvmTarget) -> Result<ParsedTarget> {
    let Some(setting) = setting else {
        return Ok(ParsedTarget::Target(default));
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

    let name = name.trim();

    // `isoltest`'s placeholder for the next, not-yet-released EVM version.
    if name.eq_ignore_ascii_case(FUTURE_EVM_VERSION) {
        return Ok(ParsedTarget::FutureSpec);
    }

    let bound = parse_evm_target_name(name)
        .with_context(|| format!("Unrecognized EVM target in 'EVMVersion: {setting}'."))?;

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
        return Ok(ParsedTarget::Target(default));
    }

    // Otherwise pick the supported target satisfying the constraint that sits
    // closest to the default (the nearest above for an unmet lower bound, the
    // nearest below for an unmet upper bound).
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
        .map(ParsedTarget::Target)
        .with_context(|| format!("No supported EVM target satisfies 'EVMVersion: {setting}'."))
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

    fn source_names(files: &OrderedMap<String, String>) -> Vec<&str> {
        files.keys().map(String::as_str).collect()
    }

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
        let resolve = |setting, default| match resolve_evm_target(setting, default).unwrap() {
            ParsedTarget::Target(target) => target,
            ParsedTarget::FutureSpec => panic!("'{setting:?}' should resolve to a concrete target"),
        };

        // No setting: the version's default target is used as-is.
        assert_eq!(resolve(None, EvmTarget::Istanbul), EvmTarget::Istanbul);

        // A lower bound the default already satisfies keeps the *version's*
        // default rather than jumping to the newest target ever — the crux of
        // the version-aware fix (a `>=byzantium` test at an Istanbul-default
        // version runs at Istanbul, not Amsterdam).
        assert_eq!(
            resolve(Some(">=byzantium"), EvmTarget::Istanbul),
            EvmTarget::Istanbul
        );

        // If the default is too old for the lower bound, pick the nearest
        // satisfying target above it.
        assert_eq!(
            resolve(Some(">=cancun"), EvmTarget::Istanbul),
            EvmTarget::Cancun
        );

        // Upper bounds the default violates resolve to the nearest satisfying
        // target below it.
        assert_eq!(
            resolve(Some("<constantinople"), EvmTarget::Osaka),
            EvmTarget::Byzantium
        );
        assert_eq!(
            resolve(Some("<=constantinople"), EvmTarget::Osaka),
            EvmTarget::Constantinople
        );

        // Exact constraints resolve to that target regardless of the default.
        assert_eq!(
            resolve(Some("=istanbul"), EvmTarget::Osaka),
            EvmTarget::Istanbul
        );
        assert_eq!(
            resolve(Some("istanbul"), EvmTarget::Osaka),
            EvmTarget::Istanbul
        );
    }

    #[test]
    fn yields_no_target_for_an_unreleased_evm_version() {
        // `@future` names an EVM version that doesn't exist yet, so there is
        // nothing to analyze the test at. That's reported as `FutureSpec` rather
        // than an error, because it says something about the test rather than
        // about this code — and the runner turns it into a failing test.
        for setting in ["=@future", "@future", ">=@future", "=@FUTURE"] {
            assert_eq!(
                resolve_evm_target(Some(setting), EvmTarget::Istanbul).unwrap(),
                ParsedTarget::FutureSpec,
                "'{setting}' should resolve to no target"
            );
        }
    }

    #[test]
    fn rejects_evm_version_settings_it_cannot_honor() {
        // A name we don't know is a gap in *our* table: resolving it to the
        // default would analyze the test at a target `solc` never used.
        assert!(resolve_evm_target(Some("nonsense"), EvmTarget::Osaka).is_err());

        // Likewise a well-formed constraint no supported target satisfies.
        assert!(resolve_evm_target(Some(">amsterdam"), EvmTarget::Osaka).is_err());
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
        let files = parse_sources(region, "input.sol", Path::new("/tmp/test.sol")).unwrap();

        assert_eq!(vec!["A", "B"], source_names(&files));
        assert!(files["A"].contains("contract A"));
        assert!(files["B"].contains("contract B is A"));
    }

    #[test]
    fn implicit_single_source_uses_default_name() {
        let region = "contract C {}\n";
        let files = parse_sources(region, "erc20.sol", Path::new("/tmp/erc20.sol")).unwrap();

        assert_eq!(vec!["erc20.sol"], source_names(&files));
    }

    /// `ExternalSource: <source name>=<path>` reads the fixture from `<path>`
    /// but enters it into the compilation under `<source name>`, so a source can
    /// be named something a relative import would never normalize to.
    #[test]
    fn external_source_is_named_by_the_left_hand_side() {
        let dir = tempfile::tempdir().unwrap();
        std::fs::create_dir_all(dir.path().join("_fixtures")).unwrap();
        std::fs::write(
            dir.path().join("_fixtures/dot_a.sol"),
            "contract Dot_A {}\n",
        )
        .unwrap();
        std::fs::write(
            dir.path().join("_fixtures/plain.sol"),
            "contract Plain {}\n",
        )
        .unwrap();

        let region = "\
==== ExternalSource: ./a.sol=_fixtures/dot_a.sol ====
==== ExternalSource: _fixtures/plain.sol ====
contract C {}
";
        let files = parse_sources(region, "input.sol", &dir.path().join("input.sol")).unwrap();

        // Sources keep their declaration order, and the remapped fixture is
        // named `./a.sol` rather than `_fixtures/dot_a.sol`; a bare directive
        // names the source after the path, as written.
        assert_eq!(
            vec!["./a.sol", "_fixtures/plain.sol", "input.sol"],
            source_names(&files)
        );
        assert!(files["./a.sol"].contains("contract Dot_A"));
        assert!(files["_fixtures/plain.sol"].contains("contract Plain"));
    }

    /// A test that defines the same source name twice is malformed; `isoltest`
    /// refuses to run it, so we refuse to parse it.
    #[test]
    fn rejects_a_redefined_source_name() {
        let parse = |region: &str| {
            parse_sources(region, "input.sol", Path::new("/tmp/input.sol"))
                .map_err(|error| error.to_string())
        };

        assert_eq!(
            parse("==== Source: A ====\ncontract A {}\n==== Source: A ====\ncontract A2 {}\n"),
            Err("multiple definitions of test source \"A\"".to_owned())
        );

        // Caught even when the redefinition is empty and would never be flushed
        // — the delimiter alone is the redefinition.
        assert_eq!(
            parse("==== Source: A ====\ncontract A {}\n==== Source: A ====\n"),
            Err("multiple definitions of test source \"A\"".to_owned())
        );

        // Distinct names are of course fine.
        assert!(
            parse("==== Source: A ====\ncontract A {}\n==== Source: B ====\ncontract B {}\n")
                .is_ok()
        );
    }

    /// The same, across the two kinds of delimiter: an external source may not
    /// take a name a `Source:` block already claimed, in either order.
    #[test]
    fn rejects_an_external_source_colliding_with_a_named_source() {
        let dir = tempfile::tempdir().unwrap();
        std::fs::write(dir.path().join("ext.sol"), "contract Ext {}\n").unwrap();

        let parse = |region: &str| {
            parse_sources(region, "input.sol", &dir.path().join("input.sol"))
                .map_err(|error| error.to_string())
        };

        assert_eq!(
            parse("==== Source: A ====\ncontract A {}\n==== ExternalSource: A=ext.sol ====\n"),
            Err("multiple definitions of test source \"A\"".to_owned())
        );
        assert_eq!(
            parse("==== ExternalSource: A=ext.sol ====\n==== Source: A ====\ncontract A {}\n"),
            Err("multiple definitions of test source \"A\"".to_owned())
        );

        // Including a bare directive naming the same fixture twice.
        assert_eq!(
            parse("==== ExternalSource: ext.sol ====\n==== ExternalSource: ext.sol ====\n"),
            Err("multiple definitions of test source \"ext.sol\"".to_owned())
        );
    }

    /// Fixture file names may contain `=`, so only the first one separates the
    /// source name from the path (`semanticTests/externalSource/multiple_equals_signs.sol`).
    #[test]
    fn external_source_splits_on_the_first_equals_sign() {
        let dir = tempfile::tempdir().unwrap();
        std::fs::write(
            dir.path().join("external.sol=sol"),
            "contract External {}\n",
        )
        .unwrap();

        let region = "==== ExternalSource: a=external.sol=sol ====\ncontract C {}\n";
        let files = parse_sources(region, "input.sol", &dir.path().join("input.sol")).unwrap();

        assert_eq!(vec!["a", "input.sol"], source_names(&files));
        assert!(files["a"].contains("contract External"));
    }
}
