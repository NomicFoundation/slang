import { printTables, testFile as testFileSlang } from "./common.slang.mjs";
import path from "node:path";
import { testFile as testFileSolC } from "./common.solc.mjs";
import * as mathjs from "mathjs";
import { checkCI, sleep } from "./common.mjs";
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

checkCI();

class Output {
  public name: String = "";
  public coldSlang: number = 0;
  public coldSolC: number = 0;
  public coldRatio: number = 0;
  public hotSlang: number = 0;
  public hotSolC: number = 0;
  public hotRatio: number = 0;
}

function round2(n: number): number {
  return Math.round(n * 100) / 100;
}

function buildOutput(resultCold: Measure, resultHot: Measure): Output {
  const output = new Output();
  output.name = resultCold.name;
  output.coldSlang = round2(resultCold.timeSlang);
  output.coldSolC = round2(resultCold.timeSolC);
  output.coldRatio = round2(resultCold.timeSlang / resultCold.timeSolC);
  output.hotSlang = round2(resultHot.timeSlang);
  output.hotSolC = round2(resultHot.timeSolC);
  output.hotRatio = round2(resultHot.timeSlang / resultHot.timeSolC);
  return output;
}

const options = commandLineArgs([
  { name: "version", type: String },
  { name: "dir", type: String },
  { name: "file", type: String },
  { name: "report-memory", type: Boolean, defaultValue: false },
  { name: "print-tables", type: Boolean, defaultValue: false }
]);

const [version, dir, file] = [options["version"], options["dir"], options["file"]];

const resultCold = await run(version, dir, file);
const resultHot = await run(version, dir, file);

const output = buildOutput(resultCold, resultHot);
console.log(JSON.stringify(output, null, 2));
