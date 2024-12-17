contract Test {
    struct Coords {
        int x;
        int y;
    }
    enum HitTest {
        Inside,
        Outside
    }
    struct Box {
        function (Coords memory) external returns (HitTest) hitTest;
    }

    function insideBox(Box memory b, Coords memory p) public returns (bool) {
        return b.hitTest(p) == HitTest.Inside;
    }
}
