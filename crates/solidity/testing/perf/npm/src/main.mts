import { checkCI, Runner, round2, Timing, SolidityProject } from "./common.mjs";
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
): Promise<Timing[]> {
  const project = SolidityProject.build(dir + ".json");
  file = file || project.compilation.entrypoint();

  // cold runs
  for (let i = 0; i < cold; i++) {
    await runner.test(project, file);
  }

  const timesMap: Map<string, number> = new Map();

  // hot runs
  for (let i = 0; i < hot; i++) {
    let timings = await runner.test(project, file);

    for (const timing of timings) {
      let time = timesMap.get(timing.component) || 0;
      time += timing.time;
      timesMap.set(timing.component, time);
    }
  }

  let timings = [];
  for (const time of timesMap.entries()) {
    timings.push(new Timing(time[0] + "_" + name, round2(time[1] / hot)));
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

console.log(JSON.stringify(results, null, 2));
