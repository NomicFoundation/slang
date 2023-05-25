--8<-- "crates/solidity/inputs/language/snippets/under-construction.md"

## Function Modifiers

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
