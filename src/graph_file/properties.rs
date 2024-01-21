extern crate byteorder;
use std::fmt::{self, Debug};
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

impl Debug for PropertyMapType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PropertyMapType::Graph => write!(f, "Graph"),
            PropertyMapType::Vertex => write!(f, "Vertex"),
            PropertyMapType::Edge => write!(f, "Edge"),
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

/** By default the values are allows
 * a vector even if it is a graph property
 * (length 1)
 */
enum PropertyData {
    Bool(Vec<u8>), //no bool vec in js
    Int16(Vec<i16>),
    Int32(Vec<i32>),
    Int64(Vec<i64>),
    Double(Vec<f64>),
    LongDouble(Vec<f64>), //no long double in js nor rust
    String(Vec<String>),
    VectorBool(Vec<Vec<u8>>),
    VectorInt16(Vec<Vec<i16>>),
    VectorInt32(Vec<Vec<i32>>),
    VectorInt64(Vec<Vec<i64>>),
    VectorDouble(Vec<Vec<f64>>),
    VectorLongDouble(Vec<Vec<f64>>), //There is no long double in js nor rust
    VectorString(Vec<Vec<String>>),
    PyObject(Vec<Vec<u8>>),
}

pub struct Property {
    pub name: String,
    pub map_type: PropertyMapType,
    data: PropertyData,
}

