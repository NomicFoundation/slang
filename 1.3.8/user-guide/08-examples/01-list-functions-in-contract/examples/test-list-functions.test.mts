import assert from "node:assert";
import { listFunctionsInContract } from "./list-functions-in-contract.mjs";
import buildSampleCompilationUnit from "../../common/sample-contract.mjs";

test("list functions by contract name", async () => {
  const unit = await buildSampleCompilationUnit();

  const functions = listFunctionsInContract(unit, "Counter");
  const functionNames = functions.map((fun) => fun.name.cst.unparse().trim());
  assert.deepEqual(functionNames, ["count", "increment"]);
});
