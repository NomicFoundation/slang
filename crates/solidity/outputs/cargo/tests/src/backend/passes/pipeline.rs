use std::rc::Rc;

use anyhow::Result;
use slang_solidity::compilation::{CompilationBuilder, CompilationBuilderConfig, CompilationUnit};
use slang_solidity::utils::LanguageFacts;

const MAIN_ID: &str = "MAIN-ID";
const MAIN_SOL_CONTENTS: &str = r#"
// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.29;

import {Ownable} from "ownable.sol";

contract Counter is Ownable {
    enum State { DISABLED, ENABLED }

    State _state;
    uint _count;
    mapping (address => uint) _clickers;

    constructor(uint initial) Ownable() {
        _count = initial;
        _state = State.DISABLED;
    }
    function count() public view returns (uint) {
        return _count;
    }
    function increment(uint delta) public onlyOwner returns (uint) {
        _count += delta;
        return _count;
    }
    function enable() public onlyOwner {
        _state = State.ENABLED;
    }
    function disable() public onlyOwner {
        _state = State.DISABLED;
    }
    function click() public returns (uint) {
        require(_state == State.ENABLED, "counter is disabled");
        _count += 1;
        _clickers[msg.sender] += 1;
        return _clickers[msg.sender];
    }
}
"#;

const OWNABLE_ID: &str = "ONWABLE-ID";
const OWNABLE_SOL_CONTENTS: &str = r#"
// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.29;

abstract contract Ownable {
    address _owner;
    constructor() {
        _owner = msg.sender;
    }
    modifier onlyOwner() {
        require(msg.sender == _owner, "Only owner allowed");
        _;
    }
}
"#;

struct Config {}

impl CompilationBuilderConfig for Config {
    type Error = anyhow::Error;

    fn read_file(&mut self, file_id: &str) -> Result<Option<String>> {
        match file_id {
            MAIN_ID => Ok(Some(MAIN_SOL_CONTENTS.to_owned())),
            OWNABLE_ID => Ok(Some(OWNABLE_SOL_CONTENTS.to_owned())),
            _ => Ok(None),
        }
    }

    fn resolve_import(
        &mut self,
        source_file_id: &str,
        import_path_cursor: &slang_solidity::cst::Cursor,
    ) -> Result<Option<String>> {
        assert_eq!(source_file_id, MAIN_ID);
        assert_eq!(import_path_cursor.node().unparse(), "\"ownable.sol\"");
        Ok(Some(OWNABLE_ID.to_owned()))
    }
}

fn build_compilation_unit() -> Result<Rc<CompilationUnit>> {
    let mut builder = CompilationBuilder::create(LanguageFacts::LATEST_VERSION, Config {})?;

    builder.add_file(MAIN_ID)?;

    Ok(Rc::new(builder.build()))
}

#[test]
fn test_backend_pipeline() -> Result<()> {
    let unit = build_compilation_unit()?;
    let semantic_analysis = unit.semantic_analysis();
    assert_eq!(2, semantic_analysis.files().len());

    Ok(())
}
