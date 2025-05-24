use std::cell::RefCell;
use std::rc::Rc;

struct GlobalStack<T> {
    inner: Rc<RefCell<Vec<T>>>,
}

impl<T> GlobalStack<T> {
    fn new() -> Self {
        Self {
            inner: Rc::new(RefCell::new(vec![])),
        }
    }

    fn push(&self, val: T) {
        self.inner.borrow_mut().push(val)
    }

    fn pop(&self) -> Option<T> {
        self.inner.borrow_mut().pop()
    }

    fn len(&self) -> usize {
        self.inner.borrow().len()
    }

    fn is_empty(&self) -> bool {
        self.inner.borrow().is_empty()
    }
}

impl<T> Clone for GlobalStack<T> {
    fn clone(&self) -> Self {
        Self {
            inner: Rc::clone(&self.inner),
        }
    }
}

fn main() {
    let stack: GlobalStack<Vec<i32>> = GlobalStack::new();
    let ref_a = &stack;
    let ref_b = &stack;

    ref_a.push(vec![1]);
    ref_b.push(vec![2]);
    assert_eq!(ref_a.pop(), Some(vec![2]));

    let cloned = stack.clone();
    let cloned_ref_a = &cloned;
    let cloned_ref_b = &cloned;

    assert_eq!(cloned_ref_b.pop(), Some(vec![1]));
    assert_eq!(cloned_ref_a.pop(), None);
    assert_eq!(ref_a.pop(), None);
    assert!(ref_b.is_empty());
    cloned_ref_a.push(vec![3]);
    cloned_ref_b.push(vec![4]);
    ref_b.push(vec![5]);
    assert_eq!(ref_a.len(), 3);
}
