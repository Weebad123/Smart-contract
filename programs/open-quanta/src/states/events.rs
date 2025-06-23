use anchor_lang::prelude::*;





#[event]
pub struct PaperSubmission {
    pub paper_submitter: Pubkey,

    pub paper_id: String,

    pub title_of_paper: String,

    pub time_of_submission: i64,

}