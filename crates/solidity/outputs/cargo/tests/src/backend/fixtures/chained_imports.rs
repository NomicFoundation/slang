use std::rc::Rc;

use anyhow::Result;
use slang_solidity::compilation::CompilationUnit;
use slang_solidity::utils::LanguageFacts;

use crate::compilation::compilation_unit::build_compilation_unit_from_multi_part_file;

const CHAINED_SAMPLE: &str = r#"
// ---- path: first.sol
import {B2 as B1} from "second.sol";
interface I1 {}
contract A1 is I1, B1 {}

// ---- path: second.sol
import {B3 as B2} from "third.sol";

// ---- path: third.sol
contract B3 {}
"#;

pub(crate) struct ChainedImports {}

impl ChainedImports {
    pub(crate) fn build_compilation_unit() -> Result<Rc<CompilationUnit>> {
        build_compilation_unit_from_multi_part_file(&LanguageFacts::LATEST_VERSION, CHAINED_SAMPLE)
    }
}
