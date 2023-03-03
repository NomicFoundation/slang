<!-- markdownlint-disable first-line-h1 -->

--8<-- "crates/solidity/inputs/schema/snippets/under-construction.md"

## Function Definitions

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
