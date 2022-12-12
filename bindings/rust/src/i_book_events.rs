pub use i_book_events::*;
#[allow(clippy::too_many_arguments, non_camel_case_types)]
pub mod i_book_events {
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
    #[doc = "IBookEvents was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    # [rustfmt :: skip] const __ABI : & str = "[{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"feePct\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"FeePctSet\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"disputeBondPct\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"tradeRebatePct\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"relayerRefundPct\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"ParamsCombinationSet\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"newSafeBlockThreshold\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"SafeBlockThresholdSet\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"tradeIndex\",\"type\":\"uint256\",\"components\":[],\"indexed\":true},{\"internalType\":\"bytes32\",\"name\":\"tradeId\",\"type\":\"bytes32\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"trader\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"TradeCancelled\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"relayer\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"tradeIndex\",\"type\":\"uint256\",\"components\":[],\"indexed\":true},{\"internalType\":\"bytes32\",\"name\":\"disputeId\",\"type\":\"bytes32\",\"components\":[],\"indexed\":true},{\"internalType\":\"bool\",\"name\":\"answer\",\"type\":\"bool\",\"components\":[],\"indexed\":false},{\"internalType\":\"address\",\"name\":\"trader\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"TradeDisputeSettled\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"relayer\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"tradeIndex\",\"type\":\"uint256\",\"components\":[],\"indexed\":true},{\"internalType\":\"bytes32\",\"name\":\"disputeId\",\"type\":\"bytes32\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"filledAtBlock\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"address\",\"name\":\"trader\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"TradeDisputed\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"relayer\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"tradeIndex\",\"type\":\"uint256\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"amountOut\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"address\",\"name\":\"trader\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"TradeFilled\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"tokenIn\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"tokenOut\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"amountIn\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"minAmountOut\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"address\",\"name\":\"recipient\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"tradeIndex\",\"type\":\"uint256\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"trader\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"TradeRequested\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"relayer\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"tradeIndex\",\"type\":\"uint256\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"filledAtBlock\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"address\",\"name\":\"trader\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"TradeSettled\",\"outputs\":[],\"anonymous\":false}]" ;
    #[doc = r" The parsed JSON-ABI of the contract."]
    pub static IBOOKEVENTS_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            ethers::core::utils::__serde_json::from_str(__ABI).expect("invalid abi")
        });
    pub struct IBookEvents<M>(ethers::contract::Contract<M>);
    impl<M> Clone for IBookEvents<M> {
        fn clone(&self) -> Self {
            IBookEvents(self.0.clone())
        }
    }
    impl<M> std::ops::Deref for IBookEvents<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> std::fmt::Debug for IBookEvents<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(IBookEvents))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ethers::providers::Middleware> IBookEvents<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(address.into(), IBOOKEVENTS_ABI.clone(), client).into()
        }
        #[doc = "Gets the contract's `FeePctSet` event"]
        pub fn fee_pct_set_filter(&self) -> ethers::contract::builders::Event<M, FeePctSetFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `ParamsCombinationSet` event"]
        pub fn params_combination_set_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, ParamsCombinationSetFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `SafeBlockThresholdSet` event"]
        pub fn safe_block_threshold_set_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, SafeBlockThresholdSetFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `TradeCancelled` event"]
        pub fn trade_cancelled_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, TradeCancelledFilter> {
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
        #[doc = r" Returns an [`Event`](#ethers_contract::builders::Event) builder for all events of this contract"]
        pub fn events(&self) -> ethers::contract::builders::Event<M, IBookEventsEvents> {
            self.0.event_with_filter(Default::default())
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>> for IBookEvents<M> {
        fn from(contract: ethers::contract::Contract<M>) -> Self {
            Self(contract)
        }
    }
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethevent(name = "FeePctSet", abi = "FeePctSet(uint256)")]
    pub struct FeePctSetFilter {
        pub fee_pct: ethers::core::types::U256,
    }
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethevent(
        name = "ParamsCombinationSet",
        abi = "ParamsCombinationSet(uint256,uint256,uint256)"
    )]
    pub struct ParamsCombinationSetFilter {
        pub dispute_bond_pct: ethers::core::types::U256,
        pub trade_rebate_pct: ethers::core::types::U256,
        pub relayer_refund_pct: ethers::core::types::U256,
    }
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethevent(name = "SafeBlockThresholdSet", abi = "SafeBlockThresholdSet(uint256)")]
    pub struct SafeBlockThresholdSetFilter {
        pub new_safe_block_threshold: ethers::core::types::U256,
    }
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethevent(
        name = "TradeCancelled",
        abi = "TradeCancelled(uint256,bytes32,address)"
    )]
    pub struct TradeCancelledFilter {
        #[ethevent(indexed)]
        pub trade_index: ethers::core::types::U256,
        #[ethevent(indexed)]
        pub trade_id: [u8; 32],
        #[ethevent(indexed)]
        pub trader: ethers::core::types::Address,
    }
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethevent(
        name = "TradeDisputeSettled",
        abi = "TradeDisputeSettled(address,uint256,bytes32,bool,address)"
    )]
    pub struct TradeDisputeSettledFilter {
        pub relayer: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub trade_index: ethers::core::types::U256,
        #[ethevent(indexed)]
        pub dispute_id: [u8; 32],
        pub answer: bool,
        #[ethevent(indexed)]
        pub trader: ethers::core::types::Address,
    }
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethevent(
        name = "TradeDisputed",
        abi = "TradeDisputed(address,uint256,bytes32,uint256,address)"
    )]
    pub struct TradeDisputedFilter {
        pub relayer: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub trade_index: ethers::core::types::U256,
        #[ethevent(indexed)]
        pub dispute_id: [u8; 32],
        pub filled_at_block: ethers::core::types::U256,
        #[ethevent(indexed)]
        pub trader: ethers::core::types::Address,
    }
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethevent(
        name = "TradeFilled",
        abi = "TradeFilled(address,uint256,uint256,address)"
    )]
    pub struct TradeFilledFilter {
        #[ethevent(indexed)]
        pub relayer: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub trade_index: ethers::core::types::U256,
        pub amount_out: ethers::core::types::U256,
        #[ethevent(indexed)]
        pub trader: ethers::core::types::Address,
    }
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethevent(
        name = "TradeRequested",
        abi = "TradeRequested(address,address,uint256,uint256,address,uint256,address)"
    )]
    pub struct TradeRequestedFilter {
        #[ethevent(indexed)]
        pub token_in: ethers::core::types::Address,
        pub token_out: ethers::core::types::Address,
        pub amount_in: ethers::core::types::U256,
        pub min_amount_out: ethers::core::types::U256,
        pub recipient: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub trade_index: ethers::core::types::U256,
        #[ethevent(indexed)]
        pub trader: ethers::core::types::Address,
    }
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethevent(
        name = "TradeSettled",
        abi = "TradeSettled(address,uint256,uint256,address)"
    )]
    pub struct TradeSettledFilter {
        #[ethevent(indexed)]
        pub relayer: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub trade_index: ethers::core::types::U256,
        pub filled_at_block: ethers::core::types::U256,
        #[ethevent(indexed)]
        pub trader: ethers::core::types::Address,
    }
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum IBookEventsEvents {
        FeePctSetFilter(FeePctSetFilter),
        ParamsCombinationSetFilter(ParamsCombinationSetFilter),
        SafeBlockThresholdSetFilter(SafeBlockThresholdSetFilter),
        TradeCancelledFilter(TradeCancelledFilter),
        TradeDisputeSettledFilter(TradeDisputeSettledFilter),
        TradeDisputedFilter(TradeDisputedFilter),
        TradeFilledFilter(TradeFilledFilter),
        TradeRequestedFilter(TradeRequestedFilter),
        TradeSettledFilter(TradeSettledFilter),
    }
    impl ethers::contract::EthLogDecode for IBookEventsEvents {
        fn decode_log(
            log: &ethers::core::abi::RawLog,
        ) -> ::std::result::Result<Self, ethers::core::abi::Error>
        where
            Self: Sized,
        {
            if let Ok(decoded) = FeePctSetFilter::decode_log(log) {
                return Ok(IBookEventsEvents::FeePctSetFilter(decoded));
            }
            if let Ok(decoded) = ParamsCombinationSetFilter::decode_log(log) {
                return Ok(IBookEventsEvents::ParamsCombinationSetFilter(decoded));
            }
            if let Ok(decoded) = SafeBlockThresholdSetFilter::decode_log(log) {
                return Ok(IBookEventsEvents::SafeBlockThresholdSetFilter(decoded));
            }
            if let Ok(decoded) = TradeCancelledFilter::decode_log(log) {
                return Ok(IBookEventsEvents::TradeCancelledFilter(decoded));
            }
            if let Ok(decoded) = TradeDisputeSettledFilter::decode_log(log) {
                return Ok(IBookEventsEvents::TradeDisputeSettledFilter(decoded));
            }
            if let Ok(decoded) = TradeDisputedFilter::decode_log(log) {
                return Ok(IBookEventsEvents::TradeDisputedFilter(decoded));
            }
            if let Ok(decoded) = TradeFilledFilter::decode_log(log) {
                return Ok(IBookEventsEvents::TradeFilledFilter(decoded));
            }
            if let Ok(decoded) = TradeRequestedFilter::decode_log(log) {
                return Ok(IBookEventsEvents::TradeRequestedFilter(decoded));
            }
            if let Ok(decoded) = TradeSettledFilter::decode_log(log) {
                return Ok(IBookEventsEvents::TradeSettledFilter(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::std::fmt::Display for IBookEventsEvents {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                IBookEventsEvents::FeePctSetFilter(element) => element.fmt(f),
                IBookEventsEvents::ParamsCombinationSetFilter(element) => element.fmt(f),
                IBookEventsEvents::SafeBlockThresholdSetFilter(element) => element.fmt(f),
                IBookEventsEvents::TradeCancelledFilter(element) => element.fmt(f),
                IBookEventsEvents::TradeDisputeSettledFilter(element) => element.fmt(f),
                IBookEventsEvents::TradeDisputedFilter(element) => element.fmt(f),
                IBookEventsEvents::TradeFilledFilter(element) => element.fmt(f),
                IBookEventsEvents::TradeRequestedFilter(element) => element.fmt(f),
                IBookEventsEvents::TradeSettledFilter(element) => element.fmt(f),
            }
        }
    }
}
