import { SlangRunner } from "./slang.runner.mjs";
import path from "node:path";
import { SolcRunner } from "./solc.runner.mjs";
import { checkCI, Runner, round2, Options } from "./common.mjs";
import commandLineArgs from "command-line-args";
import commandLineUsage from "command-line-usage";
import { SolcTypedAstRunner } from "./solc.typed.ast.runner.mjs";
import { SolidityParserRunner } from "./solidity.parser.runner.mjs";
import { assert } from "node:console";

class Measure {
  public name: string;
  public timings: Timing[] = [];

  public constructor(name: string) {
    this.name = name;
  }
}

class Timing {
  public component: string;
  public time: number;

  public constructor(component: string, time: number) {
    this.component = component;
    this.time = time;
  }
}

async function run(solidityVersion: string, dir: string, file: string, options: Options): Promise<Measure> {
  const measure = new Measure(path.parse(file).name);

  let tests: Runner[];
  if (options == Options.Parse) {
    tests = [new SlangRunner(options), new SolidityParserRunner(), new SolcTypedAstRunner()];
  } else {
    tests = [new SlangRunner(options), new SolcRunner()];
  }

  let optionsName;
  switch (options) {
    case Options.Parse:
      optionsName = "parsing";
      break;

    case Options.File:
      optionsName = "bindings main file";
      break;

    case Options.Project:
      optionsName = "bindings project";
      break;

    default:
      assert(false, "Expected options to be one of: File, Parse, Project");
      break;
  }

  for (const test of tests) {
    const time = await test.test(solidityVersion, dir, file);
    const name = test.name + " " + optionsName;
    measure.timings.push(new Timing(name, time));
  }

  return measure;
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

function capitalizeFirstLetter(str: string): string {
  return str.charAt(0).toUpperCase() + str.slice(1);
}

checkCI();

const optionDefinitions = [
  { name: "version", type: String },
  { name: "dir", type: String },
  { name: "file", type: String },
  { name: "options", type: (input: string) => Options[capitalizeFirstLetter(input) as keyof typeof Options] },
  { name: "verbose", type: Boolean, defaultValue: false },
];

const options = commandLineArgs(optionDefinitions);

const [version, dir, file, test] = [options["version"], options["dir"], options["file"], options["options"]];

if (!(version && dir && file && test)) {
  const usage = commandLineUsage([
    {
      header: "Comparisons",
      content: "Compares different parsing and binding libraries for Solidity",
    },
    {
      header: "Options",
      optionList: optionDefinitions,
    },
  ]);
  console.log(usage);
  process.exit(-1);
}
const resultCold = await run(version, dir, file, test);
const resultHot = await run(version, dir, file, test);

const output = buildOutput(resultCold, resultHot);

console.log(JSON.stringify(output, null, 2));
