/// A type representing distances in multiple units
#[derive(Clone, Copy)]
pub enum Distance {
	Meters(f32),
	Feet(f32),
	Miles(f32),
}

/// There are roughly 1609 meters in one mile
const METERS_PER_MILE: f32 = 1609.344;

/// There are roughly 3.3 feet in one meter
const FEET_PER_METER: f32 = 3.281;

impl Distance {
	/// Convert the given distance to meters
	pub fn to_meters(&self) -> Self {

	}

	/// Convert the given distance to feet
	pub fn to_feet(&self) -> Self {
		todo!()
	}

	/// Convert the given distance to miles
	pub fn to_miles(&self) -> Self {
		todo!()
	}
}
