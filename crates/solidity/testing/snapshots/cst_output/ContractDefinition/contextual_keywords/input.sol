contract Contract {
    error MyError();

    function deposit() public payable {
        uint256 error;
        uint256 from;
        uint256 revert;
        uint256 global;

        revert MyError();
    }
}
