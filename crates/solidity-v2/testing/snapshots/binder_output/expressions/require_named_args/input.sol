pragma solidity *;

contract Test {
    error Failure(
        uint severity,
        string cause
    );

    function test(bool ok) public {
        require(ok, Failure({severity: 100, cause: "Testing"}));
    }
}
