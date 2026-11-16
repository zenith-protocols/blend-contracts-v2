use soroban_sdk::{contractevent, Address, Env, Vec};

use crate::{AuctionData, ReserveConfig};

#[contractevent(topics = ["set_admin"], data_format = "single-value")]
struct SetAdmin {
    #[topic]
    admin: Address,
    new_admin: Address,
}

#[contractevent(topics = ["update_pool"], data_format = "vec")]
struct UpdatePool {
    #[topic]
    admin: Address,
    backstop_take_rate: u32,
    max_positions: u32,
    min_collateral: i128,
}

#[contractevent(topics = ["queue_set_reserve"], data_format = "vec")]
struct QueueSetReserve {
    #[topic]
    admin: Address,
    asset: Address,
    metadata: ReserveConfig,
}

#[contractevent(topics = ["cancel_set_reserve"], data_format = "single-value")]
struct CancelSetReserve {
    #[topic]
    admin: Address,
    asset: Address,
}

#[contractevent(topics = ["set_reserve"], data_format = "vec")]
struct SetReserve {
    asset: Address,
    index: u32,
}

#[contractevent(topics = ["set_status"], data_format = "single-value")]
struct SetStatus {
    new_status: u32,
}

#[contractevent(topics = ["set_status"], data_format = "single-value")]
struct SetStatusAdmin {
    #[topic]
    admin: Address,
    pool_status: u32,
}

#[contractevent(topics = ["reserve_emission_update"], data_format = "vec")]
struct ReserveEmissionUpdate {
    res_token_id: u32,
    eps: u64,
    expiration: u64,
}

#[contractevent(topics = ["gulp_emissions"], data_format = "single-value")]
struct GulpEmissions {
    emissions: i128,
}

#[contractevent(topics = ["claim"], data_format = "vec")]
struct Claim {
    #[topic]
    from: Address,
    reserve_token_ids: Vec<u32>,
    amount_claimed: i128,
}

#[contractevent(topics = ["bad_debt"], data_format = "single-value")]
struct BadDebt {
    #[topic]
    user: Address,
    #[topic]
    asset: Address,
    d_tokens: i128,
}

#[contractevent(topics = ["defaulted_debt"], data_format = "single-value")]
struct DefaultedDebt {
    #[topic]
    asset: Address,
    d_tokens_burnt: i128,
}

#[contractevent(topics = ["supply"], data_format = "vec")]
struct Supply {
    #[topic]
    asset: Address,
    #[topic]
    from: Address,
    tokens_in: i128,
    b_tokens_minted: i128,
}

#[contractevent(topics = ["withdraw"], data_format = "vec")]
struct Withdraw {
    #[topic]
    asset: Address,
    #[topic]
    from: Address,
    tokens_out: i128,
    b_tokens_burnt: i128,
}

#[contractevent(topics = ["supply_collateral"], data_format = "vec")]
struct SupplyCollateral {
    #[topic]
    asset: Address,
    #[topic]
    from: Address,
    tokens_in: i128,
    b_tokens_minted: i128,
}

#[contractevent(topics = ["withdraw_collateral"], data_format = "vec")]
struct WithdrawCollateral {
    #[topic]
    asset: Address,
    #[topic]
    from: Address,
    tokens_out: i128,
    b_tokens_burnt: i128,
}

#[contractevent(topics = ["borrow"], data_format = "vec")]
struct Borrow {
    #[topic]
    asset: Address,
    #[topic]
    from: Address,
    tokens_out: i128,
    d_tokens_minted: i128,
}

#[contractevent(topics = ["repay"], data_format = "vec")]
struct Repay {
    #[topic]
    asset: Address,
    #[topic]
    from: Address,
    tokens_in: i128,
    d_tokens_burnt: i128,
}

#[contractevent(topics = ["flash_loan"], data_format = "vec")]
struct FlashLoanEvent {
    #[topic]
    asset: Address,
    #[topic]
    from: Address,
    #[topic]
    contract: Address,
    tokens_out: i128,
    d_tokens_minted: i128,
}

#[contractevent(topics = ["gulp"], data_format = "single-value")]
struct Gulp {
    #[topic]
    asset: Address,
    token_delta: i128,
}

#[contractevent(topics = ["new_auction"], data_format = "vec")]
struct NewAuction {
    #[topic]
    auction_type: u32,
    #[topic]
    user: Address,
    percent: u32,
    auction_data: AuctionData,
}

