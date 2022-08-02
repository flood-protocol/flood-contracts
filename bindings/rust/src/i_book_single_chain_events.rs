pub use i_book_single_chain_events::*;
#[allow(clippy::too_many_arguments, non_camel_case_types)]
pub mod i_book_single_chain_events {
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
    #[doc = "IBookSingleChainEvents was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static IBOOKSINGLECHAINEVENTS_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            ethers :: core :: utils :: __serde_json :: from_str ("[{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"disputeBondPct\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"tradeRebatePct\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"relayerRefundPct\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"FeeCombinationSet\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"newSafeBlockThreshold\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"SafeBlockThresholdSet\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"bool\",\"name\":\"whitelisted\",\"type\":\"bool\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"TokenWhitelisted\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"relayer\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"tradeIndex\",\"type\":\"uint256\",\"components\":[],\"indexed\":true},{\"internalType\":\"bytes32\",\"name\":\"disputeId\",\"type\":\"bytes32\",\"components\":[],\"indexed\":true},{\"internalType\":\"bool\",\"name\":\"answer\",\"type\":\"bool\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"TradeDisputeSettled\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"relayer\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"tradeIndex\",\"type\":\"uint256\",\"components\":[],\"indexed\":true},{\"internalType\":\"bytes32\",\"name\":\"disputeId\",\"type\":\"bytes32\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"filledAtBlock\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"TradeDisputed\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"relayer\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"tradeIndex\",\"type\":\"uint256\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"feePct\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"amountOut\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"TradeFilled\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"tokenIn\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"tokenOut\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"amountIn\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"minAmountOut\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"feePct\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"tradeIndex\",\"type\":\"uint256\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"TradeRequested\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"relayer\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"tradeIndex\",\"type\":\"uint256\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"filledAtBlock\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"TradeSettled\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"trader\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"tradeIndex\",\"type\":\"uint256\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"newFeePct\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"UpdatedFeeForTrade\",\"outputs\":[],\"anonymous\":false}]") . expect ("invalid abi")
        });
    pub struct IBookSingleChainEvents<M>(ethers::contract::Contract<M>);
    impl<M> Clone for IBookSingleChainEvents<M> {
        fn clone(&self) -> Self {
            IBookSingleChainEvents(self.0.clone())
        }
    }
    impl<M> std::ops::Deref for IBookSingleChainEvents<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug for IBookSingleChainEvents<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(IBookSingleChainEvents))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ethers::providers::Middleware> IBookSingleChainEvents<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(
                address.into(),
                IBOOKSINGLECHAINEVENTS_ABI.clone(),
                client,
            )
            .into()
        }
        #[doc = "Gets the contract's `FeeCombinationSet` event"]
        pub fn fee_combination_set_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, FeeCombinationSetFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `SafeBlockThresholdSet` event"]
        pub fn safe_block_threshold_set_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, SafeBlockThresholdSetFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `TokenWhitelisted` event"]
        pub fn token_whitelisted_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, TokenWhitelistedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `TradeDisputeSettled` event"]
        pub fn trade_dispute_settled_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, TradeDisputeSettledFilter> {
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
        pub fn events(&self) -> ethers::contract::builders::Event<M, IBookSingleChainEventsEvents> {
            self.0.event_with_filter(Default::default())
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>>
        for IBookSingleChainEvents<M>
    {
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
        name = "FeeCombinationSet",
        abi = "FeeCombinationSet(uint256,uint256,uint256)"
    )]
    pub struct FeeCombinationSetFilter {
        pub dispute_bond_pct: ethers::core::types::U256,
        pub trade_rebate_pct: ethers::core::types::U256,
        pub relayer_refund_pct: ethers::core::types::U256,
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
    #[ethevent(name = "SafeBlockThresholdSet", abi = "SafeBlockThresholdSet(uint256)")]
    pub struct SafeBlockThresholdSetFilter {
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
        name = "TradeDisputeSettled",
        abi = "TradeDisputeSettled(address,uint256,bytes32,bool)"
    )]
    pub struct TradeDisputeSettledFilter {
        #[ethevent(indexed)]
        pub relayer: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub trade_index: ethers::core::types::U256,
        #[ethevent(indexed)]
        pub dispute_id: [u8; 32],
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
        name = "TradeDisputed",
        abi = "TradeDisputed(address,uint256,bytes32,uint256)"
    )]
    pub struct TradeDisputedFilter {
        #[ethevent(indexed)]
        pub relayer: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub trade_index: ethers::core::types::U256,
        #[ethevent(indexed)]
        pub dispute_id: [u8; 32],
        pub filled_at_block: ethers::core::types::U256,
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
        abi = "TradeRequested(address,address,uint256,uint256,uint256,address,uint256)"
    )]
    pub struct TradeRequestedFilter {
        #[ethevent(indexed)]
        pub token_in: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub token_out: ethers::core::types::Address,
        pub amount_in: ethers::core::types::U256,
        pub min_amount_out: ethers::core::types::U256,
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
    #[ethevent(name = "TradeSettled", abi = "TradeSettled(address,uint256,uint256)")]
    pub struct TradeSettledFilter {
        #[ethevent(indexed)]
        pub relayer: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub trade_index: ethers::core::types::U256,
        pub filled_at_block: ethers::core::types::U256,
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
    pub enum IBookSingleChainEventsEvents {
        FeeCombinationSetFilter(FeeCombinationSetFilter),
        SafeBlockThresholdSetFilter(SafeBlockThresholdSetFilter),
        TokenWhitelistedFilter(TokenWhitelistedFilter),
        TradeDisputeSettledFilter(TradeDisputeSettledFilter),
        TradeDisputedFilter(TradeDisputedFilter),
        TradeFilledFilter(TradeFilledFilter),
        TradeRequestedFilter(TradeRequestedFilter),
        TradeSettledFilter(TradeSettledFilter),
        UpdatedFeeForTradeFilter(UpdatedFeeForTradeFilter),
    }
    impl ethers::contract::EthLogDecode for IBookSingleChainEventsEvents {
        fn decode_log(
            log: &ethers::core::abi::RawLog,
        ) -> ::std::result::Result<Self, ethers::core::abi::Error>
        where
            Self: Sized,
        {
            if let Ok(decoded) = FeeCombinationSetFilter::decode_log(log) {
                return Ok(IBookSingleChainEventsEvents::FeeCombinationSetFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = SafeBlockThresholdSetFilter::decode_log(log) {
                return Ok(IBookSingleChainEventsEvents::SafeBlockThresholdSetFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = TokenWhitelistedFilter::decode_log(log) {
                return Ok(IBookSingleChainEventsEvents::TokenWhitelistedFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = TradeDisputeSettledFilter::decode_log(log) {
                return Ok(IBookSingleChainEventsEvents::TradeDisputeSettledFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = TradeDisputedFilter::decode_log(log) {
                return Ok(IBookSingleChainEventsEvents::TradeDisputedFilter(decoded));
            }
            if let Ok(decoded) = TradeFilledFilter::decode_log(log) {
                return Ok(IBookSingleChainEventsEvents::TradeFilledFilter(decoded));
            }
            if let Ok(decoded) = TradeRequestedFilter::decode_log(log) {
                return Ok(IBookSingleChainEventsEvents::TradeRequestedFilter(decoded));
            }
            if let Ok(decoded) = TradeSettledFilter::decode_log(log) {
                return Ok(IBookSingleChainEventsEvents::TradeSettledFilter(decoded));
            }
            if let Ok(decoded) = UpdatedFeeForTradeFilter::decode_log(log) {
                return Ok(IBookSingleChainEventsEvents::UpdatedFeeForTradeFilter(
                    decoded,
                ));
            }
            Err(ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::std::fmt::Display for IBookSingleChainEventsEvents {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                IBookSingleChainEventsEvents::FeeCombinationSetFilter(element) => element.fmt(f),
                IBookSingleChainEventsEvents::SafeBlockThresholdSetFilter(element) => {
                    element.fmt(f)
                }
                IBookSingleChainEventsEvents::TokenWhitelistedFilter(element) => element.fmt(f),
                IBookSingleChainEventsEvents::TradeDisputeSettledFilter(element) => element.fmt(f),
                IBookSingleChainEventsEvents::TradeDisputedFilter(element) => element.fmt(f),
                IBookSingleChainEventsEvents::TradeFilledFilter(element) => element.fmt(f),
                IBookSingleChainEventsEvents::TradeRequestedFilter(element) => element.fmt(f),
                IBookSingleChainEventsEvents::TradeSettledFilter(element) => element.fmt(f),
                IBookSingleChainEventsEvents::UpdatedFeeForTradeFilter(element) => element.fmt(f),
            }
        }
    }
}
