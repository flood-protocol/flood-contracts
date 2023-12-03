///`AddressFilter(address,bool)`
#[derive(
    Clone,
    ::ethers::contract::EthAbiType,
    ::ethers::contract::EthAbiCodec,
    serde::Serialize,
    serde::Deserialize,
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
    serde::Serialize,
    serde::Deserialize,
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
    serde::Serialize,
    serde::Deserialize,
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
    serde::Serialize,
    serde::Deserialize,
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
///`Hook(address,bytes)`
#[derive(
    Clone,
    ::ethers::contract::EthAbiType,
    ::ethers::contract::EthAbiCodec,
    serde::Serialize,
    serde::Deserialize,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash
)]
pub struct Hook {
    pub target: ::ethers::core::types::Address,
    pub data: ::ethers::core::types::Bytes,
}
///`Item(address,uint256)`
#[derive(
    Clone,
    ::ethers::contract::EthAbiType,
    ::ethers::contract::EthAbiCodec,
    serde::Serialize,
    serde::Deserialize,
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
///`Order(address,address,address,(address,uint256)[],(address,uint256),uint256,uint256,(address,bytes)[],(address,bytes)[])`
#[derive(
    Clone,
    ::ethers::contract::EthAbiType,
    ::ethers::contract::EthAbiCodec,
    serde::Serialize,
    serde::Deserialize,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash
)]
pub struct Order {
    pub offerer: ::ethers::core::types::Address,
    pub zone: ::ethers::core::types::Address,
    pub recipient: ::ethers::core::types::Address,
    pub offer: ::std::vec::Vec<Item>,
    pub consideration: Item,
    pub deadline: ::ethers::core::types::U256,
    pub nonce: ::ethers::core::types::U256,
    pub pre_hooks: ::std::vec::Vec<Hook>,
    pub post_hooks: ::std::vec::Vec<Hook>,
}
///`SignedOrder((address,address,address,(address,uint256)[],(address,uint256),uint256,uint256,(address,bytes)[],(address,bytes)[]),bytes)`
#[derive(
    Clone,
    ::ethers::contract::EthAbiType,
    ::ethers::contract::EthAbiCodec,
    serde::Serialize,
    serde::Deserialize,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash
)]
pub struct SignedOrder {
    pub order: Order,
    pub signature: ::ethers::core::types::Bytes,
}
///`FeeInfo(address,uint64)`
#[derive(
    Clone,
    ::ethers::contract::EthAbiType,
    ::ethers::contract::EthAbiCodec,
    serde::Serialize,
    serde::Deserialize,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash
)]
pub struct FeeInfo {
    pub recipient: ::ethers::core::types::Address,
    pub bps: u64,
}
///`FuzzInterface(address,string[])`
#[derive(
    Clone,
    ::ethers::contract::EthAbiType,
    ::ethers::contract::EthAbiCodec,
    serde::Serialize,
    serde::Deserialize,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash
)]
pub struct FuzzInterface {
    pub addr: ::ethers::core::types::Address,
    pub artifacts: ::std::vec::Vec<::std::string::String>,
}
///`FuzzSelector(address,bytes4[])`
#[derive(
    Clone,
    ::ethers::contract::EthAbiType,
    ::ethers::contract::EthAbiCodec,
    serde::Serialize,
    serde::Deserialize,
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
