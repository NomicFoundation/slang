import type { Config } from "jest";

const config: Config = {
  projects: [
    // List all directories with package-level "jest.config.ts" files:
    "<rootDir>/crates/solidity/outputs/npm/tests/",
    "<rootDir>/crates/testlang/outputs/npm/tests/",
  ],
};

export default config;
