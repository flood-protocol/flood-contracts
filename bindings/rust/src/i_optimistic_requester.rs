pub use i_optimistic_requester::*;
#[allow(clippy::too_many_arguments, non_camel_case_types)]
pub mod i_optimistic_requester {
    #![allow(clippy::enum_variant_names)]
    #![allow(dead_code)]
    #![allow(clippy::type_complexity)]
    #![allow(unused_imports)]
    pub use super::super::shared_types::*;
    use ethers::contract::{
        builders::{ContractCall, Event},
        Contract, Lazy,
    };
    use ethers::core::{
        abi::{Abi, Detokenize, InvalidOutputType, Token, Tokenizable},
        types::*,
    };
    use ethers::providers::Middleware;
    #[doc = "IOptimisticRequester was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    # [rustfmt :: skip] const __ABI : & str = "[{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"id\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"struct Request\",\"name\":\"request\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"address\",\"name\":\"requester\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"proposer\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"disputer\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"contract IERC20\",\"name\":\"currency\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"bond\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"enum RequestState\",\"name\":\"state\",\"type\":\"uint8\",\"components\":[]},{\"internalType\":\"bool\",\"name\":\"answer\",\"type\":\"bool\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"data\",\"type\":\"bytes\",\"components\":[]}]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"onPriceSettled\",\"outputs\":[]}]" ;
    #[doc = r" The parsed JSON-ABI of the contract."]
    pub static IOPTIMISTICREQUESTER_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            ethers::core::utils::__serde_json::from_str(__ABI).expect("invalid abi")
        });
    pub struct IOptimisticRequester<M>(ethers::contract::Contract<M>);
    impl<M> Clone for IOptimisticRequester<M> {
        fn clone(&self) -> Self {
            IOptimisticRequester(self.0.clone())
        }
    }
    impl<M> std::ops::Deref for IOptimisticRequester<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> std::fmt::Debug for IOptimisticRequester<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(IOptimisticRequester))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ethers::providers::Middleware> IOptimisticRequester<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(
                address.into(),
                IOPTIMISTICREQUESTER_ABI.clone(),
                client,
            )
            .into()
        }
        #[doc = "Calls the contract's `onPriceSettled` (0x734d1627) function"]
        pub fn on_price_settled(
            &self,
            id: [u8; 32],
            request: Request,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([115, 77, 22, 39], (id, request))
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>>
        for IOptimisticRequester<M>
    {
        fn from(contract: ethers::contract::Contract<M>) -> Self {
            Self(contract)
        }
    }
    #[doc = "Container type for all input parameters for the `onPriceSettled` function with signature `onPriceSettled(bytes32,(address,address,address,address,uint256,uint8,bool,bytes))` and selector `[115, 77, 22, 39]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(
        name = "onPriceSettled",
        abi = "onPriceSettled(bytes32,(address,address,address,address,uint256,uint8,bool,bytes))"
    )]
    pub struct OnPriceSettledCall {
        pub id: [u8; 32],
        pub request: Request,
    }
}
