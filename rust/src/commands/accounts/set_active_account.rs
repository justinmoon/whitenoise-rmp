use crate::accounts::Account;
use crate::runtime;
use crate::whitenoise::Whitenoise;
use nostr_sdk::prelude::*;
use std::sync::Arc;

/// Sets the active account.
///
/// # Arguments
///
/// * `wn` - A reference to the Whitenoise state.
/// * `hex_pubkey` - The public key in hexadecimal format.
///
/// # Returns
///
/// * `Ok(())` - If the active account was set successfully.
/// * `Err(String)` - An error message if there was an issue setting the active account.

pub async fn set_active_account(
    hex_pubkey: String,
    wn: Arc<Whitenoise>,
) -> Result<Account, String> {
    tracing::debug!(target: "whitenoise::commands::accounts", "Setting active account: {}", hex_pubkey);

    let pubkey =
        PublicKey::parse(&hex_pubkey).map_err(|e| format!("Error parsing public key: {}", e))?;

    let mut account = Account::find_by_pubkey(&pubkey, wn.clone())
        .await
        .map_err(|e| format!("Error fetching account: {}", e))?;

    account.active = true;
    account
        .set_active(runtime::wn())
        .await
        .map_err(|e| format!("Error setting active account: {}", e));
    Ok(account)
}
