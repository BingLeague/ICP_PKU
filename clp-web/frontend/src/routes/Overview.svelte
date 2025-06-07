<script>
    import {AuthClient} from "@dfinity/auth-client";
    import {afterUpdate, onMount} from "svelte";
    import {
        auth, createActor, plugWallet, whitelist, host,
        VAULT_CANISTER_ID, LEDGER_CANISTER_ID, verifyConnection, CKBTC_CANISTER_ID, CKETH_CANISTER_ID, POOL_CANISTER_ID
    } from "../store/auth";
    import {idlFactory as vaultActorIDL} from "../../declarations/vault/vault.did.js";
    import {idlFactory as poolActorIDL} from "../../declarations/reserve-pool/reserve-pool.did.js";

    import {HttpAgent} from "@dfinity/agent/lib/cjs/agent";
    import {canisters, createCanisterActor, userBalances} from "../store/store";
    import BigNumber from "bignumber.js";

    BigNumber.config({ROUNDING_MODE: BigNumber.ROUND_DOWN});
    /** @type {AuthClient} */
    let cusdDecimals = 8;
    let iiPrincipal = '';
    let isGetedData, isConneted;

    let authType = "anonymous";
    let btcDeposit = 0, ethDeposit = 0, liqTotalAssets = 0, circulatingCUSDVal = BigNumber(0);
    let vaultActor, poolActor;
    let poolAssets = 0, btcAssets, ethAssets, liquidationPool = 0;

    const plugCreateActor = async () => {

        await verifyConnection()
        iiPrincipal = window.ic.plug.agent.principal;
        vaultActor = await window.ic.plug.createActor({
            canisterId: VAULT_CANISTER_ID,
            interfaceFactory: vaultActorIDL
        });

        console.log({VAULT_CANISTER_ID, CKBTC_CANISTER_ID, CKETH_CANISTER_ID, LEDGER_CANISTER_ID})
        console.log({vaultActor})
        isConneted = true
    }
    onMount(async () => {
        // Use II as actor
        authType = "II";

        // II must display principle, since it is unique.
        iiPrincipal = $auth.principal;


        if (window?.ic?.plug && false) {
            await plugCreateActor()
        } else {

            // Create canister actors
            const authClient = await AuthClient.create();
            const identity = authClient.getIdentity();
            const agent = new HttpAgent({identity, host});

            if (process.env.DFX_NETWORK === 'local')
                agent.fetchRootKey();
            if ($auth.loggedIn) {
                iiPrincipal = $auth.principal;
            }
            vaultActor = createCanisterActor(agent, vaultActorIDL, VAULT_CANISTER_ID);
            poolActor = createCanisterActor(agent, poolActorIDL, POOL_CANISTER_ID);

            isConneted = true
        }

        getData()

    });
    afterUpdate(() => {
        if ($auth.loggedIn && !isGetedData && isConneted) {
            isGetedData = true
            iiPrincipal = $auth.principal;
            getData()
        }
    });
    const getUserUnderlyingBalances = async () => {
        try {
            const res = await poolActor.stats()
            liquidationPool = BigNumber(res[0]).div(BigNumber(10).pow(cusdDecimals))
        } catch (e) {
            console.error(e)
        }
    }
    const getData = async () => {
        getUserUnderlyingBalances()
        try {
            const balanceRes = await vaultActor.underlyingList()
            console.log(balanceRes)
            let circulatingCUSD = BigNumber(0)
            if (balanceRes) {
                for (let i = 0; i < balanceRes.length; i++) {
                  let [token, underlying] = balanceRes[i];
                    const decimals = underlying.decimals
                    if (process.env.CKBTC_CANISTER_ID == token.toString()) {
                        btcDeposit = BigNumber(underlying.deposit).div(BigNumber(10).pow(decimals))
                        circulatingCUSD = circulatingCUSD.plus(underlying.borrow)
                        btcAssets = btcDeposit.multipliedBy(underlying.price).div(BigNumber(10).pow(8))
                    }
                    if (process.env.CKETH_CANISTER_ID == token.toString()) {
                        ethDeposit = BigNumber(underlying.deposit).div(BigNumber(10).pow(decimals))
                        circulatingCUSD = circulatingCUSD.plus(underlying.borrow)
                        ethAssets = ethDeposit.multipliedBy(underlying.price).div(BigNumber(10).pow(8))
                    }
                }
                poolAssets = BigNumber(ethAssets).plus(btcAssets)
                liqTotalAssets = BigNumber(btcDeposit).plus(ethDeposit)
                circulatingCUSDVal = circulatingCUSD
            }
        } catch (e) {
            console.error(e)
        }
    }
