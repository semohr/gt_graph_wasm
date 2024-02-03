pub mod io;
pub mod properties;

use crate::graph_file::GraphFile;

/** The Graph struct represents a graph and provides a number
 * of methods to access the data.
 */

pub struct Graph {
    file: GraphFile,
}

impl Graph {
    pub fn new() -> Graph {
        Graph {
            file: GraphFile::default(),
        }
    }

    pub fn num_vertices(&self) -> u64 {
        self.file.num_vertices
    }

    pub fn num_edges(&self) -> u64 {
        self.file.num_edges
    }

    pub fn directed(&self) -> bool {
        self.file.directed
    }

    pub fn vertices(&self) -> Vec<u64> {
        (0..self.file.num_vertices).collect()
    }

    pub fn edges(&self) -> (Vec<u64>, Vec<u64>) {
        //from to pairs
        self.file
            .out_neighbors
            .iter()
            .enumerate()
            .flat_map(|(from, to)| to.iter().map(move |&to| (from as u64, to)))
            .unzip()
    }

    /// Get the out neighbors of a node
    ///
    /// # Arguments
    ///
    /// * `node` - The node for which to get the out neighbors
    ///
    /// # Example
    ///
    /// ```
    /// use gt_graph_wasm::graph::Graph;
    ///
    /// let graph = Graph::new();
    /// let out_neighbors = graph.out_neighbors(0);
    /// ```
    pub fn out_neighbors(&self, node: u64) -> &[u64] {
        self.file.out_neighbors[node as usize].as_slice()
    }

    /// Get the in neighbors of a node
    ///
    /// # Arguments
    ///
    /// * `node` - The node for which to get the in neighbors
    ///
    /// # Example
    ///
    /// ```
    /// use gt_graph_wasm::graph::Graph;
    ///
    /// let graph = Graph::new();
    /// let in_neighbors = graph.in_neighbors(0);
    /// ```
    pub fn in_neighbors(&self, node: u64) -> Vec<u64> {
        let mut in_neighbors: Vec<u64> = Vec::new();
        for (i, neighbors) in self.file.out_neighbors.iter().enumerate() {
            if neighbors.contains(&node) {
                in_neighbors.push(i as u64);
            }
        }
        in_neighbors
    }
}

impl Default for Graph {
    fn default() -> Self {
        Graph {
            file: GraphFile::default(),
        }
    }
}
