use anchor_lang::prelude::*;

use crate::{states::contexts::*, AuthorProfile, ReviewerProfile};



// INITIALIZE AUTHOR PROFILE
pub fn init_author_profile(ctx: Context<AuthorProfileInfo>, author_profile_uri: String, author_field_of_study: String) -> Result<()> {

    // Get The Context, and Update Author info
    let author_profile_information = &mut ctx.accounts.author_profile;

    author_profile_information.set_inner(
        AuthorProfile {
            author_pubkey: ctx.accounts.author.key(),
            author_bump: ctx.bumps.author_profile,
            author_profile_uri,
            reputation_score: 0,
            author_field_of_study,
            number_of_submitted_papers: 0,
            joined_at: Clock::get()?.unix_timestamp
        });
    
    Ok(())
}


// INITIALIZE REVIEWER PROFILE
pub fn init_reviewer_profile(ctx: Context<ReviewerProfileInfo>, reviewer_profile_uri: String, reviewer_field_of_study: String) -> Result<()> {

    // Update Reviewer Info
    let reviewer_profile_information = &mut ctx.accounts.reviewer_profile;

    reviewer_profile_information.set_inner(
        ReviewerProfile {
            reviewer_pubkey: ctx.accounts.reviewer.key(),
            reviewer_bump: ctx.bumps.reviewer_profile,
            reviewer_profile_uri,
            reputation_score: 0,
            reviewer_field_of_study,
            joined_at: Clock::get()?.unix_timestamp
        }
    );
    
    Ok(())
}