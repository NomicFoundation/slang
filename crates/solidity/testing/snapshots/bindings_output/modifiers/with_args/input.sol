contract FunctionModifier {
    address public owner;

    modifier validAddress(address _addr) {
        require(_addr != address(0), "Not valid address");
        _;
    }

    function changeOwner(address _newOwner)
        public
        validAddress(_newOwner)
    {
        owner = _newOwner;
    }
}
