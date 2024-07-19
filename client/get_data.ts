import {  AccountInfo, Keypair, PublicKey} from "@solana/web3.js";
import { connection } from "./connection";
import {counter, programId} from "./accounts";
import { deserialize, serialize } from "borsh";
import { deserialize_data_account } from "./utils";

var BASE58 = '123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz';
var bs58 = require('base-x')(BASE58);

export const get_role_config = async (project_no:bigint,hierarchy_in_the_roles:number) => {//0

const project = new Counter()

project.counter = project_no;

const project_no_encoded = serialize(CounterSchema,project);

const project_no_base58_encoded = bs58.encode(project_no_encoded);


const hierarchy_in_the_roles_base58_encoded = bs58.encode([hierarchy_in_the_roles]);

  const role_config_account = await connection.getProgramAccounts(
    programId,
    {
        filters:[
            {
                memcmp:{
                    offset:0,
                    bytes:project_no_base58_encoded
                },
            },
            {
                memcmp:{
                    offset:8,
                    bytes:hierarchy_in_the_roles_base58_encoded
                }
            }
        ]
    });

    const role_config = serialize(RoleConfigSchema,role_config_account[0].account.data);

   return role_config;
}

export const get_data_config = async (project_no:bigint,hierarchy_in_the_tree:number) => {//0

const project = new Counter()

project.counter = project_no;

const project_no_encoded = serialize(CounterSchema,project);

const project_no_base58_encoded = bs58.encode(project_no_encoded);


const hierarchy_in_the_tree_base58_encoded = bs58.encode([hierarchy_in_the_tree]);

  const data_config_account = await connection.getProgramAccounts(
    programId,
    {
        filters:[
            {
                memcmp:{
                    offset:0,
                    bytes:project_no_base58_encoded
                },
            },
            {
                memcmp:{
                    offset:9,
                    bytes:hierarchy_in_the_tree_base58_encoded
                }
            }
        ]
    });

    const data_config = serialize(DataConfigSchema,data_config_account[0].account.data);

   return data_config;
  
}

export const get_all_data_accounts_by_the_creator = async (initializer:Keypair,project_no:bigint) => {//0

    const project = new Counter()

    project.counter = project_no;

    const project_no_encoded = serialize(CounterSchema,project);

    const project_no_base58_encoded = bs58.encode(project_no_encoded);



      const data_accounts = await connection.getProgramAccounts(
        programId,
        {
            filters:[
                {
                    memcmp:{
                        offset:0,
                        bytes:initializer.publicKey.toBase58()
                    }
                },
                {
                    memcmp:{
                        offset:32,
                        bytes:project_no_base58_encoded
                    },
                },
            ]
        });

        const data = serialize(TheDataSchema,data_accounts[0].account.data);

       return data;

}

export const get_all_data_accounts_by_the_creator_in_a_hierarchy = async (initializer:Keypair,project_no:bigint,hierarchy_in_the_tree:number) => {//0

    const project = new Counter()

    project.counter = project_no;

    const project_no_encoded = serialize(CounterSchema,project);

    const project_no_base58_encoded = bs58.encode(project_no_encoded);


    const hierarchy_in_the_tree_base58_encoded = bs58.encode([hierarchy_in_the_tree]);

      const data_accounts = await connection.getProgramAccounts(
        programId,
        {
            filters:[
                {
                    memcmp:{
                        offset:0,
                        bytes:initializer.publicKey.toBase58()
                    }
                },
                {
                    memcmp:{
                        offset:32,
                        bytes:project_no_base58_encoded
                    },
                },
                {
                    memcmp:{
                        offset:40,
                        bytes:hierarchy_in_the_tree_base58_encoded
                    }
                }
            ]
        });

        const data = serialize(TheDataSchema,data_accounts[0].account.data);

       return data;

}

