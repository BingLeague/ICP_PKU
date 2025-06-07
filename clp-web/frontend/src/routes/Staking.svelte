<script>


    import {AuthClient} from "@dfinity/auth-client";
    import {onMount} from "svelte";
    import {
        auth, STAKING_CANISTER_ID
    } from "../store/auth";
    import {Principal} from "@dfinity/principal";
    import {showNotice, BeSwitch,loadingSerive} from '@brewer/beerui'
    import {actorList, iiPrincipal} from "../store/wallet";

    import BigNumber from "bignumber.js";

    let stakedArr = [], autoStakeArr = [], normalStakeArr = [],historyArr= []
    let activeIndex = 0;
    let activeAPY;
    let activeCoin = "ckBTC";
    let btcBalance = 0, ethBalance = 0, clptAllowance, ethAllowance, myBTCPosition = 0, myETHPosition = 0;
    let clptDecimls = 8, btcDecimals = 18;
    let clptFee = 10000
    let clptBalance = 0;
    let stakeAmount, withdrawAmount;
    let isConneted;
    let automatic = false;
    let apyList = []
    let isLoading;
    let totalBalance=BigNumber(0),totalEarned=BigNumber(0)
    /** @type {AuthClient} */


    let iiPrincipalString = '';
    let principal;

    // Actors for corresponding canisters

    // TDOD: Move to a store
    let vaultActor, ckbtcActor, clptActor, stakeActor, cusdActor;
    let poolBalance = 0;

    // Plug wallet connection request
    onMount(async () => {
        iiPrincipal.subscribe((value) => {
            if (value) {
                iiPrincipalString = Principal.fromText(value.toString());
                principal = iiPrincipalString

            }
        })
        actorList.subscribe((value) => {
            // console.log("isConneted",value)
            if (value.isGetActor) {
                vaultActor = value.vaultActor;
                ckbtcActor = value.ckbtcActor;
                cusdActor = value.cusdActor
                clptActor = value.clptActor;
                stakeActor = value.stakeActor
                isConneted = true
                getData()
            }
        })


    });
    const getCLPTBalance = async () => {
        try {
            const decimals = await clptActor.icrc1_decimals()
            clptDecimls = decimals
            const fee = await clptActor.icrc1_fee()
            clptFee = fee
            const res = await clptActor.icrc1_balance_of({
                owner: principal,
                subaccount: []
            })
            console.log(res)
            clptBalance = BigNumber(res).div(BigNumber(10).pow(clptDecimls))
        } catch (e) {
            console.error(e)
        }
    }
    const getAPY = async () => {
        const res = await stakeActor.periodConfig()
        apyList = res
        activeAPY = res[0][0]
    }
    const getAllowance = async () => {
        try {
            const res = await clptActor.icrc2_allowance({
                account: {
                    owner: principal,
                    subaccount: []
                },
                spender: {
                    owner: Principal.fromText(process.env.VAULT_CANISTER_ID),
                    subaccount: []
                }
            })
            clptAllowance = res.allowance

        } catch (e) {
            console.error(e)
        }
    }

    const handleApprove = async () => {
        try {
            isLoading = true
            const amount = BigNumber(10).pow(clptDecimls).multipliedBy(stakeAmount * 100).toFixed(0)
            const res = await clptActor.icrc2_approve({
                amount: parseInt(amount),
                created_at_time: [],
                expected_allowance: [],
                expires_at: [],
                fee: [],
                from_subaccount: [],
                memo: [],
                spender: {
                    owner: Principal.fromText(STAKING_CANISTER_ID),
                    subaccount: []
                }
            })
            console.log({
                approveRes: res
            })
            return res
        } catch (e) {
            console.log(e)
        } finally {
            isLoading = false
        }

    }
    const getData = async () => {
        getAllowance()
        getCLPTBalance()
        getAPY()
        allClptBalances()
    }
    const allClptBalances = async () => {
        stakedArr = []
        autoStakeArr = []
        normalStakeArr = []
        historyArr = []
        totalBalance=BigNumber(0)
        totalEarned=BigNumber(0)
        const res = await stakeActor.clptBalances(principal)
        try {
            for (let i = 0; i < res.length; i++) {
                stakedArr.push({...res[i][1], index: i})
            }
        } catch (e) {

        }
        for (let i = 0; i < stakedArr.length; i++) {
            let item = stakedArr[i]
            totalBalance = totalBalance.plus(item.balance)
            totalEarned = totalEarned.plus(item.earned)

            if( (Object.keys(item.status) && Object.keys(item.status)[0] == "Valid") ){
                if (item.auto_intevetment) {
                    autoStakeArr.push(item)
                } else {
                    normalStakeArr.push(item)
                }
            }else{
                historyArr.push(item)
            }

        }
        autoStakeArr = autoStakeArr
        normalStakeArr = normalStakeArr
        historyArr= historyArr
    }
    const unstake = async (index)=>{
        const res = await stakeActor.unstake(index)
        if (Object.keys(res) && Object.keys(res)[0] == "Ok") {
            allClptBalances()
            showNotice({
                type: "success",
                title: 'Withdraw success!',
                message: 'Withdraw success!'
            })
        } else {
            showNotice({
                type: "warning",
                title: 'Withdraw error',
                message: 'Withdraw error'
            })
        }
    }
    const autoIntevetment = async (index, isAuto) => {
        let loadingInstance1 = new loadingSerive({
            target: '.app-content',
            text: 'loading'

        });
        loadingInstance1.show();
        const res = await stakeActor.autoIntevetment(index, isAuto)
        if (Object.keys(res) && Object.keys(res)[0] == "Ok") {
            allClptBalances()
            showNotice({
                type: "success",
                title: 'Set success!',
                message: 'Set success!'
            })
        } else {
            showNotice({
                type: "warning",
                title: 'Set error',
                message: 'Set error'
            })
        }
        loadingInstance1.close();
    }
    const handleWithdraw = async () => {
        let principal, decimals
        if (activeCoin == "ckBTC") {
            principal = Principal.fromText(process.env.CKBTC_CANISTER_ID);
            decimals = btcDecimals
        } else {
            principal = Principal.fromText(process.env.CKETH_CANISTER_ID);
            decimals = ethDecimals
        }

        if (!$auth.loggedIn) {
            showNotice({
                type: "warning",
                title: 'Staking Failed',
                message: 'Please login'
            })
            return
        }
        try {
            const res = await vaultActor.withdraw(principal, parseInt(BigNumber(withdrawAmount).multipliedBy(BigNumber(10).pow(decimals))))
            if (res.Ok) {
                getData()
                //
                showNotice({
                    type: "success",
                    title: 'Withdraw success!',
                    message: 'Withdraw success!'
                })
            } else {
                showNotice({
                    type: "warning",
                    title: 'Withdraw error',
                    message: 'Withdraw error'
                })
            }
        } catch (e) {
            console.log(e)
            showNotice({
                type: "warning",
                title: 'Withdraw error',
                message: 'Withdraw error'
            })
        }
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
        if (!activeAPY) {
            showNotice({
                type: "warning",
                title: 'Staking Failed',
                message: 'Please choose apy '
            })
            return
        }
        const amount = BigNumber(stakeAmount).multipliedBy(BigNumber(10).pow(clptDecimls))

        try {
            isLoading = true
            if (clptAllowance < amount) {
                await handleApprove()
            }

            const res = await stakeActor.stake(activeAPY, parseInt(amount.toFixed(0)), automatic)
            if (Object.keys(res) && Object.keys(res)[0] == "Ok") {
                getData()
                stakeAmount = undefined
                //
                showNotice({
                    type: "success",
                    title: 'Staking success!',
                    message: 'Staking success!'
                })
            } else {
                showNotice({
                    type: "warning",
                    title: 'Staking error',
                    message: 'Staking error'
                })
            }
        } catch (e) {
            console.log(e)
            showNotice({
                type: "warning",
                title: 'Staking error',
                message: 'Staking error'
            })
        } finally {
            isLoading = false
        }

    }
    const setMax = () => {
        stakeAmount = BigNumber(clptBalance).minus(BigNumber(clptFee).div(10 ** clptDecimls)).toFixed(3,BigNumber.ROUND_DOWN)
    }
    const setAPY = (item) => {
        activeAPY = item[0]
    }
    const getDate = (time) => {
        let dateTime = BigNumber(time).div(1000000).toFixed(0)
        return formatDate(parseInt(dateTime))
    }
    const formatDate = function (time) {//时间戳转日期
        let date = new Date(time);
        let y = date.getFullYear();
        let MM = date.getMonth() + 1;
        MM = MM < 10 ? ('0' + MM) : MM;
        let d = date.getDate();
        d = d < 10 ? ('0' + d) : d;
        let h = date.getHours();
        h = h < 10 ? ('0' + h) : h;
        let m = date.getMinutes();
        m = m < 10 ? ('0' + m) : m;
        let s = date.getSeconds();
        s = s < 10 ? ('0' + s) : s;
        return y + '-' + MM + '-' + d + ' ' + h + ':' + m + ':' + s;
        // return y + '-' + MM + '-' + d;
    }
    const getIsCanWithdraw = (time) => {
        let dateTime = BigNumber(time).div(1000000).toFixed(0)
        let nowTime = new Date().getTime()
        return nowTime >= dateTime
    }
