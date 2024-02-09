## Struct Types

Structs are custom defined types that can group several variables. They can be defined inside or outside contracts.

```solidity
struct Voter {
    address delegate;
    uint vote;
}
```

You can also create new objects of this struct using the following syntax:

```solidity
contract MyContract {
    function create() public  {
        Voter memory v = Voter({
            delegate: msg.sender,
            vote: 1
        });
    }
}
```

--8<-- "crates/solidity/inputs/language/snippets/under-construction.md"
