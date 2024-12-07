use anchor_lang::prelude::*;
use anchor_spl::token;
use anchor_spl::token::{Token, TokenAccount, Mint, MintTo, Transfer};

declare_id!("Bm38NWQxWSRixVH7n6TtgZhuZo8NAji5MrUQZzviBd1f");


#[program]
pub mod nft_program {
    use super::*;

    pub fn setup_mint(ctx: Context<SetupMint>, token_decimals: u8) -> Result<()> {
        msg!("Mint initialized successfully: {}", ctx.accounts.token_mint.key());
        msg!("Decimals configured: {}", token_decimals);
        Ok(())
    }

    pub fn setup_pool(ctx: Context<SetupPool>) -> Result<()> {
        msg!("Staking pool setup complete.");
        msg!("Pool Token Account: {}", ctx.accounts.pool_token_account.key());
        Ok(())
    }

    pub fn distribute_airdrop(ctx: Context<DistributeAirdrop>, token_amount: u64) -> Result<()> {
        let mint_auth_bump = ctx.bumps.mint_authority;
        let mint_auth_seeds = &["mint-authority".as_bytes(), &[mint_auth_bump]];
        let signer = &[&mint_auth_seeds[..]];

        msg!("Distributing {} tokens as airdrop...", token_amount);
        let mint_ctx = ctx.accounts.mint_context().with_signer(signer);
        let _ = token::mint_to(mint_ctx, token_amount);

        msg!("Airdrop successfully completed!");
        Ok(())
    }

    pub fn perform_stake(ctx: Context<PerformStake>, stake_amount: u64) -> Result<()> {
        let transfer_instruction = Transfer {
            from: ctx.accounts.user_wallet_account.to_account_info(),
            to: ctx.accounts.pool_wallet_account.to_account_info(),
            authority: ctx.accounts.user_authority.to_account_info(),
        };

        let transfer_ctx = CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            transfer_instruction,
        );

        msg!("Staking {} tokens...", stake_amount);
        ctx.accounts.user_data_account.staked_tokens += stake_amount;
        token::transfer(transfer_ctx, stake_amount)?;

        msg!("Tokens successfully staked!");
        Ok(())
    }

    pub fn perform_unstake(ctx: Context<PerformUnstake>, unstake_amount: u64) -> Result<()> {
        let current_stake = ctx.accounts.user_data_account.staked_tokens;
        if unstake_amount > current_stake {
            msg!("Error: Unstaking amount exceeds staked tokens.");
            Ok(())
        } else {
            let pool_auth_bump = ctx.bumps.pool_authority;
            let pool_auth_seeds = &["pool-authority".as_bytes(), &[pool_auth_bump]];
            let signer = &[&pool_auth_seeds[..]];

            msg!("Unstaking {} tokens...", unstake_amount);
            ctx.accounts.user_data_account.staked_tokens -= unstake_amount;

            let unstake_ctx = ctx.accounts.unstake_context().with_signer(signer);
            let _ = token::transfer(unstake_ctx, unstake_amount);

            msg!("Tokens successfully unstaked!");
            Ok(())
        }
    }
}

#[derive(Accounts)]
#[instruction(token_decimals: u8)]
pub struct SetupMint<'info> {
    #[account(
        init, 
        mint::authority = mint_authority,
        mint::decimals = token_decimals,
        seeds = ["mint-token".as_bytes()],
        bump,
        payer = payer)]
    pub token_mint: Account<'info, Mint>,

    #[account(seeds = ["mint-authority".as_bytes()], bump)]
    /// CHECK: Mint authority
    pub mint_authority: AccountInfo<'info>,

    #[account(mut)]
    pub payer: Signer<'info>,
    pub rent: Sysvar<'info, Rent>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>
}

