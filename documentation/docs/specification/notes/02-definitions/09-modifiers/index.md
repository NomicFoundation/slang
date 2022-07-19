<!-- markdownlint-configure-file { "first-line-heading": { "level": 2 } } -->

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

--8<-- "snippets/under-construction.md"
