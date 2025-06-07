use crate::xrc::XRC;
use crate::UserInfo;
use crate::STATE;
use candid::{Nat, Principal};
use ic_cdk_macros::*;
use ic_xrc_types::{Asset, AssetClass, GetExchangeRateRequest};
use regex::Regex;
use shard_lib::token::{Token, TransferArgsICRC1};
use shard_lib::types::*;
use std::collections::HashMap;
use std::vec;

#[update(name = "collateralInspection")]
pub async fn collateral_inspection() {
    ic_cdk::println!("collateral_inspection");
    let result = update_price().await;
    match result {
        Ok(_) => {}
        Err(e) => {
            ic_cdk::println!("update price error: {:?}", e);
        }
    }
    check_borrow().await;
}

fn filter_liquidate_user(
    underlying_updates: &mut HashMap<Principal, (u128, u128, u128, Vec<Principal>)>,
) -> Principal {
    STATE.with(|state| {
        let state = state.borrow();
        for (identify, info) in state.balances.iter() {
            if info.borrow == 0 {
                continue;
            }
            let asset  = identify.underlying;
            let user = identify.user;
            let underlying_info = state.underlying.get(&asset).unwrap();
            if !underlying_info.enable {
                continue;
            }
            let decimals = underlying_info.decimals;
            let liquidate_amount = calculate_liquidate_amount(
                info.balance,
                underlying_info.price,
                underlying_info.liquidate_rate,
                decimals,
            );
            match liquidate_amount {
                Ok(liquidate_amount) => {
                    if info.borrow <= liquidate_amount {
                        continue;
                    }
                    ic_cdk::println!(
                        "user liquidate: {:?} token:{} borrow: {} > liquidate_amount: {} balance: {}",
                        user.to_text(),
                        underlying_info.name,
                        info.borrow,
                        liquidate_amount,
                        info.balance
                    );
                    let update = underlying_updates.entry(asset).or_insert((0, 0,underlying_info.fee,vec![]));
                    update.0 += info.balance;
                    update.1 += info.borrow;
                    update.3.push(user.clone());
                }
                Err(_) => {
                    ic_cdk::println!(
                        "user: {:?} token:{} borrow: {} balance: {} liquidate overflow",
                        user.to_text(),
                        underlying_info.name,
                        info.borrow,
                        info.balance
                    );
                }
            }
        }
        state.reserve_pool.unwrap()
    })
}

#[update(name = "checkBorrow")]
async fn check_borrow() -> u128 {
    let mut underlying_updates: HashMap<Principal, (u128, u128, u128, Vec<Principal>)> =
        HashMap::new();
    let mut total = 0;
    let reserve_pool = filter_liquidate_user(&mut underlying_updates);
    for (asset, (deposit_underlying, borrow_cusd, fee, users)) in underlying_updates {
        if deposit_underlying < fee {
            ic_cdk::println!(
                "token:{} InsufficientBalance pay fee balance: {}",
                asset.to_text(),
                deposit_underlying,
            );
            continue;
        }
        Token::new(asset.clone())
            .transfer(TransferArgsICRC1 {
                amount: Nat::from(deposit_underlying - fee),
                created_at_time: None,
                fee: None,
                from_subaccount: None,
                memo: None,
                to: Account {
                    owner: reserve_pool,
                    subaccount: None,
                },
            })
            .await
            .unwrap();
        let args: (u128, u128, Principal) = (borrow_cusd, deposit_underlying - fee, asset.clone());
        let amount = _liquidate(reserve_pool, args).await;
        STATE.with(|state| {
            let mut state = state.borrow_mut();
            let underlying_info = state.underlying.get_mut(&asset).unwrap();
            underlying_info.deposit -= deposit_underlying;
            underlying_info.borrow -= borrow_cusd;
            if amount < borrow_cusd {
                state.excessive_cusd += borrow_cusd - amount;
            }
            for user in users.into_iter() {
                let identify = UserAssetIdentify {
                    user,
                    underlying: asset,
                };
                state.balances.insert(identify, UserInfo::default());
            }
            total += borrow_cusd;
        });
    }
    total
}

pub async fn _liquidate(addr: Principal, args: (u128, u128, Principal)) -> u128 {
    let call_result: Result<(u128,), _> = ic_cdk::api::call::call(addr, "liquidate", args).await;
    call_result.unwrap().0
}

