use anchor_lang::prelude::*;

use crate::states::contexts::*;


// Initialize The Administrators
pub fn initialize_administrators(ctx: Context<AdministratorsInfo>, admins: Vec<Pubkey>) -> Result<()> {

    // Let's Initialize And Add The First Admin
   ctx.accounts.init(ctx.bumps)?;
   let administrators = &mut ctx.accounts.admins;

     // Let's iterate through the admins argument, and add each pubkey to the administrators if not existent
    for admin in admins.iter() {
        if !administrators.admins_pubkey.contains(admin) {
            administrators.admins_pubkey.push(*admin);
        }
    }
    Ok(())
}
