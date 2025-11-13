import type { Config } from "jest";

export default {
  testMatch: ["<rootDir>/**/*.test.mts"],

  testPathIgnorePatterns: [".*\/\\.hermit\/", ".*\/node_modules\/", ".*\/submodules\/", ".*\/target\/"],

  moduleFileExtensions: [
    /*  Plain: */ ["js", "ts"],
    /*  ESM:   */ ["mjs", "mts"],
    /*  CJS:   */ ["cjs", "cts"],
    /*  JSX:   */ ["jsx", "tsx"],
    /*  Other: */ ["json", "node", "wasm"],
  ].flat(),

  extensionsToTreatAsEsm: [
    // ".mjs" is already included.
    ".mts",
  ],

  cacheDirectory: "<rootDir>/target/jest/cache",

  slowTestThreshold: 5,

  clearMocks: true,
  resetMocks: true,

  resolver: "ts-jest-resolver",

  transform: {
    "^.+\\.m?[tj]sx?$": [
      "ts-jest",
      {
        useESM: true,
      },
    ],
  },
} satisfies Config;
