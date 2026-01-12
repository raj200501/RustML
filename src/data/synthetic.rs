use crate::data::dataset::Dataset;
use crate::math::random::DeterministicRng;

#[derive(Debug, Clone, Copy)]
pub struct SyntheticConfig {
    pub rows: usize,
    pub seed: u64,
}

impl Default for SyntheticConfig {
    fn default() -> Self {
        SyntheticConfig { rows: 256, seed: 42 }
    }
}

pub fn generate_network_dataset(config: SyntheticConfig) -> Dataset {
    let mut rng = DeterministicRng::new(config.seed);
    let mut data = Vec::with_capacity(config.rows);
    let mut target = Vec::with_capacity(config.rows);
    for i in 0..config.rows {
        let flow_duration = rng.gen_range(1.0, 2500.0);
        let src_bytes = rng.gen_range(64.0, 1024.0);
        let dst_bytes = rng.gen_range(64.0, 4096.0);
        let packet_rate = rng.gen_range(1.0, 120.0);
        let jitter = rng.gen_range(0.1, 20.0);
        let loss_rate = rng.gen_range(0.0, 0.2);
        let congestion_score =
            0.3 * packet_rate + 0.2 * jitter + 1.5 * loss_rate + (i % 5) as f64 * 0.1;
        data.push(vec![
            flow_duration,
            src_bytes,
            dst_bytes,
            packet_rate,
            jitter,
            loss_rate,
        ]);
        target.push(congestion_score);
    }
    Dataset::from_records(
        vec![
            "flow_duration".into(),
            "src_bytes".into(),
            "dst_bytes".into(),
            "packet_rate".into(),
            "jitter".into(),
            "loss_rate".into(),
        ],
        "congestion_score",
        data,
        target,
    )
    .expect("synthetic dataset should be valid")
}
