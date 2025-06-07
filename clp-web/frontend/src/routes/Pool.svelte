<script>
    import { AuthClient } from "@dfinity/auth-client";
    import { afterUpdate, onMount } from "svelte";
    import { host, POOL_CANISTER_ID } from "../store/auth";
    import { idlFactory as vaultActorIDL } from "../../declarations/vault/vault.did.js";
    import { HttpAgent } from "@dfinity/agent/lib/cjs/agent";
    import {
        canisters,
        createCanisterActor,
        userBalances,
    } from "../store/store";
    import { Principal } from "@dfinity/principal";
    import BigNumber from "bignumber.js";
    import { idlFactory as poolActorIDL } from "../../declarations/reserve-pool/reserve-pool.did.js";
    import { showNotice } from "@brewer/beerui";
    import WithdrawDepositDialog from "../components/WithdrawDepositDialog.svelte";
    import { actorList, iiPrincipal } from "../store/wallet";

    BigNumber.config({ ROUNDING_MODE: BigNumber.ROUND_DOWN });

    /** @type {AuthClient} */
    let isLoading = false;
    let activeRatio;
    let isConnected;
    let isShowWithdrawDialog;
    let isGetedData, isConneted;
    let iiPrincipalString = "",
        principal;
    let btcDeposit = 0,
        ethDeposit = 0,
        liqTotalAssets = 0;
    let btcRate = 0,
        ethRate = 0;
    let btcInfo = {},
        ethInfo;

    let poolAssets = 0,
        totalSub = 0,
        btcAssets,
        ethAssets;
    let liquidationPool = 0;
    let cusdDecimls = 8;
    let cusdBalance = BigNumber(0),
        cusdAllowance = 0,
        depositBalance = BigNumber(0),
        clptBalance = BigNumber(0);
    let vaultActor, cusdActor, poolActor, stakeActor;
    let depositAmount;
    const getAllowance = async () => {
        try {
            const res = await cusdActor.icrc2_allowance({
                account: {
                    owner: principal,
                    subaccount: [],
                },
                spender: {
                    owner: Principal.fromText(POOL_CANISTER_ID),
                    subaccount: [],
                },
            });
            cusdAllowance = res.allowance;
        } catch (e) {
            console.error(e);
        }
    };
    const getCUSDBalance = async () => {
        try {
            const res = await cusdActor.icrc1_balance_of({
                owner: principal,
                subaccount: [],
            });
            cusdBalance = BigNumber(res).div(BigNumber(10).pow(cusdDecimls));
        } catch (e) {
            console.error(e);
        }
    };
    const getUserInfo = async () => {
        try {
            const res = await poolActor.userInfo(principal);
            totalSub = res.total_sub;
        } catch (e) {
            console.error(e);
        }
    };

    const getClptBalance = async () => {
        try {
            const res = await stakeActor.balance(principal);
            clptBalance = BigNumber(res).dividedBy(10 ** 8);
        } catch (e) {
            console.error(e);
        }
    };

    const withdrawClpt = async () => {
        const res = await stakeActor.withdraw();
        if (Object.keys(res) && Object.keys(res)[0] == "Ok") {
            getData();
            //
            showNotice({
                type: "success",
                title: "Withdraw success!",
                message: "Withdraw success!",
            });
        } else {
            showNotice({
                type: "warning",
                title: "Withdraw error",
                message:
                    "Withdraw error" + res.Err ? Object.keys(res.Err)[0] : "",
            });
        }
    };

    const getUserUnderlyingBalances = async () => {
        try {
            const userUnderlyingBalances =
                await poolActor.userUnderlyingBalances(principal);
            console.log(userUnderlyingBalances);
        } catch (e) {
            console.error(e);
        }
        try {
            const res = await poolActor.stats();
            liquidationPool = BigNumber(res[0]).div(
                BigNumber(10).pow(cusdDecimls),
            );
            poolAssets = BigNumber(res[1]).div(BigNumber(10).pow(cusdDecimls));
        } catch (e) {
            console.error(e);
        }
    };
    const getUserBalance = async () => {
        try {
            const res = await poolActor.userBalance(principal);
            depositBalance = BigNumber(res).div(BigNumber(10).pow(cusdDecimls));
        } catch (e) {
            console.error(e);
        }
    };
    const setRatioAmount = (ratio) => {
        activeRatio = ratio;
        depositAmount = BigNumber(cusdBalance)
            .multipliedBy(ratio)
            .div(100)
            .toFixed(6);
    };

    const getData = () => {
        getCUSDBalance();
        getAllowance();
        getUserBalance();
        getUserUnderlyingBalances();
        getUserInfo();
        getClptBalance();
    };
    const handleApprove = async () => {
        const amount = BigNumber(10)
            .pow(cusdDecimls)
            .multipliedBy(depositAmount * 100)
            .toFixed(0);
        const res = await cusdActor.icrc2_approve({
            amount: parseInt(amount),
            created_at_time: [],
            expected_allowance: [],
            expires_at: [],
            fee: [],
            from_subaccount: [],
            memo: [],
            spender: {
                owner: Principal.fromText(POOL_CANISTER_ID),
                subaccount: [],
            },
        });
    };
    const handleDeposit = async () => {
        if (!isConneted) {
            showNotice({
                type: "warning",
                title: "Deposit Failed",
                message: "Please login",
            });
            return;
        }
        if (!depositAmount) {
            showNotice({
                type: "warning",
                title: "Deposit Failed",
                message: "Please input stake amount",
            });
            return;
        }
        try {
            isLoading = true;
            let amount = BigNumber(depositAmount)
                .multipliedBy(BigNumber(10).pow(cusdDecimls))
                .toFixed(0);
            if (BigNumber(cusdAllowance).lt(amount)) {
                await handleApprove();
            }
            const res = await poolActor.deposit(parseInt(amount));
            if (Object.keys(res) && Object.keys(res)[0] == "Ok") {
                getData();
                //
                showNotice({
                    type: "success",
                    title: "Deposit success!",
                    message: "Deposit success!",
                });
            } else {
                showNotice({
                    type: "warning",
                    title: "Deposit error",
                    message:
                        "Deposit error" + res.Err
                            ? Object.keys(res.Err)[0]
                            : "",
                });
            }
        } catch (e) {
            console.error(e);
            showNotice({
                type: "warning",
                title: "Deposit error!",
                message: "Deposit error",
            });
        } finally {
            isLoading = false;
        }
    };
    onMount(async () => {
        // Use II as actor

        // II must display principle, since it is unique.

        const authClient = await AuthClient.create();
        const identity = authClient.getIdentity();
        const agent = new HttpAgent({ identity, host });

        if (process.env.DFX_NETWORK === "local") agent.fetchRootKey();
        iiPrincipal.subscribe((value) => {
            if (value) {
                iiPrincipalString = Principal.fromText(value.toString());
                principal = iiPrincipalString;
            }
        });
        actorList.subscribe((value) => {
            if (value.isGetActor) {
                vaultActor = value.vaultActor;
                poolActor = value.poolActor;
                cusdActor = value.cusdActor;
                stakeActor = value.stakeActor;
                isConnected = true;

                getData();
            }
        });
        vaultActor = createCanisterActor(
            agent,
            vaultActorIDL,
            process.env.VAULT_CANISTER_ID,
        );
        poolActor = createCanisterActor(agent, poolActorIDL, POOL_CANISTER_ID);
        try {
            const balanceRes = await vaultActor.underlyingList();
            if (balanceRes) {
                for (let i = 0; i < balanceRes.length; i++) {
                    let [token, underlying] = balanceRes[i];
                    if (process.env.CKBTC_CANISTER_ID == token.toString()) {
                        btcDeposit = BigNumber(underlying.deposit).div(
                            10 ** underlying.decimals,
                        );
                        btcAssets = btcDeposit
                            .multipliedBy(underlying.price)
                            .div(BigNumber(10).pow(8));
                    }
                    if (process.env.CKETH_CANISTER_ID == token.toString()) {
                        ethDeposit = BigNumber(underlying.deposit).div(
                            10 ** underlying.decimals,
                        );
                        ethAssets = ethDeposit
                            .multipliedBy(underlying.price)
                            .div(BigNumber(10).pow(8));
                    }
                }
                // poolAssets = BigNumber(ethAssets).plus(btcAssets)

                liqTotalAssets = BigNumber(btcDeposit).plus(ethDeposit);
                isConneted = true;
            }
        } catch (e) {
            console.error(e);
        }
    });