pub fn calculate_liquidate_amount(
    balance: u128,
    price: u128,
    liquidate_rate: u128,
    decimals: u8,
) -> Result<u128, Error> {
    // balance * price * liquidate_rate / 100 / 10 ^ decimals
    balance
        .checked_mul(price)
        .ok_or(Error::Overflow)?
        .checked_mul(liquidate_rate)
        .ok_or(Error::Overflow)?
        .checked_div(10u128.pow(u32::from(decimals + 2)))
        .ok_or(Error::Overflow)
}

fn replace_ck_symbols(input: &str) -> String {
    // 创建一个正则表达式，匹配 "ckBTC" 或 "ckETH"
    let re = Regex::new(r"ck(BTC|ETH)").unwrap();
    // 使用正则表达式替换匹配到的部分
    re.replace_all(input, "$1").to_string()
}

fn convert_rate(rate: u128, from_decimals: u32, to_decimals: u8) -> u128 {
    if from_decimals > to_decimals as u32 {
        // 如果原始小数位数大于目标小数位数，缩小数值
        let factor = 10u128.pow((from_decimals - to_decimals as u32) as u32);
        rate / factor
    } else {
        // 如果原始小数位数小于或等于目标小数位数，放大数值
        let factor = 10u128.pow((to_decimals as u32 - from_decimals) as u32);
        rate * factor
    }
}

#[update(name = "updatePrice")]
async fn update_price() -> Result<()> {
    let underlies = STATE.with(|s| s.borrow().underlying.clone());
    let xrc = XRC::new();
    for (token, info) in underlies.iter() {
        let ret = xrc
            .get_exchange_rate(GetExchangeRateRequest {
                base_asset: Asset {
                    symbol: replace_ck_symbols(&info.name),
                    class: AssetClass::Cryptocurrency,
                },
                quote_asset: Asset {
                    symbol: "USDT".to_string(),
                    class: AssetClass::Cryptocurrency,
                },
                timestamp: None,
            })
            .await
            .unwrap();
        ic_cdk::println!("update {} price {}", info.name.clone(), ret.rate,);
        STATE.with(|s| {
            let decimal = ret.metadata.decimals;
            let rate_in_target_decimal = convert_rate(ret.rate as u128, decimal, 8);
            s.borrow_mut().underlying.get_mut(token).unwrap().price = rate_in_target_decimal;
        })
    }
    //dfx canister call --with-cycles 10000000000 uf6dk-hyaaa-aaaaq-qaaaq-cai get_exchange_rate '(record { base_asset = record { symbol = "BTC"; class = variant { Cryptocurrency } }; quote_asset = record { symbol = "USD"; class = variant { FiatCurrency } } })'
    // let result: Vec<(candid::Principal, HashMap<Platform, PriceInfo>)> = STATE.with(|state| {
    //     state
    //         .borrow()
    //         .underlying
    //         .iter()
    //         .map(|(underlying, info)| (underlying.clone(), info.prices.clone()))
    //         .collect()
    // });
    // let now = ic_cdk::api::time()/SECONDS;
    // for (underlying, prices) in result {
    //     for (platform, info) in prices {
    //         match platform.get_price(&info.name).await {
    //             Ok(price) => {
    //                 let price_f64: f64 = price
    //                     .parse()
    //                     .map_err(|_: std::num::ParseFloatError| Error::PriceFormatError)?;
    //                 let scaled_price = price_f64 * 10_f64.powi(8);
    //                 if !(scaled_price >= 0.0 && scaled_price <= u128::MAX as f64) {
    //                     ic_cdk::println!(
    //                         "token:{} platform:{:?} price:{} out of range",
    //                         &info.name,
    //                         platform,
    //                         price
    //                     );
    //                     continue;
    //                 }
    //                 STATE.with(|state| {
    //                     if let Some(price_info) =
    //                         state.borrow_mut().underlying.get_mut(&underlying).and_then(
    //                             |underlying_info| underlying_info.prices.get_mut(&platform),
    //                         )
    //                     {
    //                         let scaled_price = scaled_price as u128;
    //                         ic_cdk::println!(
    //                             "update price: {} -> {:?}: {}",
    //                             info.name,
    //                             platform,
    //                             price_f64
    //                         );
    //                         price_info.laste_update_at = now;
    //                         price_info.price = scaled_price;
    //                     }
    //                 });
    //             }
    //             Err(_) => {}
    //         };
    //     }
    // }
    Ok(())
}
