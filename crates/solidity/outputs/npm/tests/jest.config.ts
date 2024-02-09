import type { Config } from "jest";

const config: Config = {
  rootDir: __dirname,
  testMatch: ["<rootDir>/src/doc-examples/**/*.ts", "<rootDir>/src/tests/**/*.ts"],

  moduleNameMapper: {
    // __SLANG_NPM_PACKAGE_MAIN_OUTPUT_DIR__ (keep in sync)
    "^@nomicfoundation/slang$": "<rootDir>/../package/target/npm/main",
    "^@nomicfoundation/slang/(.*)?$": "<rootDir>/../package/target/npm/main/$1",
  },

  testEnvironment: "node",
  preset: "ts-jest",

  cacheDirectory: "<rootDir>/target/jest/cache",
  slowTestThreshold: 5,
  verbose: true,

  clearMocks: true,
  resetMocks: true,
};

export default config;
