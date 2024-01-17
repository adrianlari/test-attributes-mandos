#![no_std]

multiversx_sc::imports!();

mod test_attributes;
use test_attributes::TestAttributes;

#[multiversx_sc::contract]
pub trait TestAttributesContract {
    #[init]
    fn init(&self) {
    }

    #[upgrade]
    fn upgrade(&self) {
    }

    #[endpoint(test)]
    fn test(&self, token_id: TokenIdentifier, nonce: u64) {
        let caller = self.blockchain().get_caller();

        let another_vec = ManagedVec::new();
        let test_attributes = self.get_test_attributes(&token_id, nonce, another_vec);
        
        self.create_and_send(token_id, BigUint::from(1u64), test_attributes, caller.clone());
    }

    fn create_esdt<T: TopEncode>(&self, token_id: TokenIdentifier, amount: BigUint, attributes: T) -> u64 {
        let token_nonce = self.send().esdt_nft_create_compact(&token_id, &amount, &attributes);

        token_nonce
    }

    fn get_test_attributes(&self, token_id: &TokenIdentifier<Self::Api>, token_nonce: u64, another_vec: ManagedVec<ManagedBuffer<Self::Api>>) -> TestAttributes<Self::Api> {
        TestAttributes {
            token_id: token_id.clone(),
            token_nonce,
            another_vec,
        }
    }

    fn create_and_send<T: TopEncode>(&self,
        token_id: TokenIdentifier,
        amount: BigUint,
        attributes: T,
        receiver: ManagedAddress
    ) -> u64 {
        let token_nonce = self.create_esdt(token_id.clone(), amount.clone(), &attributes);
        self.send().direct_esdt(&receiver, &token_id, token_nonce, &amount);

        token_nonce
    }

}
