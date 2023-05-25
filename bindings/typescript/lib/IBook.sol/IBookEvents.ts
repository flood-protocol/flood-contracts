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

export interface IBookEventsInterface extends utils.Interface {
  functions: {};

  events: {
    "FeePctSet(uint256)": EventFragment;
    "TradeCancelled(uint256,bytes32,address)": EventFragment;
    "TradeFilled(address,uint256,uint128,address)": EventFragment;
    "TradeRequested(address[],uint128[],address,uint256,address,bool,bool)": EventFragment;
    "TradeSettled(address,uint256,uint256,address)": EventFragment;
  };

  getEvent(nameOrSignatureOrTopic: "FeePctSet"): EventFragment;
  getEvent(nameOrSignatureOrTopic: "TradeCancelled"): EventFragment;
  getEvent(nameOrSignatureOrTopic: "TradeFilled"): EventFragment;
  getEvent(nameOrSignatureOrTopic: "TradeRequested"): EventFragment;
  getEvent(nameOrSignatureOrTopic: "TradeSettled"): EventFragment;
}

export interface FeePctSetEventObject {
  feePct: BigNumber;
}
export type FeePctSetEvent = TypedEvent<[BigNumber], FeePctSetEventObject>;

export type FeePctSetEventFilter = TypedEventFilter<FeePctSetEvent>;

export interface TradeCancelledEventObject {
  tradeIndex: BigNumber;
  tradeId: string;
  trader: string;
}
export type TradeCancelledEvent = TypedEvent<
  [BigNumber, string, string],
  TradeCancelledEventObject
>;

export type TradeCancelledEventFilter = TypedEventFilter<TradeCancelledEvent>;

export interface TradeFilledEventObject {
  relayer: string;
  tradeIndex: BigNumber;
  amountOut: BigNumber;
  trader: string;
}
export type TradeFilledEvent = TypedEvent<
  [string, BigNumber, BigNumber, string],
  TradeFilledEventObject
>;

export type TradeFilledEventFilter = TypedEventFilter<TradeFilledEvent>;

export interface TradeRequestedEventObject {
  tokens: string[];
  amounts: BigNumber[];
  recipient: string;
  tradeIndex: BigNumber;
  trader: string;
  unwrapOutput: boolean;
  wrapInput: boolean;
}
export type TradeRequestedEvent = TypedEvent<
  [string[], BigNumber[], string, BigNumber, string, boolean, boolean],
  TradeRequestedEventObject
>;

export type TradeRequestedEventFilter = TypedEventFilter<TradeRequestedEvent>;

export interface TradeSettledEventObject {
  relayer: string;
  tradeIndex: BigNumber;
  filledAtBlock: BigNumber;
  trader: string;
}
export type TradeSettledEvent = TypedEvent<
  [string, BigNumber, BigNumber, string],
  TradeSettledEventObject
>;

export type TradeSettledEventFilter = TypedEventFilter<TradeSettledEvent>;

export interface IBookEvents extends BaseContract {
  connect(signerOrProvider: Signer | Provider | string): this;
  attach(addressOrName: string): this;
  deployed(): Promise<this>;

  interface: IBookEventsInterface;

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
    "FeePctSet(uint256)"(feePct?: null): FeePctSetEventFilter;
    FeePctSet(feePct?: null): FeePctSetEventFilter;

    "TradeCancelled(uint256,bytes32,address)"(
      tradeIndex?: PromiseOrValue<BigNumberish> | null,
      tradeId?: PromiseOrValue<BytesLike> | null,
      trader?: PromiseOrValue<string> | null
    ): TradeCancelledEventFilter;
    TradeCancelled(
      tradeIndex?: PromiseOrValue<BigNumberish> | null,
      tradeId?: PromiseOrValue<BytesLike> | null,
      trader?: PromiseOrValue<string> | null
    ): TradeCancelledEventFilter;

    "TradeFilled(address,uint256,uint128,address)"(
      relayer?: PromiseOrValue<string> | null,
      tradeIndex?: PromiseOrValue<BigNumberish> | null,
      amountOut?: null,
      trader?: PromiseOrValue<string> | null
    ): TradeFilledEventFilter;
    TradeFilled(
      relayer?: PromiseOrValue<string> | null,
      tradeIndex?: PromiseOrValue<BigNumberish> | null,
      amountOut?: null,
      trader?: PromiseOrValue<string> | null
    ): TradeFilledEventFilter;

    "TradeRequested(address[],uint128[],address,uint256,address,bool,bool)"(
      tokens?: null,
      amounts?: null,
      recipient?: null,
      tradeIndex?: PromiseOrValue<BigNumberish> | null,
      trader?: PromiseOrValue<string> | null,
      unwrapOutput?: null,
      wrapInput?: null
    ): TradeRequestedEventFilter;
    TradeRequested(
      tokens?: null,
      amounts?: null,
      recipient?: null,
      tradeIndex?: PromiseOrValue<BigNumberish> | null,
      trader?: PromiseOrValue<string> | null,
      unwrapOutput?: null,
      wrapInput?: null
    ): TradeRequestedEventFilter;

    "TradeSettled(address,uint256,uint256,address)"(
      relayer?: PromiseOrValue<string> | null,
      tradeIndex?: PromiseOrValue<BigNumberish> | null,
      filledAtBlock?: null,
      trader?: PromiseOrValue<string> | null
    ): TradeSettledEventFilter;
    TradeSettled(
      relayer?: PromiseOrValue<string> | null,
      tradeIndex?: PromiseOrValue<BigNumberish> | null,
      filledAtBlock?: null,
      trader?: PromiseOrValue<string> | null
    ): TradeSettledEventFilter;
  };

  estimateGas: {};

  populateTransaction: {};
}