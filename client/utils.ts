import { Keypair, PublicKey, SystemProgram, SYSVAR_RENT_PUBKEY, TransactionInstruction, TransactionMessage, VersionedTransaction } from "@solana/web3.js";
import { connection } from "./connection";
import {counter, programId} from "./accounts";
import { deserialize, serialize } from "borsh";
import { ASSOCIATED_TOKEN_PROGRAM_ID, getAssociatedTokenAddressSync, TOKEN_2022_PROGRAM_ID } from "@solana/spl-token";

var BASE58 = '123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz';
var bs58 = require('base-x')(BASE58);

export const get_role_config_account = async (project_no:bigint,hierarchy_in_the_roles:number) => {//0

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
    const hierachy_in_the_roles_base58_encoded = bs58.encode([hierachy_in_the_roles]);


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
                            bytes:hierachy_in_the_roles_base58_encoded
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


export const get_all_roles_in_a_hierarchy = async (project_no:bigint,hierachy_in_the_roles:number) => {//0

    const project = new Counter()

    project.counter = project_no;

    const project_no_encoded = serialize(CounterSchema,project);

    const project_no_base58_encoded = bs58.encode(project_no_encoded);
    const hierachy_in_the_roles_base58_encoded = bs58.encode([hierachy_in_the_roles]);


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
                        bytes:hierachy_in_the_roles_base58_encoded
                    },
                },
            ]
        });

        const data = serialize(TheDataSchema,data_accounts[0].account.data);

       return data;

}

export const get_all_roles_of_the_user= async (role_owner:Keypair,project_no:bigint) => {//0

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