pub use all_knowing_oracle::*;
#[allow(clippy::too_many_arguments, non_camel_case_types)]
pub mod all_knowing_oracle {
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
    #[doc = "AllKnowingOracle was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    # [rustfmt :: skip] const __ABI : & str = "[{\"inputs\":[{\"internalType\":\"contract FloodRegistry\",\"name\":\"_registry\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"constructor\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"id\",\"type\":\"bytes32\",\"components\":[]}],\"type\":\"error\",\"name\":\"AllKnowingOracle__AlreadySettled\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"AllKnowingOracle__NonSettler\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"id\",\"type\":\"bytes32\",\"components\":[]}],\"type\":\"error\",\"name\":\"AllKnowingOracle__RequestAlreadyExists\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"id\",\"type\":\"bytes32\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"proposer\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"disputer\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"currency\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"bond\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"NewRequest\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"previousOwner\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"newOwner\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"OwnershipTransferStarted\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"previousOwner\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"newOwner\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"OwnershipTransferred\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"id\",\"type\":\"bytes32\",\"components\":[],\"indexed\":true},{\"internalType\":\"bool\",\"name\":\"answer\",\"type\":\"bool\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"RequestSettled\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"requester\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"bool\",\"name\":\"enabled\",\"type\":\"bool\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"RequesterWhitelisted\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"settler\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"bool\",\"name\":\"enabled\",\"type\":\"bool\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"SettlerWhitelisted\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"bool\",\"name\":\"enabled\",\"type\":\"bool\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"TokenWhitelisted\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"acceptOwnership\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"proposer\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"disputer\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"currency\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"bond\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"data\",\"type\":\"bytes\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"ask\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"id\",\"type\":\"bytes32\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"sender\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"proposer\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"disputer\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"currency\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"bond\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"getRequestId\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"owner\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"pendingOwner\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"registry\",\"outputs\":[{\"internalType\":\"contract FloodRegistry\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"renounceOwnership\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"requests\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"requester\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"proposer\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"disputer\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"contract IERC20\",\"name\":\"currency\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"bond\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"enum RequestState\",\"name\":\"state\",\"type\":\"uint8\",\"components\":[]},{\"internalType\":\"bool\",\"name\":\"answer\",\"type\":\"bool\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"data\",\"type\":\"bytes\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"id\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"bool\",\"name\":\"answer\",\"type\":\"bool\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"settle\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"settlers\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"newOwner\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"transferOwnership\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"settler\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bool\",\"name\":\"enabled\",\"type\":\"bool\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"whitelistSettler\",\"outputs\":[]}]" ;
    #[doc = r" The parsed JSON-ABI of the contract."]
    pub static ALLKNOWINGORACLE_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            ethers::core::utils::__serde_json::from_str(__ABI).expect("invalid abi")
        });
    #[doc = r" Bytecode of the #name contract"]
    pub static ALLKNOWINGORACLE_BYTECODE: ethers::contract::Lazy<ethers::core::types::Bytes> =
        ethers::contract::Lazy::new(|| {
            "0x60a060405234801561001057600080fd5b506040516113b73803806113b783398101604081905261002f916100da565b61003833610063565b336000908152600360205260409020805460ff191660011790556001600160a01b031660805261010a565b600180546001600160a01b03191690556100878161008a602090811b61086d17901c565b50565b600080546001600160a01b038381166001600160a01b0319831681178455604051919092169283917f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e09190a35050565b6000602082840312156100ec57600080fd5b81516001600160a01b038116811461010357600080fd5b9392505050565b608051611293610124600039600060e301526112936000f3fe608060405234801561001057600080fd5b50600436106100b45760003560e01c8063c70a900f11610071578063c70a900f1461015a578063e30c39781461018d578063e6717ce71461019e578063f2fde38b146101bf578063f7d3b58b146101d2578063fc361c38146101e557600080fd5b8063329e53be146100b9578063715018a6146100ce57806379ba5097146100d65780637b103999146100de5780638da5cb5b146101225780639d86698514610133575b600080fd5b6100cc6100c7366004610cc4565b6101f8565b005b6100cc61025f565b6100cc610273565b6101057f000000000000000000000000000000000000000000000000000000000000000081565b6040516001600160a01b0390911681526020015b60405180910390f35b6000546001600160a01b0316610105565b610146610141366004610cfb565b6102f2565b604051610119989796959493929190610d9c565b61017d610168366004610e02565b60036020526000908152604090205460ff1681565b6040519015158152602001610119565b6001546001600160a01b0316610105565b6101b16101ac366004610e24565b6103d8565b604051908152602001610119565b6100cc6101cd366004610e02565b6103f1565b6101b16101e0366004610e80565b610462565b6100cc6101f3366004610f2d565b6106c0565b6102006108bd565b6001600160a01b038216600081815260036020908152604091829020805460ff191685151590811790915591519182527f8cc72bec7e2cf5979aefd933f40a28eb590098d522a5c458e497e71c5e6fa90f910160405180910390a25050565b6102676108bd565b6102716000610917565b565b60015433906001600160a01b031681146102e65760405162461bcd60e51b815260206004820152602960248201527f4f776e61626c6532537465703a2063616c6c6572206973206e6f7420746865206044820152683732bb9037bbb732b960b91b60648201526084015b60405180910390fd5b6102ef81610917565b50565b600260208190526000918252604090912080546001820154928201546003830154600484015460058501546006860180546001600160a01b03968716988716979587169690941694929360ff808416946101009094041692919061035590610f52565b80601f016020809104026020016040519081016040528092919081815260200182805461038190610f52565b80156103ce5780601f106103a3576101008083540402835291602001916103ce565b820191906000526020600020905b8154815290600101906020018083116103b157829003601f168201915b5050505050905088565b60006103e78686868686610930565b9695505050505050565b6103f96108bd565b600180546001600160a01b0383166001600160a01b0319909116811790915561042a6000546001600160a01b031690565b6001600160a01b03167f38d16b8cac22d99fc7c124b9cd0de2d3fa1faef420bfe791d8c362d765e2270060405160405180910390a350565b60006104713388888888610930565b90506000808281526002602081905260409091206005015460ff169081111561049c5761049c610d14565b146104bd5760405163d5e880e960e01b8152600481018290526024016102dd565b6000604051806101000160405280336001600160a01b03168152602001896001600160a01b03168152602001886001600160a01b03168152602001876001600160a01b031681526020018681526020016001600281111561052057610520610d14565b815260200160001515815260200185858080601f016020809104026020016040519081016040528093929190818152602001838380828437600092018290525093909452505084815260026020818152604092839020855181546001600160a01b03199081166001600160a01b039283161783559287015160018084018054861692841692909217909155948701518285018054851691831691909117905560608701516003830180549094169116179091556080850151600482015560a085015160058201805496975087969295509093909260ff1990921691849081111561060c5761060c610d14565b021790555060c08201516005820180549115156101000261ff001990921691909117905560e082015160068201906106449082610ff0565b5050604080516001600160a01b03898116825260208201899052808b1693508b169185917f2609e116ca576195fecb47831f147fd6fee721377e4d88fff48807f3e9442a42910160405180910390a46106b533306106a38860026110b0565b6001600160a01b038a16929190610998565b509695505050505050565b3360009081526003602052604090205460ff166106f0576040516314e9dd3960e31b815260040160405180910390fd5b600082815260026020819052604090912090600582015460ff16600281111561071b5761071b610d14565b0361073c5760405163b677167760e01b8152600481018490526024016102dd565b60008160040154600261074f91906110b0565b60058301805461ffff19166101008615159081029190911760021790915560405190815290915084907fdbed7580b9c2829ee6b384e3833f10b16f9885601c98a01c40fd705b543e9c669060200160405180910390a282156107d157600182015460038301546107cc916001600160a01b03918216911683610a03565b6107f2565b600282015460038301546107f2916001600160a01b03918216911683610a03565b81546001600160a01b03163b1561086757815460405163734d162760e01b81526001600160a01b039091169063734d1627906108349087908690600401611158565b600060405180830381600087803b15801561084e57600080fd5b505af1158015610862573d6000803e3d6000fd5b505050505b50505050565b600080546001600160a01b038381166001600160a01b0319831681178455604051919092169283917f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e09190a35050565b6000546001600160a01b031633146102715760405162461bcd60e51b815260206004820181905260248201527f4f776e61626c653a2063616c6c6572206973206e6f7420746865206f776e657260448201526064016102dd565b600180546001600160a01b03191690556102ef8161086d565b6040516bffffffffffffffffffffffff19606087811b8216602084015286811b8216603484015285811b8216604884015284901b16605c8201526070810182905260009060900160405160208183030381529060405280519060200120905095945050505050565b6040516001600160a01b03808516602483015283166044820152606481018290526108679085906323b872dd60e01b906084015b60408051601f198184030181529190526020810180516001600160e01b03166001600160e01b031990931692909217909152610a38565b6040516001600160a01b038316602482015260448101829052610a3390849063a9059cbb60e01b906064016109cc565b505050565b6000610a8d826040518060400160405280602081526020017f5361666545524332303a206c6f772d6c6576656c2063616c6c206661696c6564815250856001600160a01b0316610b0a9092919063ffffffff16565b805190915015610a335780806020019051810190610aab9190611211565b610a335760405162461bcd60e51b815260206004820152602a60248201527f5361666545524332303a204552433230206f7065726174696f6e20646964206e6044820152691bdd081cdd58d8d9595960b21b60648201526084016102dd565b6060610b198484600085610b21565b949350505050565b606082471015610b825760405162461bcd60e51b815260206004820152602660248201527f416464726573733a20696e73756666696369656e742062616c616e636520666f6044820152651c8818d85b1b60d21b60648201526084016102dd565b600080866001600160a01b03168587604051610b9e919061122e565b60006040518083038185875af1925050503d8060008114610bdb576040519150601f19603f3d011682016040523d82523d6000602084013e610be0565b606091505b5091509150610bf187838387610bfc565b979650505050505050565b60608315610c6b578251600003610c64576001600160a01b0385163b610c645760405162461bcd60e51b815260206004820152601d60248201527f416464726573733a2063616c6c20746f206e6f6e2d636f6e747261637400000060448201526064016102dd565b5081610b19565b610b198383815115610c805781518083602001fd5b8060405162461bcd60e51b81526004016102dd919061124a565b80356001600160a01b0381168114610cb157600080fd5b919050565b80151581146102ef57600080fd5b60008060408385031215610cd757600080fd5b610ce083610c9a565b91506020830135610cf081610cb6565b809150509250929050565b600060208284031215610d0d57600080fd5b5035919050565b634e487b7160e01b600052602160045260246000fd5b60038110610d4857634e487b7160e01b600052602160045260246000fd5b9052565b60005b83811015610d67578181015183820152602001610d4f565b50506000910152565b60008151808452610d88816020860160208601610d4c565b601f01601f19169290920160200192915050565b6001600160a01b0389811682528881166020830152878116604083015286166060820152608081018590526000610100610dd960a0840187610d2a565b84151560c08401528060e0840152610df381840185610d70565b9b9a5050505050505050505050565b600060208284031215610e1457600080fd5b610e1d82610c9a565b9392505050565b600080600080600060a08688031215610e3c57600080fd5b610e4586610c9a565b9450610e5360208701610c9a565b9350610e6160408701610c9a565b9250610e6f60608701610c9a565b949793965091946080013592915050565b60008060008060008060a08789031215610e9957600080fd5b610ea287610c9a565b9550610eb060208801610c9a565b9450610ebe60408801610c9a565b935060608701359250608087013567ffffffffffffffff80821115610ee257600080fd5b818901915089601f830112610ef657600080fd5b813581811115610f0557600080fd5b8a6020828501011115610f1757600080fd5b6020830194508093505050509295509295509295565b60008060408385031215610f4057600080fd5b823591506020830135610cf081610cb6565b600181811c90821680610f6657607f821691505b602082108103610f8657634e487b7160e01b600052602260045260246000fd5b50919050565b634e487b7160e01b600052604160045260246000fd5b601f821115610a3357600081815260208120601f850160051c81016020861015610fc95750805b601f850160051c820191505b81811015610fe857828155600101610fd5565b505050505050565b815167ffffffffffffffff81111561100a5761100a610f8c565b61101e816110188454610f52565b84610fa2565b602080601f831160018114611053576000841561103b5750858301515b600019600386901b1c1916600185901b178555610fe8565b600085815260208120601f198616915b8281101561108257888601518255948401946001909101908401611063565b50858210156110a05787850151600019600388901b60f8161c191681555b5050505050600190811b01905550565b80820281158282048414176110d557634e487b7160e01b600052601160045260246000fd5b92915050565b600081546110e881610f52565b808552602060018381168015611105576001811461111f5761114d565b60ff1985168884015283151560051b88018301955061114d565b866000528260002060005b858110156111455781548a820186015290830190840161112a565b890184019650505b505050505092915050565b828152604060208201526111876040820161117a84546001600160a01b031690565b6001600160a01b03169052565b600061119d60018401546001600160a01b031690565b6001600160a01b03908116606084015260028401548116608084015260038401541660a0830152600483015460c083015260058301546111e360e0840160ff8316610d2a565b6101006111f981850160ff8460081c1615159052565b61012084015250610b196101408301600685016110db565b60006020828403121561122357600080fd5b8151610e1d81610cb6565b60008251611240818460208701610d4c565b9190910192915050565b602081526000610e1d6020830184610d7056fea2646970667358221220de6cdf94fa620eb9254400f444d9701ad6c7acd3b7b709b96fc7782b978df81264736f6c63430008110033" . parse () . expect ("invalid bytecode")
        });
    pub struct AllKnowingOracle<M>(ethers::contract::Contract<M>);
    impl<M> Clone for AllKnowingOracle<M> {
        fn clone(&self) -> Self {
            AllKnowingOracle(self.0.clone())
        }
    }
    impl<M> std::ops::Deref for AllKnowingOracle<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> std::fmt::Debug for AllKnowingOracle<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(AllKnowingOracle))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ethers::providers::Middleware> AllKnowingOracle<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(address.into(), ALLKNOWINGORACLE_ABI.clone(), client)
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
                ALLKNOWINGORACLE_ABI.clone(),
                ALLKNOWINGORACLE_BYTECODE.clone().into(),
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
        #[doc = "Calls the contract's `ask` (0xf7d3b58b) function"]
        pub fn ask(
            &self,
            proposer: ethers::core::types::Address,
            disputer: ethers::core::types::Address,
            currency: ethers::core::types::Address,
            bond: ethers::core::types::U256,
            data: ethers::core::types::Bytes,
        ) -> ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash(
                    [247, 211, 181, 139],
                    (proposer, disputer, currency, bond, data),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getRequestId` (0xe6717ce7) function"]
        pub fn get_request_id(
            &self,
            sender: ethers::core::types::Address,
            proposer: ethers::core::types::Address,
            disputer: ethers::core::types::Address,
            currency: ethers::core::types::Address,
            bond: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash(
                    [230, 113, 124, 231],
                    (sender, proposer, disputer, currency, bond),
                )
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
        #[doc = "Calls the contract's `registry` (0x7b103999) function"]
        pub fn registry(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([123, 16, 57, 153], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `renounceOwnership` (0x715018a6) function"]
        pub fn renounce_ownership(&self) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([113, 80, 24, 166], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `requests` (0x9d866985) function"]
        pub fn requests(
            &self,
            p0: [u8; 32],
        ) -> ethers::contract::builders::ContractCall<
            M,
            (
                ethers::core::types::Address,
                ethers::core::types::Address,
                ethers::core::types::Address,
                ethers::core::types::Address,
                ethers::core::types::U256,
                u8,
                bool,
                ethers::core::types::Bytes,
            ),
        > {
            self.0
                .method_hash([157, 134, 105, 133], p0)
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
        #[doc = "Calls the contract's `settlers` (0xc70a900f) function"]
        pub fn settlers(
            &self,
            p0: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([199, 10, 144, 15], p0)
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
        #[doc = "Calls the contract's `whitelistSettler` (0x329e53be) function"]
        pub fn whitelist_settler(
            &self,
            settler: ethers::core::types::Address,
            enabled: bool,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([50, 158, 83, 190], (settler, enabled))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Gets the contract's `NewRequest` event"]
        pub fn new_request_filter(&self) -> ethers::contract::builders::Event<M, NewRequestFilter> {
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
        #[doc = "Gets the contract's `RequestSettled` event"]
        pub fn request_settled_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, RequestSettledFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `RequesterWhitelisted` event"]
        pub fn requester_whitelisted_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, RequesterWhitelistedFilter> {
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
        pub fn events(&self) -> ethers::contract::builders::Event<M, AllKnowingOracleEvents> {
            self.0.event_with_filter(Default::default())
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>> for AllKnowingOracle<M> {
        fn from(contract: ethers::contract::Contract<M>) -> Self {
            Self(contract)
        }
    }
    #[doc = "Custom Error type `AllKnowingOracle__AlreadySettled` with signature `AllKnowingOracle__AlreadySettled(bytes32)` and selector `[182, 119, 22, 119]`"]
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
        name = "AllKnowingOracle__AlreadySettled",
        abi = "AllKnowingOracle__AlreadySettled(bytes32)"
    )]
    pub struct AllKnowingOracle__AlreadySettled {
        pub id: [u8; 32],
    }
    #[doc = "Custom Error type `AllKnowingOracle__NonSettler` with signature `AllKnowingOracle__NonSettler()` and selector `[167, 78, 233, 200]`"]
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
        name = "AllKnowingOracle__NonSettler",
        abi = "AllKnowingOracle__NonSettler()"
    )]
    pub struct AllKnowingOracle__NonSettler;
    #[doc = "Custom Error type `AllKnowingOracle__RequestAlreadyExists` with signature `AllKnowingOracle__RequestAlreadyExists(bytes32)` and selector `[213, 232, 128, 233]`"]
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
        name = "AllKnowingOracle__RequestAlreadyExists",
        abi = "AllKnowingOracle__RequestAlreadyExists(bytes32)"
    )]
    pub struct AllKnowingOracle__RequestAlreadyExists {
        pub id: [u8; 32],
    }
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum AllKnowingOracleErrors {
        AllKnowingOracle__AlreadySettled(AllKnowingOracle__AlreadySettled),
        AllKnowingOracle__NonSettler(AllKnowingOracle__NonSettler),
        AllKnowingOracle__RequestAlreadyExists(AllKnowingOracle__RequestAlreadyExists),
    }
    impl ethers::core::abi::AbiDecode for AllKnowingOracleErrors {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::std::result::Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <AllKnowingOracle__AlreadySettled as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(AllKnowingOracleErrors::AllKnowingOracle__AlreadySettled(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <AllKnowingOracle__NonSettler as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(AllKnowingOracleErrors::AllKnowingOracle__NonSettler(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <AllKnowingOracle__RequestAlreadyExists as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(AllKnowingOracleErrors::AllKnowingOracle__RequestAlreadyExists(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for AllKnowingOracleErrors {
        fn encode(self) -> Vec<u8> {
            match self {
                AllKnowingOracleErrors::AllKnowingOracle__AlreadySettled(element) => {
                    element.encode()
                }
                AllKnowingOracleErrors::AllKnowingOracle__NonSettler(element) => element.encode(),
                AllKnowingOracleErrors::AllKnowingOracle__RequestAlreadyExists(element) => {
                    element.encode()
                }
            }
        }
    }
    impl ::std::fmt::Display for AllKnowingOracleErrors {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                AllKnowingOracleErrors::AllKnowingOracle__AlreadySettled(element) => element.fmt(f),
                AllKnowingOracleErrors::AllKnowingOracle__NonSettler(element) => element.fmt(f),
                AllKnowingOracleErrors::AllKnowingOracle__RequestAlreadyExists(element) => {
                    element.fmt(f)
                }
            }
        }
    }
    impl ::std::convert::From<AllKnowingOracle__AlreadySettled> for AllKnowingOracleErrors {
        fn from(var: AllKnowingOracle__AlreadySettled) -> Self {
            AllKnowingOracleErrors::AllKnowingOracle__AlreadySettled(var)
        }
    }
    impl ::std::convert::From<AllKnowingOracle__NonSettler> for AllKnowingOracleErrors {
        fn from(var: AllKnowingOracle__NonSettler) -> Self {
            AllKnowingOracleErrors::AllKnowingOracle__NonSettler(var)
        }
    }
    impl ::std::convert::From<AllKnowingOracle__RequestAlreadyExists> for AllKnowingOracleErrors {
        fn from(var: AllKnowingOracle__RequestAlreadyExists) -> Self {
            AllKnowingOracleErrors::AllKnowingOracle__RequestAlreadyExists(var)
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
    #[ethevent(
        name = "NewRequest",
        abi = "NewRequest(bytes32,address,address,address,uint256)"
    )]
    pub struct NewRequestFilter {
        #[ethevent(indexed)]
        pub id: [u8; 32],
        #[ethevent(indexed)]
        pub proposer: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub disputer: ethers::core::types::Address,
        pub currency: ethers::core::types::Address,
        pub bond: ethers::core::types::U256,
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
    #[ethevent(name = "RequestSettled", abi = "RequestSettled(bytes32,bool)")]
    pub struct RequestSettledFilter {
        #[ethevent(indexed)]
        pub id: [u8; 32],
        pub answer: bool,
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
        name = "RequesterWhitelisted",
        abi = "RequesterWhitelisted(address,bool)"
    )]
    pub struct RequesterWhitelistedFilter {
        #[ethevent(indexed)]
        pub requester: ethers::core::types::Address,
        pub enabled: bool,
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
    #[ethevent(name = "SettlerWhitelisted", abi = "SettlerWhitelisted(address,bool)")]
    pub struct SettlerWhitelistedFilter {
        #[ethevent(indexed)]
        pub settler: ethers::core::types::Address,
        pub enabled: bool,
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
        pub enabled: bool,
    }
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum AllKnowingOracleEvents {
        NewRequestFilter(NewRequestFilter),
        OwnershipTransferStartedFilter(OwnershipTransferStartedFilter),
        OwnershipTransferredFilter(OwnershipTransferredFilter),
        RequestSettledFilter(RequestSettledFilter),
        RequesterWhitelistedFilter(RequesterWhitelistedFilter),
        SettlerWhitelistedFilter(SettlerWhitelistedFilter),
        TokenWhitelistedFilter(TokenWhitelistedFilter),
    }
    impl ethers::contract::EthLogDecode for AllKnowingOracleEvents {
        fn decode_log(
            log: &ethers::core::abi::RawLog,
        ) -> ::std::result::Result<Self, ethers::core::abi::Error>
        where
            Self: Sized,
        {
            if let Ok(decoded) = NewRequestFilter::decode_log(log) {
                return Ok(AllKnowingOracleEvents::NewRequestFilter(decoded));
            }
            if let Ok(decoded) = OwnershipTransferStartedFilter::decode_log(log) {
                return Ok(AllKnowingOracleEvents::OwnershipTransferStartedFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = OwnershipTransferredFilter::decode_log(log) {
                return Ok(AllKnowingOracleEvents::OwnershipTransferredFilter(decoded));
            }
            if let Ok(decoded) = RequestSettledFilter::decode_log(log) {
                return Ok(AllKnowingOracleEvents::RequestSettledFilter(decoded));
            }
            if let Ok(decoded) = RequesterWhitelistedFilter::decode_log(log) {
                return Ok(AllKnowingOracleEvents::RequesterWhitelistedFilter(decoded));
            }
            if let Ok(decoded) = SettlerWhitelistedFilter::decode_log(log) {
                return Ok(AllKnowingOracleEvents::SettlerWhitelistedFilter(decoded));
            }
            if let Ok(decoded) = TokenWhitelistedFilter::decode_log(log) {
                return Ok(AllKnowingOracleEvents::TokenWhitelistedFilter(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::std::fmt::Display for AllKnowingOracleEvents {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                AllKnowingOracleEvents::NewRequestFilter(element) => element.fmt(f),
                AllKnowingOracleEvents::OwnershipTransferStartedFilter(element) => element.fmt(f),
                AllKnowingOracleEvents::OwnershipTransferredFilter(element) => element.fmt(f),
                AllKnowingOracleEvents::RequestSettledFilter(element) => element.fmt(f),
                AllKnowingOracleEvents::RequesterWhitelistedFilter(element) => element.fmt(f),
                AllKnowingOracleEvents::SettlerWhitelistedFilter(element) => element.fmt(f),
                AllKnowingOracleEvents::TokenWhitelistedFilter(element) => element.fmt(f),
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
    #[doc = "Container type for all input parameters for the `ask` function with signature `ask(address,address,address,uint256,bytes)` and selector `[247, 211, 181, 139]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "ask", abi = "ask(address,address,address,uint256,bytes)")]
    pub struct AskCall {
        pub proposer: ethers::core::types::Address,
        pub disputer: ethers::core::types::Address,
        pub currency: ethers::core::types::Address,
        pub bond: ethers::core::types::U256,
        pub data: ethers::core::types::Bytes,
    }
    #[doc = "Container type for all input parameters for the `getRequestId` function with signature `getRequestId(address,address,address,address,uint256)` and selector `[230, 113, 124, 231]`"]
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
        name = "getRequestId",
        abi = "getRequestId(address,address,address,address,uint256)"
    )]
    pub struct GetRequestIdCall {
        pub sender: ethers::core::types::Address,
        pub proposer: ethers::core::types::Address,
        pub disputer: ethers::core::types::Address,
        pub currency: ethers::core::types::Address,
        pub bond: ethers::core::types::U256,
    }
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
    #[doc = "Container type for all input parameters for the `registry` function with signature `registry()` and selector `[123, 16, 57, 153]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "registry", abi = "registry()")]
    pub struct RegistryCall;
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
    #[doc = "Container type for all input parameters for the `requests` function with signature `requests(bytes32)` and selector `[157, 134, 105, 133]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "requests", abi = "requests(bytes32)")]
    pub struct RequestsCall(pub [u8; 32]);
    #[doc = "Container type for all input parameters for the `settle` function with signature `settle(bytes32,bool)` and selector `[252, 54, 28, 56]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "settle", abi = "settle(bytes32,bool)")]
    pub struct SettleCall {
        pub id: [u8; 32],
        pub answer: bool,
    }
    #[doc = "Container type for all input parameters for the `settlers` function with signature `settlers(address)` and selector `[199, 10, 144, 15]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "settlers", abi = "settlers(address)")]
    pub struct SettlersCall(pub ethers::core::types::Address);
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
    #[doc = "Container type for all input parameters for the `whitelistSettler` function with signature `whitelistSettler(address,bool)` and selector `[50, 158, 83, 190]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "whitelistSettler", abi = "whitelistSettler(address,bool)")]
    pub struct WhitelistSettlerCall {
        pub settler: ethers::core::types::Address,
        pub enabled: bool,
    }
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum AllKnowingOracleCalls {
        AcceptOwnership(AcceptOwnershipCall),
        Ask(AskCall),
        GetRequestId(GetRequestIdCall),
        Owner(OwnerCall),
        PendingOwner(PendingOwnerCall),
        Registry(RegistryCall),
        RenounceOwnership(RenounceOwnershipCall),
        Requests(RequestsCall),
        Settle(SettleCall),
        Settlers(SettlersCall),
        TransferOwnership(TransferOwnershipCall),
        WhitelistSettler(WhitelistSettlerCall),
    }
    impl ethers::core::abi::AbiDecode for AllKnowingOracleCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::std::result::Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <AcceptOwnershipCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(AllKnowingOracleCalls::AcceptOwnership(decoded));
            }
            if let Ok(decoded) = <AskCall as ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(AllKnowingOracleCalls::Ask(decoded));
            }
            if let Ok(decoded) =
                <GetRequestIdCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(AllKnowingOracleCalls::GetRequestId(decoded));
            }
            if let Ok(decoded) = <OwnerCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(AllKnowingOracleCalls::Owner(decoded));
            }
            if let Ok(decoded) =
                <PendingOwnerCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(AllKnowingOracleCalls::PendingOwner(decoded));
            }
            if let Ok(decoded) =
                <RegistryCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(AllKnowingOracleCalls::Registry(decoded));
            }
            if let Ok(decoded) =
                <RenounceOwnershipCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(AllKnowingOracleCalls::RenounceOwnership(decoded));
            }
            if let Ok(decoded) =
                <RequestsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(AllKnowingOracleCalls::Requests(decoded));
            }
            if let Ok(decoded) = <SettleCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(AllKnowingOracleCalls::Settle(decoded));
            }
            if let Ok(decoded) =
                <SettlersCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(AllKnowingOracleCalls::Settlers(decoded));
            }
            if let Ok(decoded) =
                <TransferOwnershipCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(AllKnowingOracleCalls::TransferOwnership(decoded));
            }
            if let Ok(decoded) =
                <WhitelistSettlerCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(AllKnowingOracleCalls::WhitelistSettler(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for AllKnowingOracleCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                AllKnowingOracleCalls::AcceptOwnership(element) => element.encode(),
                AllKnowingOracleCalls::Ask(element) => element.encode(),
                AllKnowingOracleCalls::GetRequestId(element) => element.encode(),
                AllKnowingOracleCalls::Owner(element) => element.encode(),
                AllKnowingOracleCalls::PendingOwner(element) => element.encode(),
                AllKnowingOracleCalls::Registry(element) => element.encode(),
                AllKnowingOracleCalls::RenounceOwnership(element) => element.encode(),
                AllKnowingOracleCalls::Requests(element) => element.encode(),
                AllKnowingOracleCalls::Settle(element) => element.encode(),
                AllKnowingOracleCalls::Settlers(element) => element.encode(),
                AllKnowingOracleCalls::TransferOwnership(element) => element.encode(),
                AllKnowingOracleCalls::WhitelistSettler(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for AllKnowingOracleCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                AllKnowingOracleCalls::AcceptOwnership(element) => element.fmt(f),
                AllKnowingOracleCalls::Ask(element) => element.fmt(f),
                AllKnowingOracleCalls::GetRequestId(element) => element.fmt(f),
                AllKnowingOracleCalls::Owner(element) => element.fmt(f),
                AllKnowingOracleCalls::PendingOwner(element) => element.fmt(f),
                AllKnowingOracleCalls::Registry(element) => element.fmt(f),
                AllKnowingOracleCalls::RenounceOwnership(element) => element.fmt(f),
                AllKnowingOracleCalls::Requests(element) => element.fmt(f),
                AllKnowingOracleCalls::Settle(element) => element.fmt(f),
                AllKnowingOracleCalls::Settlers(element) => element.fmt(f),
                AllKnowingOracleCalls::TransferOwnership(element) => element.fmt(f),
                AllKnowingOracleCalls::WhitelistSettler(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<AcceptOwnershipCall> for AllKnowingOracleCalls {
        fn from(var: AcceptOwnershipCall) -> Self {
            AllKnowingOracleCalls::AcceptOwnership(var)
        }
    }
    impl ::std::convert::From<AskCall> for AllKnowingOracleCalls {
        fn from(var: AskCall) -> Self {
            AllKnowingOracleCalls::Ask(var)
        }
    }
    impl ::std::convert::From<GetRequestIdCall> for AllKnowingOracleCalls {
        fn from(var: GetRequestIdCall) -> Self {
            AllKnowingOracleCalls::GetRequestId(var)
        }
    }
    impl ::std::convert::From<OwnerCall> for AllKnowingOracleCalls {
        fn from(var: OwnerCall) -> Self {
            AllKnowingOracleCalls::Owner(var)
        }
    }
    impl ::std::convert::From<PendingOwnerCall> for AllKnowingOracleCalls {
        fn from(var: PendingOwnerCall) -> Self {
            AllKnowingOracleCalls::PendingOwner(var)
        }
    }
    impl ::std::convert::From<RegistryCall> for AllKnowingOracleCalls {
        fn from(var: RegistryCall) -> Self {
            AllKnowingOracleCalls::Registry(var)
        }
    }
    impl ::std::convert::From<RenounceOwnershipCall> for AllKnowingOracleCalls {
        fn from(var: RenounceOwnershipCall) -> Self {
            AllKnowingOracleCalls::RenounceOwnership(var)
        }
    }
    impl ::std::convert::From<RequestsCall> for AllKnowingOracleCalls {
        fn from(var: RequestsCall) -> Self {
            AllKnowingOracleCalls::Requests(var)
        }
    }
    impl ::std::convert::From<SettleCall> for AllKnowingOracleCalls {
        fn from(var: SettleCall) -> Self {
            AllKnowingOracleCalls::Settle(var)
        }
    }
    impl ::std::convert::From<SettlersCall> for AllKnowingOracleCalls {
        fn from(var: SettlersCall) -> Self {
            AllKnowingOracleCalls::Settlers(var)
        }
    }
    impl ::std::convert::From<TransferOwnershipCall> for AllKnowingOracleCalls {
        fn from(var: TransferOwnershipCall) -> Self {
            AllKnowingOracleCalls::TransferOwnership(var)
        }
    }
    impl ::std::convert::From<WhitelistSettlerCall> for AllKnowingOracleCalls {
        fn from(var: WhitelistSettlerCall) -> Self {
            AllKnowingOracleCalls::WhitelistSettler(var)
        }
    }
    #[doc = "Container type for all return fields from the `ask` function with signature `ask(address,address,address,uint256,bytes)` and selector `[247, 211, 181, 139]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct AskReturn {
        pub id: [u8; 32],
    }
    #[doc = "Container type for all return fields from the `getRequestId` function with signature `getRequestId(address,address,address,address,uint256)` and selector `[230, 113, 124, 231]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct GetRequestIdReturn(pub [u8; 32]);
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
    #[doc = "Container type for all return fields from the `registry` function with signature `registry()` and selector `[123, 16, 57, 153]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct RegistryReturn(pub ethers::core::types::Address);
    #[doc = "Container type for all return fields from the `requests` function with signature `requests(bytes32)` and selector `[157, 134, 105, 133]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct RequestsReturn {
        pub requester: ethers::core::types::Address,
        pub proposer: ethers::core::types::Address,
        pub disputer: ethers::core::types::Address,
        pub currency: ethers::core::types::Address,
        pub bond: ethers::core::types::U256,
        pub state: u8,
        pub answer: bool,
        pub data: ethers::core::types::Bytes,
    }
    #[doc = "Container type for all return fields from the `settlers` function with signature `settlers(address)` and selector `[199, 10, 144, 15]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct SettlersReturn(pub bool);
}
