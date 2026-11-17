pragma solidity *;

// Error parameters are not visible to the parameter type names.
contract C {
    enum EnumType {A, B, C}

    struct StructType {
        uint x;
    }

    error E1(StructType StructType);
    error E2(EnumType StructType, StructType EnumType);
}
