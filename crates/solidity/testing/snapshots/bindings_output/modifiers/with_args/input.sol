contract FunctionModifier {
    address public owner;

    modifier validAddress(address _addr) {
        assert(_addr != address(0));
        _;
    }

    function changeOwner(address _newOwner)
        public
        validAddress(_newOwner)
    {
        owner = _newOwner;
    }
}
