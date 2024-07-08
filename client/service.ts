import { Keypair, PublicKey, SystemProgram, SYSVAR_RENT_PUBKEY, TransactionInstruction, TransactionMessage, VersionedTransaction } from "@solana/web3.js";
import { connection } from "./connection";
import {counter, programId} from "./accounts";
import { deserialize, serialize } from "borsh";
import { ASSOCIATED_TOKEN_PROGRAM_ID, getAssociatedTokenAddressSync, TOKEN_2022_PROGRAM_ID } from "@solana/spl-token";


export const initialize_project_without_token = async (initializer:Keypair,initial_data:DataStr) => {//0

  const counter_acc_info = await connection.getAccountInfo(counter);
  const counter_data  = deserialize(CounterSchema,Counter,counter_acc_info!.data);

  const project_no = (counter_data.counter+BigInt(1)).toString()

  const encoded = serialize(DataStrSchema,initial_data);

  const concated = Uint8Array.of(0,...encoded)


  const project_account = PublicKey.findProgramAddressSync([Buffer.from(project_no)],programId)
  const data_account = PublicKey.findProgramAddressSync([
    Buffer.from(project_no),
    Buffer.from("hie"), Buffer.from("1"),
    Buffer.from("prnt"), Buffer.from("0"),
    Buffer.from("dat"), Buffer.from("1"),
    Buffer.from("ver"), Buffer.from("1")
    ],programId)

  const role_account = PublicKey.findProgramAddressSync([Buffer.from(project_no),Buffer.from("role"),Buffer.from("1"),initializer.publicKey.toBytes()],programId)

    const ix = new TransactionInstruction({
        programId:programId,
        keys:[
            {isSigner:true,isWritable:true,pubkey:initializer.publicKey},
            {isSigner:false,isWritable:true,pubkey:counter},
            {isSigner:false,isWritable:true,pubkey:project_account[0]},
            {isSigner:false,isWritable:true,pubkey:data_account[0]},
            {isSigner:false,isWritable:true,pubkey:role_account[0]},
            {isSigner:false,isWritable:true,pubkey:SystemProgram.programId},
        ],
        data:Buffer.from(concated)
    })

    const message = new TransactionMessage({
        instructions: [ix],
          payerKey: initializer.publicKey!,
          recentBlockhash : (await connection.getLatestBlockhash()).blockhash
        }).compileToV0Message();
    
        const tx = new VersionedTransaction(message);
  

        tx.sign([initializer]);

}

export const initialize_project_with_token = async (initializer:Keypair,initial_data:DataStr) => {//1


const counter_acc_info = await connection.getAccountInfo(counter);
const counter_data  = deserialize(CounterSchema,Counter,counter_acc_info!.data);

const project_no = (counter_data.counter+BigInt(1)).toString()

const encoded = serialize(DataStrSchema,initial_data);

const concated = Uint8Array.of(1,...encoded)


const project_account = PublicKey.findProgramAddressSync([Buffer.from(project_no)],programId)
const data_account = PublicKey.findProgramAddressSync([
  Buffer.from(project_no),
  Buffer.from("hie"), Buffer.from("1"),
  Buffer.from("prnt"), Buffer.from("0"),
  Buffer.from("dat"), Buffer.from("1"),
  Buffer.from("ver"), Buffer.from("1")
  ],programId)

const role_account = PublicKey.findProgramAddressSync([Buffer.from(project_no),Buffer.from("role"),Buffer.from("1"),initializer.publicKey.toBytes()],programId)

const token_mint = Keypair.generate();

const project_account_ata = getAssociatedTokenAddressSync(token_mint.publicKey,project_account[0],true,TOKEN_2022_PROGRAM_ID,ASSOCIATED_TOKEN_PROGRAM_ID);

  const ix = new TransactionInstruction({
      programId:programId,
      keys:[
          {isSigner:true,isWritable:true,pubkey:initializer.publicKey},
          {isSigner:false,isWritable:true,pubkey:counter},
          {isSigner:false,isWritable:true,pubkey:project_account[0]},
          {isSigner:false,isWritable:true,pubkey:project_account_ata},
          {isSigner:false,isWritable:true,pubkey:data_account[0]},
          {isSigner:false,isWritable:true,pubkey:role_account[0]},
          {isSigner:true,isWritable:true,pubkey:token_mint.publicKey},
          {isSigner:false,isWritable:true,pubkey:TOKEN_2022_PROGRAM_ID},
          {isSigner:false,isWritable:true,pubkey:SYSVAR_RENT_PUBKEY},
          {isSigner:false,isWritable:true,pubkey:ASSOCIATED_TOKEN_PROGRAM_ID},
          {isSigner:false,isWritable:true,pubkey:SystemProgram.programId},
      ],
      data:Buffer.from(concated)
  })

  const message = new TransactionMessage({
    instructions: [ix],
      payerKey: initializer.publicKey!,
      recentBlockhash : (await connection.getLatestBlockhash()).blockhash
    }).compileToV0Message();

    const tx = new VersionedTransaction(message);


    tx.sign([initializer,token_mint]);

}

