## A library of meldable min-heaps in Rust

Inspired by my Enriched Data Structures and Analysis (CSC265) course during the fall term of 2024 with Professor Adrian She at the University of Toronto. I implemented a binary heap, a binomial heap, and a lazy one-pass binomial heap as generic containers for any type that implements the Ord trait.
The binary heap and binomial heap are textbook implementations of them. The lazy heap came from a CSC265 assignment, which took ideas from this paper: https://www.cs.princeton.edu/courses/archive/spr09/cos423/Lectures/rp-heaps.pdf.

They each implement the following operations with the following runtime complexities:

| Operation   | Binary      | Binomial    | Lazy Binomial |
| ----------- | ----------- | ----------- | ------------- |
| Make-Heap   | $O(1)$      | $O(1)$      | $O(1)$        | 
| Min         | $O(1)$      | $O(\log n)$ | $O(1)$        |
| Extract-Min | $O(\log n)$ | $O(\log n)$ | $O(\log n)$ amortized |
| Insert      | $O(\log n)$ | $O(\log n)$ | $O(1)$        |
| Meld        | $O(n)$      | $O(\log n)$ | $O(1)$        |
| Heapify     | $O(n)$      | $O(n)$      | $O(n)$        |

I wrote the algorithms for the operations to be simple and not particularly efficient. It was fine for me if they did unnecessary extra work as long as they maintained the time complexities above.

The binary heap and the binomial heap are written entirely in safe Rust, and the lazy heap uses some unsafe.

There is also a benchmarking binary sub-crate that can generate lists of arbitrary length and then time the different heaps in sorting the lists. The lists to sort can have elements of sizes 8, 16, 32, 64, 128 bits, or 1 or 10 KiB.
It can sort lists generated with elements in increasing order, decreasing order, random order, or with an equal value of 0 everywhere. According to my tests, sorting arrays is fastest with the binary heap. The binomial heap is about 10x slower, and the lazy binomial heap is even 2x slower than that.

The performance drop when sorting using the binomial and lazy heaps compared to the binary heap is expected. The binary heap is the simplest data structure of the three, and sorting uses only Heapify and Extract-Min operations. The three heaps have the same time complexity for these, but their more complicated implementations make the constant factors pretty big. If the workload tested used a lot of Meld operations instead, then the other heaps would start to overcome the binary heap once past a certain amount of elements. I didn't know any simple example of workloads that need many Meld operations, so I didn't test this further.
