mod benchmarking;
mod byte_array;

use benchmarking::benchmark_heaps_sort;
use byte_array::ByteArray;
use clap::{Parser, ValueEnum};
use rand::{thread_rng, Rng};

#[derive(Parser)]
struct Cli {
    /// Number of elements in array, can be passed multiple numbers for several runs
    #[arg(short, required = true, num_args = 1..)]
    n: Vec<usize>,

    /// Size of array elements
    #[arg(short, long)]
    size: ArrayElemType,

    /// How the values in the array are generated
    #[arg(short, long)]
    mode: ArrayMode,

    /// Benchmark all heap implementations
    #[arg(short, long)]
    all: bool,
    /// Benchmark binary heap implementation
    #[arg(long)]
    binary: bool,
    /// Benchmark binomial heap implementation
    #[arg(long)]
    binomial: bool,
    /// Benchmark lazy one-pass binomial heap implementation
    #[arg(long)]
    lazy: bool,
}

#[derive(Clone, Copy, ValueEnum)]
enum ArrayElemType {
    U8,
    U16,
    U32,
    U64,
    U128,
    /// 1 KiB size elements
    Big,
    /// 10 KiB size elements
    Bigger,
}

#[derive(Clone, Copy, ValueEnum, Debug)]
enum ArrayMode {
    Random,
    IncreasingWrapped,
    DecreasingWrapped,
    Zero,
}

trait BenchElemType: heaps::Item + Sized {
    fn zero_array(len: usize) -> Vec<Self>;
    fn increasing_iter(len: usize) -> Vec<Self>;
    fn decreasing_iter(len: usize) -> Vec<Self>;
    fn random_iter(len: usize) -> Vec<Self>;
    fn build_array_with_mode(len: usize, mode: ArrayMode) -> Vec<Self> {
        match mode {
            ArrayMode::Random => Self::random_iter(len),
            ArrayMode::IncreasingWrapped => Self::increasing_iter(len),
            ArrayMode::DecreasingWrapped => Self::decreasing_iter(len),
            ArrayMode::Zero => Self::zero_array(len),
        }
    }
}

fn main() {
    let options = Cli::parse();
    let mut binary = options.binary;
    let mut binomial = options.binomial;
    let mut lazy = options.lazy;
    if options.all || (!binary && !binomial && !lazy) {
        binary = true;
        binomial = true;
        lazy = true;
    }

    for len in options.n {
        match options.size {
            ArrayElemType::U8 => {
                benchmark_heaps_sort::<u8>(options.mode, len, binary, binomial, lazy);
            }
            ArrayElemType::U16 => {
                benchmark_heaps_sort::<u16>(options.mode, len, binary, binomial, lazy);
            }
            ArrayElemType::U32 => {
                benchmark_heaps_sort::<u32>(options.mode, len, binary, binomial, lazy);
            }
            ArrayElemType::U64 => {
                benchmark_heaps_sort::<u64>(options.mode, len, binary, binomial, lazy);
            }
            ArrayElemType::U128 => {
                benchmark_heaps_sort::<u128>(options.mode, len, binary, binomial, lazy);
            }
            ArrayElemType::Big => {
                type T = ByteArray<1024>;
                benchmark_heaps_sort::<T>(options.mode, len, binary, binomial, lazy);
            }
            ArrayElemType::Bigger => {
                type T = ByteArray<10240>;
                benchmark_heaps_sort::<T>(options.mode, len, binary, binomial, lazy);
            }
        }
    }
}

impl BenchElemType for u8 {
    fn zero_array(len: usize) -> Vec<Self> {
        vec![0; len]
    }
    fn increasing_iter(len: usize) -> Vec<Self> {
        (0..len).map(|i| i as Self).collect()
    }
    fn decreasing_iter(len: usize) -> Vec<Self> {
        (0..len).rev().map(|i| i as Self).collect()
    }
    fn random_iter(len: usize) -> Vec<Self> {
        let mut array = vec![0; len];
        thread_rng().fill(&mut array[..]);
        array
    }
}
impl BenchElemType for u16 {
    fn zero_array(len: usize) -> Vec<Self> {
        vec![0; len]
    }
    fn increasing_iter(len: usize) -> Vec<Self> {
        (0..len).map(|i| i as Self).collect()
    }
    fn decreasing_iter(len: usize) -> Vec<Self> {
        (0..len).rev().map(|i| i as Self).collect()
    }
    fn random_iter(len: usize) -> Vec<Self> {
        let mut array = vec![0; len];
        thread_rng().fill(&mut array[..]);
        array
    }
}
impl BenchElemType for u32 {
    fn zero_array(len: usize) -> Vec<Self> {
        vec![0; len]
    }
    fn increasing_iter(len: usize) -> Vec<Self> {
        (0..len).map(|i| i as Self).collect()
    }
    fn decreasing_iter(len: usize) -> Vec<Self> {
        (0..len).rev().map(|i| i as Self).collect()
    }
    fn random_iter(len: usize) -> Vec<Self> {
        let mut array = vec![0; len];
        thread_rng().fill(&mut array[..]);
        array
    }
}
impl BenchElemType for u64 {
    fn zero_array(len: usize) -> Vec<Self> {
        vec![0; len]
    }
    fn increasing_iter(len: usize) -> Vec<Self> {
        (0..len).map(|i| i as Self).collect()
    }
    fn decreasing_iter(len: usize) -> Vec<Self> {
        (0..len).rev().map(|i| i as Self).collect()
    }
    fn random_iter(len: usize) -> Vec<Self> {
        let mut array = vec![0; len];
        thread_rng().fill(&mut array[..]);
        array
    }
}
impl BenchElemType for u128 {
    fn zero_array(len: usize) -> Vec<Self> {
        vec![0; len]
    }
    fn increasing_iter(len: usize) -> Vec<Self> {
        (0..len).map(|i| i as Self).collect()
    }
    fn decreasing_iter(len: usize) -> Vec<Self> {
        (0..len).rev().map(|i| i as Self).collect()
    }
    fn random_iter(len: usize) -> Vec<Self> {
        let mut array = vec![0; len];
        thread_rng().fill(&mut array[..]);
        array
    }
}
