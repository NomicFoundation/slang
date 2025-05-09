import { SlangBindingsFileRunner, SlangBindingsProjectRunner } from "./slang.runner.mjs";
import path from "node:path";
import { SolcRunner } from "./solc.runner.mjs";
import { checkCI, Runner, round2, Timing } from "./common.mjs";
import commandLineArgs from "command-line-args";
import commandLineUsage from "command-line-usage";
import { SolidityParserRunner } from "./solidity.parser.runner.mjs";

/// What to test
enum Runners {
  None, // Hack to make 0 be equal no "not specified"
  SlangFile, // resolve bindings of the main file only
  SlangProject, // resolve bindings of the entire project
  SolidityParser,
  Solc,
}

const runners: Map<Runners, Runner> = new Map([
  [Runners.SlangFile, new SlangBindingsFileRunner()],
  [Runners.SlangProject, new SlangBindingsProjectRunner()],
  [Runners.SolidityParser, new SolidityParserRunner()],
  [Runners.Solc, new SolcRunner()],
]);

async function run(
  solidityVersion: string,
  dir: string,
  file: string,
  runnerType: Runners,
  cold: number,
  hot: number,
): Promise<Timing[]> {
  const project = path.parse(file).name.toLowerCase();
  const runner = runners.get(runnerType)!;

  // cold runs
  for (let i = 0; i < cold; i++) {
    await runner.test(solidityVersion, dir, file);
  }

  const timesMap: Map<string, number> = new Map();

  // hot runs
  for (let i = 0; i < hot; i++) {
    let timings = await runner.test(solidityVersion, dir, file);

    for (const timing of timings) {
      let time = timesMap.get(timing.component) || 0;
      time += timing.time;
      timesMap.set(timing.component, time);
    }
  }

  let timings = [];
  for (const time of timesMap.entries()) {
    timings.push(new Timing(time[0] + "_" + project, round2(time[1] / hot)));
  }
  return timings;
}

checkCI();

const optionDefinitions = [
  { name: "version", type: String },
  { name: "dir", type: String },
  { name: "file", type: String },
  { name: "runner", type: (input: string) => Runners[input as keyof typeof Runners] },
  { name: "cold", type: Number, defaultValue: 2 },
  { name: "hot", type: Number, defaultValue: 5 },
  { name: "verbose", type: Boolean, defaultValue: false },
];

const options = commandLineArgs(optionDefinitions);

const [version, dir, file, runner] = [options["version"], options["dir"], options["file"], options["runner"]];

if (!(version && dir && file && runner)) {
  const usage = commandLineUsage([
    {
      header: "Perf NPM",
      content: "Executes different parsing and binding libraries for Solidity",
    },
    {
      header: "Options",
      optionList: optionDefinitions,
    },
  ]);
  console.log(usage);
  process.exit(-1);
}
const results = await run(version, dir, file, runner, options["cold"], options["hot"]);

console.log(JSON.stringify(results, null, 2));
