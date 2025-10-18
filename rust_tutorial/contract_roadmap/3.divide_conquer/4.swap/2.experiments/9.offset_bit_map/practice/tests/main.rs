use uint::construct_uint;

construct_uint! {
    pub struct U1024(16);
}

fn u1024_from_le_limbs(limbs: [u64; 16]) -> U1024 {
	let mut bytes = [0u8; 128];
	for (i, limb) in limbs.iter().enumerate() {
		bytes[i * 8..(i + 1) * 8].copy_from_slice(&limb.to_le_bytes());
	}
	U1024::from_little_endian(&bytes)
}

fn u1024_to_le_limbs(value: &U1024) -> [u64; 16] {
	let mut bytes = [0u8; 128];
	value.to_little_endian(&mut bytes);
	let mut limbs = [0u64; 16];
	for i in 0..16 {
		let mut arr = [0u8; 8];
		arr.copy_from_slice(&bytes[i * 8..(i + 1) * 8]);
		limbs[i] = u64::from_le_bytes(arr);
	}
	limbs
}

#[test]
fn test_get_offset_bit_map() {
	let limbs: [u64; 16] = [
		0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 9225641428854505472u64,
		17u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64,
	];
	let bit_map: U1024 = u1024_from_le_limbs(limbs);
	let bit_pos: usize = 498;
	let shift: usize = 1024usize - bit_pos - 1; //525
	// let shift: usize = 1;
	let offset_bit_map: U1024 = bit_map << shift;	
	let limbs_out = u1024_to_le_limbs(&offset_bit_map);
	println!("offset_bit_map limbs [u64;16]: {:?}", limbs_out);
}


