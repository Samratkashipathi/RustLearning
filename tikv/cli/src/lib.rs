use std::collections::HashMap;

struct KvStore {
    map: HashMap<String, String>,
}

impl KvStore {
    fn new() -> KvStore {
        KvStore {
            map: HashMap::new(),
        }
    }

    fn set(&mut self, key: String, value: String) {
        self.map.insert(key, value);
    }

    fn get(&mut self, key: String) -> Option<String>{
        self.map.get(&key).cloned()
    }

    fn remove(&mut self, key: String) {
        self.map.remove(&key);
    }
}
