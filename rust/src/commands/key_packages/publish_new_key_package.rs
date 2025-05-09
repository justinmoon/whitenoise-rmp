use crate::whitenoise::Whitenoise;
use std::sync::Arc;

/// Publishes a new MLS key package for the active account to Nostr

pub async fn publish_new_key_package(wn: Arc<Whitenoise>) -> Result<(), String> {
    crate::key_packages::publish_key_package(wn)
        .await
        .map_err(|e| e.to_string())
}