export const add_token_to_project = async (initializer:Keypair,project_account:PublicKey,token_mint:PublicKey) => {//2


      const ix = new TransactionInstruction({
          programId:programId,
          keys:[
              {isSigner:true,isWritable:true,pubkey:initializer.publicKey},
              {isSigner:false,isWritable:true,pubkey:project_account},
              {isSigner:false,isWritable:true,pubkey:token_mint},

          ],
          data:Buffer.from([2])
      })
    
      const message = new TransactionMessage({
        instructions: [ix],
          payerKey: initializer.publicKey!,
          recentBlockhash : (await connection.getLatestBlockhash()).blockhash
        }).compileToV0Message();
    
        const tx = new VersionedTransaction(message);
  

        tx.sign([initializer]);

}

export const init_data_config = async (
    initializer:Keypair,
    project_account:PublicKey,
    data_config:RoleConfig,
    project_no:bigint,
    hierarchy_in_the_tree:number
) => {//3



    const encoded = serialize(RoleConfigSchema,data_config);

    const concated = Uint8Array.of(3,...encoded)

    const data_config_account = PublicKey.findProgramAddressSync([
        Buffer.from(project_no.toString()),
        Buffer.from("dac"),
        Buffer.from(hierarchy_in_the_tree.toString())],programId);

    const ix = new TransactionInstruction({
        programId:programId,
        keys:[
            {isSigner:true,isWritable:true,pubkey:initializer.publicKey},
            {isSigner:false,isWritable:true,pubkey:project_account},
            {isSigner:false,isWritable:true,pubkey:data_config_account[0]},
        ],
        data:Buffer.from(concated)
    })
  
    const message = new TransactionMessage({
        instructions: [ix],
          payerKey: initializer.publicKey!,
          recentBlockhash : (await connection.getLatestBlockhash()).blockhash
        }).compileToV0Message();
    
        const tx = new VersionedTransaction(message);
  

        tx.sign([initializer]);

}

export const init_role_config = async (
    initializer:Keypair,
    project_account:PublicKey,
    role_config:RoleConfig,
    project_no:bigint,
    hierachy_in_the_roles:number
) => {//4


    const encoded = serialize(RoleConfigSchema,role_config);

    const concated = Uint8Array.of(4,...encoded)

    const role_config_account = PublicKey.findProgramAddressSync([
        Buffer.from(project_no.toString()),
        Buffer.from("roc"),
        Buffer.from(hierachy_in_the_roles.toString())],programId);

    const ix = new TransactionInstruction({
        programId:programId,
        keys:[
            {isSigner:true,isWritable:true,pubkey:initializer.publicKey},
            {isSigner:false,isWritable:true,pubkey:project_account},
            {isSigner:false,isWritable:true,pubkey:role_config_account[0]},
        ],
        data:Buffer.from(concated)
    })

    const message = new TransactionMessage({
        instructions: [ix],
          payerKey: initializer.publicKey!,
          recentBlockhash : (await connection.getLatestBlockhash()).blockhash
        }).compileToV0Message();
    
        const tx = new VersionedTransaction(message);
  

        tx.sign([initializer]);
  

        connection.sendTransaction(tx);

}

