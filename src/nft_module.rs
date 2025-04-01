multiversx_sc::imports!();

const NFT_AMOUNT: u32 = 1;

use crate::storage;
use crate::attributes_builder;
use crate::rarety_module;


#[multiversx_sc::module]
pub trait NftModule: storage::StorageModule + attributes_builder::AttributesBuilder + rarety_module::RaretyModule {
    
    // endpoints - owner-only
    #[only_owner]
    #[payable("EGLD")]
    #[endpoint(issueToken)]
    fn issue_token(&self, token_name: ManagedBuffer, token_ticker: ManagedBuffer) {
        require!(self.nft_token_id().is_empty(), "Token already issued");

        let payment_amount = self.call_value().egld();
        self.send()
            .esdt_system_sc_proxy()
            .issue_non_fungible(
                payment_amount.clone(),
                &token_name,
                &token_ticker,
                NonFungibleTokenProperties {
                    can_freeze: true,
                    can_wipe: true,
                    can_pause: true,
                    can_transfer_create_role: true,
                    can_change_owner: false,
                    can_upgrade: false,
                    can_add_special_roles: true,
                },
            )
            .with_callback(self.callbacks().issue_callback())
            .async_call_and_exit()
    }

    #[only_owner]
    #[endpoint(setLocalRoles)]
    fn set_local_roles(&self) {
        self.require_token_issued();

        self.send()
            .esdt_system_sc_proxy()
            .set_special_roles(
                self.blockchain().get_sc_address(),
                self.nft_token_id().get(),
                [EsdtLocalRole::NftCreate][..].iter().cloned(),
            )
            .async_call_and_exit()
    }

    #[payable]
    #[endpoint(buyNft)]
    fn buy_nft(&self, nft_nonce: u64) {
        let payment = self.call_value().egld_or_single_esdt();

        self.require_token_issued();
        require!(
            !self.price_tag(nft_nonce).is_empty(),
            "Invalid nonce or NFT was already sold"
        );

        let price_tag = self.price_tag(nft_nonce).get();
        require!(
            payment.token_identifier == price_tag.token,
            "Invalid token used as payment"
        );
        require!(
            payment.token_nonce == price_tag.nonce,
            "Invalid nonce for payment token"
        );
        require!(
            payment.amount == price_tag.amount,
            "Invalid amount as payment"
        );

        self.price_tag(nft_nonce).clear();

        let nft_token_id = self.nft_token_id().get();

        self.tx()
            .to(ToCaller)
            .single_esdt(&nft_token_id, nft_nonce, &BigUint::from(NFT_AMOUNT))
            .transfer();

        // Transfer payment to contract owner
        //let owner = self.blockchain().get_sc_address();
        //self.tx().to(self.contract_address().get()).payment(payment).transfer();
    }

    // callbacks

    #[callback]
    fn issue_callback(
        &self,
        #[call_result] result: ManagedAsyncCallResult<EgldOrEsdtTokenIdentifier>,
    ) {
        match result {
            ManagedAsyncCallResult::Ok(token_id) => {
                self.nft_token_id().set(token_id.unwrap_esdt());
            },
            ManagedAsyncCallResult::Err(_) => {
                let returned = self.call_value().egld_or_single_esdt();
                if returned.token_identifier.is_egld() && returned.amount > 0 {
                    self.tx().to(ToCaller).egld(returned.amount).transfer();
                }
            },
        }
    }

    // private

    #[allow(clippy::too_many_arguments)]
    fn create_nft_with_attributes(
        &self,
        name: ManagedBuffer,
        selling_price: BigUint,
        token_used_as_payment: EgldOrEsdtTokenIdentifier,
        token_used_as_payment_nonce: u64,
    ) -> u64 {
        self.require_token_issued();

        let index_to_mint: usize = self.drop_item();
        let nft_token_id = self.nft_token_id().get();

        let attributes  = self.build_attributes_buffer(index_to_mint);

        let attributes_sha256 = self.crypto().sha256(&attributes);
        let attributes_hash = attributes_sha256.as_managed_buffer();
        
        let nft_nonce = self.send().esdt_nft_create(
            &nft_token_id,
            &BigUint::from(NFT_AMOUNT),
            &name,
            &self.royalties().get(),
            attributes_hash,
            &attributes,
            &self.build_uris_vec(index_to_mint),
        );

        self.price_tag(nft_nonce).set(&storage::PriceTag {
            token: token_used_as_payment,
            nonce: token_used_as_payment_nonce,
            amount: selling_price,
        });

        nft_nonce
    }

    fn require_token_issued(&self) {
        require!(!self.nft_token_id().is_empty(), "Token not issued");
    }
}