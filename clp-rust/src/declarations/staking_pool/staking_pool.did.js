export const idlFactory = ({ IDL }) => {
  const InitArgs = IDL.Record({ 'timer_interval_secs' : IDL.Nat64 });
  const UserPeriodIdentify = IDL.Record({
    'user' : IDL.Principal,
    'index' : IDL.Nat,
  });
  const Status = IDL.Variant({ 'Invalid' : IDL.Null, 'Valid' : IDL.Null });
  const DepositInfo = IDL.Record({
    'status' : Status,
    'period_end' : IDL.Nat64,
    'balance' : IDL.Nat,
    'period' : IDL.Nat8,
    'period_start' : IDL.Nat64,
    'created_at' : IDL.Nat64,
    'earned' : IDL.Nat,
    'auto_investment' : IDL.Bool,
    'last_update' : IDL.Nat64,
  });
  const Account = IDL.Record({
    'owner' : IDL.Principal,
    'subaccount' : IDL.Opt(IDL.Vec(IDL.Nat8)),
  });
  const UserAssetIdentify = IDL.Record({
    'user' : IDL.Principal,
    'underlying' : Account,
  });
  const AssetInfo = IDL.Record({
    'reward' : IDL.Nat,
    'balance' : IDL.Nat,
    'withdrawn' : IDL.Nat,
    'reward_per_token_paid' : IDL.Nat,
  });
  const AssetConfig = IDL.Record({
    'apy' : IDL.Nat,
    'balance' : IDL.Nat,
    'reward_rate' : IDL.Nat,
    'reward_per_token_stored' : IDL.Nat,
    'price' : IDL.Nat,
    'last_update' : IDL.Nat64,
  });
  const Error = IDL.Variant({
    'Invalid' : IDL.Null,
    'InsufficientAllowance' : IDL.Null,
    'ZeroAddress' : IDL.Null,
    'TransferFailure' : IDL.Null,
    'OnlyVault' : IDL.Null,
    'InsufficientBalance' : IDL.Null,
    'PeriodError' : IDL.Null,
    'AssetRewardRateZero' : IDL.Null,
    'LessThanMinAmount' : IDL.Null,
    'InStaking' : IDL.Null,
    'BalanceIsZero' : IDL.Null,
  });
  const Result = IDL.Variant({ 'Ok' : IDL.Null, 'Err' : Error });
  const PeriodConfig = IDL.Record({
    'apy' : IDL.Nat,
    'balance' : IDL.Nat,
    'min_amount' : IDL.Nat,
    'last_update' : IDL.Nat64,
    'daily_reward' : IDL.Nat,
  });
  const Result_1 = IDL.Variant({ 'Ok' : IDL.Nat, 'Err' : Error });
  return IDL.Service({
    'allClptBalances' : IDL.Func(
        [],
        [IDL.Vec(IDL.Tuple(UserPeriodIdentify, DepositInfo))],
        ['query'],
      ),
    'allUserBalances' : IDL.Func(
        [],
        [IDL.Vec(IDL.Tuple(UserAssetIdentify, AssetInfo))],
        ['query'],
      ),
    'apr' : IDL.Func([Account], [IDL.Nat], ['query']),
    'assetConfig' : IDL.Func(
        [],
        [IDL.Vec(IDL.Tuple(Account, AssetConfig))],
        ['query'],
      ),
    'autoInvestment' : IDL.Func([IDL.Nat, IDL.Bool], [Result], []),
    'balance' : IDL.Func([IDL.Principal], [IDL.Nat], ['query']),
    'balances' : IDL.Func([UserAssetIdentify], [AssetInfo], ['query']),
    'clptBalances' : IDL.Func(
        [IDL.Principal],
        [IDL.Vec(IDL.Tuple(IDL.Nat, DepositInfo))],
        ['query'],
      ),
    'collateralInspection' : IDL.Func([], [], []),
    'DepositInfo' : IDL.Func([UserPeriodIdentify], [DepositInfo], ['query']),
    'paramInit' : IDL.Func(
        [IDL.Principal, IDL.Principal, IDL.Principal],
        [],
        [],
      ),
    'periodConfig' : IDL.Func(
        [],
        [IDL.Vec(IDL.Tuple(IDL.Nat8, PeriodConfig))],
        ['query'],
      ),
    'setAssetConfig' : IDL.Func([Account, IDL.Nat, IDL.Nat], [Result], []),
    'setClptPrice' : IDL.Func([IDL.Nat], [], []),
    'setPeriodConfig' : IDL.Func([IDL.Nat8, IDL.Nat, IDL.Nat], [Result], []),
    'setTokenPrice' : IDL.Func([Account, IDL.Nat], [Result], []),
    'stake' : IDL.Func([IDL.Nat8, IDL.Nat, IDL.Bool], [Result], []),
    'unstake' : IDL.Func([IDL.Nat], [Result], []),
    'update_balance' : IDL.Func(
        [IDL.Principal, Account, IDL.Nat, IDL.Bool],
        [Result],
        [],
      ),
    'withdraw' : IDL.Func([], [Result_1], []),
  });
};
export const init = ({ IDL }) => {
  const InitArgs = IDL.Record({ 'timer_interval_secs' : IDL.Nat64 });
  return [InitArgs];
};
