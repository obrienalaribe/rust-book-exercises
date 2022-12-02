//! In this portion, you will write a path finding algorithm that finds a path through a network of
//! hiking trails that connect several campsites.
//!
//! Some trails are inherently one way. For example they may involve sliding down
//! a zipline, or diving off a cliff into a lake, which cannot be traveled in the opposite
//! direction. Other trails are normal hiking trails and can be hiked in either direction, but even
//! these trails may have different difficulties depending on the direction of travel. For example
//! they may have a steep hill and hiking downhill is easier than hiking uphill. For these reasons,
//! ALL TRAILS ARE MODELED AS ONE-WAY. If a hiking trail can be traveled in either direction, we
//! model it as two separate one-way trails.
//!
//! Your algorithm should be general enough that it can optimize the route for several different
//! desirable properties. For example, you should be able to find:
//!
//! * Shortest Distance
//! * Shortest travel time
//! * Least danger
//!
//! You may implement any path-finding algorithm you see fit for this portion, but if you don't
//! already know of one you like, we recommend studying and implementing [Dijkastra's Algorithm](https://en.wikipedia.org/wiki/Dijkstra%27s_algorithm).
//! > REMINDER: you may _not_ search for an algorithm in Rust for reference or to depend on here,
//! > per the honor code!

// NOTE: you may use any data structure you like from `std::collections`

/// Various types of terrain that may be encountered while traversing the trail network.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Terrain {
	PavedTrail,
	UnpavedTrail,
	RockyTrail,
	Water,
	Zipline,
	RopeBridge,
}

impl TryFrom<String> for Terrain {
	type Error = ();

	fn try_from(s: String) -> Result<Terrain, ()> {
		// String encodings of terrain variants are just their names.
		// This problem is OPTIONAL
		match s.as_str() {
			"UnpavedTrail" => { Ok(Terrain::UnpavedTrail) }
			"PavedTrail" => { Ok(Terrain::PavedTrail) }
			"RockyTrail" => { Ok(Terrain::RockyTrail) }
			"Water" => { Ok(Terrain::Water) }
			"Zipline" => { Ok(Terrain::Zipline) }
			"RopeBridge" => { Ok(Terrain::RopeBridge) }
			_ => {Err(())}
		}
	}
}

/// A level of skill that a hiker may attain in various forms of hiking.
#[derive(Debug, PartialEq, Eq)]
pub enum Skill {
	Beginner,
	Intermediate,
	Expert,
}

impl TryFrom<String> for Skill {
	type Error = ();

	/// OPTIONAL
	fn try_from(s: String) -> Result<Skill, ()> {
		// String encodings of skill variants are just their names.
		// This problem is OPTIONAL
		match s.as_str() {
			"Beginner" => { Ok(Skill::Beginner) }
			"Intermediate" => { Ok(Skill::Intermediate) }
			"Expert" => { Ok(Skill::Expert) }
			_ => {Err(())}
		}
	}
}

/// A one-way trail that can be traveled from the starting campsite to the ending campsite.
/// Every campsite in the network has a unique name.
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Trail {
	/// The name of the campsite where this trail begins
	pub start: String,
	/// The name of the trail where this campsite ends
	pub end: String,
	/// The distance traveled along the trail in meters
	pub distance: u32,
	/// The type of terrain that must be traversed in this trail
	pub terrain: Terrain,
	/// The danger encountered along the way. A Danger rating is a number between 0 and 255.
	/// More dangerous paths have a higher danger rating. For example, a flat paved trail has a
	/// danger rating of 0. A trail that involves diving off a cliff, or passing through mountain
	/// lion territory has a danger rating over 200.
	pub danger: u8,
}

// Parse trail data from a string.
// It is recommended to use your implementation for Terrain as a helper here.
impl TryFrom<String> for Trail {
	type Error = ();

	/// OPTIONAL
	fn try_from(s: String) -> Result<Trail, ()> {
		let mut value = String::new();

		let mut items: Vec<String> = vec![];
		for (i, c) in s.chars().enumerate() {
			if c.is_alphanumeric() || c == ' ' {
				&value.push(c);
				continue
			}
			let sanitized_value_is_empty = value.trim().is_empty();
			if !sanitized_value_is_empty {
				&items.push(value.trim().to_string());
				value = String::new();
			}
		}
		if items.len() == 5 {
			return Ok(Trail {
				start: items[0].to_string(),
				end: items[1].to_string(),
				distance: items[2].parse().unwrap(),
				terrain: Terrain::try_from(items[3].to_string()).unwrap(),
				danger: items[4].parse().unwrap()
			})
		}
		Err(())
	}
}

