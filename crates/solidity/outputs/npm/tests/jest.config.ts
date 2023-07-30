import type { Config } from "jest";
import * as path from "path";

const config: Config = {
  rootDir: path.join(__dirname, "src"),
  testMatch: ["**/*.ts"],

  testEnvironment: "node",
  preset: "ts-jest",

  cacheDirectory: path.join(__dirname, "target/jest/cache"),
  slowTestThreshold: 5,
  verbose: true,

  clearMocks: true,
  resetMocks: true,

  globalSetup: undefined,
  globalTeardown: undefined,
};

export default config;
