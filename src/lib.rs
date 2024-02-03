#[macro_use]
mod utils;

/** The Graph struct represents a graph and provides a number
 * of methods to access the data.
 *
 * We supply a number of wrappers for interacting with the graph
 * from JavaScript.
 */
mod graph;
pub use graph::Graph;
mod graph_js;
pub use graph_js::GraphJS;

/** Decoding for compress data
 *
 * at the moment only zstd is supported
 */
pub mod decode;

pub mod graph_file;

pub use graph_file::GraphFile;
mod io;

use utils::set_panic_hook;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);

    // Use `js_namespace` here to bind `console.log(..)` instead of just
    // `log(..)`
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);

    // The `console.log` is quite polymorphic, so we can bind it with multiple
    // signatures. Note that we need to use `js_name` to ensure we always call
    // `log` in JS.
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_u32(a: u32);

    // Multiple arguments too!
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_many(a: &str, b: &str);
}

#[wasm_bindgen(start)]
// cant be named main because of wasm-bindgen-test
pub fn run() {
    set_panic_hook();
}