export const get_the_data_account_by_the_creator = async (initializer:Keypair,project_no:bigint,hierarchy_in_the_tree:number,parent_no:bigint,data_no:bigint) => {//0

    const project = new Counter()
    const parent = new Counter()
    const data_no_to_serialize =new Counter()

    project.counter = project_no;
    parent.counter = parent_no;
    data_no_to_serialize.counter = data_no;

    const project_no_encoded = serialize(CounterSchema,project);
    const parent_no_encoded = serialize(CounterSchema,parent);
    const data_no_encoded = serialize(CounterSchema,data_no_to_serialize);

    const project_no_base58_encoded = bs58.encode(project_no_encoded);
    const parent_no_base58_encoded = bs58.encode(parent_no_encoded);
    const data_no_base58_encoded = bs58.encode(data_no_encoded);


    const hierarchy_in_the_tree_base58_encoded = bs58.encode([hierarchy_in_the_tree]);

      const data_accounts = await connection.getProgramAccounts(
        programId,
        {
            filters:[
                {
                    memcmp:{
                        offset:0,
                        bytes:initializer.publicKey.toBase58()
                    }
                },
                {
                    memcmp:{
                        offset:32,
                        bytes:project_no_base58_encoded
                    },
                },
                {
                    memcmp:{
                        offset:40,
                        bytes:hierarchy_in_the_tree_base58_encoded
                    }
                },
                {
                    memcmp:{
                        offset:41,
                        bytes:parent_no_base58_encoded
                    }
                },
                {
                    memcmp:{
                        offset:49,
                        bytes:data_no_base58_encoded
                    }
                }
            ]
        });


       return data_accounts;

}

export const get_all_data_accounts_by_the_creator_of_a_parent = async (initializer:Keypair,project_no:bigint,hierarchy_in_the_tree:number,parent_no:bigint) => {//0

    const project = new Counter()
    const parent = new Counter()

    project.counter = project_no;
    parent.counter = parent_no;

    const project_no_encoded = serialize(CounterSchema,project);
    const parent_no_encoded = serialize(CounterSchema,parent);

    const project_no_base58_encoded = bs58.encode(project_no_encoded);
    const parent_no_base58_encoded = bs58.encode(parent_no_encoded);


    const hierarchy_in_the_tree_base58_encoded = bs58.encode([hierarchy_in_the_tree]);

      const data_accounts = await connection.getProgramAccounts(
        programId,
        {
            filters:[
                {
                    memcmp:{
                        offset:0,
                        bytes:initializer.publicKey.toBase58()
                    }
                },
                {
                    memcmp:{
                        offset:32,
                        bytes:project_no_base58_encoded
                    },
                },
                {
                    memcmp:{
                        offset:40,
                        bytes:hierarchy_in_the_tree_base58_encoded
                    }
                },
                {
                    memcmp:{
                        offset:41,
                        bytes:parent_no_base58_encoded
                    }
                }
            ]
        });


       return data_accounts;

}

export const get_all_data_accounts_in_this_hierarchy = async (project_no:bigint,hierarchy_in_the_tree:number) => {//0

    const project = new Counter()

    project.counter = project_no;

    const project_no_encoded = serialize(CounterSchema,project);

    const project_no_base58_encoded = bs58.encode(project_no_encoded);


    const hierarchy_in_the_tree_base58_encoded = bs58.encode([hierarchy_in_the_tree]);

      const data_accounts = await connection.getProgramAccounts(
        programId,
        {
            filters:[
                {
                    memcmp:{
                        offset:32,
                        bytes:project_no_base58_encoded
                    },
                },
                {
                    memcmp:{
                        offset:40,
                        bytes:hierarchy_in_the_tree_base58_encoded
                    }
                }
            ]
        });

        const data = serialize(TheDataSchema,data_accounts[0].account.data);

       return data;

}

