use candid::Principal;
use ic_xrc_types::{GetExchangeRateRequest, GetExchangeRateResult};
pub struct XRC {
    principal: Principal,
}

impl XRC {
    pub fn new() -> Self {
        let principal = Principal::from_text("uf6dk-hyaaa-aaaaq-qaaaq-cai")
            .expect("Could not decode the principal.");
        XRC { principal }
    }

    pub async fn get_exchange_rate(&self, args: GetExchangeRateRequest) -> GetExchangeRateResult {
        ic_cdk::println!("req {:?}", args);
        let call_result: Result<(GetExchangeRateResult,), _> =
            ic_cdk::api::call::call_with_payment(
                self.principal,
                "get_exchange_rate",
                (args,),
                1_000_000_000,
            )
            .await;
        call_result.unwrap().0
    }
}
