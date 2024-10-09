contract First {
    //   ^def:1
    enum Choice { One, Two }
    //   ^def:2
    Choice choice;
    //<ref:2
    //     ^def:3
    function get_choice() public returns (Choice) {
        //   ^def:4
        return choice;
        //     ^ref:3
    }
    function internal_get_choice() private returns (Choice) {
        //   ^def:6
        return choice;
        //     ^ref:3
    }
}

contract Second {
    First.Choice choice;
    //<ref:1
    //    ^ref:2
    //           ^def:5

    function get_second_choice() public returns (First.Choice) {
        //                                             ^ref:2
        //                                       ^ref:1
        return choice;
        //     ^ref:5
    }
    function get_first_choice() public {
        First.Choice c = First.internal_get_choice();
        //               ^ref:1
        //                     ^ref:!  -- cannot access a private/internal function through the contract type
        bytes4 sel = First.get_choice.selector;
        //                 ^ref:4  -- we can reference the public function to get the selector
    }
    function other_choice() public returns (First.Choice) {
        return First.choice;
        //     ^ref:1
        //           ^ref:!  -- cannot access a state variable in another contract
    }
}
