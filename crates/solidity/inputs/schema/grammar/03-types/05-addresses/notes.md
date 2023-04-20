--8<-- "crates/solidity/inputs/schema/snippets/under-construction.md"

## Address Types

The address type comes in two flavours, which are largely identical:

-   `address`: Holds a 20 byte value (size of an Ethereum address).
-   `address payable`: Same as `address`, but with the additional members `transfer` and `send`.

Hexadecimal literals that pass the [address checksum test](https://github.com/ethereum/EIPs/blob/master/EIPS/eip-55.md),
for example `0xdCad3a6d3569DF655070DEd06cb7A1b2Ccd1D3AF` are of `address` type.
Hexadecimal literals that are between 39 and 41 digits long and do not pass the checksum test produce an error.
You can prepend (for `int` types) or append (for `bytesNN` types) zeros to remove the error.
