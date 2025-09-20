use anchor_lang::prelude::*;
use crate::states::contexts::*;
use crate::states::accounts::CollectionEntry;
use mpl_core::{
    types::{
        PluginAuthorityPair, Plugin, PermanentFreezeDelegate, 
        Royalties, Creator, RuleSet, PermanentBurnDelegate,
    }, 
    instructions::CreateCollectionV2CpiBuilder,
};




// INITIALIZE PAPER ID GLOBAL COUNTER
pub fn init_paper_counter(ctx: Context<PaperIDCounterInfo>) -> Result<()> {

    ctx.accounts.init(ctx.bumps)?;
    
    Ok(())
}

// INITIALIZE COLLECTION REGISTRY
pub fn init_collection_registry(ctx: Context<CollectionRegistryInfo>) -> Result<()> {
    let collection_registry = &mut ctx.accounts.collection_registry;
    collection_registry.registry_bump = ctx.bumps.collection_registry;
    collection_registry.total_collections = 0;
    collection_registry.collection_entries = vec![];
    collection_registry.mint_authority = ctx.accounts.oq_nft_mint_authority.key();
    collection_registry.collection_mint = ctx.accounts.collection_mint.key();
    
    msg!("Collection Registry initialized successfully");
    Ok(())
}


// CREATE OPENQUANTA COLLECTION
pub fn create_collection(ctx: Context<CreateCollection>, name: String, uri: String) -> Result<()> {
    let mut collection_plugins = vec![];

    // 1. PermanentFreezeDelegate - Admin can freeze/unfreeze assets
    collection_plugins.push(PluginAuthorityPair { 
        plugin: Plugin::PermanentFreezeDelegate(PermanentFreezeDelegate { 
            frozen: false 
        }), 
        authority: /*Some(PluginAuthority::Owner)*/None
    });
    

    // 2. Royalties - 5% royalties for research funding with admin as sole beneficiary
    collection_plugins.push(PluginAuthorityPair {
        plugin: Plugin::Royalties(Royalties {
            basis_points: 500, // 5%
            creators: vec![Creator {
                address: ctx.accounts.admin.key(),
                percentage: 100,
            }],
            rule_set: RuleSet::None,
        }),
        authority: None
    });

    // 3. PermanentBurnDelegate - Admin can burn invalid/plagiarized research
    collection_plugins.push(PluginAuthorityPair {
        plugin: Plugin::PermanentBurnDelegate(PermanentBurnDelegate {}),
        authority: None
    });

    // Store collection information in the registry first
    let collection_registry = &mut ctx.accounts.collection_registry;
    let clock = Clock::get()?;
    
    let collection_entry = CollectionEntry {
        collection_address: ctx.accounts.collection.key(),
        collection_name: name.clone(),
        collection_uri: uri.clone(),
        created_at: clock.unix_timestamp,
        created_by: ctx.accounts.admin.key(),
    };
    
    // Create the collection via mpl-core CPI
    CreateCollectionV2CpiBuilder::new(&ctx.accounts.mpl_core_program.to_account_info())
        .collection(&ctx.accounts.collection.to_account_info())
        .payer(&ctx.accounts.payer.to_account_info())
        .system_program(&ctx.accounts.system_program.to_account_info())
        .name(name)
        .uri(uri)
        .plugins(collection_plugins)
        .invoke()?;

    // Add to registry after successful creation
    
    collection_registry.collection_entries.push(collection_entry);
    collection_registry.total_collections += 1;
    
    msg!(
        "Collection created and registered: {} at address {}", 
        collection_registry.total_collections,
        ctx.accounts.collection.key()
    );

    Ok(())
}