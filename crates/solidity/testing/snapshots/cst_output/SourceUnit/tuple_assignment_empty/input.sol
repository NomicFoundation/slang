library SafeMath {
  function tryAdd() internal pure  {
    // If we don't use the return, V1 will parse them as TupleDeconstructionStatement
    return () = foo;
    return (,) = foo;
    return (,,) = foo;

    () = foo;
    (,) = foo;
    (,,) = foo;
  }
}
