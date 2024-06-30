
use crate::state::{  RoleApplication, RoleConfig, TheRole};
use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::clock::Clock;



use solana_program::{
  account_info::{next_account_info, AccountInfo},
  entrypoint::ProgramResult, 
  pubkey::Pubkey, 
     system_instruction,program::invoke_signed,
   rent::Rent,sysvar::Sysvar
};


pub fn  create_assign_or_apply_for_a_role(
    accounts: &[AccountInfo],
    program_id:&Pubkey,
  ) -> ProgramResult {

  let accounts_iter: &mut std::slice::Iter<'_, AccountInfo<'_>> = &mut accounts.iter();


  let role_creator: &AccountInfo<'_> = next_account_info(accounts_iter)?;
  let creator_role_account: &AccountInfo<'_> = next_account_info(accounts_iter)?;
  let role_config_account: &AccountInfo<'_> = next_account_info(accounts_iter)?;
  let role_account: &AccountInfo<'_> = next_account_info(accounts_iter)?;
  let role_application_account: &AccountInfo<'_> = next_account_info(accounts_iter)?;
  let role_owner: &AccountInfo<'_> = next_account_info(accounts_iter)?;

  if role_config_account.owner != program_id {panic!()}
  if !role_creator.is_signer {panic!()}


  let role_config: RoleConfig = RoleConfig::try_from_slice(&role_config_account.data.borrow())?;

  if role_config.who_can_assign_this_role.len() != 0 {

  if role_config.who_can_assign_this_role.contains(&role_config.hierachy_in_the_roles){
    assign_a_role(role_creator, creator_role_account, role_account, role_owner.key, program_id, &role_config)?;
    }else{
        apply_for_a_role(role_creator,role_application_account,program_id,&role_config)?;
    }

  }else{
    create_a_role(role_creator, role_account, program_id, &role_config)?;
  }


    Ok(())
  }



fn apply_for_a_role<'info>(
    role_creator:&AccountInfo<'info>,
    role_application_account:&AccountInfo<'info>,
    program_id:&Pubkey,
    role_config:&RoleConfig) -> ProgramResult{
  
  
    let role_application: RoleApplication = RoleApplication{
      project_no:role_config.project_no,
      hierachy_in_the_roles:role_config.hierachy_in_the_roles,
      user:role_creator.key.to_bytes(),

    };
  
     let mut temp_slice: Vec<u8> =  Vec::new();
  
     role_application.serialize(&mut &mut temp_slice[..]).unwrap();   
  
     let rent: Rent = Rent::default();
     let lamports: u64 = rent.minimum_balance(temp_slice.len());
     let space: u64 = temp_slice.len().try_into().unwrap();
  
     let mut seed_str:String = String::new();
  
     seed_str += &String::from("app");
     seed_str += &role_config.project_no.to_string();
     seed_str += &String::from("role");
     seed_str += &role_config.hierachy_in_the_roles.to_string();
  
   let (role_application_account_address, bump) = Pubkey::find_program_address(&[&seed_str.as_bytes()], program_id);
  
    invoke_signed(&system_instruction::create_account(
      role_creator.key,
      &role_application_account_address,
      lamports, 
      space, 
      program_id), 
      &[role_creator.clone(),role_application_account.clone()], 
      &[&[seed_str.as_bytes(),&[bump]]]
     )?;
  
    role_application.serialize(&mut &mut role_application_account.data.borrow_mut()[..])?;

    Ok(())
}