</script>
<div class="overview">

    <div class="header-list">
        <div class="list-item">
            <div class="name">
                Collateral TVL
            </div>
            <div class="value">
                ${BigNumber(poolAssets).toFixed(2)}
            </div>
        </div>
        <div class="list-item">
            <div class="name">
                Circulating CUSD
            </div>
            <div class="value">
                ${BigNumber(circulatingCUSDVal).div(10 ** cusdDecimals).toFixed(2)}
            </div>
        </div>
        <div class="list-item">
            <div class="name">
                Liquidation Pool
            </div>

            <div class="value">
                {BigNumber(liquidationPool).toFixed(2)} CUSD
            </div>
        </div>
    </div>

    <div class="part-title">
        HOW IT WORKS?
    </div>
    <div class="content">
        <div class="left-part">
            <div class="content-item">
                <div class="item-logo">
                    <img class="icon" src="images/overview_content_icon1.png" alt=""/>
                </div>
                <div class="name">
                    Deposit Collateral Assets
                </div>
                <img class="img-row" src="images/overview-row.png" alt=""/>
            </div>
            <div class="content-item">
                <div class="item-logo">
                    <img class="icon" src="images/overview_content_icon2.png" alt=""/>
                </div>
                <div class="name">
                    Mint CUSD
                </div>
                <img class="img-row row1" src="images/overview-row.png" alt=""/>
                <img class="img-row row2" src="images/overview-row.png" alt=""/>

            </div>
        </div>
        <div class="right-part">
            <div class="content-item">
                <div class="item-logo">
                    <img class="icon" src="images/overview_content_icon3.png" alt=""/>
                </div>
                <div class="name">
                    Deposit to <br/> Liquidation Pool
                </div>
            </div>
            <div class="content-item">
                <div class="item-logo">
                    <img class="icon" src="images/overview_content_icon4.png" alt=""/>
                </div>
                <div class="name">
                    Use in ICP ecosystem
                </div>
            </div>
        </div>

    </div>
</div>

