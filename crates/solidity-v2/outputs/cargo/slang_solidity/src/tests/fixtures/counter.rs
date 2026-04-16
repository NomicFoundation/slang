use crate::define_fixture;

define_fixture!(
    Counter,
    file: "main.sol", r#"
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
"#,
    file: "ownable.sol", r#"
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
"#,
    file: "activatable.sol", r#"
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
"#,
);
