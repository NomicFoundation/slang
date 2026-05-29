contract D {}
contract E {}

contract C is D {
    C c;
    D d;
    E e;

    function foo() public {
        // OK
        c == this;
        d == this;

        // Not Ok
        int32(1) != this;
        e == this;
    }
}
