use {
    num_derive::FromPrimitive,
    pinocchio::program_error::ProgramError,
    thiserror::Error
};

#[derive(Clone, Debug, Eq, Error, FromPrimitive, PartialEq)]
pub enum PinocchioError {
    #[error("Not a signer")]
    NotSigner,

    #[error("Invalid owner")]
    InvalidOwner,

    #[error("Invalid account data")]
    InvalidAccountData,

    #[error("Invalid address")]
    InvalidAddress,
}

impl From<PinocchioError> for ProgramError {
    fn from(e: PinocchioError) -> Self {
        ProgramError::Custom(e as u32)
    }
}