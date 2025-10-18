use solana_program::pubkey::Pubkey;

#[test]
fn pda_derivation_compiles() {
    let seed = b"example";
    let base = Pubkey::new_unique();
    let (pda, _bump) = Pubkey::find_program_address(&[seed, base.as_ref()], &practice::id());
    assert_ne!(pda, Pubkey::default());
} 