contract S {
    function f() public {}
}

struct S {
    uint u;
}

contract Test {
    error dup(uint code);

    function dup(bool flag) public {}

    function test(S memory s) public {
        s.u;
        s.f;
        dup(true);
        revert dup(1);
    }
}
