contract Sample {}

interface ISample {}

contract Test {
    enum Direction { North, South, West, East }

        function test() public {
        string memory v1 = type(Sample).name;
        bytes memory v2 = type(Sample).creationCode;
        bytes memory v3 = type(Sample).runtimeCode;
        bytes4 v4 = type(ISample).interfaceId;
        Direction v5 = type(Direction).min;
        Direction v6 = type(Direction).max;
        uint v7 = type(uint).min;
        uint v8 = type(uint).max;
    }
}
