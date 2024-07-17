contract First {
    enum Choice { One, Two }

    int x;
    Choice choice;

    function get_x() public returns (int) {
        return x;
    }

    function get_choice() public returns (Choice) {
        return choice;
    }
}

contract Second {
    First.Choice choice;

    function get_choice() public returns (First.Choice) {
        return choice;
    }
}
