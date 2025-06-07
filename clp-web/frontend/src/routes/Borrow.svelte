<script>

    import {AuthClient} from "@dfinity/auth-client";
    import {afterUpdate, onMount} from "svelte";
    import {Principal} from "@dfinity/principal";
    import {showNotice,} from '@brewer/beerui'
    import BigNumber from "bignumber.js";
    import RepayDialog from "../components/RepayDialog.svelte";
    import {inputFun} from "../utils/helpers"
    import {actorList,iiPrincipal} from "../store/wallet";


    BigNumber.config({ ROUNDING_MODE: BigNumber.ROUND_DOWN });
    export let params = {};

    let btcBalance = 0, ethBalance = 0, btcAllowance, ethAllowance, myBTCPosition = 0, myETHPosition = 0;
    let ethDecimals = 8, btcDecimals = 18, cusdDecimals = 8;
    let btcInformation =
            {
                name: "ckBTC",
                deposit: 0,
                decimals: 8,
                liquidate_rate: 10,
                cost:0
            },
        ethInformation = {
            name: "ckETH",
            deposit: 0,
            decimals: 8,
            liquidate_rate: 10,
            cost:0
        },
        activeInformation = {
            name: "ckBTC",
            deposit: 0,
            decimals: 8,
            liquidate_rate: 10,
            cost:0
        }
    let activeRatio;
    let btcBorrowed = {
        borrow: 0,
        balance: 0
    }, ethBorrowed = {
        borrow: 0,
        balance: 0
    };
    /** @type {AuthClient} */
    let isGetedData,isConneted;
    let isLoading = false;
    let maxMintable = 0, hasBorrowed = 0;
    let depositAddressBlob;
    let iiPrincipalString = '';
    let authType = "anonymous";
    let borrowAmount;// Actors for corresponding canisters
    // TDOD: Move to a store
    let vaultActor, ckbtcActor, ckethActor, ledgerActor, cusdActor;
    let btcMaxBorrow = 0, ethMaxBorrow = 0, activeMaxBorrow = 0;
    let poolBalance = 0;
    let isShowDialog;
    let activeCoin = "ckBTC"
    let isShowRepayDialog = false;

    // Plug wallet connection request
    // wytom-4wv7f-wtd45-4dppm-3iudp-rymlh-om66t-ewe5r-xfesu-knzqo-yqe

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
                ledgerActor = value.ledgerActor;
                isConneted = true
                getData()
            }
        })




            if (params.param) {
                activeCoin = params.param
            }
            setActiveCoin(activeCoin)



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

    const getData = async () => {

        getAllowance()
        // get decimal
        try {
            const decimal = await ckbtcActor.icrc1_decimals()
            btcDecimals = decimal
            const decimal2 = await ckethActor.icrc1_decimals()
            ethDecimals = decimal2
            cusdDecimals = await cusdActor.icrc1_decimals()
        } catch (e) {
            console.error(e)
        }
        const principal = Principal.fromText(iiPrincipalString.toString());
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
        // get clp balance
        // try {
        //     const res = await ledgerActor.account_balance({account: hexToBytes(principalToAccountDefaultIdentifier(iiPrincipal))});
        // } catch (e) {
        //     console.error(e)
        // }
        try {
            const balanceRes = await vaultActor.balances(principal)
            if (balanceRes) {
                for (let i = 0; i < balanceRes.length; i++) {
                    if (process.env.CKBTC_CANISTER_ID == balanceRes[i][0].toString()) {
                        myBTCPosition = BigNumber(balanceRes[i][1].balance).div(BigNumber(10).pow(btcDecimals))
                    }
                    if (process.env.CKETH_CANISTER_ID == balanceRes[i][0].toString()) {
                        myETHPosition = BigNumber(balanceRes[i][1].balance).div(BigNumber(10).pow(ethDecimals))
                    }
                }
            }

        } catch (e) {
            console.error(e)
        }
        try {
            const res = await vaultActor.underlyingList()

            if (res) {
                for (let i = 0; i < res.length; i++) {
                    if (process.env.CKBTC_CANISTER_ID == res[i][0].toString()) {
                        btcInformation = res[i][1]
                    }
                    if (process.env.CKETH_CANISTER_ID == res[i][0].toString()) {
                        ethInformation = res[i][1]
                    }
                }
            }

        } catch (e) {
            console.error(e)
        }

        try {
            const res = await vaultActor.balances(principal)
            if (res) {
                for (let i = 0; i < res.length; i++) {
                    if (process.env.CKBTC_CANISTER_ID == res[i][0].toString()) {
                        btcBorrowed = res[i][1]
                    }
                    if (process.env.CKETH_CANISTER_ID == res[i][0].toString()) {
                        ethBorrowed = res[i][1]
                    }
                }
            }

        } catch (e) {
            console.error(e)
        }
        try {
            const principalBTC = Principal.fromText(process.env.CKBTC_CANISTER_ID);
            const checkBorrow1 = await vaultActor.maxBorrow(principal, principalBTC)

            const principalETH = Principal.fromText(process.env.CKETH_CANISTER_ID);
            const checkBorrow2 = await vaultActor.maxBorrow(principal, principalETH)
            if (checkBorrow1) {
                btcMaxBorrow = BigNumber(checkBorrow1).div(BigNumber(10).pow(cusdDecimals))
            }
            if (checkBorrow2) {
                ethMaxBorrow = BigNumber(checkBorrow2).div(BigNumber(10).pow(cusdDecimals))
            }
        } catch (e) {
            console.error(e)
        }

        getActiveData()
    }
    const handleBorrow = async () => {

        if (!isConneted) {
            showNotice({
                type: "warning",
                title: 'Buy Failed',
                message: 'Please login'
            })
            return
        }
        if (!borrowAmount) {
            showNotice({
                type: "warning",
                title: 'borrowAmount',
                message: 'Please input borrow amount'
            })
            return
        }
        if (BigNumber(borrowAmount).lt(1)) {
            showNotice({
                type: "warning",
                title: 'borrowAmount',
                message: 'Minimum mint amount 1CUSD'
            })
            return
        }
        let decimals = cusdDecimals
        const amount = BigNumber(borrowAmount).multipliedBy(BigNumber(10).pow(decimals)).toString()
        let coinPrincipal
        if (activeCoin == "ckBTC") {
            coinPrincipal = Principal.fromText(process.env.CKBTC_CANISTER_ID)
        } else {
            coinPrincipal = Principal.fromText(process.env.CKETH_CANISTER_ID)
        }
        try {
            isLoading = true
            const res = await vaultActor.borrow(coinPrincipal, parseInt(amount))

            if (Object.keys(res) && Object.keys(res)[0] == "Ok") {
                getData()
                //
                showNotice({
                    type: "success",
                    title: 'Borrow success!',
                    message: 'Borrow success!'
                })
            } else {
                showNotice({
                    type: "warning",
                    title: 'Borrow error',
                    message: 'Borrow error'
                })
            }
        } catch (e) {
            console.error(e)
            showNotice({
                type: "warning",
                title: 'Borrow error',
                message: 'Borrow error'
            })
        } finally {
            isLoading = false
        }
    }
    const setRatioAmount = (ratio) => {
        activeRatio = ratio
        borrowAmount = BigNumber(maxMintable).multipliedBy(ratio).toFixed(6)
    }
    const setActiveCoin = (name) => {
        isShowDialog = false
        activeCoin = name
        if (name == "ckBTC") {
            activeInformation = btcInformation
        } else {
            activeInformation = ethInformation
        }
        getActiveData()
    }
    const getActiveData = () => {
        hasBorrowed = BigNumber(activeCoin == "ckBTC" ? btcBorrowed.borrow : ethBorrowed.borrow).div(BigNumber(10).pow(cusdDecimals))
        activeMaxBorrow = activeCoin == "ckBTC" ? btcMaxBorrow : ethMaxBorrow
        maxMintable = BigNumber(activeMaxBorrow).minus(hasBorrowed).gt(0)?BigNumber(activeMaxBorrow).minus(hasBorrowed):0
        borrowAmount = undefined
    }
