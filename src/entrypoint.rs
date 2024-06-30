use crate::confirmdata::confirm_proposal_for_creating_data;
use crate::createrole::create_assign_or_apply_for_a_role;
use crate::initialize::{add_token_to_project, init_data_config, init_role_config, initialize_project_with_token, initialize_project_without_token};
use crate::modifydata::modify_or_propose_modification_data;
use crate::state::RoleConfig;
use crate::datastates::{ DataConfig,  TheData, DataStr, };

use borsh::BorshDeserialize;
use solana_program::{
  account_info::AccountInfo, entrypoint, entrypoint::ProgramResult, pubkey::Pubkey,
};
use crate::error::Errors::InvalidInstruction;

use crate::createdata::create_or_propose_creating_data;

entrypoint!(process_instruction);
fn process_instruction(
  program_id: &Pubkey,
  accounts: &[AccountInfo],
  instruction_data: &[u8],
) -> ProgramResult {

  let (tag, rest) = instruction_data.split_first().ok_or(InvalidInstruction)?;

    Ok(match tag{
      0 => {
        let data: TheData = TheData::try_from_slice(&rest)?;
        initialize_project_without_token(accounts, program_id, data)?;
      },
      1 => {
        let data: TheData = TheData::try_from_slice(&rest)?;
        initialize_project_with_token(accounts, program_id, data)?;
      },
      2 => {
        add_token_to_project(accounts, program_id)?;
      },
      3 => {
        let data: DataConfig = DataConfig::try_from_slice(&rest)?;
        init_data_config(accounts, program_id, data)?;
      },
      4 => {
        let data: RoleConfig = RoleConfig::try_from_slice(&rest)?;
        init_role_config(accounts, program_id, data)?;
      },
      5 => {
        let data: DataStr = DataStr::try_from_slice(&rest)?;
        create_or_propose_creating_data(accounts, program_id, data)?;
      },
      6 => {
        create_assign_or_apply_for_a_role(accounts, program_id)?;
      },
      7 => {
        let new_data: DataStr = DataStr::try_from_slice(&rest)?;
        modify_or_propose_modification_data(accounts, program_id,new_data)?;
      },
      8 => {
        confirm_proposal_for_creating_data(accounts, program_id)?;
      },


      /*  
  DeleteData,
  OfferDataDeletion,
  ConfirmDataDeletion,
  CreateRole,
  DeleteRole,
  InitCounter, 
      */

      _ => return Err(InvalidInstruction.into()),
  })

}


