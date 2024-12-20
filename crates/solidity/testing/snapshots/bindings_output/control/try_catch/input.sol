contract Foo {}

contract Test {
    function test() public {
        try new Foo() {
        } catch Panic(uint code) {
            code;
        } catch Error(string memory message) {
            message;
        }
    }
}
