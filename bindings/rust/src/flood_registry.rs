pub use flood_registry::*;
#[allow(clippy::too_many_arguments, non_camel_case_types)]
pub mod flood_registry {
    #![allow(clippy::enum_variant_names)]
    #![allow(dead_code)]
    #![allow(clippy::type_complexity)]
    #![allow(unused_imports)]
    ///FloodRegistry was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs
    use std::sync::Arc;
    use ::ethers::core::{
        abi::{Abi, Token, Detokenize, InvalidOutputType, Tokenizable},
        types::*,
    };
    use ::ethers::contract::{
        Contract, builders::{ContractCall, Event},
        Lazy,
    };
    use ::ethers::providers::Middleware;
    #[rustfmt::skip]
    const __ABI: &str = "[{\"inputs\":[{\"internalType\":\"contract IWETH9\",\"name\":\"weth\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"constructor\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"FloodRegistry__InvalidInputLength\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"FloodRegistry__InvalidToken\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"FloodRegistry__TokenAlreadyWhitelisted\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"FloodRegistry__TokenNotWhitelisted\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"contract AllKnowingOracle\",\"name\":\"oracle\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"OracleChanged\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"previousOwner\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"newOwner\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"OwnershipTransferStarted\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"previousOwner\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"newOwner\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"OwnershipTransferred\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"bool\",\"name\":\"whitelisted\",\"type\":\"bool\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"TokenWhitelisted\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"WETH\",\"outputs\":[{\"internalType\":\"contract IWETH9\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"acceptOwnership\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address[]\",\"name\":\"tokens\",\"type\":\"address[]\",\"components\":[]},{\"internalType\":\"bool[]\",\"name\":\"enabled\",\"type\":\"bool[]\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"batchWhitelistTokens\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"isTokenWhitelisted\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"latestOracle\",\"outputs\":[{\"internalType\":\"contract AllKnowingOracle\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"owner\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"pendingOwner\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"renounceOwnership\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"contract AllKnowingOracle\",\"name\":\"oracle\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setOracle\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"newOwner\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"transferOwnership\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bool\",\"name\":\"enabled\",\"type\":\"bool\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"whitelistToken\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"whitelistedTokens\",\"outputs\":[{\"internalType\":\"address[]\",\"name\":\"\",\"type\":\"address[]\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"whitelistedTokensCount\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]}]";
    /// The parsed JSON-ABI of the contract.
    pub static FLOODREGISTRY_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(||
    ::ethers::core::utils::__serde_json::from_str(__ABI).expect("invalid abi"));
    /// Bytecode of the #name contract
    pub static FLOODREGISTRY_BYTECODE: ::ethers::contract::Lazy<
        ::ethers::core::types::Bytes,
    > = ::ethers::contract::Lazy::new(|| {
        "0x60a060405234801561001057600080fd5b50604051610b67380380610b6783398101604081905261002f916100c0565b61003833610049565b6001600160a01b03166080526100f0565b600180546001600160a01b031916905561006d81610070602090811b6104c217901c565b50565b600080546001600160a01b038381166001600160a01b0319831681178455604051919092169283917f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e09190a35050565b6000602082840312156100d257600080fd5b81516001600160a01b03811681146100e957600080fd5b9392505050565b608051610a5c61010b600039600061017a0152610a5c6000f3fe608060405234801561001057600080fd5b50600436106100cf5760003560e01c80637adbf9731161008c578063b5af090f11610066578063b5af090f1461019c578063dfdc4962146101bf578063e30c3978146101d5578063f2fde38b146101e657600080fd5b80637adbf973146101515780638da5cb5b14610164578063ad5c46481461017557600080fd5b80630ffb1d8b146100d45780632eae4f60146100e957806339063c63146101195780635e1762a01461012c578063715018a61461014157806379ba509714610149575b600080fd5b6100e76100e2366004610846565b6101f9565b005b6004546100fc906001600160a01b031681565b6040516001600160a01b0390911681526020015b60405180910390f35b6100e76101273660046108c7565b61023a565b61013461032d565b6040516101109190610933565b6100e761033e565b6100e7610352565b6100e761015f366004610980565b6103d1565b6000546001600160a01b03166100fc565b6100fc7f000000000000000000000000000000000000000000000000000000000000000081565b6101af6101aa366004610980565b610423565b6040519015158152602001610110565b6101c7610445565b604051908152602001610110565b6001546001600160a01b03166100fc565b6100e76101f4366004610980565b610451565b610201610512565b816001600160a01b03163b60000361022c57604051631564b84560e21b815260040160405180910390fd5b610236828261056c565b5050565b610242610512565b8281146102625760405163046a41b360e31b815260040160405180910390fd5b60005b838110156103265784848281811061027f5761027f61099d565b90506020020160208101906102949190610980565b6001600160a01b03163b6000036102be57604051631564b84560e21b815260040160405180910390fd5b6103148585838181106102d3576102d361099d565b90506020020160208101906102e89190610980565b8484848181106102fa576102fa61099d565b905060200201602081019061030f91906109b3565b61056c565b8061031e816109e4565b915050610265565b5050505050565b6060610339600261061d565b905090565b610346610512565b6103506000610631565b565b60015433906001600160a01b031681146103c55760405162461bcd60e51b815260206004820152602960248201527f4f776e61626c6532537465703a2063616c6c6572206973206e6f7420746865206044820152683732bb9037bbb732b960b91b60648201526084015b60405180910390fd5b6103ce81610631565b50565b6103d9610512565b600480546001600160a01b0319166001600160a01b0383169081179091556040517f0e05ae75e8b926552cf6fcd744d19f422561e3ced1e426868730852702dbe41890600090a250565b6001600160a01b03811660009081526003602052604081205415155b92915050565b6000610339600261064a565b610459610512565b600180546001600160a01b0383166001600160a01b0319909116811790915561048a6000546001600160a01b031690565b6001600160a01b03167f38d16b8cac22d99fc7c124b9cd0de2d3fa1faef420bfe791d8c362d765e2270060405160405180910390a350565b600080546001600160a01b038381166001600160a01b0319831681178455604051919092169283917f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e09190a35050565b6000546001600160a01b031633146103505760405162461bcd60e51b815260206004820181905260248201527f4f776e61626c653a2063616c6c6572206973206e6f7420746865206f776e657260448201526064016103bc565b80156105a557600061057f600284610654565b90508061059f5760405163f6d9d35960e01b815260040160405180910390fd5b506105d4565b60006105b2600284610669565b9050806105d257604051630ecc952760e01b815260040160405180910390fd5b505b816001600160a01b03167fef81a9943b96c8df4ef243401c9bf5159146166211356898b52d382086168d9282604051610611911515815260200190565b60405180910390a25050565b6060600061062a8361067e565b9392505050565b600180546001600160a01b03191690556103ce816104c2565b600061043f825490565b600061062a836001600160a01b0384166106da565b600061062a836001600160a01b038416610729565b6060816000018054806020026020016040519081016040528092919081815260200182805480156106ce57602002820191906000526020600020905b8154815260200190600101908083116106ba575b50505050509050919050565b60008181526001830160205260408120546107215750815460018181018455600084815260208082209093018490558454848252828601909352604090209190915561043f565b50600061043f565b6000818152600183016020526040812054801561081257600061074d6001836109fd565b8554909150600090610761906001906109fd565b90508181146107c65760008660000182815481106107815761078161099d565b90600052602060002001549050808760000184815481106107a4576107a461099d565b6000918252602080832090910192909255918252600188019052604090208390555b85548690806107d7576107d7610a10565b60019003818190600052602060002001600090559055856001016000868152602001908152602001600020600090556001935050505061043f565b600091505061043f565b6001600160a01b03811681146103ce57600080fd5b8035801515811461084157600080fd5b919050565b6000806040838503121561085957600080fd5b82356108648161081c565b915061087260208401610831565b90509250929050565b60008083601f84011261088d57600080fd5b50813567ffffffffffffffff8111156108a557600080fd5b6020830191508360208260051b85010111156108c057600080fd5b9250929050565b600080600080604085870312156108dd57600080fd5b843567ffffffffffffffff808211156108f557600080fd5b6109018883890161087b565b9096509450602087013591508082111561091a57600080fd5b506109278782880161087b565b95989497509550505050565b6020808252825182820181905260009190848201906040850190845b818110156109745783516001600160a01b03168352928401929184019160010161094f565b50909695505050505050565b60006020828403121561099257600080fd5b813561062a8161081c565b634e487b7160e01b600052603260045260246000fd5b6000602082840312156109c557600080fd5b61062a82610831565b634e487b7160e01b600052601160045260246000fd5b6000600182016109f6576109f66109ce565b5060010190565b8181038181111561043f5761043f6109ce565b634e487b7160e01b600052603160045260246000fdfea2646970667358221220dd102c9548205676da90b2b12f7dc9bcdc28aceaa6522b93b4d86140cfa661fe64736f6c63430008110033"
            .parse()
            .expect("invalid bytecode")
    });
    pub struct FloodRegistry<M>(::ethers::contract::Contract<M>);
    impl<M> Clone for FloodRegistry<M> {
        fn clone(&self) -> Self {
            FloodRegistry(self.0.clone())
        }
    }
    impl<M> std::ops::Deref for FloodRegistry<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> std::fmt::Debug for FloodRegistry<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(FloodRegistry)).field(&self.address()).finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> FloodRegistry<M> {
        /// Creates a new contract instance with the specified `ethers`
        /// client at the given `Address`. The contract derefs to a `ethers::Contract`
        /// object
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    FLOODREGISTRY_ABI.clone(),
                    client,
                ),
            )
        }
        /// Constructs the general purpose `Deployer` instance based on the provided constructor arguments and sends it.
        /// Returns a new instance of a deployer that returns an instance of this contract after sending the transaction
        ///
        /// Notes:
        /// 1. If there are no constructor arguments, you should pass `()` as the argument.
        /// 1. The default poll duration is 7 seconds.
        /// 1. The default number of confirmations is 1 block.
        ///
        ///
        /// # Example
        ///
        /// Generate contract bindings with `abigen!` and deploy a new contract instance.
        ///
        /// *Note*: this requires a `bytecode` and `abi` object in the `greeter.json` artifact.
        ///
        /// ```ignore
        /// # async fn deploy<M: ethers::providers::Middleware>(client: ::std::sync::Arc<M>) {
        ///     abigen!(Greeter,"../greeter.json");
        ///
        ///    let greeter_contract = Greeter::deploy(client, "Hello world!".to_string()).unwrap().send().await.unwrap();
        ///    let msg = greeter_contract.greet().call().await.unwrap();
        /// # }
        /// ```
        pub fn deploy<T: ::ethers::core::abi::Tokenize>(
            client: ::std::sync::Arc<M>,
            constructor_args: T,
        ) -> ::std::result::Result<
            ::ethers::contract::builders::ContractDeployer<M, Self>,
            ::ethers::contract::ContractError<M>,
        > {
            let factory = ::ethers::contract::ContractFactory::new(
                FLOODREGISTRY_ABI.clone(),
                FLOODREGISTRY_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `WETH` (0xad5c4648) function
        pub fn weth(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([173, 92, 70, 72], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `acceptOwnership` (0x79ba5097) function
        pub fn accept_ownership(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([121, 186, 80, 151], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `batchWhitelistTokens` (0x39063c63) function
        pub fn batch_whitelist_tokens(
            &self,
            tokens: ::std::vec::Vec<::ethers::core::types::Address>,
            enabled: ::std::vec::Vec<bool>,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([57, 6, 60, 99], (tokens, enabled))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `isTokenWhitelisted` (0xb5af090f) function
        pub fn is_token_whitelisted(
            &self,
            token: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([181, 175, 9, 15], token)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `latestOracle` (0x2eae4f60) function
        pub fn latest_oracle(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([46, 174, 79, 96], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `owner` (0x8da5cb5b) function
        pub fn owner(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([141, 165, 203, 91], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `pendingOwner` (0xe30c3978) function
        pub fn pending_owner(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([227, 12, 57, 120], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `renounceOwnership` (0x715018a6) function
        pub fn renounce_ownership(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([113, 80, 24, 166], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setOracle` (0x7adbf973) function
        pub fn set_oracle(
            &self,
            oracle: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([122, 219, 249, 115], oracle)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `transferOwnership` (0xf2fde38b) function
        pub fn transfer_ownership(
            &self,
            new_owner: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([242, 253, 227, 139], new_owner)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `whitelistToken` (0x0ffb1d8b) function
        pub fn whitelist_token(
            &self,
            token: ::ethers::core::types::Address,
            enabled: bool,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([15, 251, 29, 139], (token, enabled))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `whitelistedTokens` (0x5e1762a0) function
        pub fn whitelisted_tokens(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::std::vec::Vec<::ethers::core::types::Address>,
        > {
            self.0
                .method_hash([94, 23, 98, 160], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `whitelistedTokensCount` (0xdfdc4962) function
        pub fn whitelisted_tokens_count(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([223, 220, 73, 98], ())
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `OracleChanged` event
        pub fn oracle_changed_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<M, OracleChangedFilter> {
            self.0.event()
        }
        ///Gets the contract's `OwnershipTransferStarted` event
        pub fn ownership_transfer_started_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<M, OwnershipTransferStartedFilter> {
            self.0.event()
        }
        ///Gets the contract's `OwnershipTransferred` event
        pub fn ownership_transferred_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<M, OwnershipTransferredFilter> {
            self.0.event()
        }
        ///Gets the contract's `TokenWhitelisted` event
        pub fn token_whitelisted_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<M, TokenWhitelistedFilter> {
            self.0.event()
        }
        /// Returns an [`Event`](#ethers_contract::builders::Event) builder for all events of this contract
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<M, FloodRegistryEvents> {
            self.0.event_with_filter(Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for FloodRegistry<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Custom Error type `FloodRegistry__InvalidInputLength` with signature `FloodRegistry__InvalidInputLength()` and selector `0x23520d98`
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
    )]
    #[etherror(
        name = "FloodRegistry__InvalidInputLength",
        abi = "FloodRegistry__InvalidInputLength()"
    )]
    pub struct FloodRegistry__InvalidInputLength;
    ///Custom Error type `FloodRegistry__InvalidToken` with signature `FloodRegistry__InvalidToken()` and selector `0x5592e114`
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
    )]
    #[etherror(
        name = "FloodRegistry__InvalidToken",
        abi = "FloodRegistry__InvalidToken()"
    )]
    pub struct FloodRegistry__InvalidToken;
    ///Custom Error type `FloodRegistry__TokenAlreadyWhitelisted` with signature `FloodRegistry__TokenAlreadyWhitelisted()` and selector `0xf6d9d359`
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
    )]
    #[etherror(
        name = "FloodRegistry__TokenAlreadyWhitelisted",
        abi = "FloodRegistry__TokenAlreadyWhitelisted()"
    )]
    pub struct FloodRegistry__TokenAlreadyWhitelisted;
    ///Custom Error type `FloodRegistry__TokenNotWhitelisted` with signature `FloodRegistry__TokenNotWhitelisted()` and selector `0x0ecc9527`
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
    )]
    #[etherror(
        name = "FloodRegistry__TokenNotWhitelisted",
        abi = "FloodRegistry__TokenNotWhitelisted()"
    )]
    pub struct FloodRegistry__TokenNotWhitelisted;
    #[derive(Debug, Clone, PartialEq, Eq, ::ethers::contract::EthAbiType)]
    pub enum FloodRegistryErrors {
        FloodRegistry__InvalidInputLength(FloodRegistry__InvalidInputLength),
        FloodRegistry__InvalidToken(FloodRegistry__InvalidToken),
        FloodRegistry__TokenAlreadyWhitelisted(FloodRegistry__TokenAlreadyWhitelisted),
        FloodRegistry__TokenNotWhitelisted(FloodRegistry__TokenNotWhitelisted),
    }
    impl ::ethers::core::abi::AbiDecode for FloodRegistryErrors {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::std::result::Result<Self, ::ethers::core::abi::AbiError> {
            if let Ok(decoded)
                = <FloodRegistry__InvalidInputLength as ::ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                ) {
                return Ok(
                    FloodRegistryErrors::FloodRegistry__InvalidInputLength(decoded),
                );
            }
            if let Ok(decoded)
                = <FloodRegistry__InvalidToken as ::ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                ) {
                return Ok(FloodRegistryErrors::FloodRegistry__InvalidToken(decoded));
            }
            if let Ok(decoded)
                = <FloodRegistry__TokenAlreadyWhitelisted as ::ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                ) {
                return Ok(
                    FloodRegistryErrors::FloodRegistry__TokenAlreadyWhitelisted(decoded),
                );
            }
            if let Ok(decoded)
                = <FloodRegistry__TokenNotWhitelisted as ::ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                ) {
                return Ok(
                    FloodRegistryErrors::FloodRegistry__TokenNotWhitelisted(decoded),
                );
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for FloodRegistryErrors {
        fn encode(self) -> Vec<u8> {
            match self {
                FloodRegistryErrors::FloodRegistry__InvalidInputLength(element) => {
                    element.encode()
                }
                FloodRegistryErrors::FloodRegistry__InvalidToken(element) => {
                    element.encode()
                }
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
                FloodRegistryErrors::FloodRegistry__InvalidInputLength(element) => {
                    element.fmt(f)
                }
                FloodRegistryErrors::FloodRegistry__InvalidToken(element) => {
                    element.fmt(f)
                }
                FloodRegistryErrors::FloodRegistry__TokenAlreadyWhitelisted(element) => {
                    element.fmt(f)
                }
                FloodRegistryErrors::FloodRegistry__TokenNotWhitelisted(element) => {
                    element.fmt(f)
                }
            }
        }
    }
    impl ::std::convert::From<FloodRegistry__InvalidInputLength>
    for FloodRegistryErrors {
        fn from(var: FloodRegistry__InvalidInputLength) -> Self {
            FloodRegistryErrors::FloodRegistry__InvalidInputLength(var)
        }
    }
    impl ::std::convert::From<FloodRegistry__InvalidToken> for FloodRegistryErrors {
        fn from(var: FloodRegistry__InvalidToken) -> Self {
            FloodRegistryErrors::FloodRegistry__InvalidToken(var)
        }
    }
    impl ::std::convert::From<FloodRegistry__TokenAlreadyWhitelisted>
    for FloodRegistryErrors {
        fn from(var: FloodRegistry__TokenAlreadyWhitelisted) -> Self {
            FloodRegistryErrors::FloodRegistry__TokenAlreadyWhitelisted(var)
        }
    }
    impl ::std::convert::From<FloodRegistry__TokenNotWhitelisted>
    for FloodRegistryErrors {
        fn from(var: FloodRegistry__TokenNotWhitelisted) -> Self {
            FloodRegistryErrors::FloodRegistry__TokenNotWhitelisted(var)
        }
    }
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
    )]
    #[derive(Default)]
    #[ethevent(name = "OracleChanged", abi = "OracleChanged(address)")]
    pub struct OracleChangedFilter {
        #[ethevent(indexed)]
        pub oracle: ::ethers::core::types::Address,
    }
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
    )]
    #[derive(Default)]
    #[ethevent(
        name = "OwnershipTransferStarted",
        abi = "OwnershipTransferStarted(address,address)"
    )]
    pub struct OwnershipTransferStartedFilter {
        #[ethevent(indexed)]
        pub previous_owner: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub new_owner: ::ethers::core::types::Address,
    }
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
    )]
    #[derive(Default)]
    #[ethevent(
        name = "OwnershipTransferred",
        abi = "OwnershipTransferred(address,address)"
    )]
    pub struct OwnershipTransferredFilter {
        #[ethevent(indexed)]
        pub previous_owner: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub new_owner: ::ethers::core::types::Address,
    }
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
    )]
    #[derive(Default)]
    #[ethevent(name = "TokenWhitelisted", abi = "TokenWhitelisted(address,bool)")]
    pub struct TokenWhitelistedFilter {
        #[ethevent(indexed)]
        pub token: ::ethers::core::types::Address,
        pub whitelisted: bool,
    }
    #[derive(Debug, Clone, PartialEq, Eq, ::ethers::contract::EthAbiType)]
    pub enum FloodRegistryEvents {
        OracleChangedFilter(OracleChangedFilter),
        OwnershipTransferStartedFilter(OwnershipTransferStartedFilter),
        OwnershipTransferredFilter(OwnershipTransferredFilter),
        TokenWhitelistedFilter(TokenWhitelistedFilter),
    }
    impl ::ethers::contract::EthLogDecode for FloodRegistryEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::std::result::Result<Self, ::ethers::core::abi::Error>
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
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::std::fmt::Display for FloodRegistryEvents {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                FloodRegistryEvents::OracleChangedFilter(element) => element.fmt(f),
                FloodRegistryEvents::OwnershipTransferStartedFilter(element) => {
                    element.fmt(f)
                }
                FloodRegistryEvents::OwnershipTransferredFilter(element) => {
                    element.fmt(f)
                }
                FloodRegistryEvents::TokenWhitelistedFilter(element) => element.fmt(f),
            }
        }
    }
    ///Container type for all input parameters for the `WETH` function with signature `WETH()` and selector `0xad5c4648`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
    )]
    #[derive(Default)]
    #[ethcall(name = "WETH", abi = "WETH()")]
    pub struct WethCall;
    ///Container type for all input parameters for the `acceptOwnership` function with signature `acceptOwnership()` and selector `0x79ba5097`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
    )]
    #[derive(Default)]
    #[ethcall(name = "acceptOwnership", abi = "acceptOwnership()")]
    pub struct AcceptOwnershipCall;
    ///Container type for all input parameters for the `batchWhitelistTokens` function with signature `batchWhitelistTokens(address[],bool[])` and selector `0x39063c63`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
    )]
    #[derive(Default)]
    #[ethcall(
        name = "batchWhitelistTokens",
        abi = "batchWhitelistTokens(address[],bool[])"
    )]
    pub struct BatchWhitelistTokensCall {
        pub tokens: ::std::vec::Vec<::ethers::core::types::Address>,
        pub enabled: ::std::vec::Vec<bool>,
    }
    ///Container type for all input parameters for the `isTokenWhitelisted` function with signature `isTokenWhitelisted(address)` and selector `0xb5af090f`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
    )]
    #[derive(Default)]
    #[ethcall(name = "isTokenWhitelisted", abi = "isTokenWhitelisted(address)")]
    pub struct IsTokenWhitelistedCall {
        pub token: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `latestOracle` function with signature `latestOracle()` and selector `0x2eae4f60`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
    )]
    #[derive(Default)]
    #[ethcall(name = "latestOracle", abi = "latestOracle()")]
    pub struct LatestOracleCall;
    ///Container type for all input parameters for the `owner` function with signature `owner()` and selector `0x8da5cb5b`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
    )]
    #[derive(Default)]
    #[ethcall(name = "owner", abi = "owner()")]
    pub struct OwnerCall;
    ///Container type for all input parameters for the `pendingOwner` function with signature `pendingOwner()` and selector `0xe30c3978`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
    )]
    #[derive(Default)]
    #[ethcall(name = "pendingOwner", abi = "pendingOwner()")]
    pub struct PendingOwnerCall;
    ///Container type for all input parameters for the `renounceOwnership` function with signature `renounceOwnership()` and selector `0x715018a6`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
    )]
    #[derive(Default)]
    #[ethcall(name = "renounceOwnership", abi = "renounceOwnership()")]
    pub struct RenounceOwnershipCall;
    ///Container type for all input parameters for the `setOracle` function with signature `setOracle(address)` and selector `0x7adbf973`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
    )]
    #[derive(Default)]
    #[ethcall(name = "setOracle", abi = "setOracle(address)")]
    pub struct SetOracleCall {
        pub oracle: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `transferOwnership` function with signature `transferOwnership(address)` and selector `0xf2fde38b`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
    )]
    #[derive(Default)]
    #[ethcall(name = "transferOwnership", abi = "transferOwnership(address)")]
    pub struct TransferOwnershipCall {
        pub new_owner: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `whitelistToken` function with signature `whitelistToken(address,bool)` and selector `0x0ffb1d8b`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
    )]
    #[derive(Default)]
    #[ethcall(name = "whitelistToken", abi = "whitelistToken(address,bool)")]
    pub struct WhitelistTokenCall {
        pub token: ::ethers::core::types::Address,
        pub enabled: bool,
    }
    ///Container type for all input parameters for the `whitelistedTokens` function with signature `whitelistedTokens()` and selector `0x5e1762a0`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
    )]
    #[derive(Default)]
    #[ethcall(name = "whitelistedTokens", abi = "whitelistedTokens()")]
    pub struct WhitelistedTokensCall;
    ///Container type for all input parameters for the `whitelistedTokensCount` function with signature `whitelistedTokensCount()` and selector `0xdfdc4962`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
    )]
    #[derive(Default)]
    #[ethcall(name = "whitelistedTokensCount", abi = "whitelistedTokensCount()")]
    pub struct WhitelistedTokensCountCall;
    #[derive(Debug, Clone, PartialEq, Eq, ::ethers::contract::EthAbiType)]
    pub enum FloodRegistryCalls {
        Weth(WethCall),
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
    impl ::ethers::core::abi::AbiDecode for FloodRegistryCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::std::result::Result<Self, ::ethers::core::abi::AbiError> {
            if let Ok(decoded)
                = <WethCall as ::ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(FloodRegistryCalls::Weth(decoded));
            }
            if let Ok(decoded)
                = <AcceptOwnershipCall as ::ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                ) {
                return Ok(FloodRegistryCalls::AcceptOwnership(decoded));
            }
            if let Ok(decoded)
                = <BatchWhitelistTokensCall as ::ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                ) {
                return Ok(FloodRegistryCalls::BatchWhitelistTokens(decoded));
            }
            if let Ok(decoded)
                = <IsTokenWhitelistedCall as ::ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                ) {
                return Ok(FloodRegistryCalls::IsTokenWhitelisted(decoded));
            }
            if let Ok(decoded)
                = <LatestOracleCall as ::ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                ) {
                return Ok(FloodRegistryCalls::LatestOracle(decoded));
            }
            if let Ok(decoded)
                = <OwnerCall as ::ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(FloodRegistryCalls::Owner(decoded));
            }
            if let Ok(decoded)
                = <PendingOwnerCall as ::ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                ) {
                return Ok(FloodRegistryCalls::PendingOwner(decoded));
            }
            if let Ok(decoded)
                = <RenounceOwnershipCall as ::ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                ) {
                return Ok(FloodRegistryCalls::RenounceOwnership(decoded));
            }
            if let Ok(decoded)
                = <SetOracleCall as ::ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                ) {
                return Ok(FloodRegistryCalls::SetOracle(decoded));
            }
            if let Ok(decoded)
                = <TransferOwnershipCall as ::ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                ) {
                return Ok(FloodRegistryCalls::TransferOwnership(decoded));
            }
            if let Ok(decoded)
                = <WhitelistTokenCall as ::ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                ) {
                return Ok(FloodRegistryCalls::WhitelistToken(decoded));
            }
            if let Ok(decoded)
                = <WhitelistedTokensCall as ::ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                ) {
                return Ok(FloodRegistryCalls::WhitelistedTokens(decoded));
            }
            if let Ok(decoded)
                = <WhitelistedTokensCountCall as ::ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                ) {
                return Ok(FloodRegistryCalls::WhitelistedTokensCount(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for FloodRegistryCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                FloodRegistryCalls::Weth(element) => element.encode(),
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
                FloodRegistryCalls::Weth(element) => element.fmt(f),
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
    impl ::std::convert::From<WethCall> for FloodRegistryCalls {
        fn from(var: WethCall) -> Self {
            FloodRegistryCalls::Weth(var)
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
    ///Container type for all return fields from the `WETH` function with signature `WETH()` and selector `0xad5c4648`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
    )]
    #[derive(Default)]
    pub struct WethReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `isTokenWhitelisted` function with signature `isTokenWhitelisted(address)` and selector `0xb5af090f`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
    )]
    #[derive(Default)]
    pub struct IsTokenWhitelistedReturn(pub bool);
    ///Container type for all return fields from the `latestOracle` function with signature `latestOracle()` and selector `0x2eae4f60`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
    )]
    #[derive(Default)]
    pub struct LatestOracleReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `owner` function with signature `owner()` and selector `0x8da5cb5b`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
    )]
    #[derive(Default)]
    pub struct OwnerReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `pendingOwner` function with signature `pendingOwner()` and selector `0xe30c3978`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
    )]
    #[derive(Default)]
    pub struct PendingOwnerReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `whitelistedTokens` function with signature `whitelistedTokens()` and selector `0x5e1762a0`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
    )]
    #[derive(Default)]
    pub struct WhitelistedTokensReturn(
        pub ::std::vec::Vec<::ethers::core::types::Address>,
    );
    ///Container type for all return fields from the `whitelistedTokensCount` function with signature `whitelistedTokensCount()` and selector `0xdfdc4962`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
    )]
    #[derive(Default)]
    pub struct WhitelistedTokensCountReturn(pub ::ethers::core::types::U256);
}
