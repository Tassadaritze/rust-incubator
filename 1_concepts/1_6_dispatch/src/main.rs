mod dynamic;
mod static_;

use std::borrow::Cow;
use std::collections::HashMap;
use std::hash::Hash;

trait Storage<K, V> {
    fn set(&mut self, key: K, val: V);
    fn get(&self, key: &K) -> Option<&V>;
    fn remove(&mut self, key: &K) -> Option<V>;
}

struct User {
    id: u64,
    email: Cow<'static, str>,
    activated: bool,
}

impl<K, V> Storage<K, V> for HashMap<K, V>
where
    K: Eq + Hash,
{
    fn set(&mut self, key: K, val: V) {
        self.insert(key, val);
    }

    fn get(&self, key: &K) -> Option<&V> {
        self.get(key)
    }

    fn remove(&mut self, key: &K) -> Option<V> {
        self.remove(key)
    }
}

fn main() {
    dynamic::run();
    static_::run();
}
