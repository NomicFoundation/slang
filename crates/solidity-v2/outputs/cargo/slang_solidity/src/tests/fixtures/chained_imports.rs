use crate::define_fixture;

define_fixture!(
    ChainedImports,
    file: "first.sol", r#"
import {B2 as B1} from "second.sol";
interface I1 {}
contract A1 is I1, B1 {}
"#,
    file: "second.sol", r#"
import {B3 as B2} from "third.sol";
"#,
    file: "third.sol", r#"
contract B3 {}
"#,
);
