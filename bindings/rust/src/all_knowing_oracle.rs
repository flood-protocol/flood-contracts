pub use all_knowing_oracle::*;
#[allow(clippy::too_many_arguments, non_camel_case_types)]
pub mod all_knowing_oracle {
    #![allow(clippy::enum_variant_names)]
    #![allow(dead_code)]
    #![allow(clippy::type_complexity)]
    #![allow(unused_imports)]
    ///AllKnowingOracle was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs
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
    const __ABI: &str = "[{\"inputs\":[{\"internalType\":\"contract FloodRegistry\",\"name\":\"_registry\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"constructor\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"AllKnowingOracle__NonSettler\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"id\",\"type\":\"bytes32\",\"components\":[]}],\"type\":\"error\",\"name\":\"AllKnowingOracle__NotSettleable\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"id\",\"type\":\"bytes32\",\"components\":[]}],\"type\":\"error\",\"name\":\"AllKnowingOracle__SettlerIsDisputerOrProposer\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"id\",\"type\":\"bytes32\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"proposer\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"disputer\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"currency\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"bond\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"requestIndex\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"NewRequest\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"previousOwner\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"newOwner\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"OwnershipTransferStarted\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"previousOwner\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"newOwner\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"OwnershipTransferred\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"id\",\"type\":\"bytes32\",\"components\":[],\"indexed\":true},{\"internalType\":\"bool\",\"name\":\"answer\",\"type\":\"bool\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"RequestSettled\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"requester\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"bool\",\"name\":\"enabled\",\"type\":\"bool\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"RequesterWhitelisted\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"settler\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"bool\",\"name\":\"enabled\",\"type\":\"bool\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"SettlerWhitelisted\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"bool\",\"name\":\"enabled\",\"type\":\"bool\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"TokenWhitelisted\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"acceptOwnership\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"proposer\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"disputer\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"currency\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"bond\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"data\",\"type\":\"bytes\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"ask\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"id\",\"type\":\"bytes32\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"sender\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"proposer\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"disputer\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"currency\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"bond\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"requestIndex\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"getRequestId\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"owner\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"pendingOwner\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"registry\",\"outputs\":[{\"internalType\":\"contract FloodRegistry\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"renounceOwnership\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"requestCount\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"requests\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"requester\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"proposer\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"disputer\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"contract IERC20\",\"name\":\"currency\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"bond\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"enum RequestState\",\"name\":\"state\",\"type\":\"uint8\",\"components\":[]},{\"internalType\":\"bool\",\"name\":\"answer\",\"type\":\"bool\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"data\",\"type\":\"bytes\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"id\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"bool\",\"name\":\"answer\",\"type\":\"bool\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"settle\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"settlers\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"newOwner\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"transferOwnership\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"settler\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bool\",\"name\":\"enabled\",\"type\":\"bool\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"whitelistSettler\",\"outputs\":[]}]";
    /// The parsed JSON-ABI of the contract.
    pub static ALLKNOWINGORACLE_ABI: ::ethers::contract::Lazy<
        ::ethers::core::abi::Abi,
    > = ::ethers::contract::Lazy::new(|| {
        ::ethers::core::utils::__serde_json::from_str(__ABI).expect("invalid abi")
    });
    /// Bytecode of the #name contract
    pub static ALLKNOWINGORACLE_BYTECODE: ::ethers::contract::Lazy<
        ::ethers::core::types::Bytes,
    > = ::ethers::contract::Lazy::new(|| {
        "0x60a0604052600060045534801561001557600080fd5b50604051611449380380611449833981016040819052610034916100df565b61003d33610068565b336000908152600360205260409020805460ff191660011790556001600160a01b031660805261010f565b600180546001600160a01b031916905561008c8161008f602090811b6108cb17901c565b50565b600080546001600160a01b038381166001600160a01b0319831681178455604051919092169283917f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e09190a35050565b6000602082840312156100f157600080fd5b81516001600160a01b038116811461010857600080fd5b9392505050565b60805161131f61012a600039600061012d015261131f6000f3fe608060405234801561001057600080fd5b50600436106100cf5760003560e01c80638da5cb5b1161008c578063e30c397811610066578063e30c3978146101d2578063f2fde38b146101e3578063f7d3b58b146101f6578063fc361c381461020957600080fd5b80638da5cb5b146101675780639d86698514610178578063c70a900f1461019f57600080fd5b8063329e53be146100d457806355352f82146100e95780635badbe4c1461010f578063715018a61461011857806379ba5097146101205780637b10399914610128575b600080fd5b6100e76100e2366004610d12565b61021c565b005b6100fc6100f7366004610d49565b610283565b6040519081526020015b60405180910390f35b6100fc60045481565b6100e761029e565b6100e76102b2565b61014f7f000000000000000000000000000000000000000000000000000000000000000081565b6040516001600160a01b039091168152602001610106565b6000546001600160a01b031661014f565b61018b610186366004610daf565b610331565b604051610106989796959493929190610e50565b6101c26101ad366004610eb6565b60036020526000908152604090205460ff1681565b6040519015158152602001610106565b6001546001600160a01b031661014f565b6100e76101f1366004610eb6565b610417565b6100fc610204366004610ed8565b610488565b6100e7610217366004610f85565b6106d8565b61022461091b565b6001600160a01b038216600081815260036020908152604091829020805460ff191685151590811790915591519182527f8cc72bec7e2cf5979aefd933f40a28eb590098d522a5c458e497e71c5e6fa90f910160405180910390a25050565b6000610293878787878787610975565b979650505050505050565b6102a661091b565b6102b060006109dd565b565b60015433906001600160a01b031681146103255760405162461bcd60e51b815260206004820152602960248201527f4f776e61626c6532537465703a2063616c6c6572206973206e6f7420746865206044820152683732bb9037bbb732b960b91b60648201526084015b60405180910390fd5b61032e816109dd565b50565b600260208190526000918252604090912080546001820154928201546003830154600484015460058501546006860180546001600160a01b03968716988716979587169690941694929360ff808416946101009094041692919061039490610faa565b80601f01602080910402602001604051908101604052809291908181526020018280546103c090610faa565b801561040d5780601f106103e25761010080835404028352916020019161040d565b820191906000526020600020905b8154815290600101906020018083116103f057829003601f168201915b5050505050905088565b61041f61091b565b600180546001600160a01b0383166001600160a01b031990911681179091556104506000546001600160a01b031690565b6001600160a01b03167f38d16b8cac22d99fc7c124b9cd0de2d3fa1faef420bfe791d8c362d765e2270060405160405180910390a350565b600061049a3388888888600454610975565b6004805491925060006104ac83610ffa565b91905055506000604051806101000160405280336001600160a01b03168152602001896001600160a01b03168152602001886001600160a01b03168152602001876001600160a01b031681526020018681526020016001600281111561051457610514610dc8565b815260200160001515815260200185858080601f016020809104026020016040519081016040528093929190818152602001838380828437600092018290525093909452505084815260026020818152604092839020855181546001600160a01b03199081166001600160a01b039283161783559287015160018084018054861692841692909217909155948701518285018054851691831691909117905560608701516003830180549094169116179091556080850151600482015560a085015160058201805496975087969295509093909260ff1990921691849081111561060057610600610dc8565b021790555060c08201516005820180549115156101000261ff001990921691909117905560e082015160068201906106389082611077565b50905050866001600160a01b0316886001600160a01b0316837fb10e0c42d0d8cd4a999f95c50d18109fb205fdf5e55a8ed89121cab7f9f65f69898960016004546106839190611137565b604080516001600160a01b03909416845260208401929092529082015260600160405180910390a46106cd33306106bb886002611150565b6001600160a01b038a169291906109f6565b509695505050505050565b3360009081526003602052604090205460ff16610708576040516314e9dd3960e31b815260040160405180910390fd5b60008281526002602052604090206001600582015460ff16600281111561073157610731610dc8565b1461075257604051631ad52b9b60e11b81526004810184905260240161031c565b60028101546001600160a01b0316331480610779575060018101546001600160a01b031633145b1561079a576040516371ced53d60e01b81526004810184905260240161031c565b6000816004015460026107ad9190611150565b60058301805461ffff19166101008615159081029190911760021790915560405190815290915084907fdbed7580b9c2829ee6b384e3833f10b16f9885601c98a01c40fd705b543e9c669060200160405180910390a2821561082f576001820154600383015461082a916001600160a01b03918216911683610a61565b610850565b60028201546003830154610850916001600160a01b03918216911683610a61565b81546001600160a01b03163b156108c557815460405163734d162760e01b81526001600160a01b039091169063734d16279061089290879086906004016111e4565b600060405180830381600087803b1580156108ac57600080fd5b505af11580156108c0573d6000803e3d6000fd5b505050505b50505050565b600080546001600160a01b038381166001600160a01b0319831681178455604051919092169283917f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e09190a35050565b6000546001600160a01b031633146102b05760405162461bcd60e51b815260206004820181905260248201527f4f776e61626c653a2063616c6c6572206973206e6f7420746865206f776e6572604482015260640161031c565b604080516bffffffffffffffffffffffff19606098891b811660208084019190915297891b8116603483015295881b861660488201529390961b909316605c83015260708201526090808201929092528351808203909201825260b001909252815191012090565b600180546001600160a01b031916905561032e816108cb565b6040516001600160a01b03808516602483015283166044820152606481018290526108c59085906323b872dd60e01b906084015b60408051601f198184030181529190526020810180516001600160e01b03166001600160e01b031990931692909217909152610a96565b6040516001600160a01b038316602482015260448101829052610a9190849063a9059cbb60e01b90606401610a2a565b505050565b6000610aeb826040518060400160405280602081526020017f5361666545524332303a206c6f772d6c6576656c2063616c6c206661696c6564815250856001600160a01b0316610b689092919063ffffffff16565b805190915015610a915780806020019051810190610b09919061129d565b610a915760405162461bcd60e51b815260206004820152602a60248201527f5361666545524332303a204552433230206f7065726174696f6e20646964206e6044820152691bdd081cdd58d8d9595960b21b606482015260840161031c565b6060610b778484600085610b7f565b949350505050565b606082471015610be05760405162461bcd60e51b815260206004820152602660248201527f416464726573733a20696e73756666696369656e742062616c616e636520666f6044820152651c8818d85b1b60d21b606482015260840161031c565b600080866001600160a01b03168587604051610bfc91906112ba565b60006040518083038185875af1925050503d8060008114610c39576040519150601f19603f3d011682016040523d82523d6000602084013e610c3e565b606091505b50915091506102938783838760608315610cb9578251600003610cb2576001600160a01b0385163b610cb25760405162461bcd60e51b815260206004820152601d60248201527f416464726573733a2063616c6c20746f206e6f6e2d636f6e7472616374000000604482015260640161031c565b5081610b77565b610b778383815115610cce5781518083602001fd5b8060405162461bcd60e51b815260040161031c91906112d6565b80356001600160a01b0381168114610cff57600080fd5b919050565b801515811461032e57600080fd5b60008060408385031215610d2557600080fd5b610d2e83610ce8565b91506020830135610d3e81610d04565b809150509250929050565b60008060008060008060c08789031215610d6257600080fd5b610d6b87610ce8565b9550610d7960208801610ce8565b9450610d8760408801610ce8565b9350610d9560608801610ce8565b92506080870135915060a087013590509295509295509295565b600060208284031215610dc157600080fd5b5035919050565b634e487b7160e01b600052602160045260246000fd5b60038110610dfc57634e487b7160e01b600052602160045260246000fd5b9052565b60005b83811015610e1b578181015183820152602001610e03565b50506000910152565b60008151808452610e3c816020860160208601610e00565b601f01601f19169290920160200192915050565b6001600160a01b0389811682528881166020830152878116604083015286166060820152608081018590526000610100610e8d60a0840187610dde565b84151560c08401528060e0840152610ea781840185610e24565b9b9a5050505050505050505050565b600060208284031215610ec857600080fd5b610ed182610ce8565b9392505050565b60008060008060008060a08789031215610ef157600080fd5b610efa87610ce8565b9550610f0860208801610ce8565b9450610f1660408801610ce8565b935060608701359250608087013567ffffffffffffffff80821115610f3a57600080fd5b818901915089601f830112610f4e57600080fd5b813581811115610f5d57600080fd5b8a6020828501011115610f6f57600080fd5b6020830194508093505050509295509295509295565b60008060408385031215610f9857600080fd5b823591506020830135610d3e81610d04565b600181811c90821680610fbe57607f821691505b602082108103610fde57634e487b7160e01b600052602260045260246000fd5b50919050565b634e487b7160e01b600052601160045260246000fd5b60006001820161100c5761100c610fe4565b5060010190565b634e487b7160e01b600052604160045260246000fd5b601f821115610a9157600081815260208120601f850160051c810160208610156110505750805b601f850160051c820191505b8181101561106f5782815560010161105c565b505050505050565b815167ffffffffffffffff81111561109157611091611013565b6110a58161109f8454610faa565b84611029565b602080601f8311600181146110da57600084156110c25750858301515b600019600386901b1c1916600185901b17855561106f565b600085815260208120601f198616915b82811015611109578886015182559484019460019091019084016110ea565b50858210156111275787850151600019600388901b60f8161c191681555b5050505050600190811b01905550565b8181038181111561114a5761114a610fe4565b92915050565b808202811582820484141761114a5761114a610fe4565b6000815461117481610faa565b80855260206001838116801561119157600181146111ab576111d9565b60ff1985168884015283151560051b8801830195506111d9565b866000528260002060005b858110156111d15781548a82018601529083019084016111b6565b890184019650505b505050505092915050565b828152604060208201526112136040820161120684546001600160a01b031690565b6001600160a01b03169052565b600061122960018401546001600160a01b031690565b6001600160a01b03908116606084015260028401548116608084015260038401541660a0830152600483015460c0830152600583015461126f60e0840160ff8316610dde565b61010061128581850160ff8460081c1615159052565b61012084015250610b77610140830160068501611167565b6000602082840312156112af57600080fd5b8151610ed181610d04565b600082516112cc818460208701610e00565b9190910192915050565b602081526000610ed16020830184610e2456fea264697066735822122008f410d85ce43207b9852bc54b6dba677b61603c21f867d29062171ef5e7720f64736f6c63430008110033"
            .parse()
            .expect("invalid bytecode")
    });
    pub struct AllKnowingOracle<M>(::ethers::contract::Contract<M>);
    impl<M> Clone for AllKnowingOracle<M> {
        fn clone(&self) -> Self {
            AllKnowingOracle(self.0.clone())
        }
    }
    impl<M> std::ops::Deref for AllKnowingOracle<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> std::fmt::Debug for AllKnowingOracle<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(AllKnowingOracle)).field(&self.address()).finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> AllKnowingOracle<M> {
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
                    ALLKNOWINGORACLE_ABI.clone(),
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
                ALLKNOWINGORACLE_ABI.clone(),
                ALLKNOWINGORACLE_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `acceptOwnership` (0x79ba5097) function
        pub fn accept_ownership(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([121, 186, 80, 151], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `ask` (0xf7d3b58b) function
        pub fn ask(
            &self,
            proposer: ::ethers::core::types::Address,
            disputer: ::ethers::core::types::Address,
            currency: ::ethers::core::types::Address,
            bond: ::ethers::core::types::U256,
            data: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash(
                    [247, 211, 181, 139],
                    (proposer, disputer, currency, bond, data),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getRequestId` (0x55352f82) function
        pub fn get_request_id(
            &self,
            sender: ::ethers::core::types::Address,
            proposer: ::ethers::core::types::Address,
            disputer: ::ethers::core::types::Address,
            currency: ::ethers::core::types::Address,
            bond: ::ethers::core::types::U256,
            request_index: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash(
                    [85, 53, 47, 130],
                    (sender, proposer, disputer, currency, bond, request_index),
                )
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
        ///Calls the contract's `registry` (0x7b103999) function
        pub fn registry(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([123, 16, 57, 153], ())
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
        ///Calls the contract's `requestCount` (0x5badbe4c) function
        pub fn request_count(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([91, 173, 190, 76], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `requests` (0x9d866985) function
        pub fn requests(
            &self,
            p0: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (
                ::ethers::core::types::Address,
                ::ethers::core::types::Address,
                ::ethers::core::types::Address,
                ::ethers::core::types::Address,
                ::ethers::core::types::U256,
                u8,
                bool,
                ::ethers::core::types::Bytes,
            ),
        > {
            self.0
                .method_hash([157, 134, 105, 133], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `settle` (0xfc361c38) function
        pub fn settle(
            &self,
            id: [u8; 32],
            answer: bool,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([252, 54, 28, 56], (id, answer))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `settlers` (0xc70a900f) function
        pub fn settlers(
            &self,
            p0: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([199, 10, 144, 15], p0)
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
        ///Calls the contract's `whitelistSettler` (0x329e53be) function
        pub fn whitelist_settler(
            &self,
            settler: ::ethers::core::types::Address,
            enabled: bool,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([50, 158, 83, 190], (settler, enabled))
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `NewRequest` event
        pub fn new_request_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<M, NewRequestFilter> {
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
        ///Gets the contract's `RequestSettled` event
        pub fn request_settled_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<M, RequestSettledFilter> {
            self.0.event()
        }
        ///Gets the contract's `RequesterWhitelisted` event
        pub fn requester_whitelisted_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<M, RequesterWhitelistedFilter> {
            self.0.event()
        }
        ///Gets the contract's `SettlerWhitelisted` event
        pub fn settler_whitelisted_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<M, SettlerWhitelistedFilter> {
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
        ) -> ::ethers::contract::builders::Event<M, AllKnowingOracleEvents> {
            self.0.event_with_filter(Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for AllKnowingOracle<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Custom Error type `AllKnowingOracle__NonSettler` with signature `AllKnowingOracle__NonSettler()` and selector `0xa74ee9c8`
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
        name = "AllKnowingOracle__NonSettler",
        abi = "AllKnowingOracle__NonSettler()"
    )]
    pub struct AllKnowingOracle__NonSettler;
    ///Custom Error type `AllKnowingOracle__NotSettleable` with signature `AllKnowingOracle__NotSettleable(bytes32)` and selector `0x35aa5736`
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
        name = "AllKnowingOracle__NotSettleable",
        abi = "AllKnowingOracle__NotSettleable(bytes32)"
    )]
    pub struct AllKnowingOracle__NotSettleable {
        pub id: [u8; 32],
    }
    ///Custom Error type `AllKnowingOracle__SettlerIsDisputerOrProposer` with signature `AllKnowingOracle__SettlerIsDisputerOrProposer(bytes32)` and selector `0x71ced53d`
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
        name = "AllKnowingOracle__SettlerIsDisputerOrProposer",
        abi = "AllKnowingOracle__SettlerIsDisputerOrProposer(bytes32)"
    )]
    pub struct AllKnowingOracle__SettlerIsDisputerOrProposer {
        pub id: [u8; 32],
    }
    #[derive(Debug, Clone, PartialEq, Eq, ::ethers::contract::EthAbiType)]
    pub enum AllKnowingOracleErrors {
        AllKnowingOracle__NonSettler(AllKnowingOracle__NonSettler),
        AllKnowingOracle__NotSettleable(AllKnowingOracle__NotSettleable),
        AllKnowingOracle__SettlerIsDisputerOrProposer(
            AllKnowingOracle__SettlerIsDisputerOrProposer,
        ),
    }
    impl ::ethers::core::abi::AbiDecode for AllKnowingOracleErrors {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::std::result::Result<Self, ::ethers::core::abi::AbiError> {
            if let Ok(decoded)
                = <AllKnowingOracle__NonSettler as ::ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                ) {
                return Ok(AllKnowingOracleErrors::AllKnowingOracle__NonSettler(decoded));
            }
            if let Ok(decoded)
                = <AllKnowingOracle__NotSettleable as ::ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                ) {
                return Ok(
                    AllKnowingOracleErrors::AllKnowingOracle__NotSettleable(decoded),
                );
            }
            if let Ok(decoded)
                = <AllKnowingOracle__SettlerIsDisputerOrProposer as ::ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                ) {
                return Ok(
                    AllKnowingOracleErrors::AllKnowingOracle__SettlerIsDisputerOrProposer(
                        decoded,
                    ),
                );
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for AllKnowingOracleErrors {
        fn encode(self) -> Vec<u8> {
            match self {
                AllKnowingOracleErrors::AllKnowingOracle__NonSettler(element) => {
                    element.encode()
                }
                AllKnowingOracleErrors::AllKnowingOracle__NotSettleable(element) => {
                    element.encode()
                }
                AllKnowingOracleErrors::AllKnowingOracle__SettlerIsDisputerOrProposer(
                    element,
                ) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for AllKnowingOracleErrors {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                AllKnowingOracleErrors::AllKnowingOracle__NonSettler(element) => {
                    element.fmt(f)
                }
                AllKnowingOracleErrors::AllKnowingOracle__NotSettleable(element) => {
                    element.fmt(f)
                }
                AllKnowingOracleErrors::AllKnowingOracle__SettlerIsDisputerOrProposer(
                    element,
                ) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<AllKnowingOracle__NonSettler> for AllKnowingOracleErrors {
        fn from(var: AllKnowingOracle__NonSettler) -> Self {
            AllKnowingOracleErrors::AllKnowingOracle__NonSettler(var)
        }
    }
    impl ::std::convert::From<AllKnowingOracle__NotSettleable>
    for AllKnowingOracleErrors {
        fn from(var: AllKnowingOracle__NotSettleable) -> Self {
            AllKnowingOracleErrors::AllKnowingOracle__NotSettleable(var)
        }
    }
    impl ::std::convert::From<AllKnowingOracle__SettlerIsDisputerOrProposer>
    for AllKnowingOracleErrors {
        fn from(var: AllKnowingOracle__SettlerIsDisputerOrProposer) -> Self {
            AllKnowingOracleErrors::AllKnowingOracle__SettlerIsDisputerOrProposer(var)
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
    #[ethevent(
        name = "NewRequest",
        abi = "NewRequest(bytes32,address,address,address,uint256,uint256)"
    )]
    pub struct NewRequestFilter {
        #[ethevent(indexed)]
        pub id: [u8; 32],
        #[ethevent(indexed)]
        pub proposer: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub disputer: ::ethers::core::types::Address,
        pub currency: ::ethers::core::types::Address,
        pub bond: ::ethers::core::types::U256,
        pub request_index: ::ethers::core::types::U256,
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
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
    )]
    #[derive(Default)]
    #[ethevent(
        name = "RequesterWhitelisted",
        abi = "RequesterWhitelisted(address,bool)"
    )]
    pub struct RequesterWhitelistedFilter {
        #[ethevent(indexed)]
        pub requester: ::ethers::core::types::Address,
        pub enabled: bool,
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
    #[ethevent(name = "SettlerWhitelisted", abi = "SettlerWhitelisted(address,bool)")]
    pub struct SettlerWhitelistedFilter {
        #[ethevent(indexed)]
        pub settler: ::ethers::core::types::Address,
        pub enabled: bool,
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
        pub enabled: bool,
    }
    #[derive(Debug, Clone, PartialEq, Eq, ::ethers::contract::EthAbiType)]
    pub enum AllKnowingOracleEvents {
        NewRequestFilter(NewRequestFilter),
        OwnershipTransferStartedFilter(OwnershipTransferStartedFilter),
        OwnershipTransferredFilter(OwnershipTransferredFilter),
        RequestSettledFilter(RequestSettledFilter),
        RequesterWhitelistedFilter(RequesterWhitelistedFilter),
        SettlerWhitelistedFilter(SettlerWhitelistedFilter),
        TokenWhitelistedFilter(TokenWhitelistedFilter),
    }
    impl ::ethers::contract::EthLogDecode for AllKnowingOracleEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::std::result::Result<Self, ::ethers::core::abi::Error>
        where
            Self: Sized,
        {
            if let Ok(decoded) = NewRequestFilter::decode_log(log) {
                return Ok(AllKnowingOracleEvents::NewRequestFilter(decoded));
            }
            if let Ok(decoded) = OwnershipTransferStartedFilter::decode_log(log) {
                return Ok(
                    AllKnowingOracleEvents::OwnershipTransferStartedFilter(decoded),
                );
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
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::std::fmt::Display for AllKnowingOracleEvents {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                AllKnowingOracleEvents::NewRequestFilter(element) => element.fmt(f),
                AllKnowingOracleEvents::OwnershipTransferStartedFilter(element) => {
                    element.fmt(f)
                }
                AllKnowingOracleEvents::OwnershipTransferredFilter(element) => {
                    element.fmt(f)
                }
                AllKnowingOracleEvents::RequestSettledFilter(element) => element.fmt(f),
                AllKnowingOracleEvents::RequesterWhitelistedFilter(element) => {
                    element.fmt(f)
                }
                AllKnowingOracleEvents::SettlerWhitelistedFilter(element) => {
                    element.fmt(f)
                }
                AllKnowingOracleEvents::TokenWhitelistedFilter(element) => element.fmt(f),
            }
        }
    }
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
    ///Container type for all input parameters for the `ask` function with signature `ask(address,address,address,uint256,bytes)` and selector `0xf7d3b58b`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
    )]
    #[derive(Default)]
    #[ethcall(name = "ask", abi = "ask(address,address,address,uint256,bytes)")]
    pub struct AskCall {
        pub proposer: ::ethers::core::types::Address,
        pub disputer: ::ethers::core::types::Address,
        pub currency: ::ethers::core::types::Address,
        pub bond: ::ethers::core::types::U256,
        pub data: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `getRequestId` function with signature `getRequestId(address,address,address,address,uint256,uint256)` and selector `0x55352f82`
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
        name = "getRequestId",
        abi = "getRequestId(address,address,address,address,uint256,uint256)"
    )]
    pub struct GetRequestIdCall {
        pub sender: ::ethers::core::types::Address,
        pub proposer: ::ethers::core::types::Address,
        pub disputer: ::ethers::core::types::Address,
        pub currency: ::ethers::core::types::Address,
        pub bond: ::ethers::core::types::U256,
        pub request_index: ::ethers::core::types::U256,
    }
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
    ///Container type for all input parameters for the `registry` function with signature `registry()` and selector `0x7b103999`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
    )]
    #[derive(Default)]
    #[ethcall(name = "registry", abi = "registry()")]
    pub struct RegistryCall;
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
    ///Container type for all input parameters for the `requestCount` function with signature `requestCount()` and selector `0x5badbe4c`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
    )]
    #[derive(Default)]
    #[ethcall(name = "requestCount", abi = "requestCount()")]
    pub struct RequestCountCall;
    ///Container type for all input parameters for the `requests` function with signature `requests(bytes32)` and selector `0x9d866985`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
    )]
    #[derive(Default)]
    #[ethcall(name = "requests", abi = "requests(bytes32)")]
    pub struct RequestsCall(pub [u8; 32]);
    ///Container type for all input parameters for the `settle` function with signature `settle(bytes32,bool)` and selector `0xfc361c38`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
    )]
    #[derive(Default)]
    #[ethcall(name = "settle", abi = "settle(bytes32,bool)")]
    pub struct SettleCall {
        pub id: [u8; 32],
        pub answer: bool,
    }
    ///Container type for all input parameters for the `settlers` function with signature `settlers(address)` and selector `0xc70a900f`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
    )]
    #[derive(Default)]
    #[ethcall(name = "settlers", abi = "settlers(address)")]
    pub struct SettlersCall(pub ::ethers::core::types::Address);
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
    ///Container type for all input parameters for the `whitelistSettler` function with signature `whitelistSettler(address,bool)` and selector `0x329e53be`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
    )]
    #[derive(Default)]
    #[ethcall(name = "whitelistSettler", abi = "whitelistSettler(address,bool)")]
    pub struct WhitelistSettlerCall {
        pub settler: ::ethers::core::types::Address,
        pub enabled: bool,
    }
    #[derive(Debug, Clone, PartialEq, Eq, ::ethers::contract::EthAbiType)]
    pub enum AllKnowingOracleCalls {
        AcceptOwnership(AcceptOwnershipCall),
        Ask(AskCall),
        GetRequestId(GetRequestIdCall),
        Owner(OwnerCall),
        PendingOwner(PendingOwnerCall),
        Registry(RegistryCall),
        RenounceOwnership(RenounceOwnershipCall),
        RequestCount(RequestCountCall),
        Requests(RequestsCall),
        Settle(SettleCall),
        Settlers(SettlersCall),
        TransferOwnership(TransferOwnershipCall),
        WhitelistSettler(WhitelistSettlerCall),
    }
    impl ::ethers::core::abi::AbiDecode for AllKnowingOracleCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::std::result::Result<Self, ::ethers::core::abi::AbiError> {
            if let Ok(decoded)
                = <AcceptOwnershipCall as ::ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                ) {
                return Ok(AllKnowingOracleCalls::AcceptOwnership(decoded));
            }
            if let Ok(decoded)
                = <AskCall as ::ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(AllKnowingOracleCalls::Ask(decoded));
            }
            if let Ok(decoded)
                = <GetRequestIdCall as ::ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                ) {
                return Ok(AllKnowingOracleCalls::GetRequestId(decoded));
            }
            if let Ok(decoded)
                = <OwnerCall as ::ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(AllKnowingOracleCalls::Owner(decoded));
            }
            if let Ok(decoded)
                = <PendingOwnerCall as ::ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                ) {
                return Ok(AllKnowingOracleCalls::PendingOwner(decoded));
            }
            if let Ok(decoded)
                = <RegistryCall as ::ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                ) {
                return Ok(AllKnowingOracleCalls::Registry(decoded));
            }
            if let Ok(decoded)
                = <RenounceOwnershipCall as ::ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                ) {
                return Ok(AllKnowingOracleCalls::RenounceOwnership(decoded));
            }
            if let Ok(decoded)
                = <RequestCountCall as ::ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                ) {
                return Ok(AllKnowingOracleCalls::RequestCount(decoded));
            }
            if let Ok(decoded)
                = <RequestsCall as ::ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                ) {
                return Ok(AllKnowingOracleCalls::Requests(decoded));
            }
            if let Ok(decoded)
                = <SettleCall as ::ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(AllKnowingOracleCalls::Settle(decoded));
            }
            if let Ok(decoded)
                = <SettlersCall as ::ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                ) {
                return Ok(AllKnowingOracleCalls::Settlers(decoded));
            }
            if let Ok(decoded)
                = <TransferOwnershipCall as ::ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                ) {
                return Ok(AllKnowingOracleCalls::TransferOwnership(decoded));
            }
            if let Ok(decoded)
                = <WhitelistSettlerCall as ::ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                ) {
                return Ok(AllKnowingOracleCalls::WhitelistSettler(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for AllKnowingOracleCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                AllKnowingOracleCalls::AcceptOwnership(element) => element.encode(),
                AllKnowingOracleCalls::Ask(element) => element.encode(),
                AllKnowingOracleCalls::GetRequestId(element) => element.encode(),
                AllKnowingOracleCalls::Owner(element) => element.encode(),
                AllKnowingOracleCalls::PendingOwner(element) => element.encode(),
                AllKnowingOracleCalls::Registry(element) => element.encode(),
                AllKnowingOracleCalls::RenounceOwnership(element) => element.encode(),
                AllKnowingOracleCalls::RequestCount(element) => element.encode(),
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
                AllKnowingOracleCalls::RequestCount(element) => element.fmt(f),
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
    impl ::std::convert::From<RequestCountCall> for AllKnowingOracleCalls {
        fn from(var: RequestCountCall) -> Self {
            AllKnowingOracleCalls::RequestCount(var)
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
    ///Container type for all return fields from the `ask` function with signature `ask(address,address,address,uint256,bytes)` and selector `0xf7d3b58b`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
    )]
    #[derive(Default)]
    pub struct AskReturn {
        pub id: [u8; 32],
    }
    ///Container type for all return fields from the `getRequestId` function with signature `getRequestId(address,address,address,address,uint256,uint256)` and selector `0x55352f82`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
    )]
    #[derive(Default)]
    pub struct GetRequestIdReturn(pub [u8; 32]);
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
    ///Container type for all return fields from the `registry` function with signature `registry()` and selector `0x7b103999`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
    )]
    #[derive(Default)]
    pub struct RegistryReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `requestCount` function with signature `requestCount()` and selector `0x5badbe4c`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
    )]
    #[derive(Default)]
    pub struct RequestCountReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `requests` function with signature `requests(bytes32)` and selector `0x9d866985`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
    )]
    #[derive(Default)]
    pub struct RequestsReturn {
        pub requester: ::ethers::core::types::Address,
        pub proposer: ::ethers::core::types::Address,
        pub disputer: ::ethers::core::types::Address,
        pub currency: ::ethers::core::types::Address,
        pub bond: ::ethers::core::types::U256,
        pub state: u8,
        pub answer: bool,
        pub data: ::ethers::core::types::Bytes,
    }
    ///Container type for all return fields from the `settlers` function with signature `settlers(address)` and selector `0xc70a900f`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
    )]
    #[derive(Default)]
    pub struct SettlersReturn(pub bool);
}
