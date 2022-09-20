use thiserror::Error;

use solana_program::program_error::ProgramError;

#[derive(Error, Debug, Copy, Clone)]
pub enum MultisendError {
    /// Invalid instruction
    #[error("Invalid Instruction")]
    InvalidInstruction,
}

impl From<MultisendError> for ProgramError {
  fn from(e: MultisendError) -> Self {
      ProgramError::Custom(e as u32)
  }
}