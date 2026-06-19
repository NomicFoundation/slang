interface I {
    function i() external;
}

contract C {
    function ext() external {}
    function pub() public {}
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
