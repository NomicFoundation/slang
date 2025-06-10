import { checkCI, Runner, round2, Timings, SolidityProject } from "./common.mjs";
import commandLineArgs from "command-line-args";
import commandLineUsage from "command-line-usage";
import { AntlrRunner, SlangBindingsFileRunner, SlangBindingsProjectRunner, SolcRunner } from "./runners/runners.mjs";

const runners: Map<string, Runner> = new Map([
  ["SlangFile", new SlangBindingsFileRunner()],
  ["SlangProject", new SlangBindingsProjectRunner()],
  ["Antlr", new AntlrRunner()],
  ["Solc", new SolcRunner()],
]);

async function run(
  dir: string,
  name: string,
  file: string | undefined,
  runner: Runner,
  cold: number,
  hot: number,
): Promise<Timings> {
  const project = SolidityProject.build(dir + ".json");
  file = file || project.compilation.entrypoint();

  // cold runs
  for (let i = 0; i < cold; i++) {
    await runner.test(project, file);
  }

  const timesMap: Timings = new Map();

  // hot runs
  for (let i = 0; i < hot; i++) {
    let timings = await runner.test(project, file);

    for (const [component, measured] of timings) {
      const accutime = timesMap.get(component) || 0;
      timesMap.set(component, accutime + measured);
    }
  }

  const timings: Timings = new Map();

  for (const [component, measured] of timesMap) {
    timings.set(component + "_" + name, round2(measured / hot));
  }

  return timings;
}

checkCI();

const optionDefinitions = [
  { name: "dir", type: String },
  { name: "name", type: String },
  { name: "file", type: String, defaultValue: undefined },
  { name: "runner", type: (input: string) => runners.get(input) },
  { name: "cold", type: Number, defaultValue: 2 },
  { name: "hot", type: Number, defaultValue: 5 },
  { name: "verbose", type: Boolean, defaultValue: false },
];

const options = commandLineArgs(optionDefinitions);

const [dir, name, file, runner] = [options["dir"], options["name"], options["file"], options["runner"]];

if (!(dir && name && runner)) {
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
const results = await run(dir, name, file, runner, options["cold"], options["hot"]);

console.log(JSON.stringify(Object.fromEntries(results), undefined, 2));
