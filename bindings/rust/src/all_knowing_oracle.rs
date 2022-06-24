pub use allknowingoracle_mod::*;
#[allow(clippy::too_many_arguments, non_camel_case_types)]
pub mod allknowingoracle_mod {
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
            serde_json :: from_str ("[{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"constructor\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"id\",\"type\":\"bytes32\",\"components\":[]}],\"type\":\"error\",\"name\":\"AllKnowingOracle__AlreadySettled\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\",\"components\":[]}],\"type\":\"error\",\"name\":\"AllKnowingOracle__NotWhitelisted\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"id\",\"type\":\"bytes32\",\"components\":[]}],\"type\":\"error\",\"name\":\"AllKnowingOracle__RequestAlreadyExists\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"newPct\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"BondPctChanged\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"id\",\"type\":\"bytes32\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"proposer\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"disputer\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"bondToken\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"stake\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"bond\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"NewRequest\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"user\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"newOwner\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"OwnerUpdated\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"id\",\"type\":\"bytes32\",\"components\":[],\"indexed\":true},{\"internalType\":\"bool\",\"name\":\"answer\",\"type\":\"bool\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"RequestSettled\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"bool\",\"name\":\"enabled\",\"type\":\"bool\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"TokenWhitelisted\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"proposer\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"disputer\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"bondToken\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"stake\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"ask\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"stake\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"bondForStake\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"disputeBondPct\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"owner\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"requests\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"proposer\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"disputer\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"bondToken\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"stake\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"bond\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bool\",\"name\":\"answer\",\"type\":\"bool\",\"components\":[]},{\"internalType\":\"enum RequestState\",\"name\":\"state\",\"type\":\"uint8\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_pct\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setBondPct\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"newOwner\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setOwner\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"id\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"bool\",\"name\":\"answer\",\"type\":\"bool\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"settle\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bool\",\"name\":\"enabled\",\"type\":\"bool\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"whitelistToken\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"whitelistedTokens\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]}]") . expect ("invalid abi")
        });
    #[doc = r" Bytecode of the #name contract"]
    pub static ALLKNOWINGORACLE_BYTECODE: ethers::contract::Lazy<ethers::core::types::Bytes> =
        ethers::contract::Lazy::new(|| {
            "0x608060405234801561001057600080fd5b50600080546001600160a01b031916339081178255604051909182917f8292fce18fa69edf4db7b94ea2e58241df0ae57f97e0a6c9b29067028bf92d76908290a350601960038190556040519081527fa2e09ae7caa0e84bcfe806d931934b92bf11ce67ffcaa3033af9a1e0319969519060200160405180910390a1610c578061009b6000396000f3fe608060405234801561001057600080fd5b506004361061009e5760003560e01c80638da5cb5b116100665780638da5cb5b1461010d5780639d86698514610138578063af5899fc146101ab578063daf9c210146101be578063fc361c38146101f157600080fd5b806307de99f6146100a35780630ffb1d8b146100c957806313af4035146100de578063391fe4e2146100f15780638945bd28146100fa575b600080fd5b6100b66100b13660046109e4565b610204565b6040519081526020015b60405180910390f35b6100dc6100d7366004610a2a565b610215565b005b6100dc6100ec366004610a61565b610273565b6100b660035481565b6100dc6101083660046109e4565b6102e8565b600054610120906001600160a01b031681565b6040516001600160a01b0390911681526020016100c0565b6101986101463660046109e4565b60016020819052600091825260409091208054918101546002820154600383015460048401546005909401546001600160a01b0395861695938416949390921692909160ff8082169161010090041687565b6040516100c09796959493929190610a99565b6100b66101b9366004610aff565b61034d565b6101e16101cc366004610a61565b60026020526000908152604090205460ff1681565b60405190151581526020016100c0565b6100dc6101ff366004610b4a565b6105ee565b600061020f8261093e565b92915050565b6000546001600160a01b031633146102485760405162461bcd60e51b815260040161023f90610b6f565b60405180910390fd5b6001600160a01b03919091166000908152600260205260409020805460ff1916911515919091179055565b6000546001600160a01b0316331461029d5760405162461bcd60e51b815260040161023f90610b6f565b600080546001600160a01b0319166001600160a01b0383169081178255604051909133917f8292fce18fa69edf4db7b94ea2e58241df0ae57f97e0a6c9b29067028bf92d769190a350565b6000546001600160a01b031633146103125760405162461bcd60e51b815260040161023f90610b6f565b60038190556040518181527fa2e09ae7caa0e84bcfe806d931934b92bf11ce67ffcaa3033af9a1e0319969519060200160405180910390a150565b6001600160a01b03821660009081526002602052604081205460ff166103915760405163010d7e6160e11b81526001600160a01b038416600482015260240161023f565b600061039c8361093e565b604080516001600160a01b03808a1660208301528089169282019290925290861660608201526080810185905260a0810182905290915060009060c00160408051601f19818403018152919052805160209091012090506001600082815260016020526040902060050154610100900460ff16600281111561042057610420610a83565b036104415760405163d5e880e960e01b81526004810182905260240161023f565b60006040518060e00160405280896001600160a01b03168152602001886001600160a01b03168152602001876001600160a01b03168152602001868152602001848152602001600015158152602001600160028111156104a3576104a3610a83565b9052600083815260016020818152604092839020845181546001600160a01b03199081166001600160a01b03928316178355928601519382018054841694821694909417909355928401516002808501805490931691909316179055606083015160038301556080830151600483015560a083015160058301805460ff198116921515928317825560c0860151959650869593919261ffff1990911661ff0019909116179061010090849081111561055d5761055d610a83565b021790555061057a9150506001600160a01b03871688308661095a565b61058f6001600160a01b03871633308861095a565b604080516001600160a01b03888116825260208201889052918101859052818916918a169084907fb10e0c42d0d8cd4a999f95c50d18109fb205fdf5e55a8ed89121cab7f9f65f699060600160405180910390a4509695505050505050565b6000546001600160a01b031633146106185760405162461bcd60e51b815260040161023f90610b6f565b6000828152600160208181526040808420815160e08101835281546001600160a01b03908116825294820154851693810193909352600280820154909416918301919091526003810154606083015260048101546080830152600581015460ff808216151560a08501529293919260c085019261010090920416908111156106a2576106a2610a83565b60028111156106b3576106b3610a83565b905250905060028160c0015160028111156106d0576106d0610a83565b036106f15760405163b677167760e01b81526004810184905260240161023f565b81151560010361079c5780604001516001600160a01b031663a9059cbb8260000151836060015184608001516107279190610bab565b6040516001600160e01b031960e085901b1681526001600160a01b03909216600483015260248201526044016020604051808303816000875af1158015610772573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906107969190610bc3565b50610839565b80604001516001600160a01b031663a9059cbb8260200151836060015184608001516107c89190610bab565b6040516001600160e01b031960e085901b1681526001600160a01b03909216600483015260248201526044016020604051808303816000875af1158015610813573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906108379190610bc3565b505b600260c0820181815283151560a08401908152600086815260016020818152604092839020875181546001600160a01b039182166001600160a01b0319918216178355928901519382018054948216948416949094179093559287015183870180549190931691161790556060850151600382015560808501516004820155905160058201805491151560ff198316811782559351869593949093919261ff001990911661ffff1990911617906101009084908111156108fb576108fb610a83565b02179055505060405183151581528491507fdbed7580b9c2829ee6b384e3833f10b16f9885601c98a01c40fd705b543e9c669060200160405180910390a2505050565b60006064600354836109509190610be0565b61020f9190610bff565b60006040516323b872dd60e01b81528460048201528360248201528260448201526020600060648360008a5af13d15601f3d11600160005114161716915050806109dd5760405162461bcd60e51b81526020600482015260146024820152731514905394d1915497d19493d357d1905253115160621b604482015260640161023f565b5050505050565b6000602082840312156109f657600080fd5b5035919050565b80356001600160a01b0381168114610a1457600080fd5b919050565b8015158114610a2757600080fd5b50565b60008060408385031215610a3d57600080fd5b610a46836109fd565b91506020830135610a5681610a19565b809150509250929050565b600060208284031215610a7357600080fd5b610a7c826109fd565b9392505050565b634e487b7160e01b600052602160045260246000fd5b6001600160a01b038881168252878116602083015286166040820152606081018590526080810184905282151560a082015260e0810160038310610aed57634e487b7160e01b600052602160045260246000fd5b8260c083015298975050505050505050565b60008060008060808587031215610b1557600080fd5b610b1e856109fd565b9350610b2c602086016109fd565b9250610b3a604086016109fd565b9396929550929360600135925050565b60008060408385031215610b5d57600080fd5b823591506020830135610a5681610a19565b6020808252600c908201526b15539055551213d49256915160a21b604082015260600190565b634e487b7160e01b600052601160045260246000fd5b60008219821115610bbe57610bbe610b95565b500190565b600060208284031215610bd557600080fd5b8151610a7c81610a19565b6000816000190483118215151615610bfa57610bfa610b95565b500290565b600082610c1c57634e487b7160e01b600052601260045260246000fd5b50049056fea26469706673582212205bdec491885dcc1ed8f1a4dbb49b3f7c52371c21a2e5f51f7e6207c148789ec264736f6c634300080d0033" . parse () . expect ("invalid bytecode")
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
        #[doc = "Calls the contract's `disputeBondPct` (0x391fe4e2) function"]
        pub fn dispute_bond_pct(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([57, 31, 228, 226], ())
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
            pct: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([137, 69, 189, 40], pct)
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
                AllKnowingOracleEvents::TokenWhitelistedFilter(element) => element.fmt(f),
            }
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
    #[doc = "Container type for all input parameters for the `disputeBondPct`function with signature `disputeBondPct()` and selector `[57, 31, 228, 226]`"]
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
    #[doc = "Container type for all input parameters for the `owner`function with signature `owner()` and selector `[141, 165, 203, 91]`"]
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
    #[doc = "Container type for all input parameters for the `requests`function with signature `requests(bytes32)` and selector `[157, 134, 105, 133]`"]
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
    #[doc = "Container type for all input parameters for the `setBondPct`function with signature `setBondPct(uint256)` and selector `[137, 69, 189, 40]`"]
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
        pub pct: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `setOwner`function with signature `setOwner(address)` and selector `[19, 175, 64, 53]`"]
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
    #[doc = "Container type for all input parameters for the `whitelistToken`function with signature `whitelistToken(address,bool)` and selector `[15, 251, 29, 139]`"]
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
    #[doc = "Container type for all input parameters for the `whitelistedTokens`function with signature `whitelistedTokens(address)` and selector `[218, 249, 194, 16]`"]
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
        Owner(OwnerCall),
        Requests(RequestsCall),
        SetBondPct(SetBondPctCall),
        SetOwner(SetOwnerCall),
        Settle(SettleCall),
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
                AllKnowingOracleCalls::Owner(element) => element.encode(),
                AllKnowingOracleCalls::Requests(element) => element.encode(),
                AllKnowingOracleCalls::SetBondPct(element) => element.encode(),
                AllKnowingOracleCalls::SetOwner(element) => element.encode(),
                AllKnowingOracleCalls::Settle(element) => element.encode(),
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
                AllKnowingOracleCalls::Owner(element) => element.fmt(f),
                AllKnowingOracleCalls::Requests(element) => element.fmt(f),
                AllKnowingOracleCalls::SetBondPct(element) => element.fmt(f),
                AllKnowingOracleCalls::SetOwner(element) => element.fmt(f),
                AllKnowingOracleCalls::Settle(element) => element.fmt(f),
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
}
