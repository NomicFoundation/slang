interface Example {
    //    ^def:1
    enum Choice { One, Two }
    //   ^def:2
    //                 ^def:6
    //            ^def:5
    struct Book { string author; }
    //     ^def:3
    function calculate() returns (int);
    //       ^def:4
}

contract Test {
    Example.Choice choice = Example.Choice.One;
//  ^ref:1
    //      ^ref:2
    //             ^def:7
    //                      ^ref:1
    //                              ^ref:2
    //                                     ^ref:5
    Example.Book book;
//  ^ref:1
    //      ^ref:3

    function my_test() returns (int) {
        return Example.calculate();
        //             ^ref:!
        //     ^ref:1
    }
}