export const get_the_data_account = async (project_no:bigint,hierarchy_in_the_tree:number,parent_no:bigint,data_no:bigint) => {//0

    const project = new Counter()
    const parent = new Counter()
    const data_no_to_serialize =new Counter()

    project.counter = project_no;
    parent.counter = parent_no;
    data_no_to_serialize.counter = data_no;

    const project_no_encoded = serialize(CounterSchema,project);
    const parent_no_encoded = serialize(CounterSchema,parent);
    const data_no_encoded = serialize(CounterSchema,data_no_to_serialize);

    const project_no_base58_encoded = bs58.encode(project_no_encoded);
    const parent_no_base58_encoded = bs58.encode(parent_no_encoded);
    const data_no_base58_encoded = bs58.encode(data_no_encoded);


    const hierarchy_in_the_tree_base58_encoded = bs58.encode([hierarchy_in_the_tree]);

      const data_accounts = await connection.getProgramAccounts(
        programId,
        {
            filters:[
                {
                    memcmp:{
                        offset:32,
                        bytes:project_no_base58_encoded
                    },
                },
                {
                    memcmp:{
                        offset:40,
                        bytes:hierarchy_in_the_tree_base58_encoded
                    }
                },
                {
                    memcmp:{
                        offset:41,
                        bytes:parent_no_base58_encoded
                    }
                },
                {
                    memcmp:{
                        offset:49,
                        bytes:data_no_base58_encoded
                    }
                }
            ]
        });


       return data_accounts;

}

export const get_all_child_data_accounts_of_this_parent = async (project_no:bigint,hierarchy_in_the_tree:number,parent_no:bigint) => {//0

    const project = new Counter()
    const parent = new Counter()

    project.counter = project_no;
    parent.counter = parent_no;

    const project_no_encoded = serialize(CounterSchema,project);
    const parent_no_encoded = serialize(CounterSchema,parent);

    const project_no_base58_encoded = bs58.encode(project_no_encoded);
    const parent_no_base58_encoded = bs58.encode(parent_no_encoded);


    const hierarchy_in_the_tree_base58_encoded = bs58.encode([hierarchy_in_the_tree]);

      const data_accounts = await connection.getProgramAccounts(
        programId,
        {
            filters:[

                {
                    memcmp:{
                        offset:32,
                        bytes:project_no_base58_encoded
                    },
                },
                {
                    memcmp:{
                        offset:40,
                        bytes:hierarchy_in_the_tree_base58_encoded
                    }
                },
                {
                    memcmp:{
                        offset:41,
                        bytes:parent_no_base58_encoded
                    }
                }
            ]
        });


       return data_accounts;

}

export const get_role_account_of_the_user_in_a_hierarchy = async (role_owner:Keypair,project_no:bigint,hierachy_in_the_roles:number) => {//0

    const project = new Counter()

    project.counter = project_no;

    const project_no_encoded = serialize(CounterSchema,project);

    const project_no_base58_encoded = bs58.encode(project_no_encoded);
    const hierarchy_in_the_roles_base58_encoded = bs58.encode([hierachy_in_the_roles]);


      const data_accounts = await connection.getProgramAccounts(
        programId,
        {
            filters:[
                {
                    memcmp:{
                        offset:0,
                        bytes:project_no_base58_encoded
                    },
                },
                {
                    memcmp:{
                            offset:8,
                            bytes:hierarchy_in_the_roles_base58_encoded
                        },
                    },
                {
                    memcmp:{
                        offset:9,
                        bytes:role_owner.publicKey.toBase58()
                    }
                },

            ]
        });

        const data = serialize(TheDataSchema,data_accounts[0].account.data);

       return data;

}

