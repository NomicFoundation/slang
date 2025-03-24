import { testFile } from "./common.slang.mjs";
import * as bench from "benny";
import { testFileSolC } from "./common.solc.mjs";
import { Summary } from "benny/lib/internal/common-types.js";
import * as mathjs from "mathjs";
import { INPUT_PATH, resolvePath } from "./common.mjs";
import path from "node:path";
import { readdir } from "node:fs/promises";


async function run(name: string, solidityVersion: string, dir: string, file: string, expectedDefs?: number, expectedRefs?: number): Promise<Summary> {
  const results = await bench.suite(name,
    bench.add("slang", async () => {
      await testFile(solidityVersion, [dir, file].join("/"), expectedDefs, expectedRefs);
    }),
    bench.add("solc", async () => {
      await testFileSolC(solidityVersion, dir, file);
    }),
    bench.configure({
      cases: {
        maxTime: 1,
        minSamples: 2
      }
    }),
    bench.cycle(() => undefined)); // we don't want to print anything
  return results;
}

const results: Array<Summary> = [];
results.push(await run("UiPoolDataProviderV2V3", "0.6.12", "0x00e50FAB64eBB37b87df06Aa46b8B35d5f1A4e1A", "contracts/misc/UiPoolDataProviderV2V3.sol", 58, 418));
results.push(await run("DoodledBears", "0.8.11", "0x015E220901014BAE4f7e168925CD74e725e23692/sources", "DoodledBears.sol", 57, 131));
results.push(await run("ERC721AContract", "0.8.13", "0x01665987bC6725070e56d160d75AA19d8B73273e/sources", "project:/contracts/ERC721AContract.sol", 121, 365));
results.push(await run("SeniorBond", "0.7.6", "0x0170f38fa8df1440521c8b8520BaAd0CdA132E82/sources", "contracts/SeniorBond.sol", 10, 20));
results.push(await run("Mooniswap", "0.6.12", "0x01a11a5A999E57E1B177AA2fF7fEA957605adA2b/sources", "Users/k06a/Projects/mooniswap-v2/contracts/Mooniswap.sol", 176, 672));
results.push(await run("Darts", "0.8.0", "0x01a5E3268E3987f0EE5e6Eb12fe63fa2AF992D83/sources", "contracts/Darts.sol", 17, 51));
results.push(await run("WeightedPool", "0.7.6", "0x01abc00E86C7e258823b9a055Fd62cA6CF61a163/sources", "contracts/pools/weighted/WeightedPool.sol", 143, 472));
results.push(await run("YaxisVotePower", "0.6.12", "0x01fef0d5d6fd6b5701ae913cafb11ddaee982c9a/YaxisVotePower", "contracts/governance/YaxisVotePower.sol", 27, 99));
results.push(await run("0xProject", "0.8.19", "0xProject/contracts/governance", "src/ZeroExProtocolGovernor.sol", 48, 88));
results.push(await run("Uniswap", "0.7.6", "Uniswap", "contracts/UniswapV3Factory.sol", 17, 85));
results.push(await run("AAVE", "0.8.10", "aave-v3-core-master", "contracts/protocol/pool/Pool.sol", 191, 629));
results.push(await run("GraphToken", "0.7.6", "graph_protocol/contracts", "token/GraphToken.sol", 41, 97));
results.push(await run("lidofinance", "0.8.9", "lidofinance/contracts/0.8.9", "WithdrawalQueueERC721.sol", 142, 325));

// list recursively all the files in oz_path and test it
async function listFilesRecursively(dir: string): Promise<string[]> {
  const entries = await readdir(dir, { withFileTypes: true });
  const files = await Promise.all(
    entries.map((entry) => {
      const fullPath = path.resolve(dir, entry.name);
      return entry.isDirectory() ? listFilesRecursively(fullPath) : fullPath;
    })
  );
  return files.flat();
}

async function testOpenZeppelinFiles() {
  const root = "../node_modules/@openzeppelin/contracts";
  const oz_path = resolvePath(INPUT_PATH, root);
  const files = await listFilesRecursively(oz_path);
  for (const file of files) {
    if (file.endsWith(".sol")) {
      const dir = path.relative(oz_path, path.dirname(file));
      const fileName = path.basename(file);
      const relativeFN = [dir, fileName].join("/");
      results.push(await run(`@openzeppelin/${relativeFN}`, "0.8.26", root, relativeFN));
    }
  }
}

await testOpenZeppelinFiles();


function round(n: number | mathjs.MathNumericType): string {
  return mathjs.round(n, 2).toString();
}

const titleLine = ["name, slang.samples, slang.mean, slang.stddev, solc.mean, solc.stddev, ratio"];

const resultsLine = results.map((s: Summary) => {
  const slang = s.results[0];
  const solc = s.results[1];

  if (!slang.completed) {
    console.log(`${slang.name} couldn't finish!`);
  }
  const line =
    [s.name,
    slang.samples.toString(),
    round(slang.details.mean),
    round(slang.details.standardDeviation),
    round(solc.details.mean),
    round(solc.details.standardDeviation),
    round(slang.details.mean / solc.details.mean)];
  return line.join(", ");
});

const slang_means = results.map((s: Summary) => { return s.results[0].details.mean; });
const solc_means = results.map((s: Summary) => { return s.results[1].details.mean; });

const mean_slang = mathjs.mean(slang_means);
const std_slang = mathjs.std(slang_means);
const mean_solc = mathjs.mean(solc_means);
const std_solc = mathjs.std(solc_means);

resultsLine.push([
  "Summary",
  "",
  round(mean_slang),
  `${std_slang}`,
  round(mean_solc),
  `${std_solc}`,
  round(mean_slang / mean_solc)
].join(", "));

console.log(titleLine.concat(resultsLine).join("\n"));
