contract Test {
  constructor() public internal {}

  constructor() internal public {}

  constructor() public internal payable {}

  constructor() public payable internal {}

  constructor() internal payable public {}

  constructor() public internal public {}

  constructor() internal public internal {}
}
