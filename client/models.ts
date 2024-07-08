
class DataConfig {
    project_no:bigint = BigInt(0);
    bump:number = 0;
    hierarchy_in_the_tree:number = 0;
    who_can_create:number[] = [];
    is_approval_by_the_creator_required_to_create:number = 0;
    is_confirmation_by_the_creator_required_to_create:number = 0;
    how_frequent_data_can_be_created:bigint = BigInt(0);
    token_amount_needed_to_create:bigint = BigInt(0);
    token_handled_after_creation:number = 0;
    who_can_modify:number[] = [];
    is_approval_by_the_creator_required_to_modify:number = 0;
    is_confirmation_by_the_creator_required_to_modify:number = 0;
    how_frequent_data_can_be_modified:bigint = BigInt(0);
    token_amount_needed_to_modify:bigint = BigInt(0);
    token_handled_after_modification:number = 0;
    number_of_max_branches:bigint = BigInt(0);
    number_of_max_versions:bigint = BigInt(0);
    constanst:bigint[] = [];
    initial_field_values:bigint[] = [];
    orders:TheOrder[] = [];
    who_can_execute_orders:number[][] = [];
    max_number_of_order_execution:bigint[] = [];
  
  
    constructor(fields: {
  
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
  
     } | undefined = undefined)
      {if (fields) {
        this.project_no = fields.project_no;
        this.hierarchy_in_the_tree = fields.hierarchy_in_the_tree;
        this.who_can_create = fields.who_can_create;
        this.is_approval_by_the_creator_required_to_create = fields.is_approval_by_the_creator_required_to_create;
        this.is_confirmation_by_the_creator_required_to_create = fields.is_confirmation_by_the_creator_required_to_create;
        this.how_frequent_data_can_be_created = fields.how_frequent_data_can_be_created;
        this.token_amount_needed_to_create = fields.token_amount_needed_to_create;
        this.token_handled_after_creation = fields.token_handled_after_creation;
        this.who_can_modify = fields.who_can_modify;
        this.is_approval_by_the_creator_required_to_modify = fields.is_approval_by_the_creator_required_to_modify;
        this.is_confirmation_by_the_creator_required_to_modify = fields.is_confirmation_by_the_creator_required_to_modify;
        this.how_frequent_data_can_be_modified = fields.how_frequent_data_can_be_modified;
        this.token_amount_needed_to_modify = fields.token_amount_needed_to_modify;
        this.token_handled_after_modification = fields.token_handled_after_modification;
        this.number_of_max_branches = fields.number_of_max_branches;
        this.number_of_max_versions = fields.number_of_max_versions;
        this.constanst = fields.constanst;
        this.initial_field_values = fields.initial_field_values;
        this.orders = fields.orders;
        this.who_can_execute_orders = fields.who_can_execute_orders;
        this.max_number_of_order_execution = fields.max_number_of_order_execution;
        this.bump = fields.bump;
  
      }
    }
  
  }

class TheOrder {
    order:number[]=[]
    constructor(fields: {
        order:number[]
     } | undefined = undefined)
      {if (fields) {
        this.order = fields.order
      }
    }
  }

class TheData {

    creator:number[] = Array.from({ length: 32 }, () => 0);
    project_no:bigint = BigInt(0);
    hierarchy_in_the_tree:number = 0;
    parent_no:bigint = BigInt(0);
    data_no:bigint = BigInt(0);
    data_version:bigint = BigInt(0);
    last_time_data_added:bigint = BigInt(0);
    last_modified_on:bigint = BigInt(0);
    number_of_branches:bigint = BigInt(0);
    number_of_total_proposed_data:bigint = BigInt(0);
    total_number_of_executions:bigint[] = [];
    bump:number = 0;
    data:string = "";
    fields:bigint[] = [];
  
    constructor(fields: {
  
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
  
     } | undefined = undefined)
      {if (fields) {
        this.creator = fields.creator
        this.project_no = fields.project_no
        this.hierarchy_in_the_tree = fields.hierarchy_in_the_tree
        this.parent_no = fields.parent_no
        this.data_no = fields.data_no
        this.data_version = fields.data_version
        this.last_time_data_added = fields.last_time_data_added
        this.last_modified_on = fields.last_modified_on
        this.number_of_branches = fields.number_of_branches
        this.number_of_total_proposed_data = fields.number_of_total_proposed_data
        this.total_number_of_executions = fields.total_number_of_executions
        this.bump = fields.bump
        this.data = fields.data
        this.fields = fields.fields
  
     }
    }
  
  }

