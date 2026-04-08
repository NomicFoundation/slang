library Checkpoints {
    struct Trace224 {
        Checkpoint224[] _checkpoints;
    }

    struct Checkpoint224 {
        uint32 _key;
        uint224 _value;
    }

    struct Trace160 {
        Checkpoint160[] _checkpoints;
    }

    struct Checkpoint160 {
        uint96 _key;
        uint160 _value;
    }

    function _unsafeAccess(Checkpoint224[] storage) private returns (Checkpoint224 storage) {}
    function _unsafeAccess(Checkpoint160[] storage) private returns (Checkpoint160 storage) {}

    function test(Trace224 storage self) internal returns (uint224) {
        _unsafeAccess(self._checkpoints)._value;
    }
}
