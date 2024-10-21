contract Sample {}

interface ISample {}

library Utils {}

contract Test {
    enum Direction { North, South, West, East }

    function test() public {
        string memory v1 = type(Sample).name;
        bytes memory v2 = type(Sample).creationCode;
        bytes memory v3 = type(Sample).runtimeCode;

        string memory v4 = type(ISample).name;
        bytes4 v5 = type(ISample).interfaceId;

        Direction v6 = type(Direction).min;
        Direction v7 = type(Direction).max;

        uint v8 = type(uint).min;
        uint v9 = type(uint).max;

        string memory v10 = type(Utils).name;
        bytes memory v11 = type(Utils).creationCode;
        bytes memory v12 = type(Utils).runtimeCode;
    }
}
