contract Test {
    enum Direction { North, South, East, West }
    //   ^def:1

    function getLargestValue() returns (Direction) {
        //                              ^ref:1
        return type(Direction).max;
        //          ^ref:1 (>=0.5.3)
    }

    function getSmallestValue() returns (Direction) {
        //                               ^ref:1
        return type(Direction).min;
        //          ^ref:1 (>=0.5.3)
    }
}
