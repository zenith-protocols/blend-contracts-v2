use soroban_sdk::{contractevent, Address, Env};

#[contractevent(topics = ["deposit"], data_format = "vec")]
struct Deposit {
    #[topic]
    pool_address: Address,
    #[topic]
    from: Address,
    tokens_in: i128,
    backstop_shares_minted: i128,
}

#[contractevent(topics = ["queue_withdrawal"], data_format = "vec")]
struct QueueWithdrawal {
    #[topic]
    pool_address: Address,
    #[topic]
    from: Address,
    amount: i128,
    expiration: u64,
}

#[contractevent(topics = ["dequeue_withdrawal"], data_format = "single-value")]
struct DequeueWithdrawal {
    #[topic]
    pool_address: Address,
    #[topic]
    from: Address,
    amount: i128,
}

#[contractevent(topics = ["withdraw"], data_format = "vec")]
struct Withdraw {
    #[topic]
    pool_address: Address,
    #[topic]
    from: Address,
    amount: i128,
    tokens_out: i128,
}

#[contractevent(topics = ["distribute"], data_format = "single-value")]
struct Distribute {
    new_tokens_emitted: i128,
}

#[contractevent(topics = ["gulp_emissions"], data_format = "vec")]
struct GulpEmissions {
    #[topic]
    pool_address: Address,
    new_backstop_emissions: i128,
    new_pool_emissions: i128,
}

#[contractevent(topics = ["rw_zone_add"], data_format = "vec")]
struct RwZoneAdd {
    to_add: Address,
    to_remove: Option<Address>,
}

#[contractevent(topics = ["rw_zone_remove"], data_format = "single-value")]
struct RwZoneRemove {
    to_remove: Address,
}

#[contractevent(topics = ["claim"], data_format = "single-value")]
struct Claim {
    #[topic]
    from: Address,
    amount: i128,
}

#[contractevent(topics = ["draw"], data_format = "vec")]
struct Draw {
    #[topic]
    pool_address: Address,
    to: Address,
    amount: i128,
}

#[contractevent(topics = ["donate"], data_format = "single-value")]
struct Donate {
    #[topic]
    pool_address: Address,
    #[topic]
    from: Address,
    amount: i128,
}

pub struct BackstopEvents {}

impl BackstopEvents {
    /// Emitted when tokens are deposited into a backstop
    ///
    /// - topics - `["deposit", pool_address: Address, from: Address]`
    /// - data - `[tokens_in: i128, backstop_shares_minted: i128]`
    ///
    /// ### Arguments
    /// * `pool_address` - The address of the pool
    /// * `from` - The address of the user depositing tokens
    /// * `tokens_in` - The amount of tokens sent to the backstop
    /// * `backstop_shares_minted` - The amount of backstop shares minted
    pub fn deposit(
        e: &Env,
        pool_address: Address,
        from: Address,
        tokens_in: i128,
        backstop_shares_minted: i128,
    ) {
        Deposit {
            pool_address,
            from,
            tokens_in,
            backstop_shares_minted,
        }
        .publish(e);
    }

    /// Emitted when a withdrawal is queued
    ///
    /// - topics - `["queue_withdrawal", pool_address: Address, from: Address]`
    /// - data - `[amount: i128, expiration: u64]`
    ///
    /// ### Arguments
    /// * `pool_address` - The address of the pool
    /// * `from` - The address of the user queuing the withdrawal
    /// * `amount` - The amount of shares being queued for withdrawal
    /// * `expiration` - The expiration timestamp of the withdrawal request
    pub fn queue_withdrawal(
        e: &Env,
        pool_address: Address,
        from: Address,
        amount: i128,
        expiration: u64,
    ) {
        QueueWithdrawal {
            pool_address,
            from,
            amount,
            expiration,
        }
        .publish(e);
    }

    /// Emitted when a withdrawal is dequeued
    ///
    /// - topics - `["dequeue_withdrawal", pool_address: Address, from: Address]`
    /// - data - `[amount: i128]`
    ///
    /// ### Arguments
    /// * `pool_address` - The address of the pool
    /// * `from` - The address of the user dequeuing the withdrawal
    /// * `amount` - The amount of shares being dequeued
    pub fn dequeue_withdrawal(e: &Env, pool_address: Address, from: Address, amount: i128) {
        DequeueWithdrawal {
            pool_address,
            from,
            amount,
        }
        .publish(e);
    }

