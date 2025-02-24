use std::usize;

use anchor_lang::{
    prelude::{
        borsh::{BorshDeserialize, BorshSerialize},
        *,
    },
    solana_program::hash::Hash,
};
declare_id!("6z68wfurCMYkZG51s1Et9BJEd9nJGUusjHXNt4dGbNNF");

#[program]
pub mod basic {
    use super::*;

    pub fn upload_post(ctx: Context<AddPost>, post_id: u64) -> Result<()> {
        Ok(())
    }
}

pub const DISCRIMINATOR: usize = 8;

#[derive(Accounts)]
pub struct CreateRate<'info> {
    pub rater: Signer<'info>,
}

#[derive(Accounts)]
// #[instruction(post_id: u64)]
pub struct AddPost<'info> {
    #[account(mut)]
    pub poster: Signer<'info>,
    #[account(
        init,
        payer = poster,
        space = DISCRIMINATOR + Post::INIT_SPACE,
        seeds = [poster.key().as_ref()],
        bump
    )]
    pub post: Account<'info, Post>,
    pub system_program: Program<'info, System>,
}

#[account]
#[derive(InitSpace)]
pub struct Post {
    pub post_id: u64,
    #[max_len(100)]
    pub post_time_created: String,
    #[max_len(100)]
    pub post_author: String,
    #[max_len(20)]
    pub platform_name: String,
    pub post_total_rating: u64,
    pub post_postive_rate: u64,
    pub post_negative_rate: u64,
    #[max_len(1000)]
    pub post_detail: String,
    pub post_hash: [u8; 32],
}

#[account]
#[derive(InitSpace)]
pub struct User {
    pub user_id: Pubkey,
    pub user_status: UserStatus,
    pub reputation_score: u64,
    pub balance: u64,
    pub total_rate: u64,
    pub staking_amount: u64,
    #[max_len(64)]
    pub last_active_time: String,
    #[max_len(1000)]
    pub user_detail: String,
    pub user_hash: [u8; 32],
}

#[account]
#[derive(InitSpace)]
pub struct RateData {}

#[derive(Clone, InitSpace, BorshDeserialize, BorshSerialize)]
pub enum UserStatus {
    Level1,
    Level2,
    Level3,
    Level4,
    Level5,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Error {}

impl Post {
    fn get_info(&self, post_id: u64) {
        msg!("INFO: post_id: {}", post_id);
    }
}