</script>

<div class="staking">
    <!--    <div class="coin-select">-->
    <!--        <div class="coin-select-content">-->
    <!--            <div class="select-item {activeCoin=='ckBTC'?'active':''}" on:click={()=>{activeCoin='ckBTC'}}>-->
    <!--                ckBTC-->
    <!--            </div>-->
    <!--            <div class="select-item {activeCoin=='ckETH'?'active':''}" on:click={()=>{activeCoin='ckETH'}}>-->
    <!--                ckETH-->
    <!--            </div>-->
    <!--        </div>-->
    <!--    </div>-->
    <div class="select-list">
        <div class="select-item {activeIndex==0?'active':''}" on:click={()=>{activeIndex=0}}>
            Stake
        </div>
        <div class="select-item {activeIndex==1?'active':''}" on:click={()=>{activeIndex=1}}>
            Position
        </div>
    </div>
    {#if activeIndex == 0}
        <div class="header-list">
            {#each apyList as item}
                <div class="list-item  {activeAPY===item[0]?'active':''}" on:click={setAPY(item)}>
                    <div class="item-name">
                        {item[0]}-Day
                    </div>
                    <div class="detail">
                        <div class="in-line">
                            <div class="name">
                                APY
                            </div>
                            <div class="value">
                                {BigNumber(item[1].apy).multipliedBy(10 ** 2).toFixed(2)}%
                            </div>
                        </div>
                    </div>
                </div>
            {/each}
        </div>
        <div class="staking-box">
            <div class="staking-box-title">
                Stake
            </div>
            <div class="input-box">
                <input type="number" bind:value={stakeAmount} placeholder="0">
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
                    {clptBalance.toString()} CLPT
                </div>

            </div>
            <button class="mint-btn" disabled={isLoading} on:click={()=>{handleStake()}}>
                Stake
                {#if isLoading}
                    <img class="loading-icon" src="/images/loading.svg"/>
                {/if}
            </button>
            <div class="automatic">
                <span>Automatic Reinvestment </span>
                <BeSwitch bind:checked={automatic} activeColor="#623CE7"></BeSwitch>
            </div>
            <!--            <div class="balance-box">-->
            <!--                <div class="balance-box-name">-->
            <!--                    Maturity Date(UTC)-->
            <!--                </div>-->
            <!--                <div class="balance-box-value">-->
            <!--                    Mar 26,2024-->
            <!--                </div>-->
            <!--            </div>-->
        </div>
    {:else}
       <div class="position">
           <div class="position-info">
               <div class="info-item">
                   <div class="name">
                       Balance
                   </div>
                   <div class="value">
                       {totalBalance.div(10**clptDecimls).toFixed(2)} CLPT
                   </div>
               </div>

               <div class="info-item">
                   <div class="name">
                       Earned
                   </div>
                   <div class="value">
                       { totalEarned.div(10**clptDecimls).toFixed(2)} CLPT
                   </div>
               </div>
               <!--            <div class="info-item">-->
               <!--                <div class="name">-->
               <!--                    Accumulated Rewards-->
               <!--                </div>-->
               <!--                <div class="value">-->
               <!--                    549,485.5998-->
               <!--                </div>-->
               <!--            </div>-->
               <!--            <div class="info-item">-->
               <!--                <div class="name">-->
               <!--                    Latest Rewards-->
               <!--                </div>-->
               <!--                <div class="value">-->
               <!--                    85.54-->
               <!--                </div>-->
               <!--            </div>-->
           </div>
           <div class="staked-box">
               <div class="title">
                   Staked({normalStakeArr.length})
               </div>
               <div class="stake-content">
                   {#each normalStakeArr as item}
                       <div class="stake-item">
                           <div class="line">
                               <span>Period</span>
                               <strong>
                                   {item.period}-Day
                               </strong>
                           </div>
                           <div class="line">
                               <span>Balance</span>
                               <strong>
                                   {BigNumber(item.balance).div(10 ** clptDecimls).toFixed(2)}
                               </strong>
                           </div>
                           <div class="line">
                               <span>Start time</span>
                               <strong>
                                   {getDate(item.period_start)}
                               </strong>
                           </div>
                           <div class="line">
                               <span>End time</span>
                               <strong>
                                   {getDate(item.period_end)}
                               </strong>
                           </div>
                           <div class="automatic">
                               <span>Automatic Reinvestment </span>
                               <BeSwitch on:change={()=>{autoIntevetment(item.index, true)}}
                                         activeColor="#623CE7"></BeSwitch>
                           </div>
                           {#if getIsCanWithdraw(item.period_end)}
                               <div class="mint-btn" on:click={()=>{unstake(item.index)}}>
                                   Withdraw
                               </div>
                           {/if}

                       </div>
                   {/each}
               </div>
           </div>
           <div class="staked-box">
               <div class="title">
                   Matured Stakes({autoStakeArr.length})
               </div>
               <div class="stake-content">
                   {#each autoStakeArr as item}
                       <div class="stake-item">
                           <div class="line">
                               <span>Period</span>
                               <strong>
                                   {item.period}-Day
                               </strong>
                           </div>
                           <div class="line">
                               <span>Balance</span>
                               <strong>
                                   {BigNumber(item.balance).div(10 ** clptDecimls).toFixed(2)}
                               </strong>
                           </div>
                           <div class="line">
                               <span>Start time</span>
                               <strong>
                                   {getDate(item.period_start)}
                               </strong>
                           </div>
                           <div class="line">
                               <span>End time</span>
                               <strong>
                                   {getDate(item.period_end)}
                               </strong>
                           </div>
                           <div class="automatic">
                               <span>Automatic Reinvestment </span>
                               <BeSwitch checked on:change={()=>{autoIntevetment(item.index, false   )}}
                                         activeColor="#623CE7"></BeSwitch>
                           </div>
                           {#if getIsCanWithdraw(item.period_end)}
                               <div class="mint-btn" on:click={()=>{unstake(item.index)}}>
                                   Withdraw
                               </div>
                           {/if}

                       </div>
                   {/each}
               </div>
           </div>
           <div class="staked-box">
               <div class="title">
                   History Stakes({historyArr.length})
               </div>
               <div class="stake-content">
                   {#each historyArr as item}
                       <div class="stake-item">
                           <div class="line">
                               <span>Period</span>
                               <strong>
                                   {item.period}-Day
                               </strong>
                           </div>
                           <div class="line">
                               <span>Balance</span>
                               <strong>
                                   {BigNumber(item.balance).div(10 ** clptDecimls).toFixed(2)}
                               </strong>
                           </div>
                           <div class="line">
                               <span>Start time</span>
                               <strong>
                                   {getDate(item.period_start)}
                               </strong>
                           </div>
                           <div class="line">
                               <span>End time</span>
                               <strong>
                                   {getDate(item.period_end)}
                               </strong>
                           </div>

                       </div>
                   {/each}
               </div>
           </div>
       </div>
    {/if}
</div>

<style lang="scss">
  .coin-select {
    width: 1100px;
    margin: 0 auto;
    border-bottom: 1px solid #EAEAEA;
    margin-bottom: 50px;
    display: flex;
    justify-content: center;

    .coin-select-content {
      display: flex;
      justify-content: space-between;
      width: 400px;

      .select-item {
        width: 50%;
        height: 50px;
        text-align: center;
        font-weight: bold;
        cursor: pointer;
        font-size: var(--weightFont);
        color: #282626;

        &.active {
          border-bottom: 1px solid #623DE7;
          color: #623CE7;
        }
      }
    }
  }

  .staked-box {
    max-height: 800px;
    overflow-y: auto;
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

    .stake-content {
      .stake-item {
        margin-top: 30px;
        padding: 10px;
        border: 1px solid #333;

        .line {
          padding: 5px 0;
          display: flex;
          justify-content: space-between;
        }

        .mint-btn {
          width: 160px;
          margin: 0 auto;
          font-size: 18px;
          height: 30px;
          line-height: 30px;
        }
      }
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

  .staking {
    width: 1200px;
    margin: 0 auto;

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

    .select-list {
      display: flex;
      width: 400px;
      margin: var(--weightFont) auto 30px;
      justify-content: space-between;

      .select-item {
        width: 190px;
        height: 50px;
        background: #FFFFFF;
        cursor: pointer;
        box-shadow: 0px 2px 3px 0px rgba(0, 0, 0, 0.01), 0px 8px 7px 0px rgba(0, 0, 0, 0.01), 0px var(--weightFont) 13px 0px rgba(0, 0, 0, 0.01), 0px 39px 25px 0px rgba(0, 0, 0, 0.02), 0px 65px 47px 0px rgba(0, 0, 0, 0.02), 0px 100px 80px 0px rgba(0, 0, 0, 0.03);
        border-radius: 10px 10px 10px 10px;
        border: 1px solid #F3F2F2;
        line-height: 50px;
        text-align: center;
        font-weight: bold;
        font-size: var(--weightFont);
        color: #282626;

        &.active {
          background: #623CE7;
          color: #fff;
        }
      }
    }

    .header-list {
      display: flex;
      width: 100%;
      justify-content: space-between;

      .list-item {
        cursor: pointer;
        min-width: 15%;
        display: block;

        background: #FEFEFF;
        box-shadow: 0px 2px 3px 0px rgba(41, 72, 152, 0.01), 0px 9px 7px 0px rgba(41, 72, 152, 0.02), 0px 22px 14px 0px rgba(41, 72, 152, 0.03), 0px 42px 28px 0px rgba(41, 72, 152, 0.03), 0px 71px 51px 0px rgba(41, 72, 152, 0.04), 0px 109px 87px 0px rgba(41, 72, 152, 0.05);
        border-radius: 11px 11px 11px 11px;
        padding: 30px;

        &.active {
          background: #623CE7;
          color: #fff;

          .item-name {
            color: #fff;
          }

          .detail {
            color: #fff;

            .in-line {
              .name, .value {
                color: #FFFFFF;
              }
            }
          }
        }

        .item-name {
          font-family: Roboto, Roboto;
          font-weight: bold;
          font-size: var(--bigFont);
          color: #282626;
        }

        .detail {
          .in-line {
            display: flex;
            justify-content: space-between;
            margin-top: 25px;

            .name {
              font-size: var(--weightFont);
              color: #999999;
            }

            .value {
              font-weight: bold;
              font-size: var(--weightFont);
              color: #333333;
            }
          }
        }
      }
    }

    .staking-box {
      width: 556px;
      position: relative;
      z-index: 1;
      background: #FEFEFF;
      box-shadow: 0px 2px 3px 0px rgba(41, 72, 152, 0.01), 0px 9px 7px 0px rgba(41, 72, 152, 0.02), 0px 22px 14px 0px rgba(41, 72, 152, 0.03), 0px 42px 28px 0px rgba(41, 72, 152, 0.03), 0px 71px 51px 0px rgba(41, 72, 152, 0.04), 0px 109px 87px 0px rgba(41, 72, 152, 0.05);
      border-radius: 11px 11px 11px 11px;
      padding: 30px;
      margin: 50px auto;

      .staking-box-title {
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

    .automatic {
      display: flex;
      justify-content: space-between;
      margin-top: 20px;

      span {
        font-weight: 500;
        font-size: var(--weightFont);
        color: #929292;
      }
    }

  }

  @media screen and (max-width: 800px) {
    .staking {
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

      .staking-box {
        width: 100%;

        .staking-box-title {
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
