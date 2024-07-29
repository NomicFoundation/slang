import type { Config } from "jest";

const config: Config = {
  rootDir: __dirname,
  testMatch: ["<rootDir>/src/tests/**/*.ts"],

  moduleNameMapper: {
    // __SLANG_NPM_PACKAGE_MAIN_OUTPUT_DIR__ (keep in sync)
    "^@slang-private/slang-testlang$": "<rootDir>/../package/target/npm/main",
    "^@slang-private/slang-testlang/(.*)?$": "<rootDir>/../package/target/npm/main/$1",
  },

  testEnvironment: "node",
  preset: "ts-jest",

  cacheDirectory: "<rootDir>/target/jest/cache",
  slowTestThreshold: 5,

  clearMocks: true,
  resetMocks: true,
};

export default config;
