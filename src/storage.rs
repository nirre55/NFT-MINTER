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
     #[view(nftTokenId)]
     #[storage_mapper("nftTokenId")]
     fn nft_token_id(&self) -> SingleValueMapper<TokenIdentifier>;
 
     #[storage_mapper("priceTag")]
     fn price_tag(&self, nft_nonce: u64) -> SingleValueMapper<PriceTag<Self::Api>>;

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
     #[view(commonItems)]
     #[storage_mapper("common_items")]
     fn common_items(&self) -> VecMapper<usize>;

     #[view(uncommonItems)]
     #[storage_mapper("uncommon_items")]
     fn uncommon_items(&self) -> VecMapper<usize>;
     
     #[view(rareItems)]
     #[storage_mapper("rare_items")]
     fn rare_items(&self) -> VecMapper<usize>;

     #[view(epicItems)]
     #[storage_mapper("epic_items")]
     fn epic_items(&self) -> VecMapper<usize>;

     #[view(legendaryItems)]
     #[storage_mapper("legendary_items")]
     fn legendary_items(&self) -> VecMapper<usize>;

     // nft name
     #[storage_mapper("nftName")]
     fn nft_name(&self, index: usize) -> SingleValueMapper<ManagedBuffer>;

     // Storage mapper pour la liste des NFTs disponibles
     #[storage_mapper("noncePerIndex")]
     fn nonce_per_index(&self, index: usize) -> VecMapper<u64>;

     // Storage mapper pour le prix de chaque NFT
    #[storage_mapper("packPrice")]
    fn pack_price(&self) -> SingleValueMapper<BigUint>;

    #[storage_mapper("minNftPerPack")]
    fn min_nft_per_pack(&self) -> SingleValueMapper<usize>;

    #[storage_mapper("maxNftPerPack")]
    fn max_nft_per_pack(&self) -> SingleValueMapper<usize>;
} 