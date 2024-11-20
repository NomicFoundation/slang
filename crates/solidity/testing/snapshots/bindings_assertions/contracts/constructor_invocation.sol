contract A {
    //   ^def:1
    function A(int x) {}
    //             ^def:2

    constructor(int y) {}
    //              ^def:3
}

contract Test {
    function foo() public {
        new A({x: 2});
        //  ^ref:1
        //     ^ref:2 (< 0.5.0)
        //     ^ref:! (>= 0.5.0)
        new A({y: 2});
        //  ^ref:1
        //     ^ref:3 (>= 0.4.22)
        //     ^ref:! (< 0.4.22)
    }
}
