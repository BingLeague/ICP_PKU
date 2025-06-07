<script>
  import { auth, CKBTC_CANISTER_ID, CKETH_CANISTER_ID, host } from '../store/auth';

  import { afterUpdate, onMount } from 'svelte';
  import { AuthClient } from '@dfinity/auth-client';
  import { Secp256k1KeyIdentity } from '@dfinity/identity';
  import { HttpAgent } from '@dfinity/agent/lib/cjs/agent';
  import { idlFactory as ckbtcActorIDL } from '../../declarations/icr1/ckbtc.did.js';
  import { createCanisterActor } from '../store/store';
  import { Principal } from '@dfinity/principal';
  import { showNotice } from '@brewer/beerui';
  import {actorList,iiPrincipal} from "../store/wallet";

  let iiPrincipalString = '';
  let inputValue = '';
  let isGetedData, isConneted;
  let ckbtcActor, ckethActor;
  let isLoading = false;
  onMount(async () => {
    iiPrincipal.subscribe((value)=>{
      if(value){
        iiPrincipalString = Principal.fromText(value.toString());
        inputValue = iiPrincipalString
      }
    })


    const authClient = await AuthClient.create();
    authClient.getIdentity();
    const pemString = `-----BEGIN EC PRIVATE KEY-----
MHQCAQEEIM0eRdc3FpGs/n41ArBZND25a13UDjpQ2m/12Ayx3a8toAcGBSuBBAAK
oUQDQgAEP/DxlnVIkBFxclMzZqe/x3a1+Zqppq9x2a47qmeyI42TZCo7mRY/xNSC
63fU2S4ikHX+itn0ea9dumWOQlm0Xg==
-----END EC PRIVATE KEY-----`;
    // 移除 PEM 的头部和尾部标识
    const pemHeader = '-----BEGIN EC PRIVATE KEY-----';
    const pemFooter = '-----END EC PRIVATE KEY-----';
    const base64String = pemString.replace(pemHeader, '').replace(pemFooter, '').replace(/\s+/g, '');

    // 将 Base64 字符串解码为 Uint8Array
    const binaryString = atob(base64String);
    const binaryArray = Uint8Array.from(binaryString, (char) => char.charCodeAt(0));

    // 提取私钥中的 32 字节部分
    // 这里假设你知道私钥的 32 字节部分在整个密钥中的位置
    // 对于 EC 私钥，它通常从第一个字节开始后的 32 字节
    const privateKeyStartIndex = 7; // 这个索引可能需要根据实际情况调整
    const privateKeyArray = binaryArray.slice(privateKeyStartIndex, privateKeyStartIndex + 32);
    let miner2 = Secp256k1KeyIdentity.fromSecretKey(privateKeyArray);
    console.log({
      miner2: miner2.getPrincipal().toText(),
      host,
    });
    const agent = new HttpAgent({ identity: miner2, host });
    if (process.env.DFX_NETWORK === 'local') agent.fetchRootKey();
    ckbtcActor = createCanisterActor(agent, ckbtcActorIDL, CKBTC_CANISTER_ID);
    ckethActor = createCanisterActor(agent, ckbtcActorIDL, CKETH_CANISTER_ID);

  });

  const transfer = async () => {
    transferCoin(ckbtcActor, 100_000_000,"ckBTC");
    transferCoin(ckethActor, 1000_000_000_000_000_000,"ckETH");
  };

  const transferCoin = async (actor, amount, coinName) => {

    const principal = Principal.fromText(inputValue.toString());
    try {
      isLoading = true
      const res = await actor.icrc1_transfer({
        amount: amount,
        created_at_time: [],
        fee: [],
        from_subaccount: [],
        memo: [],
        to: {
          owner: principal,
          subaccount: [],
        },
      });
      console.log(res)
      if (Object.keys(res) && Object.keys(res)[0] == 'Ok') {
        showNotice({
          type: 'success',
          title: 'Send success!',
          message: "Send " + coinName +" success",
        });
      } else {
        showNotice({
          type: 'warning',
          title: 'Send error!',
          message: "Send " + coinName +" error",
        });
      }
    } catch (e) {
      console.error(e);
    }finally {
      isLoading = false
    }
  };
  afterUpdate(() => {
    if ($auth.loggedIn && !isGetedData && isConneted) {
      isGetedData = true;
    }
  });
</script>

<div class="faucet">
  <div class="title">FAUCET</div>

  <div class="faucet-content">
    <div class="input-title">Wallet Address</div>
    <div class="input-content">
      <input type="text" bind:value={inputValue} />
    </div>
    <!-- svelte-ignore a11y-click-events-have-key-events -->
    <button class="mint-btn" disabled={isLoading} on:click={()=>{transfer()}}>
      Send
      {#if isLoading}
        <img class="loading-icon" src="/images/loading.svg"/>
      {/if}
    </button>
  </div>
</div>

<style lang="scss">
  .faucet {

    .title {
      color: #333;
      font-size: 1.5rem;
      font-weight: bold;
      letter-spacing: 0.05em;
      text-align: center;
      font-family: inherit;
    }

    .faucet-content {
      width: 660px;
      max-width: calc(100% - 40px);
      margin: 30px auto;
      background: #fff;
      padding: 30px;
      border-radius: 30px;

      .input-title {
        line-height: 1rem;
        font-size: 16px;
        padding-bottom: 10px;
      }

      .input-content {
        input {
          height: 50px;
          width: 100%;
          background: rgba(238, 238, 238, 0.73);
          padding: 0 var(--weightFont);
        }
      }

      .mint-btn {
        width: 100%;
      }
    }
  }
</style>
