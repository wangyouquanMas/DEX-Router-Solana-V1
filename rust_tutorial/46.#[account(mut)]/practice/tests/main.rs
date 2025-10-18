use anchor_lang::prelude::*;

#[cfg(test)]
mod tests {
    use super::*;

    /// A minimal Anchor-like example demonstrating how `#[instruction(index: u16)]`
    /// lets us use the instruction argument inside account validation logic. We
    /// model the usual Anchor macros with plain Rust so the concept can be tested
    /// without a blockchain runtime.
    #[derive(Accounts, Debug, PartialEq)]
    #[instruction(index: u16)]
    pub struct Example<'info> {
        /// Public key of the user creating the PDA-like account.
        pub user: Pubkey,
        /// Address derived from `user` and the instruction-supplied `index`.
        pub derived_address: Pubkey,
    }

    impl<'info> Example<'info> {
        /// Verifies that the `derived_address` matches the PDA that would have
        /// been computed using the `user` pubkey and the provided `index`.
        fn validate(&self, index: u16) -> Result<()> {
            let expected = derive_example_address(self.user, index);
            require_keys_eq!(expected, self.derived_address, ExampleError::AddressMismatch);
            Ok(())
        }
    }

    /// Derives a deterministic address by hashing the seeds of the PDA. We mimic
    /// Anchor's PDA derivation (`seeds = [b"example", user.key().as_ref(), &index]`).
    fn derive_example_address(user: Pubkey, index: u16) -> Pubkey {
        let seeds = [b"example".as_ref(), user.as_ref(), &index.to_le_bytes()];
        Pubkey::find_program_address(&seeds, &Pubkey::new_unique()).0
    }

    #[error_code]
    pub enum ExampleError {
        #[msg("Derived address does not match expected PDA seed")]
        AddressMismatch,
    }

    #[test]
    fn validates_correct_address() {
        let user = Pubkey::new_unique();
        let index = 42;
        let derived = derive_example_address(user, index);

        let accounts = Example {
            user,
            derived_address: derived,
        };

        accounts.validate(index).expect("validation should succeed");
    }

    #[test]
    fn rejects_incorrect_address() {
        let user = Pubkey::new_unique();
        let index = 7;
        let derived = derive_example_address(user, index);

        let mismatched_accounts = Example {
            user,
            derived_address: Pubkey::new_unique(),
        };

        let err = mismatched_accounts.validate(index).unwrap_err();
        match err {
            anchor_lang::error::Error::AnchorError(anchor_err)
                if anchor_err.error_code_number == ExampleError::AddressMismatch.into() => {}
            other => panic!("unexpected error: {other:?}"),
        }

        let altered_index_err = Example {
            user,
            derived_address: derived,
        }
        .validate(index + 1)
        .unwrap_err();

        match altered_index_err {
            anchor_lang::error::Error::AnchorError(anchor_err)
                if anchor_err.error_code_number == ExampleError::AddressMismatch.into() => {}
            other => panic!("unexpected error: {other:?}"),
        }
    }
}

