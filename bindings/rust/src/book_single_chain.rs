pub use book_single_chain::*;
#[allow(clippy::too_many_arguments, non_camel_case_types)]
pub mod book_single_chain {
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
    #[doc = "BookSingleChain was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static BOOKSINGLECHAIN_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            ethers :: core :: utils :: __serde_json :: from_str ("[{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_safeBlockThreshold\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"_oracleAddress\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"constructor\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"blocksLeft\",\"type\":\"uint256\",\"components\":[]}],\"type\":\"error\",\"name\":\"BookSingleChain__DisputePeriodNotOver\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"BookSingleChain__DisputePeriodOver\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"fee\",\"type\":\"uint256\",\"components\":[]}],\"type\":\"error\",\"name\":\"BookSingleChain__FeePctTooHigh\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"BookSingleChain__InvalidSignature\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\",\"components\":[]}],\"type\":\"error\",\"name\":\"BookSingleChain__InvalidToken\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"BookSingleChain__NewFeePctTooHigh\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"BookSingleChain__SameToken\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"BookSingleChain__SentToBlackHole\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"tradeId\",\"type\":\"bytes32\",\"components\":[]}],\"type\":\"error\",\"name\":\"BookSingleChain__TradeAlreadyFilled\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"tradeId\",\"type\":\"bytes32\",\"components\":[]}],\"type\":\"error\",\"name\":\"BookSingleChain__TradeNotFilled\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\",\"components\":[]}],\"type\":\"error\",\"name\":\"BookSingleChain__UnsafeTokenToWhitelist\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"BookSingleChain__ZeroAmount\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"newMaxFeePct\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"MaxFeePctChanged\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"user\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"newOwner\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"OwnerUpdated\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"newSafeBlockThreshold\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"SafeBlockThresholdChanged\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"bool\",\"name\":\"whitelisted\",\"type\":\"bool\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"TokenWhitelisted\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"relayer\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"tradeIndex\",\"type\":\"uint256\",\"components\":[],\"indexed\":true},{\"internalType\":\"bytes32\",\"name\":\"disputeId\",\"type\":\"bytes32\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"filledAmount\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"feePct\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"TradeDisputed\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"relayer\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"tradeIndex\",\"type\":\"uint256\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"feePct\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"amountOut\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"TradeFilled\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"tokenIn\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"tokenOut\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"amountIn\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"feePct\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"tradeIndex\",\"type\":\"uint256\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"TradeRequested\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"relayer\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"tradeIndex\",\"type\":\"uint256\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"filledAmount\",\"type\":\"uint256\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"feePct\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"TradeSettled\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"trader\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"tradeIndex\",\"type\":\"uint256\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"newFeePct\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"UpdatedFeeForTrade\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"tokenIn\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"tokenOut\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amountIn\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"feePct\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"recipient\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"tradeIndex\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"disputeTrade\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"tokenIn\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"tokenOut\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amountIn\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"feePct\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"recipient\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"tradeIndex\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amountToSend\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"fillTrade\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"tokenIn\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"tokenOut\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amountIn\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"feePct\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"recipient\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"tradeIndex\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amountToSend\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"trader\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"newFeePct\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"traderSignature\",\"type\":\"bytes\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"fillTradeWithUpdatedFee\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"filledAmount\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"filledAtBlock\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"filledBy\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"maxFeePct\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"numberOfTrades\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"oracle\",\"outputs\":[{\"internalType\":\"contract IOracle\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"owner\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"tokenIn\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"tokenOut\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amountIn\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"feePct\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"recipient\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"requestTrade\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"safeBlockThreshold\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"newMaxFeePct\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setMaxFeePct\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"newOwner\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setOwner\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"newSafeBlockThreshold\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setSafeBlockThreshold\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"tokenIn\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"tokenOut\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amountIn\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"feePct\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"recipient\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"tradeIndex\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"settleTrade\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"tokenIn\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"tokenOut\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amountIn\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"feePct\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"recipient\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"tradeIndex\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"trader\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"newFeePct\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"traderSignature\",\"type\":\"bytes\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"updateFeeForTrade\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bool\",\"name\":\"whitelisted\",\"type\":\"bool\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"whitelistToken\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"whitelistedTokens\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]}]") . expect ("invalid abi")
        });
    #[doc = r" Bytecode of the #name contract"]
    pub static BOOKSINGLECHAIN_BYTECODE: ethers::contract::Lazy<ethers::core::types::Bytes> =
        ethers::contract::Lazy::new(|| {
            "0x60a060405260018055600060025534801561001957600080fd5b50604051611a3f380380611a3f83398101604081905261003891610106565b600080546001600160a01b031916339081178255604051909182917f8292fce18fa69edf4db7b94ea2e58241df0ae57f97e0a6c9b29067028bf92d76908290a3506001600160a01b03811660805260038290556040518281527fcf29a5174acb8c175d760a7381ffc52c6ae644e3a4ba3fa7e01344f959cd76159060200160405180910390a16703782dace9d9000060048190556040519081527f841095ec206e4a3d8124f54a431661bd653b296066d7d695baaa9178e9d21bb49060200160405180910390a15050610143565b6000806040838503121561011957600080fd5b825160208401519092506001600160a01b038116811461013857600080fd5b809150509250929050565b6080516118c561017a600039600081816101c80152818161035c015281816106540152818161071e015261077701526118c56000f3fe608060405234801561001057600080fd5b50600436106101215760003560e01c80639170c05b116100ad578063d70e3dfd11610071578063d70e3dfd14610277578063daf9c210146102a0578063e02c0279146102d3578063ec4cd7db146102e6578063fc711c3a146102f957600080fd5b80639170c05b146102155780639501325f14610228578063bd20a85914610248578063bd4ac9971461025b578063cd805d5e1461026e57600080fd5b80632613f307116100f45780632613f3071461018a5780632b0cf6531461019d5780637d9dd85d146101b05780637dc0d1d0146101c35780638da5cb5b1461020257600080fd5b80630b20b7bc146101265780630ff0c00e146101595780630ffb1d8b1461016257806313af403514610177575b600080fd5b610146610134366004611454565b60086020526000908152604090205481565b6040519081526020015b60405180910390f35b61014660035481565b610175610170366004611497565b610302565b005b6101756101853660046114ce565b610451565b610175610198366004611454565b6104c6565b6101756101ab3660046114f0565b61052c565b6101756101be366004611591565b6107e4565b6101ea7f000000000000000000000000000000000000000000000000000000000000000081565b6040516001600160a01b039091168152602001610150565b6000546101ea906001600160a01b031681565b61017561022336600461164d565b610859565b610146610236366004611454565b60066020526000908152604090205481565b610175610256366004611454565b610a06565b6101756102693660046116a4565b610a8d565b61014660025481565b6101ea610285366004611454565b6007602052600090815260409020546001600160a01b031681565b6102c36102ae3660046114ce565b60056020526000908152604090205460ff1681565b6040519015158152602001610150565b6101756102e13660046114f0565b610b53565b6101756102f4366004611755565b610cb6565b61014660045481565b6000546001600160a01b031633146103355760405162461bcd60e51b815260040161032c906117bf565b60405180910390fd5b8080156103c95750604051630daf9c2160e41b81526001600160a01b0383811660048301527f0000000000000000000000000000000000000000000000000000000000000000169063daf9c21090602401602060405180830381865afa1580156103a3573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906103c791906117e5565b155b156103f2576040516313c42eef60e21b81526001600160a01b038316600482015260240161032c565b6001600160a01b038216600081815260056020908152604091829020805460ff191685151590811790915591519182527fef81a9943b96c8df4ef243401c9bf5159146166211356898b52d382086168d92910160405180910390a25050565b6000546001600160a01b0316331461047b5760405162461bcd60e51b815260040161032c906117bf565b600080546001600160a01b0319166001600160a01b0383169081178255604051909133917f8292fce18fa69edf4db7b94ea2e58241df0ae57f97e0a6c9b29067028bf92d769190a350565b6000546001600160a01b031633146104f05760405162461bcd60e51b815260040161032c906117bf565b60038190556040518181527fcf29a5174acb8c175d760a7381ffc52c6ae644e3a4ba3fa7e01344f959cd7615906020015b60405180910390a150565b60015460011461056b5760405162461bcd60e51b815260206004820152600a6024820152695245454e5452414e435960b01b604482015260640161032c565b60026001556000610580878787878787610d4b565b6000818152600660205260408120549192508190036105b55760405163cc4f06a160e01b81526004810183905260240161032c565b6003546105c28243611818565b106105e057604051632c02744560e11b815260040160405180910390fd5b600082815260086020818152604080842080546007845282862080546006865284882088905581546001600160a01b03191690915594909352849055516384bfabcf60e01b81526001600160a01b03928316600482018190523360248301528b8416604483015260648201839052919391927f000000000000000000000000000000000000000000000000000000000000000016906384bfabcf90608401602060405180830381865afa15801561069b573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906106bf919061182f565b90508086836001600160a01b03167fc293ada1a20d5f6111738a64aec9c7262db402de1650b576e691e321964bb01e868c604051610707929190918252602082015260400190565b60405180910390a46107436001600160a01b038b167f000000000000000000000000000000000000000000000000000000000000000085610da3565b604051632bd6267f60e21b81526001600160a01b0383811660048301523360248301528b81166044830152606482018590527f0000000000000000000000000000000000000000000000000000000000000000169063af5899fc90608401600060405180830381600087803b1580156107bb57600080fd5b505af11580156107cf573d6000803e3d6000fd5b50506001805550505050505050505050505050565b60006107f48c8c8c8c8c8c610d4b565b90506108038582868686610e20565b61080e8b8288610f8d565b6040805185815260208101889052889133917f942417ccf4f356e8d909c054f8a8147622647605cbeafd9c63b4fc3cc1dd2a53910160405180910390a3505050505050505050505050565b6001600160a01b03851660009081526005602052604090205460ff1661089d5760405163f602627d60e01b81526001600160a01b038616600482015260240161032c565b6001600160a01b03841660009081526005602052604090205460ff166108e15760405163f602627d60e01b81526001600160a01b038516600482015260240161032c565b836001600160a01b0316856001600160a01b03160361091357604051631d2792fb60e31b815260040160405180910390fd5b600454821115610939576040516305bf279d60e41b81526004810183905260240161032c565b8260000361095a5760405163abc5ee6f60e01b815260040160405180910390fd5b6001600160a01b03811661098157604051631feef77d60e01b815260040160405180910390fd5b6109966001600160a01b038616333086610fe1565b60025460408051858152602081018590526001600160a01b03848116828401529151878316928916917f7361c265d28ece9d5df249995186533440e0b7a1310ae54d496fa1783056e3da919081900360600190a4600280549060006109fa83611848565b91905055505050505050565b6000546001600160a01b03163314610a305760405162461bcd60e51b815260040161032c906117bf565b670de0b6b3a76400008110610a5857604051636a143fdd60e11b815260040160405180910390fd5b60048190556040518181527f841095ec206e4a3d8124f54a431661bd653b296066d7d695baaa9178e9d21bb490602001610521565b6000610a9d8b8b8b8b8b8b610d4b565b9050600454841115610ac5576040516305bf279d60e41b81526004810185905260240161032c565b60008181526006602052604090205415610af5576040516304daa62560e21b81526004810182905260240161032c565b610b028582868686610e20565b85856001600160a01b03167f6ab91dbc42f726b630639350395426be048c50255f12e82d28e2dffac417459386604051610b3e91815260200190565b60405180910390a35050505050505050505050565b6000610b63878787878787610d4b565b60008181526006602052604081205491925003610b965760405163cc4f06a160e01b81526004810182905260240161032c565b600354600082815260066020526040902054610bb29043611818565b1015610bfd57600081815260066020526040812054610bd19043611818565b600354610bde9190611818565b9050806040516325797e0360e11b815260040161032c91815260200190565b600081815260086020818152604080842080546007845282862080546006865293872087905580546001600160a01b031916905593909252929055906001600160a01b0390811690610c52908916868461106b565b610c666001600160a01b038a16828961106b565b8184826001600160a01b03167f3281f74a3f7405b6bd35e9687b3fcaaf242c466ac789d117f22b62b140af8dcc89604051610ca391815260200190565b60405180910390a4505050505050505050565b6000610cc6888888888888610d4b565b60008181526006602052604090205490915015610cf9576040516304daa62560e21b81526004810182905260240161032c565b610d04878284610f8d565b6040805186815260208101849052849133917f942417ccf4f356e8d909c054f8a8147622647605cbeafd9c63b4fc3cc1dd2a53910160405180910390a35050505050505050565b604080516001600160a01b039788166020808301919091529688168183015260608101959095526080850193909352941660a083015260c0808301949094528051808303909401845260e09091019052815191012090565b600060405163095ea7b360e01b8152836004820152826024820152602060006044836000895af13d15601f3d1160016000511416171691505080610e1a5760405162461bcd60e51b815260206004820152600e60248201526d1054141493d59157d1905253115160921b604482015260640161032c565b50505050565b600454831115610e46576040516305bf279d60e41b81526004810184905260240161032c565b60008481526006602052604090205415610e76576040516304daa62560e21b81526004810185905260240161032c565b604080517f0efb9dda140a951df4393d44ca40349032d31811466afd20eacd4b4136c3f4986020808301919091528183018790526060808301879052835180840390910181526080830184528051908201207f19457468657265756d205369676e6564204d6573736167653a0a33320000000060a084015260bc8084018290528451808503909101815260dc90930190935281519101206000610f4f8286868080601f0160208091040260200160405190810160405280939291908181526020018383808284376000920191909152506110e392505050565b9050876001600160a01b0316816001600160a01b031614610f83576040516324a0dbd760e21b815260040160405180910390fd5b5050505050505050565b60008281526006602090815260408083204390556007825280832080546001600160a01b031916339081179091556008909252909120829055610fdc906001600160a01b038516903084610fe1565b505050565b60006040516323b872dd60e01b81528460048201528360248201528260448201526020600060648360008a5af13d15601f3d11600160005114161716915050806110645760405162461bcd60e51b81526020600482015260146024820152731514905394d1915497d19493d357d1905253115160621b604482015260640161032c565b5050505050565b600060405163a9059cbb60e01b8152836004820152826024820152602060006044836000895af13d15601f3d1160016000511416171691505080610e1a5760405162461bcd60e51b815260206004820152600f60248201526e1514905394d1915497d19052531151608a1b604482015260640161032c565b60008060006110f28585611107565b915091506110ff81611175565b509392505050565b600080825160410361113d5760208301516040840151606085015160001a6111318782858561132e565b9450945050505061116e565b8251604003611166576020830151604084015161115b86838361141b565b93509350505061116e565b506000905060025b9250929050565b600081600481111561118957611189611861565b036111915750565b60018160048111156111a5576111a5611861565b036111f25760405162461bcd60e51b815260206004820152601860248201527f45434453413a20696e76616c6964207369676e61747572650000000000000000604482015260640161032c565b600281600481111561120657611206611861565b036112535760405162461bcd60e51b815260206004820152601f60248201527f45434453413a20696e76616c6964207369676e6174757265206c656e67746800604482015260640161032c565b600381600481111561126757611267611861565b036112bf5760405162461bcd60e51b815260206004820152602260248201527f45434453413a20696e76616c6964207369676e6174757265202773272076616c604482015261756560f01b606482015260840161032c565b60048160048111156112d3576112d3611861565b0361132b5760405162461bcd60e51b815260206004820152602260248201527f45434453413a20696e76616c6964207369676e6174757265202776272076616c604482015261756560f01b606482015260840161032c565b50565b6000807f7fffffffffffffffffffffffffffffff5d576e7357a4501ddfe92f46681b20a08311156113655750600090506003611412565b8460ff16601b1415801561137d57508460ff16601c14155b1561138e5750600090506004611412565b6040805160008082526020820180845289905260ff881692820192909252606081018690526080810185905260019060a0016020604051602081039080840390855afa1580156113e2573d6000803e3d6000fd5b5050604051601f1901519150506001600160a01b03811661140b57600060019250925050611412565b9150600090505b94509492505050565b6000806001600160ff1b0383168161143860ff86901c601b611877565b90506114468782888561132e565b935093505050935093915050565b60006020828403121561146657600080fd5b5035919050565b80356001600160a01b038116811461148457600080fd5b919050565b801515811461132b57600080fd5b600080604083850312156114aa57600080fd5b6114b38361146d565b915060208301356114c381611489565b809150509250929050565b6000602082840312156114e057600080fd5b6114e98261146d565b9392505050565b60008060008060008060c0878903121561150957600080fd5b6115128761146d565b95506115206020880161146d565b9450604087013593506060870135925061153c6080880161146d565b915060a087013590509295509295509295565b60008083601f84011261156157600080fd5b50813567ffffffffffffffff81111561157957600080fd5b60208301915083602082850101111561116e57600080fd5b60008060008060008060008060008060006101408c8e0312156115b357600080fd5b6115bc8c61146d565b9a506115ca60208d0161146d565b995060408c0135985060608c013597506115e660808d0161146d565b965060a08c0135955060c08c0135945061160260e08d0161146d565b93506101008c013592506101208c013567ffffffffffffffff81111561162757600080fd5b6116338e828f0161154f565b915080935050809150509295989b509295989b9093969950565b600080600080600060a0868803121561166557600080fd5b61166e8661146d565b945061167c6020870161146d565b935060408601359250606086013591506116986080870161146d565b90509295509295909350565b6000806000806000806000806000806101208b8d0312156116c457600080fd5b6116cd8b61146d565b99506116db60208c0161146d565b985060408b0135975060608b013596506116f760808c0161146d565b955060a08b0135945061170c60c08c0161146d565b935060e08b013592506101008b013567ffffffffffffffff81111561173057600080fd5b61173c8d828e0161154f565b915080935050809150509295989b9194979a5092959850565b600080600080600080600060e0888a03121561177057600080fd5b6117798861146d565b96506117876020890161146d565b955060408801359450606088013593506117a36080890161146d565b925060a0880135915060c0880135905092959891949750929550565b6020808252600c908201526b15539055551213d49256915160a21b604082015260600190565b6000602082840312156117f757600080fd5b81516114e981611489565b634e487b7160e01b600052601160045260246000fd5b60008282101561182a5761182a611802565b500390565b60006020828403121561184157600080fd5b5051919050565b60006001820161185a5761185a611802565b5060010190565b634e487b7160e01b600052602160045260246000fd5b6000821982111561188a5761188a611802565b50019056fea2646970667358221220113fb638bdd45eef8a6121716cc16f962536180e2705f01f90d7a5598262081364736f6c634300080f0033" . parse () . expect ("invalid bytecode")
        });
    pub struct BookSingleChain<M>(ethers::contract::Contract<M>);
    impl<M> Clone for BookSingleChain<M> {
        fn clone(&self) -> Self {
            BookSingleChain(self.0.clone())
        }
    }
    impl<M> std::ops::Deref for BookSingleChain<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug for BookSingleChain<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(BookSingleChain))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ethers::providers::Middleware> BookSingleChain<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(address.into(), BOOKSINGLECHAIN_ABI.clone(), client)
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
                BOOKSINGLECHAIN_ABI.clone(),
                BOOKSINGLECHAIN_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        #[doc = "Calls the contract's `disputeTrade` (0x2b0cf653) function"]
        pub fn dispute_trade(
            &self,
            token_in: ethers::core::types::Address,
            token_out: ethers::core::types::Address,
            amount_in: ethers::core::types::U256,
            fee_pct: ethers::core::types::U256,
            recipient: ethers::core::types::Address,
            trade_index: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [43, 12, 246, 83],
                    (
                        token_in,
                        token_out,
                        amount_in,
                        fee_pct,
                        recipient,
                        trade_index,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `fillTrade` (0xec4cd7db) function"]
        pub fn fill_trade(
            &self,
            token_in: ethers::core::types::Address,
            token_out: ethers::core::types::Address,
            amount_in: ethers::core::types::U256,
            fee_pct: ethers::core::types::U256,
            recipient: ethers::core::types::Address,
            trade_index: ethers::core::types::U256,
            amount_to_send: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [236, 76, 215, 219],
                    (
                        token_in,
                        token_out,
                        amount_in,
                        fee_pct,
                        recipient,
                        trade_index,
                        amount_to_send,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `fillTradeWithUpdatedFee` (0x7d9dd85d) function"]
        pub fn fill_trade_with_updated_fee(
            &self,
            token_in: ethers::core::types::Address,
            token_out: ethers::core::types::Address,
            amount_in: ethers::core::types::U256,
            fee_pct: ethers::core::types::U256,
            recipient: ethers::core::types::Address,
            trade_index: ethers::core::types::U256,
            amount_to_send: ethers::core::types::U256,
            trader: ethers::core::types::Address,
            new_fee_pct: ethers::core::types::U256,
            trader_signature: ethers::core::types::Bytes,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [125, 157, 216, 93],
                    (
                        token_in,
                        token_out,
                        amount_in,
                        fee_pct,
                        recipient,
                        trade_index,
                        amount_to_send,
                        trader,
                        new_fee_pct,
                        trader_signature,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `filledAmount` (0x0b20b7bc) function"]
        pub fn filled_amount(
            &self,
            p0: [u8; 32],
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([11, 32, 183, 188], p0)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `filledAtBlock` (0x9501325f) function"]
        pub fn filled_at_block(
            &self,
            p0: [u8; 32],
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([149, 1, 50, 95], p0)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `filledBy` (0xd70e3dfd) function"]
        pub fn filled_by(
            &self,
            p0: [u8; 32],
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([215, 14, 61, 253], p0)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `maxFeePct` (0xfc711c3a) function"]
        pub fn max_fee_pct(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([252, 113, 28, 58], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `numberOfTrades` (0xcd805d5e) function"]
        pub fn number_of_trades(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([205, 128, 93, 94], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `oracle` (0x7dc0d1d0) function"]
        pub fn oracle(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([125, 192, 209, 208], ())
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
        #[doc = "Calls the contract's `requestTrade` (0x9170c05b) function"]
        pub fn request_trade(
            &self,
            token_in: ethers::core::types::Address,
            token_out: ethers::core::types::Address,
            amount_in: ethers::core::types::U256,
            fee_pct: ethers::core::types::U256,
            recipient: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [145, 112, 192, 91],
                    (token_in, token_out, amount_in, fee_pct, recipient),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `safeBlockThreshold` (0x0ff0c00e) function"]
        pub fn safe_block_threshold(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([15, 240, 192, 14], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setMaxFeePct` (0xbd20a859) function"]
        pub fn set_max_fee_pct(
            &self,
            new_max_fee_pct: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([189, 32, 168, 89], new_max_fee_pct)
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
        #[doc = "Calls the contract's `setSafeBlockThreshold` (0x2613f307) function"]
        pub fn set_safe_block_threshold(
            &self,
            new_safe_block_threshold: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([38, 19, 243, 7], new_safe_block_threshold)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `settleTrade` (0xe02c0279) function"]
        pub fn settle_trade(
            &self,
            token_in: ethers::core::types::Address,
            token_out: ethers::core::types::Address,
            amount_in: ethers::core::types::U256,
            fee_pct: ethers::core::types::U256,
            recipient: ethers::core::types::Address,
            trade_index: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [224, 44, 2, 121],
                    (
                        token_in,
                        token_out,
                        amount_in,
                        fee_pct,
                        recipient,
                        trade_index,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `updateFeeForTrade` (0xbd4ac997) function"]
        pub fn update_fee_for_trade(
            &self,
            token_in: ethers::core::types::Address,
            token_out: ethers::core::types::Address,
            amount_in: ethers::core::types::U256,
            fee_pct: ethers::core::types::U256,
            recipient: ethers::core::types::Address,
            trade_index: ethers::core::types::U256,
            trader: ethers::core::types::Address,
            new_fee_pct: ethers::core::types::U256,
            trader_signature: ethers::core::types::Bytes,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [189, 74, 201, 151],
                    (
                        token_in,
                        token_out,
                        amount_in,
                        fee_pct,
                        recipient,
                        trade_index,
                        trader,
                        new_fee_pct,
                        trader_signature,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `whitelistToken` (0x0ffb1d8b) function"]
        pub fn whitelist_token(
            &self,
            token: ethers::core::types::Address,
            whitelisted: bool,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([15, 251, 29, 139], (token, whitelisted))
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
        #[doc = "Gets the contract's `MaxFeePctChanged` event"]
        pub fn max_fee_pct_changed_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, MaxFeePctChangedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `OwnerUpdated` event"]
        pub fn owner_updated_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, OwnerUpdatedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `SafeBlockThresholdChanged` event"]
        pub fn safe_block_threshold_changed_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, SafeBlockThresholdChangedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `TokenWhitelisted` event"]
        pub fn token_whitelisted_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, TokenWhitelistedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `TradeDisputed` event"]
        pub fn trade_disputed_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, TradeDisputedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `TradeFilled` event"]
        pub fn trade_filled_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, TradeFilledFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `TradeRequested` event"]
        pub fn trade_requested_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, TradeRequestedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `TradeSettled` event"]
        pub fn trade_settled_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, TradeSettledFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `UpdatedFeeForTrade` event"]
        pub fn updated_fee_for_trade_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, UpdatedFeeForTradeFilter> {
            self.0.event()
        }
        #[doc = r" Returns an [`Event`](#ethers_contract::builders::Event) builder for all events of this contract"]
        pub fn events(&self) -> ethers::contract::builders::Event<M, BookSingleChainEvents> {
            self.0.event_with_filter(Default::default())
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>> for BookSingleChain<M> {
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
    #[ethevent(name = "MaxFeePctChanged", abi = "MaxFeePctChanged(uint256)")]
    pub struct MaxFeePctChangedFilter {
        pub new_max_fee_pct: ethers::core::types::U256,
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
    #[ethevent(
        name = "SafeBlockThresholdChanged",
        abi = "SafeBlockThresholdChanged(uint256)"
    )]
    pub struct SafeBlockThresholdChangedFilter {
        pub new_safe_block_threshold: ethers::core::types::U256,
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
        pub whitelisted: bool,
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
        name = "TradeDisputed",
        abi = "TradeDisputed(address,uint256,bytes32,uint256,uint256)"
    )]
    pub struct TradeDisputedFilter {
        #[ethevent(indexed)]
        pub relayer: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub trade_index: ethers::core::types::U256,
        #[ethevent(indexed)]
        pub dispute_id: [u8; 32],
        pub filled_amount: ethers::core::types::U256,
        pub fee_pct: ethers::core::types::U256,
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
        name = "TradeFilled",
        abi = "TradeFilled(address,uint256,uint256,uint256)"
    )]
    pub struct TradeFilledFilter {
        #[ethevent(indexed)]
        pub relayer: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub trade_index: ethers::core::types::U256,
        pub fee_pct: ethers::core::types::U256,
        pub amount_out: ethers::core::types::U256,
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
        name = "TradeRequested",
        abi = "TradeRequested(address,address,uint256,uint256,address,uint256)"
    )]
    pub struct TradeRequestedFilter {
        #[ethevent(indexed)]
        pub token_in: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub token_out: ethers::core::types::Address,
        pub amount_in: ethers::core::types::U256,
        pub fee_pct: ethers::core::types::U256,
        pub to: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub trade_index: ethers::core::types::U256,
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
        name = "TradeSettled",
        abi = "TradeSettled(address,uint256,uint256,uint256)"
    )]
    pub struct TradeSettledFilter {
        #[ethevent(indexed)]
        pub relayer: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub trade_index: ethers::core::types::U256,
        #[ethevent(indexed)]
        pub filled_amount: ethers::core::types::U256,
        pub fee_pct: ethers::core::types::U256,
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
        name = "UpdatedFeeForTrade",
        abi = "UpdatedFeeForTrade(address,uint256,uint256)"
    )]
    pub struct UpdatedFeeForTradeFilter {
        #[ethevent(indexed)]
        pub trader: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub trade_index: ethers::core::types::U256,
        pub new_fee_pct: ethers::core::types::U256,
    }
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum BookSingleChainEvents {
        MaxFeePctChangedFilter(MaxFeePctChangedFilter),
        OwnerUpdatedFilter(OwnerUpdatedFilter),
        SafeBlockThresholdChangedFilter(SafeBlockThresholdChangedFilter),
        TokenWhitelistedFilter(TokenWhitelistedFilter),
        TradeDisputedFilter(TradeDisputedFilter),
        TradeFilledFilter(TradeFilledFilter),
        TradeRequestedFilter(TradeRequestedFilter),
        TradeSettledFilter(TradeSettledFilter),
        UpdatedFeeForTradeFilter(UpdatedFeeForTradeFilter),
    }
    impl ethers::contract::EthLogDecode for BookSingleChainEvents {
        fn decode_log(log: &ethers::core::abi::RawLog) -> Result<Self, ethers::core::abi::Error>
        where
            Self: Sized,
        {
            if let Ok(decoded) = MaxFeePctChangedFilter::decode_log(log) {
                return Ok(BookSingleChainEvents::MaxFeePctChangedFilter(decoded));
            }
            if let Ok(decoded) = OwnerUpdatedFilter::decode_log(log) {
                return Ok(BookSingleChainEvents::OwnerUpdatedFilter(decoded));
            }
            if let Ok(decoded) = SafeBlockThresholdChangedFilter::decode_log(log) {
                return Ok(BookSingleChainEvents::SafeBlockThresholdChangedFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = TokenWhitelistedFilter::decode_log(log) {
                return Ok(BookSingleChainEvents::TokenWhitelistedFilter(decoded));
            }
            if let Ok(decoded) = TradeDisputedFilter::decode_log(log) {
                return Ok(BookSingleChainEvents::TradeDisputedFilter(decoded));
            }
            if let Ok(decoded) = TradeFilledFilter::decode_log(log) {
                return Ok(BookSingleChainEvents::TradeFilledFilter(decoded));
            }
            if let Ok(decoded) = TradeRequestedFilter::decode_log(log) {
                return Ok(BookSingleChainEvents::TradeRequestedFilter(decoded));
            }
            if let Ok(decoded) = TradeSettledFilter::decode_log(log) {
                return Ok(BookSingleChainEvents::TradeSettledFilter(decoded));
            }
            if let Ok(decoded) = UpdatedFeeForTradeFilter::decode_log(log) {
                return Ok(BookSingleChainEvents::UpdatedFeeForTradeFilter(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::std::fmt::Display for BookSingleChainEvents {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                BookSingleChainEvents::MaxFeePctChangedFilter(element) => element.fmt(f),
                BookSingleChainEvents::OwnerUpdatedFilter(element) => element.fmt(f),
                BookSingleChainEvents::SafeBlockThresholdChangedFilter(element) => element.fmt(f),
                BookSingleChainEvents::TokenWhitelistedFilter(element) => element.fmt(f),
                BookSingleChainEvents::TradeDisputedFilter(element) => element.fmt(f),
                BookSingleChainEvents::TradeFilledFilter(element) => element.fmt(f),
                BookSingleChainEvents::TradeRequestedFilter(element) => element.fmt(f),
                BookSingleChainEvents::TradeSettledFilter(element) => element.fmt(f),
                BookSingleChainEvents::UpdatedFeeForTradeFilter(element) => element.fmt(f),
            }
        }
    }
    #[doc = "Container type for all input parameters for the `disputeTrade` function with signature `disputeTrade(address,address,uint256,uint256,address,uint256)` and selector `[43, 12, 246, 83]`"]
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
        name = "disputeTrade",
        abi = "disputeTrade(address,address,uint256,uint256,address,uint256)"
    )]
    pub struct DisputeTradeCall {
        pub token_in: ethers::core::types::Address,
        pub token_out: ethers::core::types::Address,
        pub amount_in: ethers::core::types::U256,
        pub fee_pct: ethers::core::types::U256,
        pub recipient: ethers::core::types::Address,
        pub trade_index: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `fillTrade` function with signature `fillTrade(address,address,uint256,uint256,address,uint256,uint256)` and selector `[236, 76, 215, 219]`"]
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
        name = "fillTrade",
        abi = "fillTrade(address,address,uint256,uint256,address,uint256,uint256)"
    )]
    pub struct FillTradeCall {
        pub token_in: ethers::core::types::Address,
        pub token_out: ethers::core::types::Address,
        pub amount_in: ethers::core::types::U256,
        pub fee_pct: ethers::core::types::U256,
        pub recipient: ethers::core::types::Address,
        pub trade_index: ethers::core::types::U256,
        pub amount_to_send: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `fillTradeWithUpdatedFee` function with signature `fillTradeWithUpdatedFee(address,address,uint256,uint256,address,uint256,uint256,address,uint256,bytes)` and selector `[125, 157, 216, 93]`"]
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
        name = "fillTradeWithUpdatedFee",
        abi = "fillTradeWithUpdatedFee(address,address,uint256,uint256,address,uint256,uint256,address,uint256,bytes)"
    )]
    pub struct FillTradeWithUpdatedFeeCall {
        pub token_in: ethers::core::types::Address,
        pub token_out: ethers::core::types::Address,
        pub amount_in: ethers::core::types::U256,
        pub fee_pct: ethers::core::types::U256,
        pub recipient: ethers::core::types::Address,
        pub trade_index: ethers::core::types::U256,
        pub amount_to_send: ethers::core::types::U256,
        pub trader: ethers::core::types::Address,
        pub new_fee_pct: ethers::core::types::U256,
        pub trader_signature: ethers::core::types::Bytes,
    }
    #[doc = "Container type for all input parameters for the `filledAmount` function with signature `filledAmount(bytes32)` and selector `[11, 32, 183, 188]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "filledAmount", abi = "filledAmount(bytes32)")]
    pub struct FilledAmountCall(pub [u8; 32]);
    #[doc = "Container type for all input parameters for the `filledAtBlock` function with signature `filledAtBlock(bytes32)` and selector `[149, 1, 50, 95]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "filledAtBlock", abi = "filledAtBlock(bytes32)")]
    pub struct FilledAtBlockCall(pub [u8; 32]);
    #[doc = "Container type for all input parameters for the `filledBy` function with signature `filledBy(bytes32)` and selector `[215, 14, 61, 253]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "filledBy", abi = "filledBy(bytes32)")]
    pub struct FilledByCall(pub [u8; 32]);
    #[doc = "Container type for all input parameters for the `maxFeePct` function with signature `maxFeePct()` and selector `[252, 113, 28, 58]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "maxFeePct", abi = "maxFeePct()")]
    pub struct MaxFeePctCall;
    #[doc = "Container type for all input parameters for the `numberOfTrades` function with signature `numberOfTrades()` and selector `[205, 128, 93, 94]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "numberOfTrades", abi = "numberOfTrades()")]
    pub struct NumberOfTradesCall;
    #[doc = "Container type for all input parameters for the `oracle` function with signature `oracle()` and selector `[125, 192, 209, 208]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "oracle", abi = "oracle()")]
    pub struct OracleCall;
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
    #[doc = "Container type for all input parameters for the `requestTrade` function with signature `requestTrade(address,address,uint256,uint256,address)` and selector `[145, 112, 192, 91]`"]
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
        name = "requestTrade",
        abi = "requestTrade(address,address,uint256,uint256,address)"
    )]
    pub struct RequestTradeCall {
        pub token_in: ethers::core::types::Address,
        pub token_out: ethers::core::types::Address,
        pub amount_in: ethers::core::types::U256,
        pub fee_pct: ethers::core::types::U256,
        pub recipient: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `safeBlockThreshold` function with signature `safeBlockThreshold()` and selector `[15, 240, 192, 14]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "safeBlockThreshold", abi = "safeBlockThreshold()")]
    pub struct SafeBlockThresholdCall;
    #[doc = "Container type for all input parameters for the `setMaxFeePct` function with signature `setMaxFeePct(uint256)` and selector `[189, 32, 168, 89]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "setMaxFeePct", abi = "setMaxFeePct(uint256)")]
    pub struct SetMaxFeePctCall {
        pub new_max_fee_pct: ethers::core::types::U256,
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
    #[doc = "Container type for all input parameters for the `setSafeBlockThreshold` function with signature `setSafeBlockThreshold(uint256)` and selector `[38, 19, 243, 7]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "setSafeBlockThreshold", abi = "setSafeBlockThreshold(uint256)")]
    pub struct SetSafeBlockThresholdCall {
        pub new_safe_block_threshold: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `settleTrade` function with signature `settleTrade(address,address,uint256,uint256,address,uint256)` and selector `[224, 44, 2, 121]`"]
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
        name = "settleTrade",
        abi = "settleTrade(address,address,uint256,uint256,address,uint256)"
    )]
    pub struct SettleTradeCall {
        pub token_in: ethers::core::types::Address,
        pub token_out: ethers::core::types::Address,
        pub amount_in: ethers::core::types::U256,
        pub fee_pct: ethers::core::types::U256,
        pub recipient: ethers::core::types::Address,
        pub trade_index: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `updateFeeForTrade` function with signature `updateFeeForTrade(address,address,uint256,uint256,address,uint256,address,uint256,bytes)` and selector `[189, 74, 201, 151]`"]
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
        name = "updateFeeForTrade",
        abi = "updateFeeForTrade(address,address,uint256,uint256,address,uint256,address,uint256,bytes)"
    )]
    pub struct UpdateFeeForTradeCall {
        pub token_in: ethers::core::types::Address,
        pub token_out: ethers::core::types::Address,
        pub amount_in: ethers::core::types::U256,
        pub fee_pct: ethers::core::types::U256,
        pub recipient: ethers::core::types::Address,
        pub trade_index: ethers::core::types::U256,
        pub trader: ethers::core::types::Address,
        pub new_fee_pct: ethers::core::types::U256,
        pub trader_signature: ethers::core::types::Bytes,
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
        pub whitelisted: bool,
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
    pub enum BookSingleChainCalls {
        DisputeTrade(DisputeTradeCall),
        FillTrade(FillTradeCall),
        FillTradeWithUpdatedFee(FillTradeWithUpdatedFeeCall),
        FilledAmount(FilledAmountCall),
        FilledAtBlock(FilledAtBlockCall),
        FilledBy(FilledByCall),
        MaxFeePct(MaxFeePctCall),
        NumberOfTrades(NumberOfTradesCall),
        Oracle(OracleCall),
        Owner(OwnerCall),
        RequestTrade(RequestTradeCall),
        SafeBlockThreshold(SafeBlockThresholdCall),
        SetMaxFeePct(SetMaxFeePctCall),
        SetOwner(SetOwnerCall),
        SetSafeBlockThreshold(SetSafeBlockThresholdCall),
        SettleTrade(SettleTradeCall),
        UpdateFeeForTrade(UpdateFeeForTradeCall),
        WhitelistToken(WhitelistTokenCall),
        WhitelistedTokens(WhitelistedTokensCall),
    }
    impl ethers::core::abi::AbiDecode for BookSingleChainCalls {
        fn decode(data: impl AsRef<[u8]>) -> Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <DisputeTradeCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(BookSingleChainCalls::DisputeTrade(decoded));
            }
            if let Ok(decoded) =
                <FillTradeCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(BookSingleChainCalls::FillTrade(decoded));
            }
            if let Ok(decoded) =
                <FillTradeWithUpdatedFeeCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(BookSingleChainCalls::FillTradeWithUpdatedFee(decoded));
            }
            if let Ok(decoded) =
                <FilledAmountCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(BookSingleChainCalls::FilledAmount(decoded));
            }
            if let Ok(decoded) =
                <FilledAtBlockCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(BookSingleChainCalls::FilledAtBlock(decoded));
            }
            if let Ok(decoded) =
                <FilledByCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(BookSingleChainCalls::FilledBy(decoded));
            }
            if let Ok(decoded) =
                <MaxFeePctCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(BookSingleChainCalls::MaxFeePct(decoded));
            }
            if let Ok(decoded) =
                <NumberOfTradesCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(BookSingleChainCalls::NumberOfTrades(decoded));
            }
            if let Ok(decoded) = <OracleCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(BookSingleChainCalls::Oracle(decoded));
            }
            if let Ok(decoded) = <OwnerCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(BookSingleChainCalls::Owner(decoded));
            }
            if let Ok(decoded) =
                <RequestTradeCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(BookSingleChainCalls::RequestTrade(decoded));
            }
            if let Ok(decoded) =
                <SafeBlockThresholdCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(BookSingleChainCalls::SafeBlockThreshold(decoded));
            }
            if let Ok(decoded) =
                <SetMaxFeePctCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(BookSingleChainCalls::SetMaxFeePct(decoded));
            }
            if let Ok(decoded) =
                <SetOwnerCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(BookSingleChainCalls::SetOwner(decoded));
            }
            if let Ok(decoded) =
                <SetSafeBlockThresholdCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(BookSingleChainCalls::SetSafeBlockThreshold(decoded));
            }
            if let Ok(decoded) =
                <SettleTradeCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(BookSingleChainCalls::SettleTrade(decoded));
            }
            if let Ok(decoded) =
                <UpdateFeeForTradeCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(BookSingleChainCalls::UpdateFeeForTrade(decoded));
            }
            if let Ok(decoded) =
                <WhitelistTokenCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(BookSingleChainCalls::WhitelistToken(decoded));
            }
            if let Ok(decoded) =
                <WhitelistedTokensCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(BookSingleChainCalls::WhitelistedTokens(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for BookSingleChainCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                BookSingleChainCalls::DisputeTrade(element) => element.encode(),
                BookSingleChainCalls::FillTrade(element) => element.encode(),
                BookSingleChainCalls::FillTradeWithUpdatedFee(element) => element.encode(),
                BookSingleChainCalls::FilledAmount(element) => element.encode(),
                BookSingleChainCalls::FilledAtBlock(element) => element.encode(),
                BookSingleChainCalls::FilledBy(element) => element.encode(),
                BookSingleChainCalls::MaxFeePct(element) => element.encode(),
                BookSingleChainCalls::NumberOfTrades(element) => element.encode(),
                BookSingleChainCalls::Oracle(element) => element.encode(),
                BookSingleChainCalls::Owner(element) => element.encode(),
                BookSingleChainCalls::RequestTrade(element) => element.encode(),
                BookSingleChainCalls::SafeBlockThreshold(element) => element.encode(),
                BookSingleChainCalls::SetMaxFeePct(element) => element.encode(),
                BookSingleChainCalls::SetOwner(element) => element.encode(),
                BookSingleChainCalls::SetSafeBlockThreshold(element) => element.encode(),
                BookSingleChainCalls::SettleTrade(element) => element.encode(),
                BookSingleChainCalls::UpdateFeeForTrade(element) => element.encode(),
                BookSingleChainCalls::WhitelistToken(element) => element.encode(),
                BookSingleChainCalls::WhitelistedTokens(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for BookSingleChainCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                BookSingleChainCalls::DisputeTrade(element) => element.fmt(f),
                BookSingleChainCalls::FillTrade(element) => element.fmt(f),
                BookSingleChainCalls::FillTradeWithUpdatedFee(element) => element.fmt(f),
                BookSingleChainCalls::FilledAmount(element) => element.fmt(f),
                BookSingleChainCalls::FilledAtBlock(element) => element.fmt(f),
                BookSingleChainCalls::FilledBy(element) => element.fmt(f),
                BookSingleChainCalls::MaxFeePct(element) => element.fmt(f),
                BookSingleChainCalls::NumberOfTrades(element) => element.fmt(f),
                BookSingleChainCalls::Oracle(element) => element.fmt(f),
                BookSingleChainCalls::Owner(element) => element.fmt(f),
                BookSingleChainCalls::RequestTrade(element) => element.fmt(f),
                BookSingleChainCalls::SafeBlockThreshold(element) => element.fmt(f),
                BookSingleChainCalls::SetMaxFeePct(element) => element.fmt(f),
                BookSingleChainCalls::SetOwner(element) => element.fmt(f),
                BookSingleChainCalls::SetSafeBlockThreshold(element) => element.fmt(f),
                BookSingleChainCalls::SettleTrade(element) => element.fmt(f),
                BookSingleChainCalls::UpdateFeeForTrade(element) => element.fmt(f),
                BookSingleChainCalls::WhitelistToken(element) => element.fmt(f),
                BookSingleChainCalls::WhitelistedTokens(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<DisputeTradeCall> for BookSingleChainCalls {
        fn from(var: DisputeTradeCall) -> Self {
            BookSingleChainCalls::DisputeTrade(var)
        }
    }
    impl ::std::convert::From<FillTradeCall> for BookSingleChainCalls {
        fn from(var: FillTradeCall) -> Self {
            BookSingleChainCalls::FillTrade(var)
        }
    }
    impl ::std::convert::From<FillTradeWithUpdatedFeeCall> for BookSingleChainCalls {
        fn from(var: FillTradeWithUpdatedFeeCall) -> Self {
            BookSingleChainCalls::FillTradeWithUpdatedFee(var)
        }
    }
    impl ::std::convert::From<FilledAmountCall> for BookSingleChainCalls {
        fn from(var: FilledAmountCall) -> Self {
            BookSingleChainCalls::FilledAmount(var)
        }
    }
    impl ::std::convert::From<FilledAtBlockCall> for BookSingleChainCalls {
        fn from(var: FilledAtBlockCall) -> Self {
            BookSingleChainCalls::FilledAtBlock(var)
        }
    }
    impl ::std::convert::From<FilledByCall> for BookSingleChainCalls {
        fn from(var: FilledByCall) -> Self {
            BookSingleChainCalls::FilledBy(var)
        }
    }
    impl ::std::convert::From<MaxFeePctCall> for BookSingleChainCalls {
        fn from(var: MaxFeePctCall) -> Self {
            BookSingleChainCalls::MaxFeePct(var)
        }
    }
    impl ::std::convert::From<NumberOfTradesCall> for BookSingleChainCalls {
        fn from(var: NumberOfTradesCall) -> Self {
            BookSingleChainCalls::NumberOfTrades(var)
        }
    }
    impl ::std::convert::From<OracleCall> for BookSingleChainCalls {
        fn from(var: OracleCall) -> Self {
            BookSingleChainCalls::Oracle(var)
        }
    }
    impl ::std::convert::From<OwnerCall> for BookSingleChainCalls {
        fn from(var: OwnerCall) -> Self {
            BookSingleChainCalls::Owner(var)
        }
    }
    impl ::std::convert::From<RequestTradeCall> for BookSingleChainCalls {
        fn from(var: RequestTradeCall) -> Self {
            BookSingleChainCalls::RequestTrade(var)
        }
    }
    impl ::std::convert::From<SafeBlockThresholdCall> for BookSingleChainCalls {
        fn from(var: SafeBlockThresholdCall) -> Self {
            BookSingleChainCalls::SafeBlockThreshold(var)
        }
    }
    impl ::std::convert::From<SetMaxFeePctCall> for BookSingleChainCalls {
        fn from(var: SetMaxFeePctCall) -> Self {
            BookSingleChainCalls::SetMaxFeePct(var)
        }
    }
    impl ::std::convert::From<SetOwnerCall> for BookSingleChainCalls {
        fn from(var: SetOwnerCall) -> Self {
            BookSingleChainCalls::SetOwner(var)
        }
    }
    impl ::std::convert::From<SetSafeBlockThresholdCall> for BookSingleChainCalls {
        fn from(var: SetSafeBlockThresholdCall) -> Self {
            BookSingleChainCalls::SetSafeBlockThreshold(var)
        }
    }
    impl ::std::convert::From<SettleTradeCall> for BookSingleChainCalls {
        fn from(var: SettleTradeCall) -> Self {
            BookSingleChainCalls::SettleTrade(var)
        }
    }
    impl ::std::convert::From<UpdateFeeForTradeCall> for BookSingleChainCalls {
        fn from(var: UpdateFeeForTradeCall) -> Self {
            BookSingleChainCalls::UpdateFeeForTrade(var)
        }
    }
    impl ::std::convert::From<WhitelistTokenCall> for BookSingleChainCalls {
        fn from(var: WhitelistTokenCall) -> Self {
            BookSingleChainCalls::WhitelistToken(var)
        }
    }
    impl ::std::convert::From<WhitelistedTokensCall> for BookSingleChainCalls {
        fn from(var: WhitelistedTokensCall) -> Self {
            BookSingleChainCalls::WhitelistedTokens(var)
        }
    }
    #[doc = "Container type for all return fields from the `filledAmount` function with signature `filledAmount(bytes32)` and selector `[11, 32, 183, 188]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct FilledAmountReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `filledAtBlock` function with signature `filledAtBlock(bytes32)` and selector `[149, 1, 50, 95]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct FilledAtBlockReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `filledBy` function with signature `filledBy(bytes32)` and selector `[215, 14, 61, 253]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct FilledByReturn(pub ethers::core::types::Address);
    #[doc = "Container type for all return fields from the `maxFeePct` function with signature `maxFeePct()` and selector `[252, 113, 28, 58]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct MaxFeePctReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `numberOfTrades` function with signature `numberOfTrades()` and selector `[205, 128, 93, 94]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct NumberOfTradesReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `oracle` function with signature `oracle()` and selector `[125, 192, 209, 208]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct OracleReturn(pub ethers::core::types::Address);
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
    #[doc = "Container type for all return fields from the `safeBlockThreshold` function with signature `safeBlockThreshold()` and selector `[15, 240, 192, 14]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct SafeBlockThresholdReturn(pub ethers::core::types::U256);
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
