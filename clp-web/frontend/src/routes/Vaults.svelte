<script>
    import {AuthClient} from "@dfinity/auth-client";
    import {onMount} from "svelte";
    import {Principal} from "@dfinity/principal";
    import BigNumber from "bignumber.js";
    import DepositDialog from "../components/DepositDialog.svelte";
    import WithdrawDialog from "../components/WithdrawDialog.svelte";
    import {push, replace} from 'svelte-spa-router'
    import {actorList, iiPrincipal} from "../store/wallet";

    BigNumber.config({ROUNDING_MODE: BigNumber.ROUND_DOWN});
    /** @type {AuthClient} */
    let activeCoin = "ckBTC";
    let isShowDespoitDialog = false, isShowWithdrawDialog = false;
    let depositAddressBlob;
    let btcAssets = 0, ethAssets = 0;
    let btcInfo = {}, ethInfo = {};
    let isGetedData, isConneted;
    let principal;
    let vaultActor, ckbtcActor, ckethActor, ledgerActor;
    let btcMaxBorrow = 0, ethMaxBorrow = 0;
    let poolBalance = 0;
    let ethDecimals = 18, btcDecimals = 8, cusdDecimals = 8;
    let ethFee = 2_000_000_000_000, btcFee = 10;
    let btcBalance = 0, ethBalance = 0, btcAllowance, ethAllowance, myBTCPosition = {balance: 0, borrow: 0},
        myETHPosition = {balance: 0, borrow: 0};
    let canWithdrawEth = 0, canWithdrawBtc = 0;
    // public information
    let btcBorrowInformation = {
        balance: 0,
        borrow: 0,
        decimals: 8,
        liquidate_rate: 10
    }, ethBorrowInformation = {
        balance: 0,
        borrow: 0,
        decimals: 18,
        liquidate_rate: 10
    };

    let isConnected = false


    onMount(async () => {
        iiPrincipal.subscribe((value) => {
            if (value) {
                principal = Principal.fromText(value.toString());
                console.log(principal, value)
            }
        })

        actorList.subscribe((value) => {
            console.log("isConneted", value)
            if (value.isGetActor) {
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

    const getData = async () => {
        console.log(vaultActor)
        getAllowance()
        await getMyPosition()
        setTimeout(() => {
            getAssets()
        }, 100)
        // get decimal
        try {
            const decimal = await ckbtcActor.icrc1_decimals()
            btcDecimals = decimal
            const decimal2 = await ckethActor.icrc1_decimals()
            ethDecimals = decimal2
        } catch (e) {
            console.error(e)
        }
        // get btc balance
        try {
            const res = await ckbtcActor.icrc1_balance_of({owner: principal, subaccount: []})
            btcBalance = BigNumber(res).div(BigNumber(10).pow(btcDecimals))
        } catch (e) {
            console.error(e)
        }
        // get ethBalance
        try {
            const res = await ckethActor.icrc1_balance_of({owner: principal, subaccount: []})
            ethBalance = BigNumber(res).div(BigNumber(10).pow(ethDecimals))
        } catch (e) {
            console.error(e)
        }

        // get eth/btc borrowed
        try {
            const principalBTC = Principal.fromText(process.env.CKBTC_CANISTER_ID);
            const checkBorrow1 = await vaultActor.maxBorrow(principal, principalBTC)

            const principalETH = Principal.fromText(process.env.CKETH_CANISTER_ID);
            const checkBorrow2 = await vaultActor.maxBorrow(principal, principalETH)
            if (checkBorrow1) {
                btcMaxBorrow = BigNumber(checkBorrow1)
            }
            if (checkBorrow2) {
                ethMaxBorrow = BigNumber(checkBorrow2)
            }
        } catch (e) {
            console.error(e)
        }
        // getBorrowInfomation
        try {
            const res = await vaultActor.underlyingList()
            if (res) {
                for (let i = 0; i < res.length; i++) {
                    if (process.env.CKBTC_CANISTER_ID == res[i][0].toString()) {
                        btcBorrowInformation = res[i][1]
                    }
                    if (process.env.CKETH_CANISTER_ID == res[i][0].toString()) {
                        ethBorrowInformation = res[i][1]
                    }
                }
            }

        } catch (e) {
            console.error(e)
        }
        const btcDecimalsValue = BigNumber(10).pow(btcBorrowInformation.decimals)
        if (btcMaxBorrow > 0) {
            canWithdrawBtc = BigNumber(myBTCPosition.balance).multipliedBy(BigNumber(btcMaxBorrow).minus(myBTCPosition.borrow).div(btcMaxBorrow)).div(btcDecimalsValue)
        }
        const ethDecimalsValue = BigNumber(10).pow(ethBorrowInformation.decimals)
        if (ethMaxBorrow > 0) {
            canWithdrawEth = BigNumber(myETHPosition.balance).multipliedBy(BigNumber(ethMaxBorrow).minus(myETHPosition.borrow).div(ethMaxBorrow)).div(ethDecimalsValue)
        }

    }
    const getMyPosition = async () => {
        try {
            const balanceRes = await vaultActor.balances(principal)
            console.log(balanceRes)
            if (balanceRes) {
                for (let i = 0; i < balanceRes.length; i++) {
                    if (process.env.CKBTC_CANISTER_ID == balanceRes[i][0].toString()) {
                        myBTCPosition = balanceRes[i][1]
                    }
                    if (process.env.CKETH_CANISTER_ID == balanceRes[i][0].toString()) {
                        myETHPosition = balanceRes[i][1]
                    }
                }
            }

        } catch (e) {
            console.error(e)
        }
    }
    const getAssets = async () => {
        try {
            const balanceRes = await vaultActor.underlyingList()
            if (balanceRes) {
                for (let i = 0; i < balanceRes.length; i++) {
                    let [token, underlying] = balanceRes[i];
                    if (process.env.CKBTC_CANISTER_ID == token.toString()) {
                        const decimals = underlying.decimals
                        btcAssets = BigNumber(myBTCPosition.balance).div(BigNumber(10).pow(decimals)).multipliedBy(underlying.price).div(BigNumber(10).pow(8))
                    }
                    if (process.env.CKETH_CANISTER_ID == token.toString()) {
                        const decimals = underlying.decimals
                        // get price
                        ethAssets = BigNumber(myETHPosition.balance).div(BigNumber(10).pow(decimals)).multipliedBy(underlying.price).div(BigNumber(10).pow(8))
                        console.log(ethAssets.toFixed())
                    }

                }
            }

        } catch (e) {
            console.error(e)
        }
    }
</script>
<div class="vaults">
    {#if isShowWithdrawDialog}
        <WithdrawDialog on:closeMessage={()=>{isShowWithdrawDialog=false,getData()}}
                        coinName={activeCoin}
                        paramsObj="{
                        {
                            ethDecimals,
                            btcDecimals,
                            btcAllowance,
                            ethAllowance,
                            ethFee,
                            btcFee,
                            myBTCPosition,
                            myETHPosition,
                            canWithdrawBtc,
                            canWithdrawEth
                        }

            }"
        />
    {/if}
    {#if isShowDespoitDialog}
        <DepositDialog on:closeMessage={()=>{isShowDespoitDialog=false,getData()}}
                       coinName={activeCoin}
                       paramsObj="{
                        {
                            ethDecimals,
                            btcDecimals,
                            btcAllowance,
                            ethAllowance,
                            ethFee,
                            btcFee,
                            btcBalance,
                            ethBalance
                        }

            }"
        />
    {/if}

    <div class="token-list">
        <div class="token-info">
            <div class="token-title">
                <img width="32" src="/images/BTC.png" alt="">
                <span>ckBTC</span>
            </div>
            <div class="token-info-list">
                <div class="token-info-item">
                    <div class="item-name">
                        Total Value Locked
                    </div>
                    <div class="item-value">
                        {BigNumber(myBTCPosition.balance).div(BigNumber(10).pow(btcDecimals)).toFixed(3)} BTC /
                        ${BigNumber(btcAssets).toFixed(2)}
                    </div>
                </div>
                <div class="token-info-item">
                    <div class="item-name">
                        Total CUSD minted
                    </div>
                    <div class="item-value">
                        ${BigNumber(myBTCPosition.borrow).div(BigNumber(10).pow(cusdDecimals)).toFixed(2)}
                    </div>
                </div>
                <div class="token-info-item">
                    <div class="item-name">
                        Mimal Collateral Ratio
                    </div>
                    <div class="item-value">
                        zero
                    </div>
                </div>
                <div class="token-info-item">
                    <div class="item-name">
                        Collateral Interest Rate
                    </div>
                    <div class="item-value">
                        {BigNumber(1).div(btcBorrowInformation.liquidate_rate).multipliedBy(1000).toFixed(0)}%
                    </div>
                </div>
                <div class="token-info-item">
                    <div class="item-name">
                        APR
                    </div>
                    <div class="item-value">
                        0%
                    </div>
                </div>
                <div class="token-info-item">
                    <div class="item-name">
                        Balance
                    </div>
                    <div class="item-value">
                        {BigNumber(btcBalance).toFixed(3, BigNumber.ROUND_DOWN)} ckBTC
                    </div>
                </div>
            </div>
            <div class="mint-btn" on:click={()=>{activeCoin="ckBTC",isShowDespoitDialog=true}}>
                Deposit
            </div>
        </div>
        <div class="token-info">
            <div class="token-title">
                <img width="32" src="/images/ETH.png" alt="">
                <span>ckETH</span>
            </div>
            <div class="token-info-list">
                <div class="token-info-item">
                    <div class="item-name">
                        Total Value Locked
                    </div>
                    <div class="item-value">
                        {BigNumber(myETHPosition.balance).div(BigNumber(10).pow(ethBorrowInformation.decimals)).toFixed(3)}
                        ETH / ${BigNumber(ethAssets).toFixed(2)}
                    </div>
                </div>
                <div class="token-info-item">
                    <div class="item-name">
                        Total CUSD minted
                    </div>
                    <div class="item-value">
                        ${BigNumber(myETHPosition.borrow).div(BigNumber(10).pow(cusdDecimals)).toFixed(2)}
                    </div>
                </div>
                <div class="token-info-item">
                    <div class="item-name">
                        Mimal Collateral Ratio
                    </div>
                    <div class="item-value">
                        zero
                    </div>
                </div>
                <div class="token-info-item">
                    <div class="item-name">
                        Collateral Interest Rate
                    </div>
                    <div class="item-value">
                        {BigNumber(1).div(ethBorrowInformation.liquidate_rate).multipliedBy(1000).toFixed(0)}%
                    </div>
                </div>
                <div class="token-info-item">
                    <div class="item-name">
                        APR
                    </div>
                    <div class="item-value">
                        0%
                    </div>
                </div>
                <div class="token-info-item">
                    <div class="item-name">
                        Balance
                    </div>
                    <div class="item-value">
                        {BigNumber(ethBalance).toFixed(3, BigNumber.ROUND_DOWN)} ckETH
                    </div>
                </div>
            </div>
            <div class="mint-btn" on:click={()=>{activeCoin="ckETH",isShowDespoitDialog=true}}>
                Deposit
            </div>
        </div>
    </div>
    <div class="vaults-title">
        My Vaults
    </div>
    <div class="my-vaults-list">
        <div class="list-header">
            <!--            <div class="col">-->
            <!--                Ticker No.-->
            <!--            </div>-->
            <div class="col">
                Collaterals
            </div>
            <div class="col">
                Volume
            </div>
            <div class="col">
                CUSD Borrowed
            </div>
            <div class="col">
                Collateral Ratio
            </div>
            <div class="col">
                CLPT Earnings
            </div>
            <div class="col">
                Operation
            </div>
        </div>
        <div class="list-item">
            <!--            <div class="col">-->
            <!--                #12341-->
            <!--            </div>-->
            <div class="col">
                ckBTC
            </div>
            <div class="col">
                {canWithdrawBtc.toFixed(3)}
            </div>
            <div class="col">
                ${BigNumber(myBTCPosition.borrow).div(BigNumber(10).pow(cusdDecimals)).toFixed(2)}

            </div>
            <!--Collateral Ratio-->
            <div class="col">
                {btcMaxBorrow ? BigNumber(myBTCPosition.borrow).div(btcMaxBorrow).multipliedBy(100).toFixed(0) : 0}%
            </div>
            <div class="col">
                -
            </div>
            <div class="col">
                <div class="mint-btn normal-btn" on:click={()=>{activeCoin="ckBTC" ,isShowWithdrawDialog=true}}>
                    Retrieve
                </div>
                <div class="mint-btn" on:click={()=>{push("/borrow/ckBTC")}}>
                    Mint CUSD
                </div>
            </div>
        </div>
        <div class="list-item">
            <div class="col">
                ckETH
            </div>
            <div class="col">
                {canWithdrawEth.toFixed(3)}
            </div>
            <div class="col">
                ${BigNumber(myETHPosition.borrow).div(BigNumber(10).pow(cusdDecimals)).toFixed(2)}

            </div>

            <div class="col">
                {ethMaxBorrow ? BigNumber(myETHPosition.borrow).div(ethMaxBorrow).multipliedBy(100).toFixed(0) : 0}%

            </div>
            <div class="col">
                -
            </div>
            <div class="col">
                <div class="mint-btn normal-btn" on:click={()=>{activeCoin="ckETH" ,isShowWithdrawDialog=true}}>
                    Retrieve
                </div>
                <div class="mint-btn" on:click={()=>{push("/borrow/ckETH")}}>
                    Mint CUSD
                </div>
            </div>
        </div>
    </div>
    <div class="my-vaults-list-m">
        <div class="list-item">
            <!--            <div class="row">-->
            <!--                <div class="name">-->
            <!--                    Ticker No.-->
            <!--                </div>-->
            <!--                <div class="value">-->
            <!--                    #12341-->
            <!--                </div>-->
            <!--            </div>-->


            <div class="row">
                <div class="name">
                    Collaterals
                </div>
                <div class="value">
                    ckBTC
                </div>
            </div>

            <div class="row">
                <div class="name">
                    Volume
                </div>
                <div class="value">
                    {canWithdrawBtc.toFixed(3)}
                </div>
            </div>
            <div class="row">
                <div class="name">
                    CUSD Borrowed
                </div>
                <div class="value">
                    ${BigNumber(myBTCPosition.borrow).div(BigNumber(10).pow(cusdDecimals)).toFixed(2)}
                </div>
            </div>
            <div class="row">
                <div class="name">
                    CLPT Earnings
                </div>
                <div class="value">
                    -
                </div>
            </div>
            <div class="row">
                <div class="name">
                    Collateral Ratio
                </div>
                <div class="value">
                    {BigNumber(1).div(btcBorrowInformation.liquidate_rate).multipliedBy(1000).toFixed(0)}%
                </div>
            </div>
            <div class="row" style="">
                <div class="mint-btn normal-btn" on:click={()=>{activeCoin="ckBTC" ,isShowWithdrawDialog=true}}>
                    Retrieve
                </div>
                <div class="mint-btn" on:click={()=>{push("/borrow/ckBTC")}}>
                    Mint CUSD
                </div>
            </div>
        </div>

        <div class="list-item">
            <!--            <div class="row">-->
            <!--                <div class="name">-->
            <!--                    Ticker No.-->
            <!--                </div>-->
            <!--                <div class="value">-->
            <!--                    #12341-->
            <!--                </div>-->
            <!--            </div>-->


            <div class="row">
                <div class="name">
                    Collaterals
                </div>
                <div class="value">
                    ckETH
                </div>
            </div>

            <div class="row">
                <div class="name">
                    Volume
                </div>
                <div class="value">
                    {canWithdrawEth.toFixed(3)}
                </div>
            </div>
            <div class="row">
                <div class="name">
                    CUSD Borrowed
                </div>
                <div class="value">
                    ${BigNumber(myETHPosition.borrow).div(BigNumber(10).pow(cusdDecimals)).toFixed(2)}
                </div>
            </div>
            <div class="row">
                <div class="name">
                    CLPT Earnings
                </div>
                <div class="value">
                    -
                </div>
            </div>
            <div class="row">
                <div class="name">
                    Collateral Ratio
                </div>
                <div class="value">
                    {BigNumber(1).div(ethBorrowInformation.liquidate_rate).multipliedBy(1000).toFixed(0)}%
                </div>
            </div>
            <div class="row" style="">
                <div class="mint-btn normal-btn" on:click={()=>{activeCoin="ckETH" ,isShowWithdrawDialog=true}}>
                    Retrieve
                </div>
                <div class="mint-btn" on:click={()=>{push("/borrow/ckETH")}}>
                    Mint CUSD
                </div>
            </div>
        </div>

    </div>
</div>

<style lang="scss">
  .deposit-dialog {
    display: none;
  }

  .deposit-dialog.active {
    display: block;
  }

  .vaults {
    width: 1200px;
    margin: 0 auto;

    .token-list {
      display: flex;

      justify-content: space-between;

      .token-info {
        padding: 30px;
        width: 49%;
        background: #FEFEFF;
        box-shadow: 0px 2px 3px 0px rgba(41, 72, 152, 0.01), 0px 9px 7px 0px rgba(41, 72, 152, 0.02), 0px 22px 14px 0px rgba(41, 72, 152, 0.03), 0px 42px 28px 0px rgba(41, 72, 152, 0.03), 0px 71px 51px 0px rgba(41, 72, 152, 0.04), 0px 109px 87px 0px rgba(41, 72, 152, 0.05);
        border-radius: 11px 11px 11px 11px;

        .token-title {
          display: flex;
          align-items: center;
          font-family: Roboto, Roboto;
          font-weight: bold;
          font-size: var(--weightFont);
          color: #000000;
          text-align: left;
          font-style: normal;
          text-transform: none;

          img {
            margin-right: 10px;
          }

          span {
            margin-top: 3px;
          }
        }

        .token-info-list {
          .token-info-item {
            display: flex;
            margin-top: var(--weightFont);
            justify-content: space-between;

            .item-name {
              font-weight: 500;
              font-size: var(--weightFont);
              color: #929292;
            }

            .item-value {
              font-weight: bold;
              font-size: var(--weightFont);
              color: #282626;
            }
          }
        }
      }

    }

    .vaults-title {
      font-weight: bold;
      font-size: 30px;
      color: #000000;
      margin-top: 80px;
    }

    .my-vaults-list-m {
      display: none;
    }

    .my-vaults-list {
      .list-header {
        display: flex;
        justify-content: space-between;
        padding: 0 var(--weightFont);
        margin-top: 30px;

        .col {
          font-weight: 500;
          font-size: var(--normalFont);
          color: #929292;
          width: 12%;

          &:last-child {
            text-align: right;
            width: 27%;
          }
        }
      }

      .list-item {
        display: flex;
        justify-content: space-between;
        height: 80px;
        background: #FEFEFF;
        box-shadow: 0px 2px 3px 0px rgba(41, 72, 152, 0.01), 0px 9px 7px 0px rgba(41, 72, 152, 0.02), 0px 22px 14px 0px rgba(41, 72, 152, 0.03), 0px 42px 28px 0px rgba(41, 72, 152, 0.03), 0px 71px 51px 0px rgba(41, 72, 152, 0.04), 0px 109px 87px 0px rgba(41, 72, 152, 0.05);
        border-radius: 11px 11px 11px 11px;
        margin-top: var(--weightFont);
        align-items: center;
        padding: 0 var(--weightFont);

        .col {
          font-weight: 600;
          font-size: var(--normalFont);
          color: #333333;
          width: 12%;

          &:last-child {
            display: flex;
            justify-content: flex-end;
            width: 27%;
          }
        }

        .mint-btn {
          height: 40px;
          margin-top: 0;
          line-height: 40px;
          width: 40%;
          font-size: var(--normalFont);
          margin-left: 5%;
        }
      }
    }

  }

  @media screen and (max-width: 1400px) {
    .vaults {
      width: 900px;
    }
  }

  @media screen and (max-width: 800px) {
    .vaults {
      width: 100%;
      padding: 0 10px;

      .token-list {
        width: 97vw;
        flex-direction: column;

        .token-info {
          width: 100%;
          box-shadow: 0px 2px 3px 0px rgba(41, 72, 152, 0.01), 0px 9px 7px 0px rgba(41, 72, 152, 0.02), 0px 22px 14px 0px rgba(41, 72, 152, 0.03), 0px 42px 28px 0px rgba(41, 72, 152, 0.03), 0px 71px 51px 0px rgba(41, 72, 152, 0.04), 0px 109px 87px 0px rgba(41, 72, 152, 0.05);
          margin-top: 30px;

          .token-info-list {
            .token-info-item {
              .item-name, .item-value {
                font-size: 14px;
              }
            }
          }
        }
      }

      .vaults-title {
        font-size: 26px;
      }

      .my-vaults-list {
        display: none;
      }

      .my-vaults-list-m {
        display: block;

        .list-item {
          margin-top: 30px;
          width: 100%;
          background: #FEFEFF;
          box-shadow: 0px 2px 3px 0px rgba(41, 72, 152, 0.01), 0px 9px 7px 0px rgba(41, 72, 152, 0.02), 0px 22px 14px 0px rgba(41, 72, 152, 0.03), 0px 42px 28px 0px rgba(41, 72, 152, 0.03), 0px 71px 51px 0px rgba(41, 72, 152, 0.04), 0px 109px 87px 0px rgba(41, 72, 152, 0.05);
          border-radius: 11px 11px 11px 11px;
          padding: 50px 30px;
          display: block;

          .row {
            .mint-btn {
              width: 48%;
              font-size: 16px;
            }
          }

          .row {
            display: flex;
            justify-content: space-between;
            font-size: 14px;
            margin-top: 30px;

            &:first-child {
              margin-top: 0;
            }

            .name {
              color: #929292;
            }

            .value {
              font-family: Roboto, Roboto;
              font-weight: bold;
              color: #282626;
            }
          }
        }
      }
    }
  }
</style>
