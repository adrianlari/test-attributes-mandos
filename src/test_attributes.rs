multiversx_sc::derive_imports!();
multiversx_sc::imports!();

#[derive(NestedEncode, NestedDecode, TypeAbi, TopEncode, TopDecode, ManagedVecItem, Clone)]
pub struct TestAttributes<M: ManagedTypeApi> {
  pub token_id: TokenIdentifier<M>,
  pub token_nonce: u64,
  pub another_vec: ManagedVec<M, ManagedBuffer<M>>,
}