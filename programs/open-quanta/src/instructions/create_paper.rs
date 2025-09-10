use anchor_lang::prelude::*;

use crate::states::{contexts::*, errors::*, accounts::*, events::*};


// INSTRUCTION FOR PAPER SUBMISSION
pub fn paper_submit(
    ctx: Context<PaperInfo>, 
    title_of_paper: String, paper_ipfs_hash: String, 
    field_of_research: String, paper_version: u8, paper_sub_owners: Vec<Pubkey>
) -> Result<()> {

    // Get Context
    let paper_information = &mut ctx.accounts.research_paper;
    let paper_id_counter = &mut ctx.accounts.paper_id_assigner;
    let author_profile_information = &mut ctx.accounts.author_profile;
    let paper_title = title_of_paper.clone();

    // Set Up Paper Details
    paper_information.set_inner(
        Paper {
            owner_of_paper: ctx.accounts.paper_submitter.key(),
            research_sub_owners: paper_sub_owners,
            title_of_paper,
            field_of_research,
            ipfs_hash_to_paper: paper_ipfs_hash,
            open_quanta_paper_id: format!("OQ-{:10}", paper_id_counter.current_id + 1),
            paper_version,
            time_of_submission: Clock::get()?.unix_timestamp,
            paper_bump: ctx.bumps.research_paper
        }
    );

    // Update Global Paper ID Counter
    paper_id_counter.current_id += 1;

    // Update Author Profile On Submission
    author_profile_information.number_of_submitted_papers += 1;


    
    // Emit An Event For The Paper Submission

    emit!(
        PaperSubmission {
            paper_id: paper_information.open_quanta_paper_id.clone(),
            paper_submitter: paper_information.owner_of_paper,
            title_of_paper: paper_title,
            time_of_submission: Clock::get()?.unix_timestamp
        }
    );

    Ok(())
}