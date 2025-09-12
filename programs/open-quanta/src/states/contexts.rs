use anchor_lang::prelude::*;

use mpl_core::ID as MPL_CORE_PROGRAM_ID;

use crate::{Administrators, AuthorProfile, PaperIDCounter, ReviewerProfile, Paper, CollectionRegistry};
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

// COLLECTION REGISTRY INITIALIZATION CONTEXT
#[derive(Accounts)]
pub struct CollectionRegistryInfo<'info> {
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
        space = 8 + CollectionRegistry::INIT_SPACE,
        seeds = [b"collection_registry".as_ref(), b"OpenQuanta".as_ref()],
        bump,
    )]
    pub collection_registry: Account<'info, CollectionRegistry>,

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


// REVIEWER PROFILE CONTEXT
#[derive(Accounts)]
pub struct ReviewerProfileInfo<'info> {

    #[account(mut)]
    pub reviewer: Signer<'info>,

    #[account(
        init,
        payer = reviewer,
        space = 8 + ReviewerProfile::INIT_SPACE,
        seeds = [b"reviewer_profile", reviewer.key().as_ref()],
        bump,
    )]
    pub reviewer_profile: Account<'info, ReviewerProfile>,

    pub system_program: Program<'info, System>,
}

// PAPER CONTEXT
#[derive(Accounts)]
pub struct PaperInfo<'info> {

    #[account(mut)]
    pub paper_submitter: Signer<'info>,

    #[account(
        mut,
        seeds = [b"paper_id_counter".as_ref()],
        bump = paper_id_assigner.counter_bump
    )]
    pub paper_id_assigner: Account<'info, PaperIDCounter>,

    #[account(
        mut,
        seeds = [b"author_profile", paper_submitter.key().as_ref()],
        bump = author_profile.author_bump,
    )]
    pub author_profile: Account<'info, AuthorProfile>,

    #[account(
        init,
        payer = paper_submitter,
        space = 8 + Paper::INIT_SPACE,
        seeds = [b"paper", paper_submitter.key().as_ref(), &format!("OQ-{:07}", paper_id_assigner.current_id + 1).as_bytes()],
        bump
    )]
    pub research_paper: Account<'info, Paper>,

    pub system_program: Program<'info, System>,
}


// // Open Quanta Master Authorship NFT Collection
// #[derive(Accounts)]
// pub struct MasterAuthorshipNFTCollection<'info> {
//     #[account(
//         mut,
//         constraint = admins.admins_pubkey.contains(&admin.key()) @OpenQuantaErrors::OnlyAdmin
//     )]
//     pub admin: Signer<'info>,

//     #[account(
//         seeds = [b"administrators".as_ref(), b"OpenQuanta".as_ref()],
//         bump = admins.admins_bump
//     )]
//     pub admins: Account<'info, Administrators>,

//     #[account(mut)]
//     pub oq_parent_collection_mint: InterfaceAccount<'info, Mint>,

//     #[account(
//         mut,
//         seeds = [
//             b"metadata",
//             metadata_program.key().as_ref(),
//             oq_parent_collection_mint.key().as_ref()
//         ],
//         seeds::program = metadata_program.key(),
//         bump
//     )]
//     /// CHECK: Initialized Via Metaplex CPI
//     pub oq_parent_authorship_nft_metadata: UncheckedAccount<'info>,

//     #[account(
//         mut,
//         seeds = [
//             b"metadata",
//             metadata_program.key().as_ref(),
//             oq_parent_collection_mint.key().as_ref(),
//             b"edition"
//         ],
//         seeds::program = metadata_program.key(),
//         bump
//     )]
//     /// CHECK: Initialized Via Metaplex CPI
//     pub oq_parent_authorship_nft_master_edition: UncheckedAccount<'info>,

//     pub metadata_program: Program<'info, Metadata>,

//     pub associated_token_program: Program<'info, AssociatedToken>,

//     pub token_program: Interface<'info, TokenInterface>,

//     pub system_program: Program<'info, System>,

//     pub rent: Sysvar<'info, Rent>,
// }

// OPENQUANTA COLLECTION CREATION CONTEXT
#[derive(Accounts)]
pub struct CreateCollection<'info> {
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
        mut,
        seeds = [b"collection_registry".as_ref(), b"OpenQuanta".as_ref()],
        bump
    )]
    pub collection_registry: Account<'info, CollectionRegistry>,

    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(mut)]
    pub collection: Signer<'info>,

    #[account(address = MPL_CORE_PROGRAM_ID)]
    /// CHECK: This doesn't need to be checked, because there is the address constraint
    pub mpl_core_program: UncheckedAccount<'info>,

    pub system_program: Program<'info, System>,
}