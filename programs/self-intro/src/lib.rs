use std::usize;

use anchor_lang::prelude::*;

declare_id!("7aKYn6hz2W43aGx4kXXwEihLdYbnwM5qzvDPft5iHAfk");

#[program]
pub mod self_intro {
    use super::*;

    pub fn create_intro(
        ctx: Context<CreateIntro>,
        name: String,
        status: String,
        twitter: String,
        superpower: String,
    ) -> Result<()> {
        let user_account_data = &mut ctx.accounts.user_account;
        user_account_data.bump = *ctx.bumps.get("user_account").unwrap();

        user_account_data.authority = *ctx.accounts.signer.key;
        user_account_data.name = name.to_owned();
        user_account_data.status = status.to_owned();
        user_account_data.twitter = twitter.to_owned();
        user_account_data.superpower = superpower.to_owned();

        msg!(
            "Created an account with the following details:
            Name: {0}
            Status: {1}
            Twitter: {2}
            Superpower: {3}
            ",
            name,
            status,
            twitter,
            superpower
        );
        Ok(())
    }

    pub fn update_status(ctx: Context<UpdateStatus>, new_status: String) -> Result<()> {
        msg!(
            "Updating status from {0} to {1}",
            &ctx.accounts.user_account.status,
            &new_status
        );
        ctx.accounts.user_account.status = new_status;
        Ok(())
    }
}

pub fn delete_account(_ctx: Context<DeleteAccount>) -> Result<()> {
    msg!("Account deleted successfully");
    Ok(())
}

#[derive(Accounts)]
pub struct DeleteAccount<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(
        mut,
        seeds = ["solana-intro".as_bytes(), signer.key().as_ref()],
        bump = user_account.bump,
        close = signer,
    )]
    pub user_account: Account<'info, UserAccount>,
}

#[derive(Accounts)]
pub struct UpdateStatus<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(mut, seeds = ["solana-intro".as_bytes(), signer.key().as_ref()], bump = user_account.bump)]
    pub user_account: Account<'info, UserAccount>,
}

#[derive(Accounts)]
pub struct CreateIntro<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(init, payer = signer, space = UserAccount::LEN, seeds = ["solana-intro".as_bytes(), signer.key().as_ref()], bump)]
    pub user_account: Account<'info, UserAccount>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct UserAccount {
    pub authority: Pubkey,
    pub bump: u8,
    pub name: String,
    pub status: String,
    pub twitter: String,
    pub superpower: String,
}

impl UserAccount {
    const LEN: usize = 8 + 32 + 1 + (4 + 20) + (4 + 120) + (4 + 20);
}
