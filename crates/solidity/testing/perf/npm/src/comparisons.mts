import { SlangTest } from "./common.slang.mjs";
import path from "node:path";
import { SolcPlainTest } from "./solc.plain.mjs";
import { checkCI, Test } from "./common.mjs";
import commandLineArgs from "command-line-args"
import { SolcTypedAstTest } from "./solc.typed.ast.mjs";

class Timing {
  public component: string;
  public time: number;

  public constructor(component: string, time: number) {
    this.component = component;
    this.time = time;
  }
}

class Measure {
  public name: string = "";
  public timings: Timing[] = [];
}

async function run(solidityVersion: string, dir: string, file: string): Promise<Measure> {
  const measure = new Measure();
  measure.name = path.parse(file).name;

  const tests: Test<void>[] = [new SlangTest(), new SolcPlainTest(), new SolcTypedAstTest()];

  for (const test of tests) {
    const start = performance.now();
    await test.test(solidityVersion, dir, file);
    measure.timings.push(new Timing(test.name, performance.now() - start));
  };

  return measure;
}

function round2(n: number): number {
  return Math.round(n * 100) / 100;
}

function buildOutput(resultCold: Measure, resultHot: Measure): Measure {
  resultCold.timings.forEach((value) => {
    value.component += " cold";
    value.time = round2(value.time);
  });
  resultHot.timings.forEach((value) => {
    value.component += " hot";
    value.time = round2(value.time);
  });
  resultCold.timings.push(...resultHot.timings);

  return resultCold;
}

checkCI();

const options = commandLineArgs([
  { name: "version", type: String },
  { name: "dir", type: String },
  { name: "file", type: String },
  { name: "verbose", type: Boolean, defaultValue: false }
]);

const [version, dir, file] = [options["version"], options["dir"], options["file"]];

const resultCold = await run(version, dir, file);
const resultHot = await run(version, dir, file);

const output = buildOutput(resultCold, resultHot);

console.log(JSON.stringify(output, null, 2));
