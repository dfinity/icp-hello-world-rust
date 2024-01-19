use pocket_ic::PocketIc;

#[test]
fn test_hello_world() {
    let pic = PocketIc::new();
    pic.create_canister();
}
