contract Test {
    struct Coords {
        // ^def:1
        int x;
        int y;
    }
    enum  HitTest {
        //^def:2
        Inside,
        Outside
    }
    struct Box {
        // ^def:3
        function (Coords memory) external returns (HitTest) hitTest;
        //                                                  ^def:4
        //                                         ^ref:2
        //        ^ref:1
    }
    Box[] boxes;
    //    ^def:5
    //<ref:3

    function insideBoxes(Coords memory p) public returns (bool) {
        for (uint i = 0; i < boxes.length; i++) {
            if (boxes[i].hitTest(p) == HitTest.Inside) return true;
            //                         ^ref:2
            //           ^ref:4
            //  ^ref:5
        }
        return false;
    }
}
