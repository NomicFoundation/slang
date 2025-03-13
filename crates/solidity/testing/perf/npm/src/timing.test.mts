import { TerminalKind } from "@nomicfoundation/slang/cst";
import { createBuilder, INPUT_PATH } from "./common.mjs";
import { MathNumericType, max, mean, round, std } from "mathjs";
import assert from "node:assert";
import * as solc06 from "solc06";
import * as solc07 from "solc07";
import * as solc08 from "solc08";
import * as solc089 from "solc089";
import path from "node:path";
import fs from "node:fs";

// keep in sync with the fs order of hashes
describe("slang", () => {
  test("UiPoolDataProviderV2V3 slang", async () => {
    const file = "0x00e50FAB64eBB37b87df06Aa46b8B35d5f1A4e1A/contracts/misc/UiPoolDataProviderV2V3.sol";
    await testFile("0.6.12", file, 418, 58);
  });

  test("DoodledBears slang", async () => {
    const file = `0x015E220901014BAE4f7e168925CD74e725e23692/sources/DoodledBears.sol`;
    await testFile("0.8.11", file, 131, 57);
  });

  test("ERC721AContract slang", async () => {
    const file = "0x01665987bC6725070e56d160d75AA19d8B73273e/sources/project:/contracts/ERC721AContract.sol"
    await testFile("0.8.9", file, 365, 121);
  });

  test("SeniorBond slang", async () => {
    const file = "0x0170f38fa8df1440521c8b8520BaAd0CdA132E82/sources/contracts/SeniorBond.sol";
    await testFile("0.7.6", file, 20, 10);
  });

  test("Mooniswap slang", async () => {
    const file = "0x01a11a5A999E57E1B177AA2fF7fEA957605adA2b/sources/Users/k06a/Projects/mooniswap-v2/contracts/Mooniswap.sol";
    await testFile("0.6.12", file, 672, 176);
  });

  test("Darts slang", async () => {
    const file = "0x01a5E3268E3987f0EE5e6Eb12fe63fa2AF992D83/sources/contracts/Darts.sol";
    await testFile("0.8.0", file, 51, 17);
  });

  test("WeightedPool slang", async () => {
    const file = "0x01abc00E86C7e258823b9a055Fd62cA6CF61a163/sources/contracts/pools/weighted/WeightedPool.sol";
    await testFile("0.7.6", file, 472, 143);
  });

  test("YaxisVotePower slang", async () => {
    const file = "0x01fef0d5d6fd6b5701ae913cafb11ddaee982c9a/YaxisVotePower/contracts/governance/YaxisVotePower.sol";
    await testFile("0.7.0", file, 99, 27);
  });

  test("0xProject slang", async () => {
    const file = "0xProject/contracts/governance/src/ZeroExProtocolGovernor.sol";
    await testFile("0.8.19", file, 88, 48);
  });

  test("Uniswap slang", async () => {
    await testFile("0.7.6", "Uniswap/contracts/UniswapV3Factory.sol", 85, 17);
  });

  test("AAVE slang", async () => {
    await testFile("0.8.10", "aave-v3-core-master/contracts/protocol/pool/Pool.sol", 629, 191);
  });

  test("GraphToken slang", async () => {
    await testFile("0.7.6", "graph_protocol/contracts/token/GraphToken.sol", 97, 41);
  });

  test("lidofinance slang", async () => {
    await testFile("0.8.9", "lidofinance/contracts/0.8.9/WithdrawalQueueERC721.sol", 325, 142);
  });
})


describe("solc", () => {
  test("UiPoolDataProviderV2V3 solc", async () => {
    await testFileSolC("0.6.12", ["0x00e50FAB64eBB37b87df06Aa46b8B35d5f1A4e1A", "contracts"]);
  });

  test("DoodledBears solc", async () => {
    await testFileSolC("0.8.11", ["0x015E220901014BAE4f7e168925CD74e725e23692", "sources"]);
  });

  test("ERC721AContract solc", async () => {
    await testFileSolC("0.8.13", ["0x01665987bC6725070e56d160d75AA19d8B73273e", "sources"]);
  });

  test("SeniorBond solc", async () => {
    await testFileSolC("0.7.6", ["0x0170f38fa8df1440521c8b8520BaAd0CdA132E82", "sources"]);
  });

  test("Mooniswap solc", async () => {
    await testFileSolC("0.6.12", ["0x01a11a5A999E57E1B177AA2fF7fEA957605adA2b", "sources"]);
  });

  test("Darts solc", async () => {
    await testFileSolC("0.8.0", ["0x01a5E3268E3987f0EE5e6Eb12fe63fa2AF992D83", "sources"]);
  });

  test("WeightedPool solc", async () => {
    await testFileSolC("0.7.6", ["0x01abc00E86C7e258823b9a055Fd62cA6CF61a163", "sources", "contracts"]);
  });

  test("YaxisVotePower solc", async () => {
    await testFileSolC("0.6.12", ["0x01fef0d5d6fd6b5701ae913cafb11ddaee982c9a", "YaxisVotePower"]);
  });

  test("0xProject solc", async () => {
    await testFileSolC("0.8.19", ["0xProject", "contracts", "governance"]);
  });

  test("Uniswap solc", async () => {
    await testFileSolC("0.7.6", ["Uniswap", "contracts"]);
  });

  test("AAVE slang", async () => {
    await testFileSolC("0.8.10", ["aave-v3-core-master", "contracts"]);
  });

  test("GraphToken solc", async () => {
    await testFileSolC("0.7.6", ["graph_protocol", "contracts"]);
  });

  test("lidofinance solc", async () => {
    await testFileSolC("0.8.9", ["lidofinance", "contracts", "0.8.9"]);
  });
})

