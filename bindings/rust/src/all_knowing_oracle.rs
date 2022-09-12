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
    pub static ALLKNOWINGORACLE_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            ethers :: core :: utils :: __serde_json :: from_str ("[{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"constructor\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"id\",\"type\":\"bytes32\",\"components\":[]}],\"type\":\"error\",\"name\":\"AllKnowingOracle__AlreadySettled\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"AllKnowingOracle__NonRequester\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"AllKnowingOracle__NonSettler\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"id\",\"type\":\"bytes32\",\"components\":[]}],\"type\":\"error\",\"name\":\"AllKnowingOracle__RequestAlreadyExists\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\",\"components\":[]}],\"type\":\"error\",\"name\":\"AllKnowingOracle__TokenNotWhitelisted\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"id\",\"type\":\"bytes32\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"proposer\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"disputer\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"currency\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"bond\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"NewRequest\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"user\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"newOwner\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"OwnerUpdated\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"id\",\"type\":\"bytes32\",\"components\":[],\"indexed\":true},{\"internalType\":\"bool\",\"name\":\"answer\",\"type\":\"bool\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"RequestSettled\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"requester\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"bool\",\"name\":\"enabled\",\"type\":\"bool\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"RequesterWhitelisted\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"settler\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"bool\",\"name\":\"enabled\",\"type\":\"bool\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"SettlerWhitelisted\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"bool\",\"name\":\"enabled\",\"type\":\"bool\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"TokenWhitelisted\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"proposer\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"disputer\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"currency\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"bond\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"data\",\"type\":\"bytes\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"ask\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"id\",\"type\":\"bytes32\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"sender\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"proposer\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"disputer\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"currency\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"bond\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"getRequestId\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"owner\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"requesters\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"requests\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"requester\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"proposer\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"disputer\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"contract ERC20\",\"name\":\"currency\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"bond\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"enum RequestState\",\"name\":\"state\",\"type\":\"uint8\",\"components\":[]},{\"internalType\":\"bool\",\"name\":\"answer\",\"type\":\"bool\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"data\",\"type\":\"bytes\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"newOwner\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setOwner\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"id\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"bool\",\"name\":\"answer\",\"type\":\"bool\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"settle\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"settlers\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"requester\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bool\",\"name\":\"enabled\",\"type\":\"bool\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"whitelistRequester\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"settler\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bool\",\"name\":\"enabled\",\"type\":\"bool\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"whitelistSettler\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bool\",\"name\":\"enabled\",\"type\":\"bool\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"whitelistToken\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"whitelistedTokens\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]}]") . expect ("invalid abi")
        });
    #[doc = r" Bytecode of the #name contract"]
    pub static ALLKNOWINGORACLE_BYTECODE: ethers::contract::Lazy<ethers::core::types::Bytes> =
        ethers::contract::Lazy::new(|| {
            "0x608060405234801561001057600080fd5b50600080546001600160a01b031916339081178255604051909182917f8292fce18fa69edf4db7b94ea2e58241df0ae57f97e0a6c9b29067028bf92d76908290a350336000908152600360205260409020805460ff191660011790556110e58061007b6000396000f3fe608060405234801561001057600080fd5b50600436106100b45760003560e01c8063c70a900f11610071578063c70a900f1461015e578063d6f8307f14610191578063daf9c210146101b4578063e6717ce7146101d7578063f7d3b58b146101f8578063fc361c381461020b57600080fd5b80630ffb1d8b146100b957806313af4035146100ce578063329e53be146100e1578063889590f1146100f45780638da5cb5b146101075780639d86698514610137575b600080fd5b6100cc6100c7366004610b44565b61021e565b005b6100cc6100dc366004610b77565b6102b1565b6100cc6100ef366004610b44565b610326565b6100cc610102366004610b44565b6103a8565b60005461011a906001600160a01b031681565b6040516001600160a01b0390911681526020015b60405180910390f35b61014a610145366004610b99565b61042a565b60405161012e989796959493929190610bea565b61018161016c366004610b77565b60036020526000908152604090205460ff1681565b604051901515815260200161012e565b61018161019f366004610b77565b60046020526000908152604090205460ff1681565b6101816101c2366004610b77565b60026020526000908152604090205460ff1681565b6101ea6101e5366004610c93565b610511565b60405190815260200161012e565b6101ea610206366004610cef565b61052a565b6100cc610219366004610d9c565b610804565b6000546001600160a01b031633146102515760405162461bcd60e51b815260040161024890610dbf565b60405180910390fd5b6001600160a01b038216600081815260026020908152604091829020805460ff191685151590811790915591519182527fef81a9943b96c8df4ef243401c9bf5159146166211356898b52d382086168d9291015b60405180910390a25050565b6000546001600160a01b031633146102db5760405162461bcd60e51b815260040161024890610dbf565b600080546001600160a01b0319166001600160a01b0383169081178255604051909133917f8292fce18fa69edf4db7b94ea2e58241df0ae57f97e0a6c9b29067028bf92d769190a350565b6000546001600160a01b031633146103505760405162461bcd60e51b815260040161024890610dbf565b6001600160a01b038216600081815260036020908152604091829020805460ff191685151590811790915591519182527f8cc72bec7e2cf5979aefd933f40a28eb590098d522a5c458e497e71c5e6fa90f91016102a5565b6000546001600160a01b031633146103d25760405162461bcd60e51b815260040161024890610dbf565b6001600160a01b038216600081815260046020908152604091829020805460ff191685151590811790915591519182527f2bf32d6c1bf8a8d32ace419214cf1c0fa979b6ed1135840088771f67f25f104991016102a5565b600160208190526000918252604090912080549181015460028201546003830154600484015460058501546006860180546001600160a01b0398891698968716979587169690941694929360ff808416946101009094041692919061048e90610de5565b80601f01602080910402602001604051908101604052809291908181526020018280546104ba90610de5565b80156105075780601f106104dc57610100808354040283529160200191610507565b820191906000526020600020905b8154815290600101906020018083116104ea57829003601f168201915b5050505050905088565b600061052086868686866109ae565b9695505050505050565b3360009081526004602052604081205460ff1661055a576040516325c0cb7960e11b815260040160405180910390fd5b6001600160a01b03851660009081526002602052604090205460ff1661059e5760405163028511d160e51b81526001600160a01b0386166004820152602401610248565b6105ab33888888886109ae565b90506000808281526001602052604090206005015460ff1660028111156105d4576105d4610bb2565b146105f55760405163d5e880e960e01b815260048101829052602401610248565b6000604051806101000160405280336001600160a01b03168152602001896001600160a01b03168152602001886001600160a01b03168152602001876001600160a01b031681526020018681526020016001600281111561065857610658610bb2565b815260200160001515815260200185858080601f016020809104026020016040519081016040528093929190818152602001838380828437600092018290525093909452505084815260016020818152604092839020855181546001600160a01b03199081166001600160a01b039283161783559287015182850180548516918316919091179055938601516002808301805485169287169290921790915560608701516003830180549094169516949094179091556080850151600482015560a085015160058201805496975087969295509093909260ff199092169190849081111561074857610748610bb2565b021790555060c08201516005820180549115156101000261ff001990921691909117905560e082015160068201906107809082610e84565b5050604080516001600160a01b03898116825260208201899052808b1693508b169185917f2609e116ca576195fecb47831f147fd6fee721377e4d88fff48807f3e9442a42910160405180910390a46107e46001600160a01b038716333088610a16565b6107f96001600160a01b038716883088610a16565b509695505050505050565b3360009081526003602052604090205460ff16610834576040516314e9dd3960e31b815260040160405180910390fd5b60008281526001602052604090206002600582015460ff16600281111561085d5761085d610bb2565b0361087e5760405163b677167760e01b815260048101849052602401610248565b6000816004015460026108919190610f44565b905082156108bf57600182015460038301546108ba916001600160a01b03918216911683610aa0565b6108e0565b600282015460038301546108e0916001600160a01b03918216911683610aa0565b60058201805461ffff19166101008515159081029190911760021790915560405190815284907fdbed7580b9c2829ee6b384e3833f10b16f9885601c98a01c40fd705b543e9c669060200160405180910390a281546001600160a01b03163b156109a857815460405163734d162760e01b81526001600160a01b039091169063734d1627906109759087908690600401610fee565b600060405180830381600087803b15801561098f57600080fd5b505af11580156109a3573d6000803e3d6000fd5b505050505b50505050565b6040516bffffffffffffffffffffffff19606087811b8216602084015286811b8216603484015285811b8216604884015284901b16605c8201526070810182905260009060900160405160208183030381529060405280519060200120905095945050505050565b60006040516323b872dd60e01b81528460048201528360248201528260448201526020600060648360008a5af13d15601f3d1160016000511416171691505080610a995760405162461bcd60e51b81526020600482015260146024820152731514905394d1915497d19493d357d1905253115160621b6044820152606401610248565b5050505050565b600060405163a9059cbb60e01b8152836004820152826024820152602060006044836000895af13d15601f3d11600160005114161716915050806109a85760405162461bcd60e51b815260206004820152600f60248201526e1514905394d1915497d19052531151608a1b6044820152606401610248565b80356001600160a01b0381168114610b2f57600080fd5b919050565b80358015158114610b2f57600080fd5b60008060408385031215610b5757600080fd5b610b6083610b18565b9150610b6e60208401610b34565b90509250929050565b600060208284031215610b8957600080fd5b610b9282610b18565b9392505050565b600060208284031215610bab57600080fd5b5035919050565b634e487b7160e01b600052602160045260246000fd5b60038110610be657634e487b7160e01b600052602160045260246000fd5b9052565b600061010060018060a01b03808c1684526020818c1681860152818b166040860152818a166060860152886080860152610c2760a0860189610bc8565b86151560c08601528260e0860152855191508183860152600092505b81831015610c6257858301810151858401610120015291820191610c43565b5080821115610c7657600061012082860101525b601f01601f191692909201610120019a9950505050505050505050565b600080600080600060a08688031215610cab57600080fd5b610cb486610b18565b9450610cc260208701610b18565b9350610cd060408701610b18565b9250610cde60608701610b18565b949793965091946080013592915050565b60008060008060008060a08789031215610d0857600080fd5b610d1187610b18565b9550610d1f60208801610b18565b9450610d2d60408801610b18565b935060608701359250608087013567ffffffffffffffff80821115610d5157600080fd5b818901915089601f830112610d6557600080fd5b813581811115610d7457600080fd5b8a6020828501011115610d8657600080fd5b6020830194508093505050509295509295509295565b60008060408385031215610daf57600080fd5b82359150610b6e60208401610b34565b6020808252600c908201526b15539055551213d49256915160a21b604082015260600190565b600181811c90821680610df957607f821691505b602082108103610e1957634e487b7160e01b600052602260045260246000fd5b50919050565b634e487b7160e01b600052604160045260246000fd5b601f821115610e7f57600081815260208120601f850160051c81016020861015610e5c5750805b601f850160051c820191505b81811015610e7b57828155600101610e68565b5050505b505050565b815167ffffffffffffffff811115610e9e57610e9e610e1f565b610eb281610eac8454610de5565b84610e35565b602080601f831160018114610ee75760008415610ecf5750858301515b600019600386901b1c1916600185901b178555610e7b565b600085815260208120601f198616915b82811015610f1657888601518255948401946001909101908401610ef7565b5085821015610f345787850151600019600388901b60f8161c191681555b5050505050600190811b01905550565b6000816000190483118215151615610f6c57634e487b7160e01b600052601160045260246000fd5b500290565b60008154610f7e81610de5565b808552602060018381168015610f9b5760018114610fb557610fe3565b60ff1985168884015283151560051b880183019550610fe3565b866000528260002060005b85811015610fdb5781548a8201860152908301908401610fc0565b890184019650505b505050505092915050565b8281526040602082015261101d6040820161101084546001600160a01b031690565b6001600160a01b03169052565b600061103360018401546001600160a01b031690565b6001600160a01b03908116606084015260028401548116608084015260038401541660a0830152600483015460c0830152600583015461107960e0840160ff8316610bc8565b61010061108f81850160ff8460081c1615159052565b610120840152506110a7610140830160068501610f71565b94935050505056fea2646970667358221220272874802086c2a8d6a6bd1164f1917711727c214439b069d5afcf37c33be5d264736f6c634300080f0033" . parse () . expect ("invalid bytecode")
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
    impl<M: ethers::providers::Middleware> std::fmt::Debug for AllKnowingOracle<M> {
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
        #[doc = "Calls the contract's `requesters` (0xd6f8307f) function"]
        pub fn requesters(
            &self,
            p0: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([214, 248, 48, 127], p0)
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
        #[doc = "Calls the contract's `setOwner` (0x13af4035) function"]
        pub fn set_owner(
            &self,
            new_owner: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([19, 175, 64, 53], new_owner)
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
        #[doc = "Calls the contract's `whitelistRequester` (0x889590f1) function"]
        pub fn whitelist_requester(
            &self,
            requester: ethers::core::types::Address,
            enabled: bool,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([136, 149, 144, 241], (requester, enabled))
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
        #[doc = "Calls the contract's `whitelistedTokens` (0xdaf9c210) function"]
        pub fn whitelisted_tokens(
            &self,
            p0: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([218, 249, 194, 16], p0)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Gets the contract's `NewRequest` event"]
        pub fn new_request_filter(&self) -> ethers::contract::builders::Event<M, NewRequestFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `OwnerUpdated` event"]
        pub fn owner_updated_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, OwnerUpdatedFilter> {
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
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
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
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
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
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
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
    pub enum AllKnowingOracleEvents {
        NewRequestFilter(NewRequestFilter),
        OwnerUpdatedFilter(OwnerUpdatedFilter),
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
            if let Ok(decoded) = OwnerUpdatedFilter::decode_log(log) {
                return Ok(AllKnowingOracleEvents::OwnerUpdatedFilter(decoded));
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
                AllKnowingOracleEvents::OwnerUpdatedFilter(element) => element.fmt(f),
                AllKnowingOracleEvents::RequestSettledFilter(element) => element.fmt(f),
                AllKnowingOracleEvents::RequesterWhitelistedFilter(element) => element.fmt(f),
                AllKnowingOracleEvents::SettlerWhitelistedFilter(element) => element.fmt(f),
                AllKnowingOracleEvents::TokenWhitelistedFilter(element) => element.fmt(f),
            }
        }
    }
    #[doc = "Container type for all input parameters for the `ask` function with signature `ask(address,address,address,uint256,bytes)` and selector `[247, 211, 181, 139]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
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
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
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
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "owner", abi = "owner()")]
    pub struct OwnerCall;
    #[doc = "Container type for all input parameters for the `requesters` function with signature `requesters(address)` and selector `[214, 248, 48, 127]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "requesters", abi = "requesters(address)")]
    pub struct RequestersCall(pub ethers::core::types::Address);
    #[doc = "Container type for all input parameters for the `requests` function with signature `requests(bytes32)` and selector `[157, 134, 105, 133]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "requests", abi = "requests(bytes32)")]
    pub struct RequestsCall(pub [u8; 32]);
    #[doc = "Container type for all input parameters for the `setOwner` function with signature `setOwner(address)` and selector `[19, 175, 64, 53]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "setOwner", abi = "setOwner(address)")]
    pub struct SetOwnerCall {
        pub new_owner: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `settle` function with signature `settle(bytes32,bool)` and selector `[252, 54, 28, 56]`"]
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
    #[doc = "Container type for all input parameters for the `settlers` function with signature `settlers(address)` and selector `[199, 10, 144, 15]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "settlers", abi = "settlers(address)")]
    pub struct SettlersCall(pub ethers::core::types::Address);
    #[doc = "Container type for all input parameters for the `whitelistRequester` function with signature `whitelistRequester(address,bool)` and selector `[136, 149, 144, 241]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "whitelistRequester", abi = "whitelistRequester(address,bool)")]
    pub struct WhitelistRequesterCall {
        pub requester: ethers::core::types::Address,
        pub enabled: bool,
    }
    #[doc = "Container type for all input parameters for the `whitelistSettler` function with signature `whitelistSettler(address,bool)` and selector `[50, 158, 83, 190]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "whitelistSettler", abi = "whitelistSettler(address,bool)")]
    pub struct WhitelistSettlerCall {
        pub settler: ethers::core::types::Address,
        pub enabled: bool,
    }
    #[doc = "Container type for all input parameters for the `whitelistToken` function with signature `whitelistToken(address,bool)` and selector `[15, 251, 29, 139]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "whitelistToken", abi = "whitelistToken(address,bool)")]
    pub struct WhitelistTokenCall {
        pub token: ethers::core::types::Address,
        pub enabled: bool,
    }
    #[doc = "Container type for all input parameters for the `whitelistedTokens` function with signature `whitelistedTokens(address)` and selector `[218, 249, 194, 16]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "whitelistedTokens", abi = "whitelistedTokens(address)")]
    pub struct WhitelistedTokensCall(pub ethers::core::types::Address);
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum AllKnowingOracleCalls {
        Ask(AskCall),
        GetRequestId(GetRequestIdCall),
        Owner(OwnerCall),
        Requesters(RequestersCall),
        Requests(RequestsCall),
        SetOwner(SetOwnerCall),
        Settle(SettleCall),
        Settlers(SettlersCall),
        WhitelistRequester(WhitelistRequesterCall),
        WhitelistSettler(WhitelistSettlerCall),
        WhitelistToken(WhitelistTokenCall),
        WhitelistedTokens(WhitelistedTokensCall),
    }
    impl ethers::core::abi::AbiDecode for AllKnowingOracleCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::std::result::Result<Self, ethers::core::abi::AbiError> {
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
                <RequestersCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(AllKnowingOracleCalls::Requesters(decoded));
            }
            if let Ok(decoded) =
                <RequestsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(AllKnowingOracleCalls::Requests(decoded));
            }
            if let Ok(decoded) =
                <SetOwnerCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(AllKnowingOracleCalls::SetOwner(decoded));
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
                <WhitelistRequesterCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(AllKnowingOracleCalls::WhitelistRequester(decoded));
            }
            if let Ok(decoded) =
                <WhitelistSettlerCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(AllKnowingOracleCalls::WhitelistSettler(decoded));
            }
            if let Ok(decoded) =
                <WhitelistTokenCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(AllKnowingOracleCalls::WhitelistToken(decoded));
            }
            if let Ok(decoded) =
                <WhitelistedTokensCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(AllKnowingOracleCalls::WhitelistedTokens(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for AllKnowingOracleCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                AllKnowingOracleCalls::Ask(element) => element.encode(),
                AllKnowingOracleCalls::GetRequestId(element) => element.encode(),
                AllKnowingOracleCalls::Owner(element) => element.encode(),
                AllKnowingOracleCalls::Requesters(element) => element.encode(),
                AllKnowingOracleCalls::Requests(element) => element.encode(),
                AllKnowingOracleCalls::SetOwner(element) => element.encode(),
                AllKnowingOracleCalls::Settle(element) => element.encode(),
                AllKnowingOracleCalls::Settlers(element) => element.encode(),
                AllKnowingOracleCalls::WhitelistRequester(element) => element.encode(),
                AllKnowingOracleCalls::WhitelistSettler(element) => element.encode(),
                AllKnowingOracleCalls::WhitelistToken(element) => element.encode(),
                AllKnowingOracleCalls::WhitelistedTokens(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for AllKnowingOracleCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                AllKnowingOracleCalls::Ask(element) => element.fmt(f),
                AllKnowingOracleCalls::GetRequestId(element) => element.fmt(f),
                AllKnowingOracleCalls::Owner(element) => element.fmt(f),
                AllKnowingOracleCalls::Requesters(element) => element.fmt(f),
                AllKnowingOracleCalls::Requests(element) => element.fmt(f),
                AllKnowingOracleCalls::SetOwner(element) => element.fmt(f),
                AllKnowingOracleCalls::Settle(element) => element.fmt(f),
                AllKnowingOracleCalls::Settlers(element) => element.fmt(f),
                AllKnowingOracleCalls::WhitelistRequester(element) => element.fmt(f),
                AllKnowingOracleCalls::WhitelistSettler(element) => element.fmt(f),
                AllKnowingOracleCalls::WhitelistToken(element) => element.fmt(f),
                AllKnowingOracleCalls::WhitelistedTokens(element) => element.fmt(f),
            }
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
    impl ::std::convert::From<RequestersCall> for AllKnowingOracleCalls {
        fn from(var: RequestersCall) -> Self {
            AllKnowingOracleCalls::Requesters(var)
        }
    }
    impl ::std::convert::From<RequestsCall> for AllKnowingOracleCalls {
        fn from(var: RequestsCall) -> Self {
            AllKnowingOracleCalls::Requests(var)
        }
    }
    impl ::std::convert::From<SetOwnerCall> for AllKnowingOracleCalls {
        fn from(var: SetOwnerCall) -> Self {
            AllKnowingOracleCalls::SetOwner(var)
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
    impl ::std::convert::From<WhitelistRequesterCall> for AllKnowingOracleCalls {
        fn from(var: WhitelistRequesterCall) -> Self {
            AllKnowingOracleCalls::WhitelistRequester(var)
        }
    }
    impl ::std::convert::From<WhitelistSettlerCall> for AllKnowingOracleCalls {
        fn from(var: WhitelistSettlerCall) -> Self {
            AllKnowingOracleCalls::WhitelistSettler(var)
        }
    }
    impl ::std::convert::From<WhitelistTokenCall> for AllKnowingOracleCalls {
        fn from(var: WhitelistTokenCall) -> Self {
            AllKnowingOracleCalls::WhitelistToken(var)
        }
    }
    impl ::std::convert::From<WhitelistedTokensCall> for AllKnowingOracleCalls {
        fn from(var: WhitelistedTokensCall) -> Self {
            AllKnowingOracleCalls::WhitelistedTokens(var)
        }
    }
    #[doc = "Container type for all return fields from the `ask` function with signature `ask(address,address,address,uint256,bytes)` and selector `[247, 211, 181, 139]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct AskReturn {
        pub id: [u8; 32],
    }
    #[doc = "Container type for all return fields from the `getRequestId` function with signature `getRequestId(address,address,address,address,uint256)` and selector `[230, 113, 124, 231]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct GetRequestIdReturn(pub [u8; 32]);
    #[doc = "Container type for all return fields from the `owner` function with signature `owner()` and selector `[141, 165, 203, 91]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct OwnerReturn(pub ethers::core::types::Address);
    #[doc = "Container type for all return fields from the `requesters` function with signature `requesters(address)` and selector `[214, 248, 48, 127]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct RequestersReturn(pub bool);
    #[doc = "Container type for all return fields from the `requests` function with signature `requests(bytes32)` and selector `[157, 134, 105, 133]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
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
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct SettlersReturn(pub bool);
    #[doc = "Container type for all return fields from the `whitelistedTokens` function with signature `whitelistedTokens(address)` and selector `[218, 249, 194, 16]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct WhitelistedTokensReturn(pub bool);
}
