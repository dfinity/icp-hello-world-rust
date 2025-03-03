use candid::{decode_one, encode_one, Principal};
use pocket_ic::PocketIc;
use std::fs;

const BACKEND_WASM: &str = "../../target/wasm32-unknown-unknown/release/backend.wasm";

fn setup() -> (PocketIc, Principal) {
    std::env::set_var("POCKET_IC_BIN", "/usr/local/bin/pocket-ic"); // Path of the pocket-ic binary
    let pic = PocketIc::new();

    let backend_canister = pic.create_canister();
    pic.add_cycles(backend_canister, 2_000_000_000_000); // 2T Cycles
    let wasm = fs::read(BACKEND_WASM).expect("Wasm file not found, run 'dfx build'.");
    pic.install_canister(backend_canister, wasm, vec![], None);
    (pic, backend_canister)
}

#[test]
fn test_hello_world() {
    let (pic, backend_canister) = setup();

    let Ok(response) = pic.query_call(
        backend_canister,
        Principal::anonymous(),
        "greet",
        encode_one("ICP").unwrap(),
    ) else {
        panic!("Expected reply");
    };
    let result: String = decode_one(&response).unwrap();
    assert_eq!(result, "Hello, ICP!");
}
