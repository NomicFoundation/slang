contract Base {}
contract Derived is Base {}

contract C {
  function f(
    uint8 small,
    uint256 big,
    address a,
    address payable ap,
    Base base,
    Derived der,
    bytes20 b20,
    bytes32 b32
  ) public pure {
    small == big;
    a == ap;
    base == der;
    uint256(5) == 5;
    bytes32(0) == 0;
    small == small;
    b20 == b32;
  }
}
