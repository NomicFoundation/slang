interface IFoo {
    struct AStruct {
        address token;
    }

    function doFoo(AStruct memory aStruct, string calldata witnessString) external;
    function doFoo(AStruct[] memory someStructs, string calldata witnessString) external;
}

contract Test {
    IFoo myFoo;
    string myWitness;

    function test() internal
    {
        IFoo.AStruct[] memory someStructs;

        myFoo.doFoo({
            someStructs: someStructs,
            witnessString: myWitness
        });
    }
}
