use anchor_lang::prelude::*;

use crate::{Administrators, AuthorProfile, PaperIDCounter};
use crate::states::errors::*;


// ADMINISTRATORS CONTEXT
#[derive(Accounts)]
pub struct AdministratorsInfo<'info> {

    #[account(mut)]
    pub deployer: Signer<'info>,

    #[account(
        init,
        payer = deployer,
        space = 8 + Administrators::INIT_SPACE,
        seeds = [b"administrators".as_ref(), b"openQuanta".as_ref()],
        bump,
    )]
    pub admins: Account<'info, Administrators>,

    pub system_program: Program<'info, System>,
}


// GLOBAL PAPER ID COUNTER INITIALIZATION CONTEXT
#[derive(Accounts)]
pub struct PaperIDCounterInfo<'info> {

    #[account(
        mut,
        constraint = admins.admins_pubkey.contains(&admin.key()) @OpenQuantaErrors::OnlyAdmin
    )]
    pub admin: Signer<'info>,

    #[account(
        seeds = [b"administrators".as_ref(), b"OpenQuanta".as_ref()],
        bump = admins.admins_bump
    )]
    pub admins: Account<'info, Administrators>,

    #[account(
        init,
        payer = admin,
        space = 8 + PaperIDCounter::INIT_SPACE,
        seeds = [b"paper_id_counter".as_ref()],
        bump,
    )]
    pub paper_id_assigner: Account<'info, PaperIDCounter>,

    pub system_program: Program<'info, System>,
}


// AUTHOR PROFILE CONTEXT
#[derive(Accounts)]
pub struct AuthorProfileInfo<'info> {

    #[account(mut)]
    pub author: Signer<'info>,

    #[account(
        init,
        payer = author,
        space = 8 + AuthorProfile::INIT_SPACE,
        seeds = [b"author_profile", author.key().as_ref()],
        bump,
    )]
    pub author_profile: Account<'info, AuthorProfile>,

    pub system_program: Program<'info, System>,
}