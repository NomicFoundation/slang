# Functions

Functions are the executable units of code. Functions are usually defined inside a contract, but they can also be defined outside of contracts.

```solidity
contract MyContract {
    function contractFunction() public {
        // Inside the contract
    }
}

function helperFunction() {
    // Outside the contract
}
```

Functions can be overloaded, where multiple functions with the same name, but with different parameters, can co-exist.

## Modifiers

Function modifiers can be used to amend the semantics of functions in a declarative way:

```solidity
contract MyContract {
    modifier onlySeller() {
        require(msg.sender == seller, "Only seller can call this.");
        _; // Function body will be inserted here
    }

    function myFunction() public view onlySeller {
        // Code here will be executed after `onlySeller` is executed.
    }
}
```

Unlike functions, modifiers cannot be overloaded.
