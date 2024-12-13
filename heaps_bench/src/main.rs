use std::{
    cmp::Ordering,
    iter,
    time::{Duration, Instant},
};

use heaps::{BinaryHeap, BinomialHeap, Item, LazyBinomialHeap, MinHeap};
use rand::{thread_rng, Fill, Rng};

#[derive(Clone)]
struct ByteArr<const N: usize>([u8; N]);
impl<const N: usize> ByteArr<N> {
    fn new_rand() -> Self {
        let mut arr = [0; N];
        arr.try_fill(&mut thread_rng()).unwrap();
        Self(arr)
    }
}

impl<const N: usize> Fill for ByteArr<N> {
    fn try_fill<R: Rng + ?Sized>(&mut self, rng: &mut R) -> Result<(), rand::Error> {
        self.0.try_fill(rng)
    }
}
impl<const N: usize> PartialEq for ByteArr<N> {
    fn eq(&self, other: &Self) -> bool {
        self.0.eq(&other.0)
    }
}
impl<const N: usize> Eq for ByteArr<N> {}
impl<const N: usize> PartialOrd for ByteArr<N> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl<const N: usize> Ord for ByteArr<N> {
    fn cmp(&self, other: &Self) -> Ordering {
        for i in 0..N {
            let cmp = self.0[i].cmp(&other.0[i]);
            if !cmp.is_eq() {
                return cmp;
            }
        }
        Ordering::Equal
    }
}

pub fn heap_sort<I, T: MinHeap<Item = I>>(items: Vec<I>) -> Vec<I> {
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
    if sorted_by_heap != sorted {
        panic!();
    }
    duration
}

fn benchmark_all_heaps<T: Item + Clone>(name: &str, array: &[T], sorted: &[T]) {
    println!("{}: elem_size={} n={}", name, size_of::<T>(), array.len());

    let duration = benchmark_sort::<T, BinaryHeap<T>>(array, sorted);
    println!("    Binary Heap: duration={:?}", duration);

    let duration = benchmark_sort::<T, BinomialHeap<T>>(array, sorted);
    println!("    Binomial Heap: duration={:?}", duration);

    let duration = benchmark_sort::<T, LazyBinomialHeap<T>>(array, sorted);
    println!("    Laxy One-Pass Binomial Heap: duration={:?}", duration);
}

fn random_byte_arrays<const M: usize>(len: usize) -> Vec<ByteArr<M>> {
    let mut vec: Vec<ByteArr<M>> = Vec::with_capacity(len);
    for _ in 0..len {
        vec.push(ByteArr::<M>::new_rand());
    }
    vec
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let n: usize = args
        .get(1)
        .and_then(|str| str.parse().ok())
        .expect("Size of arrays n not specified");

    let constant: Vec<u32> = vec![0; n];
    benchmark_all_heaps("Constant 0 u32 array", &constant, &constant);

    let in_order: Vec<u32> = (0..n as u32).collect();
    let in_reverse: Vec<u32> = (0..n as u32).rev().collect();

    benchmark_all_heaps("In order u32 array", &in_order, &in_order);
    benchmark_all_heaps("In reverse u32 array", &in_reverse, &in_order);

    let looping_u8s: Vec<u8> = iter::repeat(0..=255).flatten().take(n).collect(); // 0,1,...,254,255,0,1,...
    let mut sorted_u8s = looping_u8s.clone();
    sorted_u8s.sort_unstable();

    benchmark_all_heaps("Looping u8 array", &looping_u8s, &sorted_u8s);

    let in_order: Vec<u64> = (0..n as u64).collect();
    let in_reverse: Vec<u64> = (0..n as u64).rev().collect();

    benchmark_all_heaps("In order u64 array", &in_order, &in_order);
    benchmark_all_heaps("In reverse u64 array", &in_reverse, &in_order);

    let in_order: Vec<u128> = (0..n as u128).collect();
    let in_reverse: Vec<u128> = (0..n as u128).rev().collect();

    benchmark_all_heaps("In order u128 array", &in_order, &in_order);
    benchmark_all_heaps("In reverse u128 array", &in_reverse, &in_order);

    let array = random_byte_arrays::<100>(n);
    let mut sorted = array.clone();
    sorted.sort();
    benchmark_all_heaps("Random byte arrays", &array, &sorted);

    let array = random_byte_arrays::<1000>(n);
    let mut sorted = array.clone();
    sorted.sort();
    benchmark_all_heaps("Random byte arrays", &array, &sorted);

    let array = random_byte_arrays::<10000>(n);
    let mut sorted = array.clone();
    sorted.sort();
    benchmark_all_heaps("Random byte arrays", &array, &sorted);

    let array = random_byte_arrays::<50000>(n);
    let mut sorted = array.clone();
    sorted.sort();
    benchmark_all_heaps("Random byte arrays", &array, &sorted);
}
