--8<-- "crates/solidity/inputs/language/snippets/under-construction.md"

## Event Definitions

Events are convenient interfaces with the EVM logging facilities. They have to be defined inside a contract:

```solidity
contract MyContract {
    // Defining an event
    event BidPlacedEvent(address bidder, uint amount);

    function bid() public payable {
        // Triggering an event
        emit BidPlacedEvent(msg.sender, msg.value);
    }
}
```
