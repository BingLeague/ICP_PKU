import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';
import type { IDL } from '@dfinity/candid';

export interface Account {
  'owner' : Principal,
  'subaccount' : [] | [Uint8Array | number[]],
}
export type Error = { 'Overflow' : null } |
  { 'InsufficientAllowance' : null } |
  { 'InsufficientCollateral' : null } |
  { 'ZeroAddress' : null } |
  { 'AmountLessThanFee' : null } |
  { 'TransferFailure' : null } |
  { 'InsufficientBalance' : null } |
  { 'PriceFormatError' : null } |
  { 'BorrowLessThenCost' : null } |
  { 'InvalidFee' : null } |
  { 'UnderlyingNotEnable' : null } |
  { 'Unauthorized' : null } |
  { 'InvalidRate' : null } |
  { 'NotExists' : null } |
  { 'ExceedingLoanAmount' : null } |
  { 'UnderlyingNotFound' : null } |
  { 'Exists' : null };
export interface InitArgs {
  'timer_interval_secs' : bigint,
  'staking_pool' : Principal,
}
export type Result = { 'Ok' : null } |
  { 'Err' : Error };
export type Result_1 = { 'Ok' : UnderlyingInfo } |
  { 'Err' : Error };
export interface UnderlyingInfo {
  'fee' : bigint,
  'decimals' : number,
  'need_fee' : bigint,
  'cost' : bigint,
  'name' : string,
  'borrow' : bigint,
  'deposit' : bigint,
  'price' : bigint,
  'fee_balance' : bigint,
  'liquidate_rate' : bigint,
  'enable' : boolean,
}
export interface UserAssetIdentify {
  'user' : Principal,
  'underlying' : Principal,
}
export interface UserInfo { 'balance' : bigint, 'borrow' : bigint }
export interface _SERVICE {
  'account' : ActorMethod<[], Account>,
  'addUnderlying' : ActorMethod<
    [Principal, boolean, bigint, bigint, string, bigint],
    Result
  >,
  'allBalances' : ActorMethod<[], Array<[UserAssetIdentify, UserInfo]>>,
  'balance' : ActorMethod<[Principal, Principal], UserInfo>,
  'balances' : ActorMethod<[Principal], Array<[Principal, UserInfo]>>,
  'borrow' : ActorMethod<[Principal, bigint], Result>,
  'burn' : ActorMethod<[bigint], Result>,
  'checkBorrow' : ActorMethod<[], bigint>,
  'collateralInspection' : ActorMethod<[], undefined>,
  'deposit' : ActorMethod<[Principal, bigint], Result>,
  'excessive' : ActorMethod<[], bigint>,
  'feeBalance' : ActorMethod<[], bigint>,
  'getCustodian' : ActorMethod<[], Array<Principal>>,
  'isCustodian' : ActorMethod<[Principal], boolean>,
  'maxBorrow' : ActorMethod<[Principal, Principal], bigint>,
  'repayment' : ActorMethod<[Principal, bigint], Result>,
  'setCUSD' : ActorMethod<[Principal], Result>,
  'setCustodian' : ActorMethod<[Principal, boolean], Result>,
  'setMockPrice' : ActorMethod<[Principal, bigint], Result>,
  'setReservePool' : ActorMethod<[Principal], Result>,
  'underlying' : ActorMethod<[Principal], Result_1>,
  'underlyingList' : ActorMethod<[], Array<[Principal, UnderlyingInfo]>>,
  'updatePrice' : ActorMethod<[], Result>,
  'withdraw' : ActorMethod<[Principal, bigint], Result>,
  'withdrawCusdFee' : ActorMethod<[bigint], Result>,
  'withdrawFee' : ActorMethod<[bigint], Result>,
  'withdrawUnderlyingFee' : ActorMethod<[Principal, bigint], Result>,
  'withdrawnCusdFee' : ActorMethod<[], bigint>,
}
export declare const idlFactory: IDL.InterfaceFactory;
export declare const init: (args: { IDL: typeof IDL }) => IDL.Type[];
