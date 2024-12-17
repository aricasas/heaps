use std::time::{Duration, Instant};

use heaps::{BinaryHeap, BinomialHeap, Item, LazyBinomialHeap, MinHeap};

use crate::{ArrayMode, BenchElemType};

fn heap_sort<I, T: MinHeap<Item = I>>(items: Vec<I>) -> Vec<I> {
    let length = items.len();
    let mut heap = T::heapify(items);
    let mut sorted = Vec::with_capacity(length);
    while let Some(x) = heap.extract_min() {
        sorted.push(x);
    }
    sorted
}

fn benchmark_sort<T: Item + Clone, H: MinHeap<Item = T>>(array: &[T], sorted: &[T]) -> Duration {
    let array = array.to_vec();
    let before = Instant::now();
    let sorted_by_heap = heap_sort::<T, H>(array);
    let duration = before.elapsed();
    assert!(sorted_by_heap == sorted);
    duration
}

pub fn benchmark_heaps_sort<T: Item + Clone + BenchElemType>(
    mode: ArrayMode,
    len: usize,
    binary: bool,
    binomial: bool,
    lazy: bool,
) {
    eprintln!("Generating {:?} array length {}", mode, len);
    let before = Instant::now();
    let array = T::build_array_with_mode(len, mode);
    let duration = before.elapsed();
    eprintln!("Generated array in {:?}", duration);

    let mut sorted = array.clone();

    eprintln!("Sorting {:?} array length {} with std", mode, len);
    let before = Instant::now();
    sorted.sort_unstable();
    let duration = before.elapsed();
    eprintln!("Sorted {:?} array length {} in {:?}", mode, len, duration);

    println!(
        "Heapsort {:?} array: n={} elem_size={}",
        mode,
        len,
        size_of::<T>(),
    );

    if binary {
        let duration = benchmark_sort::<T, BinaryHeap<T>>(&array, &sorted);
        println!("    Binary Heap: duration={:?}", duration);
    }
    if binomial {
        let duration = benchmark_sort::<T, BinomialHeap<T>>(&array, &sorted);
        println!("    Binomial Heap: duration={:?}", duration);
    }
    if lazy {
        let duration = benchmark_sort::<T, LazyBinomialHeap<T>>(&array, &sorted);
        println!("    Lazy One-Pass Binomial Heap: duration={:?}", duration);
    }
}
