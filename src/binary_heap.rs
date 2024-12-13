use crate::{Item, MinHeap};

pub struct BinaryHeap<I: Item> {
    array: Vec<I>,
}

#[inline(always)]
const fn parent(i: usize) -> usize {
    (i - 1) / 2
}
#[inline(always)]
const fn left(i: usize) -> usize {
    2 * i + 1
}
#[inline(always)]
const fn right(i: usize) -> usize {
    2 * i + 2
}
#[inline(always)]
fn min_child<I: Item>(items: &[I], i: usize) -> usize {
    if left(i) >= items.len() {
        i
    } else if right(i) >= items.len() || items[left(i)] < items[right(i)] {
        left(i)
    } else {
        right(i)
    }
}

impl<I: Item> BinaryHeap<I> {
    fn sift_up(&mut self, mut i: usize) {
        debug_assert!(i < self.array.len());
        while i != 0 && self.array[parent(i)] > self.array[i] {
            self.array.swap(parent(i), i);
            i = parent(i);
        }
    }
    fn sift_down(&mut self, mut i: usize) {
        debug_assert!(i < self.array.len());
        let mut child = min_child(&self.array, i);
        while self.array[i] > self.array[child] {
            self.array.swap(i, child);
            i = child;
            child = min_child(&self.array, i);
        }
    }
}

impl<I: Item> Default for BinaryHeap<I> {
    fn default() -> Self {
        Self { array: Vec::new() }
    }
}

impl<I: Item> MinHeap for BinaryHeap<I> {
    type Item = I;

    fn peek_min(&self) -> Option<&Self::Item> {
        self.array.first()
    }

    fn extract_min(&mut self) -> Option<Self::Item> {
        if self.array.is_empty() {
            return None;
        }
        let min = self.array.swap_remove(0);
        if !self.array.is_empty() {
            self.sift_down(0);
        }
        Some(min)
    }

    fn insert(&mut self, item: Self::Item) {
        self.array.push(item);
        self.sift_up(self.array.len() - 1);
    }

    fn heapify(items: Vec<Self::Item>) -> Self {
        let mut heap = Self { array: items };
        for i in (0..heap.array.len() / 2).rev() {
            heap.sift_down(i);
        }
        heap
    }

    fn meld(mut heap_a: Self, mut heap_b: Self) -> Self {
        let mut items = std::mem::take(&mut heap_a.array);
        items.append(&mut heap_b.array);
        Self::heapify(items)
    }
}

#[cfg(test)]
mod tests {
    use super::BinaryHeap;
    use crate::tests;

    type HeapU32 = BinaryHeap<u32>;

    #[test]
    fn simple() {
        tests::simple::<HeapU32>();
    }
    #[test]
    fn empty_heap() {
        tests::empty_heap::<HeapU32>();
    }
    #[test]
    fn insert_after_extract() {
        tests::insert_after_extract::<HeapU32>();
    }
    #[test]
    fn duplicate_items() {
        tests::duplicate_items::<HeapU32>();
    }
    #[test]
    fn mixed_insertions_and_extractions() {
        tests::mixed_insertions_and_extractions::<HeapU32>();
    }
    #[test]
    fn meld() {
        tests::meld::<HeapU32>();
    }
    #[test]
    fn heapify() {
        tests::heapify::<HeapU32>();
    }
    #[test]
    fn heapify_duplicates() {
        tests::heapify_duplicates::<HeapU32>();
    }
    #[test]
    fn large_input() {
        tests::large_input::<HeapU32>();
    }
    #[test]
    fn meld_empty_and_non_empty() {
        tests::meld_empty_and_non_empty::<HeapU32>();
    }
    #[test]
    fn meld_two_empty_heaps() {
        tests::meld_two_empty_heaps::<HeapU32>();
    }
}
