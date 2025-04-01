multiversx_sc::imports!();

#[type_abi]
#[derive(TopEncode, TopDecode)]
pub struct ExampleAttributes<M: ManagedTypeApi> {
    pub creation_timestamp: u64,
    pub metadata: ManagedBuffer<M>,
}

#[multiversx_sc::module]
pub trait NftModule: storage::StorageModule + attributes_builder::AttributesBuilder + rarety_module::RaretyModule {
    // endpoints - owner-only

    #[only_owner]
    #[endpoint(updateNftPrice)]
    fn update_nft_price(
        &self,
        nft_nonce: u64,
        new_price: BigUint,
    ) {
        require!(
            !self.price_tag(nft_nonce).is_empty(),
            "NFT not found or already sold"
        );

        let mut price_tag = self.price_tag(nft_nonce).get();
        price_tag.amount = new_price;
        self.price_tag(nft_nonce).set(&price_tag);
    }

    #[only_owner]
    #[allow_multiple_var_args]
    #[endpoint(withdraw)]
    fn withdraw(&self, token_identifier: OptionalValue<TokenIdentifier>, token_nonce: OptionalValue<u64>) {
        // Gestion par défaut si aucun token n'est spécifié
        let token = match token_identifier {
            OptionalValue::Some(token) => EgldOrEsdtTokenIdentifier::esdt(token),
            OptionalValue::None => EgldOrEsdtTokenIdentifier::egld(),
        };

        let nonce = match token_nonce {
            OptionalValue::Some(nonce) => nonce,
            OptionalValue::None => 0,
        };
        
        let balance = self.blockchain().get_sc_balance(&token, nonce);
        require!(balance > BigUint::zero(), "No funds available");
        
        let owner = self.blockchain().get_owner_address();
        
        // Envoi direct du token (EGLD ou ESDT) au propriétaire
        if token.is_egld() {
            self.send().direct_egld(&owner, &balance);
        } else {
            self.send().direct_esdt(&owner, &token.unwrap_esdt(), nonce, &balance);
        }
    }

    #[only_owner]
    #[endpoint(setContractAddress)]
    fn set_contract_address(&self, address: ManagedAddress) {
        self.contract_address().set(address);
    }

    // views

    #[allow(clippy::type_complexity)]
    #[view(getNftPrice)]
    fn get_nft_price(&self, nft_nonce: u64) -> OptionalValue<MultiValue3<EgldOrEsdtTokenIdentifier, u64, BigUint>> {
        if self.price_tag(nft_nonce).is_empty() {
            // NFT was already sold
            OptionalValue::None
        } else {
            let price_tag = self.price_tag(nft_nonce).get();

            OptionalValue::Some((price_tag.token, price_tag.nonce, price_tag.amount).into())
        }
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

    #[storage_mapper("contractAddress")]
    fn contract_address(&self) -> SingleValueMapper<ManagedAddress>;

    
}