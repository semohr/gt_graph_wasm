use crate::{Graph, GraphFile};

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
/// Returns an error if the data could not be decoded or the graph could not be created.
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
    type Error = String;

    fn try_from(data: Vec<u8>) -> Result<Self, Self::Error> {
        let data = crate::decode::decodebuffer(&data)?;
        let graph_file: GraphFile = data.as_slice().try_into()?;
        Ok(Graph { file: graph_file })
    }
}

/// Implements the `From` trait for `Graph` from a `GraphFile`.
impl From<GraphFile> for Graph {
    fn from(file: GraphFile) -> Self {
        Graph { file }
    }
}
