library Lib {
  error IndexOutOfBounds();

  modifier test() {
    revert IndexOutOfBounds();
  }
}
