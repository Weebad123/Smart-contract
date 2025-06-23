use anchor_lang::prelude::*;


// ADMIN ACCOUNT
// There could be many privileged roles in the system, hence the Administrators
#[account]
#[derive(InitSpace)]
pub struct Administrators {

    #[max_len(5)]
    pub admins_pubkey: Vec<Pubkey>,

    pub admins_bump: u8,
}


// AUTHOR PDA ACCOUNT
// 
#[account]
#[derive(InitSpace)]
pub struct AuthorProfile {

    pub author_pubkey: Pubkey,

    pub author_bump: u8,

    #[max_len(100)]
    pub author_profile_uri: String,

    pub reputation_score: u8,

    #[max_len(20)]
    pub author_field_of_study: String,

    pub number_of_submitted_papers: u16,

    pub joined_at: i64,

}


// REVIEWER PDA ACCOUNT
//
#[account]
#[derive(InitSpace)]
pub struct ReviewerProfile {

    pub reviewer_pubkey: Pubkey,

    pub reviewer_bump: u8,

    #[max_len(64)]
    pub reviewer_profile_uri: String,

    pub reputation_score: u8,

    #[max_len(20)]
    pub reviewer_field_of_study: String,

    pub joined_at: i64,
}


// GLOBAL PAPER ID ASSIGNER
#[account]
#[derive(InitSpace)]
pub struct PaperIDCounter {
    pub current_id: u64,

    pub counter_bump: u8,
}



// PAPER SUBMISSION 
#[account]
#[derive(InitSpace)]
pub struct Paper {
    pub owner_of_paper: Pubkey,

    #[max_len(5)]
    pub research_sub_owners: Vec<Pubkey>,

    #[max_len(100)]
    pub title_of_paper: String,

    #[max_len(50)]
    pub field_of_research: String,

    #[max_len(50)]
    pub ipfs_hash_to_paper: String,

    #[max_len(10)]
    pub open_quanta_paper_id: String,

    pub paper_version: u8,

    pub time_of_submission: i64,

    pub paper_bump: u8,
}