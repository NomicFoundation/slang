use crate::ast_api::find_unused_definitions::find_unused_definitions;
use crate::ast_api::pipeline;

pub(crate) const MAIN_SOL_CONTENTS: &str = r#"
abstract contract Ownable {
  address _owner;
  constructor() {
    _owner = msg.sender;
  }
  modifier onlyOwner() {
    require(_owner == msg.sender);
    _;
  }
  function checkOwner(address addr) internal returns (bool) {
    return _owner == addr;
  }
}

contract Counter is Ownable {
  uint _count;
  uint _unused;
  constructor(uint initialCount) {
    _count = initialCount;
  }
  function count() public view returns (uint) {
    return _count;
  }
  function increment(uint delta, uint multiplier) public onlyOwner returns (uint) {
    require(delta > 0, "Delta must be positive");
    _count += delta;
    return _count;
  }
  function unusedDecrement() private {
    require(checkOwner(msg.sender));
    _count -= 1;
  }
}
"#;

#[test]
fn test_find_unused_definitions() {
    let unit = pipeline::one_file_backend_pipeline(MAIN_SOL_CONTENTS).unwrap();

    let unused = find_unused_definitions(&unit, "Counter");
    assert_eq_defs!(
        unused,
        vec!["checkOwner", "_unused", "multiplier", "unusedDecrement",]
    );
}
