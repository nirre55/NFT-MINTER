use multiversx_sc_scenario::imports::*;
use interactor::proxy;

const OWNER_ADDRESS: TestAddress = TestAddress::new("owner");
const ALICE_ADDRESS: TestSCAddress = TestSCAddress::new("adder");
const CODE_PATH: MxscPath = MxscPath::new("output/world-forge.mxsc.json");

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new();
    blockchain.register_contract(CODE_PATH, world_forge::ContractBuilder);
    blockchain
}

#[test]
fn adder_blackbox() {
    let mut world = world();

    world.start_trace();

    world.account(OWNER_ADDRESS).nonce(1);

    let new_address = world
        .tx()
        .from(OWNER_ADDRESS)
        .typed( NftMinterProxy) 
        .init(5u32)
        .code(CODE_PATH)
        .new_address(ALICE_ADDRESS)
        .returns(ReturnsNewAddress)
        .run();

    assert_eq!(new_address, ALICE_ADDRESS.to_address());
}