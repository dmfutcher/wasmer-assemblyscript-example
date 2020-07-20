# wasmer-assemblyscript-example

An example of using [Wasmer](http://wasmer.io/) to run [Assemblyscript](https://www.assemblyscript.org/) generated 
[Webassembly](https://webassembly.org/) inside a [Rust](https://www.rust-lang.org/) program.

# How to Run

You'll need Rust and Node installed on your machine.

## Build the WASM

To build the WASM, which we'll run inside the host program:

```
cd wasm

npm run asbuild:optimized
```

This will compile the Assemblyscript module down to WASM, resulting in a `build/optimized.wasm` file.

## Build the Host

Now we've built the WASM, we need to build our Rust host. In the repo root run:

`cargo build`

This will install the Wasmer library (WASM runtime) and build our WASM executor.

## Run it

`./target/debug/wasmer-assemblyscript-example`

🚀 You should see our WASM code logging 'Hello, World!' then exiting happily.


# Extending it

This is a blueprint for embedding a WASM interpreter in your program. It doesn't do anything interesting yet, but it's an easy starting point for your WASM based projects.

* Replace Rust: Wasmer supports a whole heap of host languages, so you can run WASM with whatever program/language you want
* Replace Assemblyscript: As we compile our module down to WASM to run it, you can write your module in any language with a WASM backend (which is most of them by now ...)
* Extend imports: Right now our host exposes just a couple of functions to the loaded WASM, so we can't do much very interesting work. Add extra functions to the `import_object` variable to let your WASM do something more interesting.

For brevity & simplicity, the generated WASM is currently statically linked to the host at build time (using `include_bytes!`). So every time you change your WASM you'll need to re-build it and re-build the host. Which isn't ideal. You'll likely want to replace that with a file loader before you get too far.
