use soroban_sdk::{
    testutils::Events as _,
    xdr::{ContractEventBody, ScAddress, ScVal},
    Address, Env, TryFromVal, Val, Vec,
};

/// Return all contract events published by the last contract invocation as
/// `(contract, topics, data)` tuples, oldest first.
///
/// soroban-sdk 23+ returns an opaque `ContractEvents` from `env.events().all()`. This converts
/// it back into the list form the test assertions index into.
pub fn all_events(e: &Env) -> Vec<(Address, Vec<Val>, Val)> {
    let mut result = Vec::new(e);
    for event in e.events().all().events() {
        let contract_id = event
            .contract_id
            .clone()
            .expect("contract event without a contract id");
        let contract =
            Address::try_from_val(e, &ScVal::Address(ScAddress::Contract(contract_id))).unwrap();
        let ContractEventBody::V0(body) = &event.body;
        let mut topics = Vec::new(e);
        for topic in body.topics.iter() {
            topics.push_back(Val::try_from_val(e, topic).unwrap());
        }
        let data = Val::try_from_val(e, &body.data).unwrap();
        result.push_back((contract, topics, data));
    }
    result
}
