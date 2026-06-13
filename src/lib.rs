pub trait CrossShardValidator {
    fn verify_transit_payload(payload_id: [u8; 32]) -> Result<bool, &'static str>;
    fn trigger_atomic_swap_handshake(target_shard: u64) -> u32;
    fn monitor_sync_drift_guard(shard_state: &[u8]) -> bool;
}

pub trait LiquidityBufferManager {
    fn allocate_buffer_matrix(amount: u128, asset_ticker: &[u8; 4]) -> Result<(), &'static str>;
    fn execute_atomic_escrow_release(signature: &[u8; 64]) -> bool;
}
