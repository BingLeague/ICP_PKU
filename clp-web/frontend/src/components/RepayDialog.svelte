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
    let isConneted;
    let activeRatio;
    let activeCoin = "ckBTC";
    let vaultActor, ckbtcActor, ckethActor, cusdActor;
    let cusdBalance = 0;
    let btcBalance = 0, ethBalance = 0;
    let repayAmount;
    /** @type {AuthClient} */
    let cusdAllowance;
    let decimals=18;
    let depositAddressBlob;
    let iiPrincipalString = '';
    let isLoading = false
    // Actors for corresponding canisters

    // TDOD: Move to a store

    // Plug wallet connection request
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
            if(value.isGetActor){
                vaultActor = value.vaultActor;
                ckbtcActor = value.ckbtcActor;
                ckethActor = value.ckethActor;
                cusdActor= value.cusdActor;
                isConneted = true
                getData()
            }
        })

    });
    const getAllowance = async () => {
        const principal = Principal.fromText(iiPrincipalString.toString());

        try {
            const res = await cusdActor.icrc2_allowance({
                account: {
                    owner: principal,
                    subaccount: []
                },
                spender: {
                    owner: Principal.fromText(process.env.VAULT_CANISTER_ID),
                    subaccount: []
                }
            })
            cusdAllowance = res.allowance


        } catch (e) {
            console.error(e)
        }
    }

    const handleApprove = async () => {
        const amount = BigNumber(10).pow(decimals).multipliedBy(repayAmount*100).toFixed(0)
        const res = await cusdActor.icrc2_approve({
            amount: parseInt(
                amount
            ),
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

    }
    const getData = async () => {

        const principal = Principal.fromText(iiPrincipalString.toString());

        getAllowance()
        // get decimal

        try{
            const decimal = await cusdActor.icrc1_decimals()
            const res = await cusdActor.icrc1_balance_of({owner: principal, subaccount: []})
            decimals =decimal
            const balance = BigNumber(res).div(BigNumber(10).pow(decimal))
            cusdBalance = balance
        }catch (e) {
            console.error(e)
        }
    }
    const handleRepay = async () => {
        if (!isConneted) {
            showNotice({
                type: "warning",
                title: 'Repay Failed',
                message: 'Please login'
            })
            return
        }
        if (!repayAmount || BigNumber(repayAmount).lt(0)) {
            showNotice({
                type: "warning",
                title: 'RepayAmount',
                message: 'Please input repay amount'
            })
            return
        }
        if (BigNumber(repayAmount).gt(cusdBalance)) {
            showNotice({
                type: "warning",
                title: 'RepayAmount',
                message: 'Insufficient balance'
            })
            return
        }

        let principal, balance,allowance=0
        if (activeCoin == "ckBTC") {
            principal = Principal.fromText(process.env.CKBTC_CANISTER_ID);
            balance = paramsObj.myBTCPosition.balance
        } else {
            principal = Principal.fromText(process.env.CKETH_CANISTER_ID);
            balance = paramsObj.myETHPosition.balance
        }


        if (BigNumber(repayAmount).gt(balance)) {
            showNotice({
                type: "warning",
                title: 'Withdraw Failed',
                message: 'Balance not enough'
            })
            return
        }
        try {

            isLoading = true
            let amount =BigNumber(repayAmount).multipliedBy(BigNumber(10).pow(decimals))
            if (cusdAllowance < amount) {
                await handleApprove()
            }
            const res = await vaultActor.repayment(principal, parseInt(amount))
            if (Object.keys(res) && Object.keys(res)[0] == "Ok") {
                getData()
                //
                showNotice({
                    type: "success",
                    title: 'Repay success!',
                    message: 'Repay success!'
                })
                setTimeout(() => {
                    closeHandle()
                }, 2000)
            } else {
                showNotice({
                    type: "warning",
                    title: 'Repay error',
                    message: 'Repay error' + (res.Err)?Object.keys(res.Err)[0]:""
                })
            }

        } catch (e) {
            console.log(e)
            showNotice({
                type: "warning",
                title: 'Repay error',
                message: 'Repay error'
            })
        } finally {
            isLoading = false
        }
    }
    const setMax = () => {
        repayAmount = BigNumber(paramsObj.hasBorrowed).multipliedBy(1).toFixed(6)
    }
    const setRatioAmount = (ratio) => {
        activeRatio = ratio

        repayAmount = BigNumber(paramsObj.hasBorrowed).multipliedBy(ratio).toFixed(6)
    }


</script>

<div class="deposit-dialog">
    <div class="mask" on:click={()=>{closeHandle()}}></div>
    <div class="deposit-dialog-box">
        <div class="deposit-dialog-box-title">
            Repay
        </div>
        <div class="loan-detail">
            Outstanding Loan Balance {BigNumber(paramsObj.hasBorrowed).toFixed(2)}CUSD
        </div>
        <div class="ratio-select-box">
            <div class="ratio-select-item {activeRatio==0.25?'active':''}" on:click={()=>{setRatioAmount(0.25)}}>
                25%
            </div>
            <div class="ratio-select-item {activeRatio==0.5?'active':''}" on:click={()=>{setRatioAmount(0.5)}}>
                50%
            </div>
            <div class="ratio-select-item {activeRatio==0.75?'active':''}" on:click={()=>{setRatioAmount(0.75)}}>
                75%
            </div>
            <div class="ratio-select-item {activeRatio==1?'active':''}" on:click={()=>{setRatioAmount(1)}}>
                MAX
            </div>
        </div>
        <div class="input-box">
            <input type="text" bind:value={repayAmount} on:input={()=>{repayAmount=inputFun(repayAmount)}} placeholder="0">
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
                {cusdBalance.toFixed(2)} CUSD
            </div>
        </div>
        {#if BigNumber(repayAmount).gt(cusdBalance)}

            <button disabled="{true}" class="mint-btn">
                Insufficient balance
            </button>
        {:else}
            <button disabled="{isLoading}" class="mint-btn" on:click={()=>{handleRepay()}}>
                Repay
                {#if isLoading}
                    <img class="loading-icon" src="/images/loading.svg"/>
                {/if}
            </button>
        {/if}


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
    margin: 20vh auto 0;
    left: 0;
    top: 0;
    z-index: 10;

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
        margin-right: -var(--weightFont);
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

      .loan-detail {
        margin-top: var(--weightFont);
        font-size: 18px;
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
