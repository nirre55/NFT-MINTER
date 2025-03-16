#![no_std]

use multiversx_sc::{derive_imports::*, imports::*};

mod nft_module;
mod storage;

#[type_abi]
#[derive(TopEncode, TopDecode)]
pub struct ExampleAttributes<M: ManagedTypeApi> {
    pub creation_timestamp: u64,
    pub metadata: ManagedBuffer<M>,
}

#[multiversx_sc::contract]
pub trait NftMinter: nft_module::NftModule + storage::StorageModule {
    #[init]
    fn init(&self) {}

    #[upgrade]
    fn upgrade(&self) {}

    #[allow_multiple_var_args]
    #[allow(clippy::too_many_arguments)]
    #[allow(clippy::redundant_closure)]
    #[only_owner]
    #[endpoint(createNft)]
    fn create_nft(
        &self,
        name: ManagedBuffer,
        royalties: BigUint,
        uri: ManagedBuffer,
        selling_price: BigUint,
        opt_token_used_as_payment: OptionalValue<TokenIdentifier>,
        opt_token_used_as_payment_nonce: OptionalValue<u64>,
        opt_metadata: OptionalValue<ManagedBuffer>,
    ) {
        let token_used_as_payment = match opt_token_used_as_payment {
            OptionalValue::Some(token) => EgldOrEsdtTokenIdentifier::esdt(token),
            OptionalValue::None => EgldOrEsdtTokenIdentifier::egld(),
        };
        require!(
            token_used_as_payment.is_valid(),
            "Invalid token_used_as_payment arg, not a valid token ID"
        );

        let token_used_as_payment_nonce = if token_used_as_payment.is_egld() {
            0
        } else {
            match opt_token_used_as_payment_nonce {
                OptionalValue::Some(nonce) => nonce,
                OptionalValue::None => 0,
            }
        };

        let metadata = match opt_metadata {
            OptionalValue::Some(meta) => meta,
            OptionalValue::None => ManagedBuffer::from("metadata:QmP8XL56WtNnRvWUXHh1W8MLAjekMyY5JtMw5FC72Lf3bK/7.json"),
        };

        let tags = ManagedBuffer::from("tags:worldforge");
        let mut combined_metadata = metadata;
        combined_metadata.append(&ManagedBuffer::from(";"));
        combined_metadata.append(&tags);

        let attributes = ExampleAttributes::<Self::Api> {
            creation_timestamp: self.blockchain().get_block_timestamp(),
            metadata: combined_metadata,
        };
        self.create_nft_with_attributes(
            name,
            royalties,
            attributes,
            uri,
            selling_price,
            token_used_as_payment,
            token_used_as_payment_nonce,
        );
    }

    #[view(getNftAttributes)]
    fn get_nft_attributes(&self, nft_nonce: u64) -> ExampleAttributes<Self::Api> {
        let nft_token_id = self.nft_token_id().get();
        let nft_info = self.blockchain().get_esdt_token_data(
            &self.blockchain().get_sc_address(),
            &nft_token_id,
            nft_nonce,
        );
        
        let attributes = nft_info.decode_attributes::<ExampleAttributes<Self::Api>>();
        attributes
    }
}