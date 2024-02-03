use wasm_bindgen::prelude::*;

use crate::GraphJS;

#[wasm_bindgen]
// Traits are not supported in wasm-bindgen
impl GraphJS {
    /// Get a list of all graph property names
    #[wasm_bindgen(js_name = get_graph_property_names, getter)]
    pub fn graph_property_names(&self) -> js_sys::Array {
        let names = self.graph.graph_property_names();
        let ret = js_sys::Array::new();
        for name in names {
            ret.push(&JsValue::from_str(&name));
        }
        ret
    }

    /// Get a list of all vertex property names
    #[wasm_bindgen(js_name = get_vertex_property_names, getter)]
    pub fn vertex_property_names(&self) -> js_sys::Array {
        let names = self.graph.vertex_property_names();
        let ret = js_sys::Array::new();
        for name in names {
            ret.push(&JsValue::from_str(&name));
        }
        ret
    }

    /// Get a list of all edge property names
    #[wasm_bindgen(js_name = get_edge_property_names, getter)]
    pub fn edge_property_names(&self) -> js_sys::Array {
        let names = self.graph.edge_property_names();
        let ret = js_sys::Array::new();
        for name in names {
            ret.push(&JsValue::from_str(&name));
        }
        ret
    }

    /// Get a graph property by its name
    #[wasm_bindgen(js_name = get_graph_property)]
    pub fn graph_properties(&mut self, name: String) -> Result<JsValue, JsValue> {
        let property = self.graph.graph_property(name);
        match property {
            Ok(property) => Ok(property.data_view()),
            Err(err) => Err(JsValue::from_str(&err)),
        }
    }

    /// Get a vertex property by its name
    #[wasm_bindgen(js_name = get_vertex_property)]
    pub fn vertex_properties(&mut self, name: String) -> Result<JsValue, JsValue> {
        let property = self.graph.vertex_property(name);
        match property {
            Ok(property) => Ok(property.data_view()),
            Err(err) => Err(JsValue::from_str(&err)),
        }
    }

    /// Get an edge property by its name
    #[wasm_bindgen(js_name = get_edge_property)]
    pub fn edge_properties(&mut self, name: String) -> Result<JsValue, JsValue> {
        let property = self.graph.edge_property(name);
        match property {
            Ok(property) => Ok(property.data_view()),
            Err(err) => Err(JsValue::from_str(&err)),
        }
    }
}
