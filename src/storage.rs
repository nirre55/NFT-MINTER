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


     // attributes
     #[storage_mapper("imageBaseCid")]
     fn image_base_cid(&self) -> SingleValueMapper<ManagedBuffer>;
 
     #[storage_mapper("metadaBaseCid")]
     fn metadata_base_cid(&self) -> SingleValueMapper<ManagedBuffer>;

     #[storage_mapper("royalties")]
     fn royalties(&self) -> SingleValueMapper<BigUint>;

     #[storage_mapper("file_extension")]
     fn file_extension(&self) -> SingleValueMapper<ManagedBuffer>;

     #[storage_mapper("tags")]
     fn tags(&self) -> SingleValueMapper<ManagedBuffer>;

     // rarety
     #[view(communIndex)]
     #[storage_mapper("common_index")]
     fn common_index(&self) -> VecMapper<usize>;

     #[view(uncommunIndex)]
     #[storage_mapper("uncommon_index")]
     fn uncommon_index(&self) -> VecMapper<usize>;
     
     #[view(rareIndex)]
     #[storage_mapper("rare_index")]
     fn rare_index(&self) -> VecMapper<usize>;

     #[view(epicIndex)]
     #[storage_mapper("epic_index")]
     fn epic_index(&self) -> VecMapper<usize>;

     #[view(legendaryIndex)]
     #[storage_mapper("legendary_index")]
     fn legendary_index(&self) -> VecMapper<usize>;
} 