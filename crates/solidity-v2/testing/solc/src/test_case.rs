use std::path::Path;

use anyhow::{Context, Result, bail};
use slang_solidity_v2_common::collections::OrderedMap;

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
    ///
    /// [`EvmTarget`]: slang_solidity_v2_common::evm_targets::EvmTarget
    /// [`resolve_evm_target`]: crate::evm_target::resolve_evm_target
    pub evm_version: Option<String>,
}

impl IsolTestCase {
    /// Parses the test file at `test_path` (an absolute path), loading any
    /// referenced `ExternalSource` fixtures relative to it.
    pub fn parse(test_path: &Path) -> Result<Self> {
        let contents = std::fs::read_to_string(test_path)
            .with_context(|| format!("Failed to read test file: {test_path:?}"))?;

        let (source_region, settings) = split_trailer(&contents);

        check_settings_are_known(&settings)
            .with_context(|| format!("Unsupported settings in test file: {test_path:?}"))?;

        let evm_version = setting_value(&settings, EVM_VERSION_SETTING);

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

/// The one setting that changes how we analyze a test.
const EVM_VERSION_SETTING: &str = "EVMVersion";

/// The settings we knowingly ignore.
const ALLOWED_SETTINGS: &[&str] = &[
    EVM_VERSION_SETTING,
    // All the other settings are ignored
    // ----------------------------------
    // Which code generation pipeline(s) to run the test through, and in which
    // bytecode format to emit the result.
    "compileViaYul",
    "compileViaSSACFG",
    "compileToEwasm",
    "bytecodeFormat",
    // Code generation options, likewise.
    "revertStrings",
    // Restricts which ABI coder the test runs under.
    "ABIEncoderV1Only",
    // The same, for the experimental language mode: the source carries the
    // `experimental` pragma, so we compile it exactly as written.
    "experimental",
    // Lets the runtime expectations call functions that don't exist, which only
    // concerns the expectations — and we ignore those entirely.
    "allowNonExistingFunctions",
];

/// Rejects any setting that isn't one of the two above.
fn check_settings_are_known(settings: &[(&str, &str)]) -> Result<()> {
    for (key, _) in settings {
        if !ALLOWED_SETTINGS.contains(key) {
            bail!(
                "unrecognized isoltest setting {key:?}. If it cannot affect whether the test \
                 compiles, add it to 'ALLOWED_SETTINGS'."
            );
        }
    }

    Ok(())
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

    /// A setting we've never seen has to be looked at, rather than ignored on
    /// the assumption that it doesn't matter.
    #[test]
    fn rejects_settings_it_does_not_know() {
        let check = |line: &str| {
            let contents = format!("contract C {{}}\n// ====\n{line}\n");
            let (_, settings) = split_trailer(&contents);

            check_settings_are_known(&settings)
        };

        assert!(check("// EVMVersion: >=byzantium").is_ok());
        assert!(check("// compileViaYul: also").is_ok());
        assert!(check("// nonesuch: true").is_err());

        // Casing is part of the name: a near-miss is still a setting we've
        // never looked at.
        assert!(check("// evmVersion: >=byzantium").is_err());
    }

    #[test]
    fn source_without_trailer_is_all_source() {
        let contents = "pragma solidity >=0.4.0;\ncontract C {}\n";
        let (source, settings) = split_trailer(contents);
        assert_eq!(source, contents);
        assert!(settings.is_empty());
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