</script>

<div class="pool">
    {#if isShowWithdrawDialog}
        <WithdrawDepositDialog
            on:closeMessage={() => {
                (isShowWithdrawDialog = false), getData();
            }}
            paramsObj={{
                depositBalance,
                decimals: cusdDecimls,
            }}
        />
    {/if}
    <div class="header-list">
        <div class="list-item">
            <div class="name">Liquidation Pool</div>
            <div class="value">
                <!--{BigNumber(btcDeposit).toFixed(3)} BTC-->
                <!--{BigNumber(ethDeposit).toFixed(2)} ETH-->
                {liquidationPool.toFixed(2)} CUSD
            </div>
        </div>
        <div class="list-item">
            <div class="name">Liquidated Assets</div>
            <div class="value">
                ${BigNumber(poolAssets).toFixed(2)}
            </div>
        </div>
        <div class="list-item">
            <div class="name">Weighted APR</div>
            <div class="value">0%</div>
        </div>
    </div>
    <div class="deposit-box">
        <div class="ratio-select-box">
            <div
                class="ratio-select-item {activeRatio == 25 ? 'active' : ''}"
                on:click={() => {
                    setRatioAmount(25);
                }}
            >
                25%
            </div>
            <div
                class="ratio-select-item {activeRatio == 50 ? 'active' : ''}"
                on:click={() => {
                    setRatioAmount(50);
                }}
            >
                50%
            </div>
            <div
                class="ratio-select-item {activeRatio == 75 ? 'active' : ''}"
                on:click={() => {
                    setRatioAmount(75);
                }}
            >
                75%
            </div>
            <div
                class="ratio-select-item {activeRatio == 100 ? 'active' : ''}"
                on:click={() => {
                    setRatioAmount(100);
                }}
            >
                MAX
            </div>
        </div>
        <div class="input-box">
            <input type="text" placeholder="0" bind:value={depositAmount} />
            <div class="input-tip">CUSD</div>
        </div>
        <div class="input-detail">Minimum mint amount 100CUSD</div>
        <div class="balance-box">
            <div class="balance-box-name">Balance</div>
            <div class="balance-box-value">
                {cusdBalance.toFixed(2)} CUSD
            </div>
        </div>
        {#if depositAmount && BigNumber(depositAmount).lt(100)}
            <button class="mint-btn" disabled> Minimum 100CUSD </button>
        {:else}
            <button
                class="mint-btn"
                disabled={isLoading}
                on:click={() => {
                    handleDeposit();
                }}
            >
                Deposit
                {#if isLoading}
                    <img class="loading-icon" src="/images/loading.svg" />
                {/if}
            </button>
        {/if}
    </div>
    <div class="my-share">
        <div class="share-title">My Shares</div>
        <div class="share-list">
            <div class="share-item">
                <div class="share-item-content">
                    <div class="share-item-name">Amount</div>
                    <div class="share-item-value">
                        {depositBalance.toFixed(2)} CUSD
                    </div>
                </div>

                <div
                    class="share-item-btn mint-btn"
                    on:click={() => {
                        isShowWithdrawDialog = true;
                    }}
                >
                    Withdraw
                </div>
            </div>
            <div class="share-item">
                <div class="share-item-content">
                    <div class="share-item-name">CLPT Earned</div>
                    <div class="share-item-value">
                        {clptBalance.toFixed(2)} CLPT
                    </div>
                </div>

                <div class="share-item-btn mint-btn" on:click={()=>{
                    withdrawClpt()
                }}>Withdraw</div>
            </div>
            <div class="share-item" style="display: none">
                <div class="share-item-content">
                    <div class="share-item-name">Assets from liquidation</div>
                    <div class="share-item-value">
                        {totalSub} CUSD
                    </div>
                </div>
                <div class="share-item-btn mint-btn">Withdraw</div>
            </div>
        </div>
    </div>
</div>

<style lang="scss">
    .pool {
        width: 1200px;
        margin: 0 auto;

        .header-list {
            display: flex;
            width: 100%;
            justify-content: space-between;

            .list-item {
                display: block;
                min-width: 26%;
                background: #fefeff;
                box-shadow:
                    0px 2px 3px 0px rgba(41, 72, 152, 0.01),
                    0px 9px 7px 0px rgba(41, 72, 152, 0.02),
                    0px 22px 14px 0px rgba(41, 72, 152, 0.03),
                    0px 42px 28px 0px rgba(41, 72, 152, 0.03),
                    0px 71px 51px 0px rgba(41, 72, 152, 0.04),
                    0px 109px 87px 0px rgba(41, 72, 152, 0.05);
                border-radius: 11px 11px 11px 11px;
                padding: 50px 30px;

                .name {
                    font-family: Roboto, Roboto;
                    font-weight: 500;
                    font-size: var(--bigFont);
                    color: #929292;
                    line-height: 28px;
                }

                .value {
                    margin-top: 20px;
                    font-family: Roboto, Roboto;
                    font-weight: bold;
                    font-size: var(--bigFont);
                    color: #282626;
                }
            }
        }

        .deposit-box {
            width: 556px;
            position: relative;
            z-index: 1;
            background: #fefeff;
            box-shadow:
                0px 2px 3px 0px rgba(41, 72, 152, 0.01),
                0px 9px 7px 0px rgba(41, 72, 152, 0.02),
                0px 22px 14px 0px rgba(41, 72, 152, 0.03),
                0px 42px 28px 0px rgba(41, 72, 152, 0.03),
                0px 71px 51px 0px rgba(41, 72, 152, 0.04),
                0px 109px 87px 0px rgba(41, 72, 152, 0.05);
            border-radius: 11px 11px 11px 11px;
            padding: 40px 30px;
            margin: 50px auto;

            .balance-box {
                display: flex;
                margin-top: 20px;
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

        .my-share {
            .share-title {
                font-weight: bold;
                font-size: 30px;
                color: #000000;
            }

            .share-list {
                margin-top: 10px;

                .share-item {
                    height: 80px;
                    background: #fefeff;
                    box-shadow:
                        0px 2px 3px 0px rgba(41, 72, 152, 0.01),
                        0px 9px 7px 0px rgba(41, 72, 152, 0.02),
                        0px 22px 14px 0px rgba(41, 72, 152, 0.03),
                        0px 42px 28px 0px rgba(41, 72, 152, 0.03),
                        0px 71px 51px 0px rgba(41, 72, 152, 0.04),
                        0px 109px 87px 0px rgba(41, 72, 152, 0.05);
                    border-radius: 11px 11px 11px 11px;
                    display: flex;
                    padding: 0 20px;
                    justify-content: space-between;
                    align-items: center;
                    margin-top: 20px;

                    .share-item-content {
                        display: flex;
                        justify-content: space-between;
                        flex-grow: 1;
                    }

                    .share-item-name {
                        font-weight: 600;
                        font-size: var(--weightFont);
                        color: #333333;
                    }

                    .share-item-value {
                        flex-grow: 1;
                        display: flex;
                        justify-content: flex-end;
                        padding-right: 100px;
                        font-weight: 600;
                        font-size: var(--weightFont);
                        color: #333333;
                    }

                    .share-item-btn {
                        width: 120px;
                        height: 40px;
                        box-shadow: 0px 15px 20px 0px rgba(98, 60, 231, 0.35);
                        line-height: 40px;
                        margin-top: 0;
                    }
                }
            }
        }
    }

    @media screen and (max-width: 1400px) {
        .pool {
            width: 900px;
        }
    }

    @media screen and (max-width: 800px) {
        .pool {
            width: 100%;
            padding: 0 10px;

            .header-list {
                flex-direction: column;

                .list-item {
                    width: 100%;
                    margin-top: 30px;

                    .name {
                        font-size: 18px;
                    }

                    .value {
                        font-size: 22px;
                    }
                }
            }

            .deposit-box {
                width: 100%;
            }

            .my-share {
                .share-title {
                    font-size: 26px;
                }
            }

            .share-list {
                .share-item {
                    height: 130px !important;

                    .share-item-content {
                        flex-direction: column;

                        .share-item-name {
                            font-size: 16px;
                            color: #929292;
                        }

                        .share-item-value {
                            margin-top: 20px !important;
                            font-size: 16px;
                            justify-content: flex-start;
                        }
                    }
                }
            }
        }
    }
</style>
