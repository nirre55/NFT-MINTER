#![no_std]

const DEFAULT_IMG_FILE_EXTENSION: &[u8] = ".png".as_bytes();

use multiversx_sc::{derive_imports::*, imports::*};

mod nft_module;
mod storage;
mod attributes_builder;
mod rarety_module;
#[type_abi]
#[derive(TopEncode, TopDecode)]
pub struct ExampleAttributes<M: ManagedTypeApi> {
    pub creation_timestamp: u64,
    pub metadata: ManagedBuffer<M>,
}

#[multiversx_sc::contract]
pub trait NftMinter: nft_module::NftModule + storage::StorageModule + attributes_builder::AttributesBuilder + rarety_module::RaretyModule {
    #[init]
    fn init(&self) {
        let image_base_cid = ManagedBuffer::from("Qmcb1DFADr6jJMbdrpbmzokS86frgAmcKkAAPN1Sa8JUUL");
        let metadata_base_cid = ManagedBuffer::from("QmQT87JFsARd3ccih62MyooeCaVtsvVMqvG1SuVKXJeFs4");
        self.image_base_cid().set_if_empty(&image_base_cid);
        self.metadata_base_cid().set_if_empty(&metadata_base_cid);
        self.royalties().set_if_empty(&BigUint::from(1000u64));
        self.file_extension().set_if_empty(&ManagedBuffer::new_from_bytes(DEFAULT_IMG_FILE_EXTENSION));
        self.tags().set_if_empty(&ManagedBuffer::from("world,universe,multiversx,nft"));
        self.clean_all_sotrage();   
        self.fill_all_storage(rarety_module::RarityProperties::Common);
    }

    #[upgrade]
    fn upgrade(&self) {
        //self.clean_all_sotrage();   
        self.fill_all_storage(rarety_module::RarityProperties::Common);
    }

    #[allow_multiple_var_args]
    #[allow(clippy::too_many_arguments)]
    #[allow(clippy::redundant_closure)]
    #[only_owner]
    #[endpoint(createNft)]
    fn create_nft(
        &self,
        name: ManagedBuffer,
        selling_price: BigUint,
        opt_token_used_as_payment: OptionalValue<TokenIdentifier>,
        opt_token_used_as_payment_nonce: OptionalValue<u64>
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

        self.create_nft_with_attributes(
            name,
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