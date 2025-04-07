import { printTables, testFile } from "./common.slang.mjs";
import * as bench from "benny";
import { testFileSolC } from "./common.solc.mjs";
import { Summary } from "benny/lib/internal/common-types.js";
import * as mathjs from "mathjs";
import { checkCI, INPUT_PATH, resolvePath } from "./common.mjs";
import path from "node:path";
import { readdir } from "node:fs/promises";
import commandLineArgs from "command-line-args"

// little helper function to perform the printing of numbers
function round(n: number | mathjs.MathNumericType): string {
  return mathjs.round(n, 2).toString();
}

class Comparer {
  results: Array<Summary> = [];
  pattern: RegExp;

  /// The optional pattern is used for matching cases
  constructor(pattern: string | undefined) {
    // No need to catch: it raises a meaningful exception if the pattern isn't well-formed
    this.pattern = new RegExp(pattern || ".*");
  }

  async run(name: string, solidityVersion: string, dir: string, file: string, expectedDefs?: number, expectedRefs?: number) {
    if (!name.match(this.pattern)) {
      return;
    }

    const result = await bench.suite(name,
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
    this.results.push(result);
  }

  // list recursively all the files
  private async listFilesRecursively(dir: string): Promise<string[]> {
    const entries = await readdir(dir, { withFileTypes: true });
    const files = await Promise.all(
      entries.map((entry) => {
        const fullPath = path.resolve(dir, entry.name);
        return entry.isDirectory() ? this.listFilesRecursively(fullPath) : fullPath;
      })
    );
    return files.flat();
  }

  /// the relativePath must be relative to common.INPUT_PATH
  async runRecursively(projectName: String, relativePath: string,) {
    const basePath = resolvePath(INPUT_PATH, relativePath);
    const files = await this.listFilesRecursively(basePath);
    for (const file of files) {
      if (file.endsWith(".sol")) {
        const dir = path.relative(basePath, path.dirname(file));
        const fileName = path.basename(file);
        const relativeFN = [dir, fileName].join("/");
        await comparer.run(`${projectName}/${relativeFN}`, "0.8.26", relativePath, relativeFN);
      }
    }
  }

  print_results() {
    const titleLine = ["name, slang.samples, slang.mean, slang.stddev, solc.mean, solc.stddev, ratio"];

    const resultsLine = this.results.map((s: Summary) => {
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

    const slang_means = this.results.map((s: Summary) => { return s.results[0].details.mean; });
    const solc_means = this.results.map((s: Summary) => { return s.results[1].details.mean; });

    // ignore the summary if there are no results
    if (slang_means.length == 0) {
      console.warn(`No comparisons matching ${this.pattern} were run`);
    }
    else {
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
    }
  }
}

async function logMemoryConsumption(previous: NodeJS.MemoryUsage | undefined, runGC = false): Promise<NodeJS.MemoryUsage | undefined> {
  const hasGC = typeof global.gc == "function";
  if (hasGC && runGC) {
    global.gc!();
    await sleep(100);
  }

  if (process.argv.includes("--report-memory") && previous) {
    const current = process.memoryUsage();

    if (!hasGC) {
      console.warn("Running wihtout --expose-gc");
    }
    console.log(`mem: ${current.heapUsed}, ${current.external}, ${current.heapUsed - previous.heapUsed}, ${current.external - previous.external}`);
    printTables();
    return current;
  }
  else {
    return undefined;
  }
}

function sleep(ms: number): Promise<void> {
  return new Promise((resolve) => setTimeout(resolve, ms));
}

async function runProjects(comparer: Comparer) {
  let memoryUsage: NodeJS.MemoryUsage | undefined = process.memoryUsage();
  await comparer.run("project/UiPoolDataProviderV2V3", "0.6.12", "0x00e50FAB64eBB37b87df06Aa46b8B35d5f1A4e1A", "contracts/misc/UiPoolDataProviderV2V3.sol", 58, 418);
  memoryUsage = await logMemoryConsumption(memoryUsage);
  await comparer.run("project/DoodledBears", "0.8.11", "0x015E220901014BAE4f7e168925CD74e725e23692/sources", "DoodledBears.sol", 57, 131);
  memoryUsage = await logMemoryConsumption(memoryUsage);
  await comparer.run("project/ERC721AContract", "0.8.13", "0x01665987bC6725070e56d160d75AA19d8B73273e/sources", "project:/contracts/ERC721AContract.sol", 121, 365);
  memoryUsage = await logMemoryConsumption(memoryUsage);
  await comparer.run("project/SeniorBond", "0.7.6", "0x0170f38fa8df1440521c8b8520BaAd0CdA132E82/sources", "contracts/SeniorBond.sol", 10, 20);
  memoryUsage = await logMemoryConsumption(memoryUsage);
  await comparer.run("project/Mooniswap", "0.6.12", "0x01a11a5A999E57E1B177AA2fF7fEA957605adA2b/sources", "Users/k06a/Projects/mooniswap-v2/contracts/Mooniswap.sol", 176, 672);
  memoryUsage = await logMemoryConsumption(memoryUsage);
  await comparer.run("project/Darts", "0.8.0", "0x01a5E3268E3987f0EE5e6Eb12fe63fa2AF992D83/sources", "contracts/Darts.sol", 17, 51);
  memoryUsage = await logMemoryConsumption(memoryUsage);
  await comparer.run("project/WeightedPool", "0.7.6", "0x01abc00E86C7e258823b9a055Fd62cA6CF61a163/sources", "contracts/pools/weighted/WeightedPool.sol", 143, 472);
  memoryUsage = await logMemoryConsumption(memoryUsage);
  await comparer.run("project/YaxisVotePower", "0.6.12", "0x01fef0d5d6fd6b5701ae913cafb11ddaee982c9a/YaxisVotePower", "contracts/governance/YaxisVotePower.sol", 27, 99);
  memoryUsage = await logMemoryConsumption(memoryUsage);
  await comparer.run("project/0xProject", "0.8.19", "0xProject/contracts/governance", "src/ZeroExProtocolGovernor.sol", 48, 88);
  memoryUsage = await logMemoryConsumption(memoryUsage);
  await comparer.run("project/Uniswap", "0.7.6", "Uniswap", "contracts/UniswapV3Factory.sol", 17, 85);
  memoryUsage = await logMemoryConsumption(memoryUsage);
  await comparer.run("project/AAVE", "0.8.10", "aave-v3-core-master", "contracts/protocol/pool/Pool.sol", 191, 629);
  memoryUsage = await logMemoryConsumption(memoryUsage);
  await comparer.run("project/GraphToken", "0.7.6", "graph_protocol/contracts", "token/GraphToken.sol", 41, 97);
  memoryUsage = await logMemoryConsumption(memoryUsage);
  await comparer.run("project/lidofinance", "0.8.9", "lidofinance/contracts/0.8.9", "WithdrawalQueueERC721.sol", 142, 325);
  memoryUsage = await logMemoryConsumption(memoryUsage, true);
}

checkCI();

const options = commandLineArgs([
  { name: "pattern", type: String, defaultValue: "" },
  { name: "report-memory", type: Boolean, defaultValue: false },
  { name: "print-tables", type: Boolean, defaultValue: false }
]);

const comparer = new Comparer(options["pattern"]);

await runProjects(comparer);

await comparer.runRecursively("@openzeppelin", "../node_modules/@openzeppelin/contracts");

comparer.print_results();
