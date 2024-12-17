use std::cmp::Ordering;

use rand::{thread_rng, RngCore};

use crate::BenchElemType;

/// Most significant digit stored at `arr[N-1]` and least significant digit stored at `arr[0]`
#[derive(Clone)]
pub struct ByteArray<const N: usize>([u8; N]);
impl<const N: usize> ByteArray<N> {
    fn zero() -> Self {
        Self([0; N])
    }
    fn new_rand() -> Self {
        let mut array = [0; N];
        thread_rng().fill_bytes(&mut array);
        Self(array)
    }
    fn from_usize(num: usize) -> Self {
        let mut array = [0u8; N];
        for (j, byte) in array.iter_mut().enumerate() {
            *byte = (num >> (j * 8)) as u8;
        }
        Self(array)
    }
}

impl<const N: usize> BenchElemType for ByteArray<N> {
    fn zero_array(len: usize) -> Vec<Self> {
        (0..len).map(|_| Self::zero()).collect()
    }

    fn increasing_iter(len: usize) -> Vec<Self> {
        (0..len).map(Self::from_usize).collect()
    }

    fn decreasing_iter(len: usize) -> Vec<Self> {
        (0..len).rev().map(Self::from_usize).collect()
    }

    fn random_iter(len: usize) -> Vec<Self> {
        (0..len).map(|_| Self::new_rand()).collect()
    }
}

impl<const N: usize> Ord for ByteArray<N> {
    fn cmp(&self, other: &Self) -> Ordering {
        // In Reverse to start comparisons at most significant digit
        for i in (0..N).rev() {
            let cmp = self.0[i].cmp(&other.0[i]);
            if !cmp.is_eq() {
                return cmp;
            }
        }
        Ordering::Equal
    }
}

impl<const N: usize> PartialEq for ByteArray<N> {
    fn eq(&self, other: &Self) -> bool {
        self.0.eq(&other.0)
    }
}
impl<const N: usize> Eq for ByteArray<N> {}
impl<const N: usize> PartialOrd for ByteArray<N> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
