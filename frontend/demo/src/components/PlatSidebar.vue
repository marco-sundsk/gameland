<template>
  <div class="plat-sidebar">
    <b-sidebar
      id="sidebar-backdrop"
      title="Sidebar with backdrop"
      backdrop-variant="transparent"
      no-header
      backdrop
      shadow
      right
    >
      <template #default="{ hide }">
        <div class="sidebar-header">
          <button @click="hide">
            <img src="../assets/img/icon-close.png" alt="">
          </button>
        </div>
        <div class="sidebar-body">
          <div class="body-title">
            STATUS
            <img src="../assets/img/icon-menu-status.png" alt="" width="15">
          </div>
          <div class="body-main">
            <p>
              <img src="../assets/img/icon-user.png" alt="">
              Player registered: {{contractInfo.account_num}}
            </p>
            <p>
              <img src="../assets/img/icon-games.png" alt="">
              Mini-game hosted: {{contractInfo.shop_num}}
            </p>
            <p>
              <img src="../assets/img/icon-coin.png" alt="">
              Gamecoin minted: {{contractInfo.total_supply | nearToNum}}
            </p>
            <p>
              <img src="../assets/img/icon-bet-num.png" alt="">
              Collateral：{{contractInfo.total_collateral | nearToNum}} Near
            </p>
            <p>
              <img src="../assets/img/icon-benifit.png" alt="">
              Zone Profits: {{contractInfo.sudoer_profit | nearToNum}} Near
            </p>
          </div>
          <div class="body-date" style="font-size: 12px; line-height: 30px; font-weight: 200;">
            Note: All data are managed on blockchain.
          </div>
        </div>
      </template>
    </b-sidebar>

    <b-sidebar
      id="sidebar-contract"
      title="Sidebar with backdrop"
      backdrop-variant="transparent"
      no-header
      backdrop
      shadow
      right
    >
      <template #default="{ hide }">
        <div class="sidebar-header">
          <button @click="hide">
            <img src="../assets/img/icon-close.png" alt="">
          </button>
        </div>
        <div class="sidebar-body">
          <div class="body-title">
            RULES
          </div>
          <div class="body-main">
            <p>
              Gamecoin Price: {{contractInfo.mint_price / 1000}}
            </p>
            <p>
              Play commission rate: {{getSudoer(contractInfo.sudoer_fee_play, contractInfo.shop_fee_play)}}
            </p>
            <p>
              Winner commission rate: {{getSudoer(contractInfo.sudoer_fee_win, contractInfo.shop_fee_win)}}
            </p>
            <p>
              Gamecoin commission rate: {{getSudoer(contractInfo.burn_ratio)}}
            </p>
            <p>
              Zone owner: {{contractInfo.owner}}
            </p>
          </div>
          <div class="body-date" style="font-size: 12px; line-height: 30px; font-weight: 200;">
            Note: All data are managed on blockchain.
          </div>
        </div>
      </template>
    </b-sidebar>
  </div>
</template>

<script>
export default {
  props: {
    contractInfo: {
      require: true
    }
  },
  computed: {
    getSudoer () {
      return (sudoer, shop) => {
        if (!sudoer) return ''
        if (!shop) {
          const {denominator, numerator} = sudoer
          return numerator + '/' + denominator
        } else {
          const sudoerNumerator = sudoer.numerator
          const shopNumerator = shop.numerator
          // const {denominator, numerator} = sudoer
          return (shopNumerator + sudoerNumerator) + '/' + '1000'
        }
      }
    }
  }
}
</script>

<style>
.plat-sidebar .b-sidebar {
  border-radius: 0px !important;
  background-color: #494949 !important;
  color: #f8ae1c !important;
  text-align: left !important;
  width: 100%;
  max-width: 500px !important;
  z-index: 9999;
}
.plat-sidebar .b-sidebar .sidebar-header {
  display: flex;
  width: 100%;
  justify-content: flex-end;
  padding-top: 20px;
  padding-right: 20px;
}
.plat-sidebar .b-sidebar .sidebar-header button {
  background-color: transparent;
  border: none;
}
.plat-sidebar .b-sidebar .sidebar-body {
  position: absolute;
  left: 50%;
  top: 50%;
  transform: translate(-50%, -50%);
  width: 80%;
  /* margin: 0 auto; */
}
.plat-sidebar .b-sidebar .sidebar-body .body-title {
  line-height: 60px;
  /* border-bottom: 1px solid #f8ae1c; */
  font-size: 20px;
  font-weight: 500;
}

.plat-sidebar .b-sidebar .sidebar-body .body-main {
  padding: 30px 0;
  border-bottom: 1px solid #f8ae1c;
  border-top: 1px solid #f8ae1c;
}
.plat-sidebar .b-sidebar .sidebar-body .body-main p {
  display: flex;
  width: 100%;
  align-items: center;
}
.plat-sidebar .b-sidebar .sidebar-body .body-main p img {
  margin-right: 10px;
}
</style>