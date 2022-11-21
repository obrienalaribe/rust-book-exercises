//! In the portion of the test, you will write a few functions that operate on Vectors.
//! The algorithms include sorts, searches, and reversals.
//! In addition to understanding the algorithm, this will test your understanding of Rust's
//! ownership model.

/// This is an in-place sort, so it moves data around in the original slice.
/// You may look up how the algorithm works (eg here https://en.wikipedia.org/wiki/Bubble_sort)
/// But you may not look up implementations of it in Rust.
pub fn bubble_sort(items: &mut [u32]) {
	todo!()
}

/// This is a recursive sort, so you must use recursion.
/// This is NOT an in-place sort, so it will return a copy of the data in a new Vec.
/// You may look up how the algorithm works (eg here https://en.wikipedia.org/wiki/Merge_sort)
/// But you may not look up implementations of it in Rust.
pub fn merge_sort(items: &[u32]) -> Vec<u32> {
	todo!()
}

/// Reverse a slice in-place.
/// This is what the built-in `reverse` method does. You may NOT use that method here
/// https://doc.rust-lang.org/std/primitive.slice.html#method.reverse
pub fn reverse_in_place(items: &mut [u32]) {
	todo!()
}

/// Create and return a Vec containing the same data as the parameter slice in reverse order.
pub fn reverse_copy(items: &[u32]) -> Vec<u32> {
	todo!()
}

/// Search a slice for a particular item. Return the index of the first occurrence of that item.
/// You may look up how the algorithm works (eg here https://en.wikipedia.org/wiki/Linear_search)
/// But you may not look up implementations of it in Rust.
/// This is what the built-in `contains` method does. You may NOT use that method here.
/// https://doc.rust-lang.org/std/primitive.slice.html#method.contains
pub fn linear_search(items: &[u32], target: &u32) -> Option<usize> {
	todo!()
}

/// Search a slice for a particular item. Return the index of any occurrence of that item.
/// You may look up how the algorithm works (eg here https://en.wikipedia.org/wiki/Binary_search)
/// You may (and must) assume that the data is sorted.
/// But you may not look up implementations of it in Rust.
/// This is what the built-in `binary_search` method does. You may NOT use that method here.
/// https://doc.rust-lang.org/std/primitive.slice.html#method.binary_search
pub fn binary_search(items: &[u32], target: &u32) -> Option<usize> {
	todo!()
}
