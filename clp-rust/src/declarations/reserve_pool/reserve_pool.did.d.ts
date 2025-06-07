import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';
import type { IDL } from '@dfinity/candid';

export interface AssetInfo {
  'asset_add' : bigint,
  'wait_withdrawal' : bigint,
  'cusd_sub' : bigint,
}
export type Error = { 'InsufficientAllowance' : null } |
  { 'ZeroAddress' : null } |
  { 'AmountLessThanFee' : null } |
  { 'TransferFailure' : null } |
  { 'OnlyVault' : null } |
  { 'InsufficientBalance' : null } |
  { 'InvalidFee' : null } |
  { 'Exists' : null };
export interface InitArgs {
  'vault' : Principal,
  'cusd' : Principal,
  'staking_pool' : Principal,
}
export type Result = { 'Ok' : null } |
  { 'Err' : Error };
export interface UserInfo {
  'underlies' : Array<[Principal, AssetInfo]>,
  'balance' : bigint,
  'total_sub' : bigint,
}
export interface _SERVICE {
  'deposit' : ActorMethod<[bigint], Result>,
  'liquidate' : ActorMethod<[bigint, bigint, Principal], bigint>,
  'liquidateUnderlying' : ActorMethod<[Principal], bigint>,
  'setFee' : ActorMethod<[bigint], Result>,
  'stats' : ActorMethod<[], [bigint, bigint]>,
  'userBalance' : ActorMethod<[Principal], bigint>,
  'userInfo' : ActorMethod<[Principal], UserInfo>,
  'userUnderlyingBalance' : ActorMethod<[Principal, Principal], bigint>,
  'userUnderlyingBalances' : ActorMethod<
    [Principal],
    Array<[Principal, bigint]>
  >,
  'withdraw' : ActorMethod<[bigint], Result>,
  'withdrawLiquidateUnderlying' : ActorMethod<[Principal, bigint], Result>,
  'withdrawUnderlying' : ActorMethod<[Principal, bigint], Result>,
}
export declare const idlFactory: IDL.InterfaceFactory;
export declare const init: (args: { IDL: typeof IDL }) => IDL.Type[];
