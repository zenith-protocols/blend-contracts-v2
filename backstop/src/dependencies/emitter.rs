// The v1 Emitter contract. See `emitter/README.md` for details on why this is a fixed WASM
// rather than a crate dependency.
use soroban_sdk::contractimport;

contractimport!(file = "../emitter_v1.wasm");
