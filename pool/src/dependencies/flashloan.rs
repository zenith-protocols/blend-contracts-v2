// Flash loan receiver interface, as defined by xycLoans' modified ERC-3156
// (https://github.com/xycloo/xycloans/tree/main/moderc3156).
use soroban_sdk::{contractclient, Address, Env};

#[allow(dead_code)]
#[contractclient(name = "FlashLoanClient")]
pub trait ModErc3156 {
    fn exec_op(env: Env, caller: Address, token: Address, amount: i128, fee: i128);
}
