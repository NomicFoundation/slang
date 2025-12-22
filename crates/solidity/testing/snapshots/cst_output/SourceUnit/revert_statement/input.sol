library SafeMath {
  function tryAdd() internal pure  {

    revert (1, 2);

    revert MyError(1, 2);

  // This is a variable declaration with type revert
    revert foo;
    revert foo = 34;
    revert.hello foo;
  }
}
