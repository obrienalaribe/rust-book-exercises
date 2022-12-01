use pba_entrance_exam::c_enums::*;

#[test]
fn feet_to_miles() {
	let feet = Distance::Feet(4.1);
	let miles = feet.to_miles();

	let expected_miles = 0.000776f32;

	match miles {
		Distance::Miles(converted_miles) => {
			let delta = f32::abs(converted_miles - expected_miles);
			assert!(delta < 0.00001);
		},
		_ => assert!(false),
	}
}

#[test]
fn meters_to_feet(){
	let meters = Distance::Meters(5.2);
	let feet = meters.to_feet();

	let expected_feet = 17.0604;

	match feet {
		Distance::Feet(converted_feet) => {
			let delta = f32::abs(converted_feet - expected_feet);
			assert!(delta < 0.001)
		},
		_ => assert!(false)
	}
}


#[test]
fn miles_to_meters(){
	let miles = Distance::Miles(2.3);
	let meters = miles.to_meters();
	let expected_meters = 3701.4912;

	match meters {
		Distance::Meters(converted_meters) => {
			let delta = f32::abs(converted_meters - expected_meters);
			assert!(delta < 0.001)
		},
		_ => assert!(false)
	}

}

#[test]
fn test_zero(){
	let miles = Distance::Miles(0.0);
	let meters = miles.to_meters();
	let expected = 0.0;

	match meters {
		Distance::Meters(converted_meters) => {
			let delta = f32::abs(converted_meters - expected);
			assert!(delta < 0.001)
		},
		_ => assert!(false)
	}
}