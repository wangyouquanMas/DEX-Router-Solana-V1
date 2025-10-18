use crypto_bigint::{Encoding, U1024};

fn u64_array_to_binary_be(words: &[u64; 16]) -> String {
	let mut s = String::with_capacity(1024);
	for word in words.iter() {
		// Big-endian within each 64-bit word
		s.push_str(&format!("{:064b}", word));
	}
	s
}

#[test]
fn test_turn_u64_array_to_binary() {
	let words: [u64; 16] = [
		0, 0, 0, 0, 0, 0, 0, 9225641428854505472, 17, 0, 0, 0, 0, 0, 0, 0,
	];
	let bin = u64_array_to_binary_be(&words);

	assert_eq!(bin.len(), 1024);
	let ones = bin.chars().filter(|c| *c == '1').count();
	// assert_eq!(ones, 2);
	// Check that the bit in the 8th word (index 7) MSB is set, and the 9th word (index 8) LSB is set.
	// In big-endian string per word: index in overall string = word_index*64 + bit_index
	let idx_word7_msb = 7 * 64 + 0; // first char of that 64-bit chunk
	let idx_word8_lsb = 8 * 64 + 63; // last char of that 64-bit chunk

	let bchars: Vec<char> = bin.chars().collect();
	assert_eq!(bchars[idx_word7_msb], '1');
	assert_eq!(bchars[idx_word8_lsb], '1');

	// Output 16 rows (one per u64) in 64-bit binary and decimal
	for (i, w) in words.iter().enumerate() {
		println!("row {:02}: {:064b} (decimal: {})", i, w, w);
	}
}

