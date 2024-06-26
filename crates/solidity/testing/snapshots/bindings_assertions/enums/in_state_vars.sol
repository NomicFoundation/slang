contract Foo {
    enum Answer { Yes, No }
    //                 ^def:3
    //            ^def:2
    //   ^def:1
    Answer choice = Answer.Yes;
//  ^ref:1
    //                     ^ref:2
    //              ^ref:1
}
