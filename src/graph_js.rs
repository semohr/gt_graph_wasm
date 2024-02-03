use wasm_bindgen::prelude::*;

/** Defines a number of input
 * and output methods for the graph.
 * To be used by javascript.
 */
pub mod io;

/** Defines all traits for receiving properties
 * as JavaScript objects.
 */
pub mod properties;

use crate::graph::Graph;

/** The GraphJS struct represents a graph in JavaScript
 * and provides a number of methods to access the data.
 *
 * For rust internal use see the Graph struct.
 */
#[wasm_bindgen(js_name = Graph)]
pub struct GraphJS {
    graph: Graph,
}

/** JavaScript methods for the GraphJS struct */
#[wasm_bindgen(js_class = Graph)]
impl GraphJS {
    #[wasm_bindgen(constructor)]
    pub fn new() -> GraphJS {
        GraphJS::default()
    }

    #[wasm_bindgen(getter)]
    pub fn num_vertices(&self) -> u64 {
        self.graph.num_vertices()
    }

    pub fn vertices(&self) -> js_sys::BigUint64Array {
        let ret = js_sys::BigUint64Array::from(self.graph.vertices().as_slice());
        ret
    }

    #[wasm_bindgen(getter)]
    pub fn num_edges(&self) -> u64 {
        self.graph.num_edges()
    }

    pub fn edges(&self) -> js_sys::Array {
        let (i, o) = self.graph.edges();
        let ret = js_sys::Array::new();
        let from = js_sys::BigUint64Array::from(i.as_slice());
        let to = js_sys::BigUint64Array::from(o.as_slice());
        ret.push(&from);
        ret.push(&to);
        ret
    }

    #[wasm_bindgen(getter)]
    pub fn directed(&self) -> bool {
        self.graph.directed()
    }

    pub fn out_neighbors(&self, node: u64) -> js_sys::BigUint64Array {
        let ret = js_sys::BigUint64Array::from(self.graph.out_neighbors(node));
        ret
    }

    pub fn in_neighbors(&self, node: u64) -> js_sys::BigUint64Array {
        let ret = js_sys::BigUint64Array::from(self.graph.in_neighbors(node).as_slice());
        ret
    }
}

impl Default for GraphJS {
    fn default() -> Self {
        GraphJS {
            graph: Graph::new(),
        }
    }
}
