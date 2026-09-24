// SPDX-License-Identifier: MIT
pragma solidity *;

// Accepted: every callee shape that is externally callable.

interface I {
    function f(bytes calldata data) external returns (uint256);
}

contract Other {
    function externalFn(uint256 x) external pure returns (uint256) {
        return x;
    }

    function publicFn(uint256 x) public pure returns (uint256) {
        return x;
    }
}

contract C {
    function g(bytes calldata data) external pure returns (uint256) {
        return data.length;
    }

    function externalFn(bytes calldata data) external pure returns (uint256) {
        return data.length;
    }

    function encode(function(bytes calldata) external returns (uint256) p, I i, bytes calldata data)
        external
        view
        returns (
            bytes memory a,
            bytes memory b,
            bytes memory c,
            bytes memory d,
            bytes memory e,
            bytes memory f,
            bytes memory h
        )
    {
        a = abi.encodeCall(I.f, (data));
        b = abi.encodeCall(Other.externalFn, (1));
        c = abi.encodeCall(Other.publicFn, (1));
        d = abi.encodeCall(this.g, (data));
        e = abi.encodeCall(i.f, (data));
        f = abi.encodeCall(p, (data));
        h = abi.encodeCall(C.externalFn, (data));
    }
}

contract Base {
    function externalFn(bytes calldata data) external pure returns (uint256) {
        return data.length;
    }
}

contract Derived is Base {
    function encode(bytes calldata data) external pure returns (bytes memory a) {
        a = abi.encodeCall(Base.externalFn, (data));
    }
}
