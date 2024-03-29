use std::fmt::{self, Debug};
mod io;
pub mod properties;

const PKG_NAME: &str = env!("CARGO_PKG_NAME");
const PKG_VERSION: &str = env!("CARGO_PKG_VERSION");

/** The GraphFile struct represents a graph in the gt file format.
 *
 *  see https://graph-tool.skewed.de/static/doc/gt_format.html
 *
 * The file format is as follows in bytes:
 *  [0:5] magic string: 0xe2, 0x9b, 0xbe, 0x20, 0x67, 0x74
 *  [6] version number: 0x01
 *  [7] endianness: 0x00
 *  [8:15] comment length (c): u64
 *  [16:16+c] comment: String
 *  [17+c] directed: bool
 *  [18+c:25+c] number of vertices (n): u64
 *  [26+c:26+c+8*n=t]  list of out-neighbors of all N vertices in sequence (u64 * n)
 *  [t:t+8] number of properties (p): u64
 *     -  for each property:
 *     - [t+8] key type: u8 (0x00 = graph prop, 0x01 = vertex prop, 0x02 = edge prop)
 *     - [t+9] size of name (s): u64
 *     - [t+10:t+10+s] name: String
 *     - [t+10+s] value type index: u8 (0x00 = bool, 0x01 = int, 0x02 = double, 0x03 = string)
 *
 *  
 */
pub struct GraphFile {
    version_number: u8,
    endianness: u8,

    comment: String,
    pub directed: bool,
    pub num_vertices: u64,
    pub num_edges: u64,
    pub out_neighbors: Vec<Vec<u64>>,

    // Property maps
    pub properties: Vec<properties::Property>,
}

impl Default for GraphFile {
    fn default() -> Self {
        let comment = format!(
            "graph-tool binary file (http:://graph-tool.skewed.de) generated by {} v{}",
            PKG_NAME, PKG_VERSION
        );

        GraphFile {
            version_number: 1,
            endianness: 0,
            comment: comment,
            directed: false,
            num_vertices: 0,
            num_edges: 0,
            out_neighbors: Vec::new(),
            properties: Vec::new(),
        }
    }
}

impl Debug for GraphFile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("GraphFile")
            .field("version_number", &self.version_number)
            .field("endianness", &self.endianness)
            .field("comment", &self.comment)
            .field("directed", &self.directed)
            .field("num_vertices", &self.num_vertices)
            .field("num_edges", &self.num_edges)
            .field("properties", &self.properties)
            .finish()
    }
}
