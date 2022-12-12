pub use token_whitelist::*;
#[allow(clippy::too_many_arguments, non_camel_case_types)]
pub mod token_whitelist {
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
    #[doc = "TokenWhitelist was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    # [rustfmt :: skip] const __ABI : & str = "[{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"constructor\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"user\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"newOwner\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"OwnerUpdated\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"bool\",\"name\":\"whitelisted\",\"type\":\"bool\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"TokenWhitelisted\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"isWhitelisted\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"owner\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"newOwner\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setOwner\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bool\",\"name\":\"enabled\",\"type\":\"bool\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"whitelistToken\",\"outputs\":[]}]" ;
    #[doc = r" The parsed JSON-ABI of the contract."]
    pub static TOKENWHITELIST_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            ethers::core::utils::__serde_json::from_str(__ABI).expect("invalid abi")
        });
    #[doc = r" Bytecode of the #name contract"]
    pub static TOKENWHITELIST_BYTECODE: ethers::contract::Lazy<ethers::core::types::Bytes> =
        ethers::contract::Lazy::new(|| {
            "0x608060405234801561001057600080fd5b50600080546001600160a01b031916339081178255604051909182917f8292fce18fa69edf4db7b94ea2e58241df0ae57f97e0a6c9b29067028bf92d76908290a3506102cd806100616000396000f3fe608060405234801561001057600080fd5b506004361061004c5760003560e01c80630ffb1d8b1461005157806313af4035146100665780633af32abf146100795780638da5cb5b146100b1575b600080fd5b61006461005f366004610239565b6100dc565b005b610064610074366004610275565b610189565b61009c610087366004610275565b60016020526000908152604090205460ff1681565b60405190151581526020015b60405180910390f35b6000546100c4906001600160a01b031681565b6040516001600160a01b0390911681526020016100a8565b6000546001600160a01b0316331461012a5760405162461bcd60e51b815260206004820152600c60248201526b15539055551213d49256915160a21b60448201526064015b60405180910390fd5b6001600160a01b038216600081815260016020908152604091829020805460ff191685151590811790915591519182527fef81a9943b96c8df4ef243401c9bf5159146166211356898b52d382086168d92910160405180910390a25050565b6000546001600160a01b031633146101d25760405162461bcd60e51b815260206004820152600c60248201526b15539055551213d49256915160a21b6044820152606401610121565b600080546001600160a01b0319166001600160a01b0383169081178255604051909133917f8292fce18fa69edf4db7b94ea2e58241df0ae57f97e0a6c9b29067028bf92d769190a350565b80356001600160a01b038116811461023457600080fd5b919050565b6000806040838503121561024c57600080fd5b6102558361021d565b91506020830135801515811461026a57600080fd5b809150509250929050565b60006020828403121561028757600080fd5b6102908261021d565b939250505056fea264697066735822122077702ee397a390d12feca3d91edf592b30afa11ff387285ea7918086b696e80d64736f6c63430008110033" . parse () . expect ("invalid bytecode")
        });
    pub struct TokenWhitelist<M>(ethers::contract::Contract<M>);
    impl<M> Clone for TokenWhitelist<M> {
        fn clone(&self) -> Self {
            TokenWhitelist(self.0.clone())
        }
    }
    impl<M> std::ops::Deref for TokenWhitelist<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> std::fmt::Debug for TokenWhitelist<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(TokenWhitelist))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ethers::providers::Middleware> TokenWhitelist<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(address.into(), TOKENWHITELIST_ABI.clone(), client)
                .into()
        }
        #[doc = r" Constructs the general purpose `Deployer` instance based on the provided constructor arguments and sends it."]
        #[doc = r" Returns a new instance of a deployer that returns an instance of this contract after sending the transaction"]
        #[doc = r""]
        #[doc = r" Notes:"]
        #[doc = r" 1. If there are no constructor arguments, you should pass `()` as the argument."]
        #[doc = r" 1. The default poll duration is 7 seconds."]
        #[doc = r" 1. The default number of confirmations is 1 block."]
        #[doc = r""]
        #[doc = r""]
        #[doc = r" # Example"]
        #[doc = r""]
        #[doc = r" Generate contract bindings with `abigen!` and deploy a new contract instance."]
        #[doc = r""]
        #[doc = r" *Note*: this requires a `bytecode` and `abi` object in the `greeter.json` artifact."]
        #[doc = r""]
        #[doc = r" ```ignore"]
        #[doc = r" # async fn deploy<M: ethers::providers::Middleware>(client: ::std::sync::Arc<M>) {"]
        #[doc = r#"     abigen!(Greeter,"../greeter.json");"#]
        #[doc = r""]
        #[doc = r#"    let greeter_contract = Greeter::deploy(client, "Hello world!".to_string()).unwrap().send().await.unwrap();"#]
        #[doc = r"    let msg = greeter_contract.greet().call().await.unwrap();"]
        #[doc = r" # }"]
        #[doc = r" ```"]
        pub fn deploy<T: ethers::core::abi::Tokenize>(
            client: ::std::sync::Arc<M>,
            constructor_args: T,
        ) -> ::std::result::Result<
            ethers::contract::builders::ContractDeployer<M, Self>,
            ethers::contract::ContractError<M>,
        > {
            let factory = ethers::contract::ContractFactory::new(
                TOKENWHITELIST_ABI.clone(),
                TOKENWHITELIST_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        #[doc = "Calls the contract's `isWhitelisted` (0x3af32abf) function"]
        pub fn is_whitelisted(
            &self,
            p0: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([58, 243, 42, 191], p0)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `owner` (0x8da5cb5b) function"]
        pub fn owner(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([141, 165, 203, 91], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setOwner` (0x13af4035) function"]
        pub fn set_owner(
            &self,
            new_owner: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([19, 175, 64, 53], new_owner)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `whitelistToken` (0x0ffb1d8b) function"]
        pub fn whitelist_token(
            &self,
            token: ethers::core::types::Address,
            enabled: bool,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([15, 251, 29, 139], (token, enabled))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Gets the contract's `OwnerUpdated` event"]
        pub fn owner_updated_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, OwnerUpdatedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `TokenWhitelisted` event"]
        pub fn token_whitelisted_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, TokenWhitelistedFilter> {
            self.0.event()
        }
        #[doc = r" Returns an [`Event`](#ethers_contract::builders::Event) builder for all events of this contract"]
        pub fn events(&self) -> ethers::contract::builders::Event<M, TokenWhitelistEvents> {
            self.0.event_with_filter(Default::default())
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>> for TokenWhitelist<M> {
        fn from(contract: ethers::contract::Contract<M>) -> Self {
            Self(contract)
        }
    }
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethevent(name = "OwnerUpdated", abi = "OwnerUpdated(address,address)")]
    pub struct OwnerUpdatedFilter {
        #[ethevent(indexed)]
        pub user: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub new_owner: ethers::core::types::Address,
    }
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethevent(name = "TokenWhitelisted", abi = "TokenWhitelisted(address,bool)")]
    pub struct TokenWhitelistedFilter {
        #[ethevent(indexed)]
        pub token: ethers::core::types::Address,
        pub whitelisted: bool,
    }
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum TokenWhitelistEvents {
        OwnerUpdatedFilter(OwnerUpdatedFilter),
        TokenWhitelistedFilter(TokenWhitelistedFilter),
    }
    impl ethers::contract::EthLogDecode for TokenWhitelistEvents {
        fn decode_log(
            log: &ethers::core::abi::RawLog,
        ) -> ::std::result::Result<Self, ethers::core::abi::Error>
        where
            Self: Sized,
        {
            if let Ok(decoded) = OwnerUpdatedFilter::decode_log(log) {
                return Ok(TokenWhitelistEvents::OwnerUpdatedFilter(decoded));
            }
            if let Ok(decoded) = TokenWhitelistedFilter::decode_log(log) {
                return Ok(TokenWhitelistEvents::TokenWhitelistedFilter(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::std::fmt::Display for TokenWhitelistEvents {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                TokenWhitelistEvents::OwnerUpdatedFilter(element) => element.fmt(f),
                TokenWhitelistEvents::TokenWhitelistedFilter(element) => element.fmt(f),
            }
        }
    }
    #[doc = "Container type for all input parameters for the `isWhitelisted` function with signature `isWhitelisted(address)` and selector `[58, 243, 42, 191]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "isWhitelisted", abi = "isWhitelisted(address)")]
    pub struct IsWhitelistedCall(pub ethers::core::types::Address);
    #[doc = "Container type for all input parameters for the `owner` function with signature `owner()` and selector `[141, 165, 203, 91]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "owner", abi = "owner()")]
    pub struct OwnerCall;
    #[doc = "Container type for all input parameters for the `setOwner` function with signature `setOwner(address)` and selector `[19, 175, 64, 53]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "setOwner", abi = "setOwner(address)")]
    pub struct SetOwnerCall {
        pub new_owner: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `whitelistToken` function with signature `whitelistToken(address,bool)` and selector `[15, 251, 29, 139]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "whitelistToken", abi = "whitelistToken(address,bool)")]
    pub struct WhitelistTokenCall {
        pub token: ethers::core::types::Address,
        pub enabled: bool,
    }
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum TokenWhitelistCalls {
        IsWhitelisted(IsWhitelistedCall),
        Owner(OwnerCall),
        SetOwner(SetOwnerCall),
        WhitelistToken(WhitelistTokenCall),
    }
    impl ethers::core::abi::AbiDecode for TokenWhitelistCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::std::result::Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <IsWhitelistedCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(TokenWhitelistCalls::IsWhitelisted(decoded));
            }
            if let Ok(decoded) = <OwnerCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(TokenWhitelistCalls::Owner(decoded));
            }
            if let Ok(decoded) =
                <SetOwnerCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(TokenWhitelistCalls::SetOwner(decoded));
            }
            if let Ok(decoded) =
                <WhitelistTokenCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(TokenWhitelistCalls::WhitelistToken(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for TokenWhitelistCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                TokenWhitelistCalls::IsWhitelisted(element) => element.encode(),
                TokenWhitelistCalls::Owner(element) => element.encode(),
                TokenWhitelistCalls::SetOwner(element) => element.encode(),
                TokenWhitelistCalls::WhitelistToken(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for TokenWhitelistCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                TokenWhitelistCalls::IsWhitelisted(element) => element.fmt(f),
                TokenWhitelistCalls::Owner(element) => element.fmt(f),
                TokenWhitelistCalls::SetOwner(element) => element.fmt(f),
                TokenWhitelistCalls::WhitelistToken(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<IsWhitelistedCall> for TokenWhitelistCalls {
        fn from(var: IsWhitelistedCall) -> Self {
            TokenWhitelistCalls::IsWhitelisted(var)
        }
    }
    impl ::std::convert::From<OwnerCall> for TokenWhitelistCalls {
        fn from(var: OwnerCall) -> Self {
            TokenWhitelistCalls::Owner(var)
        }
    }
    impl ::std::convert::From<SetOwnerCall> for TokenWhitelistCalls {
        fn from(var: SetOwnerCall) -> Self {
            TokenWhitelistCalls::SetOwner(var)
        }
    }
    impl ::std::convert::From<WhitelistTokenCall> for TokenWhitelistCalls {
        fn from(var: WhitelistTokenCall) -> Self {
            TokenWhitelistCalls::WhitelistToken(var)
        }
    }
    #[doc = "Container type for all return fields from the `isWhitelisted` function with signature `isWhitelisted(address)` and selector `[58, 243, 42, 191]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct IsWhitelistedReturn(pub bool);
    #[doc = "Container type for all return fields from the `owner` function with signature `owner()` and selector `[141, 165, 203, 91]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct OwnerReturn(pub ethers::core::types::Address);
}
