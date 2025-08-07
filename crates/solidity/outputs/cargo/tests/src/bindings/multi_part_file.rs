use indoc::indoc;
use regex::Regex;

#[derive(Debug, PartialEq)]
pub(crate) struct Part<'a> {
    pub(crate) name: &'a str,
    pub(crate) contents: &'a str,
}

#[derive(Debug, PartialEq)]
pub(crate) struct MultiPart<'a> {
    pub(crate) parts: Vec<Part<'a>>,
}

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
pub fn split_multi_file(contents: &str) -> MultiPart<'_> {
    let separator_re = Regex::new(r"(?m)^// -{3,} path: (.+)\s*\n").unwrap();
    let mut last_path: Option<&str> = None;
    let mut last_start = None;
    let mut parts = Vec::new();

    for captures in separator_re.captures_iter(contents) {
        let separator_match = captures.get(0).unwrap();
        let path_match = captures.get(1).unwrap();

        let end = separator_match.start();
        if let Some(start) = last_start {
            parts.push(Part {
                name: last_path.unwrap(),
                contents: &contents[start..end],
            });
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

    let last_part = match last_start {
        Some(start) => Part {
            name: last_path.unwrap(),
            contents: &contents[start..],
        },
        None => Part {
            name: "input.sol",
            contents,
        },
    };
    parts.push(last_part);

    MultiPart { parts }
}

#[test]
fn splits_a_multi_file() {
    let multi_file_contents = indoc! {r#"
        // ---- path: main.sol
        import "lib/foo.sol" as foo;
        import "lib/bar.sol" as bar;

        // --- path: lib/foo.sol
        contract Foo {}

        // --- path: lib/bar.sol
        contract Bar {}
    "#};

    let result = split_multi_file(multi_file_contents);
    assert_eq!(3, result.parts.len());
    assert_eq!(
        result.parts[0],
        Part {
            name: "main.sol",
            contents: indoc! {r#"
                import "lib/foo.sol" as foo;
                import "lib/bar.sol" as bar;

            "#}
        }
    );
    assert_eq!(
        result.parts[1],
        Part {
            name: "lib/foo.sol",
            contents: indoc! {r#"
                contract Foo {}

            "#}
        }
    );
    assert_eq!(
        result.parts[2],
        Part {
            name: "lib/bar.sol",
            contents: indoc! {r#"
                contract Bar {}
            "#}
        }
    );
}

#[test]
fn returns_single_part_for_non_multi_files() {
    let file_contents = indoc! {r#"
        contract Foo {}
    "#};

    let parts = split_multi_file(file_contents).parts;
    assert_eq!(1, parts.len());
    assert_eq!(parts[0].name, "input.sol");
    assert_eq!(parts[0].contents, file_contents);
}

#[test]
#[should_panic(expected = "leading content before first path separator is not allowed")]
fn disallows_content_before_first_path_tag() {
    let file_contents = indoc! {r#"
        contract Bar {}

        // ---- path: main.sol
        import "lib/foo.sol" as foo;

        // --- path: lib/foo.sol
        contract Foo {}
    "#};

    let _ = split_multi_file(file_contents);
}
