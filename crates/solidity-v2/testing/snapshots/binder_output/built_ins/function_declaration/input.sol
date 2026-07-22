interface I {
    function i() external;
}

contract C {
    function ext() external {}
    function pub() public {}
    function tC() internal {
        this.ext.selector; // ok
        this.ext.address; // ok (external function value)
    }
}
contract D is C {
    function tD() internal {
        C.ext.selector; // ok
        C.ext.address; // not ok
    }
}
contract E {
    function test() internal {
        I.i.selector; // ok
        I.i.address; // not ok

        C.pub.selector; // ok
        C.pub.address; // not ok
    }
}
