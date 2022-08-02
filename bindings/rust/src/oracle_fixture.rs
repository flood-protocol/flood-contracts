pub use oracle_fixture::*;
#[allow(clippy::too_many_arguments, non_camel_case_types)]
pub mod oracle_fixture {
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
    #[doc = "OracleFixture was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static ORACLEFIXTURE_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            ethers :: core :: utils :: __serde_json :: from_str ("[{\"inputs\":[{\"internalType\":\"string\",\"name\":\"\",\"type\":\"string\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"log\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"log_address\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint256[]\",\"name\":\"val\",\"type\":\"uint256[]\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"log_array\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"int256[]\",\"name\":\"val\",\"type\":\"int256[]\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"log_array\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address[]\",\"name\":\"val\",\"type\":\"address[]\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"log_array\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"bytes\",\"name\":\"\",\"type\":\"bytes\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"log_bytes\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"log_bytes32\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"int256\",\"name\":\"\",\"type\":\"int256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"log_int\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"key\",\"type\":\"string\",\"components\":[],\"indexed\":false},{\"internalType\":\"address\",\"name\":\"val\",\"type\":\"address\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"log_named_address\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"key\",\"type\":\"string\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256[]\",\"name\":\"val\",\"type\":\"uint256[]\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"log_named_array\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"key\",\"type\":\"string\",\"components\":[],\"indexed\":false},{\"internalType\":\"int256[]\",\"name\":\"val\",\"type\":\"int256[]\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"log_named_array\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"key\",\"type\":\"string\",\"components\":[],\"indexed\":false},{\"internalType\":\"address[]\",\"name\":\"val\",\"type\":\"address[]\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"log_named_array\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"key\",\"type\":\"string\",\"components\":[],\"indexed\":false},{\"internalType\":\"bytes\",\"name\":\"val\",\"type\":\"bytes\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"log_named_bytes\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"key\",\"type\":\"string\",\"components\":[],\"indexed\":false},{\"internalType\":\"bytes32\",\"name\":\"val\",\"type\":\"bytes32\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"log_named_bytes32\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"key\",\"type\":\"string\",\"components\":[],\"indexed\":false},{\"internalType\":\"int256\",\"name\":\"val\",\"type\":\"int256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"decimals\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"log_named_decimal_int\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"key\",\"type\":\"string\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"val\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"decimals\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"log_named_decimal_uint\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"key\",\"type\":\"string\",\"components\":[],\"indexed\":false},{\"internalType\":\"int256\",\"name\":\"val\",\"type\":\"int256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"log_named_int\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"key\",\"type\":\"string\",\"components\":[],\"indexed\":false},{\"internalType\":\"string\",\"name\":\"val\",\"type\":\"string\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"log_named_string\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"key\",\"type\":\"string\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"val\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"log_named_uint\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"\",\"type\":\"string\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"log_string\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"log_uint\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"bytes\",\"name\":\"\",\"type\":\"bytes\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"logs\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"IS_SCRIPT\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"IS_TEST\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"failed\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setUp\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"vm\",\"outputs\":[{\"internalType\":\"contract Vm\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]}]") . expect ("invalid abi")
        });
    #[doc = r" Bytecode of the #name contract"]
    pub static ORACLEFIXTURE_BYTECODE: ethers::contract::Lazy<ethers::core::types::Bytes> =
        ethers::contract::Lazy::new(|| {
            "0x608060408190526000805462ff00ff1916620100011790556001625e79b760e01b031990527f81376b9868b292a46a1c486d344e427a3088657fda629b5f4a647822d329cd6a608452737109709ecfa91a80626ff3989d68f67f5b1dd12d63ffa1864960a46020604051808303816000875af115801562000084573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190620000aa91906200059b565b600880546001600160a01b0319166001600160a01b03929092169190911790556040516001625e79b760e01b031981527f28cac318a86c8a0a6a9156c2dba2c8c2363677ba0514ef616592d81557e679b66004820152737109709ecfa91a80626ff3989d68f67f5b1dd12d9063ffa18649906024016020604051808303816000875af11580156200013f573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906200016591906200059b565b600980546001600160a01b0319166001600160a01b03929092169190911790556040516001625e79b760e01b031981527f9791f723bf26b668c8cac0664b66d058cdb2a424cfb08c05dd9da570be0eff6f6004820152737109709ecfa91a80626ff3989d68f67f5b1dd12d9063ffa18649906024016020604051808303816000875af1158015620001fa573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906200022091906200059b565b600a60006101000a8154816001600160a01b0302191690836001600160a01b03160217905550620002bf73a0b86991c6218b36c1d19d4a2e9eb0ce3606eb486040518060400160405280601a81526020017f546f6b656e466978747572652e736f6c3a4d6f636b546f6b656e0000000000008152506006604051602001620002a99190620005c6565b60408051601f19818403018152919052620003c4565b600b60006101000a8154816001600160a01b0302191690836001600160a01b031602179055506200039073c02aaa39b223fe8d0a0e5c4f27ead9083c756cc26040518060400160405280601a81526020017f546f6b656e466978747572652e736f6c3a4d6f636b546f6b656e0000000000008152506012604051602001620002a991906060808252600d908201526c2bb930b83832b21022ba3432b960991b608082015260a060208201819052600490820152630ae8aa8960e31b60c082015260ff91909116604082015260e00190565b600c80546001600160a01b0319166001600160a01b0392909216919091179055348015620003bd57600080fd5b50620007a3565b60006001600160a01b0384163b8015620003e2578491505062000460565b6000620003f0858562000467565b9050806001600160a01b03166395d89b416040518163ffffffff1660e01b8152600401600060405180830381865afa15801562000431573d6000803e3d6000fd5b505050506040513d6000823e601f3d908101601f191682016040526200045b9190810190620006e6565b509150505b9392505050565b604051638d1cc92560e01b81526000908190737109709ecfa91a80626ff3989d68f67f5b1dd12d90638d1cc92590620004a59087906004016200073b565b6000604051808303816000875af1158015620004c5573d6000803e3d6000fd5b505050506040513d6000823e601f3d908101601f19168201604052620004ef9190810190620006e6565b836040516020016200050392919062000770565b60405160208183030381529060405290508051602082016000f091506001600160a01b038216620005945760405162461bcd60e51b815260206004820152603160248201527f54657374206465706c6f79436f646528737472696e672c6279746573293a204460448201527032b83637bcb6b2b73a103330b4b632b21760791b606482015260840160405180910390fd5b5092915050565b600060208284031215620005ae57600080fd5b81516001600160a01b03811681146200046057600080fd5b606081526000620005eb6060830160048152635553444360e01b602082015260400190565b82810360208401526200060f8160048152635553444360e01b602082015260400190565b91505060ff8316604083015292915050565b634e487b7160e01b600052604160045260246000fd5b60005b83811015620006545781810151838201526020016200063a565b8381111562000664576000848401525b50505050565b60006001600160401b038084111562000687576200068762000621565b604051601f8501601f19908116603f01168101908282118183101715620006b257620006b262000621565b81604052809350858152868686011115620006cc57600080fd5b620006dc86602083018762000637565b5050509392505050565b600060208284031215620006f957600080fd5b81516001600160401b038111156200071057600080fd5b8201601f810184136200072257600080fd5b62000733848251602084016200066a565b949350505050565b60208152600082518060208401526200075c81604085016020870162000637565b601f01601f19169190910160400192915050565b600083516200078481846020880162000637565b8351908301906200079a81836020880162000637565b01949350505050565b611f3680620007b36000396000f3fe608060405234801561001057600080fd5b50600436106100575760003560e01c80630a9254e41461005c5780633a76846314610066578063ba414fa614610098578063f8ccbf47146100b0578063fa7626d4146100c3575b600080fd5b6100646100d0565b005b61007b600080516020611ee183398151915281565b6040516001600160a01b0390911681526020015b60405180910390f35b6100a06109e1565b604051901515815260200161008f565b6000546100a09062010000900460ff1681565b6000546100a09060ff1681565b6100d8610b00565b6040516100e490610cdf565b604051809103906000f080158015610100573d6000803e3d6000fd5b50600d80546001600160a01b0319166001600160a01b03928316908117909155600b5460405163095ea7b360e01b8152600481019290925260001960248301529091169063095ea7b3906044016020604051808303816000875af115801561016c573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906101909190610cec565b50600c54600d5460405163095ea7b360e01b81526001600160a01b039182166004820152600019602482015291169063095ea7b3906044016020604051808303816000875af11580156101e7573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061020b9190610cec565b50600d54604080516318caf8e360e31b81526001600160a01b0390921660048301526024820152601060448201526f416c6c4b6e6f77696e674f7261636c6560801b6064820152600080516020611ee18339815191529063c657c71890608401600060405180830381600087803b15801561028557600080fd5b505af1158015610299573d6000803e3d6000fd5b50506008546040516303223eab60e11b81526001600160a01b039091166004820152600080516020611ee183398151915292506306447d569150602401600060405180830381600087803b1580156102f057600080fd5b505af1158015610304573d6000803e3d6000fd5b5050600b54600d5460405163095ea7b360e01b81526001600160a01b03918216600482015260001960248201529116925063095ea7b391506044016020604051808303816000875af115801561035e573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906103829190610cec565b50600c54600d5460405163095ea7b360e01b81526001600160a01b039182166004820152600019602482015291169063095ea7b3906044016020604051808303816000875af11580156103d9573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906103fd9190610cec565b50604080516390c5013b60e01b81529051600080516020611ee1833981519152916390c5013b91600480830192600092919082900301818387803b15801561044457600080fd5b505af1158015610458573d6000803e3d6000fd5b50506009546040516303223eab60e11b81526001600160a01b039091166004820152600080516020611ee183398151915292506306447d569150602401600060405180830381600087803b1580156104af57600080fd5b505af11580156104c3573d6000803e3d6000fd5b5050600b54600d5460405163095ea7b360e01b81526001600160a01b03918216600482015260001960248201529116925063095ea7b391506044016020604051808303816000875af115801561051d573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906105419190610cec565b50600c54600d5460405163095ea7b360e01b81526001600160a01b039182166004820152600019602482015291169063095ea7b3906044016020604051808303816000875af1158015610598573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906105bc9190610cec565b50604080516390c5013b60e01b81529051600080516020611ee1833981519152916390c5013b91600480830192600092919082900301818387803b15801561060357600080fd5b505af1158015610617573d6000803e3d6000fd5b5050600a546040516303223eab60e11b81526001600160a01b039091166004820152600080516020611ee183398151915292506306447d569150602401600060405180830381600087803b15801561066e57600080fd5b505af1158015610682573d6000803e3d6000fd5b5050600b54600d5460405163095ea7b360e01b81526001600160a01b03918216600482015260001960248201529116925063095ea7b391506044016020604051808303816000875af11580156106dc573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906107009190610cec565b50600c54600d5460405163095ea7b360e01b81526001600160a01b039182166004820152600019602482015291169063095ea7b3906044016020604051808303816000875af1158015610757573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061077b9190610cec565b50604080516390c5013b60e01b81529051600080516020611ee1833981519152916390c5013b91600480830192600092919082900301818387803b1580156107c257600080fd5b505af11580156107d6573d6000803e3d6000fd5b5050600d54600b54604051630ffb1d8b60e01b81526001600160a01b0391821660048201526001602482015291169250630ffb1d8b9150604401600060405180830381600087803b15801561082a57600080fd5b505af115801561083e573d6000803e3d6000fd5b5050600d54600c54604051630ffb1d8b60e01b81526001600160a01b0391821660048201526001602482015291169250630ffb1d8b9150604401600060405180830381600087803b15801561089257600080fd5b505af11580156108a6573d6000803e3d6000fd5b5050600d54600a5460405163194f29df60e11b81526001600160a01b039182166004820152600160248201529116925063329e53be9150604401600060405180830381600087803b1580156108fa57600080fd5b505af115801561090e573d6000803e3d6000fd5b5050600d54600a5460405163889590f160e01b81526001600160a01b039182166004820152600160248201529116925063889590f19150604401600060405180830381600087803b15801561096257600080fd5b505af1158015610976573d6000803e3d6000fd5b5050600d5460405163889590f160e01b8152306004820152600160248201526001600160a01b03909116925063889590f191506044015b600060405180830381600087803b1580156109c757600080fd5b505af11580156109db573d6000803e3d6000fd5b50505050565b60008054610100900460ff1615610a015750600054610100900460ff1690565b6000600080516020611ee18339815191523b15610afb5760408051600080516020611ee1833981519152602082018190526519985a5b195960d21b82840152825180830384018152606083019093526000929091610a83917f667f9d70ca411d70ead50d8d5c22070dafc36ad75f3dcf5e7237b22ade9aecc491608001610d50565b60408051601f1981840301815290829052610a9d91610d74565b6000604051808303816000865af19150503d8060008114610ada576040519150601f19603f3d011682016040523d82523d6000602084013e610adf565b606091505b5091505080806020019051810190610af79190610cec565b9150505b919050565b600854604080516318caf8e360e31b81526001600160a01b03909216600483015260248201526005604482015264416c69636560d81b6064820152600080516020611ee18339815191529063c657c71890608401600060405180830381600087803b158015610b6e57600080fd5b505af1158015610b82573d6000803e3d6000fd5b5050600954604080516318caf8e360e31b81526001600160a01b039092166004830152602482015260036044820152622137b160e91b6064820152600080516020611ee1833981519152925063c657c7189150608401600060405180830381600087803b158015610bf257600080fd5b505af1158015610c06573d6000803e3d6000fd5b5050600a54604080516318caf8e360e31b81526001600160a01b03909216600483015260248201526007604482015266436861726c696560c81b6064820152600080516020611ee1833981519152925063c657c7189150608401600060405180830381600087803b158015610c7a57600080fd5b505af1158015610c8e573d6000803e3d6000fd5b5050604080516318caf8e360e31b815230600482015260248101919091526003604482015262596f7560e81b6064820152600080516020611ee1833981519152925063c657c71891506084016109ad565b61116080610d8183390190565b600060208284031215610cfe57600080fd5b81518015158114610d0e57600080fd5b9392505050565b6000815160005b81811015610d365760208185018101518683015201610d1c565b81811115610d45576000828601525b509290920192915050565b6001600160e01b0319831681526000610d6c6004830184610d15565b949350505050565b6000610d0e8284610d1556fe608060405234801561001057600080fd5b50600080546001600160a01b031916339081178255604051909182917f8292fce18fa69edf4db7b94ea2e58241df0ae57f97e0a6c9b29067028bf92d76908290a350336000908152600360205260409020805460ff191660011790556110e58061007b6000396000f3fe608060405234801561001057600080fd5b50600436106100b45760003560e01c8063c70a900f11610071578063c70a900f1461015e578063d6f8307f14610191578063daf9c210146101b4578063e6717ce7146101d7578063f7d3b58b146101f8578063fc361c381461020b57600080fd5b80630ffb1d8b146100b957806313af4035146100ce578063329e53be146100e1578063889590f1146100f45780638da5cb5b146101075780639d86698514610137575b600080fd5b6100cc6100c7366004610b44565b61021e565b005b6100cc6100dc366004610b77565b6102b1565b6100cc6100ef366004610b44565b610326565b6100cc610102366004610b44565b6103a8565b60005461011a906001600160a01b031681565b6040516001600160a01b0390911681526020015b60405180910390f35b61014a610145366004610b99565b61042a565b60405161012e989796959493929190610bea565b61018161016c366004610b77565b60036020526000908152604090205460ff1681565b604051901515815260200161012e565b61018161019f366004610b77565b60046020526000908152604090205460ff1681565b6101816101c2366004610b77565b60026020526000908152604090205460ff1681565b6101ea6101e5366004610c93565b610511565b60405190815260200161012e565b6101ea610206366004610cef565b61052a565b6100cc610219366004610d9c565b610804565b6000546001600160a01b031633146102515760405162461bcd60e51b815260040161024890610dbf565b60405180910390fd5b6001600160a01b038216600081815260026020908152604091829020805460ff191685151590811790915591519182527fef81a9943b96c8df4ef243401c9bf5159146166211356898b52d382086168d9291015b60405180910390a25050565b6000546001600160a01b031633146102db5760405162461bcd60e51b815260040161024890610dbf565b600080546001600160a01b0319166001600160a01b0383169081178255604051909133917f8292fce18fa69edf4db7b94ea2e58241df0ae57f97e0a6c9b29067028bf92d769190a350565b6000546001600160a01b031633146103505760405162461bcd60e51b815260040161024890610dbf565b6001600160a01b038216600081815260036020908152604091829020805460ff191685151590811790915591519182527f8cc72bec7e2cf5979aefd933f40a28eb590098d522a5c458e497e71c5e6fa90f91016102a5565b6000546001600160a01b031633146103d25760405162461bcd60e51b815260040161024890610dbf565b6001600160a01b038216600081815260046020908152604091829020805460ff191685151590811790915591519182527f2bf32d6c1bf8a8d32ace419214cf1c0fa979b6ed1135840088771f67f25f104991016102a5565b600160208190526000918252604090912080549181015460028201546003830154600484015460058501546006860180546001600160a01b0398891698968716979587169690941694929360ff808416946101009094041692919061048e90610de5565b80601f01602080910402602001604051908101604052809291908181526020018280546104ba90610de5565b80156105075780601f106104dc57610100808354040283529160200191610507565b820191906000526020600020905b8154815290600101906020018083116104ea57829003601f168201915b5050505050905088565b600061052086868686866109ae565b9695505050505050565b3360009081526004602052604081205460ff1661055a576040516325c0cb7960e11b815260040160405180910390fd5b6001600160a01b03851660009081526002602052604090205460ff1661059e5760405163028511d160e51b81526001600160a01b0386166004820152602401610248565b6105ab33888888886109ae565b90506000808281526001602052604090206005015460ff1660028111156105d4576105d4610bb2565b146105f55760405163d5e880e960e01b815260048101829052602401610248565b6000604051806101000160405280336001600160a01b03168152602001896001600160a01b03168152602001886001600160a01b03168152602001876001600160a01b031681526020018681526020016001600281111561065857610658610bb2565b815260200160001515815260200185858080601f016020809104026020016040519081016040528093929190818152602001838380828437600092018290525093909452505084815260016020818152604092839020855181546001600160a01b03199081166001600160a01b039283161783559287015182850180548516918316919091179055938601516002808301805485169287169290921790915560608701516003830180549094169516949094179091556080850151600482015560a085015160058201805496975087969295509093909260ff199092169190849081111561074857610748610bb2565b021790555060c08201516005820180549115156101000261ff001990921691909117905560e082015160068201906107809082610e84565b5050604080516001600160a01b03898116825260208201899052808b1693508b169185917f2609e116ca576195fecb47831f147fd6fee721377e4d88fff48807f3e9442a42910160405180910390a46107e46001600160a01b038716333088610a16565b6107f96001600160a01b038716883088610a16565b509695505050505050565b3360009081526003602052604090205460ff16610834576040516314e9dd3960e31b815260040160405180910390fd5b60008281526001602052604090206002600582015460ff16600281111561085d5761085d610bb2565b0361087e5760405163b677167760e01b815260048101849052602401610248565b6000816004015460026108919190610f44565b905082156108bf57600182015460038301546108ba916001600160a01b03918216911683610aa0565b6108e0565b600282015460038301546108e0916001600160a01b03918216911683610aa0565b60058201805461ffff19166101008515159081029190911760021790915560405190815284907fdbed7580b9c2829ee6b384e3833f10b16f9885601c98a01c40fd705b543e9c669060200160405180910390a281546001600160a01b03163b156109a857815460405163734d162760e01b81526001600160a01b039091169063734d1627906109759087908690600401610fee565b600060405180830381600087803b15801561098f57600080fd5b505af11580156109a3573d6000803e3d6000fd5b505050505b50505050565b6040516bffffffffffffffffffffffff19606087811b8216602084015286811b8216603484015285811b8216604884015284901b16605c8201526070810182905260009060900160405160208183030381529060405280519060200120905095945050505050565b60006040516323b872dd60e01b81528460048201528360248201528260448201526020600060648360008a5af13d15601f3d1160016000511416171691505080610a995760405162461bcd60e51b81526020600482015260146024820152731514905394d1915497d19493d357d1905253115160621b6044820152606401610248565b5050505050565b600060405163a9059cbb60e01b8152836004820152826024820152602060006044836000895af13d15601f3d11600160005114161716915050806109a85760405162461bcd60e51b815260206004820152600f60248201526e1514905394d1915497d19052531151608a1b6044820152606401610248565b80356001600160a01b0381168114610b2f57600080fd5b919050565b80358015158114610b2f57600080fd5b60008060408385031215610b5757600080fd5b610b6083610b18565b9150610b6e60208401610b34565b90509250929050565b600060208284031215610b8957600080fd5b610b9282610b18565b9392505050565b600060208284031215610bab57600080fd5b5035919050565b634e487b7160e01b600052602160045260246000fd5b60038110610be657634e487b7160e01b600052602160045260246000fd5b9052565b600061010060018060a01b03808c1684526020818c1681860152818b166040860152818a166060860152886080860152610c2760a0860189610bc8565b86151560c08601528260e0860152855191508183860152600092505b81831015610c6257858301810151858401610120015291820191610c43565b5080821115610c7657600061012082860101525b601f01601f191692909201610120019a9950505050505050505050565b600080600080600060a08688031215610cab57600080fd5b610cb486610b18565b9450610cc260208701610b18565b9350610cd060408701610b18565b9250610cde60608701610b18565b949793965091946080013592915050565b60008060008060008060a08789031215610d0857600080fd5b610d1187610b18565b9550610d1f60208801610b18565b9450610d2d60408801610b18565b935060608701359250608087013567ffffffffffffffff80821115610d5157600080fd5b818901915089601f830112610d6557600080fd5b813581811115610d7457600080fd5b8a6020828501011115610d8657600080fd5b6020830194508093505050509295509295509295565b60008060408385031215610daf57600080fd5b82359150610b6e60208401610b34565b6020808252600c908201526b15539055551213d49256915160a21b604082015260600190565b600181811c90821680610df957607f821691505b602082108103610e1957634e487b7160e01b600052602260045260246000fd5b50919050565b634e487b7160e01b600052604160045260246000fd5b601f821115610e7f57600081815260208120601f850160051c81016020861015610e5c5750805b601f850160051c820191505b81811015610e7b57828155600101610e68565b5050505b505050565b815167ffffffffffffffff811115610e9e57610e9e610e1f565b610eb281610eac8454610de5565b84610e35565b602080601f831160018114610ee75760008415610ecf5750858301515b600019600386901b1c1916600185901b178555610e7b565b600085815260208120601f198616915b82811015610f1657888601518255948401946001909101908401610ef7565b5085821015610f345787850151600019600388901b60f8161c191681555b5050505050600190811b01905550565b6000816000190483118215151615610f6c57634e487b7160e01b600052601160045260246000fd5b500290565b60008154610f7e81610de5565b808552602060018381168015610f9b5760018114610fb557610fe3565b60ff1985168884015283151560051b880183019550610fe3565b866000528260002060005b85811015610fdb5781548a8201860152908301908401610fc0565b890184019650505b505050505092915050565b8281526040602082015261101d6040820161101084546001600160a01b031690565b6001600160a01b03169052565b600061103360018401546001600160a01b031690565b6001600160a01b03908116606084015260028401548116608084015260038401541660a0830152600483015460c0830152600583015461107960e0840160ff8316610bc8565b61010061108f81850160ff8460081c1615159052565b610120840152506110a7610140830160068501610f71565b94935050505056fea2646970667358221220378e2adecaf12d8d961ecc660ca8a2e8d7f85594e2d186b05480fc09bda2009864736f6c634300080f00330000000000000000000000007109709ecfa91a80626ff3989d68f67f5b1dd12da2646970667358221220d30f6fe1e8b72c0c5faa45642fb5eda7bc47cdff2c883f6b99fd45b4f08b0bed64736f6c634300080f0033" . parse () . expect ("invalid bytecode")
        });
    pub struct OracleFixture<M>(ethers::contract::Contract<M>);
    impl<M> Clone for OracleFixture<M> {
        fn clone(&self) -> Self {
            OracleFixture(self.0.clone())
        }
    }
    impl<M> std::ops::Deref for OracleFixture<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug for OracleFixture<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(OracleFixture))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ethers::providers::Middleware> OracleFixture<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(address.into(), ORACLEFIXTURE_ABI.clone(), client)
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
                ORACLEFIXTURE_ABI.clone(),
                ORACLEFIXTURE_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        #[doc = "Calls the contract's `IS_SCRIPT` (0xf8ccbf47) function"]
        pub fn is_script(&self) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([248, 204, 191, 71], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `IS_TEST` (0xfa7626d4) function"]
        pub fn is_test(&self) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([250, 118, 38, 212], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `failed` (0xba414fa6) function"]
        pub fn failed(&self) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([186, 65, 79, 166], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setUp` (0x0a9254e4) function"]
        pub fn set_up(&self) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([10, 146, 84, 228], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `vm` (0x3a768463) function"]
        pub fn vm(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([58, 118, 132, 99], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Gets the contract's `log` event"]
        pub fn log_filter(&self) -> ethers::contract::builders::Event<M, LogFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `log_address` event"]
        pub fn log_address_filter(&self) -> ethers::contract::builders::Event<M, LogAddressFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `log_array` event"]
        pub fn log_array_1_filter(&self) -> ethers::contract::builders::Event<M, LogArray1Filter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `log_array` event"]
        pub fn log_array_2_filter(&self) -> ethers::contract::builders::Event<M, LogArray2Filter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `log_array` event"]
        pub fn log_array_3_filter(&self) -> ethers::contract::builders::Event<M, LogArray3Filter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `log_bytes` event"]
        pub fn log_bytes_filter(&self) -> ethers::contract::builders::Event<M, LogBytesFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `log_bytes32` event"]
        pub fn log_bytes_32_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, LogBytes32Filter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `log_int` event"]
        pub fn log_int_filter(&self) -> ethers::contract::builders::Event<M, LogIntFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `log_named_address` event"]
        pub fn log_named_address_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, LogNamedAddressFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `log_named_array` event"]
        pub fn log_named_array_1_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, LogNamedArray1Filter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `log_named_array` event"]
        pub fn log_named_array_2_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, LogNamedArray2Filter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `log_named_array` event"]
        pub fn log_named_array_3_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, LogNamedArray3Filter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `log_named_bytes` event"]
        pub fn log_named_bytes_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, LogNamedBytesFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `log_named_bytes32` event"]
        pub fn log_named_bytes_32_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, LogNamedBytes32Filter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `log_named_decimal_int` event"]
        pub fn log_named_decimal_int_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, LogNamedDecimalIntFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `log_named_decimal_uint` event"]
        pub fn log_named_decimal_uint_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, LogNamedDecimalUintFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `log_named_int` event"]
        pub fn log_named_int_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, LogNamedIntFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `log_named_string` event"]
        pub fn log_named_string_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, LogNamedStringFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `log_named_uint` event"]
        pub fn log_named_uint_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, LogNamedUintFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `log_string` event"]
        pub fn log_string_filter(&self) -> ethers::contract::builders::Event<M, LogStringFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `log_uint` event"]
        pub fn log_uint_filter(&self) -> ethers::contract::builders::Event<M, LogUintFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `logs` event"]
        pub fn logs_filter(&self) -> ethers::contract::builders::Event<M, LogsFilter> {
            self.0.event()
        }
        #[doc = r" Returns an [`Event`](#ethers_contract::builders::Event) builder for all events of this contract"]
        pub fn events(&self) -> ethers::contract::builders::Event<M, OracleFixtureEvents> {
            self.0.event_with_filter(Default::default())
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>> for OracleFixture<M> {
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
    #[ethevent(name = "log", abi = "log(string)")]
    pub struct LogFilter(pub String);
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
    )]
    #[ethevent(name = "log_address", abi = "log_address(address)")]
    pub struct LogAddressFilter(pub ethers::core::types::Address);
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
    )]
    #[ethevent(name = "log_array", abi = "log_array(uint256[])")]
    pub struct LogArray1Filter {
        pub val: Vec<ethers::core::types::U256>,
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
    #[ethevent(name = "log_array", abi = "log_array(int256[])")]
    pub struct LogArray2Filter {
        pub val: Vec<I256>,
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
    #[ethevent(name = "log_array", abi = "log_array(address[])")]
    pub struct LogArray3Filter {
        pub val: Vec<ethers::core::types::Address>,
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
    #[ethevent(name = "log_bytes", abi = "log_bytes(bytes)")]
    pub struct LogBytesFilter(pub ethers::core::types::Bytes);
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
    )]
    #[ethevent(name = "log_bytes32", abi = "log_bytes32(bytes32)")]
    pub struct LogBytes32Filter(pub [u8; 32]);
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
    )]
    #[ethevent(name = "log_int", abi = "log_int(int256)")]
    pub struct LogIntFilter(pub I256);
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
    )]
    #[ethevent(name = "log_named_address", abi = "log_named_address(string,address)")]
    pub struct LogNamedAddressFilter {
        pub key: String,
        pub val: ethers::core::types::Address,
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
    #[ethevent(name = "log_named_array", abi = "log_named_array(string,uint256[])")]
    pub struct LogNamedArray1Filter {
        pub key: String,
        pub val: Vec<ethers::core::types::U256>,
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
    #[ethevent(name = "log_named_array", abi = "log_named_array(string,int256[])")]
    pub struct LogNamedArray2Filter {
        pub key: String,
        pub val: Vec<I256>,
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
    #[ethevent(name = "log_named_array", abi = "log_named_array(string,address[])")]
    pub struct LogNamedArray3Filter {
        pub key: String,
        pub val: Vec<ethers::core::types::Address>,
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
    #[ethevent(name = "log_named_bytes", abi = "log_named_bytes(string,bytes)")]
    pub struct LogNamedBytesFilter {
        pub key: String,
        pub val: ethers::core::types::Bytes,
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
    #[ethevent(name = "log_named_bytes32", abi = "log_named_bytes32(string,bytes32)")]
    pub struct LogNamedBytes32Filter {
        pub key: String,
        pub val: [u8; 32],
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
        name = "log_named_decimal_int",
        abi = "log_named_decimal_int(string,int256,uint256)"
    )]
    pub struct LogNamedDecimalIntFilter {
        pub key: String,
        pub val: I256,
        pub decimals: ethers::core::types::U256,
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
        name = "log_named_decimal_uint",
        abi = "log_named_decimal_uint(string,uint256,uint256)"
    )]
    pub struct LogNamedDecimalUintFilter {
        pub key: String,
        pub val: ethers::core::types::U256,
        pub decimals: ethers::core::types::U256,
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
    #[ethevent(name = "log_named_int", abi = "log_named_int(string,int256)")]
    pub struct LogNamedIntFilter {
        pub key: String,
        pub val: I256,
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
    #[ethevent(name = "log_named_string", abi = "log_named_string(string,string)")]
    pub struct LogNamedStringFilter {
        pub key: String,
        pub val: String,
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
    #[ethevent(name = "log_named_uint", abi = "log_named_uint(string,uint256)")]
    pub struct LogNamedUintFilter {
        pub key: String,
        pub val: ethers::core::types::U256,
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
    #[ethevent(name = "log_string", abi = "log_string(string)")]
    pub struct LogStringFilter(pub String);
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
    )]
    #[ethevent(name = "log_uint", abi = "log_uint(uint256)")]
    pub struct LogUintFilter(pub ethers::core::types::U256);
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
    )]
    #[ethevent(name = "logs", abi = "logs(bytes)")]
    pub struct LogsFilter(pub ethers::core::types::Bytes);
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum OracleFixtureEvents {
        LogFilter(LogFilter),
        LogAddressFilter(LogAddressFilter),
        LogArray1Filter(LogArray1Filter),
        LogArray2Filter(LogArray2Filter),
        LogArray3Filter(LogArray3Filter),
        LogBytesFilter(LogBytesFilter),
        LogBytes32Filter(LogBytes32Filter),
        LogIntFilter(LogIntFilter),
        LogNamedAddressFilter(LogNamedAddressFilter),
        LogNamedArray1Filter(LogNamedArray1Filter),
        LogNamedArray2Filter(LogNamedArray2Filter),
        LogNamedArray3Filter(LogNamedArray3Filter),
        LogNamedBytesFilter(LogNamedBytesFilter),
        LogNamedBytes32Filter(LogNamedBytes32Filter),
        LogNamedDecimalIntFilter(LogNamedDecimalIntFilter),
        LogNamedDecimalUintFilter(LogNamedDecimalUintFilter),
        LogNamedIntFilter(LogNamedIntFilter),
        LogNamedStringFilter(LogNamedStringFilter),
        LogNamedUintFilter(LogNamedUintFilter),
        LogStringFilter(LogStringFilter),
        LogUintFilter(LogUintFilter),
        LogsFilter(LogsFilter),
    }
    impl ethers::contract::EthLogDecode for OracleFixtureEvents {
        fn decode_log(
            log: &ethers::core::abi::RawLog,
        ) -> ::std::result::Result<Self, ethers::core::abi::Error>
        where
            Self: Sized,
        {
            if let Ok(decoded) = LogFilter::decode_log(log) {
                return Ok(OracleFixtureEvents::LogFilter(decoded));
            }
            if let Ok(decoded) = LogAddressFilter::decode_log(log) {
                return Ok(OracleFixtureEvents::LogAddressFilter(decoded));
            }
            if let Ok(decoded) = LogArray1Filter::decode_log(log) {
                return Ok(OracleFixtureEvents::LogArray1Filter(decoded));
            }
            if let Ok(decoded) = LogArray2Filter::decode_log(log) {
                return Ok(OracleFixtureEvents::LogArray2Filter(decoded));
            }
            if let Ok(decoded) = LogArray3Filter::decode_log(log) {
                return Ok(OracleFixtureEvents::LogArray3Filter(decoded));
            }
            if let Ok(decoded) = LogBytesFilter::decode_log(log) {
                return Ok(OracleFixtureEvents::LogBytesFilter(decoded));
            }
            if let Ok(decoded) = LogBytes32Filter::decode_log(log) {
                return Ok(OracleFixtureEvents::LogBytes32Filter(decoded));
            }
            if let Ok(decoded) = LogIntFilter::decode_log(log) {
                return Ok(OracleFixtureEvents::LogIntFilter(decoded));
            }
            if let Ok(decoded) = LogNamedAddressFilter::decode_log(log) {
                return Ok(OracleFixtureEvents::LogNamedAddressFilter(decoded));
            }
            if let Ok(decoded) = LogNamedArray1Filter::decode_log(log) {
                return Ok(OracleFixtureEvents::LogNamedArray1Filter(decoded));
            }
            if let Ok(decoded) = LogNamedArray2Filter::decode_log(log) {
                return Ok(OracleFixtureEvents::LogNamedArray2Filter(decoded));
            }
            if let Ok(decoded) = LogNamedArray3Filter::decode_log(log) {
                return Ok(OracleFixtureEvents::LogNamedArray3Filter(decoded));
            }
            if let Ok(decoded) = LogNamedBytesFilter::decode_log(log) {
                return Ok(OracleFixtureEvents::LogNamedBytesFilter(decoded));
            }
            if let Ok(decoded) = LogNamedBytes32Filter::decode_log(log) {
                return Ok(OracleFixtureEvents::LogNamedBytes32Filter(decoded));
            }
            if let Ok(decoded) = LogNamedDecimalIntFilter::decode_log(log) {
                return Ok(OracleFixtureEvents::LogNamedDecimalIntFilter(decoded));
            }
            if let Ok(decoded) = LogNamedDecimalUintFilter::decode_log(log) {
                return Ok(OracleFixtureEvents::LogNamedDecimalUintFilter(decoded));
            }
            if let Ok(decoded) = LogNamedIntFilter::decode_log(log) {
                return Ok(OracleFixtureEvents::LogNamedIntFilter(decoded));
            }
            if let Ok(decoded) = LogNamedStringFilter::decode_log(log) {
                return Ok(OracleFixtureEvents::LogNamedStringFilter(decoded));
            }
            if let Ok(decoded) = LogNamedUintFilter::decode_log(log) {
                return Ok(OracleFixtureEvents::LogNamedUintFilter(decoded));
            }
            if let Ok(decoded) = LogStringFilter::decode_log(log) {
                return Ok(OracleFixtureEvents::LogStringFilter(decoded));
            }
            if let Ok(decoded) = LogUintFilter::decode_log(log) {
                return Ok(OracleFixtureEvents::LogUintFilter(decoded));
            }
            if let Ok(decoded) = LogsFilter::decode_log(log) {
                return Ok(OracleFixtureEvents::LogsFilter(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::std::fmt::Display for OracleFixtureEvents {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                OracleFixtureEvents::LogFilter(element) => element.fmt(f),
                OracleFixtureEvents::LogAddressFilter(element) => element.fmt(f),
                OracleFixtureEvents::LogArray1Filter(element) => element.fmt(f),
                OracleFixtureEvents::LogArray2Filter(element) => element.fmt(f),
                OracleFixtureEvents::LogArray3Filter(element) => element.fmt(f),
                OracleFixtureEvents::LogBytesFilter(element) => element.fmt(f),
                OracleFixtureEvents::LogBytes32Filter(element) => element.fmt(f),
                OracleFixtureEvents::LogIntFilter(element) => element.fmt(f),
                OracleFixtureEvents::LogNamedAddressFilter(element) => element.fmt(f),
                OracleFixtureEvents::LogNamedArray1Filter(element) => element.fmt(f),
                OracleFixtureEvents::LogNamedArray2Filter(element) => element.fmt(f),
                OracleFixtureEvents::LogNamedArray3Filter(element) => element.fmt(f),
                OracleFixtureEvents::LogNamedBytesFilter(element) => element.fmt(f),
                OracleFixtureEvents::LogNamedBytes32Filter(element) => element.fmt(f),
                OracleFixtureEvents::LogNamedDecimalIntFilter(element) => element.fmt(f),
                OracleFixtureEvents::LogNamedDecimalUintFilter(element) => element.fmt(f),
                OracleFixtureEvents::LogNamedIntFilter(element) => element.fmt(f),
                OracleFixtureEvents::LogNamedStringFilter(element) => element.fmt(f),
                OracleFixtureEvents::LogNamedUintFilter(element) => element.fmt(f),
                OracleFixtureEvents::LogStringFilter(element) => element.fmt(f),
                OracleFixtureEvents::LogUintFilter(element) => element.fmt(f),
                OracleFixtureEvents::LogsFilter(element) => element.fmt(f),
            }
        }
    }
    #[doc = "Container type for all input parameters for the `IS_SCRIPT` function with signature `IS_SCRIPT()` and selector `[248, 204, 191, 71]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "IS_SCRIPT", abi = "IS_SCRIPT()")]
    pub struct IsScriptCall;
    #[doc = "Container type for all input parameters for the `IS_TEST` function with signature `IS_TEST()` and selector `[250, 118, 38, 212]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "IS_TEST", abi = "IS_TEST()")]
    pub struct IsTestCall;
    #[doc = "Container type for all input parameters for the `failed` function with signature `failed()` and selector `[186, 65, 79, 166]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "failed", abi = "failed()")]
    pub struct FailedCall;
    #[doc = "Container type for all input parameters for the `setUp` function with signature `setUp()` and selector `[10, 146, 84, 228]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "setUp", abi = "setUp()")]
    pub struct SetUpCall;
    #[doc = "Container type for all input parameters for the `vm` function with signature `vm()` and selector `[58, 118, 132, 99]`"]
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
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum OracleFixtureCalls {
        IsScript(IsScriptCall),
        IsTest(IsTestCall),
        Failed(FailedCall),
        SetUp(SetUpCall),
        Vm(VmCall),
    }
    impl ethers::core::abi::AbiDecode for OracleFixtureCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::std::result::Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <IsScriptCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(OracleFixtureCalls::IsScript(decoded));
            }
            if let Ok(decoded) = <IsTestCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(OracleFixtureCalls::IsTest(decoded));
            }
            if let Ok(decoded) = <FailedCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(OracleFixtureCalls::Failed(decoded));
            }
            if let Ok(decoded) = <SetUpCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(OracleFixtureCalls::SetUp(decoded));
            }
            if let Ok(decoded) = <VmCall as ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(OracleFixtureCalls::Vm(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for OracleFixtureCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                OracleFixtureCalls::IsScript(element) => element.encode(),
                OracleFixtureCalls::IsTest(element) => element.encode(),
                OracleFixtureCalls::Failed(element) => element.encode(),
                OracleFixtureCalls::SetUp(element) => element.encode(),
                OracleFixtureCalls::Vm(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for OracleFixtureCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                OracleFixtureCalls::IsScript(element) => element.fmt(f),
                OracleFixtureCalls::IsTest(element) => element.fmt(f),
                OracleFixtureCalls::Failed(element) => element.fmt(f),
                OracleFixtureCalls::SetUp(element) => element.fmt(f),
                OracleFixtureCalls::Vm(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<IsScriptCall> for OracleFixtureCalls {
        fn from(var: IsScriptCall) -> Self {
            OracleFixtureCalls::IsScript(var)
        }
    }
    impl ::std::convert::From<IsTestCall> for OracleFixtureCalls {
        fn from(var: IsTestCall) -> Self {
            OracleFixtureCalls::IsTest(var)
        }
    }
    impl ::std::convert::From<FailedCall> for OracleFixtureCalls {
        fn from(var: FailedCall) -> Self {
            OracleFixtureCalls::Failed(var)
        }
    }
    impl ::std::convert::From<SetUpCall> for OracleFixtureCalls {
        fn from(var: SetUpCall) -> Self {
            OracleFixtureCalls::SetUp(var)
        }
    }
    impl ::std::convert::From<VmCall> for OracleFixtureCalls {
        fn from(var: VmCall) -> Self {
            OracleFixtureCalls::Vm(var)
        }
    }
    #[doc = "Container type for all return fields from the `IS_SCRIPT` function with signature `IS_SCRIPT()` and selector `[248, 204, 191, 71]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct IsScriptReturn(pub bool);
    #[doc = "Container type for all return fields from the `IS_TEST` function with signature `IS_TEST()` and selector `[250, 118, 38, 212]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct IsTestReturn(pub bool);
    #[doc = "Container type for all return fields from the `failed` function with signature `failed()` and selector `[186, 65, 79, 166]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct FailedReturn(pub bool);
    #[doc = "Container type for all return fields from the `vm` function with signature `vm()` and selector `[58, 118, 132, 99]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct VmReturn(pub ethers::core::types::Address);
}
