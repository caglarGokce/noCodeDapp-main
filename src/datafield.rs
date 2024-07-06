use crate::error::Errors::ArithmeticError;
use crate::rolestates::{   RoleConfig,   TheRole};
use crate::datastates::{ DataConfig,  TheData,  ExecutionData, TheOrder};

use crate::enums::{TheOperation,TheConditions,TheSubjects,ExeOrder};
use borsh::{BorshDeserialize, BorshSerialize};


use solana_program::{
 account_info::{next_account_info, AccountInfo},
 entrypoint::ProgramResult, 
 pubkey::Pubkey, sysvar::{clock::Clock, Sysvar,},

};



  pub fn  execute_order(
    accounts: &[AccountInfo],
    program_id:&Pubkey,
    execution_data:ExecutionData
  ) -> ProgramResult {

  let accounts_iter: &mut std::slice::Iter<'_, AccountInfo<'_>> = &mut accounts.iter();

  let executor: &AccountInfo<'_> = next_account_info(accounts_iter)?;
  let data_config_account: &AccountInfo<'_> = next_account_info(accounts_iter)?;
  let data_account: &AccountInfo<'_> = next_account_info(accounts_iter)?;
  let executor_role_account: &AccountInfo<'_> = next_account_info(accounts_iter)?;
  let executor_role_config_account: &AccountInfo<'_> = next_account_info(accounts_iter)?;

  if data_config_account.owner != program_id {panic!()}
  if !executor.is_signer{panic!()}


  let data_config: DataConfig = DataConfig::try_from_slice(&data_config_account.data.borrow())?;
  let mut the_data: TheData = TheData::try_from_slice(&data_account.data.borrow())?;


  let execution_order: &TheOrder = &data_config.orders[execution_data.order_no as usize];
  let total_number_of_executions: &u64 = &the_data.total_number_of_executions[execution_data.order_no as usize];
  let max_number_of_order_execution: &u64 = &data_config.max_number_of_order_execution[execution_data.order_no as usize];

  if total_number_of_executions >= max_number_of_order_execution{panic!()}

  let clock: Clock = Clock::get()?;
  let current_time: u64 = clock.unix_timestamp as u64;

  if data_config.who_can_execute_orders[execution_data.order_no as usize].len() != 0{

    let the_role: TheRole = TheRole::try_from_slice(&executor_role_account.data.borrow())?;

    if data_config.who_can_execute_orders[execution_data.order_no as usize].contains(&the_role.hierachy_in_the_roles){
      execute_with_role(executor_role_account, executor_role_config_account, &data_config, &current_time, execution_data.order_no)?;
    }else{
      panic!()
    }

  }


  let mut outputs:Vec<u64>=Vec::new();
  let  constants:Vec<u64>=data_config.constanst;
  let  inputs:Vec<u64> = execution_data.inputs;
  let mut fields: Vec<u64> = the_data.fields;


  let orders:Vec<ExeOrder> = process_order(execution_order);


  for order in orders {

    match order{
      ExeOrder::WithCondition {  condition } => {
        match condition {
            TheConditions::IsEqual { compare_1, compare_2, operation } => {
              let (the_result ,result_subject) = get_the_result(&outputs, &inputs, &constants, &fields, operation);
         
              let compare_value_1 = get_subject_value(&outputs, &inputs, &constants, &fields, compare_1);
              let compare_value_2 = get_subject_value(&outputs, &inputs, &constants, &fields, compare_2);

                if compare_value_1 == compare_value_2{
                  match result_subject {
                    TheSubjects::Outputs { no:_  } => {
                        outputs.push(the_result)
                    },
                    TheSubjects::Inputs { no: _ } => {
                      //not applied
                    },
                    TheSubjects::Constants { no: _ } => {
                      //not applied
                    },
                    TheSubjects::Fields { no } => {
                      fields[no as usize] = the_result;
                    },
                  }
                }
            },
            TheConditions::IsEqualNot { compare_1, compare_2, operation } => {
              let (the_result ,result_subject) = get_the_result(&outputs, &inputs, &constants, &fields, operation);
         
              let compare_value_1 = get_subject_value(&outputs, &inputs, &constants, &fields, compare_1);
              let compare_value_2 = get_subject_value(&outputs, &inputs, &constants, &fields, compare_2);

                if compare_value_1 != compare_value_2{
                  match result_subject {
                    TheSubjects::Outputs { no:_  } => {
                        outputs.push(the_result)
                    },
                    TheSubjects::Inputs { no: _ } => {
                      //not applied
                    },
                    TheSubjects::Constants { no: _ } => {
                      //not applied
                    },
                    TheSubjects::Fields { no } => {
                      fields[no as usize] = the_result;
                    },
                  }
                }
            },
            TheConditions::IsBigger { compare_1, compare_2, operation } => {
              let (the_result ,result_subject) = get_the_result(&outputs, &inputs, &constants, &fields, operation);
         
              let compare_value_1 = get_subject_value(&outputs, &inputs, &constants, &fields, compare_1);
              let compare_value_2 = get_subject_value(&outputs, &inputs, &constants, &fields, compare_2);

                if compare_value_1 > compare_value_2{
                  match result_subject {
                    TheSubjects::Outputs { no:_  } => {
                        outputs.push(the_result)
                    },
                    TheSubjects::Inputs { no: _ } => {
                      //not applied
                    },
                    TheSubjects::Constants { no: _ } => {
                      //not applied
                    },
                    TheSubjects::Fields { no } => {
                      fields[no as usize] = the_result;
                    },
                  }
                }
            },
            TheConditions::IsSmaller { compare_1, compare_2, operation } => {
              let (the_result ,result_subject) = get_the_result(&outputs, &inputs, &constants, &fields, operation);
         
              let compare_value_1 = get_subject_value(&outputs, &inputs, &constants, &fields, compare_1);
              let compare_value_2 = get_subject_value(&outputs, &inputs, &constants, &fields, compare_2);

                if compare_value_1 < compare_value_2{
                  match result_subject {
                    TheSubjects::Outputs { no:_  } => {
                        outputs.push(the_result)
                    },
                    TheSubjects::Inputs { no: _ } => {
                      //not applied
                    },
                    TheSubjects::Constants { no: _ } => {
                      //not applied
                    },
                    TheSubjects::Fields { no } => {
                      fields[no as usize] = the_result;
                    },
                  }
                }
            },
            TheConditions::IsBiggerEqual { compare_1, compare_2, operation } => {
              let (the_result ,result_subject) = get_the_result(&outputs, &inputs, &constants, &fields, operation);
         
              let compare_value_1 = get_subject_value(&outputs, &inputs, &constants, &fields, compare_1);
              let compare_value_2 = get_subject_value(&outputs, &inputs, &constants, &fields, compare_2);

                if compare_value_1 >= compare_value_2{
                  match result_subject {
                    TheSubjects::Outputs { no:_  } => {
                        outputs.push(the_result)
                    },
                    TheSubjects::Inputs { no: _ } => {
                      //not applied
                    },
                    TheSubjects::Constants { no: _ } => {
                      //not applied
                    },
                    TheSubjects::Fields { no } => {
                      fields[no as usize] = the_result;
                    },
                  }
                }
            },
            TheConditions::IsSmallerEqual { compare_1, compare_2, operation } => {
              let (the_result ,result_subject) = get_the_result(&outputs, &inputs, &constants, &fields, operation);
         
              let compare_value_1 = get_subject_value(&outputs, &inputs, &constants, &fields, compare_1);
              let compare_value_2 = get_subject_value(&outputs, &inputs, &constants, &fields, compare_2);

                if compare_value_1 <= compare_value_2{
                  match result_subject {
                    TheSubjects::Outputs { no:_ } => {
                        outputs.push(the_result)
                    },
                    TheSubjects::Inputs { no: _ } => {
                      //not applied
                    },
                    TheSubjects::Constants { no: _ } => {
                      //not applied
                    },
                    TheSubjects::Fields { no } => {
                      fields[no as usize] = the_result;
                    },
                  }
                }
            },

        }
      }
      ExeOrder::WithoutCondition { operation  } => {

        let (the_result ,result_subject) = get_the_result(&outputs, &inputs, &constants, &fields, operation);
         
        match result_subject {
          TheSubjects::Outputs { no:_ } => {
              outputs.push(the_result)
          },
          TheSubjects::Inputs { no: _ } => {
            //not applied
          },
          TheSubjects::Constants { no: _ } => {
            //not applied
          },
          TheSubjects::Fields { no } => {
            fields[no as usize] = the_result;
          },
        }

      }
    }

  }

  the_data.fields = fields;

  the_data.serialize(&mut &mut data_account.data.borrow_mut()[..])?;

    Ok(())
  }

  fn create_order_with_condition(
    condition_code:u16,
    compare_1_code:u16,
    compare_2_code:u16,
    result_code:u16,
    subject_1_code:u16,
    subject_2_code:u16,
    operation_code:u16,

  )-> ExeOrder{

    let compare_1: TheSubjects = get_subject(compare_1_code);
    let compare_2: TheSubjects = get_subject(compare_2_code);
    let result: TheSubjects = get_subject(result_code);
    let subject_1: TheSubjects = get_subject(subject_1_code);
    let subject_2: TheSubjects = get_subject(subject_2_code);
   
    let operation: TheOperation = get_operation(operation_code, subject_1, subject_2, result);

    let condition:TheConditions = get_condition(condition_code,compare_1,compare_2,operation.clone());


    let order: ExeOrder = ExeOrder::WithCondition {  condition };

    return  order;
  }

  fn create_order_without_condition(
    result:u16,
    subject_1:u16,
    subject_2:u16,
    operation:u16,

  ) -> ExeOrder {

    let result_value: TheSubjects = get_subject(result);
    let subject_1_value: TheSubjects = get_subject(subject_1);
    let subject_2_value: TheSubjects = get_subject(subject_2);

    let operation_value: TheOperation = get_operation(operation, subject_1_value, subject_2_value, result_value);



    let order: ExeOrder = ExeOrder::WithoutCondition { operation: operation_value };

    return order;
  }

  fn get_subject(
    subject:u16,
  ) -> TheSubjects {

    let the_subject: TheSubjects;

    if subject >= 1000 && subject < 2000 {
      let no = subject/1000;
      the_subject = TheSubjects::Outputs { no };
      
    }else if subject >= 2000 && subject < 3000{
      let no = subject/1000;
      the_subject = TheSubjects::Inputs { no };

    }else if subject >= 3000 && subject < 4000{
      let no = subject/1000;
      the_subject = TheSubjects::Constants { no };
      
    }else {
      let no = subject/1000;
      the_subject = TheSubjects::Fields { no };
      
    }

    return the_subject;
  }

  fn get_operation(
    operation:u16,
    subject_1:TheSubjects,
    subject_2:TheSubjects,
    result:TheSubjects,
  ) -> TheOperation {

    let the_operation: TheOperation;

    if operation == 1 {
      the_operation = TheOperation::Addition { subject_1,subject_2,result };
      
    }else if operation == 2 {
      the_operation = TheOperation::Substraction { subject_1,subject_2,result  };

    }else if operation == 3 {
      the_operation = TheOperation::Multiplication { subject_1,subject_2,result  };
      
    }else {
      the_operation = TheOperation::Division { subject_1,subject_2,result  };
      
    }

    return the_operation;
  }

  fn get_condition(
    condition_code:u16,
    compare_1:TheSubjects,
    compare_2:TheSubjects,
    operation:TheOperation,
  ) -> TheConditions {

    let the_condition: TheConditions;

    if condition_code == 1 {
      the_condition = TheConditions::IsEqual {compare_1,compare_2,operation };


    }else if condition_code == 2 {
      the_condition = TheConditions::IsEqualNot { compare_1,compare_2,operation };

    }else if condition_code == 3 {
      the_condition = TheConditions::IsBigger {  compare_1,compare_2,operation };
      
    }else if condition_code == 4 {
      the_condition = TheConditions::IsSmaller { compare_1,compare_2,operation  };
      
    }else if condition_code == 5 {
      the_condition = TheConditions::IsBiggerEqual { compare_1,compare_2,operation  };
      
    }else {
      the_condition = TheConditions::IsSmallerEqual { compare_1,compare_2,operation };
      
    }

    return the_condition;
  }

  fn process_order(
    execution_order:&TheOrder
  ) -> Vec<ExeOrder> {

  let mut director:u16 = 0;
  let mut orders:Vec<ExeOrder> = Vec::new();

    let order_len = execution_order.order.len() as u16;

    loop {

      if execution_order.order[director as usize] < 11 {
  
        let order: ExeOrder = create_order_with_condition(
          execution_order.order[director as usize],execution_order.order[(director+1) as usize],
          execution_order.order[(director+2) as usize], execution_order.order[(director+3) as usize],
          execution_order.order[(director+4) as usize], execution_order.order[(director+5) as usize],
          execution_order.order[(director+6) as usize],
        );
  
        orders.push(order);
        director += 6;
  
      }else{
        
        let order: ExeOrder = create_order_without_condition(
        execution_order.order[director as usize],execution_order.order[(director+1) as usize],
        execution_order.order[(director+2) as usize],execution_order.order[(director+3) as usize],);
        
        orders.push(order);
        director += 3;
      
      }
  
      if director == order_len  {
          break; 
      }
  
    }

    return orders;
  }


