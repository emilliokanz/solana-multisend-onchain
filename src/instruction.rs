use {
  solana_program::clock::UnixTimestamp,
  solana_program::program_error::ProgramError,
  borsh::BorshDeserialize,
};

use solana_program::msg;

#[derive(Debug, PartialEq)]
pub enum MultisendInstruction {
    /// Create a new MultiSend Transaction
    /// 
    /// Accounts expected:
    /// 
    /// 0. `[signer]` The account of the person initializing the multisend program
    /// 1. `[writable]` vector of accounts and amounts to be send
    /// 2. `[]` the system program
  
  
  TransferData{
    payload: TransferDataPayload,
  },
}

#[derive(BorshDeserialize, Debug, PartialEq)]
pub struct TransferDataPayload {
    pub address: Vec<String>,
    pub amount: Vec<u32>,
}

impl MultisendInstruction {
  pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
    let (ix_id, rest) = input.split_first().ok_or(ProgramError::InvalidInstructionData)?;
    match ix_id {
      0 => {
        let payload = TransferDataPayload::try_from_slice(rest)?;
        msg!("payload -> {:?}", payload);
        Ok(Self::TransferData {
            payload
        })
      },
       _ => Err(ProgramError::InvalidInstructionData),
    }
  }
}