use wasm_bindgen::prelude::*;
pub mod io;

use crate::graph_file::{properties::PropertyMapType, GraphFile};

/** The Graph struct represents a graph and provides a number
 * of methods to access the data.
 */
#[wasm_bindgen]
pub struct Graph {
    file: GraphFile,
}

trait GraphPropertiesTrait {
    /// Get vec of graph properties
    fn graph_properties(&self) -> js_sys::Map;
    /// Get vec of node properties
    fn node_properties(&self) -> js_sys::Map;
    /// Get vec of edge properties
    fn edge_properties(&self) -> js_sys::Map;
}

#[wasm_bindgen]
impl Graph {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Graph {
        Graph {
            file: GraphFile::default(),
        }
    }

    #[wasm_bindgen(getter)]
    pub fn num_nodes(&self) -> u64 {
        self.file.num_nodes
    }

    #[wasm_bindgen(getter)]
    pub fn num_edges(&self) -> u64 {
        self.file.num_edges
    }

    #[wasm_bindgen(getter)]
    pub fn directed(&self) -> bool {
        self.file.directed
    }

    pub fn get_out_neighbors(&self, node: u64) -> js_sys::BigUint64Array {
        let ret = js_sys::BigUint64Array::from(self.file.out_neighbors[node as usize].as_slice());
        ret
    }

    pub fn get_in_neighbors(&self, node: u64) -> js_sys::BigUint64Array {
        let mut in_neighbors: Vec<u64> = Vec::new();
        for (i, neighbors) in self.file.out_neighbors.iter().enumerate() {
            if neighbors.contains(&node) {
                in_neighbors.push(i as u64);
            }
        }
        let ret = js_sys::BigUint64Array::from(in_neighbors.as_slice());
        ret
    }

    pub fn graph_properties(&mut self, name: String) -> JsValue {
        let mut ret = JsValue::NULL;
        for property in &mut self.file.properties {
            if property.name == name {
                match property.map_type {
                    PropertyMapType::Graph => {
                        ret = property.data_view();
                        break;
                    }
                    _ => {}
                }
            }
        }
        ret
    }

    pub fn vertex_properties(&mut self, name: String) -> JsValue {
        let mut ret = JsValue::NULL;
        for property in &mut self.file.properties {
            if property.name == name {
                match property.map_type {
                    PropertyMapType::Vertex => {
                        ret = property.data_view();
                        break;
                    }
                    _ => {}
                }
            }
        }
        ret
    }

    pub fn edge_properties(&mut self, name: String) -> JsValue {
        let mut ret = JsValue::NULL;
        for property in &mut self.file.properties {
            if property.name == name {
                match property.map_type {
                    PropertyMapType::Edge => {
                        ret = property.data_view();
                        break;
                    }
                    _ => {}
                }
            }
        }
        ret
    }

    pub fn get_vertex_property_names(&self) -> js_sys::Array {
        let ret = js_sys::Array::new();
        for property in &self.file.properties {
            match property.map_type {
                PropertyMapType::Vertex => {
                    ret.push(&JsValue::from_str(&property.name));
                }
                _ => {}
            }
        }
        ret
    }

    pub fn get_edge_property_names(&self) -> js_sys::Array {
        let ret = js_sys::Array::new();
        for property in &self.file.properties {
            match property.map_type {
                PropertyMapType::Edge => {
                    ret.push(&JsValue::from_str(&property.name));
                }
                _ => {}
            }
        }
        ret
    }

    pub fn get_graph_property_names(&self) -> js_sys::Array {
        let ret = js_sys::Array::new();
        for property in &self.file.properties {
            match property.map_type {
                PropertyMapType::Graph => {
                    ret.push(&JsValue::from_str(&property.name));
                }
                _ => {}
            }
        }
        ret
    }
}

impl Default for Graph {
    fn default() -> Self {
        Graph {
            file: GraphFile::default(),
        }
    }
}