#[contractevent(topics = ["fill_auction"], data_format = "vec")]
struct FillAuction {
    #[topic]
    auction_type: u32,
    #[topic]
    user: Address,
    filler: Address,
    fill_percent: i128,
    filled_auction_data: AuctionData,
}

#[contractevent(topics = ["delete_auction"], data_format = "single-value")]
struct DeleteAuction {
    #[topic]
    auction_type: u32,
    #[topic]
    user: Address,
}

pub struct PoolEvents {}

impl PoolEvents {
    /// Emitted when a new admin is set for a pool
    ///
    /// - topics - `["set_admin", admin: Address]`
    /// - data - `new_admin: Address`
    ///
    /// ### Arguments
    /// * admin - The current admin of the pool
    /// * new_admin - The new admin of the pool
    pub fn set_admin(e: &Env, admin: Address, new_admin: Address) {
        SetAdmin { admin, new_admin }.publish(e);
    }

    /// Emitted when pool parameters are updated
    ///
    /// - topics - `["update_pool", admin: Address]`
    /// - data - `[backstop_take_rate: u32, max_positions: u32, min_collateral: i128]`
    ///
    /// ### Arguments
    /// * admin - The current admin of the pool
    /// * backstop_take_rate - The new backstop take rate
    /// * max_positions - The new maximum number of positions
    pub fn update_pool(
        e: &Env,
        admin: Address,
        backstop_take_rate: u32,
        max_positions: u32,
        min_collateral: i128,
    ) {
        UpdatePool {
            admin,
            backstop_take_rate,
            max_positions,
            min_collateral,
        }
        .publish(e);
    }

    /// Emitted when a new reserve configuration change is queued
    ///
    /// - topics - `["queue_set_reserve", admin: Address]`
    /// - data - `[asset: Address, metadata: ReserveMetadata]`
    ///
    /// ### Arguments
    /// * admin - The current admin of the pool
    /// * asset - The asset to change the reserve configuration of
    /// * metadata - The new reserve configuration
    pub fn queue_set_reserve(e: &Env, admin: Address, asset: Address, metadata: ReserveConfig) {
        QueueSetReserve {
            admin,
            asset,
            metadata,
        }
        .publish(e);
    }

    /// Emitted when a queued reserve configuration change is cancelled
    ///
    /// - topics - `["cancel_set_reserve", admin: Address]`
    /// - data - `asset: Address`
    ///
    /// ### Arguments
    /// * admin - The current admin of the pool
    /// * asset - The asset to cancel the reserve configuration change of
    pub fn cancel_set_reserve(e: &Env, admin: Address, asset: Address) {
        CancelSetReserve { admin, asset }.publish(e);
    }

    /// Emitted when a reserve configuration change is set
    ///
    /// - topics - `["set_reserve"]`
    /// - data - `[asset: Address, index: u32]`
    ///
    /// ### Arguments
    /// * asset - The asset to change the reserve configuration of
    /// * index - The reserve index
    pub fn set_reserve(e: &Env, asset: Address, index: u32) {
        SetReserve { asset, index }.publish(e);
    }

    /// Emitted when pool status is updated (non-admin)
    ///
    /// - topics - `["set_status"]`
    /// - data - `new_status: PoolStatus`
    ///
    /// ### Arguments
    /// * new_status - The new pool status
    pub fn set_status(e: &Env, new_status: u32) {
        SetStatus { new_status }.publish(e);
    }

    /// Emitted when pool status is updated by admin
    ///
    /// - topics - `["set_status", admin: Address]`
    /// - data - `pool_status: PoolStatus`
    ///
    /// ### Arguments
    /// * admin - The admin setting the pool status
    /// * pool_status - The new pool status
    pub fn set_status_admin(e: &Env, admin: Address, pool_status: u32) {
        SetStatusAdmin { admin, pool_status }.publish(e);
    }

    /// Emitted when reserve emissions are updated
    ///
    /// - topics - `["reserve_emission_update"]`
    /// - data - `[res_token_id: u32, eps: u64, expiration: u64]`
    ///
    /// ### Arguments
    /// * res_token_id - The reserve token ID
    /// * eps - The new emissions per second
    /// * expiration - The new expiration time
    pub fn reserve_emission_update(e: &Env, res_token_id: u32, eps: u64, expiration: u64) {
        ReserveEmissionUpdate {
            res_token_id,
            eps,
            expiration,
        }
        .publish(e);
    }

    /// Emitted when emissions are gulped
    ///
    /// - topics - `["gulp_emissions"]`
    /// - data - `emissions: i128`
    ///
    /// ### Arguments
    /// * emissions - The amount of emissions gulped
    pub fn gulp_emissions(e: &Env, emissions: i128) {
        GulpEmissions { emissions }.publish(e);
    }

