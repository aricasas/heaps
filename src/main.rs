use heaps::{naive_heap_sort, BinaryHeap};

fn main() {
    let items = vec![4, 6, 3, 2, 5, 64, 2, 1];
    let sorted = naive_heap_sort::<u32, BinaryHeap<u32>>(items);
    assert!(sorted.is_sorted());
    assert_eq!(sorted, [1, 2, 2, 3, 4, 5, 6, 64]);
}
