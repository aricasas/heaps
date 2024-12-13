use std::{
    collections::HashMap,
    mem,
    ptr::{self, NonNull},
};

use crate::{Item, MinHeap};

#[derive(Debug)]
struct LeftTreeNode<I: Item> {
    item: I,
    degree: usize,
    /// Left child
    left: LeftTreeLink<I>,
    /// Right child, unless node is a root then it is the next tree
    right: LeftTreeLink<I>,
}
type LeftTreeLink<I> = Option<Box<LeftTreeNode<I>>>;

impl<I: Item> LeftTreeNode<I> {
    fn with_item(item: I) -> Self {
        Self {
            item,
            degree: 0,
            left: None,
            right: None,
        }
    }
    fn link(&mut self, mut other: Box<Self>) {
        if self.item > other.item {
            mem::swap(self, &mut other);
        }
        self.link_as_child(other);
    }

    fn link_as_child(&mut self, mut other: Box<Self>) {
        assert_eq!(self.degree, other.degree);
        assert!(other.right.is_none());
        assert!(self.item <= other.item);

        let curr_left = self.left.take();
        other.right = curr_left;
        self.left = Some(other);
        self.degree += 1;
    }

    fn merge_matches_one_pass(
        mut head_a: LeftTreeLink<I>,
        mut head_b: LeftTreeLink<I>,
    ) -> LeftTreeLink<I> {
        let mut matchings: HashMap<usize, Box<LeftTreeNode<I>>> = HashMap::new();

        if head_a.is_none() {
            mem::swap(&mut head_a, &mut head_b);
        }

        let mut full_list_head = None;
        let mut list_tail_cursor = &mut full_list_head;
        let mut cursor = head_a;
        while let Some(mut tree) = cursor.or_else(|| head_b.take()) {
            cursor = tree.right.take();

            let degree = tree.degree;

            if let Some(matching_tree) = matchings.remove(&degree) {
                tree.link(matching_tree);
                *list_tail_cursor = Some(tree);
                list_tail_cursor = &mut list_tail_cursor.as_mut().unwrap().right;
            } else {
                matchings.insert(degree, tree);
            }
        }
        // Add leftovers
        for tree in matchings.into_values() {
            *list_tail_cursor = Some(tree);
            list_tail_cursor = &mut list_tail_cursor.as_mut().unwrap().right;
        }

        full_list_head
    }
}

pub struct LazyBinomialHeap<I: Item> {
    // Head of the list of trees. "Owns" the list
    head: LeftTreeLink<I>,
    // Pointer to the last tree in the list
    tail: Option<NonNull<LeftTreeNode<I>>>,
    // Pointer to the tree immediately before the min tree, or None if min tree is at head
    prev_min: Option<NonNull<LeftTreeNode<I>>>,
    // Pointer to the tree with min value in the list
    min: Option<NonNull<LeftTreeNode<I>>>,
}

impl<I: Item> Default for LazyBinomialHeap<I> {
    fn default() -> Self {
        Self {
            head: None,
            tail: None,
            prev_min: None,
            min: None,
        }
    }
}

impl<I: Item> LazyBinomialHeap<I> {
    fn with_item(item: I) -> Self {
        let mut boxed_root = Box::new(LeftTreeNode::with_item(item));
        let ptr = NonNull::new(&mut *boxed_root).unwrap();
        Self {
            head: Some(boxed_root),
            tail: Some(ptr),
            prev_min: None,
            min: Some(ptr),
        }
    }

    fn is_empty(&self) -> bool {
        if self.head.is_none()
            && self.tail.is_none()
            && self.prev_min.is_none()
            && self.min.is_none()
        {
            true
        } else if self.head.is_some() && self.tail.is_some() && self.min.is_some() {
            false
        } else {
            // Invalid state
            unreachable!()
        }
    }

