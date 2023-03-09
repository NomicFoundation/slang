<!-- markdownlint-disable first-line-h1 -->

--8<-- "crates/solidity/inputs/schema/snippets/under-construction.md"

## Enum Types

Enums can be used to create custom types with a finite set of constant values.
Enums can be declared on the file level, outside of contract or library definitions.

```solidity
enum ActionChoices {
    One,
    Two
}

contract MyContract {
    function choose() public pure returns (ActionChoices) {
        return ActionChoices.Two;
    }
}
```

Enums require at least one member, and its default value when declared is the first member.
Enums cannot have more than 256 members.
