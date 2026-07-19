use crate::{
    adapter::{InitializePaymentCommand, InitializePaymentResponse},
    error::PaymentError,
};

pub trait PaymnetProviderTrait {
    async fn initialize_payment(
        &self,
        command: InitializePaymentCommand,
    ) -> Result<InitializePaymentResponse, PaymentError>;
}
