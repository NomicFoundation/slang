library MyLib {
    enum Choice { One, Two }
    struct Book { string author; }

    function build_book(string memory author) public returns (Book memory) {
        return Book(author);
    }

    function favorite_choice() public returns (Choice) {
        return Choice.One;
    }
}

contract UsingLib {
    MyLib.Choice choice;
    MyLib.Book book;

    function test() public {
        book = MyLib.build_book("John Doe");
        choice = MyLib.favorite_choice();
    }
}
