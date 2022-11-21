//! This portion will test your familiarity with some of Rust's common traits
//! and your ability to implement those traits in interesting ways
//! You need to remove all of the `todo!()`s. Most of them will need to be replaced
//! by some code, but some may be able to simply be deleted.

// NOTE: You will need to `use` something from the standard library to implement `Ord` and
// `PartialOrd` here.

/// A record of an employee at a particular company
#[derive(Debug)]
pub struct Employee {
	/// The name the person likes to be called. Doesn't have to be their "passport name"
	pub name: String,
	/// Amount of experience (in months) the person has working at this company
	pub experience: u32,
	/// Hourly wage paid to this employee
	pub wage: u32,
	/// Unique identifier for this employee
	pub uid: u32,
}

// We want to consider two employee instances equal iff they have the same `uid`.

// impl PartialEq for Employee {
//     todo!()
// }

// impl Eq for Employee {
//     todo!()
// }

// We want to sort employees. First and foremost, employees are equal if they have the same
// `uid`, as explained above. For employees who are not equal, we sort by the value they
// bring to the company. Value is defined as the quotient of the experience they've acquired
// at the company divided by their wage. Use integer division for the purpose of this calculation.

// impl PartialOrd for Employee {
//     todo!()
// }

// impl Ord for Employee {
//     todo!()
// }

// We want to parse employee information from a string data
// The string data should be comma separated. Here are a few examples:
//
// * "Billy, 4, 5, 345" - Billy has been working here for 4 months and earns 5 token per hour. She
//   is employee #345
// * "Jose, 12, 6, 1" - Jose has been working here for 1 year (12 months) and earns 6
// tokens per hour. He is employee #1
//
// Any strings that have the wrong number of commas of numbers too big for `u32` may return `None`

//impl TryFrom<String> for Employee  {
//     type Error = ();
//
//     todo!()
// }

// We also want to convert employees back into strings in the same format

// impl From<Employee> for String {
//     todo!()
// }
