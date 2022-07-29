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
            ethers :: core :: utils :: __serde_json :: from_str ("[{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"constructor\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"id\",\"type\":\"bytes32\",\"components\":[]}],\"type\":\"error\",\"name\":\"AllKnowingOracle__AlreadySettled\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"AllKnowingOracle__NonSettler\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\",\"components\":[]}],\"type\":\"error\",\"name\":\"AllKnowingOracle__NotWhitelisted\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"id\",\"type\":\"bytes32\",\"components\":[]}],\"type\":\"error\",\"name\":\"AllKnowingOracle__RequestAlreadyExists\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"newPct\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"BondPctChanged\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"id\",\"type\":\"bytes32\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"proposer\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"disputer\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"bondToken\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"stake\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"bond\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"NewRequest\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"user\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"newOwner\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"OwnerUpdated\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"id\",\"type\":\"bytes32\",\"components\":[],\"indexed\":true},{\"internalType\":\"bool\",\"name\":\"answer\",\"type\":\"bool\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"RequestSettled\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"settler\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"bool\",\"name\":\"enabled\",\"type\":\"bool\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"SettlerWhitelisted\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"bool\",\"name\":\"enabled\",\"type\":\"bool\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"TokenWhitelisted\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"proposer\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"disputer\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"bondToken\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"stake\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"ask\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"stake\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"bondForStake\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"disputeBondPct\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"proposer\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"disputer\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"bondToken\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"stake\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getRequestId\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"owner\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"requests\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"proposer\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"disputer\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"bondToken\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"stake\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"bond\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bool\",\"name\":\"answer\",\"type\":\"bool\",\"components\":[]},{\"internalType\":\"enum RequestState\",\"name\":\"state\",\"type\":\"uint8\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"newPct\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setBondPct\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"newOwner\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setOwner\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"id\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"bool\",\"name\":\"answer\",\"type\":\"bool\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"settle\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"settlers\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"settler\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bool\",\"name\":\"enabled\",\"type\":\"bool\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"whitelistSettler\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bool\",\"name\":\"enabled\",\"type\":\"bool\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"whitelistToken\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"whitelistedTokens\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]}]") . expect ("invalid abi")
        });
    #[doc = r" Bytecode of the #name contract"]
    pub static ALLKNOWINGORACLE_BYTECODE: ethers::contract::Lazy<ethers::core::types::Bytes> =
        ethers::contract::Lazy::new(|| {
            "0x608060405234801561001057600080fd5b50600080546001600160a01b031916339081178255604051909182917f8292fce18fa69edf4db7b94ea2e58241df0ae57f97e0a6c9b29067028bf92d76908290a350601960049081553360009081526003602052604090819020805460ff19166001179055905490517fa2e09ae7caa0e84bcfe806d931934b92bf11ce67ffcaa3033af9a1e031996951916100a89190815260200190565b60405180910390a1610d28806100bf6000396000f3fe608060405234801561001057600080fd5b50600436106100cf5760003560e01c80638945bd281161008c578063af5899fc11610066578063af5899fc14610202578063c70a900f14610215578063daf9c21014610248578063fc361c381461026b57600080fd5b80638945bd28146101515780638da5cb5b146101645780639d8669851461018f57600080fd5b806307de99f6146100d45780630ffb1d8b146100fa57806313af40351461010f578063329e53be14610122578063391fe4e21461013557806384bfabcf1461013e575b600080fd5b6100e76100e2366004610ad9565b61027e565b6040519081526020015b60405180910390f35b61010d610108366004610b1e565b61028f565b005b61010d61011d366004610b51565b610322565b61010d610130366004610b1e565b610397565b6100e760045481565b6100e761014c366004610b73565b610419565b61010d61015f366004610ad9565b61043e565b600054610177906001600160a01b031681565b6040516001600160a01b0390911681526020016100f1565b6101ef61019d366004610ad9565b60016020819052600091825260409091208054918101546002820154600383015460048401546005909401546001600160a01b0395861695938416949390921692909160ff8082169161010090041687565b6040516100f19796959493929190610bd4565b61010d610210366004610b73565b6104a3565b610238610223366004610b51565b60036020526000908152604090205460ff1681565b60405190151581526020016100f1565b610238610256366004610b51565b60026020526000908152604090205460ff1681565b61010d610279366004610c3a565b6106fe565b600061028982610962565b92915050565b6000546001600160a01b031633146102c25760405162461bcd60e51b81526004016102b990610c5d565b60405180910390fd5b6001600160a01b038216600081815260026020908152604091829020805460ff191685151590811790915591519182527fef81a9943b96c8df4ef243401c9bf5159146166211356898b52d382086168d9291015b60405180910390a25050565b6000546001600160a01b0316331461034c5760405162461bcd60e51b81526004016102b990610c5d565b600080546001600160a01b0319166001600160a01b0383169081178255604051909133917f8292fce18fa69edf4db7b94ea2e58241df0ae57f97e0a6c9b29067028bf92d769190a350565b6000546001600160a01b031633146103c15760405162461bcd60e51b81526004016102b990610c5d565b6001600160a01b038216600081815260036020908152604091829020805460ff191685151590811790915591519182527f8cc72bec7e2cf5979aefd933f40a28eb590098d522a5c458e497e71c5e6fa90f9101610316565b60008061042583610962565b9050610434868686868561097e565b9695505050505050565b6000546001600160a01b031633146104685760405162461bcd60e51b81526004016102b990610c5d565b60048190556040518181527fa2e09ae7caa0e84bcfe806d931934b92bf11ce67ffcaa3033af9a1e0319969519060200160405180910390a150565b6001600160a01b03821660009081526002602052604090205460ff166104e75760405163010d7e6160e11b81526001600160a01b03831660048201526024016102b9565b60006104f282610962565b90506000610503868686868661097e565b90506001600082815260016020526040902060050154610100900460ff16600281111561053257610532610bbe565b036105535760405163d5e880e960e01b8152600481018290526024016102b9565b60006040518060e00160405280886001600160a01b03168152602001876001600160a01b03168152602001866001600160a01b03168152602001858152602001848152602001600015158152602001600160028111156105b5576105b5610bbe565b9052600083815260016020818152604092839020845181546001600160a01b03199081166001600160a01b03928316178355928601519382018054841694821694909417909355928401516002808501805490931691909316179055606083015160038301556080830151600483015560a083015160058301805460ff198116921515928317825560c0860151959650869593919261ffff1990911661ff0019909116179061010090849081111561066f5761066f610bbe565b021790555061068c9150506001600160a01b0386168730866109d1565b6106a16001600160a01b0386163330876109d1565b604080516001600160a01b038781168252602082018790529181018590528188169189169084907fb10e0c42d0d8cd4a999f95c50d18109fb205fdf5e55a8ed89121cab7f9f65f699060600160405180910390a450505050505050565b3360009081526003602052604090205460ff1661072e576040516314e9dd3960e31b815260040160405180910390fd5b6000828152600160208181526040808420815160e08101835281546001600160a01b03908116825294820154851693810193909352600280820154909416918301919091526003810154606083015260048101546080830152600581015460ff808216151560a08501529293919260c085019261010090920416908111156107b8576107b8610bbe565b60028111156107c9576107c9610bbe565b905250905060028160c0015160028111156107e6576107e6610bbe565b036108075760405163b677167760e01b8152600481018490526024016102b9565b81156108415780516060820151608083015161083c929161082791610c99565b60408401516001600160a01b03169190610a5b565b61085d565b61085d8160200151826060015183608001516108279190610c99565b600260c0820181815283151560a08401908152600086815260016020818152604092839020875181546001600160a01b039182166001600160a01b0319918216178355928901519382018054948216948416949094179093559287015183870180549190931691161790556060850151600382015560808501516004820155905160058201805491151560ff198316811782559351869593949093919261ff001990911661ffff19909116179061010090849081111561091f5761091f610bbe565b02179055505060405183151581528491507fdbed7580b9c2829ee6b384e3833f10b16f9885601c98a01c40fd705b543e9c669060200160405180910390a2505050565b60006064600454836109749190610cb1565b6102899190610cd0565b604080516001600160a01b0396871660208083019190915295871681830152939095166060840152608083019190915260a0808301919091528351808303909101815260c0909101909252815191012090565b60006040516323b872dd60e01b81528460048201528360248201528260448201526020600060648360008a5af13d15601f3d1160016000511416171691505080610a545760405162461bcd60e51b81526020600482015260146024820152731514905394d1915497d19493d357d1905253115160621b60448201526064016102b9565b5050505050565b600060405163a9059cbb60e01b8152836004820152826024820152602060006044836000895af13d15601f3d1160016000511416171691505080610ad35760405162461bcd60e51b815260206004820152600f60248201526e1514905394d1915497d19052531151608a1b60448201526064016102b9565b50505050565b600060208284031215610aeb57600080fd5b5035919050565b80356001600160a01b0381168114610b0957600080fd5b919050565b80358015158114610b0957600080fd5b60008060408385031215610b3157600080fd5b610b3a83610af2565b9150610b4860208401610b0e565b90509250929050565b600060208284031215610b6357600080fd5b610b6c82610af2565b9392505050565b60008060008060808587031215610b8957600080fd5b610b9285610af2565b9350610ba060208601610af2565b9250610bae60408601610af2565b9396929550929360600135925050565b634e487b7160e01b600052602160045260246000fd5b6001600160a01b038881168252878116602083015286166040820152606081018590526080810184905282151560a082015260e0810160038310610c2857634e487b7160e01b600052602160045260246000fd5b8260c083015298975050505050505050565b60008060408385031215610c4d57600080fd5b82359150610b4860208401610b0e565b6020808252600c908201526b15539055551213d49256915160a21b604082015260600190565b634e487b7160e01b600052601160045260246000fd5b60008219821115610cac57610cac610c83565b500190565b6000816000190483118215151615610ccb57610ccb610c83565b500290565b600082610ced57634e487b7160e01b600052601260045260246000fd5b50049056fea2646970667358221220a086d5378c38188b517141b491168433bcacdd1ef66aa0a3a08acd1fabd82c2a64736f6c634300080f0033" . parse () . expect ("invalid bytecode")
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
        ) -> Result<
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
        #[doc = "Calls the contract's `ask` (0xaf5899fc) function"]
        pub fn ask(
            &self,
            proposer: ethers::core::types::Address,
            disputer: ethers::core::types::Address,
            bond_token: ethers::core::types::Address,
            stake: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
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
        #[doc = "Calls the contract's `disputeBondPct` (0x391fe4e2) function"]
        pub fn dispute_bond_pct(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([57, 31, 228, 226], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getRequestId` (0x84bfabcf) function"]
        pub fn get_request_id(
            &self,
            proposer: ethers::core::types::Address,
            disputer: ethers::core::types::Address,
            bond_token: ethers::core::types::Address,
            stake: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash(
                    [132, 191, 171, 207],
                    (proposer, disputer, bond_token, stake),
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
                ethers::core::types::U256,
                ethers::core::types::U256,
                bool,
                u8,
            ),
        > {
            self.0
                .method_hash([157, 134, 105, 133], p0)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setBondPct` (0x8945bd28) function"]
        pub fn set_bond_pct(
            &self,
            new_pct: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([137, 69, 189, 40], new_pct)
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
        BondPctChangedFilter(BondPctChangedFilter),
        NewRequestFilter(NewRequestFilter),
        OwnerUpdatedFilter(OwnerUpdatedFilter),
        RequestSettledFilter(RequestSettledFilter),
        SettlerWhitelistedFilter(SettlerWhitelistedFilter),
        TokenWhitelistedFilter(TokenWhitelistedFilter),
    }
    impl ethers::contract::EthLogDecode for AllKnowingOracleEvents {
        fn decode_log(log: &ethers::core::abi::RawLog) -> Result<Self, ethers::core::abi::Error>
        where
            Self: Sized,
        {
            if let Ok(decoded) = BondPctChangedFilter::decode_log(log) {
                return Ok(AllKnowingOracleEvents::BondPctChangedFilter(decoded));
            }
            if let Ok(decoded) = NewRequestFilter::decode_log(log) {
                return Ok(AllKnowingOracleEvents::NewRequestFilter(decoded));
            }
            if let Ok(decoded) = OwnerUpdatedFilter::decode_log(log) {
                return Ok(AllKnowingOracleEvents::OwnerUpdatedFilter(decoded));
            }
            if let Ok(decoded) = RequestSettledFilter::decode_log(log) {
                return Ok(AllKnowingOracleEvents::RequestSettledFilter(decoded));
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
                AllKnowingOracleEvents::BondPctChangedFilter(element) => element.fmt(f),
                AllKnowingOracleEvents::NewRequestFilter(element) => element.fmt(f),
                AllKnowingOracleEvents::OwnerUpdatedFilter(element) => element.fmt(f),
                AllKnowingOracleEvents::RequestSettledFilter(element) => element.fmt(f),
                AllKnowingOracleEvents::SettlerWhitelistedFilter(element) => element.fmt(f),
                AllKnowingOracleEvents::TokenWhitelistedFilter(element) => element.fmt(f),
            }
        }
    }
    #[doc = "Container type for all input parameters for the `ask` function with signature `ask(address,address,address,uint256)` and selector `[175, 88, 153, 252]`"]
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
    #[doc = "Container type for all input parameters for the `bondForStake` function with signature `bondForStake(uint256)` and selector `[7, 222, 153, 246]`"]
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
    #[doc = "Container type for all input parameters for the `disputeBondPct` function with signature `disputeBondPct()` and selector `[57, 31, 228, 226]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "disputeBondPct", abi = "disputeBondPct()")]
    pub struct DisputeBondPctCall;
    #[doc = "Container type for all input parameters for the `getRequestId` function with signature `getRequestId(address,address,address,uint256)` and selector `[132, 191, 171, 207]`"]
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
        abi = "getRequestId(address,address,address,uint256)"
    )]
    pub struct GetRequestIdCall {
        pub proposer: ethers::core::types::Address,
        pub disputer: ethers::core::types::Address,
        pub bond_token: ethers::core::types::Address,
        pub stake: ethers::core::types::U256,
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
    #[doc = "Container type for all input parameters for the `setBondPct` function with signature `setBondPct(uint256)` and selector `[137, 69, 189, 40]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "setBondPct", abi = "setBondPct(uint256)")]
    pub struct SetBondPctCall {
        pub new_pct: ethers::core::types::U256,
    }
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
        BondForStake(BondForStakeCall),
        DisputeBondPct(DisputeBondPctCall),
        GetRequestId(GetRequestIdCall),
        Owner(OwnerCall),
        Requests(RequestsCall),
        SetBondPct(SetBondPctCall),
        SetOwner(SetOwnerCall),
        Settle(SettleCall),
        Settlers(SettlersCall),
        WhitelistSettler(WhitelistSettlerCall),
        WhitelistToken(WhitelistTokenCall),
        WhitelistedTokens(WhitelistedTokensCall),
    }
    impl ethers::core::abi::AbiDecode for AllKnowingOracleCalls {
        fn decode(data: impl AsRef<[u8]>) -> Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) = <AskCall as ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(AllKnowingOracleCalls::Ask(decoded));
            }
            if let Ok(decoded) =
                <BondForStakeCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(AllKnowingOracleCalls::BondForStake(decoded));
            }
            if let Ok(decoded) =
                <DisputeBondPctCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(AllKnowingOracleCalls::DisputeBondPct(decoded));
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
                <RequestsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(AllKnowingOracleCalls::Requests(decoded));
            }
            if let Ok(decoded) =
                <SetBondPctCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(AllKnowingOracleCalls::SetBondPct(decoded));
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
                AllKnowingOracleCalls::BondForStake(element) => element.encode(),
                AllKnowingOracleCalls::DisputeBondPct(element) => element.encode(),
                AllKnowingOracleCalls::GetRequestId(element) => element.encode(),
                AllKnowingOracleCalls::Owner(element) => element.encode(),
                AllKnowingOracleCalls::Requests(element) => element.encode(),
                AllKnowingOracleCalls::SetBondPct(element) => element.encode(),
                AllKnowingOracleCalls::SetOwner(element) => element.encode(),
                AllKnowingOracleCalls::Settle(element) => element.encode(),
                AllKnowingOracleCalls::Settlers(element) => element.encode(),
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
                AllKnowingOracleCalls::BondForStake(element) => element.fmt(f),
                AllKnowingOracleCalls::DisputeBondPct(element) => element.fmt(f),
                AllKnowingOracleCalls::GetRequestId(element) => element.fmt(f),
                AllKnowingOracleCalls::Owner(element) => element.fmt(f),
                AllKnowingOracleCalls::Requests(element) => element.fmt(f),
                AllKnowingOracleCalls::SetBondPct(element) => element.fmt(f),
                AllKnowingOracleCalls::SetOwner(element) => element.fmt(f),
                AllKnowingOracleCalls::Settle(element) => element.fmt(f),
                AllKnowingOracleCalls::Settlers(element) => element.fmt(f),
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
    impl ::std::convert::From<BondForStakeCall> for AllKnowingOracleCalls {
        fn from(var: BondForStakeCall) -> Self {
            AllKnowingOracleCalls::BondForStake(var)
        }
    }
    impl ::std::convert::From<DisputeBondPctCall> for AllKnowingOracleCalls {
        fn from(var: DisputeBondPctCall) -> Self {
            AllKnowingOracleCalls::DisputeBondPct(var)
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
    impl ::std::convert::From<RequestsCall> for AllKnowingOracleCalls {
        fn from(var: RequestsCall) -> Self {
            AllKnowingOracleCalls::Requests(var)
        }
    }
    impl ::std::convert::From<SetBondPctCall> for AllKnowingOracleCalls {
        fn from(var: SetBondPctCall) -> Self {
            AllKnowingOracleCalls::SetBondPct(var)
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
    #[doc = "Container type for all return fields from the `bondForStake` function with signature `bondForStake(uint256)` and selector `[7, 222, 153, 246]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct BondForStakeReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `disputeBondPct` function with signature `disputeBondPct()` and selector `[57, 31, 228, 226]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct DisputeBondPctReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `getRequestId` function with signature `getRequestId(address,address,address,uint256)` and selector `[132, 191, 171, 207]`"]
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
        pub proposer: ethers::core::types::Address,
        pub disputer: ethers::core::types::Address,
        pub bond_token: ethers::core::types::Address,
        pub stake: ethers::core::types::U256,
        pub bond: ethers::core::types::U256,
        pub answer: bool,
        pub state: u8,
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
