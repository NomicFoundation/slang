# 9.2. Considerations on memory usage of Slang's Typescript's bindings

Typical users of Slang should not worry about the memory consumption taken by Slang's objects. That said, we made this appendix for those developers that are experience issues with memory in their applications using Slang, or that are simply curious about memory management of Slang objects.

Slang is coded in Rust, cross-compiled into Wasm, and then imported in JavaScript/Typescript. Understanding how the link Wasm-JavaScript works is key to understand the behavior of memory consumption of Slang objects. We start explaining two concepts of JavaScript memory model: the distinction between _internal_ and _external_ memory, and _finalizers_.

## Internal vs external memory

JavaScript engines make a distinction between the memory space that contains JavaScript's objects, called _internal memory_ or _heap space_, and the space used for Wasm components, called _external memory_. When you interact with one of Slang's objects, that object lives in the internal memory. But a Slang object, for instance, a Cursor, will hold a _handle_ to a Wasm component that contains the fields of a cursor, _including a reference to the entire parsing tree_. Therefore, an instance of the `Cursor` class in JavaScript is thin, from the perspective of the engine, as it just contains the number for the handle. But in reality, it's associated to a considerable amount of external memory; the actual Cursor in Wasm-land.

## Finalizers

In order to dispose resources, JavaScript defines a [`FinalizationRegistry`](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/FinalizationRegistry), where each object's _finalizer_ function is registered. The finalizer is responsible to do the clean-up, and the engine is responsible for calling this function _eventually_, for those objects that were garbage-collected.

Slang's JavaScript interface, then, registers its finalizers in the `FinalizationRegistry` to dispose the memory used in Wasm. In an ideal world, once an object is disposed by the garbage collector, it will run its finalizer. However, the specification of JavaScript is clear about when such thing might happen:

> A conforming JavaScript implementation, even one that does garbage collection, is not required to call cleanup callbacks. When and whether it does so is entirely down to the implementation of the JavaScript engine. When a registered object is reclaimed, any cleanup callbacks for it may be called then, or some time later, or not at all.

In Node, for instance, [finalizers are not run](https://github.com/rustwasm/wasm-bindgen/issues/3917) when the code is purely, or mostly, synchronous.

## Giving the engine some time to breathe

If your application runs Slang in a tight loop, it is a good idea to add some asynchronous call, like sleeping for some milliseconds:

```javascript
await new Promise(r => setTimeout(r, 10));
```

In our tests, adding this timeout was sufficient to give time to the `FinalizationRegistry` to properly clean the objects. The external memory reached a peak, and it stays steady.

In an interactive environment, like a browser, it should not be necessary to add such a pause, as the engine has several opportunities to do the clean-up while waiting some input from the user.

An important point to mention is that the memory used in Wasm never shrinks back after being cleaned. This is [expected by design](https://github.com/WebAssembly/design/issues/1300#issuecomment-573867836).

> 🚧 **Warning**
>
> Garbage collection and finalizers are not deterministic by design, so a test in one machine and one JavaScript engine might differ greatly on a different machine or a different engine. In our tests, a simple timeout of 10ms was enough for the collector to collect and call the finalizers, but your experience might be different.
