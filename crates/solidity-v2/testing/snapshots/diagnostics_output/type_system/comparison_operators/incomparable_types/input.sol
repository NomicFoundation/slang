contract C {
  enum E1 { A }
  enum E2 { B }

  function equality(
    uint256 u,
    int256 i,
    address a,
    bool b,
    E1 e1,
    E2 e2,
    string memory sa,
    string memory sb
  ) public view {
    u == b; 
    u == i;
    u == a;
    e1 == e2;
    (this.f) == u;
    sa == sb;
    u != b;
    u != a;
  }
  
    function inequality(
    uint256 u,
    int256 i,
    address a,
    bool b,
    E1 e1,
    E2 e2,
    string memory sa,
    string memory sb
  ) public view {
    u > b; 
    u > i;
    u > a;
    e1 > e2;
    (this.f) > u;
    sa > sb;
    u < b;
    u < a;
    u <= b;
    u <= a;
    u >= b;
    u >= a;
  }
}
