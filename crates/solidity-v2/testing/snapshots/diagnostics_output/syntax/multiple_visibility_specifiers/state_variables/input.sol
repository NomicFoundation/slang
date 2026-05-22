contract Test {
  uint public private x = 1;
  uint private public y = 2;
  uint public private constant z = 3;
  uint internal public w = 4;
  uint public private internal v = 5;
  uint public constant immutable private a = 6;
  uint private immutable constant public b = 7;
  uint public private internal e = 10;
  uint public internal private f = 11;
  uint private internal public g = 12;
}
