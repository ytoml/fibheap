# Fibheap

Implementation of [fibonacci heap](https://en.wikipedia.org/wiki/Fibonacci_heap) in Rust.  
`FibonacciHeap` provides minimum heap on any type that implements `PartialOrd`.
It also provides `IntoIterator` to iterate over all elements.

## Example
```rust
use fibheap::FibonacciHeap;

const SIZE: usize = 100000;

let mut vec = Vec::with_capacity(SIZE);
for i in (0..SIZE).rev() {
    vec.push(i);
}

let mut heap = FibonacciHeap::from_vec(vec);
assert_eq!(heap.pop().unwrap(), 0);

for (i, v) in heap.into_iter().enumerate() {
    assert_eq!(i+1, v);
}
```