///`AddressFilter(address,bool)`
#[derive(
    Clone,
    ::ethers::contract::EthAbiType,
    ::ethers::contract::EthAbiCodec,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash
)]
pub struct AddressFilter {
    pub value: ::ethers::core::types::Address,
    pub exclude: bool,
}
///`AuthFilter(bool,(address,bool),((address,bool),(uint256,uint256))[],((address,bool),(uint256,uint256)),(uint256,uint256),(uint256,uint256))`
#[derive(
    Clone,
    ::ethers::contract::EthAbiType,
    ::ethers::contract::EthAbiCodec,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash
)]
pub struct AuthFilter {
    pub initialized: bool,
    pub offerer: AddressFilter,
    pub offer: ::std::vec::Vec<ItemFilter>,
    pub consideration: ItemFilter,
    pub deadline: RangeFilter,
    pub nonce: RangeFilter,
}
///`ItemFilter((address,bool),(uint256,uint256))`
#[derive(
    Clone,
    ::ethers::contract::EthAbiType,
    ::ethers::contract::EthAbiCodec,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash
)]
pub struct ItemFilter {
    pub token: AddressFilter,
    pub amount: RangeFilter,
}
///`RangeFilter(uint256,uint256)`
#[derive(
    Clone,
    ::ethers::contract::EthAbiType,
    ::ethers::contract::EthAbiCodec,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash
)]
pub struct RangeFilter {
    pub gte: ::ethers::core::types::U256,
    pub lte: ::ethers::core::types::U256,
}
///`Item(address,uint256)`
#[derive(
    Clone,
    ::ethers::contract::EthAbiType,
    ::ethers::contract::EthAbiCodec,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash
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
    Hash
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
    Hash
)]
pub struct OrderWithSignature {
    pub order: Order,
    pub signature: ::ethers::core::types::Bytes,
}
///`ExecutorInfo(address,bool,bool)`
#[derive(
    Clone,
    ::ethers::contract::EthAbiType,
    ::ethers::contract::EthAbiCodec,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash
)]
pub struct ExecutorInfo {
    pub executor: ::ethers::core::types::Address,
    pub has_callback: bool,
    pub is_enabled: bool,
}
///`FuzzSelector(address,bytes4[])`
#[derive(
    Clone,
    ::ethers::contract::EthAbiType,
    ::ethers::contract::EthAbiCodec,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash
)]
pub struct FuzzSelector {
    pub addr: ::ethers::core::types::Address,
    pub selectors: ::std::vec::Vec<[u8; 4]>,
}
