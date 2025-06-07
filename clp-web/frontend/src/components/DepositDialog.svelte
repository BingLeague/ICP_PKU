<script>


    import {AuthClient} from "@dfinity/auth-client";
    import {onMount} from "svelte";
    import {Principal} from "@dfinity/principal";
    import {showNotice,} from '@brewer/beerui'
    import {createEventDispatcher} from 'svelte';
    import BigNumber from "bignumber.js";
    import {inputFun} from "../utils/helpers"
    import {actorList,iiPrincipal} from "../store/wallet";


    BigNumber.config({ ROUNDING_MODE: BigNumber.ROUND_DOWN });
    const dispatch = createEventDispatcher();
    export let coinName;
    export let paramsObj;
    let isLoading = false;

    let activeIndex = 0;
    let activeCoin = "ckBTC";
    let vaultActor, ckbtcActor, ckethActor, ledgerActor;
    let isConneted;
    let stakeAmount, withdrawAmount;
    /** @type {AuthClient} */
    let btcAllowance, ethAllowance;
    let depositAddressBlob;
    let iiPrincipalString = '';


    const closeHandle = () => {
        dispatch('closeMessage', true);
    };
    activeCoin = coinName
    onMount(async () => {
        iiPrincipal.subscribe((value)=>{
            if(value){
                iiPrincipalString = Principal.fromText(value.toString());
            }
        })
        actorList.subscribe((value)=>{
            console.log("isConneted",value)
            if(value.isGetActor){
                vaultActor = value.vaultActor;
                ckbtcActor = value.ckbtcActor;
                ckethActor = value.ckethActor;
                ledgerActor = value.ledgerActor;
                isConneted = true
                getData()
            }
        })
    });
    const getAllowance = async () => {
        const principal = Principal.fromText(iiPrincipalString.toString());
        try {
            const res = await ckbtcActor.icrc2_allowance({
                account: {
                    owner: principal,
                    subaccount: []
                },
                spender: {
                    owner: Principal.fromText(process.env.VAULT_CANISTER_ID),
                    subaccount: []
                }
            })
            btcAllowance = res.allowance

            const ethres = await ckethActor.icrc2_allowance({
                account: {
                    owner: principal,
                    subaccount: []
                },
                spender: {
                    owner: Principal.fromText(process.env.VAULT_CANISTER_ID),
                    subaccount: []
                }
            })
            ethAllowance = ethres.allowance
        } catch (e) {
            console.error(e)
        }
    }

    const handleApprove = async () => {
        let decimals = 8;
        let coinActor
        if (activeCoin == "ckBTC") {
            decimals = paramsObj.btcDecimals
            coinActor = ckbtcActor
        } else {
            decimals = paramsObj.ethDecimals
            coinActor = ckethActor
        }
        const amount = BigNumber(10).pow(decimals).multipliedBy(stakeAmount * 100).toFixed(0)
        const res = await coinActor.icrc2_approve({
            amount: parseInt(amount),
            created_at_time: [],
            expected_allowance: [],
            expires_at: [],
            fee: [],
            from_subaccount: [],
            memo: [],
            spender: {
                owner: Principal.fromText(process.env.VAULT_CANISTER_ID),
                subaccount: []
            }
        })
        console.log({
            approveRes: res
        })
        return res

    }
    const getData = async () => {

        getAllowance()
    }

    const handleStake = async () => {
        if (!isConneted) {
            showNotice({
                type: "warning",
                title: 'Staking Failed',
                message: 'Please login'
            })
            return
        }
        if (!stakeAmount) {
            showNotice({
                type: "warning",
                title: 'Staking Failed',
                message: 'Please input stake amount'
            })
            return
        }
        let principal, decimals, allowance, balance,fee
        if (activeCoin == "ckBTC") {
            principal = Principal.fromText(process.env.CKBTC_CANISTER_ID);
            decimals = paramsObj.btcDecimals
            allowance = btcAllowance
            balance = paramsObj.btcBalance
            fee = paramsObj.btcFee
        } else {
            principal = Principal.fromText(process.env.CKETH_CANISTER_ID);
            decimals = paramsObj.ethDecimals
            allowance = ethAllowance
            balance = paramsObj.ethBalance
            fee = paramsObj.ethFee
        }
        if (BigNumber(stakeAmount).gt(balance)) {
            showNotice({
                type: "warning",
                title: 'Staking Failed',
                message: 'Balance not enough'
            })
            return
        }
        let amount = BigNumber(stakeAmount).multipliedBy(BigNumber(10).pow(decimals))

        try {
            isLoading = true
            amount = amount.minus(fee)
            if (allowance < amount) {
                amount = amount.minus(fee)
                await handleApprove()
            }
            const res = await vaultActor.deposit(principal, parseInt(amount))
            console.log(res)
            if (Object.keys(res) && Object.keys(res)[0] == "Ok") {
                getData()
                showNotice({
                    type: "success",
                    title: 'Deposit success!',
                    message: 'Deposit success!'
                })
                setTimeout(() => {
                    closeHandle()
                }, 2000)
            } else {
                showNotice({
                    type: "warning",
                    title: 'Deposit error',
                    message: 'Deposit error'
                })
            }

        } catch (e) {
            console.log(e)
            showNotice({
                type: "warning",
                title: 'Deposit error',
                message: 'Deposit error'
            })
        } finally {
            isLoading = false
        }

    }
    const setMax = () => {
        if (activeCoin == "ckBTC") {
            stakeAmount = paramsObj.btcBalance.toString()
        }
        if (activeCoin == "ckETH") {
            stakeAmount = paramsObj.ethBalance.toString()
        }
    }

</script>

