use anchor_lang::prelude::*;

use crate::states::contexts::*;


// Initialize The Administrators
pub fn initialize_administrators(ctx: Context<AdministratorsInfo>) -> Result<()> {

    // Let's Initialize And Add The First Admin
    let administrators = &mut ctx.accounts.admins;
    administrators.admins_bump = ctx.bumps.admins;
    administrators.admins_pubkey = Vec::new();
    Ok(())
}


// Add An Administrator
pub fn add_administrator(ctx: Context<AdministratorsInfo>, admins: Vec<Pubkey>) -> Result<()> {

    //
    let administrators = &mut ctx.accounts.admins;

    // Let's iterate through the admins argument, and add each pubkey to the administrators if not existent
    for admin in admins.iter() {
        if !administrators.admins_pubkey.contains(admin) {
            administrators.admins_pubkey.push(*admin);
        }
    }

    Ok(())
}