/// A hiker that is interested in traversing the trail network.
/// Hikers' skills are rated in multiple dimensions.
///
/// These ratings affect how fast a hiker can traverse various terrain, and in some cases, whether
/// they can traverse it at all.
#[derive(Debug, PartialEq, Eq)]
pub struct Hiker {
	/// The hikers ability to traverse trails on foot.
	pub hiking: Skill,
	/// The hikers ability to traverse trails over water.
	pub swimming: Skill,
	/// Whether the hiker is strong enough to hold their own body weight.
	pub strong: bool,
	/// Whether the hiker is brave enough to traverse scary, dangerous, or high elements.
	pub brave: bool,
}

// The default hiker is used in several of the functions below.
// Their name is Bill.
impl Default for Hiker {
	fn default() -> Self {
		Self { strong: true, brave: false, hiking: Skill::Intermediate, swimming: Skill::Expert }
	}
}

impl TryFrom<String> for Hiker {
	type Error = ();

	/// OPTIONAL
	/// The format for a hiker is a simple comma separated list of keys and values.
	/// The keys MUST be in the right order or the string is invalid.
	///
	/// Example:
	/// "hiking: Beginner, swimming: Intermediate, strong: false, brave: false"
	fn try_from(value: String) -> Result<Self, ()> {
		// This problem is OPTIONAL
		let mut word = String::new();
		let mut items: Vec<String> = vec![];
		for (i, c) in value.chars().enumerate() {
			if c.is_alphanumeric() || c == ' ' {
				&word.push(c);
				continue
			}
			let sanitized_word_is_empty = word.trim().is_empty();
			if !sanitized_word_is_empty {
				&items.push(word.trim().to_string());
				word = String::new();

			}
		}

		&items.push(value.split(" ").collect::<Vec<&str>>().last().unwrap().trim().to_string());

		if items.len() == 8 {
			return Ok(Hiker {
				hiking: Skill::try_from(items[1].to_string()).unwrap(),
				swimming: Skill::try_from(items[3].to_string()).unwrap(),
				strong: items[5].parse().unwrap(),
				brave: items[7].parse().unwrap()
			});
		}
		Err(())

	}
}

#[derive(Debug)]
struct TravelTime<'a> {
	hiker: &'a Hiker,
	terrain: Terrain,
	distance: u32,
}

// Swim
const BEGINNER_SWIMMER_MULTIPLIER: u32 = 9;
const INTERMEDIATE_SWIMMER_MULTIPLIER: u32 = 6;
const EXPERT_SWIMMER_MULTIPLIER: u32 = 3;

// Hiking
const BEGINNER_ROCKY_MULTIPLIER: u32 = 4;
const INTERMEDIATE_ROCKY_MULTIPLIER: u32 = 2;
const ADVANCED_UNPAVED_MULTIPLIER: f32 = 0.5;
const BEGINNER_UNPAVED_MULTIPLIER: u32 = 2;
const ADVANCED_PAVED_MULTIPIER: f32 = 0.25;
const INTERMEDIATE_PAVED_MULTIPIER: f32 = 0.5;

// Zipline
const HIKER_ZIPLINE_MULTIPLIER :f32 = 0.1;

