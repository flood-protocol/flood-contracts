/* Autogenerated file. Do not edit manually. */
/* tslint:disable */
/* eslint-disable */
import type {
  BaseContract,
  BigNumber,
  BigNumberish,
  BytesLike,
  Signer,
  utils,
} from "ethers";
import type { EventFragment } from "@ethersproject/abi";
import type { Listener, Provider } from "@ethersproject/providers";
import type {
  TypedEventFilter,
  TypedEvent,
  TypedListener,
  OnEvent,
  PromiseOrValue,
} from "../common";

export interface IBookSingleChainEventsInterface extends utils.Interface {
  functions: {};

  events: {
    "FeeCombinationSet(uint256,uint256,uint256)": EventFragment;
    "SafeBlockThresholdSet(uint256)": EventFragment;
    "TokenWhitelisted(address,bool)": EventFragment;
    "TradeDisputeSettled(address,uint256,bytes32,bool)": EventFragment;
    "TradeDisputed(address,uint256,bytes32,uint256)": EventFragment;
    "TradeFilled(address,uint256,uint256,uint256)": EventFragment;
    "TradeRequested(address,address,uint256,uint256,uint256,address,uint256)": EventFragment;
    "TradeSettled(address,uint256,uint256)": EventFragment;
    "UpdatedFeeForTrade(address,uint256,uint256)": EventFragment;
  };

  getEvent(nameOrSignatureOrTopic: "FeeCombinationSet"): EventFragment;
  getEvent(nameOrSignatureOrTopic: "SafeBlockThresholdSet"): EventFragment;
  getEvent(nameOrSignatureOrTopic: "TokenWhitelisted"): EventFragment;
  getEvent(nameOrSignatureOrTopic: "TradeDisputeSettled"): EventFragment;
  getEvent(nameOrSignatureOrTopic: "TradeDisputed"): EventFragment;
  getEvent(nameOrSignatureOrTopic: "TradeFilled"): EventFragment;
  getEvent(nameOrSignatureOrTopic: "TradeRequested"): EventFragment;
  getEvent(nameOrSignatureOrTopic: "TradeSettled"): EventFragment;
  getEvent(nameOrSignatureOrTopic: "UpdatedFeeForTrade"): EventFragment;
}

export interface FeeCombinationSetEventObject {
  disputeBondPct: BigNumber;
  tradeRebatePct: BigNumber;
  relayerRefundPct: BigNumber;
}
export type FeeCombinationSetEvent = TypedEvent<
  [BigNumber, BigNumber, BigNumber],
  FeeCombinationSetEventObject
>;

export type FeeCombinationSetEventFilter =
  TypedEventFilter<FeeCombinationSetEvent>;

export interface SafeBlockThresholdSetEventObject {
  newSafeBlockThreshold: BigNumber;
}
export type SafeBlockThresholdSetEvent = TypedEvent<
  [BigNumber],
  SafeBlockThresholdSetEventObject
>;

export type SafeBlockThresholdSetEventFilter =
  TypedEventFilter<SafeBlockThresholdSetEvent>;

export interface TokenWhitelistedEventObject {
  token: string;
  whitelisted: boolean;
}
export type TokenWhitelistedEvent = TypedEvent<
  [string, boolean],
  TokenWhitelistedEventObject
>;

export type TokenWhitelistedEventFilter =
  TypedEventFilter<TokenWhitelistedEvent>;

export interface TradeDisputeSettledEventObject {
  relayer: string;
  tradeIndex: BigNumber;
  disputeId: string;
  answer: boolean;
}
export type TradeDisputeSettledEvent = TypedEvent<
  [string, BigNumber, string, boolean],
  TradeDisputeSettledEventObject
>;

export type TradeDisputeSettledEventFilter =
  TypedEventFilter<TradeDisputeSettledEvent>;

export interface TradeDisputedEventObject {
  relayer: string;
  tradeIndex: BigNumber;
  disputeId: string;
  filledAtBlock: BigNumber;
}
export type TradeDisputedEvent = TypedEvent<
  [string, BigNumber, string, BigNumber],
  TradeDisputedEventObject
>;

export type TradeDisputedEventFilter = TypedEventFilter<TradeDisputedEvent>;

export interface TradeFilledEventObject {
  relayer: string;
  tradeIndex: BigNumber;
  feePct: BigNumber;
  amountOut: BigNumber;
}
export type TradeFilledEvent = TypedEvent<
  [string, BigNumber, BigNumber, BigNumber],
  TradeFilledEventObject
>;

export type TradeFilledEventFilter = TypedEventFilter<TradeFilledEvent>;

export interface TradeRequestedEventObject {
  tokenIn: string;
  tokenOut: string;
  amountIn: BigNumber;
  minAmountOut: BigNumber;
  feePct: BigNumber;
  recipient: string;
  tradeIndex: BigNumber;
}
export type TradeRequestedEvent = TypedEvent<
  [string, string, BigNumber, BigNumber, BigNumber, string, BigNumber],
  TradeRequestedEventObject
>;

export type TradeRequestedEventFilter = TypedEventFilter<TradeRequestedEvent>;

export interface TradeSettledEventObject {
  relayer: string;
  tradeIndex: BigNumber;
  filledAtBlock: BigNumber;
}
export type TradeSettledEvent = TypedEvent<
  [string, BigNumber, BigNumber],
  TradeSettledEventObject
>;

export type TradeSettledEventFilter = TypedEventFilter<TradeSettledEvent>;

export interface UpdatedFeeForTradeEventObject {
  trader: string;
  tradeIndex: BigNumber;
  newFeePct: BigNumber;
}
export type UpdatedFeeForTradeEvent = TypedEvent<
  [string, BigNumber, BigNumber],
  UpdatedFeeForTradeEventObject
