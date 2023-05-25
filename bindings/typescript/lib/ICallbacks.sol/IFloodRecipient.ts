/* Autogenerated file. Do not edit manually. */
/* tslint:disable */
/* eslint-disable */
import type {
  BaseContract,
  BigNumber,
  BigNumberish,
  BytesLike,
  CallOverrides,
  ContractTransaction,
  Overrides,
  PopulatedTransaction,
  Signer,
  utils,
} from "ethers";
import type { FunctionFragment, Result } from "@ethersproject/abi";
import type { Listener, Provider } from "@ethersproject/providers";
import type {
  TypedEventFilter,
  TypedEvent,
  TypedListener,
  OnEvent,
  PromiseOrValue,
} from "../common";

export interface IFloodRecipientInterface extends utils.Interface {
  functions: {
    "onTradeFilled(address,address[],uint128[],uint128,bytes32)": FunctionFragment;
  };

  getFunction(nameOrSignatureOrTopic: "onTradeFilled"): FunctionFragment;

  encodeFunctionData(
    functionFragment: "onTradeFilled",
    values: [
      PromiseOrValue<string>,
      PromiseOrValue<string>[],
      PromiseOrValue<BigNumberish>[],
      PromiseOrValue<BigNumberish>,
      PromiseOrValue<BytesLike>
    ]
  ): string;

  decodeFunctionResult(
    functionFragment: "onTradeFilled",
    data: BytesLike
  ): Result;

  events: {};
}

export interface IFloodRecipient extends BaseContract {
  connect(signerOrProvider: Signer | Provider | string): this;
  attach(addressOrName: string): this;
  deployed(): Promise<this>;

  interface: IFloodRecipientInterface;

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

  functions: {
    onTradeFilled(
      trader: PromiseOrValue<string>,
      tokens: PromiseOrValue<string>[],
      amounts: PromiseOrValue<BigNumberish>[],
      amountReceived: PromiseOrValue<BigNumberish>,
      tradeId: PromiseOrValue<BytesLike>,
      overrides?: Overrides & { from?: PromiseOrValue<string> }
    ): Promise<ContractTransaction>;
  };

  onTradeFilled(
    trader: PromiseOrValue<string>,
    tokens: PromiseOrValue<string>[],
    amounts: PromiseOrValue<BigNumberish>[],
    amountReceived: PromiseOrValue<BigNumberish>,
    tradeId: PromiseOrValue<BytesLike>,
    overrides?: Overrides & { from?: PromiseOrValue<string> }
  ): Promise<ContractTransaction>;

  callStatic: {
    onTradeFilled(
      trader: PromiseOrValue<string>,
      tokens: PromiseOrValue<string>[],
      amounts: PromiseOrValue<BigNumberish>[],
      amountReceived: PromiseOrValue<BigNumberish>,
      tradeId: PromiseOrValue<BytesLike>,
      overrides?: CallOverrides
    ): Promise<void>;
  };

  filters: {};

  estimateGas: {
    onTradeFilled(
      trader: PromiseOrValue<string>,
      tokens: PromiseOrValue<string>[],
      amounts: PromiseOrValue<BigNumberish>[],
      amountReceived: PromiseOrValue<BigNumberish>,
      tradeId: PromiseOrValue<BytesLike>,
      overrides?: Overrides & { from?: PromiseOrValue<string> }
    ): Promise<BigNumber>;
  };

  populateTransaction: {
    onTradeFilled(
      trader: PromiseOrValue<string>,
      tokens: PromiseOrValue<string>[],
      amounts: PromiseOrValue<BigNumberish>[],
      amountReceived: PromiseOrValue<BigNumberish>,
      tradeId: PromiseOrValue<BytesLike>,
      overrides?: Overrides & { from?: PromiseOrValue<string> }
    ): Promise<PopulatedTransaction>;
  };
}