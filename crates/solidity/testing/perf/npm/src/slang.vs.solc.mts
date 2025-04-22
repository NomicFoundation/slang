import { testFile as testFileSlang } from "./common.slang.mjs";
import path from "node:path";
import { testFile as testFileSolc } from "./common.solc.mjs";
import { checkCI, sleep } from "./common.mjs";
import commandLineArgs from "command-line-args"

class Measure {
  public name: string = "";
  public timeSolc: number = 0;
  public timeSlang: number = 0;
  public heapUsedSlang: number = 0;
  public externalSlang: number = 0;
}

const hasGC = typeof global.gc == "function";

async function runGC() {
  if (hasGC) {
    global.gc!();
    await sleep(100);
  }
}

async function run(solidityVersion: string, dir: string, file: string): Promise<Measure> {
  const measure = new Measure();
  measure.name = path.parse(file).name;

  await runGC();
  const beforeMemory = process.memoryUsage();
  let start = performance.now();
  await testFileSlang(solidityVersion, dir, file);
  measure.timeSlang = performance.now() - start;

  await runGC();
  const afterMemory = process.memoryUsage();
  measure.heapUsedSlang = afterMemory.heapUsed - beforeMemory.heapUsed;
  measure.externalSlang = afterMemory.external - beforeMemory.external;

  start = performance.now();
  await testFileSolc(solidityVersion, dir, file);
  measure.timeSolc = performance.now() - start;
  return measure;
}

class Output {
  public name: String = "";
  public coldSlang: number = 0;
  public coldSolc: number = 0;
  public coldRatio: number = 0;
  public coldHeap: number = 0;
  public coldExternal: number = 0;
  public hotSlang: number = 0;
  public hotSolc: number = 0;
  public hotRatio: number = 0;
  public hotHeap: number = 0;
  public hotExternal: number = 0;
}

function round2(n: number): number {
  return Math.round(n * 100) / 100;
}

function buildOutput(resultCold: Measure, resultHot: Measure): Output {
  const output = new Output();
  output.name = resultCold.name;
  output.coldSlang = round2(resultCold.timeSlang);
  output.coldSolc = round2(resultCold.timeSolc);
  output.coldRatio = round2(resultCold.timeSlang / resultCold.timeSolc);
  output.coldHeap = resultCold.heapUsedSlang;
  output.coldExternal = resultCold.externalSlang;

  output.hotSlang = round2(resultHot.timeSlang);
  output.hotSolc = round2(resultHot.timeSolc);
  output.hotRatio = round2(resultHot.timeSlang / resultHot.timeSolc);
  output.hotHeap = resultHot.heapUsedSlang;
  output.hotExternal = resultHot.externalSlang;

  return output;
}

checkCI();

const options = commandLineArgs([
  { name: "version", type: String },
  { name: "dir", type: String },
  { name: "file", type: String }
]);

const [version, dir, file] = [options["version"], options["dir"], options["file"]];

const resultCold = await run(version, dir, file);
const resultHot = await run(version, dir, file);

const output = buildOutput(resultCold, resultHot);

if (!hasGC) {
  console.log("Running wihtout `--expose-gc`, memory readings will not be accurate");
}
console.log(JSON.stringify(output, null, 2));
