use anchor_lang::prelude::*;

declare_id!("AqNLGb2kgHLUsCfjBfssNqarQXx9W2RuRx9v3PV3Fygt"); // devnet solana program id

#[program]
pub mod gifs {
    use super::*;
    pub fn initialize(ctx: Context<Initialize>) -> ProgramResult {
        let base_account = &mut ctx.accounts.base_account;

        base_account.total_gifs = 0;
        Ok(())
    }

    pub fn add_gif(ctx: Context<AddGif>, gif_link: String) -> ProgramResult {
        let base_account = &mut ctx.accounts.base_account;
        let user = &mut ctx.accounts.user;
        let clock = Clock::get().unwrap();

        let item = ItemStruct {
            uid: gif_link.to_string() + &clock.unix_timestamp.to_string(),
            gif_link: gif_link.to_string(),
            user_address: *user.to_account_info().key,
            votes: 0,
        };

        base_account.gif_list.push(item);
        base_account.total_gifs += 1;
        Ok(())
    }

    pub fn add_vote(ctx: Context<AddVote>, uid: String) -> ProgramResult {
        let base_account = &mut ctx.accounts.base_account;

        for item in base_account.gif_list.iter_mut() {
            if item.uid == uid {
                item.votes += 1;
            }
        }
        Ok(())
    }

    pub fn remove_gif(ctx: Context<RemoveGif>, uid: String) -> ProgramResult {
        let base_account = &mut ctx.accounts.base_account;
        let mut i = 0;
        while i < base_account.gif_list.len() {
            if base_account.gif_list[i].uid == uid {
                base_account.gif_list.remove(i);
            } else {
                i += 1;
            }
        }

        base_account.total_gifs -= 1;
        Ok(())
    }

    pub fn send_sol(ctx: Context<SendSol>, amount: u64) -> ProgramResult {
        let ix = anchor_lang::solana_program::system_instruction::transfer(
            &ctx.accounts.from.key(),
            &ctx.accounts.to.key(),
            amount,
        );
        anchor_lang::solana_program::program::invoke(
            &ix,
            &[
                ctx.accounts.from.to_account_info(),
                ctx.accounts.to.to_account_info(),
            ],
        )
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = user, space = 9000)]
    pub base_account: Account<'info, BaseAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct AddGif<'info> {
    #[account(mut)]
    pub base_account: Account<'info, BaseAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
}

#[derive(Accounts)]
pub struct AddVote<'info> {
    #[account(mut)]
    pub base_account: Account<'info, BaseAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
}

#[derive(Accounts)]
pub struct RemoveGif<'info> {
    #[account(mut)]
    pub base_account: Account<'info, BaseAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
}

#[derive(Debug, Clone, AnchorSerialize, AnchorDeserialize)]
pub struct ItemStruct {
    pub uid: String,
    pub gif_link: String,
    pub user_address: Pubkey,
    pub votes: u64,
}

#[derive(Accounts)]
pub struct SendSol<'info> {
    #[account(mut)]
    from: Signer<'info>,
    #[account(mut)]
    to: AccountInfo<'info>,
    system_program: Program<'info, System>,
}

#[account]
pub struct BaseAccount {
    pub total_gifs: u64,
    pub gif_list: Vec<ItemStruct>,
}
