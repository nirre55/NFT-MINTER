use multiversx_sc_scenario::*;

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new();
    blockchain.register_contract("mxsc:output/world-forge.mxsc.json", world_forge::ContractBuilder);
    blockchain
}

#[test]
fn update_nft_price_rs() {
    world().run("scenarios/update_nft_price.scen.json");
}

#[test]
fn update_nft_price_go() {
    ScenarioWorld::vm_go().run("scenarios/update_nft_price.scen.json");
} 