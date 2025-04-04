#![no_std]
multiversx_sc::imports!();
multiversx_sc::derive_imports!();

const DEFAULT_IMG_FILE_EXTENSION: &[u8] = ".png".as_bytes();


mod nft_module;
mod storage;
mod attributes_builder;
mod rarety_module;


#[multiversx_sc::contract]
pub trait NftMinter: nft_module::NftModule + storage::StorageModule + attributes_builder::AttributesBuilder + rarety_module::RaretyModule {
    #[init]
    fn init(&self) {
        let image_base_cid = ManagedBuffer::from("Qmcb1DFADr6jJMbdrpbmzokS86frgAmcKkAAPN1Sa8JUUL");
        let metadata_base_cid = ManagedBuffer::from("QmQT87JFsARd3ccih62MyooeCaVtsvVMqvG1SuVKXJeFs4");
        let roaylties =BigUint::from(1000u64); // 10% royalties
        let file_extention = ManagedBuffer::new_from_bytes(DEFAULT_IMG_FILE_EXTENSION);
        let tags = ManagedBuffer::from("world,universe,multiversx,nft");
        let default_price = BigUint::from(1_000_000_000_000_000_000u64); // 1 EGLD

        // Set default values
        self.image_base_cid().set_if_empty(&image_base_cid);
        self.metadata_base_cid().set_if_empty(&metadata_base_cid);
        self.royalties().set_if_empty(&roaylties);
        self.file_extension().set_if_empty(&file_extention);
        self.tags().set_if_empty(&tags);
        self.pack_price().set_if_empty(&default_price);
        self.min_nft_per_pack().set_if_empty(3);
        self.max_nft_per_pack().set_if_empty(5);
    }

    #[upgrade]
    fn upgrade(&self) {

        let base_cid = ManagedBuffer::from("QmQhfYHg5XrokfkCcz3kbDAaAG323b71GxurwxtJaFgNCS");
        self.image_base_cid().set_if_empty(&base_cid);
        self.metadata_base_cid().set_if_empty(&base_cid);
    }

    #[allow_multiple_var_args]
    #[allow(clippy::too_many_arguments)]
    #[allow(clippy::redundant_closure)]
    #[only_owner]
    #[endpoint(createNft)]
    fn create_nft(
        &self,
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
            selling_price,
            token_used_as_payment,
            token_used_as_payment_nonce,
        );
    }
    
}