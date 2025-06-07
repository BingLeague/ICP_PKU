<script>
    import {link, location} from "svelte-spa-router"

    let activeIndex = 0;
</script>

<div class="footer-box">
    <div class="m-nav-list">
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
    </div>
    <a href="/">
        <img class="logo"  src="images/logo.png" alt=""/>
    </a>
    <div class="right-part">
<!--        Crato liquidity protocol.co-->
    </div>
</div>


<style lang="scss">
  .footer-box {
    padding: var(--weightFont) 0;
    width: 1200px;
    margin: 0 auto;
    display: flex;
    align-items: center;
    justify-content: space-between;

    .logo {
      width: 200px;
      height: 46px;
    }

    .right-part {
      font-size: var(--bigFont);
      color: #999999;
      font-family: Roboto, Roboto;
    }

    .m-nav-list {
      display: none;
    }
  }
  @media screen and (max-width: 1400px) {
    .footer-box{
      width: 900px;
      .logo{
        width: 150px;
        height: 36px;
      }
    }

  }
  @media screen and (max-width: 800px) {
    .footer-box {
      width: 100%;
      flex-direction: column;
      align-items: center;

      .logo {
        width: 130px;
        height: auto;
        margin: 0 auto;
      }
      .m-nav-list{
        display: block;
        right: 0;
        width: 100%;
        padding-top: 50px;
        padding-bottom: 50px;
        .nav-item{
          width: 100%;
          padding: var(--weightFont);
          font-weight: bold;
          color: #333333;
          display: block;
          font-size: 16px;
          border-bottom:1px solid #D1D2EE;;
          text-align: center;
        }
      }

      .right-part {
        display: none;
      }
    }

  }
</style>
