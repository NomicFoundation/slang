interface ICounter {
    // returns the current count
    function count() external view returns (uint);

    // increments the counter
    function increment() external;
}