use reqwest::{
    Method,
    header::{HeaderMap, HeaderValue},
};
use secrecy::{ExposeSecret, SecretString};
use serde::{Deserialize, Serialize};

use crate::{
    adapter::{InitializePaymentCommand, InitializePaymentResponse},
    error::PaymentError,
    port::PaymnetProviderTrait,
};

#[derive(Clone)]
pub struct PaystackClient {
    private_key: SecretString,
    http_client: reqwest::Client,
    base_url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PaystackInitializeTransactionResponse {
    pub status: bool,
    pub message: String,
    pub data: PaystackInitializeTransactionData,
}

impl Into<InitializePaymentResponse> for PaystackInitializeTransactionResponse {
    fn into(self) -> InitializePaymentResponse {
        InitializePaymentResponse {
            status: self.status,
            message: self.message,
            checkout_url: Some(self.data.authorization_url),
            access_code: Some(self.data.access_code),
            reference: Some(self.data.reference),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PaystackInitializeTransactionData {
    pub authorization_url: String,
    pub access_code: String,
    pub reference: String,
}

impl PaystackClient {
    pub fn new(private_key: &SecretString, base_url: &str) -> Self {
        Self {
            private_key: private_key.clone(),
            http_client: reqwest::Client::new(),
            base_url: base_url.to_string(),
        }
    }
}

impl PaymnetProviderTrait for PaystackClient {
    async fn initialize_payment(
        &self,
        command: InitializePaymentCommand,
    ) -> Result<InitializePaymentResponse, PaymentError> {
        let mut headers = HeaderMap::new();

        headers.insert(
            "Authorization",
            HeaderValue::from_str(&format!("Bearer {}", self.private_key.expose_secret()))
                .map_err(|err| PaymentError::UnexpectedError(err.to_string()))?,
        );

        headers.insert(
            "Content-Type",
            HeaderValue::from_str("Application/json")
                .map_err(|err| PaymentError::UnexpectedError(err.to_string()))?,
        );

        let response = self
            .http_client
            .request(Method::POST, &self.base_url)
            .headers(headers)
            .json(&command)
            .send()
            .await
            .map_err(|err| PaymentError::UnexpectedError(err.to_string()))?;

        if !response.status().is_success() {
            return Err(PaymentError::UnexpectedError(
                response.text().await.unwrap_or_default(),
            ));
        }

        let parsed = response
            .json::<PaystackInitializeTransactionResponse>()
            .await
            .map_err(|err| PaymentError::UnexpectedError(err.to_string()))?;

        Ok(parsed.into())
    }
}
