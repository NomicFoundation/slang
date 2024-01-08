// Constructors introduced in 0.4.22 but `constructor` was usable as identifier until 0.5.0

contract Contract {
	constructor() {}
    function func() public {
		uint256 constructor;
    }
}
