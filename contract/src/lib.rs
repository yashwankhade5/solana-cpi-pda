use solana_program::{account_info::{next_account_info, AccountInfo}, address_lookup_table::program, entrypoint::{ ProgramResult},entrypoint, instruction::{AccountMeta, Instruction}, native_token::LAMPORTS_PER_SOL, program::{invoke, invoke_signed}, pubkey::{self, Pubkey}, system_instruction::create_account
};



entrypoint!(process_instruction);

fn process_instruction(
        program_id:&Pubkey,
        accounts:&[AccountInfo],
        instruction_data:&[u8]
)->ProgramResult{
let mut  iter = accounts.iter();
let user_acc = next_account_info(&mut iter)?;
let pda = next_account_info(&mut iter)?;
let system_program = next_account_info(&mut iter)?;

let (Pda,bump) = Pubkey::find_program_address(&[b"user",user_acc.key.as_ref()], program_id);
let ix = create_account(user_acc.key, pda.key, LAMPORTS_PER_SOL, 8, program_id);


 let signed_seeds = &[b"user",user_acc.key.as_ref(),&[bump]];
 invoke_signed(&ix, accounts,&[signed_seeds])?;
    Ok(())
}