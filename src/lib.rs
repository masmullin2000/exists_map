#![allow(dead_code, unused)]

#[derive(Clone)]
struct ExistsItem<K> {
    hashed_key: u64,
    _marker: std::marker::PhantomData<K>,
}

impl<K> ExistsItem<K>
where
    K: std::hash::Hash,
{
    fn new(key: &K, state: &ahash::RandomState) -> Self {
        ExistsItem {
            hashed_key: state.hash_one(key),
            _marker: std::marker::PhantomData,
        }
    }
}

impl<K> From<u64> for ExistsItem<K> {
    fn from(hashed_key: u64) -> Self {
        ExistsItem {
            hashed_key,
            _marker: std::marker::PhantomData,
        }
    }
}

impl<K> std::hash::Hash for ExistsItem<K> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.hashed_key.hash(state);
    }
}

impl<K> PartialEq for ExistsItem<K> {
    fn eq(&self, other: &Self) -> bool {
        self.hashed_key == other.hashed_key
    }
}

impl<K> Eq for ExistsItem<K> {}

#[derive(Default, Clone)]
pub struct ExistsMap<K, V> {
    state: ahash::RandomState,
    map: std::collections::HashMap<ExistsItem<K>, V>,
}

impl<K, V> ExistsMap<K, V>
where
    K: std::hash::Hash,
{
    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        let item = ExistsItem::new(&key, &self.state);
        self.map.insert(item, value)
    }

    pub fn get(&self, key: &K) -> Option<&V> {
        let key = ExistsItem::new(key, &self.state);
        self.map.get(&key)
    }

    pub fn contains<Q>(&self, key: &Q) -> bool
    where
        K: std::borrow::Borrow<Q>,
        Q: std::hash::Hash + std::cmp::Eq + ?Sized,
    {
        let key = self.state.hash_one(key);
        let key = ExistsItem::from(key);
        self.map.contains_key(&key)
    }

    pub fn remove<Q>(&mut self, key: &Q) -> Option<V> 
    where
        K: std::borrow::Borrow<Q>,
        Q: std::hash::Hash + std::cmp::Eq + ?Sized,
    {
        let key = self.state.hash_one(key);
        let key = ExistsItem::from(key);
        self.map.remove(&key)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_exists_map_basic() {
        let mut exists_map = ExistsMap::<i32, i32> {
            map: std::collections::HashMap::new(),
            state: ahash::RandomState::with_seed(fastrand::usize(..)),
        };

        for i in 50..=100 {
            exists_map.insert(i, i);
        }
        assert!(exists_map.contains(&50));
        assert!(exists_map.contains(&75));
        assert!(exists_map.contains(&100));
        let val = exists_map.get(&50).unwrap();
        assert_eq!(*val, 50);
        let val = exists_map.get(&75).unwrap();
        assert_eq!(*val, 75);
        let val = exists_map.get(&100).unwrap();
        assert_eq!(*val, 100);

        assert_eq!(None, exists_map.get(&-1));
        assert_eq!(None, exists_map.get(&0));
        assert_eq!(None, exists_map.get(&49));
        assert_eq!(None, exists_map.get(&101));

        let val = exists_map.remove(&50).unwrap();
        assert_eq!(val, 50);
        assert_eq!(None, exists_map.get(&50));
        assert!(!exists_map.contains(&50));
        let val = exists_map.remove(&75).unwrap();
        assert_eq!(val, 75);
        assert_eq!(None, exists_map.get(&75));
        assert!(!exists_map.contains(&75));
        let val = exists_map.remove(&100).unwrap();
        assert_eq!(val, 100);
        assert_eq!(None, exists_map.get(&100));
        assert!(!exists_map.contains(&100));
    }

    #[test]
    fn test_exists_map_string() {
        let mut exists_map = ExistsMap::<String, String> {
            map: std::collections::HashMap::new(),
            state: ahash::RandomState::with_seed(fastrand::usize(..)),
        };

        exists_map.insert("hello".into(), "hello".into());
        exists_map.insert("world".into(), "world".into());
        exists_map.insert("foo".into(), "foo".into());

        let hello = "hello".to_string();
        assert!(exists_map.contains(&hello));
        assert!(exists_map.contains(hello.as_str()));

        assert!(exists_map.contains("hello"));
        assert!(exists_map.contains("world"));
        assert!(exists_map.contains("foo"));
        let val = exists_map.get(&"hello".into()).unwrap();
        assert_eq!(val, "hello");
        let val = exists_map.get(&"world".into()).unwrap();
        assert_eq!(val, "world");
        let val = exists_map.get(&"foo".into()).unwrap();
        assert_eq!(val, "foo");

        assert_eq!(None, exists_map.get(&"baz".into()));
        assert!(!exists_map.contains("baz"));
        assert_eq!(None, exists_map.get(&"bar".into()));
        assert!(!exists_map.contains("bar"));
        assert_eq!(None, exists_map.get(&"frank".into()));
        assert!(!exists_map.contains("frank"));
        assert_eq!(None, exists_map.get(&"larry".into()));
        assert!(!exists_map.contains("larry"));

        let val = exists_map.remove("hello").unwrap();
        assert_eq!(val, "hello");
        assert_eq!(None, exists_map.get(&"hello".into()));
        assert!(!exists_map.contains("hello"));
        let val = exists_map.remove("world").unwrap();
        assert_eq!(val, "world");
        assert_eq!(None, exists_map.get(&"world".into()));
        assert!(!exists_map.contains("world"));
        let val = exists_map.remove("foo").unwrap();
        assert_eq!(val, "foo");
        assert_eq!(None, exists_map.get(&"foo".into()));
        assert!(!exists_map.contains("foo"));
    }
}
