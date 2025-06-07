<script>
    import {link, location} from "svelte-spa-router"
    import {onMount} from "svelte";
    import ConnectBtn from "./ConnectBtn.svelte";

    let isShowMMenu = false;
    let activeIndex = 0;
    onMount(async () => {
        console.log($location)
    })
</script>

<div class="header-box">
    <a href="/">
        <img class="logo" src="images/logo.png" alt=""/>
    </a>
    <div class="nav-list">
        <a use:link={"/"} class="nav-item {$location=='/'?'active':''}">
            Overview
        </a>
        <a use:link={"/vaults"} class="nav-item {$location=='/vaults'?'active':''}">
            Vaults
        </a>
        <a use:link={"/borrow" } class="nav-item {$location=='/borrow'?'active':''}">
            Borrow
        </a>
        <a use:link={"/pool" } class="nav-item {$location=='/pool'?'active':''}">
            Liquidation Pool
        </a>
        <a use:link={"/staking" } class="nav-item {$location=='/staking'?'active':''}">
            Staking
        </a>
<!--        <a use:link={"/governance" } class="nav-item {$location=='/governance'?'active':''}">-->
<!--            Governance-->
<!--        </a>-->
    </div>
    <a use:link={"/faucet" }  class="faucet-btn">
        FAUCET
    </a>
    <ConnectBtn />
    <svg t="1717561182621" class="icon menu-btn" viewBox="0 0 1024 1024" version="1.1"
         xmlns="http://www.w3.org/2000/svg"
         on:click={()=>{isShowMMenu=true}}
         p-id="1463" width="36" height="36">
        <path d="M64 64h896v128H64V64z m0 768h896v128H64v-128z m0-384h896v128H64V448z" fill="#623CE7"
              p-id="1464"></path>
    </svg>

    {#if isShowMMenu}
        <div class="mask" on:click={()=>{isShowMMenu=false}}></div>
        <div class="m-nav-list animate__animated animate__fadeInRight"  on:click={()=>{isShowMMenu=false}}>
            <a use:link={"/"} class="nav-item {$location=='/'?'active':''}">
                Overview
            </a>
            <a use:link={"/vaults"} class="nav-item {$location=='/vaults'?'active':''}">
                Vaults
            </a>
            <a use:link={"/borrow" } class="nav-item {$location=='/borrow'?'active':''}">
                Borrow
            </a>
            <a use:link={"/pool" } class="nav-item {$location=='/pool'?'active':''}">
                Liquidation Pool
            </a>
            <a use:link={"/staking" } class="nav-item {$location=='/staking'?'active':''}">
                Staking
            </a>
            <a use:link={"/governance" } class="nav-item {$location=='/governance'?'active':''}">
                Governance
            </a>
            <a use:link={"/faucet" }  class="faucet-btn-m">
                FAUCET
            </a>
        </div>

    {/if}

</div>


<style lang="scss">
  .header-box {
    padding: var(--weightFont) 0;
    width: 1400px;
    margin: 0 auto;
    display: flex;
    align-items: center;
    justify-content: space-between;
    .faucet-btn ,.faucet-btn-m{
      width: 110px;
      height: 36px;
      text-align: center;
      color: #5a1907;
      font-weight: bold;
      border-radius: 6px;
      line-height: 36px;
      cursor: pointer;
      background: #fff;
      box-shadow: 0px 0px 6px #eee;
      font-size: 16px;
    }
    .faucet-btn-m{
      margin: 0 auto;
      background: #f68e00   ;
      display: block;
    }
    .logo {
      width: 200px;
      height: 46px;
    }

    .nav-list {
      display: flex;
      padding: 3px;
      align-items: center;

      .nav-item {
        cursor: pointer;
        border-radius: 10px;
        margin-right: 50px;
        font-weight: bold;
        user-select: none;
        font-size: var(--weightFont);
        color: #000000;
        line-height: 28px;
        text-align: left;
        font-style: normal;
        text-transform: none;

        &.active {
          color: #623CE7;
        }

        &:nth-last-child(1) {
          margin-right: 0;
        }
      }
    }
  }

  .menu-btn {
    display: none;
  }
  @media screen and (max-width: 1400px) {
    .header-box{
      width: 1100px;
      .logo{
        width: 150px;
        height: 36px;
      }
      .nav-list{
        .nav-item{
          font-size: 16px;
          margin-right: 30px;
        }
      }
      .faucet-btn{
        width: 90px;
      }
    }
  }
  @media screen and (max-width: 800px) {
    .faucet-btn{
      display: none;
    }
    .header-box {
      width: 100% !important;
      padding: var(--weightFont) 10px;

      .nav-list {
        display: none !important;
      }

      .logo {
        width: 130px;
        height: auto;
      }

      .menu-btn {
        display: block;
        margin-left: -50px;
      }
    }
    .mask{
      position: fixed;
      top: 0;
      left: 0;
      background: rgba(0, 0, 0, 0.3);
      width: 100vw;
      height: 100vh;
      z-index: 100;
    }
    .m-nav-list {
      position: fixed;
      top: 0;
      z-index: 101;
      background: #fff;
      right: 0;
      width: 70vw;
      height: 100vh;
      padding-top: 50px;

      .nav-item {
        width: 100%;
        padding: var(--weightFont);
        display: block;
        font-size: var(--weightFont);
        color: #000000;
      }
    }
  }
</style>