export const create_or_propose_creating_data = async (
    creator:Keypair,
    data:DataStr,
    project_no:bigint,
    hierachy_in_the_roles:number,
    hierarchy_in_the_tree:number,
    role_account:PublicKey,
    data_no:bigint,
    data_version:bigint,
    parent_data_no:bigint,
    parent_data_version:bigint,

) => {//5


    const encoded = serialize(DataStrSchema,data);

    const concated = Uint8Array.of(5,...encoded);

    const role_config_account = PublicKey.findProgramAddressSync([
        Buffer.from(project_no.toString()),
        Buffer.from("roc"),
        Buffer.from(hierachy_in_the_roles.toString())],programId);

    const data_config_account = PublicKey.findProgramAddressSync([
        Buffer.from(project_no.toString()),
        Buffer.from("dac"),
        Buffer.from(hierarchy_in_the_tree.toString())],programId);

    const proposal_account = Keypair.generate();

    const parent_hierarchy = hierarchy_in_the_tree - 1;
    const grand_parent_hierarchy = parent_hierarchy - 1;

    const data_account = PublicKey.findProgramAddressSync([
        Buffer.from(project_no.toString()),
        Buffer.from("hie"), Buffer.from(hierarchy_in_the_tree.toString()),
        Buffer.from("prnt"), Buffer.from(parent_hierarchy.toString()),
        Buffer.from("dat"), Buffer.from(data_no.toString()),
        Buffer.from("ver"), Buffer.from(data_version.toString())
        ],programId)


    const parent_data_account = PublicKey.findProgramAddressSync([
        Buffer.from(project_no.toString()),
        Buffer.from("hie"), Buffer.from(parent_hierarchy.toString()),
        Buffer.from("prnt"), Buffer.from(grand_parent_hierarchy.toString()),
        Buffer.from("dat"), Buffer.from(parent_data_no.toString()),
        Buffer.from("ver"), Buffer.from(parent_data_version.toString())
        ],programId)

    const ix = new TransactionInstruction({
        programId:programId,
        keys:[
            {isSigner:true,isWritable:true,pubkey:creator.publicKey},
            {isSigner:false,isWritable:false,pubkey:data_config_account[0]},
            {isSigner:false,isWritable:true,pubkey:data_account[0]},
            {isSigner:false,isWritable:true,pubkey:parent_data_account[0]},
            {isSigner:true,isWritable:true,pubkey:proposal_account.publicKey},
            {isSigner:false,isWritable:true,pubkey:role_account},
            {isSigner:false,isWritable:false,pubkey:role_config_account[0]},
            {isSigner:false,isWritable:false,pubkey:SystemProgram.programId},
        ],
        data:Buffer.from(concated)
    })

    
    const message = new TransactionMessage({
        instructions: [ix],
          payerKey: creator.publicKey!,
          recentBlockhash : (await connection.getLatestBlockhash()).blockhash
        }).compileToV0Message();
    
        const tx = new VersionedTransaction(message);
  

        tx.sign([creator,proposal_account]);
  

        connection.sendTransaction(tx);


}

export const create_assign_or_apply_for_a_role = async (
    role_creator:Keypair,
    creator_role_account:PublicKey,
    project_no:bigint,
    hierachy_in_the_roles:number,
    role_account:PublicKey,
    role_owner:PublicKey
) => {//6

    const role_config_account = PublicKey.findProgramAddressSync([
        Buffer.from(project_no.toString()),
        Buffer.from("roc"),
        Buffer.from(hierachy_in_the_roles.toString())],programId);

    const role_application_account = Keypair.generate()


        const ix = new TransactionInstruction({
            programId:programId,
            keys:[
                {isSigner:true,isWritable:true,pubkey:role_creator.publicKey},
                {isSigner:false,isWritable:true,pubkey:creator_role_account},
                {isSigner:false,isWritable:false,pubkey:role_config_account[0]},
                {isSigner:false,isWritable:true,pubkey:role_account},
                {isSigner:true,isWritable:true,pubkey:role_application_account.publicKey},
                {isSigner:false,isWritable:true,pubkey:role_owner},
                {isSigner:false,isWritable:false,pubkey:SystemProgram.programId},
            ],
            data:Buffer.from([6])
        })
    
        const message = new TransactionMessage({
            instructions: [ix],
              payerKey: role_creator.publicKey!,
              recentBlockhash : (await connection.getLatestBlockhash()).blockhash
            }).compileToV0Message();
        
            const tx = new VersionedTransaction(message);
      
    
            tx.sign([role_creator,role_application_account]);
      
    
            connection.sendTransaction(tx);


}