#[derive(Accounts)]
pub struct SetupPool<'info> {
    #[account(mut, seeds = ["mint-token".as_bytes()], bump)]
    pub token_mint: Account<'info, Mint>,

    #[account(seeds = ["pool-authority".as_bytes()], bump)]
    /// CHECK: This account represents the pool authority, validated by seeds and bump
    pub pool_authority: AccountInfo<'info>,

    #[account(
        init,
        token::mint = token_mint,
        token::authority = pool_authority,
        payer = payer
    )]    
    pub pool_token_account: Account<'info, TokenAccount>,

    #[account(mut)]
    pub payer: Signer<'info>,
    pub rent: Sysvar<'info, Rent>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>
}

#[derive(Accounts)]
pub struct DistributeAirdrop<'info> {
    #[account(mut, seeds = ["mint-token".as_bytes()], bump)]
    pub token_mint: Account<'info, Mint>,

    #[account(mut, seeds = ["mint-authority".as_bytes()], bump)]
    /// CHECK: Mint authority
    pub mint_authority: AccountInfo<'info>,

    #[account(mut)]
    pub user_authority: Signer<'info>,

    #[account(
        init,
        token::mint = token_mint,
        token::authority = user_authority,
        payer = user_authority
    )]    
    pub user_wallet_account: Account<'info, TokenAccount>,

    pub rent: Sysvar<'info, Rent>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>
}

impl<'info> DistributeAirdrop<'info> {
    pub fn mint_context(&self) -> CpiContext<'_, '_, '_, 'info, MintTo<'info>> {
        let cpi_program = self.token_program.to_account_info();
        let cpi_accounts = MintTo {
            mint: self.token_mint.to_account_info(),
            to: self.user_wallet_account.to_account_info(),
            authority: self.mint_authority.to_account_info(),
        };

        CpiContext::new(cpi_program, cpi_accounts)
    }
}

#[derive(Accounts)]
pub struct PerformStake<'info> {
    #[account(mut, seeds = ["mint-token".as_bytes()], bump)]
    /// CHECK: mint account
    pub token_mint: Account<'info, Mint>,

    #[account(mut, seeds = ["pool-authority".as_bytes()], bump)]
    /// CHECK: Pool authority
    pub pool_authority: AccountInfo<'info>,

    #[account(mut)]
    pub user_authority: Signer<'info>,

    #[account(
        mut,
        token::mint = token_mint,
        token::authority = user_authority,
    )]    
    pub user_wallet_account: Account<'info, TokenAccount>,

    #[account(
        init, 
        seeds = ["user-data".as_bytes()], 
        bump, 
        payer = user_authority,
        space = 8 + 8)]
    pub user_data_account: Account<'info, UserData>,

    #[account(
        mut,
        token::mint = token_mint,
        token::authority = pool_authority
    )]    
    pub pool_wallet_account: Account<'info, TokenAccount>,

    pub rent: Sysvar<'info, Rent>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>
}

#[derive(Accounts)]
pub struct PerformUnstake<'info> {
    #[account(mut, seeds = ["mint-token".as_bytes()], bump)]
    /// CHECK: mint account
    pub token_mint: Account<'info, Mint>,

    #[account(mut, seeds = ["pool-authority".as_bytes()], bump)]
    /// CHECK: Pool authority
    pub pool_authority: AccountInfo<'info>,

    #[account(mut)]
    pub user_authority: Signer<'info>,

    #[account(
        mut,
        token::mint = token_mint,
        token::authority = user_authority,
    )]    
    pub user_wallet_account: Account<'info, TokenAccount>,

    #[account(mut, seeds = ["user-data".as_bytes()], bump)]
    pub user_data_account: Account<'info, UserData>,

    #[account(
        mut,
        token::mint = token_mint,
        token::authority = pool_authority
    )]    
    pub pool_wallet_account: Account<'info, TokenAccount>,

    pub rent: Sysvar<'info, Rent>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>
}

impl<'info> PerformUnstake<'info> {
    pub fn unstake_context(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        let cpi_program = self.token_program.to_account_info();
        let cpi_accounts = Transfer {
            from: self.pool_wallet_account.to_account_info(),
            to: self.user_wallet_account.to_account_info(),
            authority: self.pool_authority.to_account_info(),
        };

        CpiContext::new(cpi_program, cpi_accounts)
    }
}

#[account]
pub struct UserData {
    pub staked_tokens: u64,
}
