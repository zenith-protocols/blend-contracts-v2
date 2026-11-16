use soroban_sdk::{Address, Env};

mod emitter_contract {
    soroban_sdk::contractimport!(file = "../emitter_v1.wasm");
}
pub use emitter_contract::{Client as EmitterClient, WASM as EmitterWASM};

pub fn create_emitter<'a>(e: &Env) -> (Address, EmitterClient<'a>) {
    let contract_id = e.register(EmitterWASM, ());
    (contract_id.clone(), EmitterClient::new(e, &contract_id))
}
