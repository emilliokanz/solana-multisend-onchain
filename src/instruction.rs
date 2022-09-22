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

#[derive(Debug, PartialEq)]
pub struct TransferDataPayload {
    pub addresses: Vec<String>,
    pub amounts: Vec<u32>,
}

impl TransferDataPayload {
  fn from_slice(bytes: &[u8]) -> Self {
    let data_length: [u8; 4] = bytes[0..4];

    let data_length = u32::from_le_bytes(data_length);

    let total_addresses_bytes = (data_length * 48) as usize;
    let addresses: [u8; total_addresses_bytes] = bytes[4..total_addresses_bytes];

    for i in 0..data_length {
      let address: [u8; 44] = addresses[(i+4) as usize..(i+44) as usize];
      msg!("address: {:?}", address);
    }

    return TransferDataPayload { addresses: Vec::new(), amounts: Vec::new() }
  }
}

impl MultisendInstruction {
  pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
    let (ix_id, rest)= input.split_first().ok_or(ProgramError::InvalidInstructionData)?;
    msg!("ix_id -> {:?}", ix_id);
    msg!("rest => {:?}", rest);
    match ix_id {
      0 => Ok(Self::TransferData { 
        payload: TransferDataPayload::from_slice(rest)
      }),
      _ => Err(ProgramError::InvalidInstructionData) 
    }
  }
}


    // pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
    //   let (tag, rest) = input.split_first().ok_or(ProgramError::InvalidInstructionData)?;
    //   Ok (match tag {
    //       0 => Self::TransferData {
    //               payload: Self::unpack_payload(rest)?,
    //            },
    //       _ => return Err(ProgramError::InvalidInstructionData),
    // })
    // }
    // fn unpack_payload(input: &[u8]) -> Result<u64, ProgramError> {
    //   let payload = input
    //       .get(..8)
    //       .and_then(|slice| slice.try_into().ok())
    //       .map(u64::from_le_bytes)
    //       .ok_or(ProgramError::InvalidInstructionData)?;
    //   Ok(payload)