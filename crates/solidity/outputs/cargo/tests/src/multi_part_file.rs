use regex::Regex;

/// Gets the parts in a multi-file, each separated by a path designation. If the
/// contents input string does not contain any separator, a single part is
/// returned for the path "input.sol".
///
/// The separators are Solidity comments of the form:
///   // --- path: lib/foo.sol
///
/// Any number of dashes greater than or equal to 3 is accepted. In the above
/// case, the returned vector will contain a single entry with the contents
/// below the separator line.
///
/// Non-whitespace content is not allowed before the first separator.
///
pub fn split_multi_file(contents: &str) -> Vec<(&str, &str)> {
    let separator_re = Regex::new("(?m)^// -{3,} path: (.+)\\s*$").unwrap();
    let mut last_path: Option<&str> = None;
    let mut last_start = None;
    let mut parts = Vec::new();

    for captures in separator_re.captures_iter(contents) {
        let separator_match = captures.get(0).unwrap();
        let path_match = captures.get(1).unwrap();

        let end = separator_match.start();
        if let Some(start) = last_start {
            parts.push((last_path.unwrap(), &contents[start..end]));
        } else {
            let leading_content = &contents[..end];
            assert!(
                leading_content.trim().is_empty(),
                "leading content before first path separator is not allowed"
            );
        }

        last_start = Some(separator_match.end());
        last_path = Some(path_match.as_str());
    }

    if let Some(start) = last_start {
        parts.push((last_path.unwrap(), &contents[start..]));
    } else {
        parts.push(("input.sol", contents));
    }
    parts
}

#[test]
fn splits_a_multi_file() {
    let multi_file_contents = r#"
// ---- path: main.sol
import "lib/foo.sol" as foo;
import "lib/bar.sol" as bar;

// --- path: lib/foo.sol
contract Foo {}

// --- path: lib/bar.sol
contract Bar {}
"#;

    let parts = split_multi_file(multi_file_contents);
    assert_eq!(3, parts.len());
    assert_eq!(
        parts[0],
        (
            "main.sol",
            r#"
import "lib/foo.sol" as foo;
import "lib/bar.sol" as bar;

"#
        )
    );
    assert_eq!(
        parts[1],
        (
            "lib/foo.sol",
            r#"
contract Foo {}

"#
        )
    );
    assert_eq!(
        parts[2],
        (
            "lib/bar.sol",
            r#"
contract Bar {}
"#
        )
    );
}

#[test]
fn returns_single_part_for_non_multi_files() {
    let file_contents = r#"
contract Foo {}
"#;

    let parts = split_multi_file(file_contents);
    assert_eq!(1, parts.len());
    assert_eq!(parts[0].0, "input.sol");
    assert_eq!(parts[0].1, file_contents);
}

#[test]
#[should_panic(expected = "leading content before first path separator is not allowed")]
fn disallows_content_before_first_path_tag() {
    let file_contents = r#"
contract Bar {}

// ---- path: main.sol
import "lib/foo.sol" as foo;

// --- path: lib/foo.sol
contract Foo {}
"#;

    let _ = split_multi_file(file_contents);
}
