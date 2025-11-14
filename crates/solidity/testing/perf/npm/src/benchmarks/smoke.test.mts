import { expect } from "@jest/globals";
import { SolcSubject } from "./subjects/solc.subject.mjs";
import { SolidityProject, Subject, Timings } from "./common.mjs";
import { SlangSubject } from "./subjects/slang.subject.mjs";
import { AntlrSubject } from "./subjects/solidity.parser.subject.mjs";

async function testSubject(subject: Subject): Promise<Timings> {
  let result = await subject.test(SolidityProject.mockProject(), "MockContract.sol");
  expect(result).toBeDefined();
  return result;
}

test("slang works", async () => {
  let subject = new SlangSubject();
  let result = await testSubject(subject);
  expect(result.get("slang_total")).toBeDefined();
}, 1000);

test("solc works", async () => {
  let subject = new SolcSubject();
  let result = await testSubject(subject);
  expect(result.get("solc_build_ast_duration")).toBeDefined();
}, 1000);

test("solidity-parser works", async () => {
  let subject = new AntlrSubject();
  let result = await testSubject(subject);
  expect(result.get("antlr_build_ast_duration")).toBeDefined();
}, 1000);
