use std::io::Read;

use super::GraphFile;
use crate::graph_file::properties::Property;
use byteorder::{LittleEndian, ReadBytesExt};

impl TryFrom<&[u8]> for GraphFile {
    type Error = String;

    fn try_from(file: &[u8]) -> Result<Self, Self::Error> {
        let mut cursor = std::io::Cursor::new(file);
        let magic_string = &[0xe2, 0x9b, 0xbe, 0x20, 0x67, 0x74];
        if file.len() < 14 {
            return Err("file is too short".into());
        }

        // Read magic string with cursor
        let mut magic = vec![0; 6];
        cursor.read_exact(&mut magic).unwrap();
        if &magic[0..6] != magic_string {
            return Err("Invalid file header".into());
        }

        // Read version number
        let version_number = cursor.read_u8().unwrap();
        if version_number != 0x01 {
            return Err("Invalid file header: version not supported (yet)".into());
        }

        // Read endianness
        let endianness = cursor.read_u8().unwrap();
        if endianness != 0x00 {
            return Err("Invalid file header: only little endianess  supported (yet)".into());
        }

        // Read comment
        let c_len = cursor.read_u64::<LittleEndian>().unwrap() as usize;
        let mut comment = vec![0; c_len];
        cursor.read_exact(&mut comment).unwrap();
        let comment = String::from_utf8_lossy(&comment).to_string();

        // Read directed
        let directed = cursor.read_u8().unwrap() == 0x01;

        // Read number of vertices
        let num_vertices = cursor.read_u64::<LittleEndian>().unwrap();

        // Read neighbor list
        let out_neighbors = get_out_neighbors(&mut cursor, num_vertices);

        // Calculate number of edges
        let num_edges: u64 = out_neighbors
            .iter()
            .map(|v| v.len())
            .sum::<usize>()
            .try_into()
            .unwrap();

        // Parse properties
        let num_properties = cursor.read_u64::<LittleEndian>().unwrap();

        let properties: Vec<Property> = (0..num_properties)
            .into_iter()
            .map(|_| Property::from_data(&mut cursor, num_vertices, num_edges))
            .collect::<Result<Vec<Property>, String>>()?;

        let gf = GraphFile {
            version_number: version_number,
            endianness: endianness,
            comment: comment,
            directed: directed,
            num_vertices: num_vertices,
            num_edges: num_edges,
            out_neighbors: out_neighbors,
            properties: properties,
        };

        Ok(gf)
    }
}

fn get_out_neighbors(cursor: &mut std::io::Cursor<&[u8]>, num_vertices: u64) -> Vec<Vec<u64>> {
    let s_t;
    match num_vertices {
        n if n <= u8::MAX as u64 => {
            s_t = std::mem::size_of::<u8>();
        }
        n if n <= u16::MAX as u64 => {
            s_t = std::mem::size_of::<u16>();
        }
        n if n <= u32::MAX as u64 => {
            s_t = std::mem::size_of::<u32>();
        }
        _ => {
            s_t = std::mem::size_of::<u64>();
        }
    }

    let mut out_neighbors: Vec<Vec<u64>> = Vec::new();

    for _ in 0..num_vertices {
        // Get num neighbors for each node
        let num_neighbors = cursor.read_u64::<LittleEndian>().unwrap() as usize;

        // Get neighbors
        let mut neighbors = Vec::with_capacity(num_neighbors);
        for _ in 0..num_neighbors {
            let neighbor = match s_t {
                1 => cursor.read_u8().unwrap() as u64,
                2 => cursor.read_u16::<LittleEndian>().unwrap() as u64,
                4 => cursor.read_u32::<LittleEndian>().unwrap() as u64,
                8 => cursor.read_u64::<LittleEndian>().unwrap() as u64,
                _ => panic!("Invalid size of type"),
            };
            neighbors.push(neighbor);
        }

        out_neighbors.push(neighbors);
    }

    out_neighbors
}
