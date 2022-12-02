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
	return vals.map(|x| x * x).sum();
}

/// This function takes an iterator of i32 values, calculates the absolute value of each, and throws
/// away any values that are greater than 100. The remaining positive values are returned as an
/// iterator of u32s.
pub fn bounded_absolute_values(vals: impl Iterator<Item = i32>) -> impl Iterator<Item = u32> {
	vals.filter(|&x: &i32| x.abs() <= 100).map(|x| x.abs() as u32)
}


/// This function takes an iterator of u32 values. The first value in the iterator, call it n, is
/// special: it represents the maximum count of the resultant iterator. Once n is known, create an
/// iterator that yields the first n even values from the remainder of the input iterator.
///
/// If the input iterator is empty, return None
/// If there are fewer than n even values left in the input, return as many as possible
pub fn first_n_even(mut vals: impl Iterator<Item = u32>) -> Option<impl Iterator<Item = u32>> {

	//check if first element exist
	match vals.next().unwrap_or(0) {
		first_element if first_element > 0 => {
			let n_even_values = vals.filter(|&x| x % 2 == 0)
				.take(first_element as usize).collect::<Vec<u32>>();
			match n_even_values.len() {
				even_values if even_values > 0  => {
					Some(n_even_values.into_iter())
				}
				// No even values in collection
				_ => { return Some(n_even_values.into_iter())  }
			}
		}
		// Input iterator is empty
		_ => { return  None }
	}
}


/// Return an "infinite" iterator that yields the squares of the whole numbers.
/// For example, the first few values should be 0, 1, 4, 9, 16, 25, ...
///
/// The iterator should be bounded only by the u32 type, not by your code
pub fn square_whole_numbers() -> impl Iterator<Item = u32> {
	return (0..).into_iter().map(|x| x * x);
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
		let mut sum = 0;

		if self.prev_prev == None{
			self.prev_prev = Some(0);
			return Some(0);
		} else if self.prev == None  {
			self.prev = Some(1);
			return Some(1);
		}
		sum = self.prev_prev.unwrap() + self.prev.unwrap();
		self.prev_prev = self.prev;
		self.prev = Some(sum);
		return Some(sum);
	}
}
