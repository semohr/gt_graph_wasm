use wasm_bindgen::prelude::*;

use crate::{Graph, GraphFile};

#[wasm_bindgen]
impl Graph {
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
    /// use gt_graph_wasm::Graph;
    /// let url = "https://example.com/graph.gt";
    /// let graph = Graph::from_url(url.to_string()).await;
    /// ```
    pub async fn from_url(url: String) -> Result<Graph, JsValue> {
        let data = crate::io::fetch_binary(url).await?;
        let data = crate::decode::decodebuffer(&data)?;
        let graph_file: GraphFile = data.as_slice().try_into()?;
        Ok(Graph { file: graph_file })
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
    /// let graph = Graph::from_netzschleuder(network.to_string(), sub_network).await;
    /// ```
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
    pub fn from_data(data: js_sys::Uint8Array) -> Result<Graph, JsValue> {
        let data = data.to_vec();
        Self::try_from(data)
    }
}

/// Implements the `TryFrom` trait for `Graph` from a `Vec<u8>`.
///
/// This conversion allows creating a `Graph` from a byte vector, decoding the data
/// and constructing a `Graph` object from it.
///
/// # Arguments
///
/// * `data` - The byte vector containing the encoded graph data.
///
/// # Errors
///
/// Returns a `JsValue` error if the decoding or conversion fails.
///
/// # Examples
///
/// ```
/// use std::convert::TryFrom;
/// use wasm_bindgen::JsValue;
/// use crate::graph::Graph;
///
/// let data: Vec<u8> = vec![/* encoded graph data */];
/// let graph_result = Graph::try_from(data);
///
/// match graph_result {
///     Ok(graph) => {
///         // Successfully created a graph from the data
///         // ...
///     }
///     Err(error) => {
///         // Failed to create a graph from the data
///         // Handle the error
///         // ...
///     }
/// }
/// ```
impl TryFrom<Vec<u8>> for Graph {
    type Error = JsValue;

    fn try_from(data: Vec<u8>) -> Result<Self, Self::Error> {
        let data = crate::decode::decodebuffer(&data)?;
        let graph_file: GraphFile = data.as_slice().try_into()?;
        Ok(Graph { file: graph_file })
    }
}
