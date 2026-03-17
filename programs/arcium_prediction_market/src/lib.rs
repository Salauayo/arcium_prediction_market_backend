use anchor_lang::prelude::*;

declare_id!("A3AwUM2QyAuSo76RJCtSp8PBxbzSs2Xvaqd8j8w8PXPp");

#[program]
pub mod arcium_prediction_market {
    use super::*;

    pub fn create_market(ctx: Context<CreateMarket>, title: String, options: Vec<String>) -> Result<()> {
        let market = &mut ctx.accounts.market;
        market.title = title;
        market.options = options;
        market.creator = *ctx.accounts.user.key;
        Ok(())
    }

    pub fn cast_vote(ctx: Context<CastVote>, vote: String) -> Result<()> {
        let vote_account = &mut ctx.accounts.vote;
        vote_account.market = *ctx.accounts.market.key;
        vote_account.voter = *ctx.accounts.user.key;
        vote_account.vote = vote;
        Ok(())
    }
}

#[account]
pub struct Market {
    pub title: String,
    pub options: Vec<String>,
    pub creator: Pubkey,
}

#[account]
pub struct Vote {
    pub market: Pubkey,
    pub voter: Pubkey,
    pub vote: String,
}

#[derive(Accounts)]
pub struct CreateMarket<'info> {
    #[account(init, payer = user, space = 9000)]
    pub market: Account<'info, Market>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct CastVote<'info> {
    #[account(init, payer = user, space = 9000)]
    pub vote: Account<'info, Vote>,
    pub market: Account<'info, Market>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}
