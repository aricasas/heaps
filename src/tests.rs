use crate::MinHeap;

pub fn simple<H: MinHeap<Item = u32>>() {
    let mut heap = H::make_heap();
    heap.insert(3);
    heap.insert(2);
    heap.insert(1);

    assert_eq!(heap.peek_min(), Some(&1));

    assert_eq!(heap.extract_min(), Some(1));
    assert_eq!(heap.extract_min(), Some(2));
    assert_eq!(heap.extract_min(), Some(3));
    assert_eq!(heap.extract_min(), None);
}

pub fn heapify<H: MinHeap<Item = u32>>() {
    let mut heap = H::heapify(vec![3, 2, 1]);
    assert_eq!(heap.extract_min(), Some(1));
    assert_eq!(heap.extract_min(), Some(2));
    assert_eq!(heap.extract_min(), Some(3));
    assert_eq!(heap.extract_min(), None);

    heap = H::heapify(vec![]);
    assert_eq!(heap.extract_min(), None);
}

pub fn meld<H: MinHeap<Item = u32>>() {
    let mut heap_a = H::make_heap();
    heap_a.insert(0);
    heap_a.insert(3);
    heap_a.insert(6);

    let mut heap_b = H::make_heap();
    heap_b.insert(1);
    heap_b.insert(4);
    heap_b.insert(7);

    let mut heap_c = H::make_heap();
    heap_c.insert(2);
    heap_c.insert(5);
    heap_c.insert(8);

    let heap_ac = H::meld(heap_a, heap_c);
    let mut heap_abc = H::meld(heap_ac, heap_b);

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

    heap_a = H::make_heap();
    heap_b = H::make_heap();
    let mut heap_ab = H::meld(heap_a, heap_b);
    assert_eq!(heap_ab.extract_min(), None);
}

pub fn sort<H: MinHeap<Item = u32>>() {
    let items = vec![4, 6, 3, 2, 5, 64, 2, 1];
    let sorted = crate::naive_heap_sort::<u32, H>(items);
    assert!(sorted.is_sorted());
    assert_eq!(sorted, [1, 2, 2, 3, 4, 5, 6, 64]);
}
