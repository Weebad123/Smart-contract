use anchor_lang::prelude::*;
use crate::states::{contexts::*, errors::*};

use anchor_spl::
    metadata::{create_master_edition_v3, create_metadata_accounts_v3, mpl_token_metadata::types::{ CollectionDetails, DataV2},
     CreateMasterEditionV3, CreateMetadataAccountsV3};


// INITIALIZE PAPER ID GLOBAL COUNTER
pub fn init_paper_counter(ctx: Context<PaperIDCounterInfo>) -> Result<()> {

    let paper_id_counter = &mut ctx.accounts.paper_id_assigner;
    paper_id_counter.current_id = 0;
    paper_id_counter.counter_bump = ctx.bumps.paper_id_assigner;
    
    Ok(())
}


// INITIALIZE THE OpenQuanta PARENT AUTHORSHIP NFT COLLECTION
pub fn init_master_authorship_nft_collection(ctx: Context<MasterAuthorshipNFTCollection>, nft_uri: String) -> Result<()> {

    Ok(())
}