pub use iallknowingoracleevents_mod::*;
#[allow(clippy::too_many_arguments, non_camel_case_types)]
pub mod iallknowingoracleevents_mod {
    #![allow(clippy::enum_variant_names)]
    #![allow(dead_code)]
    #![allow(clippy::type_complexity)]
    #![allow(unused_imports)]
    use ethers::contract::{
        builders::{ContractCall, Event},
        Contract, Lazy,
    };
    use ethers::core::{
        abi::{Abi, Detokenize, InvalidOutputType, Token, Tokenizable},
        types::*,
    };
    use ethers::providers::Middleware;
    #[doc = "IAllKnowingOracleEvents was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static IALLKNOWINGORACLEEVENTS_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            serde_json :: from_str ("[{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"newPct\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"BondPctChanged\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"id\",\"type\":\"bytes32\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"proposer\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"disputer\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"bondToken\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"stake\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"bond\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"NewRequest\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"id\",\"type\":\"bytes32\",\"components\":[],\"indexed\":false},{\"internalType\":\"bool\",\"name\":\"answer\",\"type\":\"bool\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"RequestSettled\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"settler\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"bool\",\"name\":\"enabled\",\"type\":\"bool\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"SettlerWhitelisted\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"bool\",\"name\":\"enabled\",\"type\":\"bool\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"TokenWhitelisted\",\"outputs\":[],\"anonymous\":false}]") . expect ("invalid abi")
        });
    pub struct IAllKnowingOracleEvents<M>(ethers::contract::Contract<M>);
    impl<M> Clone for IAllKnowingOracleEvents<M> {
        fn clone(&self) -> Self {
            IAllKnowingOracleEvents(self.0.clone())
        }
    }
    impl<M> std::ops::Deref for IAllKnowingOracleEvents<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug for IAllKnowingOracleEvents<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(IAllKnowingOracleEvents))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ethers::providers::Middleware> IAllKnowingOracleEvents<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(
                address.into(),
                IALLKNOWINGORACLEEVENTS_ABI.clone(),
                client,
            )
            .into()
        }
        #[doc = "Gets the contract's `BondPctChanged` event"]
        pub fn bond_pct_changed_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, BondPctChangedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `NewRequest` event"]
        pub fn new_request_filter(&self) -> ethers::contract::builders::Event<M, NewRequestFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `RequestSettled` event"]
        pub fn request_settled_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, RequestSettledFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `SettlerWhitelisted` event"]
        pub fn settler_whitelisted_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, SettlerWhitelistedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `TokenWhitelisted` event"]
        pub fn token_whitelisted_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, TokenWhitelistedFilter> {
            self.0.event()
        }
        #[doc = r" Returns an [`Event`](#ethers_contract::builders::Event) builder for all events of this contract"]
        pub fn events(
            &self,
        ) -> ethers::contract::builders::Event<M, IAllKnowingOracleEventsEvents> {
            self.0.event_with_filter(Default::default())
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>>
        for IAllKnowingOracleEvents<M>
    {
        fn from(contract: ethers::contract::Contract<M>) -> Self {
            Self(contract)
        }
    }
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
    )]
    #[ethevent(name = "BondPctChanged", abi = "BondPctChanged(uint256)")]
    pub struct BondPctChangedFilter {
        pub new_pct: ethers::core::types::U256,
    }
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
    )]
    #[ethevent(
        name = "NewRequest",
        abi = "NewRequest(bytes32,address,address,address,uint256,uint256)"
    )]
    pub struct NewRequestFilter {
        #[ethevent(indexed)]
        pub id: [u8; 32],
        #[ethevent(indexed)]
        pub proposer: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub disputer: ethers::core::types::Address,
        pub bond_token: ethers::core::types::Address,
        pub stake: ethers::core::types::U256,
        pub bond: ethers::core::types::U256,
    }
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
    )]
    #[ethevent(name = "RequestSettled", abi = "RequestSettled(bytes32,bool)")]
    pub struct RequestSettledFilter {
        pub id: [u8; 32],
        pub answer: bool,
    }
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
    )]
    #[ethevent(name = "SettlerWhitelisted", abi = "SettlerWhitelisted(address,bool)")]
    pub struct SettlerWhitelistedFilter {
        #[ethevent(indexed)]
        pub settler: ethers::core::types::Address,
        pub enabled: bool,
    }
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
    )]
    #[ethevent(name = "TokenWhitelisted", abi = "TokenWhitelisted(address,bool)")]
    pub struct TokenWhitelistedFilter {
        #[ethevent(indexed)]
        pub token: ethers::core::types::Address,
        pub enabled: bool,
    }
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum IAllKnowingOracleEventsEvents {
        BondPctChangedFilter(BondPctChangedFilter),
        NewRequestFilter(NewRequestFilter),
        RequestSettledFilter(RequestSettledFilter),
        SettlerWhitelistedFilter(SettlerWhitelistedFilter),
        TokenWhitelistedFilter(TokenWhitelistedFilter),
    }
    impl ethers::contract::EthLogDecode for IAllKnowingOracleEventsEvents {
        fn decode_log(log: &ethers::core::abi::RawLog) -> Result<Self, ethers::core::abi::Error>
        where
            Self: Sized,
        {
            if let Ok(decoded) = BondPctChangedFilter::decode_log(log) {
                return Ok(IAllKnowingOracleEventsEvents::BondPctChangedFilter(decoded));
            }
            if let Ok(decoded) = NewRequestFilter::decode_log(log) {
                return Ok(IAllKnowingOracleEventsEvents::NewRequestFilter(decoded));
            }
            if let Ok(decoded) = RequestSettledFilter::decode_log(log) {
                return Ok(IAllKnowingOracleEventsEvents::RequestSettledFilter(decoded));
            }
            if let Ok(decoded) = SettlerWhitelistedFilter::decode_log(log) {
                return Ok(IAllKnowingOracleEventsEvents::SettlerWhitelistedFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = TokenWhitelistedFilter::decode_log(log) {
                return Ok(IAllKnowingOracleEventsEvents::TokenWhitelistedFilter(
                    decoded,
                ));
            }
            Err(ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::std::fmt::Display for IAllKnowingOracleEventsEvents {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                IAllKnowingOracleEventsEvents::BondPctChangedFilter(element) => element.fmt(f),
                IAllKnowingOracleEventsEvents::NewRequestFilter(element) => element.fmt(f),
                IAllKnowingOracleEventsEvents::RequestSettledFilter(element) => element.fmt(f),
                IAllKnowingOracleEventsEvents::SettlerWhitelistedFilter(element) => element.fmt(f),
                IAllKnowingOracleEventsEvents::TokenWhitelistedFilter(element) => element.fmt(f),
            }
        }
    }
}
