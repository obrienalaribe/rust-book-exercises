//! This portion will test your familiarity with some of Rust's common traits
//! and your ability to implement those traits in interesting ways
//! You need to remove all of the `todo!()`s. Most of them will need to be replaced
//! by some code, but some may be able to simply be deleted.

// NOTE: You will need to `use` something from the standard library to implement `Ord` and
// `PartialOrd` here.

use std::cmp::Ordering;
use std::cmp::Ordering::{Equal, Greater, Less};

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

impl PartialEq for Employee {
	fn eq(&self, other: &Self) -> bool {
		self.uid == other.uid
	}
}

impl Eq for Employee {}

// We want to sort employees. First and foremost, employees are equal if they have the same
// `uid`, as explained above. For employees who are not equal, we sort by the value they
// bring to the company. Value is defined as the quotient of the experience they've acquired
// at the company divided by their wage. Use integer division for the purpose of this calculation.

impl PartialOrd for Employee {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		Some(self.cmp(&other))
	}
}

impl Ord for Employee {
	fn cmp(&self, other: &Self) -> Ordering {
		let self_exp_quotient = &self.experience / &self.wage;
		let other_exp_quotient = &other.experience / &other.wage;

		match self {
			_e if self_exp_quotient > other_exp_quotient => Greater,
			_e if self_exp_quotient < other_exp_quotient => Less,
			_ => Equal
		}
	}
}

// We want to parse employee information from a string data
// The string data should be comma separated. Here are a few examples:
//
// * "Billy, 4, 5, 345" - Billy has been working here for 4 months and earns 5 token per hour. She
//   is employee #345
// * "Jose, 12, 6, 1" - Jose has been working here for 1 year (12 months) and earns 6
// tokens per hour. He is employee #1
//
// Any strings that have the wrong number of commas of numbers too big for `u32` may return `None`

impl TryFrom<String> for Employee  {
	type Error = ();

	fn try_from(value: String) -> Result<Self, Self::Error> {
		let input:Vec<&str> = value.split(",").collect();
		if input.len() == 4 {
			let name = input[0].trim().to_string();

			// Parse string into u32 on best case otherwise handle & return 0 for unknowns
			let experience:u32 = match input[1].trim().parse() { Ok(v) => v, Err(_) => 0 };
			let wage:u32 = match input[2].trim().parse() { Ok(v) => v, Err(_) => 0 };
			let uid:u32 = match input[3].trim().parse() { Ok(v) => v, Err(_) => 0 };
			return Ok(Employee { name: name, experience: experience, wage: wage, uid: uid })
		}
		return Err(())
	}
}

// We also want to convert employees back into strings in the same format

impl From<Employee> for String {
	fn from(e: Employee) -> Self {
		return format!("{}, {}, {}, {}", e.name, e.experience, e.wage, e.uid).to_string();
	}
}
