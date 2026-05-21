error MyCustomError(uint, bool);

contract C {
    function f() pure public {
        MyCustomError == MyCustomError;
        MyCustomError != MyCustomError;
        MyCustomError >= MyCustomError;
        MyCustomError <= MyCustomError;
        MyCustomError < MyCustomError;
        MyCustomError > MyCustomError;
    }
}
