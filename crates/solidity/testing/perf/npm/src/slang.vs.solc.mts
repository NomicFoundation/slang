import { printTables, testFile as testFileSlang } from "./common.slang.mjs";
import path from "node:path";
import { testFile as testFileSolC } from "./common.solc.mjs";
import * as mathjs from "mathjs";
import { checkCI } from "./common.mjs";
import commandLineArgs from "command-line-args"

// little helper function to perform the printing of numbers
function round(n: number | mathjs.MathNumericType): string {
  return mathjs.round(n, 2).toString();
}

class Measure {
  public name: string = "";
  public timeSolC: number = 0;
  public timeSlang: number = 0;
}

async function run(solidityVersion: string, dir: string, file: string): Promise<Measure> {
  const resultCold = new Measure();
  resultCold.name = path.parse(file).name;
  const start = performance.now();
  await testFileSlang(solidityVersion, dir, file);
  resultCold.timeSlang = performance.now() - start;
  await testFileSolC(solidityVersion, dir, file);
  resultCold.timeSolC = performance.now() - resultCold.timeSlang - start;
  return resultCold;
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

const options = commandLineArgs([
  { name: "version", type: String },
  { name: "dir", type: String },
  { name: "file", type: String },
  { name: "report-memory", type: Boolean, defaultValue: false },
  { name: "print-tables", type: Boolean, defaultValue: false }
]);

const resultCold = await run(options["version"], options["dir"], options["file"]);
const resultHot = await run(options["version"], options["dir"], options["file"]);

const line =
  [resultCold.name,
  round(resultCold.timeSlang),
  round(resultCold.timeSolC),
  round(resultCold.timeSlang / resultCold.timeSolC),
  round(resultHot.timeSlang),
  round(resultHot.timeSolC),
  round(resultHot.timeSlang / resultHot.timeSolC)];
line.join(", ");

console.log(line);
