use anchor_lang::prelude::*;

use crate::states::{contexts::*, accounts::*, events::*};


// INSTRUCTION FOR PAPER SUBMISSION
pub fn paper_submit(
    ctx: Context<PaperInfo>, 
    paper_args: PaperArgs, paper_sub_owners: Vec<Pubkey>
) -> Result<()> {

    // Get Context
    //let paper_information = &mut ctx.accounts.research_paper;
    let paper_id_counter = &mut ctx.accounts.paper_id_assigner;
    let author_profile_information = &mut ctx.accounts.author_profile;
    let paper_title = paper_args.title_of_paper.clone();

    // Set Up Paper Details
    let _paper_information = &mut ctx.accounts.research_paper.set_inner(
        Paper {
            owner_of_paper: ctx.accounts.paper_submitter.key(),
            research_sub_owners: paper_sub_owners,
            title_of_paper: paper_args.title_of_paper.clone(),
            field_of_research: paper_args.field_of_research.clone(),
            arweave_hash_to_paper: paper_args.paper_arweave_hash.clone(),
            open_quanta_paper_id: format!("OQ-{:10}", paper_id_counter.current_id + 1),
            paper_version: paper_args.paper_version.clone(),
            time_of_submission: Clock::get()?.unix_timestamp,
            paper_bump: ctx.bumps.research_paper
        }
    );

    // Update Global Paper ID Counter
    paper_id_counter.current_id += 1;

    // Update Author Profile On Submission
    author_profile_information.number_of_submitted_papers += 1;

    // Mint AUTHORSHIP NFT HERE
    ctx.accounts.mint_authorship_nft(ctx.bumps, paper_args)?;
    
    // Emit An Event For The Paper Submission

    emit!(
        PaperSubmission {
            paper_id: ctx.accounts.research_paper.open_quanta_paper_id.clone(),
            paper_submitter: ctx.accounts.research_paper.owner_of_paper,
            title_of_paper: paper_title,
            time_of_submission: Clock::get()?.unix_timestamp
        }
    );

    Ok(())
}