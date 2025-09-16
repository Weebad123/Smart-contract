use anchor_lang::prelude::*;

use mpl_core::{
    ID as MPL_CORE_PROGRAM_ID,
    accounts::BaseCollectionV1,
    instructions::{
        CreateV2CpiAccounts,
        CreateV2CpiBuilder
    }};

use crate::{Administrators, AuthorProfile, CollectionRegistry, Paper, PaperIDCounter, ReviewerProfile};
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
        seeds = [b"administrators".as_ref(), b"OpenQuanta".as_ref()],
        bump,
    )]
    pub admins: Account<'info, Administrators>,

    pub system_program: Program<'info, System>,
}

impl<'info> AdministratorsInfo<'info> {
    pub fn init(&mut self, bumps: AdministratorsInfoBumps) -> Result<()> {

        let mut administrators = self.admins.clone();
        administrators.admins_pubkey = Vec::new();
        administrators.admins_bump = bumps.admins;
        Ok(())
    }
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
        bump
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

    ///CHECK: SAFE TO IGNORE FOR NOW
     #[account(
        mut,
        seeds = [b"OpenQuanta_Nft_Mint_Authority".as_ref()],
        bump,
    )]
    pub oq_nft_mint_authority: AccountInfo<'info>,

    // Created off-chain, and stored here
    ///CHECK: SAFE TO USE
    #[account(mut)]
    pub collection_mint: AccountInfo<'info>,


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
        bump
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

    /// CHECK: SAFE TO USE
    #[account(
        init,
        payer = admin,
        space = 8,
        seeds = [b"OpenQuanta_Nft_Mint_Authority".as_ref()],
        bump,
    )]
    pub oq_nft_mint_authority: AccountInfo<'info>,

    pub system_program: Program<'info, System>,
}

impl<'info> PaperIDCounterInfo<'info> {
    pub fn init(&mut self, bumps: PaperIDCounterInfoBumps) -> Result<()> {

        // Initialize Paper ID Counter
        let mut paper_id_counter = self.paper_id_assigner.clone();
        paper_id_counter.current_id = 0;
        paper_id_counter.counter_bump = bumps.paper_id_assigner;

        Ok(())
    }
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

    #[account(
        mut,
        seeds = [b"collection_registry".as_ref(), b"OpenQuanta".as_ref()],
        bump
    )]
    pub collection_registry: Account<'info, CollectionRegistry>,

    /// CHECK: SAFE TO USE
    #[account(
        mut,
        constraint = collection.key() == collection_registry.collection_mint @OpenQuantaErrors::InvalidCollection
    )]
    pub collection: AccountInfo<'info>,

    /// CHECK: SAFE TO USE
    #[account(
        mut,
        seeds = [b"OpenQuanta_Nft_Mint_Authority".as_ref()],
        bump,
    )]
    pub oq_nft_mint_authority: AccountInfo<'info>,

    //
    #[account(mut)]
    pub nft_asset: Signer<'info>,

    #[account(address = MPL_CORE_PROGRAM_ID)]
    /// CHECK: This doesn't need to be checked, because there is the address constraint
    pub mpl_core_program: UncheckedAccount<'info>,

    pub system_program: Program<'info, System>,
}

// IMPLEMENT NFT MINTING LOGIC HERE
// Mint from the OQ Collections, but Change the Metadata Account
impl<'info> PaperInfo<'info> {
    pub fn mint_authorship_nft(&mut self) -> Result<()> {
        let mint_accounts = CreateV2CpiAccounts {
            asset: &self.nft_asset.to_account_info(),
            collection: Some(&self.collection.to_account_info()),
            authority: Some(&self.oq_nft_mint_authority.to_account_info()),
            payer: &self.paper_submitter.to_account_info(),
            owner: Some(&self.paper_submitter.to_account_info()),
            update_authority: Some(&self.oq_nft_mint_authority.to_account_info()),
            system_program: &self.system_program.to_account_info(),
            log_wrapper: None
        };
        Ok(())
    }
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
