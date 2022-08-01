#[doc = "`Request(address,address,address,address,uint256,uint8,bool,bytes)`"]
#[derive(
    Clone,
    Debug,
    Default,
    Eq,
    PartialEq,
    ethers :: contract :: EthAbiType,
    ethers :: contract :: EthAbiCodec,
)]
pub struct Request {
    pub requester: ethers::core::types::Address,
    pub proposer: ethers::core::types::Address,
    pub disputer: ethers::core::types::Address,
    pub currency: ethers::core::types::Address,
    pub bond: ethers::core::types::U256,
    pub state: u8,
    pub answer: bool,
    pub data: ethers::core::types::Bytes,
}
