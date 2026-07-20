use crate::{
    adapter::{InitializePaymentCommand, InitializePaymentResponse},
    error::PaymentError,
};

pub trait PaymnetProviderTrait {
    fn initialize_payment(
        &self,
        command: InitializePaymentCommand,
    ) -> impl std::future::Future<Output = Result<InitializePaymentResponse, PaymentError>> + Send;
}
