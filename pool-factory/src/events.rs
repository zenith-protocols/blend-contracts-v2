use soroban_sdk::{contractevent, Address, Env};

#[contractevent(topics = ["deploy"], data_format = "single-value")]
struct Deploy {
    pool_address: Address,
}

pub struct PoolFactoryEvents {}

impl PoolFactoryEvents {
    /// Emitted when a pool is deployed by the factory
    ///
    /// - topics - `["deploy"]`
    /// - data - `Address`
    ///
    /// ### Arguments
    /// * `pool_address` - The address of the pool
    pub fn deploy(e: &Env, pool_address: Address) {
        Deploy { pool_address }.publish(e);
    }
}
