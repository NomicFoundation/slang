use std::collections::HashMap;

use regex::Regex;

pub fn split_multi_file(contents: &str) -> HashMap<String, &str> {
    let separator_re = Regex::new("(?m)^// -{3,} path: (.+)\\s*$").unwrap();
    let mut last_path: Option<String> = None;
    let mut last_start = None;
    let mut parts = HashMap::new();
    for captures in separator_re.captures_iter(contents) {
        let separator_match = captures.get(0).unwrap();
        let path_match = captures.get(1).unwrap();

        if let Some(start) = last_start {
            let end = separator_match.start();
            parts.insert(last_path.unwrap(), &contents[start..end]);
        }
        last_start = Some(separator_match.end());
        last_path = Some(path_match.as_str().to_string());
    }
    if let Some(start) = last_start {
        parts.insert(last_path.unwrap(), &contents[start..]);
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
        parts.get("main.sol").unwrap(),
        &r#"
import "lib/foo.sol" as foo;
import "lib/bar.sol" as bar;

"#
    );
    assert_eq!(
        parts.get("lib/foo.sol").unwrap(),
        &r#"
contract Foo {}

"#
    );
    assert_eq!(
        parts.get("lib/bar.sol").unwrap(),
        &r#"
contract Bar {}
"#
    );
}