impl Hiker {
	/// Returns the time it takes a hiker to traverse a trail, if they can traverse it at all.
	/// If the hiker cannot traverse the terrain, return None.
	///
	/// ### Rope  Bridges:
	/// Hikers can only traverse a rope bridge if they are brave. When traversing a rope bridge,
	/// the travel time is equal to the length of the bridge.
	///
	/// ### Zip Lines:
	/// Hikers can only traverse ziplines if they are both brave and strong. When traversing a
	/// zipline, the travel time is 1/10 of the length of the zipline (use integer division).
	///
	/// ### Water:
	/// All hikers can swim, but not at the same level.
	/// - Beginner swimmer travel time is 9X the length of the swim.
	/// - Intermediate swimmer travel time is 6X the length of the swim.
	/// - Expert swimmer travel time is 3X the length of the swim.
	///
	/// ### Hiking:
	/// All hikers can hike! But the travel time depends on both the terrain and the hikers ability.
	/// Use this table to find a hiker's speed multiplier. For multipliers less than 1, use integer
	/// division.
	///           Advanced | Intermediate | Beginner |
	///         | -------- | ------------ | -------- |
	///   Rocky |    1X    |      2X      |    4X    |
	/// Unpaved |   1/2X   |      1X      |    2X    |
	///   Paved |   1/4X   |     1/2X     |    1X    |
	pub fn travel_time(&self, terrain: &Terrain, distance: u32) -> Option<u32> {

		let travel_time = TravelTime {hiker: self, terrain: *terrain, distance: distance};

		match travel_time  {
			tt if tt.terrain == Terrain::Water && tt.hiker.swimming == Skill::Beginner => { return Some(BEGINNER_SWIMMER_MULTIPLIER * distance) }
			tt if tt.terrain == Terrain::Water && tt.hiker.swimming == Skill::Intermediate => { return Some(INTERMEDIATE_SWIMMER_MULTIPLIER * distance) }
			tt if tt.terrain == Terrain::Water && tt.hiker.swimming == Skill::Expert => { return Some(EXPERT_SWIMMER_MULTIPLIER * distance) }

			tt if tt.hiker.hiking == Skill::Beginner && tt.terrain == Terrain::RockyTrail => { return Some(BEGINNER_ROCKY_MULTIPLIER * distance) }
			tt if tt.hiker.hiking == Skill::Intermediate && tt.terrain == Terrain::RockyTrail => { return Some(INTERMEDIATE_ROCKY_MULTIPLIER * distance) }
			tt if tt.hiker.hiking == Skill::Expert && tt.terrain == Terrain::RockyTrail => { return Some(distance) }

			tt if tt.hiker.hiking == Skill::Beginner && tt.terrain == Terrain::UnpavedTrail => { return Some(BEGINNER_UNPAVED_MULTIPLIER * distance) }
			tt if tt.hiker.hiking == Skill::Intermediate && tt.terrain == Terrain::UnpavedTrail => { return Some(distance) }
			tt if tt.hiker.hiking == Skill::Expert && tt.terrain == Terrain::UnpavedTrail => { return Some((ADVANCED_UNPAVED_MULTIPLIER * distance as f32 ) as u32) }

			tt if tt.hiker.hiking == Skill::Beginner && tt.terrain == Terrain::PavedTrail => { return Some(distance) }
			tt if tt.hiker.hiking == Skill::Intermediate && tt.terrain == Terrain::PavedTrail => { return Some((INTERMEDIATE_PAVED_MULTIPIER * distance as f32) as u32) }
			tt if tt.hiker.hiking == Skill::Expert && tt.terrain == Terrain::PavedTrail => { return Some((ADVANCED_PAVED_MULTIPIER * distance as f32) as u32) }

			tt if tt.hiker.brave && tt.hiker.strong => { return Some((HIKER_ZIPLINE_MULTIPLIER * distance as f32) as u32) }

			tt if tt.hiker.brave => { return Some(distance) }
			_ => { return None }
		};
	}
}

/// OPTIONAL
/// This is the main path-finding function. It should be abstract enough that it can solve all of
/// the more specific path finding problems below. That is, all of the following problem specific
/// functions, should call this function to do the heavy lifting.
///
/// Given a hiker, their starting and ending points, a trail network, and a function that determines
/// the cost of the hiker traversing a given trail, determine whether the hiker can reach the
/// destination at all, and if they can, the minimal total cost reaching the destination.
pub fn optimal_path(
	start: String,
	destination: String,
	hiker: &Hiker,
	trails: impl Iterator<Item = Trail>,
	cost_function: impl Fn(&Hiker, &Trail) -> Option<u32>,
) -> Option<u32> {
	// This problem is OPTIONAL
	todo!("OPTIONAL")
}

/// A specific path optimization problem. Bill wants to find the shortest (least distance) path from
/// "Green Lake" to "Prairie Meadows" though the given trail network.
///
/// Reminder: Bill is the Default::default() hiker.
pub fn bills_shortest_path_from_green_lake_to_prairie_meadows(
	trails: impl Iterator<Item = Trail>,
) -> Option<u32> {
	todo!("Call optimal_path here")
}

/// A specific path optimization problem. Bill wants to find the safest (least dangerous) path
/// between two arbitrary campsites.
///
/// Reminder: Bill is the Default::default() hiker.
pub fn bills_safest_path(
	start: String,
	destination: String,
	trails: impl Iterator<Item = Trail>,
) -> Option<u32> {
	todo!("Call optimal_path here")
}

/// A specific path optimization problem. Hikers often want to find the fastest (least travel time)
/// path from "Green Lake" to "Prairie Meadows" though the given trail network.
pub fn fastest_path_from_green_lake_to_prairie_meadows(
	hiker: &Hiker,
	trails: impl Iterator<Item = Trail>,
) -> Option<u32> {
	todo!("Call optimal_path here")
}
