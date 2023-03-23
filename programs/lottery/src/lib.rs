use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod lottery {
    use super::*;

    pub fn initialize_lottery(ctx: Context<InitializeLottery>) -> Result<()> {

        let initializer_id = Pubkey::new_from_array([
            80, 97, 223, 1, 83, 109, 8, 147, 151, 40, 159, 3, 204, 231, 107, 20, 85, 34, 21, 236,
            209, 141, 60, 7, 160, 185, 70, 160, 206, 226, 231, 158,
        ]);

        //check to see if initializer id is the same as the signer
        


        Ok(())
    }

    pub fn stake_tokens(ctx: Context<StakeTokens>) -> Result<()> {
        Ok(())
    }

    pub fn store_winning_numbers(ctx: Context<StoreWinningNumbers>) -> Result<()> {
        Ok(())
    }

    pub fn reward_winners(ctx: Context<RewardWinners>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitializeLottery<'info> {
    #[account(init, payer = initializer_id, space = 8 + 32)]
    pub lottery: Account<'info, Lottery>,
    #[account(init, seeds = [b"global_stake"], bump, payer = initializer_id, space = 8 + 1+32+8)]
    pub global_stake: Account<'info, Lottery>,
    #[account(mut)]
    pub initializer_id: Signer<'info>,
    pub system_program: AccountInfo<'info>
}



//stake tokens to sol end
#[derive(Accounts)]
pub struct StakeTokens<'info> {
    #[account(mut)]
    pub lottery: Account<'info, Lottery>,
    //global stake account
    #[account(mut)]
    pub global_stake: AccountInfo<'info>,
    #[account(init, seeds = [b"user_stake", payer.key().as_ref()], bump, payer = payer, space = 8 + 1+32+8)]
    //user stake account
    pub user_stake: AccountInfo<'info>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: AccountInfo<'info>
}

#[derive(Accounts)]
pub struct StoreWinningNumbers {

}

#[derive(Accounts)]
pub struct RewardWinners {

}

#[account]
pub struct Lottery {
    pub initializer_id: Pubkey,
}

#[account]
pub struct GlobalStake {
    pub stake_mint: Option<Pubkey>,
    pub stake_amount:u64
}

#[account]
pub struct UserStake {
    pub stake_mint: Option<Pubkey>,
    pub stake_amount:u64
}