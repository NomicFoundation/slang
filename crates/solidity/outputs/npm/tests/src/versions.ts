import test from "ava";
import { Language } from "@nomicfoundation/slang/language";

test("list supported versions", (t) => {
  const versions = Language.supportedVersions();

  t.true(versions.length > 0);
  t.true(versions.includes("0.4.11"));
  t.false(versions.includes("0.0.0"));
});
