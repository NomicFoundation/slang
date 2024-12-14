contract Base1 {
    function base1() returns (int) { return 1; }
}

contract Base2 {
    function base2() returns (int) { return 2; }
}

contract Derived is
    Base1,
    Base2
{
    function test() returns (int) {
        return base1() + base2();
    }
}
