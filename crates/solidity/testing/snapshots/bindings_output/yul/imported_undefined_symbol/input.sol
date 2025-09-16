import { slot } from "./slots.sol";  // even if undefined, the local def should have 2 references

contract Foo {
    function useSlotInAssembly() public {
        uint value;
        assembly {
            value := sload(slot)
        }
    }

    function useSlotInSolidity() public {
        uint value = slot;
    }
}
