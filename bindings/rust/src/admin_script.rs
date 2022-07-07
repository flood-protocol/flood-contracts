pub use adminscript_mod::*;
#[allow(clippy::too_many_arguments, non_camel_case_types)]
pub mod adminscript_mod {
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
    #[doc = "AdminScript was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static ADMINSCRIPT_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            serde_json :: from_str ("[{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"vm\",\"outputs\":[{\"internalType\":\"contract Vm\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"whitelistToken\",\"outputs\":[]}]") . expect ("invalid abi")
        });
    #[doc = r" Bytecode of the #name contract"]
    pub static ADMINSCRIPT_BYTECODE: ethers::contract::Lazy<ethers::core::types::Bytes> =
        ethers::contract::Lazy::new(|| {
            "0x6080604052600080546001600160a01b031990811673d7a2cc8ad2a9b3bf0704367cb021111404a78457179091556001805490911673489a3cc661d72595971cc97fc53ddf466931e00e17905534801561005857600080fd5b50610285806100686000396000f3fe608060405234801561001057600080fd5b50600436106100365760003560e01c80633a7684631461003b5780636247f6f214610072575b600080fd5b610056737109709ecfa91a80626ff3989d68f67f5b1dd12d81565b6040516001600160a01b03909116815260200160405180910390f35b61008561008036600461021f565b610087565b005b60408051637fb5297f60e01b81529051737109709ecfa91a80626ff3989d68f67f5b1dd12d91637fb5297f91600480830192600092919082900301818387803b1580156100d357600080fd5b505af11580156100e7573d6000803e3d6000fd5b505060018054604051630ffb1d8b60e01b81526001600160a01b038681166004830152602482019390935291169250630ffb1d8b9150604401600060405180830381600087803b15801561013a57600080fd5b505af115801561014e573d6000803e3d6000fd5b5050600054604051630ffb1d8b60e01b81526001600160a01b038581166004830152600160248301529091169250630ffb1d8b9150604401600060405180830381600087803b1580156101a057600080fd5b505af11580156101b4573d6000803e3d6000fd5b505060408051633b756e9b60e11b81529051737109709ecfa91a80626ff3989d68f67f5b1dd12d93506376eadd369250600480830192600092919082900301818387803b15801561020457600080fd5b505af1158015610218573d6000803e3d6000fd5b5050505050565b60006020828403121561023157600080fd5b81356001600160a01b038116811461024857600080fd5b939250505056fea26469706673582212205234710f05a55ac878aa0b65496a165f26b3cdabcf37064b1c3b7f651af9d7fa64736f6c634300080f0033" . parse () . expect ("invalid bytecode")
        });
    pub struct AdminScript<M>(ethers::contract::Contract<M>);
    impl<M> Clone for AdminScript<M> {
        fn clone(&self) -> Self {
            AdminScript(self.0.clone())
        }
    }
    impl<M> std::ops::Deref for AdminScript<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug for AdminScript<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(AdminScript))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ethers::providers::Middleware> AdminScript<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(address.into(), ADMINSCRIPT_ABI.clone(), client).into()
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
        ) -> Result<
            ethers::contract::builders::ContractDeployer<M, Self>,
            ethers::contract::ContractError<M>,
        > {
            let factory = ethers::contract::ContractFactory::new(
                ADMINSCRIPT_ABI.clone(),
                ADMINSCRIPT_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        #[doc = "Calls the contract's `vm` (0x3a768463) function"]
        pub fn vm(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([58, 118, 132, 99], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `whitelistToken` (0x6247f6f2) function"]
        pub fn whitelist_token(
            &self,
            token: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([98, 71, 246, 242], token)
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>> for AdminScript<M> {
        fn from(contract: ethers::contract::Contract<M>) -> Self {
            Self(contract)
        }
    }
    #[doc = "Container type for all input parameters for the `vm`function with signature `vm()` and selector `[58, 118, 132, 99]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "vm", abi = "vm()")]
    pub struct VmCall;
    #[doc = "Container type for all input parameters for the `whitelistToken`function with signature `whitelistToken(address)` and selector `[98, 71, 246, 242]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "whitelistToken", abi = "whitelistToken(address)")]
    pub struct WhitelistTokenCall {
        pub token: ethers::core::types::Address,
    }
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum AdminScriptCalls {
        Vm(VmCall),
        WhitelistToken(WhitelistTokenCall),
    }
    impl ethers::core::abi::AbiDecode for AdminScriptCalls {
        fn decode(data: impl AsRef<[u8]>) -> Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) = <VmCall as ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(AdminScriptCalls::Vm(decoded));
            }
            if let Ok(decoded) =
                <WhitelistTokenCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(AdminScriptCalls::WhitelistToken(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for AdminScriptCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                AdminScriptCalls::Vm(element) => element.encode(),
                AdminScriptCalls::WhitelistToken(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for AdminScriptCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                AdminScriptCalls::Vm(element) => element.fmt(f),
                AdminScriptCalls::WhitelistToken(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<VmCall> for AdminScriptCalls {
        fn from(var: VmCall) -> Self {
            AdminScriptCalls::Vm(var)
        }
    }
    impl ::std::convert::From<WhitelistTokenCall> for AdminScriptCalls {
        fn from(var: WhitelistTokenCall) -> Self {
            AdminScriptCalls::WhitelistToken(var)
        }
    }
}
