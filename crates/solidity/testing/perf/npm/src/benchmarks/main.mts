import { Subject, round2, Timings, SolidityProject } from "./common.mjs";
import commandLineArgs from "command-line-args";
import commandLineUsage from "command-line-usage";
import { AntlrSubject, SlangSubject, SolcSubject } from "./subjects/subjects.mjs";

const subjects: Map<string, Subject> = new Map([
  ["Slang", new SlangSubject()],
  ["Antlr", new AntlrSubject()],
  ["Solc", new SolcSubject()],
]);

async function run(
  dir: string,
  name: string,
  file: string | undefined,
  subject: Subject,
  cold: number,
  hot: number,
): Promise<Timings> {
  const project = SolidityProject.build(dir + ".json");
  file = file || project.compilation.entrypoint();

  // cold runs
  for (let i = 0; i < cold; i++) {
    await subject.test(project, file);
  }

  const timesMap: Timings = new Map();

  // hot runs
  for (let i = 0; i < hot; i++) {
    let timings = await subject.test(project, file);

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

const dirOption = "dir";
const nameOption = "name";
const fileOption = "file";
const subjectOption = "subject";
const coldOption = "cold";
const hotOption = "hot";

const optionDefinitions = [
  { name: dirOption, type: String },
  { name: nameOption, type: String },
  { name: fileOption, type: String, defaultValue: undefined },
  { name: subjectOption, type: (input: string) => subjects.get(input) },
  { name: coldOption, type: Number, defaultValue: 2 },
  { name: hotOption, type: Number, defaultValue: 5 },
];

const options = commandLineArgs(optionDefinitions);

const [dir, name, file, subject] = [
  options[dirOption],
  options[nameOption],
  options[fileOption],
  options[subjectOption],
];

if (!(dir && name && subject)) {
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
const results = await run(dir, name, file, subject, options[coldOption], options[hotOption]);

console.log(JSON.stringify(Object.fromEntries(results), undefined, 2));
