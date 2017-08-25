
use std::ops::IndexMut;
use std::ops::Index;

/// A non-empty Vec<A>
pub struct NonEmptyVec<A> { pub head: A, pub tail: Vec<A> }

impl<A> NonEmptyVec<A> {

    pub fn new(a: A, tail: Vec<A>) -> NonEmptyVec<A> {
        NonEmptyVec { head: a, tail: tail }
    }

    pub fn singleton(a: A) -> NonEmptyVec<A> {
        NonEmptyVec { head: a, tail: Vec::new() }
    }

    pub fn push(&mut self, value: A) {
        self.tail.push(value)
    }

    pub fn len(&self) -> usize {
        1 + self.tail.len()
    }
}

impl<A> Index<usize> for NonEmptyVec<A> {
    type Output = A;
    fn index(&self, index: usize) -> &A {
        if index == 0 {
            &self.head
        } else {
            &self.tail[index-1]
        }
    }
}

impl<A> IndexMut<usize> for NonEmptyVec<A> {

    fn index_mut(&mut self, index: usize) -> &mut A {
        if index == 0 {
            &mut self.head
        } else {
            &mut self.tail[index-1]
        }
    }
}
