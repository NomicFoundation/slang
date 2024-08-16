interface DataFeed { function getData(address token) external returns (uint value); }

contract FeedConsumer {
    DataFeed feed;
    //       ^def:1
    uint errorCount;
    //   ^def:2
    function rate(address token) public returns (uint value, bool success) {
        //                                                        ^def:5
        //                                            ^def:4
        //                ^def:3
        try feed.getData(token) returns (uint v) {
            //                                ^def:6
            //           ^ref:3
            //<ref:1
            value = v;
            //<ref:4
            //      ^ref:6
            success = true;
            //<ref:5
        } catch {
            errorCount++;
            //<ref:2

            return (0, false);
        }
    }
}
