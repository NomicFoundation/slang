import { TerminalKindExtensions, TerminalKind } from "@nomicfoundation/slang/cst";

describe("is_identifier()", () => {
  for (const kind in TerminalKind) {
    it(`TerminalKind.${kind}`, () => {
      switch (kind) {
        case TerminalKind.YulIdentifier:
        case TerminalKind.Identifier: {
          expect(TerminalKindExtensions.isIdentifier(kind)).toBeTruthy();
          break;
        }
        default: {
          expect(TerminalKindExtensions.isIdentifier(kind as TerminalKind)).toBeFalsy();
          break;
        }
      }
    });
  }
});

describe("is_trivia()", () => {
  for (const kind in TerminalKind) {
    it(`TerminalKind.${kind}`, () => {
      switch (kind) {
        case TerminalKind.EndOfLine:
        case TerminalKind.MultiLineComment:
        case TerminalKind.SingleLineComment:
        case TerminalKind.SingleLineNatSpecComment:
        case TerminalKind.MultiLineNatSpecComment:
        case TerminalKind.Whitespace: {
          expect(TerminalKindExtensions.isTrivia(kind)).toBeTruthy();
          break;
        }
        default: {
          expect(TerminalKindExtensions.isTrivia(kind as TerminalKind)).toBeFalsy();
          break;
        }
      }
    });
  }
});

describe("is_valid()", () => {
  for (const kind in TerminalKind) {
    it(`TerminalKind.${kind}`, () => {
      switch (kind) {
        case TerminalKind.Missing:
        case TerminalKind.Unrecognized: {
          expect(TerminalKindExtensions.isValid(kind)).toBeFalsy();
          break;
        }
        default: {
          expect(TerminalKindExtensions.isValid(kind as TerminalKind)).toBeTruthy();
          break;
        }
      }
    });
  }
});
