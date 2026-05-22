contract A {
  receive() payable {}
}

contract B {
  receive() external payable external {}
}

contract C {
  receive() external payable {}
}

contract D {
  receive() external {}
}

contract E {
  receive() payable payable {}
}

contract F {
  receive() {}
}