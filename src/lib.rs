mod binary_heap;
mod bino_heap;
mod binomial_heap;
mod nodes;

#[cfg(test)]
mod tests;

pub trait Item: Ord {}
impl<I: Ord> Item for I {}

pub trait MinHeap: Default {
    type Item;

    fn make_heap() -> Self {
        Self::default()
    }
    fn peek_min(&self) -> Option<&Self::Item>;
    fn extract_min(&mut self) -> Option<Self::Item>;
    fn insert(&mut self, item: Self::Item);
    fn heapify(items: Vec<Self::Item>) -> Self;
    fn meld(heap_a: Self, heap_b: Self) -> Self;
}

pub fn naive_heap_sort<I, T: MinHeap<Item = I>>(items: Vec<I>) -> Vec<I> {
    let length = items.len();
    let mut heap = T::heapify(items);
    let mut sorted = Vec::with_capacity(length);
    while let Some(x) = heap.extract_min() {
        sorted.push(x);
    }
    sorted
}

pub use binary_heap::BinaryHeap;
pub use binomial_heap::BinomialHeap;
