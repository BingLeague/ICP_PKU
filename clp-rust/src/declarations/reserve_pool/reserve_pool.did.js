export const idlFactory = ({ IDL }) => {
  const InitArgs = IDL.Record({
    'vault' : IDL.Principal,
    'cusd' : IDL.Principal,
    'staking_pool' : IDL.Principal,
  });
  const Error = IDL.Variant({
    'InsufficientAllowance' : IDL.Null,
    'ZeroAddress' : IDL.Null,
    'AmountLessThanFee' : IDL.Null,
    'TransferFailure' : IDL.Null,
    'OnlyVault' : IDL.Null,
    'InsufficientBalance' : IDL.Null,
    'InvalidFee' : IDL.Null,
    'Exists' : IDL.Null,
  });
  const Result = IDL.Variant({ 'Ok' : IDL.Null, 'Err' : Error });
  const AssetInfo = IDL.Record({
    'asset_add' : IDL.Nat,
    'wait_withdrawal' : IDL.Nat,
    'cusd_sub' : IDL.Nat,
  });
  const UserInfo = IDL.Record({
    'underlies' : IDL.Vec(IDL.Tuple(IDL.Principal, AssetInfo)),
    'balance' : IDL.Nat,
    'total_sub' : IDL.Nat,
  });
  return IDL.Service({
    'deposit' : IDL.Func([IDL.Nat], [Result], []),
    'liquidate' : IDL.Func([IDL.Nat, IDL.Nat, IDL.Principal], [IDL.Nat], []),
    'liquidateUnderlying' : IDL.Func([IDL.Principal], [IDL.Nat], ['query']),
    'setFee' : IDL.Func([IDL.Nat], [Result], []),
    'stats' : IDL.Func([], [IDL.Nat, IDL.Nat], ['query']),
    'userBalance' : IDL.Func([IDL.Principal], [IDL.Nat], ['query']),
    'userInfo' : IDL.Func([IDL.Principal], [UserInfo], ['query']),
    'userUnderlyingBalance' : IDL.Func(
        [IDL.Principal, IDL.Principal],
        [IDL.Nat],
        ['query'],
      ),
    'userUnderlyingBalances' : IDL.Func(
        [IDL.Principal],
        [IDL.Vec(IDL.Tuple(IDL.Principal, IDL.Nat))],
        ['query'],
      ),
    'withdraw' : IDL.Func([IDL.Nat], [Result], []),
    'withdrawLiquidateUnderlying' : IDL.Func(
        [IDL.Principal, IDL.Nat],
        [Result],
        [],
      ),
    'withdrawUnderlying' : IDL.Func([IDL.Principal, IDL.Nat], [Result], []),
  });
};
export const init = ({ IDL }) => {
  const InitArgs = IDL.Record({
    'vault' : IDL.Principal,
    'cusd' : IDL.Principal,
    'staking_pool' : IDL.Principal,
  });
  return [InitArgs];
};
