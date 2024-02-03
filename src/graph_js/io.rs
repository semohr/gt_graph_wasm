use wasm_bindgen::prelude::*;

use crate::graph_file::GraphFile;
use crate::{Graph, GraphJS};

#[wasm_bindgen(js_class = Graph)]
impl GraphJS {
    /// Create a graph from a URL.
    ///
    /// The URL should point to a binary file in the gt format. It can be compressed with zsdt.
    ///
    /// # Arguments
    ///
    /// * `url` - The URL of the binary file.
    ///
    /// # Examples
    ///
    /// ```
    /// use gt_graph_wasm::GraphJS;
    /// let url = "https://example.com/graph.gt";
    /// let graph = GraphJS::from_url(url.to_string()).await;
    /// ```
    pub async fn from_url(url: String) -> Result<GraphJS, JsValue> {
        let data = crate::io::fetch_binary(url).await?;
        let data = crate::decode::decodebuffer(&data)?;
        let graph_file: GraphFile = data.as_slice().try_into()?;
        let graph: GraphJS = graph_file.into();
        Ok(graph)
    }

    /// Create a graph from the netzschleuder repository.
    ///
    /// The graph is fetched from the [netzschleuder repository](https://networks.skewed.de).
    ///
    /// # Arguments
    ///
    /// * `network` - The name of the network.
    /// * `sub_network` - Optional sub-network name. If not provided, the network name will be used.
    ///
    /// # Examples
    ///
    /// ```
    /// use gt_graph_wasm::Graph;
    /// let network = "karate";
    /// let sub_network = Some("karate_club".to_string());
    /// let graph = GraphJS::from_netzschleuder(network.to_string(), sub_network).await;
    /// ```
    pub async fn from_netzschleuder(
        network: String,
        sub_network: Option<String>,
    ) -> Result<GraphJS, JsValue> {
        let sub_network = match sub_network {
            Some(sub_network) => sub_network,
            None => network.clone(),
        };

        let url = format!(
            "https://networks.skewed.de/net/{}/files/{}.gt.zst",
            network, sub_network
        );
        GraphJS::from_url(url).await
    }

    /// Create a graph from a binary file directly by passing a javascript Uint8Array.
    ///
    /// This is a binding to the `Graph::try_from` method for javascript.
    ///
    /// # Arguments
    ///
    /// * `data` - The binary data of the graph file.
    ///
    /// # Examples
    ///
    /// ```
    /// use gt_graph_wasm::graph::Graph;
    /// // Load data
    /// let data = include_bytes!("../tests/data/advogato.gt.zst");
    /// let graph = Graph::from_data(data).unwrap();
    /// ```
    pub fn from_data(data: js_sys::Uint8Array) -> Result<GraphJS, JsValue> {
        let data = data.to_vec();
        let graph = Graph::try_from(data)?;
        Ok(graph.into())
    }
}

impl From<Graph> for GraphJS {
    fn from(graph: Graph) -> Self {
        GraphJS { graph }
    }
}

impl From<GraphFile> for GraphJS {
    fn from(file: GraphFile) -> Self {
        GraphJS {
            graph: Graph::from(file),
        }
    }
}

impl TryFrom<Vec<u8>> for GraphJS {
    type Error = JsValue;

    fn try_from(data: Vec<u8>) -> Result<Self, Self::Error> {
        let graph = Graph::try_from(data)?;
        Ok(graph.into())
    }
}
