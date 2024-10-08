import { TerminalKindExtensions, TerminalKind } from "@slang-private/testlang-npm-package/cst";

describe("is_trivia()", () => {
  for (const kind in TerminalKind) {
    it(`TerminalKind.${kind}`, () => {
      switch (kind) {
        case TerminalKind.EndOfLine:
        case TerminalKind.MultiLineComment:
        case TerminalKind.SingleLineComment:
        case TerminalKind.Whitespace: {
          expect(TerminalKindExtensions.isTrivia(kind)).toBeTruthy();
          break;
        }
        case TerminalKind.Bang:
        case TerminalKind.CloseBracket:
        case TerminalKind.DelimitedIdentifier:
        case TerminalKind.Identifier:
        case TerminalKind.Missing:
        case TerminalKind.OpenBracket:
        case TerminalKind.Period:
        case TerminalKind.Plus:
        case TerminalKind.Semicolon:
        case TerminalKind.StringLiteral:
        case TerminalKind.TreeKeyword:
        case TerminalKind.Unrecognized: {
          expect(TerminalKindExtensions.isTrivia(kind)).toBeFalsy();
          break;
        }
        default: {
          throw new Error(`Unexpected terminal kind: ${kind}`);
        }
      }
    });
  }
});

describe("is_valid()", () => {
  for (const kind in TerminalKind) {
    it(`TerminalKind.${kind}`, () => {
      switch (kind) {
        case TerminalKind.Bang:
        case TerminalKind.CloseBracket:
        case TerminalKind.DelimitedIdentifier:
        case TerminalKind.EndOfLine:
        case TerminalKind.Identifier:
        case TerminalKind.MultiLineComment:
        case TerminalKind.OpenBracket:
        case TerminalKind.Period:
        case TerminalKind.Plus:
        case TerminalKind.Semicolon:
        case TerminalKind.SingleLineComment:
        case TerminalKind.StringLiteral:
        case TerminalKind.TreeKeyword:
        case TerminalKind.Whitespace: {
          expect(TerminalKindExtensions.isValid(kind)).toBeTruthy();
          break;
        }
        case TerminalKind.Missing:
        case TerminalKind.Unrecognized: {
          expect(TerminalKindExtensions.isValid(kind)).toBeFalsy();
          break;
        }
        default: {
          throw new Error(`Unexpected terminal kind: ${kind}`);
        }
      }
    });
  }
});
