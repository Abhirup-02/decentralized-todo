use anchor_lang::prelude::*;

pub mod constant;
pub mod states;
pub mod error;
use crate::{constant::*, error::*, states::*};

declare_id!("C9vSD5Fhc6Jm1BXeHZzYScpLqGrXneDVQ79GtP8sFW6p");

#[program]
pub mod abhi_todo {
    use super::*;

    // Add user profile to the blockchain
    pub fn initialize_user(ctx: Context<InitializeUser>) -> Result<()> {
        // Initialize User with default data
        let user_profile = &mut ctx.accounts.user_profile;
        user_profile.authority = ctx.accounts.authority.key();
        user_profile.last_todo = 0;
        user_profile.todo_count = 0;

        Ok(())
    }

    // Add a Todo
    pub fn add_todo(ctx: Context<AddTodo>, _content: String) -> Result<()> {
        // Fill todo account with proper values
        let todo_account = &mut ctx.accounts.todo_account;
        let user_profile = &mut ctx.accounts.user_profile;

        todo_account.authority = ctx.accounts.authority.key();
        todo_account.idx = user_profile.last_todo;
        todo_account.marked = false;
        todo_account.content = _content;

        // Increase todo idx
        user_profile.last_todo = user_profile.last_todo.checked_add(1).unwrap();

        // Increase total todo count
        user_profile.todo_count = user_profile.todo_count.checked_add(1).unwrap();

        Ok(())
    }

    // Mark a Todo
    pub fn mark_todo(ctx: Context<MarkTodo>, todo_idx: u8) -> Result<()> {
        // Change marked to TRUE
        let todo_account = &mut ctx.accounts.todo_account;
        require!(!todo_account.marked, FetchError::AlreadyMarked);

        todo_account.marked = true;

        Ok(())
    }

    
    // Delete Todo
    pub fn delete_todo(ctx: Context<RemoveTodo>, todo_idx: u8) -> Result<()> {
        // Decrement total todo count
        let user_profile = &mut ctx.accounts.user_profile;
        user_profile.todo_count = user_profile.todo_count
        .checked_sub(1)
        .unwrap();

        // No need to decrease last todo idx

        // Todo PDA already closed in context

        Ok(())
    }
}

#[derive(Accounts)]
#[instruction()]
pub struct InitializeUser<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        init,
        seeds = [USER_TAG, authority.key().as_ref()],
        bump,
        payer = authority,
        space = 8 + std::mem::size_of::<UserProfile>(),
    )]
    pub user_profile: Box<Account<'info, UserProfile>>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction()]
pub struct AddTodo<'info> {
    #[account(
        mut,
        seeds = [USER_TAG, authority.key().as_ref()],
        bump,
        has_one = authority,
    )]
    pub user_profile: Box<Account<'info, UserProfile>>,

    #[account(
        init,
        seeds = [TODO_TAG, authority.key().as_ref(), &[user_profile.last_todo as u8].as_ref()],
        bump,
        payer = authority,
        space = std::mem::size_of::<TodoAccount>() + 8,
    )]
    pub todo_account: Box<Account<'info, TodoAccount>>,

    #[account(mut)]
    pub authority: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(todo_idx: u8)]
pub struct MarkTodo<'info> {
    #[account(
        mut,
        seeds = [USER_TAG, authority.key().as_ref()],
        bump,
        has_one = authority,
    )]
    pub user_profile: Box<Account<'info, UserProfile>>,

    #[account(
        mut,
        seeds = [TODO_TAG, authority.key().as_ref(), &[todo_idx].as_ref()],
        bump,
        has_one = authority,
    )]
    pub todo_account: Box<Account<'info, TodoAccount>>,

    #[account(mut)]
    pub authority: Signer<'info>,

    pub system_program: Program<'info, System>,
}


#[derive(Accounts)]
#[instruction(todo_idx: u8)]
pub struct RemoveTodo<'info> {
    #[account(
        mut,
        seeds = [USER_TAG, authority.key().as_ref()],
        bump,
        has_one = authority,
    )]
    pub user_profile: Box<Account<'info, UserProfile>>,

   #[account(
       mut,
       close = authority,
       seeds = [TODO_TAG, authority.key().as_ref(), &[todo_idx].as_ref()],
       bump,
       has_one = authority,
   )] 
     pub todo_account: Box<Account<'info, TodoAccount>>,

    #[account(mut)]
    pub authority: Signer<'info>,

    pub system_program: Program<'info, System>,
}