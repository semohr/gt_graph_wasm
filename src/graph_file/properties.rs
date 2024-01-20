extern crate byteorder;

use std::io::Read;

use byteorder::{LittleEndian, ReadBytesExt};
use wasm_bindgen::JsValue;

pub enum PropertyMapType {
    Graph,
    Vertex,
    Edge,
}

impl TryFrom<u8> for PropertyMapType {
    type Error = JsValue;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x00 => Ok(PropertyMapType::Graph),
            0x01 => Ok(PropertyMapType::Vertex),
            0x02 => Ok(PropertyMapType::Edge),
            _ => Err("Property map type not supported".into()),
        }
    }
}

// No idea if the following is the most elegant way but it works
// There might be a way to do the same with generics in less lines
enum PropertyType {
    Bool,
    Int16,
    Int32,
    Int64,
    Double,
    LongDouble,
    String,
    VectorBool,
    VectorInt16,
    VectorInt32,
    VectorInt64,
    VectorDouble,
    VectorLongDouble,
    VectorString,
    PyObject,
}

enum PropertyData {
    Bool(bool),
    Int16(i16),
    Int32(i32),
    Int64(i64),
    Double(f64),
    LongDouble(u128),
    String(String),
    VectorBool(Vec<bool>),
    VectorInt16(Vec<i16>),
    VectorInt32(Vec<i32>),
    VectorInt64(Vec<i64>),
    VectorDouble(Vec<f64>),
    VectorLongDouble(Vec<u128>), //There is no long double in rust
    VectorString(Vec<String>),
    PyObject(Vec<u8>),
}

pub struct Property {
    name: String,
    r#type: PropertyType,
    data: Vec<PropertyData>,
}

