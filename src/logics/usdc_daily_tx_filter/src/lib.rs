use usdc_daily_tx_filter_accessors::*;

pub type LensValue = (
    Vec<String>,
    Vec<String>,
    Vec<u128>,
    Vec<u64>,
);
pub type CalculateArgs = (u64, u64);

pub async fn calculate(targets: Vec<String>, args: CalculateArgs) -> LensValue {
    let results =
        get_events_from_to_in_usdc_transfer_indexer(targets.get(0usize).unwrap().clone(), args)
            .await
            .unwrap();

    results
        .iter()
        .map(|r| {
            let block = r.0;
            let events = &r.1;
            let e: Vec<(String, String, u128, u64)> = events
            .iter()
            .map(|e| {
                (
                    e.from.clone(),
                    e.to.clone(),
                    e.value.value.parse().unwrap(),
                    block,
                )
            })
            .collect();
            e
        })
        .flatten()
        .map(|e| (e.0, e.1, e.2, e.3))
        .fold(
            (vec![], vec![], vec![], vec![]),
            |mut acc, e| {
                acc.0.push(e.0);
                acc.1.push(e.1);
                acc.2.push(e.2);
                acc.3.push(e.3.into());
                acc
            },
        )
}