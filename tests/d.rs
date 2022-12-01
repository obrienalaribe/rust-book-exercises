use pba_entrance_exam::d_pattern_matching::*;

#[test]
fn test_match_1_true() {
	let strs =
		vec!["Hello".to_string(), "World".to_string(), "PBA".to_string(), "PBASDFGR".to_string()];
	assert!(match_1(strs))
}

#[test]
fn test_match_1_empty_vec() {
	let strs =
		vec![];
	assert!(!match_1(strs))
}
#[test]
fn test_match_1_with_2_items() {
	let strs =
		vec!["PBA".to_string(), "PBASDFGR".to_string()];
	assert!(match_1(strs))
}

#[test]
fn test_match_1_multicase_true() {
	let strs =
		vec!["Hello".to_string(), "World".to_string(), "pBa".to_string(), "pbaSDFGR".to_string()];
	assert!(match_1(strs))
}

#[test]
fn test_match_1_multicase_with_symbol_true() {
	let strs =
		vec!["Hello".to_string(), "World".to_string(), "pBa".to_string(), "pba-SDFGR".to_string()];
	assert!(match_1(strs))
}

#[test]
fn test_match_1_one_item() {
	let strs = vec!["PBA".to_string()];
	assert!(!match_1(strs))
}


#[test]
fn test_match_1_false() {
	let strs = vec!["Hello".to_string(), "World".to_string(), "PBA".to_string()];
	assert!(!match_1(strs))
}

#[test]
fn test_match_2() {
	let strs = vec![
		"PBAHello".to_string(),
		"World".to_string(),
		"PBA".to_string(),
		"PBASDFGR".to_string(),
	];
	assert!(match_2(strs))
}
#[test]
fn test_match_2_empty_vec() {
	let strs = vec![];
	assert!(!match_2(strs))
}

#[test]
fn test_match_2_one_item() {
	let strs = vec!["PBA".to_string()];
	assert!(!match_2(strs))
}


#[test]
fn test_match_2_multicase() {
	let strs = vec![
		"PbaHello".to_string(),
		"World".to_string(),
		"PBA".to_string(),
		"PbA-SDFGR".to_string(),
	];
	assert!(match_2(strs))
}

#[test]
fn test_match_3() {
	assert!(match_3((true, false, true)))
}

#[test]
fn test_match_3_false() {
	assert!(!match_3((false, false, true)));
}

#[test]
fn test_match_4_true() {
	assert!(match_4(Ok(6)))
}

#[test]
fn test_match_4_with_odd() {
	assert!(!match_4(Ok(3)))
}

#[test]
fn test_match_4_false() {
	assert!(!match_4(Err("bogus")))
}

#[test]
fn test_match_4_string_number() {
	assert!(match_4(Ok("6".parse().unwrap())))
}
