import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';
import type { IDL } from '@dfinity/candid';

export interface Account {
  'owner' : Principal,
  'subaccount' : [] | [Uint8Array | number[]],
}
export interface AssetConfig {
  'apy' : bigint,
  'balance' : bigint,
  'reward_rate' : bigint,
  'reward_per_token_stored' : bigint,
  'price' : bigint,
  'last_update' : bigint,
}
export interface AssetInfo {
  'reward' : bigint,
  'balance' : bigint,
  'withdrawn' : bigint,
  'reward_per_token_paid' : bigint,
}
export interface DepositInfo {
  'status' : Status,
  'period_end' : bigint,
  'balance' : bigint,
  'period' : number,
  'period_start' : bigint,
  'created_at' : bigint,
  'earned' : bigint,
  'auto_investment' : boolean,
  'last_update' : bigint,
}
export type Error = { 'Invalid' : null } |
  { 'InsufficientAllowance' : null } |
  { 'ZeroAddress' : null } |
  { 'TransferFailure' : null } |
  { 'OnlyVault' : null } |
  { 'InsufficientBalance' : null } |
  { 'PeriodError' : null } |
  { 'AssetRewardRateZero' : null } |
  { 'LessThanMinAmount' : null } |
  { 'InStaking' : null } |
  { 'BalanceIsZero' : null };
export interface InitArgs { 'timer_interval_secs' : bigint }
export interface PeriodConfig {
  'apy' : bigint,
  'balance' : bigint,
  'min_amount' : bigint,
  'last_update' : bigint,
  'daily_reward' : bigint,
}
export type Result = { 'Ok' : null } |
  { 'Err' : Error };
export type Result_1 = { 'Ok' : bigint } |
  { 'Err' : Error };
export type Status = { 'Invalid' : null } |
  { 'Valid' : null };
export interface UserAssetIdentify {
  'user' : Principal,
  'underlying' : Account,
}
export interface UserPeriodIdentify { 'user' : Principal, 'index' : bigint }
export interface _SERVICE {
  'allClptBalances' : ActorMethod<
    [],
    Array<[UserPeriodIdentify, DepositInfo]>
  >,
  'allUserBalances' : ActorMethod<[], Array<[UserAssetIdentify, AssetInfo]>>,
  'apr' : ActorMethod<[Account], bigint>,
  'assetConfig' : ActorMethod<[], Array<[Account, AssetConfig]>>,
  'autoInvestment' : ActorMethod<[bigint, boolean], Result>,
  'balance' : ActorMethod<[Principal], bigint>,
  'balances' : ActorMethod<[UserAssetIdentify], AssetInfo>,
  'clptBalances' : ActorMethod<[Principal], Array<[bigint, DepositInfo]>>,
  'collateralInspection' : ActorMethod<[], undefined>,
  'DepositInfo' : ActorMethod<[UserPeriodIdentify], DepositInfo>,
  'paramInit' : ActorMethod<[Principal, Principal, Principal], undefined>,
  'periodConfig' : ActorMethod<[], Array<[number, PeriodConfig]>>,
  'setAssetConfig' : ActorMethod<[Account, bigint, bigint], Result>,
  'setClptPrice' : ActorMethod<[bigint], undefined>,
  'setPeriodConfig' : ActorMethod<[number, bigint, bigint], Result>,
  'setTokenPrice' : ActorMethod<[Account, bigint], Result>,
  'stake' : ActorMethod<[number, bigint, boolean], Result>,
  'unstake' : ActorMethod<[bigint], Result>,
  'update_balance' : ActorMethod<[Principal, Account, bigint, boolean], Result>,
  'withdraw' : ActorMethod<[], Result_1>,
}
export declare const idlFactory: IDL.InterfaceFactory;
export declare const init: (args: { IDL: typeof IDL }) => IDL.Type[];