<style lang="scss">

  .overview {
    width: 1100px;
    margin: 0 auto;

    .header-list {
      display: flex;
      width: 100%;
      justify-content: space-between;

      .list-item {
        min-width: 26%;
        background: #FEFEFF;
        box-shadow: 0px 2px 3px 0px rgba(41, 72, 152, 0.01), 0px 9px 7px 0px rgba(41, 72, 152, 0.02), 0px 22px 14px 0px rgba(41, 72, 152, 0.03), 0px 42px 28px 0px rgba(41, 72, 152, 0.03), 0px 71px 51px 0px rgba(41, 72, 152, 0.04), 0px 109px 87px 0px rgba(41, 72, 152, 0.05);
        border-radius: 11px 11px 11px 11px;
        padding: 50px 30px;
        display: block;
        text-wrap: wrap;

        .name {
          font-family: Roboto, Roboto;
          font-weight: 500;
          font-size: var(--bigFont);
          color: #929292;
          line-height: 28px;
        }

        .value {
          margin-top: var(--weightFont);
          font-family: Roboto, Roboto;
          font-weight: bold;
          font-size: var(--bigFont);
          color: #282626;
        }
      }
    }

    .part-title {
      font-family: Roboto, Roboto;
      font-weight: bold;
      font-size: 30px;
      color: #000000;
      line-height: 35px;
      font-style: normal;
      text-transform: none;
      text-align: center;
      margin-top: 100px;
    }

    .content {
      display: flex;
      justify-content: space-between;

      .content-item {
        position: relative;

        padding: var(--weightFont);
        width: 310px;
        height: 160px;
        background: #FEFEFF;
        box-shadow: 0px 2px 3px 0px rgba(41, 72, 152, 0.01), 0px 9px 7px 0px rgba(41, 72, 152, 0.02), 0px 22px 14px 0px rgba(41, 72, 152, 0.03), 0px 42px 28px 0px rgba(41, 72, 152, 0.03), 0px 71px 51px 0px rgba(41, 72, 152, 0.04), 0px 109px 87px 0px rgba(41, 72, 152, 0.05);
        border-radius: 11px 11px 11px 11px;
        display: flex;
        align-items: center;
        justify-content: center;
        flex-direction: column;

        .icon {
          height: 68px;
        }

        .name {
          font-family: Roboto, Roboto;
          font-weight: bold;
          font-size: var(--weightFont);
          color: #282626;
          line-height: 23px;
          text-align: center;
          font-style: normal;
          text-transform: none;
        }

      }

      .left-part {
        display: flex;
        margin-top: 160px;

        .content-item {
          &:nth-child(2) {
            margin-left: 100px;
          }

          &:nth-child(1) {
            .img-row {
              position: absolute;
              right: -80px;
              top: calc(50% - 12px);
              width: 50px;
              height: 22px;
            }
          }

          &:nth-child(2) {
            .img-row {
              position: absolute;
              right: -80px;
              width: 50px;
              height: 22px;
            }

            .row1 {
              top: 5px;
            }

            .row2 {
              bottom: 5px;

            }
          }
        }
      }

      .right-part {
        display: flex;
        flex-direction: column;
        margin-left: 100px;

        .content-item {
          &:nth-child(1) {
            height: 180px !important;

            .name {
              margin-top: -5px;
            }
          }

          &:nth-child(2) {
            margin-top: 45px;
          }
        }
      }

    }

  }
  @media screen and (max-width: 1400px) {
    .overview {
      width: 900px;
      .content{
        .content-item{
          width: 220px;
        }
      }
    }
  }
  @media screen and (max-width: 800px) {
    .overview {
      width: 100%;

      .header-list {
        display: block;
        padding: 0 10px;

        .list-item {
          margin-top: var(--weightFont);

          .name {
            font-size: 18px;
          }

          .value {
            font-size: var(--weightFont);
          }
        }

      }

      .part-title {
        font-size: 26px;
      }
    }
    .content {
      flex-direction: column;

      .left-part, .right-part {
        justify-content: center !important;
        align-items: center;

        .content-item {
          margin-left: 0 !important;
          margin-top: 70px !important;
          position: relative;


          .img-row {
            transform: rotate(90deg);
          }
        }
      }

      .left-part {
        flex-direction: column;
        margin-top: 30px !important;

        .content-item {
          &:first-child {
            margin-top: var(--weightFont) !important;
          }

          &:first-child {
            .img-row {
              top: auto !important;
              bottom: -50px;
              left: calc(50% - 26px) !important;
            }
          }

          &:last-child {
            .row1 {
              top: auto !important;
              bottom: -50px !important;
              left: var(--weightFont) !important;
            }

            .row2 {
              top: auto !important;
              bottom: -50px !important;
              left: auto !important;
              right: var(--weightFont) !important;
            }
          }
        }
      }

      .right-part {
        flex-direction: row !important;
        margin-left: 0 !important;
        justify-content: space-between !important;
        margin-top: 30px;
        padding: 15px;

        .content-item {
          width: 48% !important;
          height: 180px !important;
          margin-top: 40px !important;

          .item-icon {
            .logo {
              width: 50px;
            }
          }

          .name {
            font-size: 18px;
          }
        }
      }
    }
  }
</style>
  