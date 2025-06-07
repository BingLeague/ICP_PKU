<script>
    import {AuthClient,} from "@dfinity/auth-client";

    import {afterUpdate, onMount} from "svelte";
    import {
        auth, createActor, plugWallet, whitelist, host,
        VAULT_CANISTER_ID, LEDGER_CANISTER_ID, CKBTC_CANISTER_ID, POOL_CANISTER_ID,
        CUSD_CANISTER_ID, CKETH_CANISTER_ID,CLPT_CANISTER_ID
    } from "../store/auth";
    import {idlFactory as vaultActorIDL, idlFactory as backendIDL} from "../../declarations/vault/vault.did.js";
    import {showNotice,} from '@brewer/beerui'
    import {sha224} from 'js-sha256';
    import {connector, getConnectorPrincipal, WalletConnector, getConnectorIsConnected} from "../connector/index";
    import {idlFactory as ckbtcActorIDL} from "../../declarations/icr1/ckbtc.did.js";
    import {isConnected, iiPrincipal, actorList, userPrincipal} from "../store/wallet";
    import {idlFactory as poolActorIDL} from "../../declarations/reserve-pool/reserve-pool.did.js";
    import {idlFactory as ledgerIDL} from "../../declarations/ledger/ledger.did.js";
    import {idlFactory as StakingIDL} from "../../declarations/staking_pool/staking_pool.did.js";

    /** @type {AuthClient} */
    let client;
    let accountID;
    let didCopyDepositAddress;
    let iiPrincipalString, walletIsConnected;
    let isShowDialog;
    let isShowConnectDialog = false;
    // Plug wallet connection request
    onMount(async () => {
        // Internet Identity:q
        isConnected.subscribe(value => {
            walletIsConnected = value;
        });
        iiPrincipal.subscribe(value => {
            iiPrincipalString = value;
            if (value) {
                accountID = principalToAccount(iiPrincipalString).toString()
            }
        });
        autoConnect()
    });


    /**
     * Converts a Principal ID to an Account ID using SHA-224 hashing.
     *
     * @param {string} principalId - The Principal ID in hexadecimal format.
     * @param {string} [subaccount='0000000000000000000000000000000000000000000000000000000000000000'] - The Subaccount ID in hexadecimal format (optional).
     * @returns {string} - The derived Account ID in hexadecimal format.
     */
    function principalToAccount(principalId, subaccount = '0000000000000000000000000000000000000000000000000000000000000000') {
        // Helper function to convert hex string to byte array
        const hexStringToByteArray = (hexString) => {
            const byteArray = [];
            for (let i = 0; i < hexString.length; i += 2) {
                byteArray.push(parseInt(hexString.substr(i, 2), 16));
            }
            return byteArray;
        };

        // Helper function to convert byte array to hex string
        const byteArrayToHexString = (byteArray) => {
            return byteArray
                .map((byte) => ('0' + (byte & 0xff).toString(16)).slice(-2))
                .join('');
        };

        // Convert principalId and subaccount to byte arrays
        const principalBytes = hexStringToByteArray(principalId);
        const subaccountBytes = hexStringToByteArray(subaccount);

        if (subaccountBytes.length !== 32) {
            throw new Error('Subaccount must be 32 bytes long.');
        }

        // Concatenate principalBytes and subaccountBytes
        const data = new Uint8Array(principalBytes.length + subaccountBytes.length);
        data.set(principalBytes);
        data.set(subaccountBytes, principalBytes.length);

        // Compute SHA-224 hash of the data
        const hash = sha224.update(data).array();

        // Convert hash to hexadecimal string
        const accountIdHex = byteArrayToHexString(hash);

        return accountIdHex;
    }


    async function getActors() {
        const vaultActor = await connector.createActor(VAULT_CANISTER_ID, vaultActorIDL)
        const poolActor = await connector.createActor(POOL_CANISTER_ID, poolActorIDL)
        const ckbtcActor = await connector.createActor(CKBTC_CANISTER_ID, ckbtcActorIDL)

        const ckethActor = await connector.createActor(CKETH_CANISTER_ID, ckbtcActorIDL)
        const cusdActor = await connector.createActor(CUSD_CANISTER_ID, ckbtcActorIDL)
        const ledgerActor = await connector.createActor(process.env.LEDGER_CANISTER_ID, ledgerIDL);
        const stakeActor = await connector.createActor(process.env.STAKING_CANISTER_ID, StakingIDL);
        console.log(CLPT_CANISTER_ID)
        const clptActor = await connector.createActor(CLPT_CANISTER_ID, ckbtcActorIDL)

        await actorList.update(() => {
            return {
                vaultActor,
                poolActor,
                ckbtcActor,
                ckethActor,
                cusdActor,
                clptActor,
                stakeActor,
                ledgerActor,
                isGetActor: true
            }
        })
    }

    async function autoConnect() {
        const connectType = window.localStorage.getItem("connectType")
        if (!connectType) {
            return
        }
        const new_connector = await WalletConnector.create(connectType);
        const expired = await new_connector.expired();
        if (expired) {
            logout();
            return
        }
        await connector.init(connectType);
        if (!(await connector.isConnected())) {
            await connector.connect();
        } else if (connector.connector) {
            window.icConnector = connector.connector;
        }
        // dealconnected
        isConnected.set(true)
        iiPrincipal.update(() => {
            let res = getConnectorPrincipal()
            iiPrincipalString = res
            return res
        })
        await getActors()
        isShowConnectDialog = false

        return await getConnectorIsConnected();
    }

    async function handleConnect(param) {
        await connector.init(param)
        const isC = await connector.connect()
        isConnected.set(isC)
        if (!isC) {
            showNotice({
                toast: true,
                message: "Please install " + param,
                duration: 3000,
                type: "warning"
            });
        }
        iiPrincipal.update(() => {
            let res = getConnectorPrincipal()
            iiPrincipalString = res
            return res
        })
        await getActors()
        window.localStorage.setItem("connectType", param)
        isShowConnectDialog = false
    }

    function copyDepositAddress(text) {
        if (window.isSecureContext) {
            didCopyDepositAddress = true;
            navigator.clipboard.writeText(text);
            showNotice({
                toast: true,
                message: text + ' Copied!',
                duration: 3000,
                type: "success"
            });

        } else {
            showNotice({
                toast: true,
                message: "Copy error",
                duration: 3000,
                type: "warning"
            });
        }
        setTimeout(() => {
            didCopyDepositAddress = false
        }, 1000)
    };

    async function logout() {
        window.icConnector?.disconnect()
        window.localStorage.setItem("connectType", undefined)
        auth.update(() => ({
            loggedIn: false,
            principal: ''
        }));
        isShowDialog = false
        iiPrincipal.set(undefined)
        isConnected.set(false)
    }
