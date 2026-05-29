contract C {
    event MyCustomEvent(uint);
    function f() pure public {
        MyCustomEvent == MyCustomEvent;
        MyCustomEvent != MyCustomEvent;
        MyCustomEvent >= MyCustomEvent;
        MyCustomEvent <= MyCustomEvent;
        MyCustomEvent < MyCustomEvent;
        MyCustomEvent > MyCustomEvent;
    }
}
