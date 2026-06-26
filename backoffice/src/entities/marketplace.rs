use rust_decimal::Decimal;
use sea_orm::prelude::DateTimeWithTimeZone;
use serde::{Deserialize, Serialize};

use crate::entities::{countries, marketplaces, products};

#[derive(Debug, Serialize, Deserialize)]
pub struct ProductWithCurrency {
    pub identifier: String,
    pub name: String,
    pub price: Decimal,
    pub description: String,
    pub picture: Option<String>,
    pub created_by_identifier: Option<String>,
    pub marketplace_identifier: Option<String>,
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: Option<DateTimeWithTimeZone>,
    pub currency_code: Option<String>,
    pub currency: Option<String>,
    pub country: Option<String>,
    pub flag: Option<String>,
    pub currency_identifier: Option<String>,
}

impl ProductWithCurrency {
    pub fn from_models(product: products::Model, country: Option<countries::Model>) -> Self {
        Self {
            identifier: product.identifier,
            name: product.name,
            price: product.price,
            description: product.description,
            picture: product.picture,
            created_by_identifier: product.created_by_identifier,
            marketplace_identifier: product.marketplace_identifier,
            created_at: product.created_at,
            updated_at: product.updated_at,
            currency_code: country.as_ref().map(|c| c.currency_code.clone()),
            currency: country.as_ref().map(|c| c.currency.clone()),
            country: country.as_ref().map(|c| c.country.clone()),
            flag: country.as_ref().and_then(|c| c.flag.clone()),
            currency_identifier: country.map(|c| c.identifier),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MarketplaceWithProducts {
    pub identifier: String,
    pub user_identifier: Option<String>,
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: Option<DateTimeWithTimeZone>,
    pub name: String,
    pub description: String,
    pub slug: String,
    pub products: Vec<ProductWithCurrency>,
}

impl MarketplaceWithProducts {
    pub fn from_marketplace(
        marketplace: marketplaces::Model,
        products: Vec<ProductWithCurrency>,
    ) -> Self {
        Self {
            identifier: marketplace.identifier,
            user_identifier: marketplace.user_identifier,
            created_at: marketplace.created_at,
            updated_at: marketplace.updated_at,
            name: marketplace.name,
            description: marketplace.description,
            slug: marketplace.slug,
            products,
        }
    }
}
