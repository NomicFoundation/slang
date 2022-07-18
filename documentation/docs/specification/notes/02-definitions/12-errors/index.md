<!-- markdownlint-configure-file { "first-line-heading": { "level": 2 } } -->

## Errors

Errors allow you to define descriptive names and data for failure situations. Errors can be used in revert statements.
In comparison to string descriptions, errors are much cheaper and allow you to encode additional data. You can use NatSpec to describe the error to the user.
They can also be defined inside or outside contracts:

```solidity
contract Token {
    error NotEnoughFunds(uint requested, uint available);

    function transfer(address to, uint amount) public {
        uint balance = balances[msg.sender];
        if (balance < amount)
            revert NotEnoughFunds(amount, balance);

        // Continue with the transfer...
    }
}
```

--8<-- "snippets/under-construction.md"
