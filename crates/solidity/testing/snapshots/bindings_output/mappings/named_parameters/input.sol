contract NamedMapping {
  mapping(address name => uint256) public justNameInKey;

  mapping(address => uint256 amount) public justNameInValue;

  mapping(address name => uint256 amount) public nameInBoth;
}
