pub use ioracle_mod::*;
#[allow(clippy::too_many_arguments, non_camel_case_types)]
pub mod ioracle_mod {
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
    #[doc = "IOracle was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static IORACLE_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            serde_json :: from_str ("[{\"inputs\":[{\"internalType\":\"address\",\"name\":\"proposer\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"disputer\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"bondToken\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"stake\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"ask\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"stake\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"bondForStake\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"id\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"bool\",\"name\":\"answer\",\"type\":\"bool\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"settle\",\"outputs\":[]}]") . expect ("invalid abi")
        });
    pub struct IOracle<M>(ethers::contract::Contract<M>);
    impl<M> Clone for IOracle<M> {
        fn clone(&self) -> Self {
            IOracle(self.0.clone())
        }
    }
    impl<M> std::ops::Deref for IOracle<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug for IOracle<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(IOracle))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ethers::providers::Middleware> IOracle<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(address.into(), IORACLE_ABI.clone(), client).into()
        }
        #[doc = "Calls the contract's `ask` (0xaf5899fc) function"]
        pub fn ask(
            &self,
            proposer: ethers::core::types::Address,
            disputer: ethers::core::types::Address,
            bond_token: ethers::core::types::Address,
            stake: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([175, 88, 153, 252], (proposer, disputer, bond_token, stake))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `bondForStake` (0x07de99f6) function"]
        pub fn bond_for_stake(
            &self,
            stake: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([7, 222, 153, 246], stake)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `settle` (0xfc361c38) function"]
        pub fn settle(
            &self,
            id: [u8; 32],
            answer: bool,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([252, 54, 28, 56], (id, answer))
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>> for IOracle<M> {
        fn from(contract: ethers::contract::Contract<M>) -> Self {
            Self(contract)
        }
    }
    #[doc = "Container type for all input parameters for the `ask`function with signature `ask(address,address,address,uint256)` and selector `[175, 88, 153, 252]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "ask", abi = "ask(address,address,address,uint256)")]
    pub struct AskCall {
        pub proposer: ethers::core::types::Address,
        pub disputer: ethers::core::types::Address,
        pub bond_token: ethers::core::types::Address,
        pub stake: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `bondForStake`function with signature `bondForStake(uint256)` and selector `[7, 222, 153, 246]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "bondForStake", abi = "bondForStake(uint256)")]
    pub struct BondForStakeCall {
        pub stake: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `settle`function with signature `settle(bytes32,bool)` and selector `[252, 54, 28, 56]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "settle", abi = "settle(bytes32,bool)")]
    pub struct SettleCall {
        pub id: [u8; 32],
        pub answer: bool,
    }
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum IOracleCalls {
        Ask(AskCall),
        BondForStake(BondForStakeCall),
        Settle(SettleCall),
    }
    impl ethers::core::abi::AbiDecode for IOracleCalls {
        fn decode(data: impl AsRef<[u8]>) -> Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) = <AskCall as ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(IOracleCalls::Ask(decoded));
            }
            if let Ok(decoded) =
                <BondForStakeCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IOracleCalls::BondForStake(decoded));
            }
            if let Ok(decoded) = <SettleCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IOracleCalls::Settle(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for IOracleCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                IOracleCalls::Ask(element) => element.encode(),
                IOracleCalls::BondForStake(element) => element.encode(),
                IOracleCalls::Settle(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for IOracleCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                IOracleCalls::Ask(element) => element.fmt(f),
                IOracleCalls::BondForStake(element) => element.fmt(f),
                IOracleCalls::Settle(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<AskCall> for IOracleCalls {
        fn from(var: AskCall) -> Self {
            IOracleCalls::Ask(var)
        }
    }
    impl ::std::convert::From<BondForStakeCall> for IOracleCalls {
        fn from(var: BondForStakeCall) -> Self {
            IOracleCalls::BondForStake(var)
        }
    }
    impl ::std::convert::From<SettleCall> for IOracleCalls {
        fn from(var: SettleCall) -> Self {
            IOracleCalls::Settle(var)
        }
    }
}
