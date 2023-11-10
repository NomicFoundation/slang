import type { Config } from "jest";

const config: Config = {
  rootDir: __dirname,
  testMatch: ["<rootDir>/src/doc-examples/**/*.ts", "<rootDir>/src/tests/**/*.ts"],

  testEnvironment: "node",
  preset: "ts-jest",

  cacheDirectory: "<rootDir>/target/jest/cache",
  slowTestThreshold: 5,
  verbose: true,

  clearMocks: true,
  resetMocks: true,
};

export default config;
