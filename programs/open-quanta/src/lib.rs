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
    pub fn initialize_admins(ctx: Context<AdministratorsInfo>) -> Result<()> {

        instructions::initialize_administrators(ctx)?;
        Ok(())
    }

    // ADD ADMINS TO THE ADMINISTRATORS ACCOUNT
    pub fn add_admin(ctx: Context<AdministratorsInfo>, admins: Vec<Pubkey>) -> Result<()> {

        instructions::add_administrator(ctx, admins)?;
        Ok(())
    }

    // INITIALIZE AUTHOR PROFILE
    pub fn initialize_author_profile(ctx: Context<AuthorProfileInfo>, profile_uri: String, field_of_study: String) -> Result<()> {

        instructions::init_author_profile(ctx, profile_uri, field_of_study)?;
        Ok(())
    }
}
