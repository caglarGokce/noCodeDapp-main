
import { deserialize } from "borsh";


export const deserialize_role_account = async (
data:Buffer
) => {

    const the_data = deserialize(TheDataSchema,TheData,data);
  
    return the_data;

}

export const deserialize_data_account = (
    data:Buffer
) => {

    const role = deserialize(TheDataSchema,TheData,data);
  
    return role;
}

export const deserialize_data_config_account = (
    data:Buffer
) => {

    const config = deserialize(DataConfigSchema,DataConfig,data);
  
    return config;
}

export const deserialize_role_config_account = (
    data:Buffer
) => {

    const config = deserialize(RoleConfigSchema,RoleConfig,data);
  
    return config;
}

export const create_role = (
    project_no:bigint ,
    hierachy_in_the_roles:number,
    user:number[] ,
    created_on:bigint ,
    approved_to_create_data:number,
    approved_to_modify_data:number,
    approved_to_delete_data:number,
    last_time_created_data:bigint ,
    last_time_modified_data:bigint ,
    data_created:bigint[] ,
    data_modified:bigint[] ,
    data_proposed_to_create:bigint[] ,
    data_proposed_to_modify:bigint[] ,
    number_of_orders_executed:bigint[][] ,
    is_enabled:number,
) => {

    const the_role = new TheRole({
        project_no: project_no,
        hierachy_in_the_roles: hierachy_in_the_roles,
        user: user,
        created_on: created_on,
        approved_to_create_data: approved_to_create_data,
        approved_to_modify_data: approved_to_modify_data,
        approved_to_delete_data: approved_to_delete_data,
        last_time_created_data: last_time_created_data,
        last_time_modified_data: last_time_modified_data,
        data_created: data_created,
        data_modified: data_modified,
        data_proposed_to_create: data_proposed_to_create,
        data_proposed_to_modify: data_proposed_to_modify,
        number_of_orders_executed: number_of_orders_executed,
        is_enabled: is_enabled,
    })
}

export const create_data = (
    creator:number[],
    project_no:bigint,
    hierarchy_in_the_tree:number,
    parent_no:bigint,
    data_no:bigint,
    data_version:bigint,
    last_time_data_added:bigint,
    last_modified_on:bigint,
    number_of_branches:bigint,
    number_of_total_proposed_data:bigint,
    total_number_of_executions:bigint[],
    bump:number,
    data:string,
    fields:bigint[],
) => {

    const the_data = new TheData(
        {
            creator:creator,
            project_no:project_no,
            hierarchy_in_the_tree:hierarchy_in_the_tree,
            parent_no:parent_no,
            data_no:data_no,
            data_version:data_version,
            last_time_data_added:last_time_data_added,
            last_modified_on:last_modified_on,
            number_of_branches:number_of_branches,
            number_of_total_proposed_data:number_of_total_proposed_data,
            total_number_of_executions:total_number_of_executions,
            bump:bump,
            data:data,
            fields:fields,
        }
    )


}