export const enable_or_disable_role = async (
    role_manager:Keypair,
    manager_role_account:PublicKey,
    role_account:PublicKey,
    project_no:bigint,
    hierachy_in_the_roles:number,

) => {//10

    const role_config_account = PublicKey.findProgramAddressSync([
        Buffer.from(project_no.toString()),
        Buffer.from("roc"),
        Buffer.from(hierachy_in_the_roles.toString())],programId);


        const ix = new TransactionInstruction({
            programId:programId,
            keys:[
                {isSigner:true,isWritable:true,pubkey:role_manager.publicKey},//account that is enabling or disabling the role account
                {isSigner:false,isWritable:true,pubkey:manager_role_account},//role account of the role_manager
                {isSigner:false,isWritable:false,pubkey:role_config_account[0]},
                {isSigner:false,isWritable:true,pubkey:role_account},//role to be enabled or diabled
            ],
            data:Buffer.from([10])
        })

        const message = new TransactionMessage({
            instructions: [ix],
              payerKey: role_manager.publicKey!,
              recentBlockhash : (await connection.getLatestBlockhash()).blockhash
            }).compileToV0Message();
        
            const tx = new VersionedTransaction(message);
      
    
            tx.sign([role_manager]);
      
    
            connection.sendTransaction(tx);

}

export const modify_or_propose_modification_data = async (
modifier:Keypair,
data:DataStr,
project_no:bigint,
hierachy_in_the_roles:number,
hierarchy_in_the_tree:number,
role_account:PublicKey,
data_no:bigint,
data_version:bigint,
) => {//7


    const role_config_account = PublicKey.findProgramAddressSync([
        Buffer.from(project_no.toString()),
        Buffer.from("roc"),
        Buffer.from(hierachy_in_the_roles.toString())],programId);

    const data_config_account = PublicKey.findProgramAddressSync([
        Buffer.from(project_no.toString()),
        Buffer.from("dac"),
        Buffer.from(hierarchy_in_the_tree.toString())],programId);

    const proposal_account = Keypair.generate();

    const parent_hierarchy = hierarchy_in_the_tree - 1;

    const data_account = PublicKey.findProgramAddressSync([
        Buffer.from(project_no.toString()),
        Buffer.from("hie"), Buffer.from(hierarchy_in_the_tree.toString()),
        Buffer.from("prnt"), Buffer.from(parent_hierarchy.toString()),
        Buffer.from("dat"), Buffer.from(data_no.toString()),
        Buffer.from("ver"), Buffer.from(data_version.toString())
        ],programId)


    const encoded = serialize(DataStrSchema,data);

    const concated = Uint8Array.of(7,...encoded);


    const ix = new TransactionInstruction({
        programId:programId,
        keys:[
            {isSigner:true,isWritable:true,pubkey:modifier.publicKey},
            {isSigner:false,isWritable:false,pubkey:data_config_account[0]},
            {isSigner:false,isWritable:true,pubkey:data_account[0]},
            {isSigner:true,isWritable:true,pubkey:proposal_account.publicKey},
            {isSigner:false,isWritable:true,pubkey:role_account},
            {isSigner:false,isWritable:false,pubkey:role_config_account[0]},
            {isSigner:false,isWritable:false,pubkey:SystemProgram.programId},
        ],
        data:Buffer.from(concated)
    })

    const message = new TransactionMessage({
        instructions: [ix],
          payerKey: modifier.publicKey!,
          recentBlockhash : (await connection.getLatestBlockhash()).blockhash
        }).compileToV0Message();
    
        const tx = new VersionedTransaction(message);
  

        tx.sign([modifier,proposal_account]);
  

        connection.sendTransaction(tx);

}