function findImports(folder: string[]): (file: string) => { contents: string } {
  const repoRoot = process.env["REPO_ROOT"];
  assert(repoRoot);
  return (file: string) => {
    const absolutePath = path.resolve(
      repoRoot,
      INPUT_PATH,
      ...folder,
      file,
    );
    const source = fs.readFileSync(absolutePath, "utf8");
    return { contents: source };
  };
}

async function testFileSolC(languageVersion: string, folder: string[]) {
  var solc = undefined;
  if (languageVersion.startsWith("0.6")) {
    solc = solc06;
  }
  else if (languageVersion.startsWith("0.7")) {
    solc = solc07;
  }
  else if (languageVersion.startsWith("0.8.9")) {
    solc = solc089;
  }
  else if (languageVersion.startsWith("0.8")) {
    solc = solc08;
  }

  const repoRoot = process.env["REPO_ROOT"];
  assert(repoRoot);

  const start = performance.now();
  const folderPath = path.resolve(
    repoRoot,
    INPUT_PATH,
    ...folder,
    "meta.json",
  );
  var folderMeta = fs.readFileSync(folderPath, "utf8");
  const _value = JSON.parse(solc.default.compile(folderMeta, { import: findImports(folder) }));
  console.log(`Processing ${folder[0]} with solc takes ${round(performance.now() - start)}ms`);
  assert(_value["sources"] != undefined);
  if (_value["errors"] && !_value["errors"].every((value: any) => value["type"] == "Warning")) {
    console.log(_value["errors"]);
    assert(false);
  }
}

class Record {
  file: string;
  totalTime: number = 0;
  buildGraphTime: number = 0;
  setupTime: number = 0;
  resolutionTime: number = 0;
  maxGoto: number = 0;
  meanGoto: number = 0;
  stdGoto: MathNumericType = 0;
  numberOfIdentifiers: number = 0;

  public constructor(file: string) {
    this.file = file
  }
}

const records: Record[] = []

afterAll(() => {
  var timeTable = `| File |	Total (ms) |	Setup (ms) |	Build (ms) |	Resolution Total (ms) |	Resolution Max (ms) |	Resolution mean (ms) |	Resolution std (ms) | Number of Ids |
|:-----|-----------:|------------:|------------:|-----------------------:|--------------------:|---------------------:|---------------------:|:-------------:|\n`;
  records.forEach((record) => {
    timeTable += `| ${record.file.split("/").pop()} | ${record.totalTime} | ${record.setupTime} | ${record.buildGraphTime} | ${record.resolutionTime} | ${record.maxGoto} | ${record.meanGoto} | ${record.stdGoto} | ${record.numberOfIdentifiers} |\n`;
  });
  console.log(timeTable);
});

async function testFile(languageVersion: string, file: string, expectedRefs: number, expectedDefs: number) {
  let gotoDefTimes: number[] = Array();
  const startTime = performance.now();
  const builder = await createBuilder(languageVersion);

  const record = new Record(file);

  await builder.addFile(file);

  const unit = builder.build();
  const cursor = unit.file(file)!.createTreeCursor();
  record.setupTime = round(performance.now() - startTime);

  // Validation: there shouldn't be any parsing errors, but if there are, let's print them nicely
  const files_w_errors = unit.files().filter((f) => f.errors().length > 0);
  const errors = files_w_errors.flatMap((f) => f.errors().map((e) => [f.id, e.message, e.textRange]));
  assert.deepStrictEqual(errors, []);

  let defs = 0;
  let refs = 0;
  let emptyDefList = [];
  let neitherDefNorRefSet = new Set<string>();

  // first access constructs the graph
  assert(typeof unit.bindingGraph.definitionAt == "function");
  record.buildGraphTime = round(performance.now() - startTime - record.setupTime);

  while (cursor.goToNextTerminalWithKind(TerminalKind.Identifier)) {
    record.numberOfIdentifiers++;
    const startDefRef = performance.now();
    const definition = unit.bindingGraph.definitionAt(cursor);
    const reference = unit.bindingGraph.referenceAt(cursor);

    const gotoDefTime = performance.now() - startDefRef;

    if (reference) {
      const defs = reference.definitions().length;
      if (defs > 0) {
        refs++;
      } else {
        emptyDefList.push(cursor.node.unparse());
      }
    }

    if (definition) {
      defs++;
    }

    const value = definition || reference;
    if (!value) {
      neitherDefNorRefSet.add(cursor.node.unparse());
    }

    gotoDefTimes.push(gotoDefTime);
  }

  record.totalTime = round(performance.now() - startTime);
  record.resolutionTime = record.totalTime - record.buildGraphTime - record.setupTime;
  record.maxGoto = round(max(gotoDefTimes));
  record.meanGoto = round(mean(gotoDefTimes));
  record.stdGoto = round(std(gotoDefTimes)[0] || 0);

  records.push(record);

  // We don't recognize `ABIEncoderV2` in `pragma experimental`, so let's ignore it
  const allowed = ["ABIEncoderV2", "v2"];
  const neitherDefNorRefList = Array.from(neitherDefNorRefSet);
  assert.deepStrictEqual(neitherDefNorRefList.filter((e) => !allowed.includes(e)), []);
  assert.deepStrictEqual(emptyDefList, []);
  assert.equal(refs, expectedRefs);
  assert.equal(defs, expectedDefs);
}
