use wasm_bindgen::prelude::*;

use crate::decode;
use crate::graph_file::GraphFile;
use crate::io;

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

    /** Create a graph from the netzschleuder repository.
     *
     *  https://networks.skewed.de
     */
    pub async fn from_netzschleuder(
        network: String,
        sub_network: Option<String>,
    ) -> Result<Graph, JsValue> {
        let sub_network = match sub_network {
            Some(sub_network) => sub_network,
            None => network.clone(),
        };

        let url = format!(
            "https://networks.skewed.de/net/{}/files/{}.gt.zst",
            network, sub_network
        );
        Graph::from_url(url).await
    }

    /** Create a graph from a binary file.
     */
    pub fn from_data(data: js_sys::Uint8Array) -> Result<Graph, JsValue> {
        let data = data.to_vec();
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
