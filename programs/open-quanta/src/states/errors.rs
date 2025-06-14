use anchor_lang::prelude::error_code;


#[error_code]
pub enum OpenQuantaErrors {
    #[msg("Privileged Instruction: Callable By Admin Only")]
    OnlyAdmin,
}