//! This portion of the exam tests your abilities to work with iterators and their functional-style
//! methods.
//!
//! Throughout this portion of the test, you may refer to https://doc.rust-lang.org/std/iter/trait.Iterator.html
//! and other docs about iterators. You may NOT look up specific implementations for these problems
//! in Rust or any other Functional language.
//!
//! If you find that you simply cannot write these methods in the functional style using iterator
//! methods, writing them in the imperative style with loops will still earn partial credit.

/// This function takes an iterator of u32 values, squares each value, and returns the sum of the
/// squares. You may assume that no individual square, nor the entire sum, overflows the u32 type.
pub fn sum_of_squares(vals: impl Iterator<Item = u32>) -> u32 {
	todo!()
}

/// This function takes an iterator of i32 values, calculates the absolute value of each, and throws
/// away any values that are greater than 100. The remaining positive values are returned as an
/// iterator of u32s.
pub fn bounded_absolute_values(vals: impl Iterator<Item = i32>) -> impl Iterator<Item = u32> {
	// You should remove the following line (and this comment). It is just there because the
	// compiler doesn't allow todo!() when the return type is impl Trait
	Vec::new().into_iter()
}

// We allow `unused_mut` only so that there is no build warning on the starter code.
// You should remove this line once you have completed the following function
#[allow(unused_mut)]
/// This function takes an iterator of u32 values. The first value in the iterator, call it n, is
/// special: it represents the maximum count of the resultant iterator. Once n is known, create an
/// iterator that yields the first n even values from the remainder of the input iterator.
///
/// If the input iterator is empty, return None
/// If there are fewer than n even values left in the input, return as many as possible
pub fn first_n_even(mut vals: impl Iterator<Item = u32>) -> Option<impl Iterator<Item = u32>> {
	// You should remove the following line (and this comment). It is just there because the
	// compiler doesn't allow todo!() when the return type is impl Trait
	Some(Vec::new().into_iter())
}

/// Return an "infinite" iterator that yields the squares of the whole numbers.
/// For example, the first few values should be 0, 1, 4, 9, 16, 25, ...
///
/// The iterator should be bounded only by the u32 type, not by your code
pub fn square_whole_numbers() -> impl Iterator<Item = u32> {
	// You should remove the following line (and this comment). It is just there because the
	// compiler doesn't allow todo!() when the return type is impl Trait
	Vec::new().into_iter()
}

/// An iterator that generates the Fibonacci sequence.
#[derive(Default)]
pub struct Fibonacci {
	/// The most recent value this iterator has yielded
	prev: Option<u32>,
	/// The second most recent value that this iterator has yielded
	prev_prev: Option<u32>,
}

impl Iterator for Fibonacci {
	type Item = u32;

	fn next(&mut self) -> Option<u32> {
		todo!()
	}
}
