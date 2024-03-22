use std::collections::HashMap;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SequenceIndex {
    name_to_id: HashMap<String, u32>,
    id_to_name: HashMap<u32, String>,
    next_id: u32,
}

impl SequenceIndex {
    pub fn new() -> Self {
        SequenceIndex {
            name_to_id: HashMap::new(),
            id_to_name: HashMap::new(),
            next_id: 0,
        }
    }

    pub fn get_or_insert_id(&mut self, name: &str) -> u32 {
        *self.name_to_id.entry(name.to_owned()).or_insert_with(|| {
            let id = self.next_id;
            self.id_to_name.insert(id, name.to_owned());
            self.next_id += 1;
            id
        })
    }

    pub fn get_id(&self, name: &str) -> Option<u32> {
        self.name_to_id.get(name).copied()
    }

    pub fn get_name(&self, id: u32) -> Option<&str> {
        self.id_to_name.get(&id).map(|s| s.as_str())
    }

    pub fn is_empty(&self) -> bool {
        self.name_to_id.is_empty()
    }
    
    pub fn len(&self) -> usize {
        self.name_to_id.len()
    }
}
