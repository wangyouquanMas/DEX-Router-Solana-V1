#[cfg(test)]
mod tests {
    use crate::instruction;
    use crate::instructions::create_pool;
    use crate::state::{self, AmmConfig, PoolState};
    use anchor_lang::prelude::*;
    use anchor_lang::{system_program, AnchorSerialize, Discriminator, InstructionData, ToAccountMetas};
    use solana_program_test::{processor, ProgramTest, ProgramTestContext};
    use solana_sdk::{
        account::Account,
        signature::Keypair,
        signer::Signer,
        transaction::Transaction,
    };
    use spl_token_2022::state::Mint as Token2022Mint;

    const DECIMALS: u8 = 6;
    const TICK_SPACING: u16 = 60;

    fn pack_anchor_account<T: AnchorSerialize + Discriminator>(account: &T) -> Vec<u8> {
        let mut data = Vec::new();
        data.extend_from_slice(&T::discriminator());
        data.extend_from_slice(&account.try_to_vec().unwrap());
        data
    }

    fn mock_mint() -> (Account, Pubkey) {
        let mut mint = Token2022Mint::default();
        mint.decimals = DECIMALS;
        mint.is_initialized = true;
        let mut data = vec![0u8; Token2022Mint::LEN];
        Token2022Mint::pack(mint, &mut data).unwrap();
        let account = Account {
            lamports: 1_000_000_000,
            data,
            owner: spl_token_2022::id(),
            executable: false,
            rent_epoch: 0,
        };
        (account, Pubkey::new_unique())
    }

    fn mock_amm_config(authority: Pubkey) -> (Account, Pubkey) {
        let amm = AmmConfig {
            bump: 255,
            authority,
            trade_fee_rate: 0,
            protocol_fee_rate: 0,
            tick_spacing: TICK_SPACING,
            fund_fee_rate: 0,
            padding: [0; 4],
        };
        let account = Account {
            lamports: 1_000_000_000,
            data: pack_anchor_account(&amm),
            owner: crate::id(),
            executable: false,
            rent_epoch: 0,
        };
        (account, Pubkey::new_unique())
    }

    fn setup_program(authority: Pubkey) -> (ProgramTest, Pubkey, Pubkey, Pubkey) {
        let mut program_test = ProgramTest::new("fun_uniswap_v3", crate::id(), processor!(crate::entry));
        program_test.add_program(
            "spl_token_2022",
            spl_token_2022::id(),
            processor!(spl_token_2022::processor::Processor::process),
        );
        let (amm_account, amm_key) = mock_amm_config(authority);
        program_test.add_account(amm_key, amm_account);
        let (mint0_account, mint0_key) = mock_mint();
        let (mint1_account, mint1_key) = mock_mint();
        program_test.add_account(mint0_key, mint0_account);
        program_test.add_account(mint1_key, mint1_account);
        (program_test, amm_key, mint0_key, mint1_key)
    }

    async fn create_pool_transaction(
        context: &mut ProgramTestContext,
        amm_config: Pubkey,
        authority: &Keypair,
        mint0: Pubkey,
        mint1: Pubkey,
        sqrt_price_x64: u128,
        tick_hint: i32,
    ) -> Transaction {
        let payer = &context.payer;
        let (pool_state, _) = Pubkey::find_program_address(
            &[
                state::POOL_STATE_SEED.as_bytes(),
                amm_config.as_ref(),
                mint0.as_ref(),
                mint1.as_ref(),
            ],
            &crate::id(),
        );
        let (vault0, _) = Pubkey::find_program_address(
            &[
                state::POOL_VAULT_SEED.as_bytes(),
                pool_state.as_ref(),
                mint0.as_ref(),
            ],
            &crate::id(),
        );
        let (vault1, _) = Pubkey::find_program_address(
            &[
                state::POOL_VAULT_SEED.as_bytes(),
                pool_state.as_ref(),
                mint1.as_ref(),
            ],
            &crate::id(),
        );
        let (observation, _) = Pubkey::find_program_address(
            &[
                state::POOL_OBSERVATION_SEED.as_bytes(),
                pool_state.as_ref(),
            ],
            &crate::id(),
        );
        let (bitmap, _) = Pubkey::find_program_address(
            &[
                state::POOL_TICK_ARRAY_BITMAP_SEED.as_bytes(),
                pool_state.as_ref(),
            ],
            &crate::id(),
        );
        let (bitmap_ext, _) = Pubkey::find_program_address(
            &[
                state::TICK_ARRAY_BITMAP_EXTENSION_SEED.as_bytes(),
                pool_state.as_ref(),
            ],
            &crate::id(),
        );

        let accounts = create_pool::CreatePool {
            payer: payer.pubkey(),
            authority: authority.pubkey(),
            amm_config,
            pool_state,
            token_mint_0: mint0,
            token_mint_1: mint1,
            token_vault_0: vault0,
            token_vault_1: vault1,
            observation_state: observation,
            tick_array_bitmap: bitmap,
            tick_array_bitmap_extension: bitmap_ext,
            token_program_0: spl_token_2022::id(),
            token_program_1: spl_token_2022::id(),
            system_program: system_program::id(),
            rent: solana_program::sysvar::rent::id(),
        };

        let ix = Instruction {
            program_id: crate::id(),
            accounts: accounts.to_account_metas(None),
            data: instruction::CreatePool {
                sqrt_price_x64,
                tick_current: tick_hint,
            }
            .data(),
        };

        let blockhash = context.banks_client.get_latest_blockhash().await.unwrap();
        Transaction::new_signed_with_payer(
            &[ix],
            Some(&payer.pubkey()),
            &[payer, authority],
            blockhash,
        )
    }

    fn decode_pool_state(data: &[u8]) -> PoolState {
        let mut slice: &[u8] = data;
        PoolState::try_deserialize(&mut slice).unwrap()
    }

    #[tokio::test]
    async fn create_pool_smoke_test() {
        let authority = Keypair::new();
        let (program_test, amm_config, mut mint_a, mut mint_b) = setup_program(authority.pubkey());
        if mint_a > mint_b {
            std::mem::swap(&mut mint_a, &mut mint_b);
        }
        let mut context = program_test.start_with_context().await;

        let tx = create_pool_transaction(
            &mut context,
            amm_config,
            &authority,
            mint_a,
            mint_b,
            1u128 << 64,
            0,
        )
        .await;
        context.banks_client.process_transaction(tx).await.unwrap();

        let (pool_state, _) = Pubkey::find_program_address(
            &[
                state::POOL_STATE_SEED.as_bytes(),
                amm_config.as_ref(),
                mint_a.as_ref(),
                mint_b.as_ref(),
            ],
            &crate::id(),
        );
        let account = context
            .banks_client
            .get_account(pool_state)
            .await
            .unwrap()
            .unwrap();
        let pool = decode_pool_state(&account.data);
        assert_eq!(pool.token_mint_0, mint_a);
        assert_eq!(pool.token_mint_1, mint_b);
        assert_eq!(pool.sqrt_price_x64, 1u128 << 64);
    }
}

