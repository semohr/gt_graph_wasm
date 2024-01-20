use wasm_bindgen::prelude::*;

use crate::decode;
use crate::io;
use crate::GraphFile::GraphFile;

/** This is a wrapper around the GraphFile class
 * and allows for a number of graph operations.
 */
#[wasm_bindgen]
pub struct Graph {
    file: GraphFile,
}

#[wasm_bindgen]
impl Graph {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Graph {
        Graph {
            file: GraphFile::default(),
        }
    }

    /** Create a graph from a url. The url should point
     * to a binary file in the gt format. Can be compressed
     * with zsdt.
     */
    pub async fn from_url(url: String) -> Result<Graph, JsValue> {
        let data = io::fetch_binary(url).await?;
        let data = decode::decodebuffer(&data)?;
        let graph_file: GraphFile = data.as_slice().try_into()?;
        Ok(Graph { file: graph_file })
    }
}

impl Default for Graph {
    fn default() -> Self {
        Graph {
            file: GraphFile::default(),
        }
    }
}
