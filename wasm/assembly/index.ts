// The entry file of your WebAssembly module.

// Import our host-exposed `log` function
import { log } from './host';

// Export a `start()` function, which our Rust/Wasmer host calls
export function start(): i32 {
  log("Hello, World!");
  return 0;
}
