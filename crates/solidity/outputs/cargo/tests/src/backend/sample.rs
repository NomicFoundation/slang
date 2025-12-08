use std::rc::Rc;

use anyhow::Result;
use slang_solidity::compilation::{CompilationBuilder, CompilationBuilderConfig, CompilationUnit};
use slang_solidity::utils::LanguageFacts;

pub const MAIN_ID: &str = "main.sol";
pub const MAIN_SOL_CONTENTS: &str = r#"
// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.29;

import {Ownable} from "ownable.sol";
import {Activatable} from "activatable.sol";

contract Counter is Ownable, Activatable {
    uint public count;
    mapping (address => uint) _clickers;

    constructor(uint initial) {
        count = initial;
    }
    function increment(uint delta) public onlyOwner returns (uint) {
        count += delta;
        return count;
    }
    function click() public checkEnabled returns (uint) {
        count += 1;
        _clickers[msg.sender] += 1;
        return _clickers[msg.sender];
    }
}
"#;

pub const OWNABLE_ID: &str = "ownable.sol";
pub const OWNABLE_SOL_CONTENTS: &str = r#"
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

pub const ACTIVATABLE_ID: &str = "activatable.sol";
pub const ACTIVATABLE_SOL_CONTENTS: &str = r#"
// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.29;

import {Ownable} from "ownable.sol";

abstract contract Activatable is Ownable {
    enum State { DISABLED, ENABLED }

    State _state;

    constructor() {
        _state = State.DISABLED;
    }
    function enable() public onlyOwner {
        _state = State.ENABLED;
    }
    function disable() public onlyOwner {
        _state = State.DISABLED;
    }
    function isEnabled() public view returns (bool) {
        return _state == State.ENABLED;
    }
    modifier checkEnabled() {
        require(_state == State.ENABLED, "Contract is disabled");
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
            ACTIVATABLE_ID => Ok(Some(ACTIVATABLE_SOL_CONTENTS.to_owned())),
            _ => Ok(None),
        }
    }

    fn resolve_import(
        &mut self,
        _source_file_id: &str,
        import_path_cursor: &slang_solidity::cst::Cursor,
    ) -> Result<Option<String>> {
        match import_path_cursor.node().unparse().as_str() {
            "\"ownable.sol\"" => Ok(Some(OWNABLE_ID.to_owned())),
            "\"activatable.sol\"" => Ok(Some(ACTIVATABLE_ID.to_owned())),
            _ => Ok(None),
        }
    }
}

pub fn build_compilation_unit() -> Result<Rc<CompilationUnit>> {
    let mut builder = CompilationBuilder::create(LanguageFacts::LATEST_VERSION, Config {})?;

    builder.add_file(MAIN_ID)?;

    Ok(Rc::new(builder.build()))
}
