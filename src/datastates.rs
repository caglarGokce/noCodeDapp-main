use borsh::{BorshDeserialize, BorshSerialize};


#[derive(BorshSerialize, BorshDeserialize)]
pub struct DataConfig{
    pub project_no:u64,
    pub bump:u8,
    pub hierachy_in_the_tree:u8,

    pub who_can_create:Vec<u8>,//if empty anybody can create
    pub is_approval_by_the_creator_required_to_create:u8,//if there is a role
    pub is_confirmation_by_the_creator_required_to_create:u8,
    pub how_frequent_data_can_be_created:u64, // if not zero applied
  //pub to_whom_frequincy_required_to_create_is_applied:Vec<u8>,
  //pub do_not_create_until_this_time_required:u8, 
  //pub do_not_create_until_this_time:u64, 
  //pub to_whom_do_not_create_until_is_applied:Vec<u8>,
  

    pub token_amount_needed_to_create:u64,// if not zero applied
    pub token_handled_after_creation:u8, // 1 - kept, 2 - sent, 3 - burnt

    pub who_can_modify:Vec<u8>,
    pub is_approval_by_the_creator_required_to_modify:u8,
    pub is_confirmation_by_the_creator_required_to_modify:u8,
    pub how_frequent_data_can_be_modified:u64,
  //pub to_whom_frequincy_required_to_modify_is_applied:Vec<u8>,

  //pub do_not_modify_until_this_time_required:u8, 
  //pub do_not_modify_until_this_time:u64, 
  //pub to_whom_do_not_modify_until_is_applied:Vec<u8>,
    pub token_amount_needed_to_modify:u64,
    pub token_handled_after_modification:u8, // 1 - kept, 2 - sent, 3 - burnt

    pub number_of_max_branches:u64,//if zero is not applied
    pub number_of_max_versions:u64,
    //limit versions & is configurable
    //limit approved roles creation number & is configurable
    //limit approved roles for this data & is configurable
    pub constanst:Vec<u64>,
    pub initial_field_values:Vec<u64>,

    pub orders:Vec<TheOrder>,

    // nested vector represents the roles. parent vector represents orders by index. 
    //u8 represents the role. if zero anybody can execute. 
    pub who_can_execute_orders:Vec<Vec<u8>>,

    //each index represents the order at corresponding index of orders:Vec<TheOrder>.
    //u64 represent the limit
    pub max_number_of_order_execution:Vec<u64>,


}


#[derive(BorshSerialize, BorshDeserialize,)]
pub struct TheData{
    pub creator:[u8;32],
    pub project_no:u64,
    pub hierachy_in_the_tree:u8,
    pub parent_no:u64,
    pub data_no:u64,
    pub data_version:u64,
    pub last_time_data_added:u64,
    pub last_modified_on:u64,
    pub number_of_branches:u64,
    pub number_of_total_proposed_data:u64,
    pub total_number_of_executions:Vec<u64>,
    pub bump:u8,
    pub data:String,
    pub fields:Vec<u64>,
}

#[derive(BorshSerialize, BorshDeserialize,)]
pub struct TheOrder{
    pub order:Vec<u16>,
}

#[derive(BorshSerialize, BorshDeserialize,)]
pub struct ExecutionData{
    pub inputs:Vec<u64>,
    pub order_no:u8
}

#[derive(BorshSerialize, BorshDeserialize,)]
pub struct DataStr{
    pub data:String,
    pub fields:Vec<u64>,
}
