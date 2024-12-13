#![warn(clippy::all, clippy::cargo)]

mod binary_heap;
mod binomial_heap;
mod lazy_binomial;

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

pub use binary_heap::BinaryHeap;
pub use binomial_heap::BinomialHeap;
pub use lazy_binomial::LazyBinomialHeap;