impl Property {
    /** Create a property from data
     * given a cursor and the length of the property
     */
    pub fn from_data(
        cursor: &mut std::io::Cursor<&[u8]>,
        num_nodes: u64,
        num_edges: u64,
    ) -> Result<Property, JsValue> {
        // Property map type
        let property_map_type = cursor.read_u8().unwrap();
        let property_map_type = PropertyMapType::try_from(property_map_type)?;

        let length = match property_map_type {
            PropertyMapType::Graph => 1,
            PropertyMapType::Vertex => num_nodes,
            PropertyMapType::Edge => num_edges,
        };

        // Property name
        let name_size = cursor.read_u64::<LittleEndian>().unwrap();
        let mut name = vec![0; name_size as usize];
        cursor.read_exact(&mut name).unwrap();
        let name = String::from_utf8_lossy(&name).to_string();

        // Property type
        let property_type = cursor.read_u8().unwrap();
        let property_type = PropertyType::try_from(property_type)?;

        let property_data: Vec<PropertyData> = (0..length)
            .into_iter()
            .map(|_| {
                match property_type {
                    PropertyType::Bool => {
                        // read the bool
                        let bool = cursor.read_u8().unwrap();
                        // Convert to bool
                        let bool = match bool {
                            0x00 => false,
                            0x01 => true,
                            _ => panic!("Invalid bool value"),
                        };
                        PropertyData::Bool(bool)
                    }
                    PropertyType::Int16 => {
                        let int16 = cursor.read_i16::<LittleEndian>().unwrap();
                        PropertyData::Int16(int16)
                    }
                    PropertyType::Int32 => {
                        let int32 = cursor.read_i32::<LittleEndian>().unwrap();
                        PropertyData::Int32(int32)
                    }
                    PropertyType::Int64 => {
                        let int64 = cursor.read_i64::<LittleEndian>().unwrap();
                        PropertyData::Int64(int64)
                    }
                    PropertyType::Double => {
                        let double = cursor.read_f64::<LittleEndian>().unwrap();
                        PropertyData::Double(double)
                    }
                    PropertyType::LongDouble => {
                        let long_double = cursor.read_u128::<LittleEndian>().unwrap();
                        PropertyData::LongDouble(long_double)
                    }
                    PropertyType::String => {
                        let string_length = cursor.read_u64::<LittleEndian>().unwrap();
                        let mut string = vec![0; string_length as usize];
                        cursor.read_exact(&mut string).unwrap();
                        let string = String::from_utf8_lossy(&string).to_string();
                        PropertyData::String(string)
                    }
                    PropertyType::VectorBool => {
                        let vector_length = cursor.read_u64::<LittleEndian>().unwrap();
                        let mut vector = Vec::with_capacity(vector_length as usize);
                        for _ in 0..vector_length {
                            let bool = cursor.read_u8().unwrap();
                            let bool = match bool {
                                0x00 => false,
                                0x01 => true,
                                _ => panic!("Invalid bool value"),
                            };
                            vector.push(bool);
                        }
                        PropertyData::VectorBool(vector)
                    }
                    PropertyType::VectorInt16 => {
                        let vector_length = cursor.read_u64::<LittleEndian>().unwrap();
                        let mut vector = Vec::with_capacity(vector_length as usize);
                        for _ in 0..vector_length {
                            let int16 = cursor.read_i16::<LittleEndian>().unwrap();
                            vector.push(int16);
                        }
                        PropertyData::VectorInt16(vector)
                    }
                    PropertyType::VectorInt32 => {
                        let vector_length = cursor.read_u64::<LittleEndian>().unwrap();
                        let mut vector = Vec::with_capacity(vector_length as usize);
                        for _ in 0..vector_length {
                            let int32 = cursor.read_i32::<LittleEndian>().unwrap();
                            vector.push(int32);
                        }
                        PropertyData::VectorInt32(vector)
                    }
                    PropertyType::VectorInt64 => {
                        let vector_length = cursor.read_u64::<LittleEndian>().unwrap();
                        let mut vector = Vec::with_capacity(vector_length as usize);
                        for _ in 0..vector_length {
                            let int64 = cursor.read_i64::<LittleEndian>().unwrap();
                            vector.push(int64);
                        }
                        PropertyData::VectorInt64(vector)
                    }
                    PropertyType::VectorDouble => {
                        let vector_length = cursor.read_u64::<LittleEndian>().unwrap();
                        let mut vector = Vec::with_capacity(vector_length as usize);
                        for _ in 0..vector_length {
                            let double = cursor.read_f64::<LittleEndian>().unwrap();
                            vector.push(double);
                        }
                        PropertyData::VectorDouble(vector)
                    }
                    PropertyType::VectorLongDouble => {
                        let vector_length = cursor.read_u64::<LittleEndian>().unwrap();
                        let mut vector = Vec::with_capacity(vector_length as usize);
                        for _ in 0..vector_length {
                            let long_double = cursor.read_u128::<LittleEndian>().unwrap();
                            vector.push(long_double);
                        }
                        PropertyData::VectorLongDouble(vector)
                    }
                    PropertyType::VectorString => {
                        let vector_length = cursor.read_u64::<LittleEndian>().unwrap();
                        let mut vector = Vec::with_capacity(vector_length as usize);
                        for _ in 0..vector_length {
                            let string_length = cursor.read_u64::<LittleEndian>().unwrap();
                            let mut string = vec![0; string_length as usize];
                            cursor.read_exact(&mut string).unwrap();
                            let string = String::from_utf8_lossy(&string).to_string();
                            vector.push(string);
                        }
                        PropertyData::VectorString(vector)
                    }
                    PropertyType::PyObject => {
                        let object_length = cursor.read_u64::<LittleEndian>().unwrap();
                        let mut object = vec![0; object_length as usize];
                        cursor.read_exact(&mut object).unwrap();
                        PropertyData::PyObject(object)
                    }
                }
            })
            .collect::<Vec<PropertyData>>();

        // Read the size of the property
        // This depends further on the type of the property and the length
        Ok(Property {
            name,
            r#type: property_type,
            data: property_data,
        })
    }
}

impl TryFrom<u8> for PropertyType {
    type Error = JsValue;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x00 => Ok(PropertyType::Bool),
            0x01 => Ok(PropertyType::Int16),
            0x02 => Ok(PropertyType::Int32),
            0x03 => Ok(PropertyType::Int64),
            0x04 => Ok(PropertyType::Double),
            0x05 => Ok(PropertyType::LongDouble),
            0x06 => Ok(PropertyType::String),
            0x07 => Ok(PropertyType::VectorBool),
            0x08 => Ok(PropertyType::VectorInt16),
            0x09 => Ok(PropertyType::VectorInt32),
            0x0a => Ok(PropertyType::VectorInt64),
            0x0b => Ok(PropertyType::VectorDouble),
            0x0c => Ok(PropertyType::VectorLongDouble),
            0x0d => Ok(PropertyType::VectorString),
            0x0e => Ok(PropertyType::PyObject),
            _ => Err("Property type not supported".into()),
        }
    }
}
