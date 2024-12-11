use std::{mem, ptr};

use crate::{Item, MinHeap};

struct BinomialTreeNode<I: Item> {
    /// Item the node holds
    item: I,
    /// Number of children node has
    degree: usize,
    /// Right sibling of the node
    next_sibling: BinomialTreeLink<I>,
    /// Left child of the node
    left_child: BinomialTreeLink<I>,
}

type BinomialTreeLink<I> = Option<Box<BinomialTreeNode<I>>>;

impl<I: Item> BinomialTreeNode<I> {
    fn with_item(item: I) -> Box<Self> {
        Box::new(Self {
            item,
            degree: 0,
            next_sibling: None,
            left_child: None,
        })
    }

    fn split_remove_root(mut self) -> (I, BinomialHeap<I>) {
        debug_assert!(self.next_sibling.is_none());

        let mut left_child = self.left_child.take();

        // Reverse singly linked list from https://stackoverflow.com/a/65854843
        let rev = &mut None;
        while left_child.is_some() {
            mem::swap(&mut left_child.as_mut().unwrap().next_sibling, rev);
            mem::swap(&mut left_child, rev);
        }
        mem::swap(&mut left_child, rev);

        (self.item, BinomialHeap { head: left_child })
    }

    fn link(&mut self, mut other: Box<Self>) {
        if self.item > other.item {
            mem::swap(self, &mut other);
        }
        self.link_as_child(other);
    }

    fn link_as_child(&mut self, mut other: Box<Self>) {
        debug_assert_eq!(self.degree, other.degree);
        debug_assert!(other.next_sibling.is_none());
        debug_assert!(self.item <= other.item);

        other.next_sibling = self.left_child.take();
        self.left_child = Some(other);
        self.degree += 1;
    }

    fn merge_by_degree(
        list_a: BinomialTreeLink<I>,
        list_b: BinomialTreeLink<I>,
    ) -> BinomialTreeLink<I> {
        let mut new_list = None;

        let mut list_tail = &mut new_list;
        let mut cursor_a = list_a;
        let mut cursor_b = list_b;
        loop {
            debug_assert!(list_tail.is_none());

            match (cursor_a.take(), cursor_b.take()) {
                (None, None) => break,
                (Some(mut a), None) => {
                    cursor_a = a.next_sibling.take();
                    *list_tail = Some(a);
                }
                (None, Some(mut b)) => {
                    cursor_b = b.next_sibling.take();
                    *list_tail = Some(b);
                }
                (Some(mut a), Some(mut b)) => {
                    if a.degree <= b.degree {
                        cursor_a = a.next_sibling.take();
                        cursor_b = Some(b);
                        *list_tail = Some(a);
                    } else {
                        cursor_a = Some(a);
                        cursor_b = b.next_sibling.take();
                        *list_tail = Some(b);
                    }
                }
            }
            list_tail = &mut list_tail.as_mut().unwrap().next_sibling;
        }

        new_list
    }
}

pub struct BinomialHeap<I: Item> {
    /// List of binomial trees
    head: BinomialTreeLink<I>,
}

impl<I: Item> Default for BinomialHeap<I> {
    fn default() -> Self {
        Self { head: None }
    }
}
impl<I: Item> BinomialHeap<I> {
    fn with_item(item: I) -> Self {
        Self {
            head: Some(BinomialTreeNode::with_item(item)),
        }
    }
    fn find_min(&self) -> Option<&BinomialTreeNode<I>> {
        let mut min: Option<&BinomialTreeNode<I>> = None;
        let mut curr_tree = &self.head;

        while let Some(tree) = curr_tree {
            if min.is_none_or(|min| min.item > tree.item) {
                min = Some(tree.as_ref());
            }
            curr_tree = &tree.next_sibling;
        }

        min
    }
    fn find_link_to_min(&mut self) -> Option<&mut BinomialTreeLink<I>> {
        let min: *const BinomialTreeNode<I> = self.find_min()?;

        let mut curr_link = &mut self.head;
        loop {
            if curr_link
                .as_ref()
                .is_some_and(|tree| ptr::eq(tree.as_ref(), min))
            {
                return Some(curr_link);
            } else if let Some(tree) = curr_link {
                curr_link = &mut tree.next_sibling;
            } else {
                // We return in the first line if no min exists, so we expect to reach in some iteration
                unreachable!()
            }
        }
    }
}
impl<I: Item> MinHeap for BinomialHeap<I> {
    type Item = I;

