use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
};

// use rand::Rng; // 0.8.0

/// Define the type of state stored in accounts
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct GreetingAccount {
    pub counter: u32,
}


#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct MoreData { 
    pub more:u32,
    // pub another_number : u32,
}

// Declare and export the program's entrypoint
entrypoint!(process_instruction);

// Program entrypoint's implementation
pub fn process_instruction(
    program_id: &Pubkey, // Public key of the account the hello world program was loaded into
    accounts: &[AccountInfo], // The account to say hello to
    _instruction_data: &[u8], // Ignored, all helloworld instructions are hellos
) -> ProgramResult {
    

    // Iterating accounts is safer than indexing
    let accounts_iter = &mut accounts.iter();

    // Get the account to say hello to
    let account = next_account_info(accounts_iter)?;

    // Increment and store the number of times the account has been greeted
    let mut greeting_account = GreetingAccount::try_from_slice(&account.data.borrow())?;
    greeting_account.counter += 1;
    // greeting_account.my_str = "sss";
    msg!("Making a log here on the smart contract \n Log no  {}",greeting_account.counter);
    // greeting_account.my_str = 'h';
    greeting_account.serialize(&mut &mut account.data.borrow_mut()[..])?;

    let mut moreD = MoreData::try_from_slice(&account.data.borrow())?;
    moreD.more += 1;
    moreD.more += moreD.more*2/3;
    // // // ,moreD.another_number
    msg!("More of more data  {} another number ",moreD.more);
    // msg!("Writeing a log here ");
    moreD.serialize(&mut &mut account.data.borrow_mut()[..])?;

    Ok(())
}
// npm run build:program-rust 
// solana program deploy dist/program/helloworld.so  
//  npm run start 