<div class="deposit-dialog">
    <div class="mask" on:click={()=>{closeHandle()}}></div>
    <div class="deposit-dialog-box">
        <div class="deposit-dialog-box-title">
            Deposit
        </div>
        <div class="input-box">
            <input type="text" on:input={()=>{stakeAmount=inputFun(stakeAmount)}} bind:value={stakeAmount} placeholder="0">
            <div class="input-tip">
                <div class="mint-btn" on:click={()=>{setMax()}}>
                    MAX
                </div>
            </div>
        </div>

        <div class="balance-box">
            <div class="balance-box-name">
                Balance
            </div>
            <div class="balance-box-value">
                {activeCoin == "ckBTC" ? paramsObj.btcBalance.toFixed(4) : paramsObj.ethBalance.toFixed(4)} {activeCoin}
            </div>
        </div>
        <button class="mint-btn" disabled={isLoading} on:click={()=>{handleStake()}}>
            Deposit
            {#if isLoading}
                <img class="loading-icon" src="/images/loading.svg"/>
            {/if}
        </button>

    </div>

</div>

<style lang="scss">


  .staked-box {
    width: 556px;
    background: #FEFEFF;
    box-shadow: 0px 2px 3px 0px rgba(41, 72, 152, 0.01), 0px 9px 7px 0px rgba(41, 72, 152, 0.02), 0px 22px 14px 0px rgba(41, 72, 152, 0.03), 0px 42px 28px 0px rgba(41, 72, 152, 0.03), 0px 71px 51px 0px rgba(41, 72, 152, 0.04), 0px 109px 87px 0px rgba(41, 72, 152, 0.05);
    border-radius: 11px 11px 11px 11px;
    padding: 30px;
    margin: 30px auto;

    .title {
      font-family: Roboto, Roboto;
      font-weight: bold;
      font-size: var(--bigFont);
      color: #000000;
    }
  }

  .position-info {
    width: 556px;
    background: #FEFEFF;
    box-shadow: 0px 2px 3px 0px rgba(41, 72, 152, 0.01), 0px 9px 7px 0px rgba(41, 72, 152, 0.02), 0px 22px 14px 0px rgba(41, 72, 152, 0.03), 0px 42px 28px 0px rgba(41, 72, 152, 0.03), 0px 71px 51px 0px rgba(41, 72, 152, 0.04), 0px 109px 87px 0px rgba(41, 72, 152, 0.05);
    border-radius: 11px 11px 11px 11px;
    margin: 0 auto;
    padding: 30px;

    .info-item {
      display: flex;
      justify-content: space-between;
      margin-top: var(--weightFont);

      &:first-child {
        margin-top: 0;
      }

      .name {
        font-weight: 500;
        font-size: var(--weightFont);
        color: #929292;
      }

      .value {
        font-weight: bold;
        font-size: var(--weightFont);
        color: #282626;
      }
    }
  }

  .deposit-dialog {
    position: fixed;
    margin: 30vh auto 0;
    left: 0;
    top: 0;

    .mask {
      position: fixed;
      width: 100vw;
      height: 100vh;
      top: 0;
      left: 0;
      background: rgba(0, 0, 0, 0.27);
    }

    .input-tip {
      .mint-btn {
        width: 100px;
        height: 40px;
        line-height: 40px;
        margin-top: -10px;
        margin-right: var(--weightFont);
        border-radius: 0 10px 10px 10px;
      }
    }


    .deposit-dialog-box {
      width: 556px;
      position: relative;
      z-index: 1;
      background: #FEFEFF;
      box-shadow: 0px 2px 3px 0px rgba(41, 72, 152, 0.01), 0px 9px 7px 0px rgba(41, 72, 152, 0.02), 0px 22px 14px 0px rgba(41, 72, 152, 0.03), 0px 42px 28px 0px rgba(41, 72, 152, 0.03), 0px 71px 51px 0px rgba(41, 72, 152, 0.04), 0px 109px 87px 0px rgba(41, 72, 152, 0.05);
      border-radius: 11px 11px 11px 11px;
      padding: 30px;
      left: calc(50vw - 278px);


      .deposit-dialog-box-title {
        font-weight: bold;
        font-size: 30px;
        color: #000000;
      }

      .balance-box {
        display: flex;
        margin-top: var(--weightFont);
        justify-content: space-between;

        .balance-box-name {
          font-weight: 500;
          font-size: var(--weightFont);
          color: #929292;
        }

        .balance-box-value {
          font-weight: bold;
          font-size: var(--weightFont);
          color: #282626;
        }
      }
    }


  }

  @media screen and (max-width: 800px) {
    .deposit-dialog {
      width: 100%;
      padding: 0 10px;

      .coin-select {
        width: 100%;

        .coin-select-content {
          width: 100%;
        }
      }

      .select-list {
        width: 100%;

        .select-item {
          width: 48%;
        }
      }

      .header-list {
        flex-direction: column;

        .list-item {
          margin-top: 30px;

          .item-name {
            font-size: var(--weightFont);
          }

          .detail {
            .in-line {
              .name {
                font-size: 18px;
              }

              .value {
                font-size: var(--weightFont);
              }
            }
          }

          &:first-child {
            margin-top: 0;
          }
        }
      }

      .deposit-dialog-box {
        width: 100%;
        left: 0;

        .deposit-dialog-box-title {
          font-size: 26px;
        }
      }

      .position-info {
        width: 100%;

        .info-item {
          .name {
            font-size: 16px;
          }

          .value {
            font-size: 18px;
          }
        }

      }

      .staked-box {
        width: 100%;

        .title {
          font-size: 22px;
        }
      }
    }


  }
</style>