    /// Emitted when tokens are withdrawn from the backstop
    ///
    /// - topics - `["withdraw", pool_address: Address, from: Address]`
    /// - data - `[amount: i128, tokens_out: i128]`
    ///
    /// ### Arguments
    /// * `pool_address` - The address of the pool
    /// * `from` - The address of the user withdrawing tokens
    /// * `amount` - The amount of backstop shares being burned
    /// * `tokens_out` - The amount of tokens being withdrawn
    pub fn withdraw(e: &Env, pool_address: Address, from: Address, amount: i128, tokens_out: i128) {
        Withdraw {
            pool_address,
            from,
            amount,
            tokens_out,
        }
        .publish(e);
    }

    /// Emitted when new emissions are distributed
    /// - topics - `["distribute"]`
    /// - data - `[new_tokens_emitted: i128]`
    ///
    /// ### Arguments
    /// * `new_tokens_emitted` - The amount of new tokens emitted
    pub fn distribute(e: &Env, new_tokens_emitted: i128) {
        Distribute { new_tokens_emitted }.publish(e);
    }

    /// Emitted when new emissions are gulped
    ///
    /// - topics - `["gulp_emissions", pool_address: Address]`
    /// - data - `[new_backstop_emissions: i128, new_pool_emissions: i128]`
    ///
    /// ### Arguments
    /// * `pool_address` - The address of the pool that gulped emissions
    /// * `new_backstop_emissions` - The amount of new emissions for the backstop
    /// * `new_pool_emissions` - The amount of new emissions for the pool
    pub fn gulp_emissions(
        e: &Env,
        pool_address: Address,
        new_backstop_emissions: i128,
        new_pool_emissions: i128,
    ) {
        GulpEmissions {
            pool_address,
            new_backstop_emissions,
            new_pool_emissions,
        }
        .publish(e);
    }

    /// Emitted when the reward zone is updated
    ///
    /// - topics - `["rw_zone_add"]`
    /// - data - `[to_add: Address, to_remove: Address]`
    ///
    /// ### Arguments
    /// * `to_add` - The address to add to the reward zone
    /// * `to_remove` - The address to remove from the reward zone
    pub fn rw_zone_add(e: &Env, to_add: Address, to_remove: Option<Address>) {
        RwZoneAdd { to_add, to_remove }.publish(e);
    }

    /// Emitted when a pool is removed from the reward zone
    ///
    /// - topics - `["rw_zone_remove", pool_address: Address]`
    /// - data - `[to_remove: Address]`
    ///
    /// ### Arguments
    /// * `to_remove` - The address to remove from the reward zone
    pub fn rw_zone_remove(e: &Env, to_remove: Address) {
        RwZoneRemove { to_remove }.publish(e);
    }

    /// Emitted when emissions are claimed
    ///
    /// - topics - `["claim", from: Address]`
    /// - data - `[amount: i128]`
    ///
    /// ### Arguments
    /// * `from` - The address of the user claiming emissions
    /// * `amount` - The amount of LP tokens minted
    pub fn claim(e: &Env, from: Address, amount: i128) {
        Claim { from, amount }.publish(e);
    }

    /// Emitted when tokens are drawn from the backstop
    ///
    /// - topics - `["draw", pool_address: Address]`
    /// - data - `[to: Address, amount: i128]`
    ///
    /// ### Arguments
    /// * `pool_address` - The address of the pool
    /// * `to` - The address receiving the drawn tokens
    /// * `amount` - The amount of tokens drawn
    pub fn draw(e: &Env, pool_address: Address, to: Address, amount: i128) {
        Draw {
            pool_address,
            to,
            amount,
        }
        .publish(e);
    }

    /// Emitted when tokens are donated to the backstop
    ///
    /// - topics - `["donate", pool_address: Address, from: Address]`
    /// - data - `[amount: i128]`
    ///
    /// ### Arguments
    /// * `pool_address` - The address of the pool
    /// * `from` - The address of the donor
    /// * `amount` - The amount of tokens donated
    pub fn donate(e: &Env, pool_address: Address, from: Address, amount: i128) {
        Donate {
            pool_address,
            from,
            amount,
        }
        .publish(e);
    }
}
