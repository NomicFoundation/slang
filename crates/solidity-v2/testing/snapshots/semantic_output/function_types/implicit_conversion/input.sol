contract Test {
    function intern() internal {}
    function extern() external {}
    function privat() private {}
    function publi() public {}

    function callback(function() internal) private pure {}
    function callback(function() external) private pure {}

    function test() internal view {
        callback(intern);       // internal
        callback(this.extern);  // external
        callback(privat);       // internal
        callback(publi);        // internal
        callback(this.publi);   // external
    }
}
