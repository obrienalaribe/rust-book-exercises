// Imagine you have an outcome enum like this.

#[derive(Clone, PartialEq)]
pub enum Outcome {
	Ok,
	SomethingWentWrong,
	IDontKnow,
}

// A function takes some arbitrary input that's a collection of `T`, and processes each item
// individually. Each process can be an `Outcome`. We return `Vec<Outcome>`.

pub fn process_stuff<T>(input: impl Iterator<Item = T>) -> Vec<Outcome> {
	unimplemented!("You are not expected to implement this function");
}

// What we want to achieve is a quick way (in terms of lines of code) to scan the output and
// determine how many were okay, how many were error, etc.
//
// A bring solution follows 🫣:

pub fn ok_count(outcomes: Vec<Outcome>) -> usize {
	outcomes.iter().filter(|&x| *x == Outcome::Ok).count()
}
pub fn something_went_wrong_count(outcomes: Vec<Outcome>) -> usize {
	outcomes.iter().filter(|&x| *x == Outcome::SomethingWentWrong).count()
}
pub fn i_dont_know_count(outcomes: Vec<Outcome>) -> usize {
	outcomes.iter().filter(|&x| *x == Outcome::IDontKnow).count()
}
// This is quite lame. We want to be able to call these methods directly on the `Vec<Outcome>`. But
// how do we do this? We can't add a function to type `Vec`. This type is part of the standard
// library!
//
// Correct, but we can define a trait in the current module, and implement that for `Vec<_>`.
//
// This is a very common approach, and is called an "extension trait".

pub trait OutcomeCount {
	fn ok_count(&self) -> usize;
	fn something_went_wrong_count(&self) -> usize;
	fn i_dont_know_count(&self) -> usize;
}

// First, implement this trait.

impl OutcomeCount for Vec<Outcome> {
	fn ok_count(&self) -> usize {
		self.iter().filter(|&x| *x == Outcome::Ok).count()
	}
	fn i_dont_know_count(&self) -> usize {
		self.iter().filter(|&x| *x == Outcome::IDontKnow).count()
	}
	fn something_went_wrong_count(&self) -> usize {
		self.iter().filter(|&x| *x == Outcome::SomethingWentWrong).count()
	}
}

// Now we can call these functions directly on `Vec<Outcome>`.

// But all of that is a lot of boilerplate. Wouldn't it be nice to have a `derive` macro that
// exactly does this, on any enum?
//
// So, for any `enum Foo { X, Y, .. }`, `#[derive(CountOf)]` would generate a trait `CountOfFoo`,
// with functions named `fn x_count`, `fn y_count` etc. Finally, it would implement `CountOfFoo` for
// `Vec<Foo>`.
//
// And heck, you could then easily implement it for other collections of `Foo`, such as
// `HashMap<_, Foo>` etc.