export const get_data_proposals = async (
    project_no:bigint,
    hierarchy_in_the_tree:number,
    parent_no:bigint,
    data_no:bigint
) => {

    const project = new Counter()
    const parent = new Counter()
    const data_no_to_serialize =new Counter()

    project.counter = project_no;
    parent.counter = parent_no;
    data_no_to_serialize.counter = data_no;

    const project_no_encoded = serialize(CounterSchema,project);
    const parent_no_encoded = serialize(CounterSchema,parent);
    const data_no_encoded = serialize(CounterSchema,data_no_to_serialize);

    const project_no_base58_encoded = bs58.encode(project_no_encoded);
    const parent_no_base58_encoded = bs58.encode(parent_no_encoded);
    const data_no_base58_encoded = bs58.encode(data_no_encoded);


    const hierarchy_in_the_tree_base58_encoded = bs58.encode([hierarchy_in_the_tree]);

    const data_accounts = await connection.getProgramAccounts(
        programId,
        {
            filters:[
                {
                    memcmp:{
                        offset:32,
                        bytes:project_no_base58_encoded
                    },
                },
                {
                    memcmp:{
                        offset:40,
                        bytes:hierarchy_in_the_tree_base58_encoded
                    }
                },
                {
                    memcmp:{
                        offset:41,
                        bytes:parent_no_base58_encoded
                    }
                },
                {
                    memcmp:{
                        offset:49,
                        bytes:data_no_base58_encoded
                    }
                }
            ]
    });

    let proposal_accounts:TheData[] = [];

    for (let index = 0; index < data_accounts.length; index++) {

        const the_data = deserialize_data_account(data_accounts[index].account.data)
        if (the_data.data_version == BigInt(0)){
            proposal_accounts.push(the_data)
        }
        
    }
    
}

export const get_all_roles_in_a_hierarchy = async (project_no:bigint,hierachy_in_the_roles:number) => {//0

    const project = new Counter()

    project.counter = project_no;

    const project_no_encoded = serialize(CounterSchema,project);

    const project_no_base58_encoded = bs58.encode(project_no_encoded);
    const hierarchy_in_the_roles_base58_encoded = bs58.encode([hierachy_in_the_roles]);


      const data_accounts = await connection.getProgramAccounts(
        programId,
        {
            filters:[
                {
                    memcmp:{
                        offset:0,
                        bytes:project_no_base58_encoded
                    },
                },
                {
                memcmp:{
                        offset:8,
                        bytes:hierarchy_in_the_roles_base58_encoded
                    },
                },
            ]
        });

        const data = serialize(TheDataSchema,data_accounts[0].account.data);

       return data;

}

export const get_all_roles_of_the_user = async (role_owner:Keypair,project_no:bigint) => {//0

    const project = new Counter()

    project.counter = project_no;

    const project_no_encoded = serialize(CounterSchema,project);

    const project_no_base58_encoded = bs58.encode(project_no_encoded);


      const data_accounts = await connection.getProgramAccounts(
        programId,
        {
            filters:[
                {
                    memcmp:{
                        offset:0,
                        bytes:project_no_base58_encoded
                    },
                },
                {
                    memcmp:{
                        offset:9,
                        bytes:role_owner.publicKey.toBase58()
                    }
                },

            ]
        });

        const data = serialize(TheDataSchema,data_accounts[0].account.data);

       return data;

}

export const get_role_application_of_the_user = async(project_no:bigint,hierachy_in_the_roles:number,applicant:Keypair) => {

    const project = new Counter()

    project.counter = project_no;

    const project_no_encoded = serialize(CounterSchema,project);

    const project_no_base58_encoded = bs58.encode(project_no_encoded);
    const hierarchy_in_the_roles_base58_encoded = bs58.encode([hierachy_in_the_roles]);


      const data_accounts = await connection.getProgramAccounts(
        programId,
        {
            filters:[
                {
                    memcmp:{
                        offset:0,
                        bytes:applicant.publicKey.toBase58()
                    }
                },
                {
                    memcmp:{
                        offset:32,
                        bytes:project_no_base58_encoded
                    },
                },
                {
                    memcmp:{
                        offset:40,
                        bytes:hierarchy_in_the_roles_base58_encoded
                    }
                },

            ]
        });

        const data = serialize(RoleApplicationSchema,data_accounts[0].account.data);
}

