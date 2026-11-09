pragma solidity *;

// Event parameters are not visible to the parameter type names.
contract C {
    enum EnumType {A, B, C}

    struct StructType {
        uint x;
    }

    event E1(StructType StructType);
    event E2(EnumType StructType, StructType EnumType);
}