class ExecutionData {
    inputs:bigint[] = [];
    order_no:number = 0;
    constructor(fields: {
        inputs:bigint[]
        order_no:number
     } | undefined = undefined)
      {if (fields) {
        this.inputs = fields.inputs
        this.order_no = fields.order_no
      }
    }
  }

class DataStr {
    data:string = "";
    fields:bigint[] = [];
    constructor(fields: {
        data:string;
        fields:bigint[];
     } | undefined = undefined)
      {if (fields) {
        this.data = fields.data
        this.fields = fields.fields
      }
    }
  }

class TheProject {
    project_no:bigint = BigInt(0);
    initializer:number[] = Array.from({ length: 32 }, () => 0);
    token_mint:number[] = Array.from({ length: 32 }, () => 0);

    constructor(fields: {
        project_no:bigint;
        initializer:number[];
        token_mint:number[];

     } | undefined = undefined)
      {if (fields) {
        this.project_no = fields.project_no
        this.initializer = fields.initializer
        this.token_mint = fields.token_mint

      }
    }
  }

class RoleApplication {
    user:number[] = Array.from({ length: 32 }, () => 0);
    project_no:bigint = BigInt(0);
    hierachy_in_the_roles:number = 0;

    constructor(fields: {
        user:number[];
        project_no:bigint;
        hierachy_in_the_roles:number;

     } | undefined = undefined)
      {if (fields) {
        this.user = fields.user
        this.project_no = fields.project_no
        this.hierachy_in_the_roles = fields.hierachy_in_the_roles

      }
    }
  }

class RoleConfig {
     project_no:bigint = BigInt(0);
     hierachy_in_the_roles:number = 0;
     who_can_assign_this_role:number[] = [];
     who_can_enable_this_role:number[] = [];
     who_can_disable_this_role:number[] = [];
     time_required_to_create:number = 0;
     time_required_until_creation:bigint = BigInt(0);
     time_required_to_modify:number = 0;
     time_required_until_modification:bigint = BigInt(0);
     time_required_to_delete:number = 0;
     time_required_until_delete:bigint = BigInt(0);
     creation_limit_of_this_role_on_data:number[] = [];
     creation_limit:bigint[] = [];
     modification_limit_of_this_role_on_data:number[] = [];
     modification_limit:bigint[] = [];
     proposal_for_creation_limit_of_this_role_on_data:number[] = [];
     proposal_for_creation_limit:bigint[] = [];
     proposal_for_modification_limit_of_this_role_on_data:number[] = [];
     proposal_for_modification_limit:bigint[] = [];
     number_of_limit_to_execute_orders:bigint[][] = [];
     bump:number = 0;

    constructor(fields: {
        project_no:bigint ;
        hierachy_in_the_roles:number;
        who_can_assign_this_role:number[] ;
        who_can_enable_this_role:number[] ;
        who_can_disable_this_role:number[] ;
        time_required_to_create:number;
        time_required_until_creation:bigint ;
        time_required_to_modify:number;
        time_required_until_modification:bigint ;
        time_required_to_delete:number;
        time_required_until_delete:bigint ;
        creation_limit_of_this_role_on_data:number[] ;
        creation_limit:bigint[] ;
        modification_limit_of_this_role_on_data:number[] ;
        modification_limit:bigint[] ;
        proposal_for_creation_limit_of_this_role_on_data:number[] ;
        proposal_for_creation_limit:bigint[] ;
        proposal_for_modification_limit_of_this_role_on_data:number[] ;
        proposal_for_modification_limit:bigint[] ;
        number_of_limit_to_execute_orders:bigint[][] ;
        bump:number;

     } | undefined = undefined)
      {if (fields) {
        this.project_no = fields.project_no;
        this.hierachy_in_the_roles = fields.hierachy_in_the_roles;
        this.who_can_assign_this_role = fields.who_can_assign_this_role;
        this.who_can_enable_this_role = fields.who_can_enable_this_role;
        this.who_can_disable_this_role = fields.who_can_disable_this_role;
        this.time_required_to_create = fields.time_required_to_create;
        this.time_required_until_creation = fields.time_required_until_creation;
        this.time_required_to_modify = fields.time_required_to_modify;
        this.time_required_until_modification = fields.time_required_until_modification;
        this.time_required_to_delete = fields.time_required_to_delete;
        this.time_required_until_delete = fields.time_required_until_delete;
        this.creation_limit_of_this_role_on_data = fields.creation_limit_of_this_role_on_data;
        this.creation_limit = fields.creation_limit;
        this.modification_limit_of_this_role_on_data = fields.modification_limit_of_this_role_on_data;
        this.modification_limit = fields.modification_limit;
        this.proposal_for_creation_limit_of_this_role_on_data = fields.proposal_for_creation_limit_of_this_role_on_data;
        this.proposal_for_creation_limit = fields.proposal_for_creation_limit;
        this.proposal_for_modification_limit_of_this_role_on_data = fields.proposal_for_modification_limit_of_this_role_on_data;
        this.proposal_for_modification_limit = fields.proposal_for_modification_limit;
        this.number_of_limit_to_execute_orders = fields.number_of_limit_to_execute_orders;
        this.bump = fields.bump;

      }
    }
  }

