///`Item(address,uint256)`
#[derive(
    Clone,
    ::ethers::contract::EthAbiType,
    ::ethers::contract::EthAbiCodec,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash,
)]
pub struct Item {
    pub token: ::ethers::core::types::Address,
    pub amount: ::ethers::core::types::U256,
}
///`Order(address,address,(address,uint256)[],(address,uint256)[],uint256,uint256)`
#[derive(
    Clone,
    ::ethers::contract::EthAbiType,
    ::ethers::contract::EthAbiCodec,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash,
)]
pub struct Order {
    pub offerer: ::ethers::core::types::Address,
    pub zone: ::ethers::core::types::Address,
    pub offer: ::std::vec::Vec<Item>,
    pub consideration: ::std::vec::Vec<Item>,
    pub deadline: ::ethers::core::types::U256,
    pub nonce: ::ethers::core::types::U256,
}
///`OrderWithSignature((address,address,(address,uint256)[],(address,uint256)[],uint256,uint256),bytes)`
#[derive(
    Clone,
    ::ethers::contract::EthAbiType,
    ::ethers::contract::EthAbiCodec,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash,
)]
pub struct OrderWithSignature {
    pub order: Order,
    pub signature: ::ethers::core::types::Bytes,
}
