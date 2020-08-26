use crate::char::*;

#[test]
fn test_char() {
	let x: char = 'x';
	assert_eq!(x.as_char(), 'x');

	let x: char = 'x';
	assert_eq!(x.is_alpha(), true);
	
	let x: char = '1';
	assert_eq!(x.is_alpha(), false);

	let x: char = 'x';
	assert_eq!(x.is_alphanum(), true);

	let x: char = '1';
	assert_eq!(x.is_alphanum(), true);

	let x: char = ')';
	assert_eq!(x.is_alphanum(), false);

	let x: char = '1';
	assert!(x.is_dec_digit());

	let x: char = 'a';
	assert!(!x.is_dec_digit());

	let x: char = 'A';
	assert!(!x.is_dec_digit());
	
	let x: char = 'x';
	assert!(!x.is_dec_digit());
	
	let x: char = ')';
	assert!(!x.is_dec_digit());

	let x: char = ')';
	assert!(!x.is_hex_digit());

	let x: char = '0';
	assert!(x.is_hex_digit());

	let x: char = '9';
	assert!(x.is_hex_digit());

	let x: char = 'a';
	assert!(x.is_hex_digit());

	let x: char = 'f';
	assert!(x.is_hex_digit());

	let x: char = 'A';
	assert!(x.is_hex_digit());

	let x: char = 'F';
	assert!(x.is_hex_digit());

	let x: char = 'G';
	assert!(!x.is_hex_digit());
	
	let x: char = 'x';
	assert!(!x.is_hex_digit());

	let x: char = '0';
	assert!(x.is_oct_digit());

	let x: char = '7';
	assert!(x.is_oct_digit());

	let x: char = '8';
	assert!(!x.is_oct_digit());

	let x: char = 'a';
	assert!(!x.is_oct_digit());

	let x: char = ')';
	assert!(!x.is_oct_digit());
	
	let x: char = ')';
	assert_eq!(x.len(), 1);

	let x: char = 'x';
	assert_eq!(x.is_a(|v| v == &'x'), true);
	assert_eq!(x.is_a(|v| v == &'y'), false);
}
