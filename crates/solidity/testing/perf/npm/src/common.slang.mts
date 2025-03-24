import { TerminalKind } from "@nomicfoundation/slang/cst";
import { createBuilder, INPUT_PATH } from "./common.mjs";
import { MathNumericType, max, mean, round, std } from "mathjs";
import assert from "node:assert";

export class Record {
  file: string;
  totalTime: number = 0;
  buildGraphTime: number = 0;
  setupTime: number = 0;
  resolutionTime: number = 0;
  maxGoto: number = 0;
  meanGoto: number = 0;
  stdGoto: MathNumericType = 0;
  numberOfIdentifiers: number = 0;

  public constructor(file: string) {
    this.file = file
  }
}

export async function testFile(languageVersion: string, file: string, expectedDefs?: number, expectedRefs?: number): Promise<Record> {
  let gotoDefTimes: number[] = Array();
  const startTime = performance.now();
  const builder = createBuilder(languageVersion);

  const record = new Record(file);

  await builder.addFile(file);

  const unit = builder.build();
  const cursor = unit.file(file)!.createTreeCursor();
  record.setupTime = round(performance.now() - startTime);

  // Validation: there shouldn't be any parsing errors, but if there are, let's print them nicely
  const files_w_errors = unit.files().filter((f) => f.errors().length > 0);
  const errors = files_w_errors.flatMap((f) => f.errors().map((e) => [f.id, e.message, e.textRange]));
  assert.deepStrictEqual(errors, []);

  let defs = 0;
  let refs = 0;
  let emptyDefList = [];
  let neitherDefNorRefSet = new Set<string>();

  // first access constructs the graph
  assert(typeof unit.bindingGraph.definitionAt == "function");
  record.buildGraphTime = round(performance.now() - startTime - record.setupTime);

  while (cursor.goToNextTerminalWithKind(TerminalKind.Identifier)) {
    record.numberOfIdentifiers++;
    const startDefRef = performance.now();
    const definition = unit.bindingGraph.definitionAt(cursor);
    const reference = unit.bindingGraph.referenceAt(cursor);

    const gotoDefTime = performance.now() - startDefRef;

    if (reference) {
      const defs = reference.definitions().length;
      if (defs > 0) {
        refs++;
      } else {
        emptyDefList.push(cursor.node.unparse());
      }
    }

    if (definition) {
      defs++;
    }

    const value = definition || reference;
    if (!value) {
      neitherDefNorRefSet.add(cursor.node.unparse());
    }

    gotoDefTimes.push(gotoDefTime);
  }

  record.totalTime = round(performance.now() - startTime);
  record.resolutionTime = record.totalTime - record.buildGraphTime - record.setupTime;
  record.maxGoto = round(max(gotoDefTimes));
  record.meanGoto = round(mean(gotoDefTimes));
  record.stdGoto = round(std(gotoDefTimes)[0] || 0);

  // We don't recognize `ABIEncoderV2` in `pragma experimental`, so let's ignore it
  const allowed = ["ABIEncoderV2", "v2"];
  const neitherDefNorRefList = Array.from(neitherDefNorRefSet);
  assert.deepStrictEqual(neitherDefNorRefList.filter((e) => !allowed.includes(e)), []);
  assert.deepStrictEqual(emptyDefList, []);

  if (expectedDefs) {
    assert.equal(refs, expectedRefs);
    assert.equal(defs, expectedDefs);
  }
  return record;
}
