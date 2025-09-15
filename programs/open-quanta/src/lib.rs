use anchor_lang::prelude::*;

pub mod instructions;
pub mod states;


pub use instructions::*;
pub use states::*;

declare_id!("AQ3rnwW5ymhjJEgaer7xMcQMm9tVCUkDHfZptgVtYDS2");

#[program]
pub mod open_quanta {
    use super::*;


    // INITIALIZE ADMINSTRATORS ACCOUNT
    pub fn initialize_admins(ctx: Context<AdministratorsInfo>, admins: Vec<Pubkey>) -> Result<()> {

        instructions::initialize_administrators(ctx, admins)?;
        Ok(())
    }

    // INITIALIZE COLLECTION REGISTRY
    pub fn initialize_collection_registry(ctx: Context<CollectionRegistryInfo>) -> Result<()> {
        instructions::init_collection_registry(ctx)?;
        Ok(())
    }

    // INITIALIZE PAPER ID COUNTER
    pub fn initialize_paper_id(ctx: Context<PaperIDCounterInfo>) -> Result<()> {

        instructions::init_paper_counter(ctx)?;
        Ok(())
    }

    // INITIALIZE AUTHOR PROFILE
    pub fn initialize_author_profile(ctx: Context<AuthorProfileInfo>, profile_uri: String, field_of_study: String) -> Result<()> {

        instructions::init_author_profile(ctx, profile_uri, field_of_study)?;
        Ok(())
    }

    // INITIALIZE REVIEWER PROFILE
    pub fn initialize_reviewer_profile(ctx: Context<ReviewerProfileInfo>, reviewer_profile_uri: String, field_of_study: String) -> Result<()> {

        instructions::init_reviewer_profile(ctx, reviewer_profile_uri, field_of_study)?;
        Ok(())
    }

    // SUBMIT A PAPER
    pub fn submit_paper(
    ctx: Context<PaperInfo>, 
    title_of_paper: String, paper_ipfs_hash: String, 
    field_of_research: String, paper_version: u8, paper_sub_owners: Vec<Pubkey>
) -> Result<()> {

    instructions::paper_submit(ctx, title_of_paper, paper_ipfs_hash, field_of_research, paper_version, paper_sub_owners)?;
    Ok(())
}

    // CREATE OPENQUANTA COLLECTION
    pub fn create_collection(ctx: Context<CreateCollection>, name: String, uri: String) -> Result<()> {
        instructions::create_collection(ctx, name, uri)
    }
}
