//! Complete the following functions using the pattern matching syntax. That includes the `match`
//! statement of the `matches!()` marco, if you feel like have a 1-liner.
//!
//! You can try and write them imperatively at first as well, but at the end of the day, we highly
//! prefer you to write them using the `match` or `matches!`.

/// Returns true if the last two strings in the vector start with `PBA`.
pub fn match_1(input: Vec<String>) -> bool {
	if input.len() < 2 {return false}
	match input {
		s if check_prefix(&s[s.len()-1]) && check_prefix(&s[s.len()-2]) => { true }
		_ => { false}
	}
}

/// Returns true if the first and last string in the vector start with `PBA`.
pub fn match_2(input: Vec<String>) -> bool {
	if input.len() < 2 {return false}
	match input {
		s if check_prefix(&s[0]) && check_prefix(&s[s.len()-1]) => { true }
		_ => { false }
	}
}

/// Returns true if the first item in `input` is true.
pub fn match_3(input: (bool, bool, bool)) -> bool {
	match input {
		v if input.0 == true => { true }
		_ => {false}
	}
}

/// Returns true if the input is `Ok(x)` of some even `x`.
pub fn match_4(input: Result<u32, &'static str>) -> bool {
	match input {
		Ok(n) => { return n % 2 == 0 }
		Err(_) => { return false }
	}
}

fn check_prefix(string: &str ) -> bool {
	return string.to_lowercase().starts_with("pba");
}