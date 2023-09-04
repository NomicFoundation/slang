function bingo(
  uint256 gameId,
  uint8[][] memory cardNumbers,
  bytes memory signedGameLabel
) external override nonReentrant onlyOngoingGame(gameId) {
  _bingo(gameId, _msgSender(), cardNumbers, bytes(keyLabel(_userJoinedCounts(_msgSender()))), signedGameLabel);

  _afterGameWon(gameId);
  _logGameWon(_msgSender());
}