export const create_role_config = (
    project_no:bigint ,
    hierachy_in_the_roles:number,
    who_can_assign_this_role:number[] ,
    who_can_enable_this_role:number[] ,
    who_can_disable_this_role:number[] ,
    time_required_to_create:number,
    time_required_until_creation:bigint ,
    time_required_to_modify:number,
    time_required_until_modification:bigint ,
    time_required_to_delete:number,
    time_required_until_delete:bigint ,
    creation_limit_of_this_role_on_data:number[] ,
    creation_limit:bigint[] ,
    modification_limit_of_this_role_on_data:number[] ,
    modification_limit:bigint[] ,
    proposal_for_creation_limit_of_this_role_on_data:number[] ,
    proposal_for_creation_limit:bigint[] ,
    proposal_for_modification_limit_of_this_role_on_data:number[] ,
    proposal_for_modification_limit:bigint[] ,
    number_of_limit_to_execute_orders:bigint[][] ,
    bump:number,
) => {

    const config = new RoleConfig({
        project_no:project_no,
        hierachy_in_the_roles:hierachy_in_the_roles,
        who_can_assign_this_role:who_can_assign_this_role,
        who_can_enable_this_role:who_can_enable_this_role,
        who_can_disable_this_role:who_can_disable_this_role,
        time_required_to_create:time_required_to_create,
        time_required_until_creation:time_required_until_creation,
        time_required_to_modify:time_required_to_modify,
        time_required_until_modification:time_required_until_modification,
        time_required_to_delete:time_required_to_delete,
        time_required_until_delete:time_required_until_delete,
        creation_limit_of_this_role_on_data:creation_limit_of_this_role_on_data,
        creation_limit:creation_limit,
        modification_limit_of_this_role_on_data:modification_limit_of_this_role_on_data,
        modification_limit:modification_limit,
        proposal_for_creation_limit_of_this_role_on_data:proposal_for_creation_limit_of_this_role_on_data,
        proposal_for_creation_limit:proposal_for_creation_limit,
        proposal_for_modification_limit_of_this_role_on_data:proposal_for_modification_limit_of_this_role_on_data,
        proposal_for_modification_limit:proposal_for_modification_limit,
        number_of_limit_to_execute_orders:number_of_limit_to_execute_orders,
        bump:bump,
    })

    return config;
}

export const create_data_config = (
        project_no:bigint,
        hierarchy_in_the_tree:number,
        who_can_create:number[],
        is_approval_by_the_creator_required_to_create:number,
        is_confirmation_by_the_creator_required_to_create:number,
        how_frequent_data_can_be_created:bigint,
        token_amount_needed_to_create:bigint,
        token_handled_after_creation:number,
        who_can_modify:number[],
        is_approval_by_the_creator_required_to_modify:number,
        is_confirmation_by_the_creator_required_to_modify:number,
        how_frequent_data_can_be_modified:bigint,
        token_amount_needed_to_modify:bigint,
        token_handled_after_modification:number,
        number_of_max_branches:bigint,
        number_of_max_versions:bigint,
        constanst:bigint[],
        initial_field_values:bigint[],
        orders:TheOrder[],
        who_can_execute_orders:number[][],
        max_number_of_order_execution:bigint[],
        bump:number,
) => {

    const config = new DataConfig({
        project_no:project_no,
        hierarchy_in_the_tree:hierarchy_in_the_tree,
        who_can_create:who_can_create,
        is_approval_by_the_creator_required_to_create:is_approval_by_the_creator_required_to_create,
        is_confirmation_by_the_creator_required_to_create:is_confirmation_by_the_creator_required_to_create,
        how_frequent_data_can_be_created:how_frequent_data_can_be_created,
        token_amount_needed_to_create:token_amount_needed_to_create,
        token_handled_after_creation:token_handled_after_creation,
        who_can_modify:who_can_modify,
        is_approval_by_the_creator_required_to_modify:is_approval_by_the_creator_required_to_modify,
        is_confirmation_by_the_creator_required_to_modify:is_confirmation_by_the_creator_required_to_modify,
        how_frequent_data_can_be_modified:how_frequent_data_can_be_modified,
        token_amount_needed_to_modify:token_amount_needed_to_modify,
        token_handled_after_modification:token_handled_after_modification,
        number_of_max_branches:number_of_max_branches,
        number_of_max_versions:number_of_max_versions,
        constanst:constanst,
        initial_field_values:initial_field_values,
        orders:orders,
        who_can_execute_orders:who_can_execute_orders,
        max_number_of_order_execution:max_number_of_order_execution,
        bump:bump,
    })

    return config;
}

export const create_execution_order = (
    inputs:bigint[],
    order_no:number
) => {

    const execution_order = new ExecutionData({
        inputs:inputs,
        order_no:order_no
    })

    return execution_order;
}
