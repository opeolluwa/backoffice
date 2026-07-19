use serde::{Deserialize, Serialize};

/// Command used to initialize a payment with a payment provider.
///
/// # Examples
///
/// ```
/// use your_crate::InitializePaymentCommand;
///
/// let command = InitializePaymentCommand::builder()
///     .email("john@example.com")
///     .amount(500_000)
///     .currency("NGN")
///     .build();
///
/// assert_eq!(command.email, "john@example.com");
/// assert_eq!(command.amount, 500_000);
/// assert_eq!(command.currency.as_deref(), Some("NGN"));
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InitializePaymentCommand {
    pub email: String,
    /// Amount in the smallest currency unit (e.g. kobo for NGN).
    pub amount: u64,
    pub currency: Option<String>,
}

#[derive(Debug, Default)]
pub struct InitializePaymentCommandBuilder {
    email: String,
    amount: u64,
    currency: Option<String>,
}

impl InitializePaymentCommand {
    /// Creates a builder for constructing an [`InitializePaymentCommand`].
    pub fn builder() -> InitializePaymentCommandBuilder {
        InitializePaymentCommandBuilder::default()
    }
}

impl InitializePaymentCommandBuilder {
    pub fn email(mut self, email: impl Into<String>) -> Self {
        self.email = email.into();
        self
    }

    pub fn amount(mut self, amount: u64) -> Self {
        self.amount = amount;
        self
    }

    pub fn currency(mut self, currency: impl Into<String>) -> Self {
        self.currency = Some(currency.into());
        self
    }

    pub fn build(self) -> InitializePaymentCommand {
        InitializePaymentCommand {
            email: self.email,
            amount: self.amount,
            currency: self.currency,
        }
    }
}

/// Standard response returned by a payment provider after initializing a
/// payment.
///
/// # Examples
///
/// ```
/// use your_crate::InitializePaymentResponse;
///
/// let response = InitializePaymentResponse::builder()
///     .status(true)
///     .message("Authorization URL created")
///     .checkout_url("https://checkout.paystack.com/nkdks46nymizns7")
///     .access_code("nkdks46nymizns7")
///     .reference("nms6uvr1pl")
///     .build();
///
/// assert!(response.status);
/// assert_eq!(response.message, "Authorization URL created");
/// assert_eq!(
///     response.checkout_url.as_deref(),
///     Some("https://checkout.paystack.com/nkdks46nymizns7")
/// );
/// assert_eq!(
///     response.access_code.as_deref(),
///     Some("nkdks46nymizns7")
/// );
/// assert_eq!(response.reference.as_deref(), Some("nms6uvr1pl"));
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InitializePaymentResponse {
    pub status: bool,
    pub message: String,
    pub checkout_url: Option<String>,
    pub access_code: Option<String>,
    pub reference: Option<String>,
}

#[derive(Debug, Default)]
pub struct InitializePaymentResponseBuilder {
    status: bool,
    message: String,
    checkout_url: Option<String>,
    access_code: Option<String>,
    reference: Option<String>,
}

impl InitializePaymentResponse {
    /// Creates a builder for constructing a [`InitializePaymentResponse`].
    pub fn builder() -> InitializePaymentResponseBuilder {
        InitializePaymentResponseBuilder::default()
    }
}

impl InitializePaymentResponseBuilder {
    pub fn status(mut self, status: bool) -> Self {
        self.status = status;
        self
    }

    pub fn message(mut self, message: impl Into<String>) -> Self {
        self.message = message.into();
        self
    }

    pub fn checkout_url(mut self, url: impl Into<String>) -> Self {
        self.checkout_url = Some(url.into());
        self
    }

    pub fn access_code(mut self, code: impl Into<String>) -> Self {
        self.access_code = Some(code.into());
        self
    }

    pub fn reference(mut self, reference: impl Into<String>) -> Self {
        self.reference = Some(reference.into());
        self
    }

    pub fn build(self) -> InitializePaymentResponse {
        InitializePaymentResponse {
            status: self.status,
            message: self.message,
            checkout_url: self.checkout_url,
            access_code: self.access_code,
            reference: self.reference,
        }
    }
}