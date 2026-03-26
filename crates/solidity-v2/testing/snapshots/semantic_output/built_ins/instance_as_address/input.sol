contract Test {
    function test() public {
        // This was valid before 0.5.0
        Foo foo;
        foo.balance;

        Bar bar;
        bar.balance;
    }
}

contract Foo {}
interface Bar {}
