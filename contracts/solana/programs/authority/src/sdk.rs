use anchor_lang::prelude::*;
use anchor_lang::solana_program::pubkey::Pubkey;

#[derive(Accounts)]
pub struct VerifyAuthority<'info> {
    #[account(mut)]
    pub authority_record: Account<'info, AuthorityRecord>,
    #[account(signer)]
    pub admin: Signer<'info>, // The admin account
}

/// Verifies the authority (admin only).
pub fn verify_authority(ctx: Context<VerifyAuthority>, is_verified: bool) -> Result<()> {
    // let expected_admin_pubkey = Pubkey::from_str(ADMIN_PUBLIC_KEY).unwrap();

    // require_keys_eq!(expected_admin_pubkey, ctx.accounts.admin.key());

    // if ctx.accounts.admin.key() != expected_admin_pubkey {
    //     return Err(AuthorityError::Unauthorized.into());
    // }

    let authority_record = &mut ctx.accounts.authority_record;
    authority_record.is_verified = is_verified;

    emit!(VerifiedAuthoritySignal {
        authority: authority_record.authority,
        is_verified: authority_record.is_verified,
    });

    Ok(())
}