</script>

<div id="connect-btn">

    <div>
        <div style="color: #fff">
            {#if walletIsConnected && iiPrincipalString}
                <!--                <button on:click={logout}>Log out</button>-->
                <button on:click={()=>{isShowDialog=true}}>
                    <span>{iiPrincipalString && iiPrincipalString.length > 0 ? (iiPrincipalString.substring(0, 3) + "..." + iiPrincipalString.substring((iiPrincipalString.length - 5), iiPrincipalString.length)) : ""}
                    </span>
                </button>
                {#if isShowDialog}
                    <div class="mask" on:click={()=>{
                        isShowDialog=false
                    }}>

                    </div>
                    <div class="log-out-dialog">
                        <div class="account-info" on:click={()=>{
                            copyDepositAddress(accountID)
                        }}>
                            <div class="title">
                                Account ID
                            </div>
                            <div class="content">
                                {accountID}
                            </div>
                        </div>
                        <div class="account-info" style="margin-top:10px" on:click={()=>{
                            copyDepositAddress(iiPrincipalString)
                        }}>
                            <div class="title">
                                Principal
                            </div>
                            <div class="content">
                                {iiPrincipalString}
                            </div>
                        </div>
                        <div class="log-out" on:click={()=>{
                        logout()
                    }}>
                            <svg width="20" height="20" viewBox="0 0 20 20" fill="none"
                                 xmlns="http://www.w3.org/2000/svg">
                                <path fill-rule="evenodd" clip-rule="evenodd"
                                      d="M10.0078 1.83008C9.45553 1.83008 9.00781 2.27779 9.00781 2.83008V9.22935C9.00781 9.78164 9.45553 10.2294 10.0078 10.2294C10.5601 10.2294 11.0078 9.78164 11.0078 9.22935V2.83008C11.0078 2.27779 10.5601 1.83008 10.0078 1.83008Z"
                                      fill="#111936"></path>
                                <path fill-rule="evenodd" clip-rule="evenodd"
                                      d="M14.335 3.784C13.8184 3.43542 13.1545 3.66423 12.8823 4.18242C12.6028 4.71456 12.8222 5.3393 13.2748 5.66561C14.0671 6.23673 14.7001 7.00814 15.1052 7.90681C15.6191 9.04671 15.7367 10.3258 15.4393 11.5403C15.1419 12.7548 14.4465 13.8348 13.4641 14.6082C12.4817 15.3817 11.2686 15.8041 10.0183 15.8082C8.76791 15.8123 7.55215 15.3978 6.56469 14.6307C5.57723 13.8637 4.87488 12.7883 4.56954 11.5757C4.2642 10.3632 4.37345 9.0834 4.87986 7.94017C5.27911 7.03888 5.9071 6.26336 6.69559 5.68707C7.14608 5.35782 7.36137 4.73166 7.07844 4.20136C6.80291 3.68495 6.13752 3.46048 5.62322 3.81242C4.43984 4.62222 3.49936 5.74815 2.91409 7.06939C2.21325 8.65154 2.06207 10.4227 2.48463 12.1008C2.9072 13.7788 3.8792 15.2671 5.24577 16.3287C6.61235 17.3902 8.29488 17.9638 10.0253 17.9582C11.7557 17.9525 13.4345 17.3679 14.7941 16.2975C16.1537 15.2271 17.116 13.7324 17.5276 12.0517C17.9392 10.3709 17.7764 8.60076 17.0653 7.02322C16.4714 5.70582 15.5236 4.58606 14.335 3.784Z"
                                      fill="#111936"></path>
                            </svg>
                            <span>Log Out</span>
                        </div>
                    </div>
                {/if}
            {:else}
                <button on:click={
                ()=>{
                    isShowConnectDialog = true
                }
                }>Login
                </button>
            {/if}
        </div>

        <!--        plug identity -->
        {#if isShowConnectDialog}
            <div class="connect-dialog">
                <div class="mask" on:click={()=>{
                        isShowConnectDialog=false
                    }}>
                </div>
                <div class="dialog-content">
                    <h2 class="title">
                        Connect a wallet
                    </h2>
                    <div class="connect-list">
                        <div class="connect-item" on:click={()=>{
                            handleConnect('identity')
                        }}>
                            <div class="name">
                                Internet Identity
                            </div>
                            <div class="icon">
                                <img src="/images/InternetIdentity.svg" alt="">
                            </div>
                        </div>
                        <div class="connect-item" on:click={()=>{
                            handleConnect('plug')
                        }}>
                            <div class="name">
                                Plug
                            </div>
                            <div class="icon">
                                <img src="/images/Plug.svg" alt="">
                            </div>
                        </div>
                        <div class="connect-item" on:click={()=>{
                            handleConnect('bitfinity')
                        }}>
                            <div class="name">
                                Bitfinity
                            </div>
                            <div class="icon">
                                <img width="40" height="40" src="/images/bitfinity.png" alt="">
                            </div>
                        </div>
<!--                        <div class="connect-item" on:click={()=>{-->
<!--                            handleConnect('stoic')-->
<!--                        }}>-->
<!--                            <div class="name">-->
<!--                                Stoic-->
<!--                            </div>-->
<!--                            <div class="icon">-->
<!--                                <img width="40" height="40" src="/images/stoic.png" alt="">-->
<!--                            </div>-->
<!--                        </div>-->
                    </div>
                </div>
            </div>
        {/if}
    </div>
</div>

<style lang="scss">
  #connect-btn {
    position: relative;
  }

  .connect-dialog {
    position: fixed;
    left: 0;
    top: 0;
    display: flex;
    z-index: 100;

    .dialog-content {
      width: 600px;
      position: absolute;
      left: calc(50vw - 300px);
      top: 28vh;
      background: #fff;
      transition: box-shadow 300ms cubic-bezier(0.4, 0, 0.2, 1) 0ms;
      box-shadow: rgba(0, 0, 0, 0.2) 0px 11px 15px -7px, rgba(0, 0, 0, 0.14) 0px 24px 38px 3px, rgba(0, 0, 0, 0.12) 0px 9px 46px 8px;
      border-radius: 12px;
      padding: 30px;

      .connect-list {
        display: flex;
        justify-content: space-between;
        flex-wrap: wrap;
        .connect-item {
          cursor: pointer;
          padding: 20px 10px;
          width: 48%;
          border-radius: 10px;
          display: flex;
          align-items: center;
          background: rgba(202, 202, 202, 0.5);
          justify-content: space-between;
          &:nth-child(n + 3){
            margin-top: 20px;
          }
          &:hover {
            background: rgba(202, 202, 202, 0.7);

          }

          .name {
            font-size: 20px;
          }

          .icon {
            width: 36px;
            height: 36px;
            margin-left: 15px;
          }
        }
      }
    }
  }

  .mask {
    position: fixed;
    left: 0;
    top: 0;
    width: 100vw;
    height: 100vh;
  }

  .log-out-dialog {
    position: absolute;
    width: 350px;
    top: 110%;
    right: 0;
    background: #fff;
    border: 1px solid #F3F2F2;
    border-radius: 10px;
    padding: 15px;
    color: #534364;
    box-shadow: 0px 2px 3px 0px rgba(0, 0, 0, 0.01), 0px 8px 7px 0px rgba(0, 0, 0, 0.01), 0px var(--weightFont) 13px 0px rgba(0, 0, 0, 0.01), 0px 39px 25px 0px rgba(0, 0, 0, 0.02), 0px 65px 47px 0px rgba(0, 0, 0, 0.02), 0px 100px 80px 0px rgba(0, 0, 0, 0.03);
    z-index: 100;

    .account-info {
      cursor: pointer;
      word-break: break-all;
      padding: 12px;
      text-align: left;
      border: 1px solid rgb(239, 239, 255);
      border-radius: 8px;

      .title {
        font-size: 18px;
        font-weight: bold;
        padding-bottom: 10px;
      }
    }

    .log-out {
      display: flex;
      padding-top: 10px;
      margin-top: 5px;
      align-items: center;
      cursor: pointer;

      span {
        color: #111936;
        margin-left: 10px;
      }
    }
  }

  button {
    padding: 8px 32px;
    color: var(--color-black);

    background: #4726bd;

    letter-spacing: 0.35px;
    appearance: none;
    font-size: 15px;
    font-family: Inter, -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto,
    Oxygen-Sans, Ubuntu, Cantarell, "Helvetica Neue", sans-serif;
    font-weight: 600;
    border: 1px solid transparent;
    cursor: pointer;
    border-radius: 5px;

    overflow: hidden;
    white-space: nowrap;
    text-overflow: ellipsis;
    color: #fff;
    position: relative;
  }

  @media screen and (max-width: 800px) {
    .connect-dialog {
      .dialog-content {
        width: 90vw;
        left: 5vw;
      }
    }
  }
</style>
