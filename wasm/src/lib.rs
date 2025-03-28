// Code generated by the multiversx-sc build system. DO NOT EDIT.

////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

// Init:                                 1
// Upgrade:                              1
// Endpoints:                           14
// Async Callback:                       1
// Total number of exported functions:  17

#![no_std]

multiversx_sc_wasm_adapter::allocator!();
multiversx_sc_wasm_adapter::panic_handler!();

multiversx_sc_wasm_adapter::endpoints! {
    world_forge
    (
        init => init
        upgrade => upgrade
        createNft => create_nft
        getNftAttributes => get_nft_attributes
        updateNftPrice => update_nft_price
        issueToken => issue_token
        setLocalRoles => set_local_roles
        withdraw => withdraw
        buyNft => buy_nft
        setContractAddress => set_contract_address
        getNftPrice => get_nft_price
        communItems => common_items
        uncommunItems => uncommon_items
        rareItems => rare_items
        epicItems => epic_items
        legendaryItems => legendary_items
    )
}

multiversx_sc_wasm_adapter::async_callback! { world_forge }
