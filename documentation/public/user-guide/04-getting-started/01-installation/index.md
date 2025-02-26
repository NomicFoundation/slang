# 4.1. Installation

## Adding the NPM package

You can install [Slang NPM package](https://www.npmjs.com/package/@nomicfoundation/slang) simply by running the following `npm` command:

```bash title="bash"
npm install "@nomicfoundation/slang"
```

Or if you are using `yarn` for package management:

```bash title="bash"
yarn add "@nomicfoundation/slang"
```

## ESM vs CommonJS

Slang is implemented in Rust, and compiled as a WASM component, which is exposed to TypeScript/JavaScript, and loaded asynchronously.
If you are working with a modern ESM project, this will just work out of the box, with no additional configuration needed.

```ts title="use-from-esm.mts"
--8<-- "documentation/public/user-guide/04-getting-started/01-installation/examples/01-use-from-esm.test.mts"
```

But if you are working with a legacy CommonJS project, asynchronous imports are not supported.
In that case, you can use the `await import()` syntax to load Slang:

```ts title="use-from-commonjs.mts"
--8<-- "documentation/public/user-guide/04-getting-started/01-installation/examples/02-use-from-commonjs.test.mts"
```
