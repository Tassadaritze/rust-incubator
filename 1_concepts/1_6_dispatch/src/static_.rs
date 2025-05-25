use std::borrow::Cow;
use std::collections::HashMap;

use crate::{Storage, User};

struct UserRepository<S> {
    storage: S,
}

impl<S: Storage<u64, User>> UserRepository<S> {
    fn new(storage: S) -> Self {
        Self { storage }
    }
}

impl<S: Storage<u64, User>> Storage<u64, User> for UserRepository<S> {
    fn set(&mut self, key: u64, val: User) {
        self.storage.set(key, val);
    }

    fn get(&self, key: &u64) -> Option<&User> {
        self.storage.get(key)
    }

    fn remove(&mut self, key: &u64) -> Option<User> {
        self.storage.remove(key)
    }
}

pub(super) fn run() {
    let mut repository = UserRepository::new(HashMap::new());
    let user = User {
        id: 0,
        email: Default::default(),
        activated: false,
    };

    repository.set(0, user);
    assert_eq!(repository.get(&0).map(|u| u.id), Some(0));
    assert_eq!(repository.get(&0).map(|u| &u.email), Some(&Cow::default()));
    assert_eq!(repository.get(&0).map(|u| u.activated), Some(false));

    assert_eq!(repository.remove(&0).map(|u| u.id), Some(0));
    assert!(repository.remove(&0).is_none());
    assert!(repository.get(&0).is_none());
}
