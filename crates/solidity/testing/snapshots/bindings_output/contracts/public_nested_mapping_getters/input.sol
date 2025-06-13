contract Test {
    function test(TokenState tokenState) public {
        tokenState.nested_maps(1, 1).balance;
        tokenState.array_in_map(1, 1).balance;
        tokenState.map_in_array(1, 1).balance;
    }
}
contract TokenState {
    mapping(uint => mapping(uint => address)) public nested_maps;
    mapping(uint => address[]) public array_in_map;
    mapping(uint => address)[] public map_in_array;
}
