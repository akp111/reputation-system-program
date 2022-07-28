use anchor_lang::prelude::*;

declare_id!("DKocEci6nnEQUCF3MihQRZiUKXLqSJWihjbdz91uqB7r");

#[program]
pub mod myepicproject {
  use super::*;
  pub fn start_stuff_off(ctx: Context<StartStuffOff>) -> Result <()> {
    let base_account = &mut ctx.accounts.base_account;
    base_account.total_accounts = 0;
    Ok(())
  }

  // The function now accepts a gif_link param from the user. We also reference the user from the Context
  pub fn add_profile(ctx: Context<AddProfile>, github_profile_link: String, ipfs_hash : String) -> Result <()> {
    let base_account = &mut ctx.accounts.base_account;
    let user = &mut ctx.accounts.user;

	// Build the struct.
    let item = ItemStruct {
      github_profile_link: github_profile_link.to_string(),
      ipfs_hash: ipfs_hash.to_string(),
      user_address: *user.to_account_info().key,
    };
		
	// Add it to the gif_list vector.
    base_account.profile_list.push(item);
    base_account.total_accounts += 1;
    Ok(())
  }
}

#[derive(Accounts)]
pub struct StartStuffOff<'info> {
  #[account(init, payer = user, space = 9000)]
  pub base_account: Account<'info, BaseAccount>,
  #[account(mut)]
  pub user: Signer<'info>,
  pub system_program: Program <'info, System>,
}

// Add the signer who calls the AddGif method to the struct so that we can save it
#[derive(Accounts)]
pub struct AddProfile<'info> {
  #[account(mut)]
  pub base_account: Account<'info, BaseAccount>,
  #[account(mut)]
  pub user: Signer<'info>,
}

// Create a custom struct for us to work with.
#[derive(Debug, Clone, AnchorSerialize, AnchorDeserialize)]
pub struct ItemStruct {
    pub github_profile_link: String,
    pub ipfs_hash: String,
    pub user_address: Pubkey,
}

#[account]
pub struct BaseAccount {
    pub total_accounts: u64,
	// Attach a Vector of type ItemStruct to the account.
    pub profile_list: Vec<ItemStruct>,
}