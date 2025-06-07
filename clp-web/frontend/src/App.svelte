<script>
    import Header from "./components/Header.svelte";
    import routes from "./routes";
    import Router from 'svelte-spa-router';
    import Footer from "./components/Footer.svelte";
</script>

<main>
    <div class="app-box">
        <div class="bg-box">
            <div class="bg1"></div>
            <div class="bg2"></div>
            <div class="bg3"></div>
        </div>
        <div class="app-content">
            <Header/>
            <div class="route-view">
                <div class="tip">
                    Note: This is a testnet. Please use the Faucet to get test tokens. Data may reset, but all your test contributions will be recorded.
                </div>
                <Router {routes}/>
            </div>
            <Footer/>
        </div>

    </div>

</main>

<style lang="scss">


  .app-box {
    .route-view {
      flex-grow: 1;
      padding: 50px 0;
      .tip{
        width: 1200px;
        margin: 10px auto 30px;
        font-size: 18px;
        padding: var(--weightFont);
        border-radius: 10px;
        background: #f68e00;
        color: #FFFFFF;
      }
    }

    .app-content {
      overflow: hidden;
      display: flex;
      flex-direction: column;
      min-height: 100vh;
      position: relative;
      z-index: 1;
    }
  }

  .bg-box {
    z-index: 0;
    position: fixed;
    left: 0;
    top: 0;
    width: 100%;
    height: 100%;

    .bg1 {
      width: 30vw;
      height: 30vw;
      position: absolute;
      left: 0;
      top: 0;
      background: url("images/overview_bg3.png") no-repeat;
      background-size: 100%;
    }

    .bg2 {
      background: url("images/overview_bg2.png") no-repeat;
      width: 30vw;
      height: 30vw;
      position: absolute;
      right: 0;
      top: 0;
      background-size: 100%;

    }

    .bg3 {
      position: absolute;
      width: 30vw;
      height: 30vw;
      bottom: -20vw;
      left: 50%;
      background: url("images/overview_bg1.png") no-repeat;
      background-size: 100%;

    }
  }


  :root{
    --bigFont:24px;
    --weightFont:20px;
    --normalFont:18px;
  }
  @media screen and (max-width: 1400px) {
    :root{
      --bigFont:20px;
      --weightFont:18px;
      --normalFont:14px;
    }
    .tip{
      width: 900px!important;
      font-size: 16px!important;
      text-wrap: wrap;
    }
  }
  @media screen and (max-width: 800px) {
    .app-box{
      padding: var(--weightFont) 0;
    }
    .tip{
      width: 90%!important;

      text-wrap: wrap;
    }
  }

</style>
