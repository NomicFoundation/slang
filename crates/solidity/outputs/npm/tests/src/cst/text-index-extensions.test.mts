import { TextIndexExtensions } from "@nomicfoundation/slang/cst";

it("zero()", () => {
  const zero = TextIndexExtensions.zero();

  expect(zero).toEqual({
    utf8: 0,
    utf16: 0,
    line: 0,
    column: 0,
  });
});
