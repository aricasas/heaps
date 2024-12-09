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

    fn split_remove_root(mut self) -> (I, BinomialTreeLink<I>) {
        debug_assert!(self.next_sibling.is_none());

        let mut left_child = self.left_child.take();

        // Reverse singly linked list from https://stackoverflow.com/a/65854843
        let rev = &mut None;
        while left_child.is_some() {
            mem::swap(&mut left_child.as_mut().unwrap().next_sibling, rev);
            mem::swap(&mut left_child, rev);
        }
        mem::swap(&mut left_child, rev);

        (self.item, left_child)
    }

    fn binomial_link(&mut self, mut other: Box<Self>) {
        debug_assert_eq!(self.degree, other.degree);
        debug_assert!(self.next_sibling.is_none());
        debug_assert!(other.next_sibling.is_none());

        if self.item > other.item {
            mem::swap(self, &mut other);
        }

        other.next_sibling = self.left_child.take();
        self.left_child = Some(other);
        self.degree += 1;

        // Check that this is all thats needed, and if the asserts make sense, and maybe assume that self.item <= other.item
        todo!();
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
                return None;
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
        let rest = Self { head: rest };

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
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
        assert_eq!(0, num);
        assert_eq!(1, rest.as_ref().unwrap().item);
        assert_eq!(
            2,
            rest.as_ref().unwrap().next_sibling.as_ref().unwrap().item
        );
        assert_eq!(
            3,
            rest.as_ref()
                .unwrap()
                .next_sibling
                .as_ref()
                .unwrap()
                .next_sibling
                .as_ref()
                .unwrap()
                .item
        );
    }
}
