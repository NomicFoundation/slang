# 9.2. Considerations on memory usage of Slang's TypeScript bindings

Typical users of Slang should not worry about the memory consumption of Slang's objects. That said, we created this appendix for developers experiencing memory issues in their applications using Slang, or for those who are simply curious about the memory management of Slang objects.

Slang is written in Rust, cross-compiled into Wasm, and then imported into JavaScript/TypeScript. Understanding how the Wasm-JavaScript link works is key to understanding the memory consumption behavior of Slang objects. We begin by explaining two concepts of the JavaScript memory model: the distinction between _internal_ and _external_ memory, and _finalizers_.

## Internal vs external memory

JavaScript engines distinguish between the memory space that contains JavaScript objects, called _internal memory_ or _heap space_, and the space used for Wasm components, called _external memory_. When you interact with one of Slang's objects, that object resides in the internal memory. However, a Slang object, such as a Cursor, holds a _handle_ to a Wasm component that contains the fields of a cursor, _including a reference to the entire parsing tree_. Therefore, an instance of the `Cursor` class in JavaScript appears lightweight from the engine's perspective, as it only contains the handle number. In reality, it is associated with a significant amount of external memory—the actual Cursor in Wasm-land.

## Finalizers

To dispose of resources, JavaScript defines a [`FinalizationRegistry`](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/FinalizationRegistry), where each object's _finalizer_ function is registered. The finalizer is responsible for cleanup, and the engine is responsible for eventually calling this function for objects that have been garbage-collected.

Slang's JavaScript interface registers its finalizers in the `FinalizationRegistry` to dispose of the memory used in Wasm. One might hold the false expectation that once an object is disposed of by the garbage collector, its finalizer will run. However, the JavaScript specification defines something different:

> A conforming JavaScript implementation, even one that does garbage collection, is not required to call cleanup callbacks. When and whether it does so is entirely down to the implementation of the JavaScript engine. When a registered object is reclaimed, any cleanup callbacks for it may be called then, or some time later, or not at all.

In Node.js, for instance, [finalizers are not run](https://github.com/rustwasm/wasm-bindgen/issues/3917) when the code is purely or mostly synchronous. Depending on your engine and your application, you might experience a high usage of memory, even leading to a crash of the application due to an out-of-memory error. If this is your case, there's a simple solution to it:

## Giving the engine some time to breathe

If your application runs Slang in a tight loop, it is a good idea to add an asynchronous call, such as sleeping for a few milliseconds:

```javascript
await new Promise(r => setTimeout(r, 10));
```

In our tests, adding this timeout was sufficient to give the `FinalizationRegistry` time to properly clean up the objects. The external memory reached a peak and then remained steady.

In an interactive environment, such as a browser, it should not be necessary to add such a pause, as the engine has several opportunities to perform cleanup while waiting for user input.

An important point to mention is that the memory used in Wasm never shrinks back after being cleaned. This is [expected by design](https://github.com/WebAssembly/design/issues/1300#issuecomment-573867836).

> 🚧 **Warning**
>
> Garbage collection and finalizers are not deterministic by design, so tests on one machine and one JavaScript engine might differ greatly from those on a different machine or engine. In our tests, a simple timeout of 10ms was enough for the collector to collect and call the finalizers, but your experience might vary.