fn get_subject_value(
  outputs:&Vec<u64>,
  inputs:&Vec<u64>,
  constants:&Vec<u64>,
  fields:&Vec<u64>,
  subject:TheSubjects
) -> u64 {

  let compare_value: u64;

  match subject {
    TheSubjects::Outputs { no } => {
        let index = no as usize;
        compare_value =  outputs[index]
    },
    TheSubjects::Inputs { no } => {
        let index = no as usize;
        compare_value = inputs[index]
    },
    TheSubjects::Constants { no } => {
        let index = no as usize;
        compare_value =  constants[index]
    },
    TheSubjects::Fields { no } => {
        let index = no as usize;
        compare_value  = fields[index]
        
    },
  }

  return compare_value;
}

fn get_the_result(
  outputs:&Vec<u64>,
  inputs:&Vec<u64>,
  constants:&Vec<u64>,
  fields:&Vec<u64>,
  operation:TheOperation
) -> (u64,TheSubjects) {

  let the_result: u64;
  let result_subject: TheSubjects;

  match operation {
    TheOperation::Addition { subject_1, subject_2, result } => {
      let subject_value_1: u64 = get_subject_value(&outputs, &inputs, &constants, &fields, subject_1);
      let subject_value_2: u64 = get_subject_value(&outputs, &inputs, &constants, &fields, subject_2);
      the_result = subject_value_1+subject_value_2;
      result_subject = result;
      
    },
    TheOperation::Substraction { subject_1, subject_2, result } => {
      let subject_value_1: u64 = get_subject_value(&outputs, &inputs, &constants, &fields, subject_1);
      let subject_value_2: u64 = get_subject_value(&outputs, &inputs, &constants, &fields, subject_2);
      the_result = subject_value_1-subject_value_2;
      result_subject = result;
      
    },
    TheOperation::Multiplication { subject_1, subject_2, result } => {
      let subject_value_1: u64 = get_subject_value(&outputs, &inputs, &constants, &fields, subject_1);
      let subject_value_2: u64 = get_subject_value(&outputs, &inputs, &constants, &fields, subject_2);
      the_result = subject_value_1*subject_value_2;
      result_subject = result;
      
    },
    TheOperation::Division { subject_1, subject_2, result } => {
      let subject_value_1: u64 = get_subject_value(&outputs, &inputs, &constants, &fields, subject_1);
      let subject_value_2: u64 = get_subject_value(&outputs, &inputs, &constants, &fields, subject_2);
      the_result = subject_value_1/subject_value_2;
      result_subject = result;
      
    },
              }

  return (the_result,result_subject);
}

fn execute_with_role(
  role_account:&AccountInfo,
  role_config_account:&AccountInfo,
  data_config:&DataConfig,
  current_time:&u64,
  order_no:u8
) -> ProgramResult {

  let mut the_role: TheRole = TheRole::try_from_slice(&role_account.data.borrow())?;
  let role_config: RoleConfig = RoleConfig::try_from_slice(&role_config_account.data.borrow())?;

  let execution_limit: u64 = role_config.number_of_limit_to_execute_orders
  [data_config.hierachy_in_the_tree as usize]
  [order_no as usize] ;

    if execution_limit != 0{
    

      if execution_limit <= the_role.number_of_orders_executed[data_config.hierachy_in_the_tree as usize][order_no as usize]{panic!()}

      the_role.number_of_orders_executed[data_config.hierachy_in_the_tree as usize][order_no as usize]
      = the_role.number_of_orders_executed[data_config.hierachy_in_the_tree as usize][order_no as usize].checked_add(1).ok_or(ArithmeticError)?;

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
