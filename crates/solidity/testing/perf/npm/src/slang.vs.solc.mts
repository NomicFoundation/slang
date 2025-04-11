import { printTables, testFile } from "./common.slang.mjs";
import * as bench from "benny";
import { testFileSolC } from "./common.solc.mjs";
import { Summary } from "benny/lib/internal/common-types.js";
import * as mathjs from "mathjs";
import { checkCI } from "./common.mjs";
import commandLineArgs from "command-line-args"

// little helper function to perform the printing of numbers
function round(n: number | mathjs.MathNumericType): string {
  return mathjs.round(n, 2).toString();
}

async function run(solidityVersion: string, dir: string, file: string) {
  const result = await bench.suite(file,
    bench.add("slang", async () => {
      await testFile(solidityVersion, dir, file);
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
  printResult(result);
}


function printResult(result: Summary) {
  const slang = result.results[0];
  const solc = result.results[1];

  if (!slang.completed) {
    console.log(`${slang.name} couldn't finish!`);
  }
  const line =
    [result.name,
    slang.samples.toString(),
    round(slang.details.mean),
    round(slang.details.standardDeviation),
    round(solc.details.mean),
    round(solc.details.standardDeviation),
    round(slang.details.mean / solc.details.mean)];
  line.join(", ");

  // const slang_means = results.map((s: Summary) => { return s.results[0].details.mean; });
  // const solc_means = this.results.map((s: Summary) => { return s.results[1].details.mean; });

  // // ignore the summary if there are no results
  // if (slang_means.length == 0) {
  //   console.warn(`No comparisons matching ${this.pattern} were run`);
  // }
  // else {
  //   const mean_slang = mathjs.mean(slang_means);
  //   const std_slang = mathjs.std(slang_means);
  //   const mean_solc = mathjs.mean(solc_means);
  //   const std_solc = mathjs.std(solc_means);

  //   resultsLine.push([
  //     "Summary",
  //     "",
  //     round(mean_slang),
  //     `${std_slang}`,
  //     round(mean_solc),
  //     `${std_solc}`,
  //     round(mean_slang / mean_solc)
  //   ].join(", "));

  //   console.log(titleLine.concat(resultsLine).join("\n"));
  console.log(line);
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

checkCI();

// PROBLEM: can't run benny, as it will consume memory in each run
const options = commandLineArgs([
  { name: "version", type: String },
  { name: "dir", type: String },
  { name: "file", type: String },
  { name: "report-memory", type: Boolean, defaultValue: false },
  { name: "print-tables", type: Boolean, defaultValue: false }
]);

await run(options["version"], options["dir"], options["file"]);
