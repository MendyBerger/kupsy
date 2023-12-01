use core::cmp::Reverse;

use alloc::collections::BinaryHeap;

/// Thin wrapper around std::collections::BinaryHeap with std::cmp::Reverse
pub struct MinHeap<T>
where
    T: Ord,
{
    heap: BinaryHeap<Reverse<T>>,
}
impl<T> MinHeap<T>
where
    T: Ord,
{
    pub fn new() -> Self {
        Self {
            heap: BinaryHeap::new(),
        }
    }
    pub fn pop(&mut self) -> Option<T> {
        self.heap.pop().map(|val| val.0)
    }
    pub fn peek(&self) -> Option<&T> {
        self.heap.peek().map(|val| &val.0)
    }
    pub fn push(&mut self, item: T) {
        self.heap.push(Reverse(item));
    }
    pub fn len(&self) -> usize {
        self.heap.len()
    }
}

/// Thin wrapper around std::collections::BinaryHeap
pub struct MaxHeap<T>
where
    T: Ord,
{
    heap: BinaryHeap<T>,
}
impl<T> MaxHeap<T>
where
    T: Ord,
{
    pub fn new() -> Self {
        Self {
            heap: BinaryHeap::new(),
        }
    }
    pub fn pop(&mut self) -> Option<T> {
        self.heap.pop()
    }
    pub fn peek(&mut self) -> Option<&T> {
        self.heap.peek()
    }
    pub fn push(&mut self, item: T) {
        self.heap.push(item);
    }
    pub fn len(&self) -> usize {
        self.heap.len()
    }
}
