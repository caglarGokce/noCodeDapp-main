use crate::error::Errors::ArithmeticError;
use crate::state::{  RoleConfig,  TheRole};
use crate::datastates::{ DataConfig,  TheData, DataStr, };

use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::program::{invoke,  invoke_signed};

use solana_program::{
 account_info::{next_account_info, AccountInfo},
 entrypoint::ProgramResult, 
 pubkey::Pubkey, sysvar::{clock::Clock, Sysvar,},
    system_instruction,
  rent::Rent
};


pub fn  modify_or_propose_modification_data(
    accounts: &[AccountInfo],
    program_id:&Pubkey,
    new_data:DataStr
  ) -> ProgramResult {

  let accounts_iter: &mut std::slice::Iter<'_, AccountInfo<'_>> = &mut accounts.iter();


  let modifier: &AccountInfo<'_> = next_account_info(accounts_iter)?;
  let data_config_account: &AccountInfo<'_> = next_account_info(accounts_iter)?;
  let data_account: &AccountInfo<'_> = next_account_info(accounts_iter)?;
  let proposal_account: &AccountInfo<'_> = next_account_info(accounts_iter)?;
  let role_account: &AccountInfo<'_> = next_account_info(accounts_iter)?;
  let role_config_account: &AccountInfo<'_> = next_account_info(accounts_iter)?;

  if data_config_account.owner != program_id {panic!()}
  if data_account.owner != program_id {panic!()}
  if !modifier.is_signer{panic!()}

  let data_config: DataConfig = DataConfig::try_from_slice(&data_config_account.data.borrow())?;
  let the_data: TheData = TheData::try_from_slice(&data_account.data.borrow())?;


  let clock: Clock = Clock::get()?;
  let current_time: u64 = clock.unix_timestamp as u64;


  if the_data.project_no != data_config.project_no {panic!()}
  if the_data.hierachy_in_the_tree != data_config.hierachy_in_the_tree {panic!()}


  if data_config.is_confirmation_by_the_creator_required_to_modify == 1 {

    propose_modifying_data(
      modifier,proposal_account,data_account,
      role_account,role_config_account,program_id,
      &data_config,the_data,current_time,new_data)?;

  }else{

    modify_data(modifier, data_account, role_account, role_config_account, program_id, &data_config,  current_time, new_data,the_data)?;

  }


    Ok(())
  }

  fn modify_data<'info>(
    modifier:&AccountInfo<'info>,
    data_account:&AccountInfo<'info>,
    role_account:&AccountInfo<'info>,
    role_config_account:&AccountInfo<'info>,
    program_id:&Pubkey,
    data_config:&DataConfig,
    current_time:u64,
    new_data:DataStr,
    the_data:TheData
  ) -> ProgramResult {


   is_modification_valid(&the_data,&data_config,&current_time)?;

   if data_config.who_can_create.len() != 0 {
    modify_data_with_role(role_account,role_config_account,data_config,&current_time)?;
   }

   let project_no: u64 = the_data.project_no;
   let data_hierarchy_in_the_tree: u8 = the_data.hierachy_in_the_tree;
   let parent_no: u64 = the_data.parent_no;
   let data_no: u64 = the_data.data_no;
   let version_no:u64 = the_data.data_version.checked_add(1).ok_or(ArithmeticError)?;

   let seed: String = get_seed(project_no, data_hierarchy_in_the_tree, parent_no, data_no, version_no);


   let (data_account_address, bump) = Pubkey::find_program_address(&[&seed.as_bytes()], program_id);

   let fields: Vec<u64> = the_data.fields.clone();

   let data: TheData = TheData{
    creator: modifier.key.to_bytes(),
    project_no: the_data.project_no,
    hierachy_in_the_tree:the_data.hierachy_in_the_tree,
    parent_no: the_data.parent_no,
    data_no:the_data.data_no,
    data_version: version_no,
    last_time_data_added: the_data.last_time_data_added,
    last_modified_on: current_time,
    number_of_branches: the_data.number_of_branches,
    number_of_total_proposed_data: the_data.number_of_total_proposed_data,
    bump,
    data: new_data.data,
    fields,
    };

   let mut temp_slice: Vec<u8> =  Vec::new();

   the_data.serialize(&mut &mut temp_slice[..]).unwrap();

   let rent: Rent = Rent::default();
   let lamports: u64 = rent.minimum_balance(temp_slice.len());
   let space: u64 = temp_slice.len().try_into().unwrap();


   invoke_signed(&system_instruction::create_account(
    modifier.key,
    &data_account_address,
    lamports, 
    space, 
    program_id), 
    &[modifier.clone(),data_account.clone()], 
    &[&[&seed.as_bytes(), &[bump]]],
   )?;


   data.serialize(&mut &mut data_account.data.borrow_mut()[..])?;

    Ok(())
  }

  fn propose_modifying_data<'info>(
    creator:&AccountInfo<'info>,
    proposal_account:&AccountInfo<'info>,
    parent_data_account:&AccountInfo<'info>,
    role_account:&AccountInfo<'info>,
    role_config_account:&AccountInfo<'info>,
    program_id:&Pubkey,
    data_config:&DataConfig,
    mut parent_data:TheData,
    current_time:u64,
    data:DataStr
  ) -> ProgramResult {

   
   if data_config.who_can_create.len() != 0 {
    propose_modifying_data_with_role(role_account,role_config_account,data_config,&current_time)?;
   }

   let project_no: u64 = parent_data.project_no;
   let data_hierarchy_in_the_tree: u8 = parent_data.hierachy_in_the_tree.checked_add(1).ok_or(ArithmeticError)?;
   let parent_no: u64 = parent_data.number_of_branches.checked_add(1).ok_or(ArithmeticError)?;
   let data_no: u64 = parent_data.data_no;


   let the_data: TheData = TheData{
    creator: creator.key.to_bytes(),
    project_no: project_no,
    hierachy_in_the_tree:data_hierarchy_in_the_tree,
    parent_no: parent_no,
    data_no: 0,
    data_version: 0,
    last_time_data_added: current_time,
    last_modified_on: current_time,
    number_of_branches: 0,
    number_of_total_proposed_data: 0,
    bump: 0,
    data: data.data,
    fields: data.fields,
    };

   let mut temp_slice: Vec<u8> =  Vec::new();

   the_data.serialize(&mut &mut temp_slice[..]).unwrap();

   let rent: Rent = Rent::default();
   let lamports: u64 = rent.minimum_balance(temp_slice.len());
   let space: u64 = temp_slice.len().try_into().unwrap();


   invoke(&system_instruction::create_account(
    creator.key,
    &proposal_account.key,
    lamports, 
    space, 
    program_id), 
    &[creator.clone(),proposal_account.clone()], 
   )?;

   parent_data.number_of_total_proposed_data = data_no;

   parent_data.serialize(&mut &mut parent_data_account.data.borrow_mut()[..])?;
   the_data.serialize(&mut &mut proposal_account.data.borrow_mut()[..])?;

    Ok(())
  }

  fn modify_data_with_role(
    role_account:&AccountInfo,
    role_config_account:&AccountInfo,
    data_config:&DataConfig,
    current_time:&u64
  ) -> ProgramResult {

    let mut the_role: TheRole = TheRole::try_from_slice(&role_account.data.borrow())?;
    let role_config: RoleConfig = RoleConfig::try_from_slice(&role_config_account.data.borrow())?;


      if role_config.modification_limit_of_this_role_on_data.contains(&data_config.hierachy_in_the_tree){
      
        let index = role_config.modification_limit_of_this_role_on_data.iter().position(|&x| x == data_config.hierachy_in_the_tree).unwrap();
  
        if role_config.modification_limit[index] <= the_role.data_modified[index]{panic!()}
  
        the_role.data_modified[index] = the_role.data_modified[index].checked_add(1).ok_or(ArithmeticError)?;
  
      }


      is_role_valid(&the_role, &role_config, data_config, current_time)?;

      the_role.serialize(&mut &mut role_account.data.borrow_mut()[..])?;

    Ok(())
  }

  fn propose_modifying_data_with_role(
    role_account:&AccountInfo,
    role_config_account:&AccountInfo,
    data_config:&DataConfig,
    current_time:&u64
  ) -> ProgramResult {

    let mut the_role: TheRole = TheRole::try_from_slice(&role_account.data.borrow())?;
    let role_config: RoleConfig = RoleConfig::try_from_slice(&role_config_account.data.borrow())?;


      if role_config.proposal_for_modification_limit_of_this_role_on_data.contains(&data_config.hierachy_in_the_tree){
      
        let index = role_config.proposal_for_modification_limit_of_this_role_on_data.iter().position(|&x| x == data_config.hierachy_in_the_tree).unwrap();
  
        if role_config.proposal_for_modification_limit[index] <= the_role.data_proposed_to_modify[index]{panic!()}
  
        the_role.data_proposed_to_modify[index] = the_role.data_proposed_to_modify[index].checked_add(1).ok_or(ArithmeticError)?;
  
      }

      is_role_valid(&the_role, &role_config, data_config, current_time)?;


    the_role.serialize(&mut &mut role_account.data.borrow_mut()[..])?;

    Ok(())
  }

  fn is_role_valid(
    the_role:&TheRole,
    role_config:&RoleConfig,
    data_config:&DataConfig,
    current_time:&u64
  ) -> ProgramResult {


    if data_config.project_no != role_config.project_no{panic!()}
    if data_config.project_no != the_role.project_no{panic!()}
    if role_config.hierachy_in_the_roles != the_role.hierachy_in_the_roles{panic!()}

    if !data_config.who_can_modify.contains(&the_role.hierachy_in_the_roles){panic!()}


    if data_config.is_approval_by_the_creator_required_to_modify != 0 {
        if the_role.approved_to_modify_data != 1 {panic!()}
    }

    if role_config.time_required_to_modify != 0 {

        let time_passed: u64 = current_time - the_role.last_time_modified_data;

        if time_passed < role_config.time_required_until_modification{panic!()}
    }


    Ok(())
  }

  fn is_modification_valid(
    the_data:&TheData,
    data_config:&DataConfig,
    current_time:&u64
  )-> ProgramResult {


    if data_config.how_frequent_data_can_be_modified != 0 {
        let time_passed: u64 = current_time - the_data.last_modified_on;
        if time_passed < data_config.how_frequent_data_can_be_modified{panic!()}
    }
  
    if data_config.number_of_max_versions != 0{
      if the_data.data_version >= data_config.number_of_max_versions{panic!()}
    }

    Ok(())
  }

  fn get_seed(
    project_no:u64,
    data_hierarchy_in_the_tree:u8,
    parent_no:u64,
    data_no:u64,
    verison_no:u64
  )-> String {


    let mut seed_str:String = String::new();

    seed_str += &project_no.to_string();
    seed_str += &String::from("hie");
    seed_str += &data_hierarchy_in_the_tree.to_string();
    seed_str += &String::from("prnt");
    seed_str += &parent_no.to_string();
    seed_str += &String::from("dat");
    seed_str += &data_no.to_string();
    seed_str += &String::from("ver");
    seed_str += &verison_no.to_string();



    return seed_str;
  }


