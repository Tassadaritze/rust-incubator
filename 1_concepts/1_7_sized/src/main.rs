use std::borrow::Cow;
use std::cell::{Ref, RefCell};
use std::collections::HashMap;

#[derive(Clone, Debug)]
struct User {
    id: u64,
    email: Cow<'static, str>,
    activated: bool,
}

trait UserRepository {
    fn set(&self, key: u64, val: User);
    fn get(&self, key: &u64) -> Option<Ref<User>>;
    fn remove(&self, key: &u64) -> Option<User>;
}

struct InMemoryRepository {
    storage: RefCell<HashMap<u64, User>>,
}

impl InMemoryRepository {
    fn new() -> Self {
        Self {
            storage: RefCell::new(HashMap::new()),
        }
    }
}

impl UserRepository for InMemoryRepository {
    fn set(&self, key: u64, val: User) {
        self.storage.borrow_mut().insert(key, val);
    }

    fn get(&self, key: &u64) -> Option<Ref<User>> {
        Ref::filter_map(self.storage.borrow(), |storage| storage.get(key)).ok()
    }

    fn remove(&self, key: &u64) -> Option<User> {
        self.storage.borrow_mut().remove(key)
    }
}

trait Command {}

trait CommandHandler<C: Command> {
    type Context: ?Sized;
    type Result;

    fn handle_command(&self, cmd: &C, ctx: &Self::Context) -> Self::Result;
}

struct CreateUser;
impl Command for CreateUser {}

type UserError = Box<dyn std::error::Error>;

impl CommandHandler<CreateUser> for User {
    type Context = dyn UserRepository;
    type Result = Result<(), UserError>;

    fn handle_command(&self, cmd: &CreateUser, user_repo: &Self::Context) -> Self::Result {
        user_repo.set(
            self.id,
            User {
                id: self.id,
                email: self.email.clone(),
                activated: self.activated,
            },
        );

        Ok(())
    }
}

fn main() {
    let repository = InMemoryRepository::new();
    let user = User {
        id: 0,
        email: Default::default(),
        activated: false,
    };

    user.handle_command(&CreateUser, &repository).unwrap();

    assert_eq!(repository.get(&0).map(|u| u.id), Some(0));
    assert_eq!(repository.get(&0).unwrap().email, Cow::<'_, str>::default());
    assert_eq!(repository.get(&0).map(|u| u.activated), Some(false));

    assert_eq!(repository.remove(&0).map(|u| u.id), Some(0));
    assert!(repository.remove(&0).is_none());
    assert!(repository.get(&0).is_none());
}

#[cfg(test)]
mod tests {
    use super::*;

    struct MockRepository {
        value: RefCell<Option<User>>,
    }

    impl MockRepository {
        fn new() -> Self {
            Self {
                value: RefCell::new(Some(User {
                    id: 123,
                    email: Cow::Borrowed("foo"),
                    activated: true,
                })),
            }
        }
    }

    impl UserRepository for MockRepository {
        fn set(&self, key: u64, val: User) {
            eprintln!("added {val:?}");
        }

        fn get(&self, key: &u64) -> Option<Ref<User>> {
            if key == &123 {
                Ref::filter_map(self.value.borrow(), |v| v.as_ref()).ok()
            } else {
                None
            }
        }

        fn remove(&self, key: &u64) -> Option<User> {
            if key == &123 {
                self.value.replace(None)
            } else {
                None
            }
        }
    }

    #[test]
    fn mock_repository() {
        let repository = MockRepository::new();
        let user = User {
            id: 0,
            email: Default::default(),
            activated: false,
        };

        user.handle_command(&CreateUser, &repository).unwrap();

        assert!(repository.get(&0).is_none());
        assert!(repository.remove(&0).is_none());

        assert_eq!(repository.get(&123).map(|u| u.id), Some(123));
        assert_eq!(repository.get(&123).unwrap().email, "foo");
        assert_eq!(repository.get(&123).map(|u| u.activated), Some(true));
        assert_eq!(repository.remove(&123).map(|u| u.id), Some(123));
        assert!(repository.remove(&123).is_none());
        assert!(repository.get(&123).is_none());
        assert!(repository.get(&0).is_none());
    }
}
