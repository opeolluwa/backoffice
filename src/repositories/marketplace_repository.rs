//! Marketplace repository module
//! The Marketplace contains different products, see it as a catalogue of items.
//!
use std::sync::Arc;

use sqlx::PgPool;

use crate::repositories::base::Repository;

#[derive(Debug, Clone)]
pub struct MarketplaceRepository {
    pool: Arc<PgPool>,
}

impl Repository for MarketplaceRepository {
    fn init(pool: &PgPool) -> Self {
        Self {
            pool: Arc::new(pool.clone()),
        }
    }
}
