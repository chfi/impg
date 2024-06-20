use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SequenceIndex {
    name_to_id: HashMap<String, u32>,
    id_to_name: HashMap<u32, String>,
    id_to_len: HashMap<u32, usize>,
}

impl SequenceIndex {
    pub fn new() -> Self {
        SequenceIndex {
            name_to_id: HashMap::new(),
            id_to_name: HashMap::new(),
            id_to_len: HashMap::new(),
        }
    }

    pub fn get_or_insert_id(&mut self, name: &str, length: Option<usize>) -> u32 {
        let id = if let Some(id) = self.name_to_id.get(name) {
            *id
        } else {
            let id = self.id_to_name.len() as u32;
            self.id_to_name.insert(id, name.to_owned());
            self.name_to_id.insert(name.to_owned(), id);
            id
        };

        if let Some(len) = length {
            self.id_to_len.entry(id).or_insert(len);
        }

        id
    }

    pub fn sequences(&self) -> impl Iterator<Item = (u32, &str, Option<usize>)> + '_ {
        let ids = 0..self.id_to_name.len();

        ids.filter_map(|id| {
            let id = id as u32;
            let name = self.id_to_name.get(&id)?;
            let len = self.id_to_len.get(&id).copied();
            Some((id, name.as_str(), len))
        })
    }

    pub fn get_id(&self, name: &str) -> Option<u32> {
        self.name_to_id.get(name).copied()
    }

    pub fn get_name(&self, id: u32) -> Option<&str> {
        self.id_to_name.get(&id).map(|s| s.as_str())
    }

    pub fn get_len_from_id(&self, id: u32) -> Option<usize> {
        self.id_to_len.get(&id).copied()
    }

    pub fn is_empty(&self) -> bool {
        self.name_to_id.is_empty()
    }

    pub fn len(&self) -> usize {
        self.name_to_id.len()
    }
}