    fn from_list(list: LeftTreeLink<I>) -> LazyBinomialHeap<I> {
        if list.is_none() {
            return Self::make_heap();
        }

        // Moving the min to front of list to simplify prev_min pointer
        let mut min_tree = list.unwrap(); // start with first element, find actual min in loop
        let mut remaining_list = min_tree.right.take();
        let mut cursor = &mut remaining_list;
        let mut tail_ptr = NonNull::new(&mut *min_tree).unwrap();

        while let Some(mut curr_tree) = cursor.take() {
            if min_tree.item > curr_tree.item {
                min_tree.right = curr_tree.right.take();
                mem::swap(&mut min_tree, &mut curr_tree);
            }

            tail_ptr = NonNull::new(&mut *curr_tree).unwrap();
            *cursor = Some(curr_tree);
            cursor = &mut cursor.as_mut().unwrap().right;
        }
        min_tree.right = remaining_list;

        let min_ptr = NonNull::new(&mut *min_tree).unwrap();

        Self {
            head: Some(min_tree),
            tail: Some(tail_ptr),
            prev_min: None,
            min: Some(min_ptr),
        }
    }
}

impl<I: Item> MinHeap for LazyBinomialHeap<I> {
    type Item = I;

    fn peek_min(&self) -> Option<&Self::Item> {
        self.min.map(|ptr| {
            // SAFETY: We only store Some(ptr) from valid trees
            // Since we have &self borrowed, we know there are not mutable references
            let x = unsafe { ptr.as_ref() };
            &x.item
        })
    }

    fn extract_min(&mut self) -> Option<Self::Item> {
        if self.is_empty() {
            return None;
        }

        let min_ptr = self.min.unwrap();
        let mut list = self.head.take();

        let min = if let Some(mut prev_ptr) = self.prev_min {
            // Extract min from middle of list
            // SAFETY: We only ever store valid pointers to trees
            // We hold a mutable reference to self, so no one can have any other reference
            let prev_tree = unsafe { prev_ptr.as_mut() };
            let mut min = prev_tree.right.take().unwrap();
            prev_tree.right = min.right.take();

            min
        } else {
            // Extract min from start of list
            let mut min = list.take().unwrap();
            list = min.right.take();
            min
        };

        assert!(ptr::eq(min_ptr.as_ptr(), &*min)); // Sanity check

        // Extracting the min item and the rest of the elements from the tree that contained it
        let LeftTreeNode {
            item: min_item,
            degree: _,
            left: min_tree_remaining, // Left child of a root can be thought of as list of trees
            right: _,
        } = *min;

        let merged = LeftTreeNode::merge_matches_one_pass(list, min_tree_remaining);
        let full_heap = Self::from_list(merged);
        *self = full_heap;

        Some(min_item)
    }

    fn insert(&mut self, item: Self::Item) {
        let heap = mem::take(self);
        let new_item = Self::with_item(item);
        *self = Self::meld(heap, new_item);
    }

    fn heapify(items: Vec<Self::Item>) -> Self {
        let mut heap = Self::make_heap();
        for item in items.into_iter() {
            heap.insert(item);
        }
        heap
    }

    fn meld(heap_a: Self, heap_b: Self) -> Self {
        if heap_a.is_empty() {
            return heap_b;
        }
        if heap_b.is_empty() {
            return heap_a;
        }

        let head_a = heap_a.head.unwrap();
        let mut tail_ptr_a = heap_a.tail.unwrap();
        let prev_min_a = heap_a.prev_min;
        let min_ptr_a = heap_a.min.unwrap();

        let head_b = heap_b.head.unwrap();
        let tail_ptr_b = heap_b.tail.unwrap();
        let prev_min_b = heap_b.prev_min;
        let min_ptr_b = heap_b.min.unwrap();

        // Append list b to the tail of list a
        {
            // SAFETY: We only store valid pointers to trees
            // We own heap_a so no-one else can have a reference
            let tail_a = unsafe { tail_ptr_a.as_mut() };
            tail_a.right = Some(head_b);
        }
        let (prev_min, min) = {
            // Safety: We only store valid pointer to trees
            // We own both heap_a and heap_b so there can't be mutable references
            let min_a = unsafe { min_ptr_a.as_ref() };
            let min_b = unsafe { min_ptr_b.as_ref() };

            // update min
            if min_a.item <= min_b.item {
                (prev_min_a, min_ptr_a)
            } else if prev_min_b.is_some() {
                (prev_min_b, min_ptr_b)
            } else {
                // min_b was the first element of list b
                (Some(tail_ptr_a), min_ptr_b)
            }
        };

        Self {
            head: Some(head_a),
            tail: Some(tail_ptr_b),
            prev_min,
            min: Some(min),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::LazyBinomialHeap;
    use crate::tests;

    type HeapU32 = LazyBinomialHeap<u32>;

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
