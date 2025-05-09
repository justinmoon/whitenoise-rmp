use crate::types::EnrichedContact;
use crate::whitenoise::Whitenoise;
use nostr_sdk::prelude::*;
use std::collections::HashMap;
use std::sync::Arc;

pub async fn search_for_enriched_contacts(
    query: String,
    wn: Arc<Whitenoise>,
) -> Result<HashMap<String, EnrichedContact>, String> {
    let enriched_users = wn
        .nostr
        .search_users(query, wn.clone())
        .await
        .map_err(|e| e.to_string())?;

    Ok(enriched_users)
}
