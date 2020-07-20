extern crate wasmer_runtime;

// use std::sync::{Arc, Mutex};
use wasmer_runtime::{error, Func, func, imports, instantiate};

static WASM: &'static [u8] = include_bytes!("../belka/build/optimized.wasm");

fn main() -> error::Result<()> {
    // Assemblyscript requires an `env::abort(i32, i32, i32, i32)` function defined
    //      see: https://www.assemblyscript.org/debugging.html#overriding-abort
    let abort = |_: i32, _: i32, _: i32, _: i32| std::process::exit(-1);
    let import_object = imports! {
        "env" => {
            "abort" => func!(abort),
        },
    };

    let instance = instantiate(WASM, &import_object)?;

    let add_one: Func<i32, i32> = instance.exports.get("add_one")?;

    let value = add_one.call(42)?;

    assert_eq!(value, 43);

    Ok(())
}