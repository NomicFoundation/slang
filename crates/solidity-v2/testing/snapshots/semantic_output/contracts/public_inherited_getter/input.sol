contract RegistrarAccess {
    Root root;
    function test() public {
        root.controllers;
    }
}

contract Root is Controllable {}

contract Controllable {
    mapping (address => bool) public controllers;
}
