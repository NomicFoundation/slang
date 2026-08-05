import { CompilationUnit } from "@nomicfoundation/slang/compilation";
import { buildCompilationUnit } from "./compilation-builder.mjs";

const CONTRACT_VFS = new Map<string, string>([
  [
    "contract.sol",
    `
contract Counter {
  uint _count;
  constructor(uint initialCount) {
    _count = initialCount;
  }
  function count() public view returns (uint) {
    return _count;
  }
  function increment(uint delta) public returns (uint) {
    require(delta > 0, "Delta must be positive");
    _count += delta;
    return _count;
  }
}
    `,
  ],
]);

export default function (): Promise<CompilationUnit> {
  return buildCompilationUnit(CONTRACT_VFS, "0.8.0", "contract.sol");
}
