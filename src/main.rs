extern crate wasmer_runtime;

use wasmer_runtime::{error, Func, func, imports, instantiate, Array, Ctx, WasmPtr};

static WASM: &'static [u8] = include_bytes!("../wasm/build/optimized.wasm");

fn main() -> error::Result<()> {
    // Assemblyscript requires an `env::abort(i32, i32, i32, i32)` function defined
    //      see: https://www.assemblyscript.org/debugging.html#overriding-abort
    let abort = |_: i32, _: i32, _: i32, _: i32| std::process::exit(-1);
    let log = move |ctx: &mut Ctx, ptr: WasmPtr<u8, Array>, len: u32| {
        let memory = ctx.memory(0);

        // Use helper method on `WasmPtr` to read a utf8 string
        let string = ptr.get_utf8_string(memory, len * 2).unwrap();

        // Print it!
        println!("log: {}", string);
    };
    let import_object = imports! {
        "env" => {
            "abort" => func!(abort),
        },
        "host" => {
            "host.log" => func!(log),
        }
    };

    let instance = instantiate(WASM, &import_object)?;

    let start_fn: Func<(), i32> = instance.exports.get("start")?;
    let exit_code = start_fn.call()?;

    println!("WASM exited with code {}", exit_code);
    Ok(())
}