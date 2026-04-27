contract Test {
    function test() public {
        function() v1 = Test.test;
        function() v2 = test;
        function() v3 = Foo.bar;
    }
}

library Foo {
    function bar() internal {}
}
