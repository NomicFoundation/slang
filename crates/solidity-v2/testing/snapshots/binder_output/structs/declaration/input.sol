struct TopLevelStruct {
  int x;
}

library SomeLib {
  struct LibStruct {
    int y;
    TopLevelStruct top_level_struct;
  }
}

contract MyContract {
  struct ContractStruct {
    int z;
    SomeLib.LibStruct lib_struct;
  }
}