>;

export type UpdatedFeeForTradeEventFilter =
  TypedEventFilter<UpdatedFeeForTradeEvent>;

export interface IBookSingleChainEvents extends BaseContract {
  connect(signerOrProvider: Signer | Provider | string): this;
  attach(addressOrName: string): this;
  deployed(): Promise<this>;

  interface: IBookSingleChainEventsInterface;

  queryFilter<TEvent extends TypedEvent>(
    event: TypedEventFilter<TEvent>,
    fromBlockOrBlockhash?: string | number | undefined,
    toBlock?: string | number | undefined
  ): Promise<Array<TEvent>>;

  listeners<TEvent extends TypedEvent>(
    eventFilter?: TypedEventFilter<TEvent>
  ): Array<TypedListener<TEvent>>;
  listeners(eventName?: string): Array<Listener>;
  removeAllListeners<TEvent extends TypedEvent>(
    eventFilter: TypedEventFilter<TEvent>
  ): this;
  removeAllListeners(eventName?: string): this;
  off: OnEvent<this>;
  on: OnEvent<this>;
  once: OnEvent<this>;
  removeListener: OnEvent<this>;

  functions: {};

  callStatic: {};

  filters: {
    "FeeCombinationSet(uint256,uint256,uint256)"(
      disputeBondPct?: null,
      tradeRebatePct?: null,
      relayerRefundPct?: null
    ): FeeCombinationSetEventFilter;
    FeeCombinationSet(
      disputeBondPct?: null,
      tradeRebatePct?: null,
      relayerRefundPct?: null
    ): FeeCombinationSetEventFilter;

    "SafeBlockThresholdSet(uint256)"(
      newSafeBlockThreshold?: null
    ): SafeBlockThresholdSetEventFilter;
    SafeBlockThresholdSet(
      newSafeBlockThreshold?: null
    ): SafeBlockThresholdSetEventFilter;

    "TokenWhitelisted(address,bool)"(
      token?: PromiseOrValue<string> | null,
      whitelisted?: null
    ): TokenWhitelistedEventFilter;
    TokenWhitelisted(
      token?: PromiseOrValue<string> | null,
      whitelisted?: null
    ): TokenWhitelistedEventFilter;

    "TradeDisputeSettled(address,uint256,bytes32,bool)"(
      relayer?: PromiseOrValue<string> | null,
      tradeIndex?: PromiseOrValue<BigNumberish> | null,
      disputeId?: PromiseOrValue<BytesLike> | null,
      answer?: null
    ): TradeDisputeSettledEventFilter;
    TradeDisputeSettled(
      relayer?: PromiseOrValue<string> | null,
      tradeIndex?: PromiseOrValue<BigNumberish> | null,
      disputeId?: PromiseOrValue<BytesLike> | null,
      answer?: null
    ): TradeDisputeSettledEventFilter;

    "TradeDisputed(address,uint256,bytes32,uint256)"(
      relayer?: PromiseOrValue<string> | null,
      tradeIndex?: PromiseOrValue<BigNumberish> | null,
      disputeId?: PromiseOrValue<BytesLike> | null,
      filledAtBlock?: null
    ): TradeDisputedEventFilter;
    TradeDisputed(
      relayer?: PromiseOrValue<string> | null,
      tradeIndex?: PromiseOrValue<BigNumberish> | null,
      disputeId?: PromiseOrValue<BytesLike> | null,
      filledAtBlock?: null
    ): TradeDisputedEventFilter;

    "TradeFilled(address,uint256,uint256,uint256)"(
      relayer?: PromiseOrValue<string> | null,
      tradeIndex?: PromiseOrValue<BigNumberish> | null,
      feePct?: null,
      amountOut?: null
    ): TradeFilledEventFilter;
    TradeFilled(
      relayer?: PromiseOrValue<string> | null,
      tradeIndex?: PromiseOrValue<BigNumberish> | null,
      feePct?: null,
      amountOut?: null
    ): TradeFilledEventFilter;

    "TradeRequested(address,address,uint256,uint256,uint256,address,uint256)"(
      tokenIn?: PromiseOrValue<string> | null,
      tokenOut?: PromiseOrValue<string> | null,
      amountIn?: null,
      minAmountOut?: null,
      feePct?: null,
      recipient?: null,
      tradeIndex?: PromiseOrValue<BigNumberish> | null
    ): TradeRequestedEventFilter;
    TradeRequested(
      tokenIn?: PromiseOrValue<string> | null,
      tokenOut?: PromiseOrValue<string> | null,
      amountIn?: null,
      minAmountOut?: null,
      feePct?: null,
      recipient?: null,
      tradeIndex?: PromiseOrValue<BigNumberish> | null
    ): TradeRequestedEventFilter;

    "TradeSettled(address,uint256,uint256)"(
      relayer?: PromiseOrValue<string> | null,
      tradeIndex?: PromiseOrValue<BigNumberish> | null,
      filledAtBlock?: null
    ): TradeSettledEventFilter;
    TradeSettled(
      relayer?: PromiseOrValue<string> | null,
      tradeIndex?: PromiseOrValue<BigNumberish> | null,
      filledAtBlock?: null
    ): TradeSettledEventFilter;

    "UpdatedFeeForTrade(address,uint256,uint256)"(
      trader?: PromiseOrValue<string> | null,
      tradeIndex?: PromiseOrValue<BigNumberish> | null,
      newFeePct?: null
    ): UpdatedFeeForTradeEventFilter;
    UpdatedFeeForTrade(
      trader?: PromiseOrValue<string> | null,
      tradeIndex?: PromiseOrValue<BigNumberish> | null,
      newFeePct?: null
    ): UpdatedFeeForTradeEventFilter;
  };

  estimateGas: {};

  populateTransaction: {};
}