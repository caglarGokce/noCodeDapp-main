use crate::error::Errors::ArithmeticError;
use crate::state::{   RoleConfig,   TheRole};
use crate::datastates::{ DataConfig,  TheData,  ExecutionData, TheOrder};

use crate::enums::{TheOperation,TheConditions,TheSubjects,ExeOrder};
use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::program::{invoke,  invoke_signed};

use solana_program::{
 account_info::{next_account_info, AccountInfo},
 entrypoint::ProgramResult, 
 pubkey::Pubkey, sysvar::{clock::Clock, Sysvar,},
    system_instruction,
  rent::Rent
};



  pub fn  execute_order(
    accounts: &[AccountInfo],
    program_id:&Pubkey,
    execution_data:ExecutionData
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
  let mut the_data: TheData = TheData::try_from_slice(&data_account.data.borrow())?;


  let execution_order: &TheOrder = &data_config.orders[execution_data.order_no as usize];

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