fn create_a_role<'info>(
    role_creator:&AccountInfo<'info>,
    role_account:&AccountInfo<'info>,
    program_id:&Pubkey,
    role_config:&RoleConfig
) -> ProgramResult{

    let clock: Clock = Clock::get()?;
    let current_time: u64 = clock.unix_timestamp as u64;
  
  
    let the_role: TheRole = TheRole{
      project_no:role_config.project_no,
      hierachy_in_the_roles:role_config.hierachy_in_the_roles,
      user:role_creator.key.to_bytes(),
      created_on:current_time,
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
  
     seed_str += &role_config.project_no.to_string();
     seed_str += &String::from("role");
     seed_str += &role_config.hierachy_in_the_roles.to_string();
  
   let (role_account_address, bump) = Pubkey::find_program_address(&[&seed_str.as_bytes(),&role_creator.key.to_bytes()], program_id);
  
    invoke_signed(&system_instruction::create_account(
      role_creator.key,
      &role_account_address,
      lamports, 
      space, 
      program_id), 
      &[role_creator.clone(),role_account.clone()], 
      &[&[seed_str.as_bytes(),&role_creator.key.to_bytes(),&[bump]]]
     )?;
  
    the_role.serialize(&mut &mut role_account.data.borrow_mut()[..])?;

    Ok(())
}


fn assign_a_role<'info>(
    role_creator:&AccountInfo<'info>,
    creator_role_account:&AccountInfo,
    role_account:&AccountInfo<'info>,
    role_owner:&Pubkey,
    program_id:&Pubkey,
    role_config:&RoleConfig
) -> ProgramResult{

    is_assigner_valid(role_creator, creator_role_account, program_id, role_config)?;

    let clock: Clock = Clock::get()?;
    let current_time: u64 = clock.unix_timestamp as u64;
  
  
    let the_role: TheRole = TheRole{
      project_no:role_config.project_no,
      hierachy_in_the_roles:role_config.hierachy_in_the_roles,
      user:role_owner.to_bytes(),
      created_on:current_time,
      approved_to_create_data:0,
      approved_to_modify_data:0,
      approved_to_delete_data:0,
      last_time_created_data:0,
      last_time_modified_data:0,
      data_created:Vec::new(),
      data_modified: Vec::new(),
      data_proposed_to_create: Vec::new(),
      data_proposed_to_modify: Vec::new(),
      number_of_orders_executed: Vec::new(),
    };
  
     let mut temp_slice: Vec<u8> =  Vec::new();
  
     the_role.serialize(&mut &mut temp_slice[..]).unwrap();   
  
     let rent: Rent = Rent::default();
     let lamports: u64 = rent.minimum_balance(temp_slice.len());
     let space: u64 = temp_slice.len().try_into().unwrap();
  
     let mut seed_str:String = String::new();
  
     seed_str += &role_config.project_no.to_string();
     seed_str += &String::from("role");
     seed_str += &role_config.hierachy_in_the_roles.to_string();
  
   let (role_account_address, bump) = Pubkey::find_program_address(&[&seed_str.as_bytes(),&role_owner.to_bytes()], program_id);
  
    invoke_signed(&system_instruction::create_account(
      role_creator.key,
      &role_account_address,
      lamports, 
      space, 
      program_id), 
      &[role_creator.clone(),role_account.clone()], 
      &[&[seed_str.as_bytes(),&role_owner.to_bytes(),&[bump]]]
     )?;
  
    the_role.serialize(&mut &mut role_account.data.borrow_mut()[..])?;

    Ok(())
}

fn is_assigner_valid(
    role_creator:&AccountInfo,
    creator_role_account:&AccountInfo,
    program_id:&Pubkey,
    role_config:&RoleConfig,
) -> ProgramResult{


  let creator_role: TheRole = TheRole::try_from_slice(&creator_role_account.data.borrow())?;

  let creator_from_bytes: Pubkey = Pubkey::new_from_array(creator_role.user);

  if &creator_from_bytes != role_creator.key {panic!()}

  if creator_role_account.owner != program_id{panic!()}

  if creator_role.project_no != role_config.project_no{panic!()}

  if !role_config.who_can_assign_this_role.contains(&creator_role.hierachy_in_the_roles){panic!()}

    Ok(())
}

fn enable_role()-> ProgramResult{

  Ok(())
}

fn disable_role()-> ProgramResult{

  Ok(())
}