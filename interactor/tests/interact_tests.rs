use multiversx_sc_snippets::imports::*;
use rust_interact::{config::Config, ContractInteract};

// Simple deploy test that runs on the real blockchain configuration.
// In order for this test to work, make sure that the `config.toml` file contains the real blockchain config (or choose it manually)
// Can be run with `sc-meta test`.
#[tokio::test]
#[ignore = "run on demand, relies on real blockchain state"]
async fn deploy_test_and_create_collection_without_roles_world_forge() {
    let mut interactor = ContractInteract::new(Config::new()).await;

    interactor.deploy().await;
    let token_name = "WorldForge";
    let token_ticker = "WFORGE";
    interactor.issue_token(token_name, token_ticker).await;
    interactor.nft_token_id().await;
}

#[tokio::test]
#[ignore = "run on demand, relies on real blockchain state"]
async fn deploy_test_and_create_collection_with_roles_world_forge() {
    let mut interactor = ContractInteract::new(Config::new()).await;

    interactor.deploy().await;
    let token_name = "WorldForge";
    let token_ticker = "WFORGE";
    interactor.issue_token(token_name, token_ticker).await;
    interactor.set_local_roles().await;
}