impl Debug for Property {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Property")
            .field("name", &self.name)
            .field("map_type", &self.map_type)
            .finish()
    }
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

        // Initialize the property data array
        let mut property_data = match property_type {
            PropertyType::Bool => PropertyData::Bool(Vec::with_capacity(length as usize)),
            PropertyType::Int16 => PropertyData::Int16(Vec::with_capacity(length as usize)),
            PropertyType::Int32 => PropertyData::Int32(Vec::with_capacity(length as usize)),
            PropertyType::Int64 => PropertyData::Int64(Vec::with_capacity(length as usize)),
            PropertyType::Double => PropertyData::Double(Vec::with_capacity(length as usize)),
            PropertyType::LongDouble => {
                PropertyData::LongDouble(Vec::with_capacity(length as usize))
            }
            PropertyType::String => PropertyData::String(Vec::with_capacity(length as usize)),
            PropertyType::VectorBool => {
                PropertyData::VectorBool(Vec::with_capacity(length as usize))
            }
            PropertyType::VectorInt16 => {
                PropertyData::VectorInt16(Vec::with_capacity(length as usize))
            }
            PropertyType::VectorInt32 => {
                PropertyData::VectorInt32(Vec::with_capacity(length as usize))
            }
            PropertyType::VectorInt64 => {
                PropertyData::VectorInt64(Vec::with_capacity(length as usize))
            }
            PropertyType::VectorDouble => {
                PropertyData::VectorDouble(Vec::with_capacity(length as usize))
            }
            PropertyType::VectorLongDouble => {
                PropertyData::VectorLongDouble(Vec::with_capacity(length as usize))
            }
            PropertyType::VectorString => {
                PropertyData::VectorString(Vec::with_capacity(length as usize))
            }
            PropertyType::PyObject => PropertyData::PyObject(Vec::with_capacity(length as usize)),
        };
        //let property_data = property_data;
        // Fill the array
        for _ in 0..length {
            match &mut property_data {
                PropertyData::Bool(v) => {
                    // read the bool
                    let bool = cursor.read_u8().unwrap();
                    // Convert to bool
                    v.push(bool);
                }
                PropertyData::Int16(v) => {
                    let int16 = cursor.read_i16::<LittleEndian>().unwrap();
                    v.push(int16);
                }
                PropertyData::Int32(v) => {
                    let int32 = cursor.read_i32::<LittleEndian>().unwrap();
                    v.push(int32);
                }
                PropertyData::Int64(v) => {
                    let int64 = cursor.read_i64::<LittleEndian>().unwrap();
                    v.push(int64);
                }
                PropertyData::Double(v) => {
                    let double = cursor.read_f64::<LittleEndian>().unwrap();
                    v.push(double);
                }
                PropertyData::LongDouble(v) => {
                    let long_double = cursor.read_u128::<LittleEndian>().unwrap();
                    // Convert to f64 (should work am not sure though)
                    let long_double = long_double as f64;
                    v.push(long_double);
                }
                PropertyData::String(v) => {
                    let string_length = cursor.read_u64::<LittleEndian>().unwrap();
                    let mut string = vec![0; string_length as usize];
                    cursor.read_exact(&mut string).unwrap();
                    let string = String::from_utf8_lossy(&string).to_string();
                    v.push(string);
                }
                PropertyData::VectorBool(v) => {
                    let vector_length = cursor.read_u64::<LittleEndian>().unwrap();
                    let mut vector = Vec::with_capacity(vector_length as usize);
                    cursor.read_exact(&mut vector).unwrap();
                    v.push(vector);
                }
                PropertyData::VectorInt16(v) => {
                    let vector_length = cursor.read_u64::<LittleEndian>().unwrap();
                    let mut vector: Vec<i16> = Vec::with_capacity(vector_length as usize);
                    for _ in 0..vector_length {
                        let int16 = cursor.read_i16::<LittleEndian>().unwrap();
                        vector.push(int16);
                    }
                    v.push(vector);
                }
                PropertyData::VectorInt32(v) => {
                    let vector_length = cursor.read_u64::<LittleEndian>().unwrap();
                    let mut vector = Vec::with_capacity(vector_length as usize);
                    for _ in 0..vector_length {
                        let int32 = cursor.read_i32::<LittleEndian>().unwrap();
                        vector.push(int32);
                    }
                    v.push(vector);
                }
                PropertyData::VectorInt64(v) => {
                    let vector_length = cursor.read_u64::<LittleEndian>().unwrap();
                    let mut vector = Vec::with_capacity(vector_length as usize);
                    for _ in 0..vector_length {
                        let int64 = cursor.read_i64::<LittleEndian>().unwrap();
                        vector.push(int64);
                    }
                    v.push(vector);
                }
                PropertyData::VectorDouble(v) => {
                    let vector_length = cursor.read_u64::<LittleEndian>().unwrap();
                    let mut vector = Vec::with_capacity(vector_length as usize);
                    for _ in 0..vector_length {
                        let double = cursor.read_f64::<LittleEndian>().unwrap();
                        vector.push(double);
                    }
                    v.push(vector);
                }
                PropertyData::VectorLongDouble(v) => {
                    let vector_length = cursor.read_u64::<LittleEndian>().unwrap();
                    let mut vector = Vec::with_capacity(vector_length as usize);
                    for _ in 0..vector_length {
                        let long_double = cursor.read_u128::<LittleEndian>().unwrap();
                        let long_double = long_double as f64;
                        vector.push(long_double);
                    }
                    v.push(vector);
                }
                PropertyData::VectorString(v) => {
                    let vector_length = cursor.read_u64::<LittleEndian>().unwrap();
                    let mut vector = Vec::with_capacity(vector_length as usize);
                    for _ in 0..vector_length {
                        let string_length = cursor.read_u64::<LittleEndian>().unwrap();
                        let mut string = vec![0; string_length as usize];
                        cursor.read_exact(&mut string).unwrap();
                        let string = String::from_utf8_lossy(&string).to_string();
                        vector.push(string);
                    }
                    v.push(vector);
                }
                PropertyData::PyObject(v) => {
                    let object_length = cursor.read_u64::<LittleEndian>().unwrap();
                    let mut object = vec![0; object_length as usize];
                    cursor.read_exact(&mut object).unwrap();
                    v.push(object);
                }
            }
        }

        // Static data with leaky box
        //let data: &'a PropertyData = Box::leak(property_data.into_boxed_slice());

        // Read the size of the property
        // This depends further on the type of the property and the length
        let property: Property = Property {
            name,
            data: property_data,
            map_type: property_map_type,
        };
        Ok(property)
    }

    pub fn data_view(&mut self) -> JsValue {
        match &mut self.data {
            PropertyData::Bool(v) => unsafe {
                let array = js_sys::Uint8Array::view_mut_raw(v.as_mut_ptr(), v.len());
                array.into()
            },
            PropertyData::Int16(v) => unsafe {
                let array = js_sys::Int16Array::view_mut_raw(v.as_mut_ptr(), v.len());
                array.into()
            },
            PropertyData::Int32(v) => unsafe {
                let array = js_sys::Int32Array::view_mut_raw(v.as_mut_ptr(), v.len());
                array.into()
            },
            PropertyData::Int64(v) => unsafe {
                let array = js_sys::BigInt64Array::view_mut_raw(v.as_mut_ptr(), v.len());
                array.into()
            },
            PropertyData::Double(v) => unsafe {
                let array = js_sys::Float64Array::view_mut_raw(v.as_mut_ptr(), v.len());
                array.into()
            },
            PropertyData::LongDouble(v) => unsafe {
                let array = js_sys::Float64Array::view_mut_raw(v.as_mut_ptr(), v.len());
                array.into()
            },
            PropertyData::String(v) => {
                let array = js_sys::Array::new();
                for (i, value) in v.iter().enumerate() {
                    array.set(i as u32, JsValue::from_str(value));
                }
                array.into()
            }
            PropertyData::VectorBool(v) => {
                // Array buffer view
                let array = js_sys::Array::new();
                for (i, value) in v.iter_mut().enumerate() {
                    let array_view = unsafe {
                        let array_view =
                            js_sys::Uint8Array::view_mut_raw(value.as_mut_ptr(), value.len());
                        array_view.into()
                    };
                    array.set(i as u32, array_view);
                }
                array.into()
            }
            PropertyData::VectorInt16(v) => {
                let array = js_sys::Array::new();
                for (i, value) in v.iter_mut().enumerate() {
                    let array_view = unsafe {
                        let array_view =
                            js_sys::Int16Array::view_mut_raw(value.as_mut_ptr(), value.len());
                        array_view.into()
                    };
                    array.set(i as u32, array_view);
                }
                array.into()
            }
            PropertyData::VectorInt32(v) => {
                let array = js_sys::Array::new();
                for (i, value) in v.iter_mut().enumerate() {
                    let array_view = unsafe {
                        let array_view =
                            js_sys::Int32Array::view_mut_raw(value.as_mut_ptr(), value.len());
                        array_view.into()
                    };
                    array.set(i as u32, array_view);
                }
                array.into()
            }
            PropertyData::VectorInt64(v) => {
                let array = js_sys::Array::new();
                for (i, value) in v.iter_mut().enumerate() {
                    let array_view = unsafe {
                        let array_view =
                            js_sys::BigInt64Array::view_mut_raw(value.as_mut_ptr(), value.len());
                        array_view.into()
                    };
                    array.set(i as u32, array_view);
                }
                array.into()
            }
            PropertyData::VectorDouble(v) => {
                let array = js_sys::Array::new();
                for (i, value) in v.iter_mut().enumerate() {
                    let array_view = unsafe {
                        let array_view =
                            js_sys::Float64Array::view_mut_raw(value.as_mut_ptr(), value.len());
                        array_view.into()
                    };
                    array.set(i as u32, array_view);
                }
                array.into()
            }
            PropertyData::VectorLongDouble(v) => {
                let array = js_sys::Array::new();
                for (i, value) in v.iter_mut().enumerate() {
                    let array_view = unsafe {
                        let array_view =
                            js_sys::Float64Array::view_mut_raw(value.as_mut_ptr(), value.len());
                        array_view.into()
                    };
                    array.set(i as u32, array_view);
                }
                array.into()
            }
            PropertyData::VectorString(v) => {
                let array = js_sys::Array::new();
                for (_, value) in v.iter_mut().enumerate() {
                    let array_view = js_sys::Array::new();
                    for (j, value) in value.iter().enumerate() {
                        array_view.set(j as u32, JsValue::from_str(value));
                    }
                    array.push(&array_view);
                }
                array.into()
            }
            PropertyData::PyObject(v) => {
                let array = js_sys::Array::new();
                for (i, value) in v.iter_mut().enumerate() {
                    let array_view = unsafe {
                        let array_view =
                            js_sys::Uint8Array::view_mut_raw(value.as_mut_ptr(), value.len());
                        array_view.into()
                    };
                    array.set(i as u32, array_view);
                }
                array.into()
            }
        }
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
