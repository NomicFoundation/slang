library Utils {
    function foo(bytes memory x) internal {}
    function foo(bytes1 x) internal {}
}

contract Test {
    using Utils for bytes;
    using Utils for bytes1;

    function test(bytes calldata name) public {
        bytes(name[0:16]).foo();
        name[0].foo();
    }
}
