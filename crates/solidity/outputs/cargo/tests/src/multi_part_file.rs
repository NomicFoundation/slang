use indoc::indoc;
use regex::Regex;

#[derive(Debug, PartialEq)]
pub(crate) struct Part<'a> {
    pub(crate) name: &'a str,
    pub(crate) contents: &'a str,
}

#[derive(Debug, PartialEq)]
pub(crate) struct MultiPart<'a> {
    pub(crate) context: Option<&'a str>,
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
/// The leading section before the first path separator may contain context
/// information of the form:
///   // --- context: Derived
///
/// Other than that, non-whitespace content is not allowed before the first
/// path separator.
///
pub fn split_multi_file(contents: &str) -> MultiPart<'_> {
    let separator_re = Regex::new(r"(?m)^// -{3,} path: (.+)\s*\n").unwrap();
    let mut last_path: Option<&str> = None;
    let mut last_start = None;
    let mut parts = Vec::new();

    let context_re = Regex::new(r"(?m)\A// -{3,} context: (.+)\s*\n").unwrap();
    let (context, contents) = if let Some(context_capture) = context_re.captures(contents) {
        let context = context_capture.get(1).unwrap().as_str();
        (
            Some(context),
            &contents[context_capture.get(0).unwrap().end()..],
        )
    } else {
        (None, contents)
    };

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

    if let Some(start) = last_start {
        parts.push(Part {
            name: last_path.unwrap(),
            contents: &contents[start..],
        });
    } else {
        parts.push(Part {
            name: "input.sol",
            contents,
        });
    }

    MultiPart { context, parts }
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
    assert!(result.context.is_none());
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

#[test]
fn captures_context() {
    let file_contents = indoc! {r#"
        // --- context: Base
        contract Base {}
    "#};
    let result = split_multi_file(file_contents);
    assert_eq!(Some("Base"), result.context);
    assert_eq!(1, result.parts.len());
    assert_eq!("input.sol", result.parts[0].name);
    assert_eq!("contract Base {}\n", result.parts[0].contents);
}

#[test]
fn captures_context_and_split_a_multi_file() {
    let file_contents = indoc! {r#"
        // --- context: Derived
        // --- path: lib/base.sol
        contract Base {}
        // --- path: main.sol
        contract Derived is Base {}
    "#};
    let result = split_multi_file(file_contents);
    assert_eq!(Some("Derived"), result.context);
    assert_eq!(2, result.parts.len());
    assert_eq!("lib/base.sol", result.parts[0].name);
    assert_eq!("contract Base {}\n", result.parts[0].contents);
    assert_eq!("main.sol", result.parts[1].name);
    assert_eq!("contract Derived is Base {}\n", result.parts[1].contents);
}
