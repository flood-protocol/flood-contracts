pub use mock_requester::*;
#[allow(clippy::too_many_arguments, non_camel_case_types)]
pub mod mock_requester {
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
    #[doc = "MockRequester was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    # [rustfmt :: skip] const __ABI : & str = "[{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"id\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"_id\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"struct Request\",\"name\":\"_request\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"address\",\"name\":\"requester\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"proposer\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"disputer\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"contract ERC20\",\"name\":\"currency\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"bond\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"enum RequestState\",\"name\":\"state\",\"type\":\"uint8\",\"components\":[]},{\"internalType\":\"bool\",\"name\":\"answer\",\"type\":\"bool\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"data\",\"type\":\"bytes\",\"components\":[]}]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"onPriceSettled\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"price\",\"outputs\":[{\"internalType\":\"int256\",\"name\":\"\",\"type\":\"int256\",\"components\":[]}]}]" ;
    #[doc = r" The parsed JSON-ABI of the contract."]
    pub static MOCKREQUESTER_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            ethers::core::utils::__serde_json::from_str(__ABI).expect("invalid abi")
        });
    #[doc = r" Bytecode of the #name contract"]
    pub static MOCKREQUESTER_BYTECODE: ethers::contract::Lazy<ethers::core::types::Bytes> =
        ethers::contract::Lazy::new(|| {
            "0x6080604052600060015534801561001557600080fd5b506102e6806100256000396000f3fe608060405234801561001057600080fd5b50600436106100415760003560e01c8063734d162714610046578063a035b1fe1461005b578063af640d0f14610076575b600080fd5b6100596100543660046101ad565b61007f565b005b61006460015481565b60405190815260200160405180910390f35b61006460005481565b600082815560e0820151805161009d91602091810182019101610297565b600155505050565b634e487b7160e01b600052604160045260246000fd5b604051610100810167ffffffffffffffff811182821017156100df576100df6100a5565b60405290565b80356001600160a01b03811681146100fc57600080fd5b919050565b8035600381106100fc57600080fd5b803580151581146100fc57600080fd5b600082601f83011261013157600080fd5b813567ffffffffffffffff8082111561014c5761014c6100a5565b604051601f8301601f19908116603f01168101908282118183101715610174576101746100a5565b8160405283815286602085880101111561018d57600080fd5b836020870160208301376000602085830101528094505050505092915050565b600080604083850312156101c057600080fd5b82359150602083013567ffffffffffffffff808211156101df57600080fd5b9084019061010082870312156101f457600080fd5b6101fc6100bb565b610205836100e5565b8152610213602084016100e5565b6020820152610224604084016100e5565b6040820152610235606084016100e5565b60608201526080830135608082015261025060a08401610101565b60a082015261026160c08401610110565b60c082015260e08301358281111561027857600080fd5b61028488828601610120565b60e0830152508093505050509250929050565b6000602082840312156102a957600080fd5b505191905056fea2646970667358221220f8bced2baff4db5cb680ead7acbe52846a48a6ca503850c73ed08c00450b3f1764736f6c63430008110033" . parse () . expect ("invalid bytecode")
        });
    pub struct MockRequester<M>(ethers::contract::Contract<M>);
    impl<M> Clone for MockRequester<M> {
        fn clone(&self) -> Self {
            MockRequester(self.0.clone())
        }
    }
    impl<M> std::ops::Deref for MockRequester<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> std::fmt::Debug for MockRequester<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(MockRequester))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ethers::providers::Middleware> MockRequester<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(address.into(), MOCKREQUESTER_ABI.clone(), client)
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
                MOCKREQUESTER_ABI.clone(),
                MOCKREQUESTER_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        #[doc = "Calls the contract's `id` (0xaf640d0f) function"]
        pub fn id(&self) -> ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([175, 100, 13, 15], ())
                .expect("method not found (this should never happen)")
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
        #[doc = "Calls the contract's `price` (0xa035b1fe) function"]
        pub fn price(&self) -> ethers::contract::builders::ContractCall<M, I256> {
            self.0
                .method_hash([160, 53, 177, 254], ())
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>> for MockRequester<M> {
        fn from(contract: ethers::contract::Contract<M>) -> Self {
            Self(contract)
        }
    }
    #[doc = "Container type for all input parameters for the `id` function with signature `id()` and selector `[175, 100, 13, 15]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "id", abi = "id()")]
    pub struct IdCall;
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
    #[doc = "Container type for all input parameters for the `price` function with signature `price()` and selector `[160, 53, 177, 254]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "price", abi = "price()")]
    pub struct PriceCall;
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum MockRequesterCalls {
        Id(IdCall),
        OnPriceSettled(OnPriceSettledCall),
        Price(PriceCall),
    }
    impl ethers::core::abi::AbiDecode for MockRequesterCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::std::result::Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) = <IdCall as ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(MockRequesterCalls::Id(decoded));
            }
            if let Ok(decoded) =
                <OnPriceSettledCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MockRequesterCalls::OnPriceSettled(decoded));
            }
            if let Ok(decoded) = <PriceCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MockRequesterCalls::Price(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for MockRequesterCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                MockRequesterCalls::Id(element) => element.encode(),
                MockRequesterCalls::OnPriceSettled(element) => element.encode(),
                MockRequesterCalls::Price(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for MockRequesterCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                MockRequesterCalls::Id(element) => element.fmt(f),
                MockRequesterCalls::OnPriceSettled(element) => element.fmt(f),
                MockRequesterCalls::Price(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<IdCall> for MockRequesterCalls {
        fn from(var: IdCall) -> Self {
            MockRequesterCalls::Id(var)
        }
    }
    impl ::std::convert::From<OnPriceSettledCall> for MockRequesterCalls {
        fn from(var: OnPriceSettledCall) -> Self {
            MockRequesterCalls::OnPriceSettled(var)
        }
    }
    impl ::std::convert::From<PriceCall> for MockRequesterCalls {
        fn from(var: PriceCall) -> Self {
            MockRequesterCalls::Price(var)
        }
    }
    #[doc = "Container type for all return fields from the `id` function with signature `id()` and selector `[175, 100, 13, 15]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct IdReturn(pub [u8; 32]);
    #[doc = "Container type for all return fields from the `price` function with signature `price()` and selector `[160, 53, 177, 254]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct PriceReturn(pub I256);
}
