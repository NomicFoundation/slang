# 9.2. Slang Memory Management

For typical uses of Slang, developers should not worry about memory usage. That said, this appendix may be useful if you're experiencing memory issues in your applications using Slang, or you're curious about the memory management when working with Slang objects.

Slang's core is written in Rust, cross-compiled into Web Assembly, and then imported into JavaScript/TypeScript. Understanding how the Wasm-JavaScript link works is key to understanding the memory usage behavior of Slang objects.

## Internal vs external memory

JavaScript engines distinguish between the memory space that contains JavaScript objects, called _internal memory_ or _heap space_, and the space used for Wasm components, called _external memory_. The Slang objects that you interact with reside in internal memory. However, a Slang object, such as a `Cursor`, holds a _handle_ to a Wasm component that manages a corresponding resource in the Rust implementation. In the case of a `Cursor`, that resource contains among other things _a reference to the entire parsing tree_. Therefore, an instance of the `Cursor` class in JavaScript is lightweight in terms of internal memory as it only contains the handle number. But it is associated with a significant amount of external memory—the actual cursor and its tree in Wasm-land.

## Finalizers and cleanup callbacks

To dispose of external resources, JavaScript provides a [`FinalizationRegistry`](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/FinalizationRegistry), where a cleanup callback or object _finalizer_ function can be registered. This cleanup callback can be used to free any external resources associated with the JavaScript object being finalized.

Slang's JavaScript interface registers its finalizers in a `FinalizationRegistry` to dispose of the memory used in Wasm. The JavaScript engine is responsible for calling them for objects that have been garbage-collected. However, the JavaScript specification provides no warranties of _when_ this happens:

> A conforming JavaScript implementation, even one that does garbage collection, is not required to call cleanup callbacks. When and whether it does so is entirely down to the implementation of the JavaScript engine. When a registered object is reclaimed, any cleanup callbacks for it may be called then, or some time later, or not at all.

In practice, cleanup callbacks are run _asynchronously_ and _after_ the objects in internal memory have been garbage-collected. For instance in Node.js, [finalizers are not run](https://github.com/rustwasm/wasm-bindgen/issues/3917) when the code is purely or mostly synchronous.

Depending on your engine and your application, you might experience high memory usage, even leading to a crash of the application due to an out-of-memory error. If this is your case, here's what you can do to mitigate the problem:

## Giving the engine some time to breathe

If your application runs Slang in a tight loop, it is a good idea to add an asynchronous call, such as sleeping for a few milliseconds:

```javascript
await new Promise((r) => setTimeout(r, 10));
```

In our tests, adding this timeout was sufficient to give the `FinalizationRegistry` time to properly clean up the objects. The external memory usage grows at the start, but eventually stabilizes and remains steady.

In an interactive environment, such as an IDE or a browser, it should not be necessary to add such a pause, as the engine has several opportunities to perform cleanup while waiting for user input.

An important point to mention is that the memory used in Wasm never shrinks back after being cleaned. This is [expected by design](https://github.com/WebAssembly/design/issues/1300#issuecomment-573867836).

> 🚧 **Warning**
>
> Garbage collection and finalizers are not deterministic by design, so tests on one machine and one JavaScript engine might differ greatly from those on a different machine or engine. In our tests, a simple timeout of 10ms was enough for the collector to collect and call the finalizers, but your experience might vary.
