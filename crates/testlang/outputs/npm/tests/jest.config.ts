import type { Config } from "jest";

const config: Config = {
  testMatch: ["<rootDir>/src/tests/**/*.mts"],

  preset: "ts-jest/presets/default-esm",
  resolver: "ts-jest-resolver",
  moduleFileExtensions: ["js", "mjs", "cjs", "jsx", "ts", "mts", "cts", "tsx", "json", "node"],

  cacheDirectory: "<rootDir>/target/jest/cache",
  slowTestThreshold: 5,

  clearMocks: true,
  resetMocks: true,
};

export default config;
