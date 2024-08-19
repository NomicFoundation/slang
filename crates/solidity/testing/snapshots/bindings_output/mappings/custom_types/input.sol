contract Mappings {
    enum Direction { North, East, South, West }
    enum Kind { Zombie, Bat, Skeleton, Dragon }
    struct Monster {
        Kind kind;
        uint life;
    }

    mapping(Direction => Monster) monsters;

    function spawn(Direction _dir, Kind _kind) public {
        monsters[_dir] = Monster(_kind, 100);
    }

    function attack(Direction _dir, uint _power) public {
        if (monsters[_dir].life > _power) {
            monsters[_dir].life -= _power;
        } else {
            delete monsters[_dir];
        }
    }

    function get_type(Direction _dir) public returns (Kind) {
        return monsters[_dir].kind;
    }
}
