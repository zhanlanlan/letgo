use std::collections::HashMap;

struct TimeMap {
    inner: HashMap<String, Vec<(i32, String)>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl TimeMap {
    /** Initialize your data structure here. */
    fn new() -> Self {
        TimeMap {
            inner: HashMap::new(),
        }
    }

    fn set(&mut self, key: String, value: String, timestamp: i32) {
        let v = self.inner.entry(key).or_insert(vec![]);
        v.push((timestamp, value));
    }

    fn get(&self, key: String, timestamp: i32) -> String{

        if let Some(v) = self.inner.get(&key) {
            todo!()
        };
        return String::from("");
    }
}

/**
 * Your TimeMap object will be instantiated and called as such:
 * let obj = TimeMap::new();
 * obj.set(key, value, timestamp);
 * let ret_2: String = obj.get(key, timestamp);
 */

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_example() {}
}
