use std::collections::btree_map::Iter;
use std::fmt;

use solana_program::{
  account_info::{next_account_info, AccountInfo},
  entrypoint::ProgramResult,
  program_error::ProgramError,
  msg,
  system_instruction::{transfer_many, transfer},
  pubkey::Pubkey,
  program_pack::{Pack, IsInitialized},
  sysvar::{rent::Rent, Sysvar},
  program::{invoke, invoke_signed}
};

use crate::{instruction::{MultisendInstruction, TransferDataPayload}, error::MultisendError};

pub struct Processor;
impl Processor {
  pub fn process(program_id: &Pubkey, accounts: &[AccountInfo], instruction_data: &[u8]) -> ProgramResult {
    // calling the instruction function to unpack the instruction data from entrypoint
    let instruction = MultisendInstruction::unpack(instruction_data)?;

    match instruction {
        MultisendInstruction::TransferData { payload } => {
            msg!("Instruction: InitMultisend");
            Self::process_init_multisend(program_id, accounts, payload)
        }
    }
}
 pub fn process_init_multisend(
      program_id: &Pubkey,
      accounts: &[AccountInfo],
      payload: TransferDataPayload,
  ) -> ProgramResult {
    // making initializer account, made &mut or mutable so it can be changed
      let account_info_iter = &mut accounts.iter();
    // initializer is made a signer 
      let from_account = next_account_info(account_info_iter)?;

      if !from_account.is_signer {
        return Err(ProgramError::MissingRequiredSignature);
      }

      let program_id = next_account_info(account_info_iter)?;
      
      msg!("data: {:?}", payload);

      Ok(())
  }
}