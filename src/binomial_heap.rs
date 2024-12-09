use std::mem;

use crate::{
    nodes::{List, ListNode, Tree, TreeNode},
    Item, MinHeap,
};

pub struct BinomialHeap<I: Item> {
    list: List<Tree<I>>,
}

impl<I: Item> Default for BinomialHeap<I> {
    fn default() -> Self {
        Self {
            list: List::default(),
        }
    }
}

impl<I: Item> BinomialHeap<I> {
    fn with_item(item: I) -> Self {
        Self {
            list: List::with_item(Tree::with_item(item)),
        }
    }
    fn merge_by_degree(heap_a: List<Tree<I>>, heap_b: List<Tree<I>>) -> List<Tree<I>> {
        let mut new_list = List::default();

        let mut cursor = &mut new_list.head;
        let mut iter_a = heap_a.into_iter().peekable();
        let mut iter_b = heap_b.into_iter().peekable();
        loop {
            match (iter_a.peek(), iter_b.peek()) {
                (None, None) => break,
                (Some(_), None) => {
                    let a = iter_a.next().unwrap();
                    *cursor = Some(a);
                }
                (None, Some(_)) => {
                    let b = iter_b.next().unwrap();
                    *cursor = Some(b);
                }
                (Some(a), Some(b)) => {
                    if a.item.degree < b.item.degree {
                        let a = iter_a.next().unwrap();
                        *cursor = Some(a);
                    } else {
                        let b = iter_b.next().unwrap();
                        *cursor = Some(b);
                    }
                }
            }
            cursor = &mut cursor.as_deref_mut().unwrap().next;
        }
        new_list
    }
}

impl<I: Item> MinHeap for BinomialHeap<I> {
    type Item = I;

    fn peek_min(&self) -> Option<&Self::Item> {
        let mut min: Option<&Self::Item> = None;
        let mut curr_tree = &self.list.head;
        while let Some(ListNode {
            item: tree,
            next: next_tree,
        }) = curr_tree.as_deref()
        {
            if let Some(TreeNode {
                item,
                child: _,
                sibling: _,
            }) = tree.root.as_deref()
            {
                if min.is_none_or(|min| item < min) {
                    min = Some(item);
                }
            }
            curr_tree = next_tree;
        }

        min
    }

    fn extract_min(&mut self) -> Option<Self::Item> {
        todo!()
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
                let mut first_half = items;
                let second_half = first_half.split_off(first_half.len() / 2);

                Self::meld(Self::heapify(first_half), Self::heapify(second_half))
            }
        }
    }

    fn meld(heap_a: Self, heap_b: Self) -> Self {
        let merged = Self::merge_by_degree(heap_a.list, heap_b.list);
        if merged.head.is_none() {
            return Self::make_heap();
        }
        let mut merged = merged.into_iter().peekable();
        let mut prev = None;
        let mut curr = merged.next();
        let mut next = merged.next();

        while let (Some(curr_tree), Some(next_tree)) = (curr, next) {
            if curr_tree.item.degree != next_tree.item.degree
                || (merged
                    .peek()
                    .is_some_and(|next_next| next_next.item.degree == curr_tree.item.degree))
            {
                prev = Some(curr_tree);
                curr = Some(next_tree);
            } else {
                curr = Some(next_tree)
            }
            // else if (&curr_tree.item.root)
            //     .zip(next_tree.item.root)
            //     .is_some_and(|(x, next_x)| x.item <= next_x.item)
            // {
            // }
            next = merged.next();
        }

        todo!()
    }
}
