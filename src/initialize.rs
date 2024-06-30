
use crate::state::{Counter,  RoleConfig,  TheProject, TheRole};
use crate::datastates::{ DataConfig,  TheData,  };

use crate::error::Errors::ArithmeticError;
use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::instruction:: Instruction;

use solana_program::program:: invoke_signed;

use solana_program::{
  account_info::{next_account_info, AccountInfo},
  entrypoint::ProgramResult, 
  pubkey::Pubkey, 
     system_instruction,program::invoke,
   rent::Rent
};

use spl_associated_token_account::instruction::create_associated_token_account;

use spl_token_2022::extension::metadata_pointer::instruction::initialize;

  pub fn  initialize_project_without_token(
    accounts: &[AccountInfo],
    program_id:&Pubkey,
    data: TheData
  ) -> ProgramResult {

  let accounts_iter: &mut std::slice::Iter<'_, AccountInfo<'_>> = &mut accounts.iter();

  let creator: &AccountInfo<'_> = next_account_info(accounts_iter)?;
  let counter_account: &AccountInfo<'_> = next_account_info(accounts_iter)?;
  let project_account: &AccountInfo<'_> = next_account_info(accounts_iter)?;
  let data_account: &AccountInfo<'_> = next_account_info(accounts_iter)?;
  let role_account: &AccountInfo<'_> = next_account_info(accounts_iter)?;

  if !creator.is_signer{panic!()}

  let mut counter:Counter = Counter::try_from_slice(&counter_account.data.borrow())?;

  let project_no: u64 = counter.counter.checked_add(1).ok_or(ArithmeticError)?;

  let counter_str: String = project_no.to_string();
  let seed: &[u8] = counter_str.as_bytes();

  let (project_account_address, bump) = Pubkey::find_program_address(&[&seed], program_id);

  let rent: Rent = Rent::default();
  let rent_amount: u64 = rent.minimum_balance(80);


  invoke_signed(&system_instruction::create_account(
    creator.key,
    &project_account_address,
    rent_amount, 
    80, 
    program_id), 
    &[creator.clone(),project_account.clone()], 
    &[&[&seed, &[bump]]],
   )?;

   create_initial_data_account(creator, data_account, program_id, project_no, data)?;

   create_creator_role(creator, role_account, program_id, project_no)?;


  let project: TheProject = TheProject{
    project_no: project_no,
    initializer: creator.key.to_bytes(),
    token_mint: [0;32],
  };

  counter.counter = project_no;


  project.serialize(&mut &mut project_account.data.borrow_mut()[..])?;
  counter.serialize(&mut &mut counter_account.data.borrow_mut()[..])?;

    Ok(())
  }

  pub fn  initialize_project_with_token(
    accounts: &[AccountInfo],
    program_id:&Pubkey,
    data: TheData
  ) -> ProgramResult {

  let accounts_iter: &mut std::slice::Iter<'_, AccountInfo<'_>> = &mut accounts.iter();

  let creator: &AccountInfo<'_> = next_account_info(accounts_iter)?;
  let counter_account: &AccountInfo<'_> = next_account_info(accounts_iter)?;
  let project_account: &AccountInfo<'_> = next_account_info(accounts_iter)?;
  let project_account_ata: &AccountInfo<'_> = next_account_info(accounts_iter)?;
  let data_account: &AccountInfo<'_> = next_account_info(accounts_iter)?;
  let role_account: &AccountInfo<'_> = next_account_info(accounts_iter)?;
  let token_mint: &AccountInfo<'_> = next_account_info(accounts_iter)?;
  let token_2022_program: &AccountInfo<'_> = next_account_info(accounts_iter)?;
  let sysvar: &AccountInfo<'_> = next_account_info(accounts_iter)?;


  if !creator.is_signer{panic!()}

  let mut counter:Counter = Counter::try_from_slice(&counter_account.data.borrow())?;

  let project_no: u64 = counter.counter.checked_add(1).ok_or(ArithmeticError)?;

  let counter_str: String = project_no.to_string();
  let seed: &[u8] = counter_str.as_bytes();

  let (project_account_address, bump) = Pubkey::find_program_address(&[&seed], program_id);

  let rent: Rent = Rent::default();
  let project_account_rent: u64 = rent.minimum_balance(80);
  let mint_account_rent: u64 = rent.minimum_balance(82);


  invoke_signed(&system_instruction::create_account(
    creator.key,
    &project_account_address,
    project_account_rent, 
    80, 
    program_id),
    &[creator.clone(),project_account.clone()],&[&[&seed, &[bump]]],)?;

  let ix: Instruction = system_instruction::create_account(&creator.key,&token_mint.key,mint_account_rent,82,&token_2022_program.key);
  let init_metadata_pointer: Instruction = initialize(token_2022_program.key,token_mint.key,Some(*project_account.key),Some(*token_mint.key),)?;
  let init_mint: Instruction = spl_token_2022::instruction::initialize_mint(token_2022_program.key,token_mint.key,project_account.key,Some(project_account.key),0)?;
  let create_ata: solana_program::instruction::Instruction = create_associated_token_account(creator.key,project_account.key,token_mint.key,token_2022_program.key);

  invoke(&ix,  &[creator.clone(),token_mint.clone(),token_2022_program.clone(),])?;
  invoke(&init_metadata_pointer,  &[creator.clone(),project_account.clone(),token_mint.clone(),token_2022_program.clone(),sysvar.clone()])?;
  invoke(&init_mint,  &[creator.clone(),project_account.clone(),token_mint.clone(),token_2022_program.clone(),sysvar.clone()])?;
  invoke(&create_ata,&[creator.clone(),project_account_ata.clone(),project_account.clone(),token_mint.clone(),token_2022_program.clone(),sysvar.clone()])?;

  create_initial_data_account(creator, data_account, program_id, project_no, data)?;

  create_creator_role(creator, role_account, program_id, project_no)?;

  let project: TheProject = TheProject{
    project_no: project_no,
    initializer: creator.key.to_bytes(),
    token_mint: token_mint.key.to_bytes(),
  };

  counter.counter=project_no;

  counter.serialize(&mut &mut counter_account.data.borrow_mut()[..])?;
  project.serialize(&mut &mut project_account.data.borrow_mut()[..])?;

    Ok(())
  }

  pub fn  add_token_to_project(
    accounts: &[AccountInfo],
    program_id:&Pubkey,
  ) -> ProgramResult {

  let accounts_iter: &mut std::slice::Iter<'_, AccountInfo<'_>> = &mut accounts.iter();

  let initializer: &AccountInfo<'_> = next_account_info(accounts_iter)?;
  let project_account: &AccountInfo<'_> = next_account_info(accounts_iter)?;
  let token_mint: &AccountInfo<'_> = next_account_info(accounts_iter)?;


  if !initializer.is_signer{panic!()}
  if project_account.owner != program_id{panic!()}

  let project: TheProject  = TheProject::try_from_slice(&project_account.data.borrow())?;


  let project: TheProject = TheProject{
    project_no: project.project_no,
    initializer: project.initializer,
    token_mint: token_mint.key.to_bytes(),
  };


  project.serialize(&mut &mut project_account.data.borrow_mut()[..])?;

    Ok(())
  }

  pub fn  init_data_config(
    accounts: &[AccountInfo],
    program_id:&Pubkey,
    data:DataConfig
  ) -> ProgramResult {

  let accounts_iter: &mut std::slice::Iter<'_, AccountInfo<'_>> = &mut accounts.iter();

  let initializer: &AccountInfo<'_> = next_account_info(accounts_iter)?;
  let project_account: &AccountInfo<'_> = next_account_info(accounts_iter)?;
  let data_config_account: &AccountInfo<'_> = next_account_info(accounts_iter)?;


  if !initializer.is_signer{panic!()}
  if project_account.owner != program_id{panic!()}

  let project: TheProject  = TheProject::try_from_slice(&project_account.data.borrow())?;

  let initializer_address: Pubkey = Pubkey::new_from_array(project.initializer);

  if initializer.key != &initializer_address {panic!()}

  let mut seed_str:String = String::new();
  let project_no_str: String = project.project_no.to_string();
  let data_hierarchy_str: String = data.hierachy_in_the_tree.to_string();
  seed_str += &project_no_str;
  seed_str += &String::from("dac");
  seed_str += &data_hierarchy_str;

  let seed: &[u8] = seed_str.as_bytes();

  let mut temp_slice: Vec<u8> = [].to_vec();

  data.serialize(&mut &mut temp_slice[..]).unwrap();

  let (data_config_account_address, bump) = Pubkey::find_program_address(&[&seed], program_id);

  let rent: Rent = Rent::default();
  let rent_amount: u64 = rent.minimum_balance(temp_slice.len());


  invoke_signed(&system_instruction::create_account(
    initializer.key,
    &data_config_account_address,
    rent_amount, 
    temp_slice.len().try_into().unwrap(), 
    program_id), 
    &[initializer.clone(),data_config_account.clone()], 
    &[&[&seed, &[bump]]],
   )?;



  data.serialize(&mut &mut data_config_account.data.borrow_mut()[..])?;

    Ok(())
  }

  pub fn  init_role_config(
    accounts: &[AccountInfo],
    program_id:&Pubkey,
    data:RoleConfig
  ) -> ProgramResult {

  let accounts_iter: &mut std::slice::Iter<'_, AccountInfo<'_>> = &mut accounts.iter();

  let initializer: &AccountInfo<'_> = next_account_info(accounts_iter)?;
  let project_account: &AccountInfo<'_> = next_account_info(accounts_iter)?;
  let role_config_account: &AccountInfo<'_> = next_account_info(accounts_iter)?;


  if !initializer.is_signer{panic!()}
  if project_account.owner != program_id{panic!()}

  let project: TheProject  = TheProject::try_from_slice(&project_account.data.borrow())?;

  let initializer_address: Pubkey = Pubkey::new_from_array(project.initializer);

  if initializer.key != &initializer_address {panic!()}

  let mut seed_str:String = String::new();
  let project_no_str: String = project.project_no.to_string();
  let data_hierarchy_str: String = data.hierachy_in_the_roles.to_string();
  seed_str += &project_no_str;
  seed_str += &String::from("roc");
  seed_str += &data_hierarchy_str;

  let seed: &[u8] = seed_str.as_bytes();

  let mut temp_slice: Vec<u8> = [].to_vec();

  data.serialize(&mut &mut temp_slice[..]).unwrap();

  let (role_config_account_address, bump) = Pubkey::find_program_address(&[&seed], program_id);

  let rent: Rent = Rent::default();
  let rent_amount: u64 = rent.minimum_balance(temp_slice.len());


  invoke_signed(&system_instruction::create_account(
    initializer.key,
    &role_config_account_address,
    rent_amount, 
    temp_slice.len().try_into().unwrap(), 
    program_id), 
    &[initializer.clone(),role_config_account.clone()], 
    &[&[&seed, &[bump]]],
   )?;



  data.serialize(&mut &mut role_config_account.data.borrow_mut()[..])?;

    Ok(())
  }


