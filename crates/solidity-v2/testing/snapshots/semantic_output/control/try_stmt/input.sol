interface DataFeed { function getData(address token) external returns (uint value); }

contract FeedConsumer {
    DataFeed feed;
    uint errorCount;
    uint lastValue;
    function rate(address token) public returns (uint value, bool success) {
        string memory last_reason;
        try feed.getData(token) returns (uint v) {
            lastValue = v;
            return (v, true);
        } catch Error(string memory reason) {
            last_reason = reason;
            errorCount++;
            return (0, false);
        } catch {
            errorCount++;
            return (0, false);
        }
    }
}
