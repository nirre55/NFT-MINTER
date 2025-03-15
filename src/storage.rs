multiversx_sc::imports!();
multiversx_sc::derive_imports!();

#[type_abi]
#[derive(TopEncode, TopDecode)]
pub struct PriceTag<M: ManagedTypeApi> {
    pub token: EgldOrEsdtTokenIdentifier<M>,
    pub nonce: u64,
    pub amount: BigUint<M>,
}

#[multiversx_sc::module]
pub trait StorageModule {
     // storage

     #[storage_mapper("nftTokenId")]
     fn nft_token_id(&self) -> SingleValueMapper<TokenIdentifier>;
 
     #[storage_mapper("priceTag")]
     fn price_tag(&self, nft_nonce: u64) -> SingleValueMapper<PriceTag<Self::Api>>;

     #[storage_mapper("contractAddress")]
     fn contract_address(&self) -> SingleValueMapper<ManagedAddress>;
} 