    fn peek_min(&self) -> Option<&Self::Item> {
        self.find_min().map(|tree| &tree.item)
    }

    fn extract_min(&mut self) -> Option<Self::Item> {
        // Extract tree with min root from middle of linked list
        let link_to_min = self.find_link_to_min()?;
        let mut min_tree = link_to_min.take()?;
        *link_to_min = min_tree.next_sibling.take();

        // Build a new heap with the elements from the min tree except the root
        let (min, rest) = min_tree.split_remove_root();

        // Push all those elements back into the heap
        let original_heap = mem::take(self);
        *self = Self::meld(original_heap, rest);

        Some(min)
    }

    fn insert(&mut self, item: Self::Item) {
        let current = mem::take(self);
        let new_item = Self::with_item(item);
        *self = Self::meld(current, new_item);
    }

    fn heapify(items: Vec<Self::Item>) -> Self {
        match items.len() {
            0 => Self::make_heap(),
            1 => Self::with_item(items.into_iter().next().unwrap()),
            _ => {
                // Recursively heapify by splitting vector in half and melding results
                let mut left = items;
                let right = left.split_off(left.len() / 2);

                Self::meld(Self::heapify(left), Self::heapify(right))
            }
        }
    }

    fn meld(heap_a: Self, heap_b: Self) -> Self {
        let mut list_head = BinomialTreeNode::merge_by_degree(heap_a.head, heap_b.head);
        if list_head.is_none() {
            return Self::make_heap();
        }

        let mut x = list_head.take().unwrap();
        let mut next_x = x.next_sibling.take();
        let mut list_tail = &mut list_head;

        while let Some(mut next) = next_x {
            if x.degree != next.degree
                || next
                    .next_sibling
                    .as_ref()
                    .is_some_and(|nn_x| nn_x.degree == x.degree)
            {
                *list_tail = Some(x);
                list_tail = &mut list_tail.as_mut().unwrap().next_sibling;
                x = next;
                next_x = x.next_sibling.take();
            } else {
                next_x = next.next_sibling.take();
                x.link(next);
            }
        }
        debug_assert!(list_tail.is_none());
        *list_tail = Some(x);

        Self { head: list_head }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests;

    #[test]
    fn binomial_insertion() {
        let mut heap = BinomialHeap::make_heap();
        heap.insert(10);
        let head = heap.head.as_ref().unwrap();
        assert_eq!(head.degree, 0);
        assert_eq!(head.item, 10);
        assert!(head.next_sibling.is_none());
        assert!(head.left_child.is_none());
        heap.insert(5);
        let head = heap.head.as_ref().unwrap();
        assert_eq!(head.degree, 1);
        assert_eq!(head.item, 5);
        assert!(head.next_sibling.is_none());
        let child = head.left_child.as_ref().unwrap();
        assert_eq!(child.degree, 0);
        assert_eq!(child.item, 10);
        assert!(child.left_child.is_none());
        assert!(child.next_sibling.is_none());
        assert_eq!(heap.extract_min().unwrap(), 5);
        assert_eq!(heap.extract_min().unwrap(), 10);
    }

    #[test]
    fn binomial_heap_split() {
        let x = Box::new(BinomialTreeNode {
            item: 1,
            degree: 0,
            next_sibling: None,
            left_child: None,
        });
        let y = Box::new(BinomialTreeNode {
            item: 2,
            degree: 0,
            next_sibling: Some(x),
            left_child: None,
        });
        let z = Box::new(BinomialTreeNode {
            item: 3,
            degree: 0,
            next_sibling: Some(y),
            left_child: None,
        });
        let parent = BinomialTreeNode {
            item: 0,
            degree: 3,
            next_sibling: None,
            left_child: Some(z),
        };
        let (num, rest) = parent.split_remove_root();
        let first = rest.head.as_ref().unwrap();
        let second = first.next_sibling.as_ref().unwrap();
        let third = second.next_sibling.as_ref().unwrap();
        assert_eq!(0, num);
        assert_eq!(1, first.item);
        assert_eq!(2, second.item);
        assert_eq!(3, third.item);
    }

    type HeapU32 = BinomialHeap<u32>;

    #[test]
    fn binomial_heap_simple() {
        tests::simple::<HeapU32>();
    }

    #[test]
    fn binomial_heap_heapify() {
        tests::heapify::<HeapU32>();
    }

    #[test]
    fn binomial_heap_meld() {
        tests::meld::<HeapU32>();
    }

    #[test]
    fn binomial_heap_sort() {
        tests::sort::<HeapU32>();
    }
}