    /// Emitted when emissions are claimed
    ///
    /// - topics - `["claim", from: Address]`
    /// - data - `[reserve_token_ids: Vec<u32>, amount_claimed: i128]`
    ///
    /// ### Arguments
    /// * from - The address claiming the emissions
    /// * reserve_token_ids - The reserve token IDs claimed
    /// * amount_claimed - The amount claimed
    pub fn claim(e: &Env, from: Address, reserve_token_ids: Vec<u32>, amount_claimed: i128) {
        Claim {
            from,
            reserve_token_ids,
            amount_claimed,
        }
        .publish(e);
    }

    /// Emitted when bad debt is recorded
    ///
    /// - topics - `["bad_debt", user: Address, asset: Address]`
    /// - data - `[d_tokens: i128]`
    ///
    /// ### Arguments
    /// * user - The user with bad debt
    /// * asset - The asset with bad debt
    /// * d_tokens - The amount of bad debt
    pub fn bad_debt(e: &Env, user: Address, asset: Address, d_tokens: i128) {
        BadDebt {
            user,
            asset,
            d_tokens,
        }
        .publish(e);
    }

    /// Emitted when bad debt is defaulted
    ///
    /// - topics - `["defaulted_debt", asset: Address]`
    /// - data - `[d_tokens_burnt: i128]`
    ///
    /// ### Arguments
    /// * asset - The asset with defaulted debt
    /// * d_tokens_burnt - The amount of defaulted d_tokens
    pub fn defaulted_debt(e: &Env, asset: Address, d_tokens_burnt: i128) {
        DefaultedDebt {
            asset,
            d_tokens_burnt,
        }
        .publish(e);
    }

    /// Emitted when tokens are supplied
    ///
    /// - topics - `["supply", asset: Address, from: Address]`
    /// - data - `[tokens_in: i128, b_tokens_minted: i128]`
    ///
    /// ### Arguments
    /// * asset - The asset
    /// * from - The address whose position is being modified
    /// * tokens_in - The amount of tokens sent to the pool
    /// * b_tokens_minted - The amount of b_tokens minted
    pub fn supply(e: &Env, asset: Address, from: Address, tokens_in: i128, b_tokens_minted: i128) {
        Supply {
            asset,
            from,
            tokens_in,
            b_tokens_minted,
        }
        .publish(e);
    }

    /// Emitted when tokens are withdrawn
    ///
    /// - topics - `["withdraw", asset: Address, from: Address]`
    /// - data - `[tokens_out: i128, b_tokens_burnt: i128]`
    ///
    /// ### Arguments
    /// * asset - The asset
    /// * from - The address whose position is being modified
    /// * tokens_out - The amount of tokens withdrawn from the pool
    /// * b_tokens_burnt - The amount of b_tokens burnt
    pub fn withdraw(
        e: &Env,
        asset: Address,
        from: Address,
        tokens_out: i128,
        b_tokens_burnt: i128,
    ) {
        Withdraw {
            asset,
            from,
            tokens_out,
            b_tokens_burnt,
        }
        .publish(e);
    }

    /// Emitted when collateral is supplied
    ///
    /// - topics - `["supply_collateral", asset: Address, from: Address]`
    /// - data - `[tokens_in: i128, b_tokens_minted: i128]`
    ///
    /// ### Arguments
    /// * asset - The asset
    /// * from - The address whose position is being modified
    /// * tokens_in - The amount of tokens sent to the pool
    /// * b_tokens_minted - The amount of b_tokens minted
    pub fn supply_collateral(
        e: &Env,
        asset: Address,
        from: Address,
        tokens_in: i128,
        b_tokens_minted: i128,
    ) {
        SupplyCollateral {
            asset,
            from,
            tokens_in,
            b_tokens_minted,
        }
        .publish(e);
    }

    /// Emitted when collateral is withdrawn
    ///
    /// - topics - `["withdraw_collateral", asset: Address, from: Address]`
    /// - data - `[tokens_out: i128, b_tokens_burnt: i128]`
    ///
    /// ### Arguments
    /// * asset - The asset
    /// * from - The address whose position is being modified
    /// * tokens_out - The amount of tokens withdrawn from the pool
    /// * b_tokens_burnt - The amount of b_tokens burnt
    pub fn withdraw_collateral(
        e: &Env,
        asset: Address,
        from: Address,
        tokens_out: i128,
        b_tokens_burnt: i128,
    ) {
        WithdrawCollateral {
            asset,
            from,
            tokens_out,
            b_tokens_burnt,
        }
        .publish(e);
    }

