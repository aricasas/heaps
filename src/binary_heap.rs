use crate::{Item, MinHeap};

pub struct BinaryHeap<I: Item> {
    array: Vec<I>,
}

#[inline(always)]
fn parent(i: usize) -> usize {
    (i - 1) / 2
}
#[inline(always)]
fn left(i: usize) -> usize {
    2 * i + 1
}
#[inline(always)]
fn right(i: usize) -> usize {
    2 * i + 2
}
#[inline(always)]
fn min_child<I: Item>(items: &[I], i: usize) -> usize {
    if left(i) >= items.len() {
        return i;
    }
    if right(i) >= items.len() {
        return left(i);
    }
    if items[left(i)] < items[right(i)] {
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
        self.array.get(0)
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
    use super::*;

    #[test]
    fn simple() {
        let mut heap: BinaryHeap<u32> = BinaryHeap::new();
        heap.insert(3);
        heap.insert(2);
        heap.insert(1);

        assert_eq!(heap.peek_min(), Some(&1));

        assert_eq!(heap.extract_min(), Some(1));
        assert_eq!(heap.extract_min(), Some(2));
        assert_eq!(heap.extract_min(), Some(3));
        assert_eq!(heap.extract_min(), None);
    }

    #[test]
    fn heapify() {
        let mut heap: BinaryHeap<u32> = BinaryHeap::heapify(vec![3, 2, 1]);
        assert_eq!(heap.extract_min(), Some(1));
        assert_eq!(heap.extract_min(), Some(2));
        assert_eq!(heap.extract_min(), Some(3));
        assert_eq!(heap.extract_min(), None);

        heap = BinaryHeap::heapify(vec![]);
        assert_eq!(heap.extract_min(), None);
    }

    #[test]
    fn meld() {
        let mut heap_a: BinaryHeap<u32> = BinaryHeap::new();
        heap_a.insert(0);
        heap_a.insert(3);
        heap_a.insert(6);

        let mut heap_b: BinaryHeap<u32> = BinaryHeap::new();
        heap_b.insert(1);
        heap_b.insert(4);
        heap_b.insert(7);

        let mut heap_c: BinaryHeap<u32> = BinaryHeap::new();
        heap_c.insert(2);
        heap_c.insert(5);
        heap_c.insert(8);

        let heap_ac = BinaryHeap::meld(heap_a, heap_c);
        let mut heap_abc = BinaryHeap::meld(heap_ac, heap_b);

        assert_eq!(heap_abc.extract_min(), Some(0));
        assert_eq!(heap_abc.extract_min(), Some(1));
        assert_eq!(heap_abc.extract_min(), Some(2));
        assert_eq!(heap_abc.extract_min(), Some(3));
        assert_eq!(heap_abc.extract_min(), Some(4));
        assert_eq!(heap_abc.extract_min(), Some(5));
        assert_eq!(heap_abc.extract_min(), Some(6));
        assert_eq!(heap_abc.extract_min(), Some(7));
        assert_eq!(heap_abc.extract_min(), Some(8));
        assert_eq!(heap_abc.extract_min(), None);

        heap_a = BinaryHeap::new();
        heap_b = BinaryHeap::new();
        let mut heap_ab = BinaryHeap::meld(heap_a, heap_b);
        assert_eq!(heap_ab.extract_min(), None);
    }

    #[test]
    fn heap_sort() {
        let items = vec![4, 6, 3, 2, 5, 64, 2, 1];
        let sorted = crate::naive_heap_sort::<u32, BinaryHeap<u32>>(items);
        assert!(sorted.is_sorted());
        assert_eq!(sorted, [1, 2, 2, 3, 4, 5, 6, 64]);
    }
}
