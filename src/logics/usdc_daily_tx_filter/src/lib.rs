use usdc_daily_tx_filter_accessors::*;
#[derive(Clone, Debug, Default, candid :: CandidType, serde :: Deserialize, serde :: Serialize)]
pub struct LensValue {
    pub dummy: u64,
}
pub type CalculateArgs = (u64, u64);

pub async fn calculate(targets: Vec<String>, args: CalculateArgs) -> LensValue {
    let results =
        get_events_from_to_in_usdc_transfer_indexer(targets.get(0usize).unwrap().clone(), args)
            .await
            .unwrap();

    
}
