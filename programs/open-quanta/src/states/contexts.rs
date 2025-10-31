
use anchor_lang::prelude::*;

use mpl_core::{
    ID as MPL_CORE_PROGRAM_ID, accounts::BaseCollectionV1, instructions::CreateV2CpiBuilder, types::{
        Attribute, Attributes, BurnDelegate, FreezeDelegate, Plugin, PluginAuthority, PluginAuthorityPair
    }};

use crate::{Administrators, AuthorProfile, CollectionRegistry, Paper, PaperArgs, PaperIDCounter, ReviewerProfile};
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
        seeds = [b"administrators".as_ref(), b"openQuanta".as_ref()],
        bump
    )]
    pub admins: Account<'info, Administrators>,

    #[account(
        init,
        payer = admin,
        space = 8 + CollectionRegistry::INIT_SPACE,
        seeds = [b"collections_registry".as_ref(), b"openQuanta".as_ref()],
        bump,
    )]
    pub collection_registry: Account<'info, CollectionRegistry>,

    ///CHECK: SAFE TO IGNORE FOR NOW
     #[account(
        mut,
        seeds = [b"openQuanta_Nft_Mint_Authority".as_ref()],
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
        seeds = [b"administrators".as_ref(), b"openQuanta".as_ref()],
        bump
    )]
    pub admins: Account<'info, Administrators>,

    #[account(
        init,
        payer = admin,
        space = 8 + PaperIDCounter::INIT_SPACE,
        seeds = [b"paper_id_counter".as_ref(), b"openQuanta".as_ref()],
        bump,
    )]
    pub paper_id_assigner: Account<'info, PaperIDCounter>,

    /// CHECK: SAFE TO USE
    #[account(
        init,
        payer = admin,
        space = 8,
        seeds = [b"openQuanta_Nft_Mint_Authority".as_ref()],
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
        seeds = [b"paper_id_counter".as_ref(), b"openQuanta".as_ref()],
        bump,
    )]
    pub paper_id_assigner: Account<'info, PaperIDCounter>,

    #[account(
        mut,
        seeds = [b"author_profile", paper_submitter.key().as_ref()],
        bump,
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
        seeds = [b"collections_registry".as_ref(), b"openQuanta".as_ref()],
        bump
    )]
    pub collection_registry: Account<'info, CollectionRegistry>,

    /// CHECK: SAFE TO USE
    #[account(
        mut,
        constraint = collection_registry.collection_mints.contains(&collection.key()) @OpenQuantaErrors::InvalidCollection
    )]
    //pub collection: AccountInfo<'info>,
    pub collection: Option<Account<'info, BaseCollectionV1>>,

    /// CHECK: SAFE TO USE
    #[account(
        mut,
        seeds = [b"openQuanta_Nft_Mint_Authority".as_ref()],
        bump,
    )]
    pub oq_nft_mint_authority: AccountInfo<'info>,

    /// CHECK: SAFE TO USE
    #[account(
        mut,
        signer,
        //seeds = [b"asset", paper_submitter.key().as_ref(), &format!("OQ-{:07}", paper_id_assigner.current_id + 1).as_bytes()],
        //bump
    )]
    pub nft_asset: AccountInfo<'info>,

    #[account(address = MPL_CORE_PROGRAM_ID)]
    /// CHECK: This doesn't need to be checked, because there is the address constraint
    pub mpl_core_program: UncheckedAccount<'info>,

    pub system_program: Program<'info, System>,
}

