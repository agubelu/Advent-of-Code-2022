use rustc_hash::FxHashMap;
use std::hash::Hash;

pub struct IDAssigner<T: Hash + Eq> {
    data: FxHashMap<T, u32>,
}

impl<T: Hash + Eq> IDAssigner<T> {
    pub fn new() -> Self {
        Self { data: FxHashMap::default() }
    }

    pub fn get_id(&mut self, elem: T) -> u32 {
        if let Some(id) = self.data.get(&elem) {
            *id
        } else {
            let next_id = self.data.len() as u32;
            self.data.insert(elem, next_id);
            next_id
        }
    }
}
