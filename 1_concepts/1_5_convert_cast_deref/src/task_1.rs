use std::borrow::Borrow;
use std::collections::HashMap;
use std::convert::Infallible;

#[derive(Hash, Ord, PartialOrd, Eq, PartialEq)]
struct EmailString(String);

impl TryFrom<String> for EmailString {
    type Error = Infallible;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        // epic validation logic goes here

        Ok(Self(value))
    }
}

impl Borrow<str> for EmailString {
    fn borrow(&self) -> &str {
        &self.0
    }
}

impl AsRef<str> for EmailString {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl Borrow<String> for EmailString {
    fn borrow(&self) -> &String {
        &self.0
    }
}

impl AsRef<String> for EmailString {
    fn as_ref(&self) -> &String {
        &self.0
    }
}

pub(super) fn run() {
    let mut email_map: HashMap<EmailString, i32> = HashMap::new();
    email_map.insert("foo".to_string().try_into().unwrap(), 1);
    email_map.insert("bar".to_string().try_into().unwrap(), 2);

    assert_eq!(email_map.get("foo"), Some(&1));
    assert_eq!(email_map.remove("foo"), Some(1));
    assert_eq!(email_map.get("foo"), None);
    assert_eq!(email_map.get("bar"), Some(&2));
}
