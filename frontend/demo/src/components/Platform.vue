<template>
  <div class="plat">
    <div class="plat-left">
      <div class="logo">
        <img src="../assets/img/logo.png" alt="">
      </div>
      <div class="info">
        <div class="plat-name">
          <div class="plat-version">v{{metadata.version}}</div>
          {{metadata.name}}
        </div>
        <div class="plat-desc">{{metadata.description}}</div>
      </div>
    </div>
    <div class="plat-right">
      <button v-b-toggle.sidebar-backdrop>
        平台状态
        <img src="../assets/img/icon-menu-status.png" alt="">
      </button>
      <button v-b-toggle.sidebar-contract>平台政策</button>
    </div>

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
            平台状态
            <img src="../assets/img/icon-menu-status.png" alt="" width="15">
          </div>
          <div class="body-main">
            <p>
              <img src="../assets/img/icon-user.png" alt="">
              用户人数：{{contractInfo.account_num}}
            </p>
            <p>
              <img src="../assets/img/icon-games.png" alt="">
              入驻游戏：{{contractInfo.shop_num}}
            </p>
            <p>
              <img src="../assets/img/icon-coin.png" alt="">
              游戏币投放：{{contractInfo.total_supply | nearToNum}}
            </p>
            <p>
              <img src="../assets/img/icon-bet-num.png" alt="">
              抵押规模：{{contractInfo.total_collateral | nearToNum}}
            </p>
            <p>
              <img src="../assets/img/icon-benifit.png" alt="">
              平台盈利：{{contractInfo.sudoer_profit | nearToNum}}
            </p>
          </div>
          <div class="body-date" style="font-size: 12px; line-height: 30px; font-weight: 200;">
            截止日期xxxx
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
            平台政策
          </div>
          <div class="body-main">
            <p>
              币数：{{contractInfo.mint_price}}
            </p>
            <p>
              游戏费：{{getSudoer(contractInfo.sudoer_fee_play, contractInfo.shop_fee_play)}}
            </p>
            <p>
              赢家税：{{getSudoer(contractInfo.sudoer_fee_win, contractInfo.shop_fee_win)}}
            </p>
            <p>
              铸币费：{{getSudoer(contractInfo.burn_ratio)}}
            </p>
            <p>
              平台归属：{{contractInfo.owner}}
            </p>
          </div>
          <div class="body-date" style="font-size: 12px; line-height: 30px; font-weight: 200;">
            截止日期xxxx
          </div>
        </div>
      </template>
    </b-sidebar>
  </div>
</template>

<script>
export default {
  created () {
    this.getMetadata()
    console.log('123', this.contractInfo)
  },
  props: {
    contractInfo: {
      require: true
    }
  },
  data () {
    return {
      metadata: {}
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
  },
  methods: {
    async getMetadata () {
      const metadata = await window.contract_platform.metadata()
      console.log(metadata)
      this.metadata = metadata
    }
  }
}
</script>

<style>
.plat {
  color: #f8ae1c;
  display: flex;
  width: 100%;
  justify-content: space-between;
}
.plat .plat-left {
  display: flex;
  align-items: center;
}
.plat .plat-left .logo {
  margin-right: 10px;
}
.plat .plat-left .info .plat-name .plat-version {
  right: 0;
  top: 0;
  transform: translate(110%, -10%);
  box-shadow: 2px 2px 5px 2px rgb(32, 32, 32);
  position: absolute;
  line-height: 15px;
  padding: 0 5px;
  height: 15px;
  color: #000;
  font-size: 12px;
  text-align: center;
  background-color: #44d690;
  border-radius: 3px 3px 3px 0;
}
.plat .plat-left .info .plat-name {
  padding-top: 5px;
  display: inline-block;
  position: relative;
  font-size: 20px;
  font-weight: 500;
}
.plat .plat-left .info .plat-desc {
  font-size: 14px;
  color: #f6c94a;
}
.plat .plat-right {
  width: 200px;
  justify-content: space-around;
  display: flex;
  align-items: flex-end;
}
.plat .plat-right button {
  color: #f8ae1c;
  background-color: transparent;
  border: none;
}
.plat .b-sidebar {
  border-radius: 0px !important;
  background-color: #494949 !important;
  color: #f8ae1c !important;
  text-align: left !important;
  width: 100%;
  max-width: 500px !important;
}
.plat .b-sidebar .sidebar-header {
  display: flex;
  width: 100%;
  justify-content: flex-end;
  padding-top: 20px;
  padding-right: 20px;
}
.plat .b-sidebar .sidebar-header button {
  background-color: transparent;
  border: none;
}
.plat .b-sidebar .sidebar-body {
  position: absolute;
  left: 50%;
  top: 50%;
  transform: translate(-50%, -50%);
  width: 80%;
  /* margin: 0 auto; */
}
.plat .b-sidebar .sidebar-body .body-title {
  line-height: 60px;
  /* border-bottom: 1px solid #f8ae1c; */
  font-size: 20px;
  font-weight: 500;
}

.plat .b-sidebar .sidebar-body .body-main {
  padding: 30px 0;
  border-bottom: 1px solid #f8ae1c;
  border-top: 1px solid #f8ae1c;
}
.plat .b-sidebar .sidebar-body .body-main p {
  display: flex;
  width: 100%;
  align-items: center;
}
.plat .b-sidebar .sidebar-body .body-main p img {
  margin-right: 10px;
}
</style>