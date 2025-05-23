use core::fmt;
use std::collections::LinkedList;
use std::pin::{pin, Pin};
use std::rc::Rc;

trait SayHi: fmt::Debug {
    fn say_hi(self: Pin<&Self>) {
        println!("Hi from {self:?}")
    }
}

impl<T: fmt::Debug> SayHi for T {}

mod mut_non_t {
    use std::pin::Pin;
    use std::rc::Rc;

    pub(super) trait MutMeSomehow {
        fn mut_me_somehow(self: Pin<&mut Self>);
    }

    impl<T: Default> MutMeSomehow for Box<T> {
        fn mut_me_somehow(mut self: Pin<&mut Self>) {
            *self.as_mut() = Box::default();
        }
    }

    impl<T: Default> MutMeSomehow for Rc<T> {
        fn mut_me_somehow(self: Pin<&mut Self>) {
            if let Some(reference) = Rc::get_mut(self.get_mut()) {
                *reference = Default::default();
            }
        }
    }

    impl<T> MutMeSomehow for Vec<T> {
        fn mut_me_somehow(self: Pin<&mut Self>) {
            unsafe {
                // SAFETY: the `Vec` is not moved, only the data it points to is modified.
                self.get_unchecked_mut().reverse();
            }
        }
    }

    impl MutMeSomehow for String {
        fn mut_me_somehow(mut self: Pin<&mut Self>) {
            self.push_str("hello world");
        }
    }

    impl MutMeSomehow for &[u8] {
        fn mut_me_somehow(mut self: Pin<&mut Self>) {
            *self = b"hello world";
        }
    }
}

mod mut_t {
    use std::pin::Pin;

    pub(super) trait MutMeSomehow {
        fn mut_me_somehow(self: Pin<&mut Self>);
    }

    impl<T: Default> MutMeSomehow for T {
        fn mut_me_somehow(self: Pin<&mut Self>) {
            unsafe {
                // SAFETY: the previous value's destructor gets run before replacement,
                // and T's Default::default() is a valid T.
                *self.get_unchecked_mut() = Default::default();
            }
        }
    }
}

pub(super) fn run() {
    let mut pinned_box = pin!(Box::new(1));
    let mut pinned_rc = pin!(Rc::new(2));
    let mut pinned_vec = pin!(vec![3]);
    let mut pinned_string = pin!(4.to_string());
    let mut pinned_byte_slice = pin!([5].as_slice());
    let mut pinned_t = pin!(LinkedList::from([6, 7, 8, 9]));

    pinned_box.as_ref().say_hi();
    pinned_rc.as_ref().say_hi();
    pinned_vec.as_ref().say_hi();
    pinned_string.as_ref().say_hi();
    pinned_byte_slice.as_ref().say_hi();
    pinned_t.as_ref().say_hi();

    mut_non_t::MutMeSomehow::mut_me_somehow(pinned_box.as_mut());
    mut_non_t::MutMeSomehow::mut_me_somehow(pinned_rc.as_mut());
    mut_non_t::MutMeSomehow::mut_me_somehow(pinned_vec.as_mut());
    mut_non_t::MutMeSomehow::mut_me_somehow(pinned_string.as_mut());
    mut_non_t::MutMeSomehow::mut_me_somehow(pinned_byte_slice.as_mut());
    mut_t::MutMeSomehow::mut_me_somehow(pinned_t.as_mut());

    println!("=======");

    pinned_box.as_ref().say_hi();
    pinned_rc.as_ref().say_hi();
    pinned_vec.as_ref().say_hi();
    pinned_string.as_ref().say_hi();
    pinned_byte_slice.as_ref().say_hi();
    pinned_t.as_ref().say_hi();
}