fn create_initial_data_account<'info> (
  creator:&AccountInfo<'info>,
  data_account:&AccountInfo<'info>,
  program_id:&Pubkey,
  project_no:u64,
  data:TheData
) -> ProgramResult {

  let seed = get_seed();

  let (data_account_address, bump) = Pubkey::find_program_address(&[&seed.as_bytes()], program_id);

  let the_data: TheData = TheData{
   creator: creator.key.to_bytes(),
   project_no: project_no,
   hierachy_in_the_tree:1,
   parent_no: 0,
   data_no: 1,
   data_version: 1,
   last_time_data_added: 0,
   last_modified_on: 0,
   number_of_branches: 0,
   number_of_total_proposed_data: 0,
   bump: bump,
   data: data.data,
   fields: Vec::new()
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

  the_data.serialize(&mut &mut data_account.data.borrow_mut()[..])?;

  Ok(())
}

fn create_creator_role<'info>(
  creator:&AccountInfo<'info>,
  role_account:&AccountInfo<'info>,
  program_id:&Pubkey,
  project_no:u64,
) -> ProgramResult{


  let the_role: TheRole = TheRole{
    project_no:project_no,
    hierachy_in_the_roles:1,
    user:creator.key.to_bytes(),
    created_on:0,
    approved_to_create_data:0,
    approved_to_modify_data:0,
    approved_to_delete_data:0,
    last_time_created_data:0,
    last_time_modified_data:0,
    data_created:Vec::new(),
    data_modified:Vec::new(),
    data_proposed_to_create:Vec::new(),
    data_proposed_to_modify:Vec::new(),
    number_of_orders_executed: Vec::new(),
  };

   let mut temp_slice: Vec<u8> =  Vec::new();

   the_role.serialize(&mut &mut temp_slice[..]).unwrap();   

   let rent: Rent = Rent::default();
   let lamports: u64 = rent.minimum_balance(temp_slice.len());
   let space: u64 = temp_slice.len().try_into().unwrap();

   let mut seed_str:String = String::new();

   seed_str += &project_no.to_string();
   seed_str += &String::from("role");
   seed_str += &String::from("1");

 let (role_account_address, bump) = Pubkey::find_program_address(&[&seed_str.as_bytes(),&creator.key.to_bytes()], program_id);

  invoke_signed(&system_instruction::create_account(
    creator.key,
    &role_account_address,
    lamports, 
    space, 
    program_id), 
    &[creator.clone(),role_account.clone()], 
    &[&[seed_str.as_bytes(),&creator.key.to_bytes(),&[bump]]]
   )?;

  the_role.serialize(&mut &mut role_account.data.borrow_mut()[..])?;

  Ok(())
}


fn get_seed()-> String {

  let project_no:u64 = 1;
  let data_hierarchy_in_the_tree:u8 = 1;
  let parent_no:u64 = 0;
  let data_no:u64 = 1;
  let verison_no:u64 = 1;

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