    /// Emitted when tokens are borrowed
    ///
    /// - topics - `["borrow", asset: Address, from: Address]`
    /// - data - `[tokens_out: i128, d_tokens_minted: i128]`
    ///
    /// ### Arguments
    /// * asset - The asset
    /// * from - The address whose position is being modified
    /// * tokens_out - The amount of tokens sent from the pool
    /// * d_tokens_burnt - The amount of d_tokens burnt
    pub fn borrow(e: &Env, asset: Address, from: Address, tokens_out: i128, d_tokens_minted: i128) {
        Borrow {
            asset,
            from,
            tokens_out,
            d_tokens_minted,
        }
        .publish(e);
    }

    /// Emitted when a loan is repaid
    ///
    /// - topics - `["repay", asset: Address, from: Address]`
    /// - data - `[tokens_in: i128, d_tokens_burnt: i128]`
    ///
    /// ### Arguments
    /// * asset - The asset
    /// * from - The address whose position is being modified
    /// * tokens_in - The amount of tokens sent to the pool
    /// * d_tokens_burnt - The amount of d_tokens burnt
    pub fn repay(e: &Env, asset: Address, from: Address, tokens_in: i128, d_tokens_burnt: i128) {
        Repay {
            asset,
            from,
            tokens_in,
            d_tokens_burnt,
        }
        .publish(e);
    }

    /// Emitted during a flash loan
    ///
    /// - topics - `["flash_loan", asset: Address, from: Address]`
    /// - data - `[tokens_out: i128, d_tokens_minted: i128]`
    ///
    /// ### Arguments
    /// * asset - The asset
    /// * from - The address whose position is being modified
    /// * contract - The address of the flash loan contract
    /// * tokens_out - The amount of tokens sent from the pool
    /// * d_tokens_burnt - The amount of d_tokens burnt
    pub fn flash_loan(
        e: &Env,
        asset: Address,
        from: Address,
        contract: Address,
        tokens_out: i128,
        d_tokens_minted: i128,
    ) {
        FlashLoanEvent {
            asset,
            from,
            contract,
            tokens_out,
            d_tokens_minted,
        }
        .publish(e);
    }

    /// Emitted when a reserve gulps excess tokens
    ///
    /// - topics - `["gulp", asset: Address]`
    /// - data - `[token_delta: i128]`
    ///
    /// ### Arguments
    /// * asset - The asset
    /// * token_delta - The number of tokens gulped
    pub fn gulp(e: &Env, asset: Address, token_delta: i128) {
        Gulp { asset, token_delta }.publish(e);
    }

    /// Emitted when a new auction is created
    ///
    /// - topics - `["new_auction", auction_type: u32, user: Address]`
    /// - data - `[percent: u32, auction_data: AuctionData]`
    ///
    /// ### Arguments
    /// * auction_type - The type of auction
    /// * user - The auction user
    /// * percent - The percent of assets auctioned off
    /// * auction_data - The auction data
    pub fn new_auction(
        e: &Env,
        auction_type: u32,
        user: Address,
        percent: u32,
        auction_data: AuctionData,
    ) {
        NewAuction {
            auction_type,
            user,
            percent,
            auction_data,
        }
        .publish(e);
    }

    /// Emitted when an auction is filled
    ///
    /// - topics - `["fill_auction", auction_type: u32, user: Address]`
    /// - data - `[filler: Address, fill_percent: i128, filled_auction_data: AuctionData]`
    ///
    /// ### Arguments
    /// * auction_type - The type of auction
    /// * user - The auction user
    /// * filler - The address of the filler
    /// * fill_percent - The percentage of the auction filled
    /// * filled_auction_data - The filled auction data
    pub fn fill_auction(
        e: &Env,
        auction_type: u32,
        user: Address,
        filler: Address,
        fill_percent: i128,
        filled_auction_data: AuctionData,
    ) {
        FillAuction {
            auction_type,
            user,
            filler,
            fill_percent,
            filled_auction_data,
        }
        .publish(e);
    }

    /// Emitted when an auction is deleted
    ///
    /// - topics - `["delete_auction", auction_type: u32, user: Address]`
    /// - data - `()`
    ///
    /// ### Arguments
    /// * auction_type - The type of auction
    /// * user - The address of the user
    pub fn delete_auction(e: &Env, auction_type: u32, user: Address) {
        DeleteAuction { auction_type, user }.publish(e);
    }
}
