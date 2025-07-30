use anyhow::Result;
use slang_solidity::backend::passes;
use slang_solidity::compilation::{CompilationUnit, InternalCompilationBuilder};
use slang_solidity::utils::LanguageFacts;

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

fn build_compilation_unit() -> Result<CompilationUnit> {
    let mut internal_builder = InternalCompilationBuilder::create(LanguageFacts::LATEST_VERSION)?;

    let main_id = "MAIN-ID";
    let ownable_id = "OWNABLE-ID";

    let main_add_file_response = internal_builder.add_file(main_id.to_string(), MAIN_SOL_CONTENTS);
    internal_builder.resolve_import(
        main_id,
        &main_add_file_response.import_paths[0],
        ownable_id.to_string(),
    )?;

    let _ = internal_builder.add_file(ownable_id.to_string(), OWNABLE_SOL_CONTENTS);

    Ok(internal_builder.build())
}

#[test]
fn test_backend_pipeline() -> Result<()> {
    let unit = build_compilation_unit()?;
    let data = passes::p0_build_ast::run(unit);
    let data = passes::p1_flatten_contracts::run(data);
    let data = passes::p2_collect_definitions::run(data);
    let data = passes::p3_type_definitions::run(data);
    let data = passes::p4_resolve_references::run(data);
    assert_eq!(2, data.files.len());

    Ok(())
}
