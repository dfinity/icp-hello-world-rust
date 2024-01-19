use std::fs;
use candid::{Principal, encode_one, decode_one};
use pocket_ic::{PocketIc, WasmResult};

const BACKEND_WASM: &str = "../../icp_hello_world_rust_backend.wasm";

fn setup() -> (PocketIc, Principal) {
    let pic = PocketIc::new();

    let backend_canister = pic.create_canister();
    pic.add_cycles(backend_canister, 2_000_000_000_000); // 2 Tcycles
    let wasm = fs::read(BACKEND_WASM).expect("Build the wasm with ./scripts/build.sh");
    pic.install_canister(backend_canister, wasm, vec![], None);
    (pic, backend_canister)
}

#[test]
fn test_hello_world() {
    let (pic, backend_canister) = setup();

    let Ok(WasmResult::Reply(response)) = pic.query_call(backend_canister, Principal::anonymous(), "greet", encode_one("ICP").unwrap()) else {
        panic!("Expected reply");
    };

    let result: String = decode_one(&response).unwrap();
    assert_eq!(result, "Hello, ICP!");
}
