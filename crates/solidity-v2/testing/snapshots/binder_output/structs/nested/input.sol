struct Base {
  int x;
}

library MyLib {
  enum Direction { North, South, West, East }

  struct LibStruct {
    Base base;
    int y;
  }
}

interface OneInterface {
  struct IfaceStruct {
    MyLib.LibStruct lib_struct;
    int z;
  }
}

contract Sample {
  struct SampleStruct {
    OneInterface.IfaceStruct iface_struct;
    MyLib.Direction direction;
    int w;
  }

  function hello() {
    SampleStruct memory s;
    s.iface_struct.lib_struct.base.x = 1;
    s.iface_struct.lib_struct.y = 2;
    s.iface_struct.z = 3;
    s.w = 4;
    s.direction = MyLib.Direction.North;

    MyLib.LibStruct memory ls;
    ls.base.x = s.iface_struct.lib_struct.base.x;
  }
}