class TheRole {
    project_no:bigint = BigInt(0);
    hierachy_in_the_roles:number = 0;
    user:number[] = Array.from({ length: 32 }, () => 0);
    created_on:bigint = BigInt(0);
    approved_to_create_data:number = 0;
    approved_to_modify_data:number = 0;
    approved_to_delete_data:number = 0;
    last_time_created_data:bigint = BigInt(0);
    last_time_modified_data:bigint = BigInt(0);
    data_created:bigint[] = [];
    data_modified:bigint[] = [];
    data_proposed_to_create:bigint[] = [];
    data_proposed_to_modify:bigint[] = [];
    number_of_orders_executed:bigint[][] = [];
    is_enabled:number = 0;

    constructor(fields: {
        project_no:bigint ;
        hierachy_in_the_roles:number;
        user:number[] ;
        created_on:bigint ;
        approved_to_create_data:number;
        approved_to_modify_data:number;
        approved_to_delete_data:number;
        last_time_created_data:bigint ;
        last_time_modified_data:bigint ;
        data_created:bigint[] ;
        data_modified:bigint[] ;
        data_proposed_to_create:bigint[] ;
        data_proposed_to_modify:bigint[] ;
        number_of_orders_executed:bigint[][] ;
        is_enabled:number;

     } | undefined = undefined)
      {if (fields) {
        this.project_no = fields.project_no;
        this.hierachy_in_the_roles = fields.hierachy_in_the_roles;
        this.user = fields.user;
        this.created_on = fields.created_on;
        this.approved_to_create_data = fields.approved_to_create_data;
        this.approved_to_modify_data = fields.approved_to_modify_data;
        this.approved_to_delete_data = fields.approved_to_delete_data;
        this.last_time_created_data = fields.last_time_created_data;
        this.last_time_modified_data = fields.last_time_modified_data;
        this.data_created = fields.data_created;
        this.data_modified = fields.data_modified;
        this.data_proposed_to_create = fields.data_proposed_to_create;
        this.data_proposed_to_modify = fields.data_proposed_to_modify;
        this.number_of_orders_executed = fields.number_of_orders_executed;
        this.is_enabled = fields.is_enabled;

      }
    }
  }

class Counter {
    counter:bigint = BigInt(0);

    constructor(fields: {
        counter:bigint
     } | undefined = undefined)
      {if (fields) {
        this.counter = fields.counter
      }
    }
  }

const DataConfigSchema = new Map([
    [RoleConfig,
      {
        kind: "struct",
        fields: [
            ["project_no","u64"],
            ["bump","u8"],
            ["hierarchy_in_the_tree","u8"],
            ["who_can_create",["u8"]],
            ["is_approval_by_the_creator_required_to_create","u8"],
            ["is_confirmation_by_the_creator_required_to_create","u8"],
            ["how_frequent_data_can_be_created","u64"],
            ["token_amount_needed_to_create","u64"],
            ["token_handled_after_creation","u8"],
            ["who_can_modify",["u8"]],
            ["is_approval_by_the_creator_required_to_modify","u8"],
            ["is_confirmation_by_the_creator_required_to_modify","u8"],
            ["how_frequent_data_can_be_modified","u64"],
            ["token_amount_needed_to_modify","u64"],
            ["token_handled_after_modification","u8"],
            ["number_of_max_branches","u64"],
            ["number_of_max_versions","u64"],
            ["constanst",["u64"]],
            ["initial_field_values",["u64"]],
            ["orders",[TheOrder]],
            ["who_can_execute_orders",[["u8"]]],
            ["max_number_of_order_execution",["u64"]],
        ]
      }
    ]
])