</script>

<div class="borrow">
    {#if isShowRepayDialog}
        <RepayDialog on:closeMessage={()=>{isShowRepayDialog=false,getData()}}
                     coinName={activeCoin}
                     paramsObj="{
                        {
                            ethDecimals,
                            btcDecimals,
                            btcAllowance,
                            ethAllowance,
                            hasBorrowed,
                            btcBalance,
                            ethBalance,
                            myBTCPosition,
                            myETHPosition
                        }

            }"
        />
    {/if}
    <div class="borrow-content">
        <div class="switch-box">
            <div class="switch-box-title">
                Switch Collateral
            </div>
            <div class="switch-content">
                <div class="switch-active-name" on:click={()=>{
                    isShowDialog=true
                }}>
                    {activeCoin}
                    <svg style="margin-left: 3px" t="1717854679765" class="icon" viewBox="0 0 1024 1024" version="1.1"
                         xmlns="http://www.w3.org/2000/svg" p-id="4235" width="16" height="16">
                        <path d="M153 294.6l351.5 351.5c1.9 1.9 4.5 3.1 7.5 3.1s5.6-1.2 7.6-3.2L871 294.6c13.9-13.9 36.7-13.9 50.6 0 13.9 13.9 13.9 36.7 0 50.6L537.3 729.4c-13.9 13.9-36.7 13.9-50.6 0-0.1-0.1-0.1-0.1-0.1-0.2L102.4 345.1c-13.9-13.9-13.9-36.7 0-50.6 13.9-13.8 36.7-13.8 50.6 0.1z"
                              p-id="4236" fill="#623CE7"></path>
                        <path d="M501.8 641.2c0-0.2-0.1-0.3-0.1-0.5 0 0.2 0 0.3 0.1 0.5zM502.4 642.9c0-0.1-0.1-0.1-0.1-0.2 0 0.1 0 0.1 0.1 0.2zM501.4 638.6v0z"
                              p-id="4237" fill="#623CE7"></path>
                    </svg>
                </div>
                {#if isShowDialog}
                    <div class="switch-dialog animate__animated animate__fadeInUp">
                        <div class="switch-option  {activeCoin=='ckBTC'?'active':''}" on:click={()=>{
                            setActiveCoin("ckBTC")
                        }}>
                            ckBTC
                        </div>
                        <div class="switch-option {activeCoin=='ckETH'?'active':''}" on:click={()=>{

                            setActiveCoin("ckETH")
                        }}>
                            ckETH
                        </div>
                    </div>
                {/if}
            </div>
        </div>
        <div class="collateral-info-list">
            <div class="collateral-info-item">
                <div class="collateral-info-item-name">
                    {activeInformation.name}
                </div>
                <div class="collateral-info-item-value">
                    Collateral
                </div>
            </div>
            <div class="collateral-info-item">
                <div class="collateral-info-item-name">
                    {BigNumber(activeCoin == "ckBTC" ? btcBorrowed.balance : ethBorrowed.balance).div(BigNumber(10).pow(activeInformation.decimals)).toFixed(3)}
                </div>
                <div class="collateral-info-item-value">
                    Amount
                </div>
            </div>
            <div class="collateral-info-item">
                <div class="collateral-info-item-name">
                    {BigNumber(maxMintable).toFixed(2)}
                    CUSD
                </div>
                <div class="collateral-info-item-value">
                    Max mintable
                </div>
            </div>
        </div>

        <div class="line"></div>
        <div class="collateral-detail-box">
            <div class="collateral-detail-item">
                <div class="collateral-detail-item-name">
                    Collateral Ratio
                </div>
                <div class="collateral-detail-item-value">
                    {activeMaxBorrow>0?BigNumber(hasBorrowed).div(activeMaxBorrow).multipliedBy(100).toFixed(0):0}%
                </div>
            </div>
            <div class="collateral-detail-item">
                <div class="collateral-detail-item-name">
                    Liquidation Threshold
                </div>
                <div class="collateral-detail-item-value">
                    {BigNumber(1).div(activeInformation.liquidate_rate).multipliedBy(1000).toFixed(0)}%
                </div>
            </div>
            <div class="collateral-detail-item">
                <div class="collateral-detail-item-name">
                    Borrowing Interest
                </div>
                <div class="collateral-detail-item-value">
                    zero
                </div>
            </div>
            <div class="collateral-detail-item">
                <div class="collateral-detail-item-name">
                    Borrowing Fee
                </div>
                <div class="collateral-detail-item-value">
                    {BigNumber(activeInformation.cost).div(BigNumber(10).pow(cusdDecimals)).toString()} CUSD
<!--                    0-->
                </div>
            </div>
            <div class="collateral-detail-item">
                <div class="collateral-detail-item-name">
                    Mimal Collateral Ratio
                </div>
                <div class="collateral-detail-item-value">
                    {activeInformation.liquidate_rate}%
                </div>
            </div>
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
            <input type="text" placeholder="0" on:input={borrowAmount=inputFun(borrowAmount)} bind:value={borrowAmount}>
            <div class="input-tip">
                CUSD
            </div>
        </div>
        <div class="input-detail">
            Minimum mint amount 1CUSD
        </div>

        <button disabled="{isLoading}" on:click={()=>{
                handleBorrow()
            }} class="mint-btn">
            Mint
            {#if isLoading}
                <img class="loading-icon" src="/images/loading.svg"/>
            {/if}
        </button>

        <div class="mint-information">
            <div>
                Notice:
            </div>
            <p>
                You will receive CUSD at zero interest rate, but please pay attention to the price changes of the
                pledged assets and pay back in time or deposit more pledged assets before liquidation.
            </p>
            <p>
                The borrowing fee is
                a one-time charge. Part of it is used to purchase CLPT for destruction.
            </p>
        </div>
        <button on:click={()=>{
                isShowRepayDialog=true
            }} class="mint-btn">
            Repay
        </button>
    </div>
</div>

<style lang="scss">

  .borrow {
    .borrow-content {
      width: 556px;
      background: #FEFEFF;
      box-shadow: 0px 2px 3px 0px rgba(41, 72, 152, 0.01), 0px 9px 7px 0px rgba(41, 72, 152, 0.02), 0px 22px 14px 0px rgba(41, 72, 152, 0.03), 0px 42px 28px 0px rgba(41, 72, 152, 0.03), 0px 71px 51px 0px rgba(41, 72, 152, 0.04), 0px 109px 87px 0px rgba(41, 72, 152, 0.05);
      border-radius: 11px 11px 11px 11px;
      margin: 0 auto;
      padding: 30px 30px;

      .switch-box {
        display: flex;
        justify-content: space-between;

        .switch-box-title {

          font-family: Roboto, Roboto;
          font-weight: bold;
          font-size: var(--weightFont);
          color: #000000;
          line-height: 40px;
          text-align: left;
          font-style: normal;
        }

        .switch-content {
          width: 120px;
          height: 40px;
          font-family: Roboto, Roboto;
          font-weight: 500;
          font-size: 18px;
          color: #623CE7;
          font-style: normal;
          border: 1px solid #623CE7;
          border-radius: 0 10px 0 10px;
          line-height: 40px;
          text-align: center;
          position: relative;

          .switch-active-name {
            cursor: pointer;
            display: flex;
            align-items: center;
            justify-content: center;
          }

          .switch-dialog {
            position: absolute;
            width: 110%;
            height: auto;
            top: 50px;
            left: -5%;
            z-index: 1;
            background: #FFFFFF;
            border-radius: 10px;
            border: 1px solid #F3F2F2;
            color: #534364;
            box-shadow: 0px 2px 3px 0px rgba(0, 0, 0, 0.01), 0px 8px 7px 0px rgba(0, 0, 0, 0.01), 0px var(--weightFont) 13px 0px rgba(0, 0, 0, 0.01), 0px 39px 25px 0px rgba(0, 0, 0, 0.02), 0px 65px 47px 0px rgba(0, 0, 0, 0.02), 0px 100px 80px 0px rgba(0, 0, 0, 0.03);

            .switch-option {
              font-size: 18px;
              cursor: pointer;
              border-radius: 10px;


              &:hover {
                color: #623CE7;

              }
            }

          }
        }
      }

      .line {
        height: 1px;
        background: #EAEAEA;
        width: 100%;
        margin: var(--weightFont) 0;
      }

      .collateral-info-list {
        display: flex;
        margin-top: var(--weightFont);
        justify-content: space-between;

        .collateral-info-item {
          .collateral-info-item-name {
            font-family: Roboto, Roboto;
            font-weight: bold;
            font-size: 26px;
            color: #282626;
            line-height: 35px;
            white-space: nowrap;
            text-align: center;
          }

          .collateral-info-item-value {
            font-family: Roboto, Roboto;
            font-weight: 500;
            font-size: var(--weightFont);
            color: #929292;
            line-height: 23px;
            margin-top: 10px;
            text-align: center;
          }
        }
      }

      .collateral-detail-box {
        .collateral-detail-item {
          display: flex;
          width: 100%;
          justify-content: space-between;
          margin-top: var(--weightFont);

          .collateral-detail-item-name {
            font-family: Roboto, Roboto;
            font-weight: 500;
            font-size: var(--weightFont);
            color: #929292;
          }

          .collateral-detail-item-value {
            font-family: Roboto, Roboto;
            font-weight: bold;
            font-size: var(--weightFont);
            color: #282626;
          }
        }
      }


      .mint-information {
        margin-top: 20px;
        font-weight: 500;
        font-size: var(--normalFont);
        color: #929292;

        p {
          margin-top: 5px;
          margin-bottom: 0;
        }
      }
    }

  }

  @media screen and (max-width: 1400px) {
   .borrow{
     .borrow-content{
        .collateral-info-list{
          .collateral-info-item {
            .collateral-info-item-name {
              font-size: 20px;
            }

            .collateral-info-item-value {
              font-size: 16px;
            }
          }
          .collateral-detail-box{
            .collateral-detail-item {

              .collateral-detail-item-name {
                font-size: 14px;
              }

              .collateral-detail-item-value {
                font-size: 16px;
              }
            }
          }
        }
     }
   }
  }
  @media screen and (max-width: 800px) {
    .borrow {
      width: 100%;
      padding: 0 10px;

      .borrow-content {
        width: 100%;

        .mint-information {
          font-size: 14px;
        }

        .collateral-info-list {
          .collateral-info-item {
            .collateral-info-item-name {
              font-size: 16px;
            }

            .collateral-info-item-value {
              font-size: 14px;
            }
          }
        }

        .collateral-detail-box {
          .collateral-detail-item {

            .collateral-detail-item-name {
              font-size: 14px;
            }

            .collateral-detail-item-value {
              font-size: 16px;
            }
          }
        }
      }
    }

  }
</style>
