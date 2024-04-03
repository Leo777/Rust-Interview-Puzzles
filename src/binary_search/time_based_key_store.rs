use std::collections::HashMap;

struct TimeMap {
    store: HashMap<String, Vec<(String, i32)>>,
}


impl TimeMap {
    fn new() -> Self {
        Self {
            store: HashMap::new(),
        }
    }

    fn set(&mut self, key: String, value: String, timestamp: i32) {
        self.store.entry(key).or_default().push((value, timestamp));
    }

    fn get(&self, key: String, timestamp: i32) -> String {
        let mut res = String::new();

        if let Some(vector) = self.store.get(&key) {
            let (mut l, mut r) = (0, vector.len());

            while l < r {
                let m = l + (r - l) / 2;

                if timestamp < vector[m].1 {
                    r = m;
                } else {
                    res = vector[m].0.clone();
                    l = m + 1;
                }
            }
        }

        res
    }
}
