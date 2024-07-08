use borsh::{BorshDeserialize, BorshSerialize};


#[derive(BorshSerialize, BorshDeserialize,)]
pub struct TheProject{
    pub project_no:u64,
    pub initializer:[u8;32],
    pub token_mint:[u8;32],
}


#[derive(BorshSerialize, BorshDeserialize,)]
pub struct RoleApplication{
    pub user:[u8;32],
    pub project_no:u64,
    pub hierachy_in_the_roles:u8,
}

#[derive(BorshSerialize, BorshDeserialize,)]
pub struct RoleConfig{
    pub project_no:u64,
    pub hierachy_in_the_roles:u8,

    pub who_can_assign_this_role:Vec<u8>,//if empty anyone can get this
    pub who_can_enable_this_role:Vec<u8>,//if empty noone can enable
    pub who_can_disable_this_role:Vec<u8>,//if empty noone can disable

    pub time_required_to_create:u8, 
    pub time_required_until_creation:u64,

    pub time_required_to_modify:u8,
    pub time_required_until_modification:u64,

    pub time_required_to_delete:u8,
    pub time_required_until_delete:u64,

    pub creation_limit_of_this_role_on_data:Vec<u8>,//vector represent on which data type(hierarchy_in_the_tree) this role has limit. if empty there is no limit
    pub creation_limit:Vec<u64>,//the limit of creation. index of each limit is related to indexes in creation_limit_over_data. They must be the same length

    pub modification_limit_of_this_role_on_data:Vec<u8>,
    pub modification_limit:Vec<u64>,

    pub proposal_for_creation_limit_of_this_role_on_data:Vec<u8>,
    pub proposal_for_creation_limit:Vec<u64>,

    pub proposal_for_modification_limit_of_this_role_on_data:Vec<u8>,
    pub proposal_for_modification_limit:Vec<u64>,

    //nested vector represents the order indexes in DataConfig - orders:Vec<TheOrder>. parent vector represents data types(hierarchy_in_the_tree). 
    //u64 represents allowed number of modification. if zero there is no limit. 
    //Prevention of modification by this role is configured in data config
    pub number_of_limit_to_execute_orders:Vec<Vec<u64>>, 

    pub bump:u8

    //limit number of roles 
    //limit number of creation, modification, deletion 
    //limit roles that can be approved 
    //limit data creation modification deletion proposals
    //time limited roles 
}



#[derive(BorshSerialize, BorshDeserialize, )]
pub struct TheRole{
    pub project_no:u64,
    pub hierachy_in_the_roles:u8,
    pub user:[u8;32],
    pub created_on:u64,
    pub approved_to_create_data:u8,
    pub approved_to_modify_data:u8,
    pub approved_to_delete_data:u8,
    pub last_time_created_data:u64,
    pub last_time_modified_data:u64,
    pub data_created:Vec<u64>,
    pub data_modified:Vec<u64>,
    pub data_proposed_to_create:Vec<u64>,
    pub data_proposed_to_modify:Vec<u64>,
    pub number_of_orders_executed:Vec<Vec<u64>>,
    pub is_enabled:u8
}


#[derive(BorshSerialize, BorshDeserialize,)]
pub struct Counter{
    pub counter:u64,
}