// IMPLEMENT NFT MINTING LOGIC HERE
// Mint from the OQ Collections, and Add Research Attributes to NFT
impl<'info> PaperInfo<'info> {
    pub fn mint_authorship_nft(&mut self, bumps: PaperInfoBumps, paper_args: PaperArgs) -> Result<()> {

        let authority_seeds = &[
            b"openQuanta_Nft_Mint_Authority".as_ref(),
            &[bumps.oq_nft_mint_authority]
        ];

        let _submitter = self.paper_submitter.key();
        let _id_gen = format!("OQ-{:07}", self.paper_id_assigner.current_id + 1);
        /*let _nft_asset_seeds = &[
            b"asset",
            submitter.as_ref(), 
            &id_gen.as_bytes(),
            &[bumps.nft_asset]
        ];*/
        let signers_seeds = &[&authority_seeds[..] /* , &nft_asset_seeds[..]*/];

        let mut asset_plugins : Vec<PluginAuthorityPair> = vec![];
        let asset_attributes: Vec<Attribute> = vec![
            Attribute {
                key: "OpenQuanta NFT".to_string(),
                value: "Authorship NFT".to_string()
            },
            Attribute {
                key: "owner_of_paper".to_string(),
                value: self.paper_submitter.key().to_string()
            },
            
            /*Attribute {
                key: "title of research paper".to_string(),
                value: paper_args.title_of_paper
            },*/

            Attribute {
                key: "field of research paper".to_string(),
                value: paper_args.field_of_research
            },

            Attribute {
                key: "paper version".to_string(),
                value: paper_args.paper_version.to_string()
            },
/* 
            Attribute {
                key: "ipfs hash of research".to_string(),
                value: paper_args.paper_ipfs_hash
            },*/
        ];

        asset_plugins.push(PluginAuthorityPair {
            plugin: Plugin::Attributes(Attributes { attribute_list: asset_attributes }),
            authority: None/*Some(PluginAuthority::UpdateAuthority) might not be needed */
        });
 
        asset_plugins.push(PluginAuthorityPair {
            plugin: Plugin::FreezeDelegate(FreezeDelegate{ frozen: true }),
            authority: Some(PluginAuthority::UpdateAuthority)
        });
 
        asset_plugins.push(PluginAuthorityPair {
            plugin: Plugin::BurnDelegate(BurnDelegate {}),
            authority: Some(PluginAuthority::UpdateAuthority)
        });
        /* Might add later
        asset_plugins.push(PluginAuthorityPair {
            plugin: Plugin::Edition(Edition {
                number: 1,
            }),
            authority: None
        });*/

        /*let collection = match &self.collection {
            Some(collection) => Some(collection.to_account_info()),
            None => None,
        };*/

        msg!("paper_submitter:   {} signer={} writable={}", self.paper_submitter.key(), self.paper_submitter.to_account_info().is_signer, self.paper_submitter.to_account_info().is_writable);
        msg!("paper_id_assigner: {} signer={} writable={}", self.paper_id_assigner.key(), self.paper_id_assigner.to_account_info().is_signer, self.paper_id_assigner.to_account_info().is_writable);
        msg!("author_profile:    {} signer={} writable={}", self.author_profile.key(), self.author_profile.to_account_info().is_signer, self.author_profile.to_account_info().is_writable);
        msg!("collection_registry:{} signer={} writable={}", self.collection_registry.key(), self.collection_registry.to_account_info().is_signer, self.collection_registry.to_account_info().is_writable);
        if let Some(c) = &self.collection {
        msg!("collection:        {} signer={} writable={}", c.key(), c.to_account_info().is_signer, c.to_account_info().is_writable);
        } else {
        msg!("collection:        None");
        }
        msg!("oq_nft_mint_authority:{} signer={} writable={}", self.oq_nft_mint_authority.key(), self.oq_nft_mint_authority.is_signer, self.oq_nft_mint_authority.is_writable);
        msg!("nft_asset:         {} signer={} writable={}", self.nft_asset.key(), self.nft_asset.to_account_info().is_signer, self.nft_asset.to_account_info().is_writable);
        msg!("mpl_core_program:  {} signer={} writable={}", self.mpl_core_program.key(), self.mpl_core_program.to_account_info().is_signer, self.mpl_core_program.to_account_info().is_writable);


        CreateV2CpiBuilder::new(&self.mpl_core_program.to_account_info())
        .asset(&self.nft_asset.to_account_info())
        //.collection(collection.as_ref())
        .owner(Some(&self.paper_submitter.to_account_info()))
        .authority(Some(&self.oq_nft_mint_authority.to_account_info()))
        .payer(&self.paper_submitter.to_account_info())
        .update_authority(Some(&self.oq_nft_mint_authority.to_account_info()))
        .system_program(&self.system_program.to_account_info())
        .name(paper_args.title_of_paper)
        .uri(paper_args.paper_arweave_hash)
        .plugins(asset_plugins)
        .invoke_signed(signers_seeds)?;
        Ok(())
    }
}


// OPENQUANTA COLLECTION CREATION CONTEXT
#[derive(Accounts)]
pub struct CreateCollection<'info> {
    #[account(
        mut,
        constraint = admins.admins_pubkey.contains(&admin.key()) @OpenQuantaErrors::OnlyAdmin
    )]
    pub admin: Signer<'info>,

    #[account(
        seeds = [b"administrators".as_ref(), b"openQuanta".as_ref()],
        bump 
    )]
    pub admins: Account<'info, Administrators>,

    #[account(
        mut,
        seeds = [b"collections_registry".as_ref(), b"openQuanta".as_ref()],
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
