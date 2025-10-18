use uint::construct_uint;

construct_uint! {
    pub struct U1024(16);
}
// Meaning of construct_uint! { pub struct U1024(16); }
// It defines a big-unsigned-integer type U1024 with 1024 bits.
// The (16) means it uses 16 limbs of 64 bits each (16 × 64 = 1024).
// It’s implemented as a tuple struct with a single private field holding [u64; 16] (you don’t access it directly)

fn u1024_to_u64x16_le(x: U1024) -> [u64; 16] {
    let mut le_bytes = [0u8; 128];
    x.to_little_endian(&mut le_bytes);
    let mut limbs = [0u64; 16];
    for i in 0..16 {
        limbs[i] = u64::from_le_bytes(le_bytes[i * 8..i * 8 + 8].try_into().unwrap());
    }
    limbs
}

#[test]
fn test_bit_mask_and_log() {
    // minimal reproduction of the mask-and test
    let bit_pos: usize = 511;
    //U1024::one() returns the 1024-bit unsigned integer value equal to 1.
    //
    let one = U1024::one();
    println!("one:{:?}",one);
    println!("one (decimal) = {}", one);     

    let mask = U1024::one() << bit_pos;
    println!("mask:{:?}",mask);

    // Inspect bytes (little-endian): 128 bytes for 1024 bits
    let mut le_bytes = [0u8; 128];
    mask.to_little_endian(&mut le_bytes);
    println!("mask le bytes (first 32): {:?}", &le_bytes[..32]);

    // Inspect the limb that contains bit_pos (each limb is 64 bits)
    let limb_index = bit_pos / 64; // 0..15
    // Extract that limb from the little-endian bytes
    let start = limb_index * 8;
    let limb_bytes = &le_bytes[start..start+8];
    let limb_value = u64::from_le_bytes(limb_bytes.try_into().unwrap());
    println!("limb_index: {} limb_value: {:#018x}", limb_index, limb_value);

    // Sanity: the selected bit should be set
    let bit_in_limb = bit_pos % 64;
    assert_eq!((limb_value >> bit_in_limb) & 1, 1);
}

#[test]
fn test_mask_u1024() {
    // Provided decimal value
    // let s = "6703903964971298549787012499102923063739682910296196688861780721860882015036773488400937149083451713845015929093243025426876941405973284973216824503042048";
    let value = U1024::one() << 511;
    // let value: U1024 = s.parse().expect("parse U1024 from decimal");

    let limbs_le = u1024_to_u64x16_le(value);
    println!("limbs_le: {:?}", limbs_le);

    // Let's analyze what this means
    println!("Decimal value: {}", value);
    println!("Hex value: {:#x}", value);
    
    // Check which bit is set (9223372036854775808 = 2^63)
    let expected_bit = 7 * 64 + 63; // limb 7, bit 63
    println!("Expected bit position: {}", expected_bit);
    
    // Verify this is 2^511 (since 7*64 + 63 = 511)
    let expected_value = U1024::one() << 511;
    println!("2^511: {}", expected_value);
    println!("2^511 hex: {:#x}", expected_value);
    
    // Check if they match
    println!("Values match: {}", value == expected_value);

    // Example: reconstruct from limbs to verify roundtrip
    let mut bytes = [0u8; 128];
    for i in 0..16 {
        bytes[i*8..i*8+8].copy_from_slice(&limbs_le[i].to_le_bytes());
    }
    let roundtrip = U1024::from_little_endian(&bytes);
    assert_eq!(roundtrip, value);
}