const TheOrderSchema = new Map([
    [TheOrder,
      {
        kind: "struct",
        fields: [
          ["order",["u16"]],
        ]
      }
    ]
])

const TheDataSchema = new Map([
    [TheData, {
        kind: 'struct',
        fields: [
            ['creator', ['u8',32]],
            ['project_no', 'u64'],
            ['hierarchy_in_the_tree', 'u8'],
            ['parent_no', 'u64'],
            ['data_no', 'u64'],
            ['data_version', 'u64'],
            ['last_time_data_added', 'u64'],
            ['last_modified_on', 'u64'],
            ['number_of_branches', 'u64'],
            ['number_of_total_proposed_data', 'u64'],
            ['total_number_of_executions', ['u64']],
            ['bump', 'u8'],
            ['data', 'string'],
            ['fields', ['u64']],
        ]
    }]
]);

const DataStrSchema = new Map([
    [DataStr, {
        kind: 'struct',
        fields: [
            ['data', 'string'],
            ['fields', ['u64']],
        ]
    }]
]);

const ExecutionDataSchema = new Map([
    [ExecutionData, {
        kind: 'struct',
        fields: [
            ['inputs', ['u64']],
            ['order_no', 'u32'],
        ]
    }]
]);

const TheProjectSchema = new Map([
    [TheProject, {
        kind: 'struct',
        fields: [
            ['project_no', 'u64'],
            ['initializer', ['u8',32]],
            ['token_mint', ['u8',32]],
        ]
    }]
]);

const RoleApplicationSchema = new Map([
    [RoleApplication, {
        kind: 'struct',
        fields: [
            ['user', ['u8',32]],
            ['project_no', 'u64'],
            ['hierachy_in_the_roles', 'u8'],
        ]
    }]
]);

const RoleConfigSchema = new Map([
    [RoleConfig, {
        kind: 'struct',
        fields: [
            ['project_no', 'u64'],
            ['hierachy_in_the_roles', 'u8'],
            ['who_can_assign_this_role', ['u8']],
            ['who_can_enable_this_role', ['u8']],
            ['who_can_disable_this_role', ['u8']],
            ['time_required_to_create', 'u8'],
            ['time_required_until_creation', 'u64'],
            ['time_required_to_modify', 'u8'],
            ['time_required_until_modification', 'u64'],
            ['time_required_to_delete', 'u8'],
            ['time_required_until_delete', 'u64'],
            ['creation_limit_of_this_role_on_data', ['u8']],
            ['creation_limit', ['u64']],
            ['modification_limit_of_this_role_on_data', ['u8']],
            ['modification_limit', ['u64']],
            ['proposal_for_creation_limit_of_this_role_on_data', ['u8']],
            ['proposal_for_creation_limit', ['u64']],
            ['proposal_for_modification_limit_of_this_role_on_data', ['u8']],
            ['proposal_for_modification_limit', ['u64']],
            ['number_of_limit_to_execute_orders', [['u64']]],
            ['bump', 'u8'],
        ]
    }]
]);

const TheRoleSchema = new Map([
    [TheRole, {
        kind: 'struct',
        fields: [
            ['project_no', 'u64'],
            ['hierachy_in_the_roles', 'u8'],
            ['user', ['u8',32]],
            ['created_on', 'u64'],
            ['approved_to_create_data', 'u8'],
            ['approved_to_modify_data', 'u8'],
            ['approved_to_delete_data', 'u8'],
            ['last_time_created_data', 'u64'],
            ['last_time_modified_data', 'u64'],
            ['data_created', ['u64']],
            ['data_modified', ['u64']],
            ['data_proposed_to_create', ['u64']],
            ['data_proposed_to_modify', ['u64']],
            ['number_of_orders_executed', [['u64']]],
            ['is_enabled', 'u8'],
        ]
    }]
]);

const CounterSchema = new Map([
    [Counter, {
        kind: 'struct',
        fields: [
            ['counter', 'u64'],
        ]
    }]
]);

