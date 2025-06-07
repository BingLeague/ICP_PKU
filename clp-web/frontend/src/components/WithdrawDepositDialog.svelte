<script>


    import {AuthClient} from "@dfinity/auth-client";
    import {onMount} from "svelte";
    import {showNotice,} from '@brewer/beerui'
    import {createEventDispatcher} from 'svelte';
    import BigNumber from "bignumber.js";
    import {inputFun} from "../utils/helpers"
    import {actorList,iiPrincipal} from "../store/wallet";


    BigNumber.config({ROUNDING_MODE: BigNumber.ROUND_DOWN});
    const dispatch = createEventDispatcher();
    export let coinName;
    export let paramsObj;
    const feeAmount = 10_000;
    let activeIndex = 0;
    let poolActor, ckbtcActor, ckethActor, ledgerActor;

    let btcBalance = 0, ethBalance = 0;
    let withdrawAmount;
    /** @type {AuthClient} */
    let btcAllowance, ethAllowance;

    let isConnected;
    let isLoading = false


    const closeHandle = () => {
        dispatch('closeMessage', true);
    };
    onMount(async () => {
        actorList.subscribe((value)=>{
            if(value.isGetActor){
                poolActor = value.poolActor;
                isConnected = true
            }
        })

    });


    const handleWithdraw = async () => {
        if (!isConnected) {
            showNotice({
                type: "warning",
                title: 'Withdraw Failed',
                message: 'Please login'
            })
            return
        }
        if (!withdrawAmount || BigNumber(withdrawAmount).lt(0)) {
            showNotice({
                type: "warning",
                title: 'WithdrawAmount',
                message: 'Please input withdraw amount'
            })
            return
        }

        if (BigNumber(withdrawAmount).gt(paramsObj.depositBalance)) {
            showNotice({
                type: "warning",
                title: 'Withdraw Failed',
                message: 'Balance not enough'
            })
            return
        }
        try {
            console.log(parseInt(BigNumber(withdrawAmount).multipliedBy(BigNumber(10).pow(paramsObj.decimals))))
            isLoading = true

            const res = await poolActor.withdraw( parseInt(BigNumber(withdrawAmount).multipliedBy(BigNumber(10).pow(paramsObj.decimals)).minus(feeAmount)))
            if (Object.keys(res) && Object.keys(res)[0] == "Ok") {

                showNotice({
                    type: "success",
                    title: 'Withdraw success!',
                    message: 'Withdraw success!'
                })
                setTimeout(() => {
                    closeHandle()
                }, 2000)
            } else {
                showNotice({
                    type: "warning",
                    title: 'Withdraw error',
                    message: 'Withdraw error ' + (res.Err)?Object.keys(res.Err)[0]:""
                })
            }
        } catch (e) {
            console.log(e)
            showNotice({
                type: "warning",
                title: 'Withdraw error',
                message: 'Withdraw error'
            })
        } finally {
            isLoading = false
        }
    }
    const setMax = () => {
        withdrawAmount = paramsObj.depositBalance
    }

</script>

<div class="deposit-dialog">
    <div class="mask" on:click={()=>{closeHandle()}}></div>
    <div class="deposit-dialog-box">
        <div class="deposit-dialog-box-title">
            Withdraw
        </div>
        <div class="input-box">
            <input type="text" bind:value={withdrawAmount} on:input={()=>{withdrawAmount=inputFun(withdrawAmount)}}
                   placeholder="0">
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
                {BigNumber(paramsObj.depositBalance).toFixed(2)} CUSD
            </div>
        </div>
        <button disabled="{isLoading}" class="mint-btn" on:click={()=>{handleWithdraw()}}>
            Withdraw
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
    z-index: 100;
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
