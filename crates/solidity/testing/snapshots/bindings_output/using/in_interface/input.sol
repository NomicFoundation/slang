interface Bar {}

interface Foo {
    // these should not bind in >= 0.7.1
    using Utils for Bar;
}

library Utils {
}
