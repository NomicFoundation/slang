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
    function get_first_choice() public returns (First.Choice) {
        // Cannot access a member function through the contract type
        return First.get_choice();
        //     ^ref:1
        //           ^ref:!
    }
    function other_choice() public returns (First.Choice) {
        // Cannot access a state variable in another contract
        return First.choice;
        //     ^ref:1
        //           ^ref:!
    }
}