export const confirm_proposal_for_creating_data = async (
    creator:Keypair,
    data:DataStr,
    project_no:bigint,
    hierachy_in_the_roles:number,
    hierarchy_in_the_tree:number,
    role_account:PublicKey,
    data_no:bigint,
    data_version:bigint,
    parent_data_no:bigint,
    parent_data_version:bigint,
) => {//8

    const encoded = serialize(DataStrSchema,data);

    const concated = Uint8Array.of(8,...encoded);

    const role_config_account = PublicKey.findProgramAddressSync([
        Buffer.from(project_no.toString()),
        Buffer.from("roc"),
        Buffer.from(hierachy_in_the_roles.toString())],programId);

    const data_config_account = PublicKey.findProgramAddressSync([
        Buffer.from(project_no.toString()),
        Buffer.from("dac"),
        Buffer.from(hierarchy_in_the_tree.toString())],programId);

    const proposal_account = Keypair.generate();

    const parent_hierarchy = hierarchy_in_the_tree - 1;
    const grand_parent_hierarchy = parent_hierarchy - 1;

    const data_account = PublicKey.findProgramAddressSync([
        Buffer.from(project_no.toString()),
        Buffer.from("hie"), Buffer.from(hierarchy_in_the_tree.toString()),
        Buffer.from("prnt"), Buffer.from(parent_hierarchy.toString()),
        Buffer.from("dat"), Buffer.from(data_no.toString()),
        Buffer.from("ver"), Buffer.from(data_version.toString())
        ],programId);


    const parent_data_account = PublicKey.findProgramAddressSync([
        Buffer.from(project_no.toString()),
        Buffer.from("hie"), Buffer.from(parent_hierarchy.toString()),
        Buffer.from("prnt"), Buffer.from(grand_parent_hierarchy.toString()),
        Buffer.from("dat"), Buffer.from(parent_data_no.toString()),
        Buffer.from("ver"), Buffer.from(parent_data_version.toString())
        ],programId);


    const ix = new TransactionInstruction({
        programId:programId,
        keys:[
            {isSigner:true,isWritable:true,pubkey:creator.publicKey},
            {isSigner:false,isWritable:false,pubkey:data_config_account[0]},
            {isSigner:false,isWritable:true,pubkey:data_account[0]},
            {isSigner:false,isWritable:true,pubkey:parent_data_account[0]},
            {isSigner:false,isWritable:true,pubkey:proposal_account.publicKey},
            {isSigner:false,isWritable:true,pubkey:role_account},
            {isSigner:false,isWritable:false,pubkey:role_config_account[0]},
            {isSigner:false,isWritable:false,pubkey:SystemProgram.programId},
        ],
        data:Buffer.from(concated)
    });
    
    const message = new TransactionMessage({
        instructions: [ix],
          payerKey: creator.publicKey!,
          recentBlockhash : (await connection.getLatestBlockhash()).blockhash
        }).compileToV0Message();
    
        const tx = new VersionedTransaction(message);
  

        tx.sign([creator]);
  

        connection.sendTransaction(tx);
}

