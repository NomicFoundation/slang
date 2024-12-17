import { TerminalKind } from "@nomicfoundation/slang/cst";
import { createBuilder } from "./common.mjs";
import { max, mean, std } from "mathjs";

test("DoodledBears sanctuary", async () => {
  await testFile("015E220901014BAE4f7e168925CD74e725e23692_DoodledBears.sol");
});

test("DoodledBears individual file", async () => {
  const file = `0x015E220901014BAE4f7e168925CD74e725e23692/sources/DoodledBears.sol`
  await testFile(file);
});

test("WeightedPool sanctuary", async () => {
  await testFile("01abc00E86C7e258823b9a055Fd62cA6CF61a163_WeightedPool.sol");
});

test("WeightedPool individual file", async () => {
  await testFile("0x01abc00E86C7e258823b9a055Fd62cA6CF61a163/sources/contracts/pools/weighted/WeightedPool.sol");
});

test("UiPoolDataProviderV2V3 individual file", async () => {
  await testFile("0x00e50FAB64eBB37b87df06Aa46b8B35d5f1A4e1A/contracts/misc/UiPoolDataProviderV2V3.sol");
});

test("YaxisVotePower sanctuary", async () => {
  await testFile("01fef0d5d6fd6b5701ae913cafb11ddaee982c9a_YaxisVotePower.sol");
});

test("YaxisVotePower individual file", async () => {
  await testFile("0x01fef0d5d6fd6b5701ae913cafb11ddaee982c9a/YaxisVotePower/contracts/governance/YaxisVotePower.sol");
});

async function testFile(file: string) {
  let gotoDefTimes: number[] = Array();
  const start = performance.now();
  const builder = await createBuilder();

  await builder.addFile(file);

  const unit = builder.build();
  const cursor = unit.file(file)!.createTreeCursor();

  let undefinedRefs = 0;
  let definedRefs = 0;
  while (cursor.goToNextTerminalWithKind(TerminalKind.Identifier)) {
    const startDefRef = performance.now();
    const definition = unit.bindingGraph.definitionAt(cursor)!;
    const reference = unit.bindingGraph.referenceAt(cursor)!;
    const gotoDefTime = performance.now() - startDefRef;

    const value = definition || reference;
    if (!value) {
      //      console.log(`UNDEF: ${cursor.node}`);
      undefinedRefs += 1;
    }
    else {
      gotoDefTimes[definedRefs] = gotoDefTime
      definedRefs += 1;
    }
    //assert.notEqual(definition || reference, undefined);
  }

  const measure = performance.now() - start;
  const maxGoto = max(gotoDefTimes);
  const meanGoto = mean(gotoDefTimes);
  const stdGoto = std(gotoDefTimes);
  console.log(`file: ${file}\n\tundef: ${undefinedRefs}\n\tdef: ${definedRefs}\n\ttime: ${measure}\n\tmax: ${maxGoto}\tmean: ${meanGoto}\tstd: ${stdGoto}`);
}
