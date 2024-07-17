interface Example {
    enum Choice { One, Two }
    struct Book { string author; }
    function calculate() returns (Choice);
    function get_book() returns (Book);
}

contract Test {
    Example.Choice choice = Example.Choice.One;
    Example.Book book;
}