export const confirm_proposal_for_modifying_data = async (
    confirmer:Keypair,
    proposed_data_account:PublicKey,
    proposer:PublicKey,
    project_no:bigint,
    hierarchy_in_the_tree:number,
    parent_no:bigint,
    data_no:bigint,
    data_version:bigint,

) => {


    const data_account = PublicKey.findProgramAddressSync([
        Buffer.from(project_no.toString()),
        Buffer.from("hie"), Buffer.from(hierarchy_in_the_tree.toString()),
        Buffer.from("prnt"), Buffer.from(parent_no.toString()),
        Buffer.from("dat"), Buffer.from(data_no.toString()),
        Buffer.from("ver"), Buffer.from(data_version.toString())
        ],programId)

    const data_config_account = PublicKey.findProgramAddressSync([
        Buffer.from(project_no.toString()),
        Buffer.from("dac"),
        Buffer.from(hierarchy_in_the_tree.toString())],programId);

    const new_data_version = data_version + BigInt(1);

    const new_version_data_account = PublicKey.findProgramAddressSync([
        Buffer.from(project_no.toString()),
        Buffer.from("hie"), Buffer.from(hierarchy_in_the_tree.toString()),
        Buffer.from("prnt"), Buffer.from(parent_no.toString()),
        Buffer.from("dat"), Buffer.from(data_no.toString()),
        Buffer.from("ver"), Buffer.from(new_data_version.toString())
        ],programId)



    const ix = new TransactionInstruction({
      programId:programId,
      keys:[
          {isSigner:true,isWritable:true,pubkey:confirmer.publicKey},
          {isSigner:true,isWritable:true,pubkey:proposer},
          {isSigner:true,isWritable:true,pubkey:data_config_account[0]},
          {isSigner:true,isWritable:true,pubkey:proposed_data_account},
          {isSigner:true,isWritable:true,pubkey:data_account[0]},
          {isSigner:true,isWritable:true,pubkey:new_version_data_account[0]},
          {isSigner:false,isWritable:false,pubkey:SystemProgram.programId},
      ],
      data:Buffer.from([11])
    });

   const message = new TransactionMessage({
    instructions: [ix],
      payerKey: confirmer.publicKey!,
      recentBlockhash : (await connection.getLatestBlockhash()).blockhash
    }).compileToV0Message();

    const tx = new VersionedTransaction(message);


    tx.sign([confirmer]);


    connection.sendTransaction(tx);
}

export const execute_order = async (
executor:Keypair,
execution_data:ExecutionData,
project_no:bigint,
hierachy_in_the_roles:number,
hierarchy_in_the_tree:number,
executor_role_account:PublicKey,
data_no:bigint,
data_version:bigint,

) => {//9


   const encoded = serialize(ExecutionDataSchema,execution_data);

   const concated = Uint8Array.of(9,...encoded);

   const executor_role_config_account = PublicKey.findProgramAddressSync([
       Buffer.from(project_no.toString()),
       Buffer.from("roc"),
       Buffer.from(hierachy_in_the_roles.toString())],programId);

   const data_config_account = PublicKey.findProgramAddressSync([
       Buffer.from(project_no.toString()),
       Buffer.from("dac"),
       Buffer.from(hierarchy_in_the_tree.toString())],programId);


   const parent_hierarchy = hierarchy_in_the_tree - 1;

   const data_account = PublicKey.findProgramAddressSync([
       Buffer.from(project_no.toString()),
       Buffer.from("hie"), Buffer.from(hierarchy_in_the_tree.toString()),
       Buffer.from("prnt"), Buffer.from(parent_hierarchy.toString()),
       Buffer.from("dat"), Buffer.from(data_no.toString()),
       Buffer.from("ver"), Buffer.from(data_version.toString())
       ],programId);

       const ix = new TransactionInstruction({
        programId:programId,
        keys:[
            {isSigner:true,isWritable:true,pubkey:executor.publicKey},
            {isSigner:false,isWritable:true,pubkey:data_config_account[0]},
            {isSigner:false,isWritable:true,pubkey:data_account[0]},
            {isSigner:false,isWritable:true,pubkey:executor_role_account},
            {isSigner:false,isWritable:true,pubkey:executor_role_config_account[0]},

        ],
        data:Buffer.from(concated)
    });
    
    const message = new TransactionMessage({
        instructions: [ix],
          payerKey: executor.publicKey!,
          recentBlockhash : (await connection.getLatestBlockhash()).blockhash
        }).compileToV0Message();
    
        const tx = new VersionedTransaction(message);
  

        tx.sign([executor]);
  

        connection.sendTransaction(tx);


}