export const get_all_role_applications_of_the_user = async(project_no:bigint,applicant:Keypair) => {

    const project = new Counter()

    project.counter = project_no;

    const project_no_encoded = serialize(CounterSchema,project);

    const project_no_base58_encoded = bs58.encode(project_no_encoded);


      const data_accounts = await connection.getProgramAccounts(
        programId,
        {
            filters:[
                {
                    memcmp:{
                        offset:0,
                        bytes:applicant.publicKey.toBase58()
                    }
                },
                {
                    memcmp:{
                        offset:32,
                        bytes:project_no_base58_encoded
                    },
                },

            ]
        });

        const data = serialize(RoleApplicationSchema,data_accounts[0].account.data);
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

    const data_config_account = PublicKey.findProgramAddressSync([
        Buffer.from(project_no.toString()),
        Buffer.from("dac"),
        Buffer.from(hierarchy_in_the_tree.toString())],programId);

    const data_config = new DataConfig({
        project_no:    project_no,
        hierarchy_in_the_tree:    hierarchy_in_the_tree,
        who_can_create: who_can_create,
        is_approval_by_the_creator_required_to_create:    is_approval_by_the_creator_required_to_create,
        is_confirmation_by_the_creator_required_to_create:    is_confirmation_by_the_creator_required_to_create,
        how_frequent_data_can_be_created:    how_frequent_data_can_be_created,
        token_amount_needed_to_create:    token_amount_needed_to_create,
        token_handled_after_creation:    token_handled_after_creation,
        who_can_modify:  who_can_modify,
        is_approval_by_the_creator_required_to_modify:    is_approval_by_the_creator_required_to_modify,
        is_confirmation_by_the_creator_required_to_modify:    is_confirmation_by_the_creator_required_to_modify,
        how_frequent_data_can_be_modified:    how_frequent_data_can_be_modified,
        token_amount_needed_to_modify:    token_amount_needed_to_modify,
        token_handled_after_modification:    token_handled_after_modification,
        number_of_max_branches:    number_of_max_branches,
        number_of_max_versions:    number_of_max_versions,
        constanst:  constanst,
        initial_field_values:  initial_field_values,
        orders:  orders,
        who_can_execute_orders:  who_can_execute_orders,
        max_number_of_order_execution:  max_number_of_order_execution,
        bump:   data_config_account[1],
    })

    return data_config;

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

    const role_config_account = PublicKey.findProgramAddressSync([
        Buffer.from(project_no.toString()),
        Buffer.from("roc"),
        Buffer.from(hierachy_in_the_roles.toString())],programId);

const role_config = new RoleConfig({
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
    bump:role_config_account[1],
})

return role_config;

}

export const get_counter_data = async () => {
    const acc_info = await connection.getAccountInfo(counter);

    const counter_data = deserialize(CounterSchema,Counter,acc_info!.data);

    return counter_data.counter;
}

export const get_project= async (project_no:bigint) => {

    const project = new Counter()

    project.counter = project_no;

    const project_no_encoded = serialize(CounterSchema,project);

    const project_no_base58_encoded = bs58.encode(project_no_encoded);


      const project_account = await connection.getProgramAccounts(
        programId,
        {
            filters:[
                {
                    memcmp:{
                        offset:0,
                        bytes:project_no_base58_encoded
                    }
                },

            ]
        });
    
    const project_account_data = deserialize(TheProjectSchema,TheProject,project_account[0].account.data);

    return project_account_data;
}

export const get_all_projects_of_the_user = async (project_no:bigint,initializer:PublicKey) => {

    const project = new Counter()

    project.counter = project_no;

    const project_no_encoded = serialize(CounterSchema,project);

    const project_no_base58_encoded = bs58.encode(project_no_encoded);


      const data_accounts = await connection.getProgramAccounts(
        programId,
        {
            filters:[
                {
                    dataSize:72
                },
                {
                    memcmp:{
                        offset:8,
                        bytes:initializer.toBase58()
                    },
                },

            ]
        });
}

