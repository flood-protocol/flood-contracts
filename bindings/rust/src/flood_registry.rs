pub use flood_registry::*;
#[allow(clippy::too_many_arguments, non_camel_case_types)]
pub mod flood_registry {
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
    #[doc = "FloodRegistry was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    # [rustfmt :: skip] const __ABI : & str = "[{\"inputs\":[],\"type\":\"error\",\"name\":\"FloodRegistry__InvalidInputLength\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"FloodRegistry__TokenAlreadyWhitelisted\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"FloodRegistry__TokenNotWhitelisted\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"contract AllKnowingOracle\",\"name\":\"oracle\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"OracleChanged\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"previousOwner\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"newOwner\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"OwnershipTransferStarted\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"previousOwner\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"newOwner\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"OwnershipTransferred\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"bool\",\"name\":\"whitelisted\",\"type\":\"bool\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"TokenWhitelisted\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"acceptOwnership\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address[]\",\"name\":\"tokens\",\"type\":\"address[]\",\"components\":[]},{\"internalType\":\"bool[]\",\"name\":\"enabled\",\"type\":\"bool[]\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"batchWhitelistTokens\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"isTokenWhitelisted\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"latestOracle\",\"outputs\":[{\"internalType\":\"contract AllKnowingOracle\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"owner\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"pendingOwner\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"renounceOwnership\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"contract AllKnowingOracle\",\"name\":\"oracle\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setOracle\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"newOwner\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"transferOwnership\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bool\",\"name\":\"enabled\",\"type\":\"bool\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"whitelistToken\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"whitelistedTokens\",\"outputs\":[{\"internalType\":\"address[]\",\"name\":\"\",\"type\":\"address[]\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"whitelistedTokensCount\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]}]" ;
    #[doc = r" The parsed JSON-ABI of the contract."]
    pub static FLOODREGISTRY_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            ethers::core::utils::__serde_json::from_str(__ABI).expect("invalid abi")
        });
    #[doc = r" Bytecode of the #name contract"]
    pub static FLOODREGISTRY_BYTECODE: ethers::contract::Lazy<ethers::core::types::Bytes> =
        ethers::contract::Lazy::new(|| {
            "0x608060405234801561001057600080fd5b5061001a3361001f565b610096565b600180546001600160a01b031916905561004381610046602090811b61040417901c565b50565b600080546001600160a01b038381166001600160a01b0319831681178455604051919092169283917f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e09190a35050565b61099e806100a56000396000f3fe608060405234801561001057600080fd5b50600436106100b45760003560e01c80637adbf973116100715780637adbf973146101365780638da5cb5b14610149578063b5af090f1461015a578063dfdc49621461017d578063e30c397814610193578063f2fde38b146101a457600080fd5b80630ffb1d8b146100b95780632eae4f60146100ce57806339063c63146100fe5780635e1762a014610111578063715018a61461012657806379ba50971461012e575b600080fd5b6100cc6100c7366004610788565b6101b7565b005b6004546100e1906001600160a01b031681565b6040516001600160a01b0390911681526020015b60405180910390f35b6100cc61010c366004610809565b6101cd565b61011961026f565b6040516100f59190610875565b6100cc610280565b6100cc610294565b6100cc6101443660046108c2565b610313565b6000546001600160a01b03166100e1565b61016d6101683660046108c2565b610365565b60405190151581526020016100f5565b610185610387565b6040519081526020016100f5565b6001546001600160a01b03166100e1565b6100cc6101b23660046108c2565b610393565b6101bf610454565b6101c982826104ae565b5050565b6101d5610454565b8281146101f55760405163046a41b360e31b815260040160405180910390fd5b60005b8381101561026857610256858583818110610215576102156108df565b905060200201602081019061022a91906108c2565b84848481811061023c5761023c6108df565b905060200201602081019061025191906108f5565b6104ae565b8061026081610926565b9150506101f8565b5050505050565b606061027b600261055f565b905090565b610288610454565b6102926000610573565b565b60015433906001600160a01b031681146103075760405162461bcd60e51b815260206004820152602960248201527f4f776e61626c6532537465703a2063616c6c6572206973206e6f7420746865206044820152683732bb9037bbb732b960b91b60648201526084015b60405180910390fd5b61031081610573565b50565b61031b610454565b600480546001600160a01b0319166001600160a01b0383169081179091556040517f0e05ae75e8b926552cf6fcd744d19f422561e3ced1e426868730852702dbe41890600090a250565b6001600160a01b03811660009081526003602052604081205415155b92915050565b600061027b600261058c565b61039b610454565b600180546001600160a01b0383166001600160a01b031990911681179091556103cc6000546001600160a01b031690565b6001600160a01b03167f38d16b8cac22d99fc7c124b9cd0de2d3fa1faef420bfe791d8c362d765e2270060405160405180910390a350565b600080546001600160a01b038381166001600160a01b0319831681178455604051919092169283917f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e09190a35050565b6000546001600160a01b031633146102925760405162461bcd60e51b815260206004820181905260248201527f4f776e61626c653a2063616c6c6572206973206e6f7420746865206f776e657260448201526064016102fe565b80156104e75760006104c1600284610596565b9050806104e15760405163f6d9d35960e01b815260040160405180910390fd5b50610516565b60006104f46002846105ab565b90508061051457604051630ecc952760e01b815260040160405180910390fd5b505b816001600160a01b03167fef81a9943b96c8df4ef243401c9bf5159146166211356898b52d382086168d9282604051610553911515815260200190565b60405180910390a25050565b6060600061056c836105c0565b9392505050565b600180546001600160a01b031916905561031081610404565b6000610381825490565b600061056c836001600160a01b03841661061c565b600061056c836001600160a01b03841661066b565b60608160000180548060200260200160405190810160405280929190818152602001828054801561061057602002820191906000526020600020905b8154815260200190600101908083116105fc575b50505050509050919050565b600081815260018301602052604081205461066357508154600181810184556000848152602080822090930184905584548482528286019093526040902091909155610381565b506000610381565b6000818152600183016020526040812054801561075457600061068f60018361093f565b85549091506000906106a39060019061093f565b90508181146107085760008660000182815481106106c3576106c36108df565b90600052602060002001549050808760000184815481106106e6576106e66108df565b6000918252602080832090910192909255918252600188019052604090208390555b855486908061071957610719610952565b600190038181906000526020600020016000905590558560010160008681526020019081526020016000206000905560019350505050610381565b6000915050610381565b6001600160a01b038116811461031057600080fd5b8035801515811461078357600080fd5b919050565b6000806040838503121561079b57600080fd5b82356107a68161075e565b91506107b460208401610773565b90509250929050565b60008083601f8401126107cf57600080fd5b50813567ffffffffffffffff8111156107e757600080fd5b6020830191508360208260051b850101111561080257600080fd5b9250929050565b6000806000806040858703121561081f57600080fd5b843567ffffffffffffffff8082111561083757600080fd5b610843888389016107bd565b9096509450602087013591508082111561085c57600080fd5b50610869878288016107bd565b95989497509550505050565b6020808252825182820181905260009190848201906040850190845b818110156108b65783516001600160a01b031683529284019291840191600101610891565b50909695505050505050565b6000602082840312156108d457600080fd5b813561056c8161075e565b634e487b7160e01b600052603260045260246000fd5b60006020828403121561090757600080fd5b61056c82610773565b634e487b7160e01b600052601160045260246000fd5b60006001820161093857610938610910565b5060010190565b8181038181111561038157610381610910565b634e487b7160e01b600052603160045260246000fdfea2646970667358221220f33fce1fbb013bb3338152c4a9620b66780a7b32aed51582ec0c450a77e7340364736f6c63430008110033" . parse () . expect ("invalid bytecode")
        });
    pub struct FloodRegistry<M>(ethers::contract::Contract<M>);
    impl<M> Clone for FloodRegistry<M> {
        fn clone(&self) -> Self {
            FloodRegistry(self.0.clone())
        }
    }
    impl<M> std::ops::Deref for FloodRegistry<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> std::fmt::Debug for FloodRegistry<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(FloodRegistry))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ethers::providers::Middleware> FloodRegistry<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(address.into(), FLOODREGISTRY_ABI.clone(), client)
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
                FLOODREGISTRY_ABI.clone(),
                FLOODREGISTRY_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        #[doc = "Calls the contract's `acceptOwnership` (0x79ba5097) function"]
        pub fn accept_ownership(&self) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([121, 186, 80, 151], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `batchWhitelistTokens` (0x39063c63) function"]
        pub fn batch_whitelist_tokens(
            &self,
            tokens: ::std::vec::Vec<ethers::core::types::Address>,
            enabled: ::std::vec::Vec<bool>,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([57, 6, 60, 99], (tokens, enabled))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `isTokenWhitelisted` (0xb5af090f) function"]
        pub fn is_token_whitelisted(
            &self,
            token: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([181, 175, 9, 15], token)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `latestOracle` (0x2eae4f60) function"]
        pub fn latest_oracle(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([46, 174, 79, 96], ())
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
        #[doc = "Calls the contract's `pendingOwner` (0xe30c3978) function"]
        pub fn pending_owner(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([227, 12, 57, 120], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `renounceOwnership` (0x715018a6) function"]
        pub fn renounce_ownership(&self) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([113, 80, 24, 166], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setOracle` (0x7adbf973) function"]
        pub fn set_oracle(
            &self,
            oracle: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([122, 219, 249, 115], oracle)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `transferOwnership` (0xf2fde38b) function"]
        pub fn transfer_ownership(
            &self,
            new_owner: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([242, 253, 227, 139], new_owner)
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
        #[doc = "Calls the contract's `whitelistedTokens` (0x5e1762a0) function"]
        pub fn whitelisted_tokens(
            &self,
        ) -> ethers::contract::builders::ContractCall<
            M,
            ::std::vec::Vec<ethers::core::types::Address>,
        > {
            self.0
                .method_hash([94, 23, 98, 160], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `whitelistedTokensCount` (0xdfdc4962) function"]
        pub fn whitelisted_tokens_count(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([223, 220, 73, 98], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Gets the contract's `OracleChanged` event"]
        pub fn oracle_changed_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, OracleChangedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `OwnershipTransferStarted` event"]
        pub fn ownership_transfer_started_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, OwnershipTransferStartedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `OwnershipTransferred` event"]
        pub fn ownership_transferred_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, OwnershipTransferredFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `TokenWhitelisted` event"]
        pub fn token_whitelisted_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, TokenWhitelistedFilter> {
            self.0.event()
        }
        #[doc = r" Returns an [`Event`](#ethers_contract::builders::Event) builder for all events of this contract"]
        pub fn events(&self) -> ethers::contract::builders::Event<M, FloodRegistryEvents> {
            self.0.event_with_filter(Default::default())
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>> for FloodRegistry<M> {
        fn from(contract: ethers::contract::Contract<M>) -> Self {
            Self(contract)
        }
    }
    #[doc = "Custom Error type `FloodRegistry__InvalidInputLength` with signature `FloodRegistry__InvalidInputLength()` and selector `[35, 82, 13, 152]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthError,
        ethers :: contract :: EthDisplay,
    )]
    #[etherror(
        name = "FloodRegistry__InvalidInputLength",
        abi = "FloodRegistry__InvalidInputLength()"
    )]
    pub struct FloodRegistry__InvalidInputLength;
    #[doc = "Custom Error type `FloodRegistry__TokenAlreadyWhitelisted` with signature `FloodRegistry__TokenAlreadyWhitelisted()` and selector `[246, 217, 211, 89]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthError,
        ethers :: contract :: EthDisplay,
    )]
    #[etherror(
        name = "FloodRegistry__TokenAlreadyWhitelisted",
        abi = "FloodRegistry__TokenAlreadyWhitelisted()"
    )]
    pub struct FloodRegistry__TokenAlreadyWhitelisted;
    #[doc = "Custom Error type `FloodRegistry__TokenNotWhitelisted` with signature `FloodRegistry__TokenNotWhitelisted()` and selector `[14, 204, 149, 39]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthError,
        ethers :: contract :: EthDisplay,
    )]
    #[etherror(
        name = "FloodRegistry__TokenNotWhitelisted",
        abi = "FloodRegistry__TokenNotWhitelisted()"
    )]
    pub struct FloodRegistry__TokenNotWhitelisted;
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum FloodRegistryErrors {
        FloodRegistry__InvalidInputLength(FloodRegistry__InvalidInputLength),
        FloodRegistry__TokenAlreadyWhitelisted(FloodRegistry__TokenAlreadyWhitelisted),
        FloodRegistry__TokenNotWhitelisted(FloodRegistry__TokenNotWhitelisted),
    }
    impl ethers::core::abi::AbiDecode for FloodRegistryErrors {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::std::result::Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <FloodRegistry__InvalidInputLength as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(FloodRegistryErrors::FloodRegistry__InvalidInputLength(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <FloodRegistry__TokenAlreadyWhitelisted as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(FloodRegistryErrors::FloodRegistry__TokenAlreadyWhitelisted(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <FloodRegistry__TokenNotWhitelisted as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(FloodRegistryErrors::FloodRegistry__TokenNotWhitelisted(
                    decoded,
                ));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for FloodRegistryErrors {
        fn encode(self) -> Vec<u8> {
            match self {
                FloodRegistryErrors::FloodRegistry__InvalidInputLength(element) => element.encode(),
                FloodRegistryErrors::FloodRegistry__TokenAlreadyWhitelisted(element) => {
                    element.encode()
                }
                FloodRegistryErrors::FloodRegistry__TokenNotWhitelisted(element) => {
                    element.encode()
                }
            }
        }
    }
    impl ::std::fmt::Display for FloodRegistryErrors {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                FloodRegistryErrors::FloodRegistry__InvalidInputLength(element) => element.fmt(f),
                FloodRegistryErrors::FloodRegistry__TokenAlreadyWhitelisted(element) => {
                    element.fmt(f)
                }
                FloodRegistryErrors::FloodRegistry__TokenNotWhitelisted(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<FloodRegistry__InvalidInputLength> for FloodRegistryErrors {
        fn from(var: FloodRegistry__InvalidInputLength) -> Self {
            FloodRegistryErrors::FloodRegistry__InvalidInputLength(var)
        }
    }
    impl ::std::convert::From<FloodRegistry__TokenAlreadyWhitelisted> for FloodRegistryErrors {
        fn from(var: FloodRegistry__TokenAlreadyWhitelisted) -> Self {
            FloodRegistryErrors::FloodRegistry__TokenAlreadyWhitelisted(var)
        }
    }
    impl ::std::convert::From<FloodRegistry__TokenNotWhitelisted> for FloodRegistryErrors {
        fn from(var: FloodRegistry__TokenNotWhitelisted) -> Self {
            FloodRegistryErrors::FloodRegistry__TokenNotWhitelisted(var)
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
    #[ethevent(name = "OracleChanged", abi = "OracleChanged(address)")]
    pub struct OracleChangedFilter {
        #[ethevent(indexed)]
        pub oracle: ethers::core::types::Address,
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
    #[ethevent(
        name = "OwnershipTransferStarted",
        abi = "OwnershipTransferStarted(address,address)"
    )]
    pub struct OwnershipTransferStartedFilter {
        #[ethevent(indexed)]
        pub previous_owner: ethers::core::types::Address,
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
    #[ethevent(
        name = "OwnershipTransferred",
        abi = "OwnershipTransferred(address,address)"
    )]
    pub struct OwnershipTransferredFilter {
        #[ethevent(indexed)]
        pub previous_owner: ethers::core::types::Address,
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
    pub enum FloodRegistryEvents {
        OracleChangedFilter(OracleChangedFilter),
        OwnershipTransferStartedFilter(OwnershipTransferStartedFilter),
        OwnershipTransferredFilter(OwnershipTransferredFilter),
        TokenWhitelistedFilter(TokenWhitelistedFilter),
    }
    impl ethers::contract::EthLogDecode for FloodRegistryEvents {
        fn decode_log(
            log: &ethers::core::abi::RawLog,
        ) -> ::std::result::Result<Self, ethers::core::abi::Error>
        where
            Self: Sized,
        {
            if let Ok(decoded) = OracleChangedFilter::decode_log(log) {
                return Ok(FloodRegistryEvents::OracleChangedFilter(decoded));
            }
            if let Ok(decoded) = OwnershipTransferStartedFilter::decode_log(log) {
                return Ok(FloodRegistryEvents::OwnershipTransferStartedFilter(decoded));
            }
            if let Ok(decoded) = OwnershipTransferredFilter::decode_log(log) {
                return Ok(FloodRegistryEvents::OwnershipTransferredFilter(decoded));
            }
            if let Ok(decoded) = TokenWhitelistedFilter::decode_log(log) {
                return Ok(FloodRegistryEvents::TokenWhitelistedFilter(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::std::fmt::Display for FloodRegistryEvents {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                FloodRegistryEvents::OracleChangedFilter(element) => element.fmt(f),
                FloodRegistryEvents::OwnershipTransferStartedFilter(element) => element.fmt(f),
                FloodRegistryEvents::OwnershipTransferredFilter(element) => element.fmt(f),
                FloodRegistryEvents::TokenWhitelistedFilter(element) => element.fmt(f),
            }
        }
    }
    #[doc = "Container type for all input parameters for the `acceptOwnership` function with signature `acceptOwnership()` and selector `[121, 186, 80, 151]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "acceptOwnership", abi = "acceptOwnership()")]
    pub struct AcceptOwnershipCall;
    #[doc = "Container type for all input parameters for the `batchWhitelistTokens` function with signature `batchWhitelistTokens(address[],bool[])` and selector `[57, 6, 60, 99]`"]
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
        name = "batchWhitelistTokens",
        abi = "batchWhitelistTokens(address[],bool[])"
    )]
    pub struct BatchWhitelistTokensCall {
        pub tokens: ::std::vec::Vec<ethers::core::types::Address>,
        pub enabled: ::std::vec::Vec<bool>,
    }
    #[doc = "Container type for all input parameters for the `isTokenWhitelisted` function with signature `isTokenWhitelisted(address)` and selector `[181, 175, 9, 15]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "isTokenWhitelisted", abi = "isTokenWhitelisted(address)")]
    pub struct IsTokenWhitelistedCall {
        pub token: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `latestOracle` function with signature `latestOracle()` and selector `[46, 174, 79, 96]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "latestOracle", abi = "latestOracle()")]
    pub struct LatestOracleCall;
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
    #[doc = "Container type for all input parameters for the `pendingOwner` function with signature `pendingOwner()` and selector `[227, 12, 57, 120]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "pendingOwner", abi = "pendingOwner()")]
    pub struct PendingOwnerCall;
    #[doc = "Container type for all input parameters for the `renounceOwnership` function with signature `renounceOwnership()` and selector `[113, 80, 24, 166]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "renounceOwnership", abi = "renounceOwnership()")]
    pub struct RenounceOwnershipCall;
    #[doc = "Container type for all input parameters for the `setOracle` function with signature `setOracle(address)` and selector `[122, 219, 249, 115]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "setOracle", abi = "setOracle(address)")]
    pub struct SetOracleCall {
        pub oracle: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `transferOwnership` function with signature `transferOwnership(address)` and selector `[242, 253, 227, 139]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "transferOwnership", abi = "transferOwnership(address)")]
    pub struct TransferOwnershipCall {
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
    #[doc = "Container type for all input parameters for the `whitelistedTokens` function with signature `whitelistedTokens()` and selector `[94, 23, 98, 160]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "whitelistedTokens", abi = "whitelistedTokens()")]
    pub struct WhitelistedTokensCall;
    #[doc = "Container type for all input parameters for the `whitelistedTokensCount` function with signature `whitelistedTokensCount()` and selector `[223, 220, 73, 98]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "whitelistedTokensCount", abi = "whitelistedTokensCount()")]
    pub struct WhitelistedTokensCountCall;
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum FloodRegistryCalls {
        AcceptOwnership(AcceptOwnershipCall),
        BatchWhitelistTokens(BatchWhitelistTokensCall),
        IsTokenWhitelisted(IsTokenWhitelistedCall),
        LatestOracle(LatestOracleCall),
        Owner(OwnerCall),
        PendingOwner(PendingOwnerCall),
        RenounceOwnership(RenounceOwnershipCall),
        SetOracle(SetOracleCall),
        TransferOwnership(TransferOwnershipCall),
        WhitelistToken(WhitelistTokenCall),
        WhitelistedTokens(WhitelistedTokensCall),
        WhitelistedTokensCount(WhitelistedTokensCountCall),
    }
    impl ethers::core::abi::AbiDecode for FloodRegistryCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::std::result::Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <AcceptOwnershipCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(FloodRegistryCalls::AcceptOwnership(decoded));
            }
            if let Ok(decoded) =
                <BatchWhitelistTokensCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(FloodRegistryCalls::BatchWhitelistTokens(decoded));
            }
            if let Ok(decoded) =
                <IsTokenWhitelistedCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(FloodRegistryCalls::IsTokenWhitelisted(decoded));
            }
            if let Ok(decoded) =
                <LatestOracleCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(FloodRegistryCalls::LatestOracle(decoded));
            }
            if let Ok(decoded) = <OwnerCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(FloodRegistryCalls::Owner(decoded));
            }
            if let Ok(decoded) =
                <PendingOwnerCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(FloodRegistryCalls::PendingOwner(decoded));
            }
            if let Ok(decoded) =
                <RenounceOwnershipCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(FloodRegistryCalls::RenounceOwnership(decoded));
            }
            if let Ok(decoded) =
                <SetOracleCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(FloodRegistryCalls::SetOracle(decoded));
            }
            if let Ok(decoded) =
                <TransferOwnershipCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(FloodRegistryCalls::TransferOwnership(decoded));
            }
            if let Ok(decoded) =
                <WhitelistTokenCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(FloodRegistryCalls::WhitelistToken(decoded));
            }
            if let Ok(decoded) =
                <WhitelistedTokensCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(FloodRegistryCalls::WhitelistedTokens(decoded));
            }
            if let Ok(decoded) =
                <WhitelistedTokensCountCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(FloodRegistryCalls::WhitelistedTokensCount(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for FloodRegistryCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                FloodRegistryCalls::AcceptOwnership(element) => element.encode(),
                FloodRegistryCalls::BatchWhitelistTokens(element) => element.encode(),
                FloodRegistryCalls::IsTokenWhitelisted(element) => element.encode(),
                FloodRegistryCalls::LatestOracle(element) => element.encode(),
                FloodRegistryCalls::Owner(element) => element.encode(),
                FloodRegistryCalls::PendingOwner(element) => element.encode(),
                FloodRegistryCalls::RenounceOwnership(element) => element.encode(),
                FloodRegistryCalls::SetOracle(element) => element.encode(),
                FloodRegistryCalls::TransferOwnership(element) => element.encode(),
                FloodRegistryCalls::WhitelistToken(element) => element.encode(),
                FloodRegistryCalls::WhitelistedTokens(element) => element.encode(),
                FloodRegistryCalls::WhitelistedTokensCount(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for FloodRegistryCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                FloodRegistryCalls::AcceptOwnership(element) => element.fmt(f),
                FloodRegistryCalls::BatchWhitelistTokens(element) => element.fmt(f),
                FloodRegistryCalls::IsTokenWhitelisted(element) => element.fmt(f),
                FloodRegistryCalls::LatestOracle(element) => element.fmt(f),
                FloodRegistryCalls::Owner(element) => element.fmt(f),
                FloodRegistryCalls::PendingOwner(element) => element.fmt(f),
                FloodRegistryCalls::RenounceOwnership(element) => element.fmt(f),
                FloodRegistryCalls::SetOracle(element) => element.fmt(f),
                FloodRegistryCalls::TransferOwnership(element) => element.fmt(f),
                FloodRegistryCalls::WhitelistToken(element) => element.fmt(f),
                FloodRegistryCalls::WhitelistedTokens(element) => element.fmt(f),
                FloodRegistryCalls::WhitelistedTokensCount(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<AcceptOwnershipCall> for FloodRegistryCalls {
        fn from(var: AcceptOwnershipCall) -> Self {
            FloodRegistryCalls::AcceptOwnership(var)
        }
    }
    impl ::std::convert::From<BatchWhitelistTokensCall> for FloodRegistryCalls {
        fn from(var: BatchWhitelistTokensCall) -> Self {
            FloodRegistryCalls::BatchWhitelistTokens(var)
        }
    }
    impl ::std::convert::From<IsTokenWhitelistedCall> for FloodRegistryCalls {
        fn from(var: IsTokenWhitelistedCall) -> Self {
            FloodRegistryCalls::IsTokenWhitelisted(var)
        }
    }
    impl ::std::convert::From<LatestOracleCall> for FloodRegistryCalls {
        fn from(var: LatestOracleCall) -> Self {
            FloodRegistryCalls::LatestOracle(var)
        }
    }
    impl ::std::convert::From<OwnerCall> for FloodRegistryCalls {
        fn from(var: OwnerCall) -> Self {
            FloodRegistryCalls::Owner(var)
        }
    }
    impl ::std::convert::From<PendingOwnerCall> for FloodRegistryCalls {
        fn from(var: PendingOwnerCall) -> Self {
            FloodRegistryCalls::PendingOwner(var)
        }
    }
    impl ::std::convert::From<RenounceOwnershipCall> for FloodRegistryCalls {
        fn from(var: RenounceOwnershipCall) -> Self {
            FloodRegistryCalls::RenounceOwnership(var)
        }
    }
    impl ::std::convert::From<SetOracleCall> for FloodRegistryCalls {
        fn from(var: SetOracleCall) -> Self {
            FloodRegistryCalls::SetOracle(var)
        }
    }
    impl ::std::convert::From<TransferOwnershipCall> for FloodRegistryCalls {
        fn from(var: TransferOwnershipCall) -> Self {
            FloodRegistryCalls::TransferOwnership(var)
        }
    }
    impl ::std::convert::From<WhitelistTokenCall> for FloodRegistryCalls {
        fn from(var: WhitelistTokenCall) -> Self {
            FloodRegistryCalls::WhitelistToken(var)
        }
    }
    impl ::std::convert::From<WhitelistedTokensCall> for FloodRegistryCalls {
        fn from(var: WhitelistedTokensCall) -> Self {
            FloodRegistryCalls::WhitelistedTokens(var)
        }
    }
    impl ::std::convert::From<WhitelistedTokensCountCall> for FloodRegistryCalls {
        fn from(var: WhitelistedTokensCountCall) -> Self {
            FloodRegistryCalls::WhitelistedTokensCount(var)
        }
    }
    #[doc = "Container type for all return fields from the `isTokenWhitelisted` function with signature `isTokenWhitelisted(address)` and selector `[181, 175, 9, 15]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct IsTokenWhitelistedReturn(pub bool);
    #[doc = "Container type for all return fields from the `latestOracle` function with signature `latestOracle()` and selector `[46, 174, 79, 96]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct LatestOracleReturn(pub ethers::core::types::Address);
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
    #[doc = "Container type for all return fields from the `pendingOwner` function with signature `pendingOwner()` and selector `[227, 12, 57, 120]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct PendingOwnerReturn(pub ethers::core::types::Address);
    #[doc = "Container type for all return fields from the `whitelistedTokens` function with signature `whitelistedTokens()` and selector `[94, 23, 98, 160]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct WhitelistedTokensReturn(pub ::std::vec::Vec<ethers::core::types::Address>);
    #[doc = "Container type for all return fields from the `whitelistedTokensCount` function with signature `whitelistedTokensCount()` and selector `[223, 220, 73, 98]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct WhitelistedTokensCountReturn(pub ethers::core::types::U256);
}
