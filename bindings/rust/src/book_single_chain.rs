pub use booksinglechain_mod::*;
#[allow(clippy::too_many_arguments, non_camel_case_types)]
pub mod booksinglechain_mod {
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
            serde_json :: from_str ("[{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_safeBlockThreshold\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"_oracleAddress\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"constructor\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"blocksLeft\",\"type\":\"uint256\",\"components\":[]}],\"type\":\"error\",\"name\":\"BookSingleChain__DisputePeriodNotOver\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"BookSingleChain__DisputePeriodOver\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"fee\",\"type\":\"uint256\",\"components\":[]}],\"type\":\"error\",\"name\":\"BookSingleChain__FeePctTooHigh\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"BookSingleChain__InvalidSignature\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\",\"components\":[]}],\"type\":\"error\",\"name\":\"BookSingleChain__InvalidToken\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"BookSingleChain__NewFeePctTooHigh\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"BookSingleChain__SameToken\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"BookSingleChain__SentToBlackHole\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"tradeIndex\",\"type\":\"uint256\",\"components\":[]}],\"type\":\"error\",\"name\":\"BookSingleChain__TradeAlreadyFilled\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"tradeIndex\",\"type\":\"uint256\",\"components\":[]}],\"type\":\"error\",\"name\":\"BookSingleChain__TradeNotFilled\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\",\"components\":[]}],\"type\":\"error\",\"name\":\"BookSingleChain__UnsafeTokenToWhitelist\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"BookSingleChain__ZeroAmount\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"newMaxFeePct\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"MaxFeePctChanged\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"user\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"newOwner\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"OwnerUpdated\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"newSafeBlockThreshold\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"SafeBlockThresholdChanged\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"bool\",\"name\":\"whitelisted\",\"type\":\"bool\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"TokenWhitelisted\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"relayer\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"tradeIndex\",\"type\":\"uint256\",\"components\":[],\"indexed\":true},{\"internalType\":\"bytes32\",\"name\":\"disputeId\",\"type\":\"bytes32\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"filledAmount\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"feePct\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"TradeDisputed\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"relayer\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"tradeIndex\",\"type\":\"uint256\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"filledAtBlock\",\"type\":\"uint256\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"feePct\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"amountOut\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"TradeFilled\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"tokenIn\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"tokenOut\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"amountIn\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"feePct\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"tradeIndex\",\"type\":\"uint256\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"TradeRequested\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"relayer\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"tradeIndex\",\"type\":\"uint256\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"filledAmount\",\"type\":\"uint256\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"feePct\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"TradeSettled\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"trader\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"tradeIndex\",\"type\":\"uint256\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"newFeePct\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"UpdatedFeeForTrade\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"tokenOut\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"feePct\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"tradeIndex\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"disputeTrade\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"tokenOut\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"feePct\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"tradeIndex\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amountToSend\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"fillTrade\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"tokenOut\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"tradeIndex\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amountToSend\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"trader\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"newFeePct\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"traderSignature\",\"type\":\"bytes\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"fillTradeWithUpdatedFee\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"filledTrades\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"relayer\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amountOut\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"blockHeight\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"maxFeePct\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"numberOfTrades\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"oracle\",\"outputs\":[{\"internalType\":\"contract IOracle\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"owner\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"tokenIn\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"tokenOut\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amountIn\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"feePct\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"requestTrade\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"safeBlockThreshold\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"newMaxFeePct\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setMaxFeePct\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"newOwner\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setOwner\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"newSafeBlockThreshold\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setSafeBlockThreshold\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"tokenIn\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"tokenOut\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amountIn\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"feePct\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"tradeIndex\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"settleTrade\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"trader\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"newFeePct\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"tradeIndex\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"traderSignature\",\"type\":\"bytes\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"updateFeeForTrade\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bool\",\"name\":\"whitelisted\",\"type\":\"bool\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"whitelistToken\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"whitelistedTokens\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]}]") . expect ("invalid abi")
        });
    #[doc = r" Bytecode of the #name contract"]
    pub static BOOKSINGLECHAIN_BYTECODE: ethers::contract::Lazy<ethers::core::types::Bytes> =
        ethers::contract::Lazy::new(|| {
            "0x60a060405260018055600060025534801561001957600080fd5b5060405161195d38038061195d83398101604081905261003891610106565b600080546001600160a01b031916339081178255604051909182917f8292fce18fa69edf4db7b94ea2e58241df0ae57f97e0a6c9b29067028bf92d76908290a3506001600160a01b03811660805260038290556040518281527fcf29a5174acb8c175d760a7381ffc52c6ae644e3a4ba3fa7e01344f959cd76159060200160405180910390a16703782dace9d9000060048190556040519081527f841095ec206e4a3d8124f54a431661bd653b296066d7d695baaa9178e9d21bb49060200160405180910390a15050610143565b6000806040838503121561011957600080fd5b825160208401519092506001600160a01b038116811461013857600080fd5b809150509250929050565b6080516117e361017a6000396000818161021601528181610425015281816104ef01528181610548015261060201526117e36000f3fe608060405234801561001057600080fd5b506004361061010b5760003560e01c80635881266d116100a2578063bd20a85911610071578063bd20a85914610276578063cd805d5e14610289578063daf9c21014610292578063e02c0279146102c5578063fc711c3a146102d857600080fd5b80635881266d146101fe5780637dc0d1d0146102115780638da5cb5b146102505780639170c05b1461026357600080fd5b806313af4035116100de57806313af4035146101675780632613f3071461017a57806329d56bc01461018d5780632d4d9a63146101a057600080fd5b806308ef3d44146101105780630ff0c00e146101255780630ffb1d8b1461014157806310f40e6a14610154575b600080fd5b61012361011e36600461140f565b6102e1565b005b61012e60035481565b6040519081526020015b60405180910390f35b61012361014f366004611450565b6105b1565b6101236101623660046114c9565b6106f7565b61012361017536600461154b565b610757565b61012361018836600461156d565b6107cc565b61012361019b366004611586565b610832565b6101d96101ae36600461156d565b6006602052600090815260409020805460018201546002909201546001600160a01b03909116919083565b604080516001600160a01b039094168452602084019290925290820152606001610138565b61012361020c3660046115bf565b6108b5565b6102387f000000000000000000000000000000000000000000000000000000000000000081565b6040516001600160a01b039091168152602001610138565b600054610238906001600160a01b031681565b610123610271366004611627565b61095d565b61012361028436600461156d565b610b0a565b61012e60025481565b6102b56102a036600461154b565b60056020526000908152604090205460ff1681565b6040519015158152602001610138565b6101236102d336600461167e565b610b91565b61012e60045481565b6001546001146103255760405162461bcd60e51b815260206004820152600a6024820152695245454e5452414e435960b01b60448201526064015b60405180910390fd5b600260015561033381610cf5565b61035357604051631acc120960e11b81526004810182905260240161031c565b600081815260066020908152604091829020825160608101845281546001600160a01b0316815260018201549281019290925260020154918101829052600354909161039f90436116f3565b106103bd57604051632c02744560e11b815260040160405180910390fd5b602081810151825160008581526006909352604080842080546001600160a01b031916815560018101859055600201849055516384bfabcf60e01b81526001600160a01b038083166004830152336024830152888116604483015260648201849052929391927f000000000000000000000000000000000000000000000000000000000000000016906384bfabcf90608401602060405180830381865afa15801561046c573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190610490919061170a565b90508085836001600160a01b03167fc293ada1a20d5f6111738a64aec9c7262db402de1650b576e691e321964bb01e868a6040516104d8929190918252602082015260400190565b60405180910390a46105146001600160a01b0388167f000000000000000000000000000000000000000000000000000000000000000085610d36565b604051632bd6267f60e21b81526001600160a01b0383811660048301523360248301528881166044830152606482018590527f0000000000000000000000000000000000000000000000000000000000000000169063af5899fc90608401600060405180830381600087803b15801561058c57600080fd5b505af11580156105a0573d6000803e3d6000fd5b505060018055505050505050505050565b6000546001600160a01b031633146105db5760405162461bcd60e51b815260040161031c90611723565b80801561066f5750604051630daf9c2160e41b81526001600160a01b0383811660048301527f0000000000000000000000000000000000000000000000000000000000000000169063daf9c21090602401602060405180830381865afa158015610649573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061066d9190611749565b155b15610698576040516313c42eef60e21b81526001600160a01b038316600482015260240161031c565b6001600160a01b038216600081815260056020908152604091829020805460ff191685151590811790915591519182527fef81a9943b96c8df4ef243401c9bf5159146166211356898b52d382086168d92910160405180910390a25050565b6107048487858585610db3565b61070f878787610f1a565b60408051848152602081018790524391889133917f970202b118cfe4d540f3c2eac57995322281324b625e43838fb87e30e0753b9e910160405180910390a450505050505050565b6000546001600160a01b031633146107815760405162461bcd60e51b815260040161031c90611723565b600080546001600160a01b0319166001600160a01b0383169081178255604051909133917f8292fce18fa69edf4db7b94ea2e58241df0ae57f97e0a6c9b29067028bf92d769190a350565b6000546001600160a01b031633146107f65760405162461bcd60e51b815260040161031c90611723565b60038190556040518181527fcf29a5174acb8c175d760a7381ffc52c6ae644e3a4ba3fa7e01344f959cd7615906020015b60405180910390a150565b6000828152600660205260409020600201541561086557604051638b6bbbad60e01b81526004810183905260240161031c565b610870848383610f1a565b60408051848152602081018390524391849133917f970202b118cfe4d540f3c2eac57995322281324b625e43838fb87e30e0753b9e910160405180910390a450505050565b6004548411156108db576040516305bf279d60e41b81526004810185905260240161031c565b6108e483610cf5565b1561090557604051638b6bbbad60e01b81526004810184905260240161031c565b6109128584868585610db3565b82856001600160a01b03167f6ab91dbc42f726b630639350395426be048c50255f12e82d28e2dffac41745938660405161094e91815260200190565b60405180910390a35050505050565b6001600160a01b03851660009081526005602052604090205460ff166109a15760405163f602627d60e01b81526001600160a01b038616600482015260240161031c565b6001600160a01b03841660009081526005602052604090205460ff166109e55760405163f602627d60e01b81526001600160a01b038516600482015260240161031c565b836001600160a01b0316856001600160a01b031603610a1757604051631d2792fb60e31b815260040160405180910390fd5b600454821115610a3d576040516305bf279d60e41b81526004810183905260240161031c565b82600003610a5e5760405163abc5ee6f60e01b815260040160405180910390fd5b6001600160a01b038116610a8557604051631feef77d60e01b815260040160405180910390fd5b610a9a6001600160a01b038616333086610f80565b60025460408051858152602081018590526001600160a01b03848116828401529151878316928916917f7361c265d28ece9d5df249995186533440e0b7a1310ae54d496fa1783056e3da919081900360600190a460028054906000610afe83611766565b91905055505050505050565b6000546001600160a01b03163314610b345760405162461bcd60e51b815260040161031c90611723565b670de0b6b3a76400008110610b5c57604051636a143fdd60e11b815260040160405180910390fd5b60048190556040518181527f841095ec206e4a3d8124f54a431661bd653b296066d7d695baaa9178e9d21bb490602001610827565b610b9a81610cf5565b610bba57604051631acc120960e11b81526004810182905260240161031c565b600081815260066020908152604091829020825160608101845281546001600160a01b03168152600182015492810192909252600201549181018290526003549091610c0690436116f3565b1015610c4a576000816040015143610c1e91906116f3565b600354610c2b91906116f3565b9050806040516325797e0360e11b815260040161031c91815260200190565b6020808201518251600085815260069093526040832080546001600160a01b0319168155600181018490556002019290925590610c916001600160a01b038916868461100a565b610ca56001600160a01b038a16828961100a565b8184826001600160a01b03167f3281f74a3f7405b6bd35e9687b3fcaaf242c466ac789d117f22b62b140af8dcc89604051610ce291815260200190565b60405180910390a4505050505050505050565b600090815260066020908152604091829020825160608101845281546001600160a01b03168152600182015492810192909252600201549101819052151590565b600060405163095ea7b360e01b8152836004820152826024820152602060006044836000895af13d15601f3d1160016000511416171691505080610dad5760405162461bcd60e51b815260206004820152600e60248201526d1054141493d59157d1905253115160921b604482015260640161031c565b50505050565b600454831115610dd9576040516305bf279d60e41b81526004810184905260240161031c565b610de284610cf5565b15610e0357604051638b6bbbad60e01b81526004810185905260240161031c565b604080517f0efb9dda140a951df4393d44ca40349032d31811466afd20eacd4b4136c3f4986020808301919091528183018790526060808301879052835180840390910181526080830184528051908201207f19457468657265756d205369676e6564204d6573736167653a0a33320000000060a084015260bc8084018290528451808503909101815260dc90930190935281519101206000610edc8286868080601f01602080910402602001604051908101604052809392919081815260200183838082843760009201919091525061108292505050565b9050876001600160a01b0316816001600160a01b031614610f10576040516324a0dbd760e21b815260040160405180910390fd5b5050505050505050565b60408051606081018252338082526020808301858152438486019081526000888152600690935294909120835181546001600160a01b0319166001600160a01b039182161782559151600182015593516002909401939093559091610dad919086169030855b60006040516323b872dd60e01b81528460048201528360248201528260448201526020600060648360008a5af13d15601f3d11600160005114161716915050806110035760405162461bcd60e51b81526020600482015260146024820152731514905394d1915497d19493d357d1905253115160621b604482015260640161031c565b5050505050565b600060405163a9059cbb60e01b8152836004820152826024820152602060006044836000895af13d15601f3d1160016000511416171691505080610dad5760405162461bcd60e51b815260206004820152600f60248201526e1514905394d1915497d19052531151608a1b604482015260640161031c565b600080600061109185856110a6565b9150915061109e81611114565b509392505050565b60008082516041036110dc5760208301516040840151606085015160001a6110d0878285856112cd565b9450945050505061110d565b825160400361110557602083015160408401516110fa8683836113ba565b93509350505061110d565b506000905060025b9250929050565b60008160048111156111285761112861177f565b036111305750565b60018160048111156111445761114461177f565b036111915760405162461bcd60e51b815260206004820152601860248201527f45434453413a20696e76616c6964207369676e61747572650000000000000000604482015260640161031c565b60028160048111156111a5576111a561177f565b036111f25760405162461bcd60e51b815260206004820152601f60248201527f45434453413a20696e76616c6964207369676e6174757265206c656e67746800604482015260640161031c565b60038160048111156112065761120661177f565b0361125e5760405162461bcd60e51b815260206004820152602260248201527f45434453413a20696e76616c6964207369676e6174757265202773272076616c604482015261756560f01b606482015260840161031c565b60048160048111156112725761127261177f565b036112ca5760405162461bcd60e51b815260206004820152602260248201527f45434453413a20696e76616c6964207369676e6174757265202776272076616c604482015261756560f01b606482015260840161031c565b50565b6000807f7fffffffffffffffffffffffffffffff5d576e7357a4501ddfe92f46681b20a083111561130457506000905060036113b1565b8460ff16601b1415801561131c57508460ff16601c14155b1561132d57506000905060046113b1565b6040805160008082526020820180845289905260ff881692820192909252606081018690526080810185905260019060a0016020604051602081039080840390855afa158015611381573d6000803e3d6000fd5b5050604051601f1901519150506001600160a01b0381166113aa576000600192509250506113b1565b9150600090505b94509492505050565b6000806001600160ff1b038316816113d760ff86901c601b611795565b90506113e5878288856112cd565b935093505050935093915050565b80356001600160a01b038116811461140a57600080fd5b919050565b60008060006060848603121561142457600080fd5b61142d846113f3565b95602085013595506040909401359392505050565b80151581146112ca57600080fd5b6000806040838503121561146357600080fd5b61146c836113f3565b9150602083013561147c81611442565b809150509250929050565b60008083601f84011261149957600080fd5b50813567ffffffffffffffff8111156114b157600080fd5b60208301915083602082850101111561110d57600080fd5b600080600080600080600060c0888a0312156114e457600080fd5b6114ed886113f3565b96506020880135955060408801359450611509606089016113f3565b93506080880135925060a088013567ffffffffffffffff81111561152c57600080fd5b6115388a828b01611487565b989b979a50959850939692959293505050565b60006020828403121561155d57600080fd5b611566826113f3565b9392505050565b60006020828403121561157f57600080fd5b5035919050565b6000806000806080858703121561159c57600080fd5b6115a5856113f3565b966020860135965060408601359560600135945092505050565b6000806000806000608086880312156115d757600080fd5b6115e0866113f3565b94506020860135935060408601359250606086013567ffffffffffffffff81111561160a57600080fd5b61161688828901611487565b969995985093965092949392505050565b600080600080600060a0868803121561163f57600080fd5b611648866113f3565b9450611656602087016113f3565b93506040860135925060608601359150611672608087016113f3565b90509295509295909350565b60008060008060008060c0878903121561169757600080fd5b6116a0876113f3565b95506116ae602088016113f3565b945060408701359350606087013592506116ca608088016113f3565b915060a087013590509295509295509295565b634e487b7160e01b600052601160045260246000fd5b600082821015611705576117056116dd565b500390565b60006020828403121561171c57600080fd5b5051919050565b6020808252600c908201526b15539055551213d49256915160a21b604082015260600190565b60006020828403121561175b57600080fd5b815161156681611442565b600060018201611778576117786116dd565b5060010190565b634e487b7160e01b600052602160045260246000fd5b600082198211156117a8576117a86116dd565b50019056fea2646970667358221220b41ca7110c27843c888fd86833e8f4658e46a22feba9517f56d8855f80a64b5264736f6c634300080f0033" . parse () . expect ("invalid bytecode")
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
        #[doc = "Calls the contract's `disputeTrade` (0x08ef3d44) function"]
        pub fn dispute_trade(
            &self,
            token_out: ethers::core::types::Address,
            fee_pct: ethers::core::types::U256,
            trade_index: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([8, 239, 61, 68], (token_out, fee_pct, trade_index))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `fillTrade` (0x29d56bc0) function"]
        pub fn fill_trade(
            &self,
            token_out: ethers::core::types::Address,
            fee_pct: ethers::core::types::U256,
            trade_index: ethers::core::types::U256,
            amount_to_send: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [41, 213, 107, 192],
                    (token_out, fee_pct, trade_index, amount_to_send),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `fillTradeWithUpdatedFee` (0x10f40e6a) function"]
        pub fn fill_trade_with_updated_fee(
            &self,
            token_out: ethers::core::types::Address,
            trade_index: ethers::core::types::U256,
            amount_to_send: ethers::core::types::U256,
            trader: ethers::core::types::Address,
            new_fee_pct: ethers::core::types::U256,
            trader_signature: ethers::core::types::Bytes,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [16, 244, 14, 106],
                    (
                        token_out,
                        trade_index,
                        amount_to_send,
                        trader,
                        new_fee_pct,
                        trader_signature,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `filledTrades` (0x2d4d9a63) function"]
        pub fn filled_trades(
            &self,
            p0: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<
            M,
            (
                ethers::core::types::Address,
                ethers::core::types::U256,
                ethers::core::types::U256,
            ),
        > {
            self.0
                .method_hash([45, 77, 154, 99], p0)
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
            to: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [145, 112, 192, 91],
                    (token_in, token_out, amount_in, fee_pct, to),
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
            to: ethers::core::types::Address,
            trade_index: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [224, 44, 2, 121],
                    (token_in, token_out, amount_in, fee_pct, to, trade_index),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `updateFeeForTrade` (0x5881266d) function"]
        pub fn update_fee_for_trade(
            &self,
            trader: ethers::core::types::Address,
            new_fee_pct: ethers::core::types::U256,
            trade_index: ethers::core::types::U256,
            trader_signature: ethers::core::types::Bytes,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [88, 129, 38, 109],
                    (trader, new_fee_pct, trade_index, trader_signature),
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
        abi = "TradeFilled(address,uint256,uint256,uint256,uint256)"
    )]
    pub struct TradeFilledFilter {
        #[ethevent(indexed)]
        pub relayer: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub trade_index: ethers::core::types::U256,
        #[ethevent(indexed)]
        pub filled_at_block: ethers::core::types::U256,
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
    #[doc = "Container type for all input parameters for the `disputeTrade`function with signature `disputeTrade(address,uint256,uint256)` and selector `[8, 239, 61, 68]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "disputeTrade", abi = "disputeTrade(address,uint256,uint256)")]
    pub struct DisputeTradeCall {
        pub token_out: ethers::core::types::Address,
        pub fee_pct: ethers::core::types::U256,
        pub trade_index: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `fillTrade`function with signature `fillTrade(address,uint256,uint256,uint256)` and selector `[41, 213, 107, 192]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "fillTrade", abi = "fillTrade(address,uint256,uint256,uint256)")]
    pub struct FillTradeCall {
        pub token_out: ethers::core::types::Address,
        pub fee_pct: ethers::core::types::U256,
        pub trade_index: ethers::core::types::U256,
        pub amount_to_send: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `fillTradeWithUpdatedFee`function with signature `fillTradeWithUpdatedFee(address,uint256,uint256,address,uint256,bytes)` and selector `[16, 244, 14, 106]`"]
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
        abi = "fillTradeWithUpdatedFee(address,uint256,uint256,address,uint256,bytes)"
    )]
    pub struct FillTradeWithUpdatedFeeCall {
        pub token_out: ethers::core::types::Address,
        pub trade_index: ethers::core::types::U256,
        pub amount_to_send: ethers::core::types::U256,
        pub trader: ethers::core::types::Address,
        pub new_fee_pct: ethers::core::types::U256,
        pub trader_signature: ethers::core::types::Bytes,
    }
    #[doc = "Container type for all input parameters for the `filledTrades`function with signature `filledTrades(uint256)` and selector `[45, 77, 154, 99]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "filledTrades", abi = "filledTrades(uint256)")]
    pub struct FilledTradesCall(pub ethers::core::types::U256);
    #[doc = "Container type for all input parameters for the `maxFeePct`function with signature `maxFeePct()` and selector `[252, 113, 28, 58]`"]
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
    #[doc = "Container type for all input parameters for the `numberOfTrades`function with signature `numberOfTrades()` and selector `[205, 128, 93, 94]`"]
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
    #[doc = "Container type for all input parameters for the `oracle`function with signature `oracle()` and selector `[125, 192, 209, 208]`"]
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
    #[doc = "Container type for all input parameters for the `requestTrade`function with signature `requestTrade(address,address,uint256,uint256,address)` and selector `[145, 112, 192, 91]`"]
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
        pub to: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `safeBlockThreshold`function with signature `safeBlockThreshold()` and selector `[15, 240, 192, 14]`"]
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
    #[doc = "Container type for all input parameters for the `setMaxFeePct`function with signature `setMaxFeePct(uint256)` and selector `[189, 32, 168, 89]`"]
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
    #[doc = "Container type for all input parameters for the `setSafeBlockThreshold`function with signature `setSafeBlockThreshold(uint256)` and selector `[38, 19, 243, 7]`"]
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
    #[doc = "Container type for all input parameters for the `settleTrade`function with signature `settleTrade(address,address,uint256,uint256,address,uint256)` and selector `[224, 44, 2, 121]`"]
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
        pub to: ethers::core::types::Address,
        pub trade_index: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `updateFeeForTrade`function with signature `updateFeeForTrade(address,uint256,uint256,bytes)` and selector `[88, 129, 38, 109]`"]
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
        abi = "updateFeeForTrade(address,uint256,uint256,bytes)"
    )]
    pub struct UpdateFeeForTradeCall {
        pub trader: ethers::core::types::Address,
        pub new_fee_pct: ethers::core::types::U256,
        pub trade_index: ethers::core::types::U256,
        pub trader_signature: ethers::core::types::Bytes,
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
        pub whitelisted: bool,
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
    pub enum BookSingleChainCalls {
        DisputeTrade(DisputeTradeCall),
        FillTrade(FillTradeCall),
        FillTradeWithUpdatedFee(FillTradeWithUpdatedFeeCall),
        FilledTrades(FilledTradesCall),
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
                <FilledTradesCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(BookSingleChainCalls::FilledTrades(decoded));
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
                BookSingleChainCalls::FilledTrades(element) => element.encode(),
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
                BookSingleChainCalls::FilledTrades(element) => element.fmt(f),
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
    impl ::std::convert::From<FilledTradesCall> for BookSingleChainCalls {
        fn from(var: FilledTradesCall) -> Self {
            BookSingleChainCalls::FilledTrades(var)
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
}
