use std::collections::HashMap;

/// used to have a limited color palatte
pub struct ColorMap(HashMap<usize, u32>);

impl ColorMap {
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    /// returns Some(u32) if color is not already present. Returns None otherwise
    pub fn add(&mut self, key: usize, value: u32) -> Option<u32> {
        self.0.insert(key, value)
    }

    /// returns value at given key if Some
    pub fn get(&self, key: usize) -> Option<&u32> {
        self.0.get(&key)
    }

    /// returns mutible value at given key if Some
    pub fn get_mut(&mut self, key: usize) -> Option<&mut u32> {
        self.0.get_mut(&key)
    }
}

//todo! make this store fn for graphics calculations rather than single colors
