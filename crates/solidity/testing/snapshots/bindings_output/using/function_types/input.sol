using {invoke} for function(uint);

function invoke(function(uint) x) {
    x(1);
}

function foo(uint x) {}

function test() {
    foo.invoke();

    function (uint) bar = foo;
    bar.invoke();
}
