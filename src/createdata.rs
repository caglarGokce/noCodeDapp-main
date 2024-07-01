use crate::error::Errors::ArithmeticError;
use crate::rolestates::{  RoleConfig,   TheRole};
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



  pub fn  create_or_propose_creating_data(
    accounts: &[AccountInfo],
    program_id:&Pubkey,
    data:DataStr
  ) -> ProgramResult {

  let accounts_iter: &mut std::slice::Iter<'_, AccountInfo<'_>> = &mut accounts.iter();


  let creator: &AccountInfo<'_> = next_account_info(accounts_iter)?;
  let data_config_account: &AccountInfo<'_> = next_account_info(accounts_iter)?;
  let data_account: &AccountInfo<'_> = next_account_info(accounts_iter)?;
  let parent_data_account: &AccountInfo<'_> = next_account_info(accounts_iter)?;
  let proposal_account: &AccountInfo<'_> = next_account_info(accounts_iter)?;
  let role_account: &AccountInfo<'_> = next_account_info(accounts_iter)?;
  let role_config_account: &AccountInfo<'_> = next_account_info(accounts_iter)?;

  if data_config_account.owner != program_id {panic!()}
  if parent_data_account.owner != program_id {panic!()}
  if !creator.is_signer{panic!()}

  let data_config: DataConfig = DataConfig::try_from_slice(&data_config_account.data.borrow())?;
  let parent_data: TheData = TheData::try_from_slice(&parent_data_account.data.borrow())?;


  let clock: Clock = Clock::get()?;
  let current_time: u64 = clock.unix_timestamp as u64;

  let data_hierarchy_in_the_tree: u8 = parent_data.hierachy_in_the_tree.checked_add(1).ok_or(ArithmeticError)?;

  if parent_data.project_no != data_config.project_no {panic!()}
  if data_hierarchy_in_the_tree != data_config.hierachy_in_the_tree {panic!()}


  if data_config.is_confirmation_by_the_creator_required_to_create == 1 {

    propose_data(
      creator,proposal_account,parent_data_account,
      role_account,role_config_account,program_id,
      &data_config,parent_data,current_time,data)?;
  
  }else{

    create_data(
      creator,data_account,parent_data_account,
      role_account,role_config_account,program_id,
      &data_config,parent_data,current_time,data
    )?;


  }


    Ok(())
  }

  fn create_data<'info>(
    creator:&AccountInfo<'info>,
    data_account:&AccountInfo<'info>,
    parent_data_account:&AccountInfo<'info>,
    role_account:&AccountInfo<'info>,
    role_config_account:&AccountInfo<'info>,
    program_id:&Pubkey,
    data_config:&DataConfig,
    mut parent_data:TheData,
    current_time:u64,
    data:DataStr
  ) -> ProgramResult {

   
   is_creation_valid(&parent_data,&data_config,&current_time)?;

   if data_config.who_can_create.len() != 0 {
    create_data_with_role(role_account,role_config_account,data_config,&current_time)?;
   }

   let project_no: u64 = parent_data.project_no;
   let data_hierarchy_in_the_tree: u8 = parent_data.hierachy_in_the_tree.checked_add(1).ok_or(ArithmeticError)?;
   let parent_no: u64 = parent_data.number_of_branches.checked_add(1).ok_or(ArithmeticError)?;
   let data_no: u64 = parent_data.data_no;
   let verison_no:u64 = 1;

   let seed: String = get_seed(project_no, data_hierarchy_in_the_tree, parent_no, data_no, verison_no);


   let (data_account_address, bump) = Pubkey::find_program_address(&[&seed.as_bytes()], program_id);

   let the_data: TheData = TheData{
    creator: creator.key.to_bytes(),
    project_no: project_no,
    hierachy_in_the_tree:data_hierarchy_in_the_tree,
    parent_no: parent_no,
    data_no: data_no,
    data_version: verison_no,
    last_time_data_added: current_time,
    last_modified_on: current_time,
    number_of_branches: 0,
    number_of_total_proposed_data: 0,
    total_number_of_executions: Vec::new(),
    bump: bump,
    data: data.data,
    fields: data_config.initial_field_values.clone(),
    };

   let mut temp_slice: Vec<u8> =  Vec::new();

   the_data.serialize(&mut &mut temp_slice[..]).unwrap();

   let rent: Rent = Rent::default();
   let lamports: u64 = rent.minimum_balance(temp_slice.len());
   let space: u64 = temp_slice.len().try_into().unwrap();


   invoke_signed(&system_instruction::create_account(
    creator.key,
    &data_account_address,
    lamports, 
    space, 
    program_id), 
    &[creator.clone(),data_account.clone()], 
    &[&[&seed.as_bytes(), &[bump]]],
   )?;

   parent_data.number_of_branches = data_no;
   parent_data.last_time_data_added = current_time;

   parent_data.serialize(&mut &mut parent_data_account.data.borrow_mut()[..])?;
   the_data.serialize(&mut &mut data_account.data.borrow_mut()[..])?;

    Ok(())
  }

  fn propose_data<'info>(
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
    propose_creating_data_with_role(role_account,role_config_account,data_config,&current_time)?;
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
    total_number_of_executions: Vec::new(),
    bump: 0,
    data: data.data,
    fields: data_config.initial_field_values.clone()
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

  fn create_data_with_role(
    role_account:&AccountInfo,
    role_config_account:&AccountInfo,
    data_config:&DataConfig,
    current_time:&u64
  ) -> ProgramResult {

    let mut the_role: TheRole = TheRole::try_from_slice(&role_account.data.borrow())?;
    let role_config: RoleConfig = RoleConfig::try_from_slice(&role_config_account.data.borrow())?;


      if role_config.creation_limit_of_this_role_on_data.contains(&data_config.hierachy_in_the_tree){
      
        let index = role_config.creation_limit_of_this_role_on_data.iter().position(|&x| x == data_config.hierachy_in_the_tree).unwrap();
  
        if role_config.creation_limit[index] <= the_role.data_created[index]{panic!()}
  
        the_role.data_created[index] = the_role.data_created[index].checked_add(1).ok_or(ArithmeticError)?;
  
      }


      is_role_valid(&the_role, &role_config, data_config, current_time)?;

      the_role.serialize(&mut &mut role_account.data.borrow_mut()[..])?;

    Ok(())
  }

  fn propose_creating_data_with_role(
    role_account:&AccountInfo,
    role_config_account:&AccountInfo,
    data_config:&DataConfig,
    current_time:&u64
  ) -> ProgramResult {

    let mut the_role: TheRole = TheRole::try_from_slice(&role_account.data.borrow())?;
    let role_config: RoleConfig = RoleConfig::try_from_slice(&role_config_account.data.borrow())?;


      if role_config.proposal_for_creation_limit_of_this_role_on_data.contains(&data_config.hierachy_in_the_tree){
      
        let index = role_config.proposal_for_creation_limit_of_this_role_on_data.iter().position(|&x| x == data_config.hierachy_in_the_tree).unwrap();
  
        if role_config.proposal_for_creation_limit[index] <= the_role.data_proposed_to_create[index]{panic!()}
  
        the_role.data_proposed_to_create[index] = the_role.data_proposed_to_create[index].checked_add(1).ok_or(ArithmeticError)?;
  
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

    if !data_config.who_can_create.contains(&the_role.hierachy_in_the_roles){panic!()}

    if the_role.is_enabled != 1 {panic!()}

    if data_config.is_approval_by_the_creator_required_to_create != 0 {
        if the_role.approved_to_create_data != 1 {panic!()}
    }

    if role_config.time_required_to_create != 0 {

        let time_passed: u64 = current_time - the_role.last_time_created_data;

        if time_passed < role_config.time_required_until_creation{panic!()}
    }


    Ok(())
  }

  fn is_creation_valid(
    parent_data:&TheData,
    data_config:&DataConfig,
    current_time:&u64
  )-> ProgramResult {


    if data_config.how_frequent_data_can_be_created != 0 {
        let time_passed: u64 = current_time - parent_data.last_time_data_added;
        if time_passed < data_config.how_frequent_data_can_be_created{panic!()}
    }
  
    if data_config.number_of_max_branches != 0{
      if parent_data.number_of_branches >= data_config.number_of_max_branches{panic!()}
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


