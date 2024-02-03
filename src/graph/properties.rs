use crate::{
    graph_file::properties::{Property, PropertyMapType},
    Graph,
};

impl Graph {
    /// Get a property by name and (optionally) property_type
    /// If duplicate property names exist, the first property found will be returned
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the property
    /// * `property_type` - The type of the property
    ///
    /// # Errors
    ///
    /// Returns an error if the property does not exist
    ///
    /// # Example
    ///
    /// ```
    /// use gt_graph_wasm::graph::Graph;
    ///
    /// let graph = Graph::new();
    /// let property = graph.property("name".to_string(), PropertyMapType::Graph);
    /// ```
    pub fn property(
        &mut self,
        name: String,
        property_type: Option<PropertyMapType>,
    ) -> Result<&mut Property, String> {
        let property_type = property_type.as_ref();
        for property in &mut self.file.properties {
            if property.name == name
                && (property_type.as_ref().is_none()
                    || property.map_type == *property_type.unwrap())
            {
                return Ok(property);
            }
        }
        Err(format!("Property '{}' not found", name))
    }

    pub fn graph_property(&mut self, name: String) -> Result<&mut Property, String> {
        self.property(name, Some(PropertyMapType::Graph))
    }
    pub fn vertex_property(&mut self, name: String) -> Result<&mut Property, String> {
        self.property(name, Some(PropertyMapType::Vertex))
    }
    pub fn edge_property(&mut self, name: String) -> Result<&mut Property, String> {
        self.property(name, Some(PropertyMapType::Edge))
    }

    /// Get a list of all property names
    /// If no property type is provided, all properties will be returned
    /// (might contain duplicates)
    ///
    /// # Arguments
    ///
    /// * `property_type` - The type of the property
    ///
    /// # Example
    ///
    /// ```
    /// use gt_graph_wasm::graph::Graph;
    /// use gt_graph_wasm::graph_file::properties::PropertyMapType;
    ///
    /// let graph = Graph::new();
    ///
    /// // Get all graph properties
    /// let graph_properties = graph.property_names(Some(PropertyMapType::Graph));
    ///
    ///
    /// // Get all properties
    /// let all_properties = graph.property_names(None);
    /// ```
    pub fn property_names(&self, property_type: Option<PropertyMapType>) -> Vec<String> {
        let mut ret = Vec::new();

        let property_type = property_type.as_ref();
        for property in &self.file.properties {
            if property_type.is_none() || property.map_type == *property_type.unwrap() {
                ret.push(property.name.clone());
            }
        }
        ret
    }

    pub fn graph_property_names(&self) -> Vec<String> {
        self.property_names(Some(PropertyMapType::Graph))
    }
    pub fn vertex_property_names(&self) -> Vec<String> {
        self.property_names(Some(PropertyMapType::Vertex))
    }
    pub fn edge_property_names(&self) -> Vec<String> {
        self.property_names(Some(PropertyMapType::Edge))
    }
}
