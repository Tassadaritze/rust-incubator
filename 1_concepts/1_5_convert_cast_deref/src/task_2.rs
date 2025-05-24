use std::ops::{Deref, DerefMut};

use rand::prelude::*;

#[derive(Debug)]
struct Random<T>([T; 3]);

impl<T> Random<T> {
    fn new(one: T, two: T, three: T) -> Self {
        Self([one, two, three])
    }
}

impl<T> Deref for Random<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.0.choose(&mut rand::rng()).unwrap()
    }
}

impl<T> DerefMut for Random<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.0.choose_mut(&mut rand::rng()).unwrap()
    }
}

pub(super) fn run() {
    let mut random = Random::new(vec![1], vec![2], vec![3]);

    for _ in 0..10 {
        println!("{:?}", *random);
    }

    for _ in 0..10 {
        random.push((0..100).choose(&mut rand::rng()).unwrap());
    }

    for _ in 0..10 {
        println!("{:?}", *random);
    }
}
