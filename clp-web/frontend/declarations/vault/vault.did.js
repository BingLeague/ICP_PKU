export const idlFactory = ({ IDL }) => {
  const InitArgs = IDL.Record({
    'timer_interval_secs' : IDL.Nat64,
    'staking_pool' : IDL.Principal,
  });
  const Error = IDL.Variant({
    'Overflow' : IDL.Null,
    'InsufficientAllowance' : IDL.Null,
    'InsufficientCollateral' : IDL.Null,
    'ZeroAddress' : IDL.Null,
    'AmountLessThanFee' : IDL.Null,
    'TransferFailure' : IDL.Null,
    'InsufficientBalance' : IDL.Null,
    'PriceFormatError' : IDL.Null,
    'BorrowLessThenCost' : IDL.Null,
    'InvalidFee' : IDL.Null,
    'UnderlyingNotEnable' : IDL.Null,
    'Unauthorized' : IDL.Null,
    'InvalidRate' : IDL.Null,
    'NotExists' : IDL.Null,
    'ExceedingLoanAmount' : IDL.Null,
    'UnderlyingNotFound' : IDL.Null,
    'Exists' : IDL.Null,
  });
  const Result = IDL.Variant({ 'Ok' : IDL.Null, 'Err' : Error });
  const UserAssetIdentify = IDL.Record({
    'user' : IDL.Principal,
    'underlying' : IDL.Principal,
  });
  const UserInfo = IDL.Record({ 'balance' : IDL.Nat, 'borrow' : IDL.Nat });
  const UnderlyingInfo = IDL.Record({
    'fee' : IDL.Nat,
    'decimals' : IDL.Nat8,
    'need_fee' : IDL.Nat,
    'cost' : IDL.Nat,
    'name' : IDL.Text,
    'borrow' : IDL.Nat,
    'deposit' : IDL.Nat,
    'price' : IDL.Nat,
    'fee_balance' : IDL.Nat,
    'liquidate_rate' : IDL.Nat,
    'enable' : IDL.Bool,
  });
  const Result_1 = IDL.Variant({ 'Ok' : UnderlyingInfo, 'Err' : Error });
  return IDL.Service({
    'addUnderlying' : IDL.Func(
        [IDL.Principal, IDL.Bool, IDL.Nat, IDL.Nat, IDL.Text, IDL.Nat],
        [Result],
        [],
      ),
    'allBalances' : IDL.Func(
        [],
        [IDL.Vec(IDL.Tuple(UserAssetIdentify, UserInfo))],
        ['query'],
      ),
    'balance' : IDL.Func([IDL.Principal, IDL.Principal], [UserInfo], ['query']),
    'balances' : IDL.Func(
        [IDL.Principal],
        [IDL.Vec(IDL.Tuple(IDL.Principal, UserInfo))],
        ['query'],
      ),
    'borrow' : IDL.Func([IDL.Principal, IDL.Nat], [Result], []),
    'burn' : IDL.Func([IDL.Nat], [Result], []),
    'checkBorrow' : IDL.Func([], [IDL.Nat], []),
    'collateralInspection' : IDL.Func([], [], []),
    'deposit' : IDL.Func([IDL.Principal, IDL.Nat], [Result], []),
    'excessive' : IDL.Func([], [IDL.Nat], ['query']),
    'feeBalance' : IDL.Func([], [IDL.Nat], ['query']),
    'getCustodian' : IDL.Func([], [IDL.Vec(IDL.Principal)], ['query']),
    'isCustodian' : IDL.Func([IDL.Principal], [IDL.Bool], ['query']),
    'maxBorrow' : IDL.Func(
        [IDL.Principal, IDL.Principal],
        [IDL.Nat],
        ['query'],
      ),
    'repayment' : IDL.Func([IDL.Principal, IDL.Nat], [Result], []),
    'setCUSD' : IDL.Func([IDL.Principal], [Result], []),
    'setCustodian' : IDL.Func([IDL.Principal, IDL.Bool], [Result], []),
    'setMockPrice' : IDL.Func([IDL.Principal, IDL.Nat], [Result], []),
    'setReservePool' : IDL.Func([IDL.Principal], [Result], []),
    'underlying' : IDL.Func([IDL.Principal], [Result_1], ['query']),
    'underlyingList' : IDL.Func(
        [],
        [IDL.Vec(IDL.Tuple(IDL.Principal, UnderlyingInfo))],
        ['query'],
      ),
    'updatePrice' : IDL.Func([], [Result], []),
    'withdraw' : IDL.Func([IDL.Principal, IDL.Nat], [Result], []),
    'withdrawCusdFee' : IDL.Func([IDL.Nat], [Result], []),
    'withdrawFee' : IDL.Func([IDL.Nat], [Result], []),
    'withdrawUnderlyingFee' : IDL.Func([IDL.Principal, IDL.Nat], [Result], []),
    'withdrawedCusdFee' : IDL.Func([], [IDL.Nat], ['query']),
  });
};
export const init = ({ IDL }) => {
  const InitArgs = IDL.Record({
    'timer_interval_secs' : IDL.Nat64,
    'staking_pool' : IDL.Principal,
  });
  return [InitArgs];
};
