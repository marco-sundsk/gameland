<template>
  <div class="index">
    <div class="header">

      <b-navbar v-if="!isMobile" class="container" type="dark">
        <!-- 平台元信息 -->
        <div class="col-6 nav-item">
          <platform :metadata="metadata" :contractInfo="contractInfo"></platform>
        </div>
        <!-- 用户信息 -->
        <div class="col-6 nav-item">
          <b-navbar-nav class="sign">
            <!-- <button class="btn btn-info" v-on:click="logout" v-show="isSignedIn">
              Sign Out
            </button> -->
            <user-block @logout="logout" :contractInfo="contractInfo" :currentUser="currentUser" @updateUser="updateUser" v-show="isSignedIn"></user-block>
            <button class="btn btn-info" v-on:click="login" v-show="!isSignedIn">
              Sign In
            </button>
          </b-navbar-nav>
        </div>
      </b-navbar>

      <b-navbar v-else class="container" type="dark">
        <div class="plat-collapse" v-b-toggle.sidebar-mobile @click="domStop">
          <img src="../assets/img/Icon-menu.png" alt="" width="30px">
        </div>
        <div class="plat-info">
          <div class="plat-logo-wrap">
            <img src="../assets/img/logo.png" alt="">
          </div>
          <div class="plat-info-wrap">
            <div class="plat-name">{{metadata.name}}</div>
            <div class="plat-desc">{{metadata.description}}</div>
          </div>
        </div>
      </b-navbar>
    </div>
    <!-- 游戏信息 -->
    <div class="game-fa">
      <game-list :isMobile="isMobile"></game-list>
    </div>
    <!-- 注册一栏 -->
    <div v-if="!isMobile" class="register-wrap">
      <div class="register-message">Game Zone Open Beta Test, Fill application from to get free game coins.</div>
      <button class="register">Apply Now</button>
    </div>
    <!-- footer -->
    <div class="footer">
      <footers :metadata="metadata" :isMobile="isMobile"></footers>
    </div>
    <!-- 交易框 -->
    <mobile-sidebar @domStop="domStop" @domMove="domMove" @login="login" @logout="logout" :isMobile="isMobile" :currentUser="currentUser" @getGptBalance="getGptBalance" :gptBalance="gptBalance"></mobile-sidebar>
    <business-sidebar :contractInfo="contractInfo" @getGptBalance="getGptBalance" :gptBalance="gptBalance" :currentUser="currentUser" @updateUser="updateUser"></business-sidebar>
    <plat-sidebar :contractInfo="contractInfo"></plat-sidebar>
  </div>
</template>

<script>
import getConfig from "../config"
import { login,logout } from "../utils"
import UserBlock from '../components/UserBlock'
import Platform from '../components/Platform'
import GameList from '../components/GameList'
import Footers from '../components/Footer'
import BusinessSidebar from '../components/BusinessSidebar'
import PlatSidebar from '../components/PlatSidebar'
import MobileSidebar from '../components/MobileSidebar'

const nearConfig = getConfig("development")
window.networkId = nearConfig.networkId;

export default {
  name: "App",
  components: {
    UserBlock,
    Platform,
    GameList,
    Footers,
    BusinessSidebar,
    PlatSidebar,
    MobileSidebar
  },
  data () {
    return {
      rollCount: 1,
      contractInfo: {},
      currentUser: {
        accountId: '',
        balance: ''
      },
      isMobile: false,
      metadata: {
        version: 0,
        name: '',
        description: ''
      },
      gptBalance: 0
    }
  },
  computed: {
    isSignedIn () {
      return window.walletConnection.isSignedIn()
    }
  },
  methods: {
    domStop () {
      document.documentElement.style.position = 'fixed'
      document.body.style.overflow = 'hidden'
    },
    domMove () {
      document.documentElement.style.position = 'static'
      document.body.style.overflow = ''
    },
    login() {
      login()
    },
    logout: logout,
    // 获取平台信息
    async getMetadata () {
      const metadata = await window.contract_platform.metadata()
      this.metadata = metadata
    },
    // 获取平台详细
    async getContractInfo () {
      const contractInfo = await window.contract_gamecoin.get_contract_info()
      this.contractInfo = contractInfo
    },
    // 更新用户信息
    async updateUser () {
      await window.getCurrentUser()
      this.currentUser = window.currentUser
    },
    // 用户登录时获取游戏币总额
    async getGptBalance () {
      const gptBalance = await window.contract_gamecoin.ft_balance_of({
        account_id: this.currentUser.accountId
      })
      this.gptBalance = gptBalance
    }
  },
  created() {
    if (window.currentUser) {
      this.currentUser = window.currentUser
    }
    this.getMetadata()
    this.getContractInfo()
    if (window.walletConnection.isSignedIn()) {
      this.getGptBalance()
    }
    // document.title = "GAMELAND - NEARDICE"
  },
  mounted () {
    const getDomWidth = () => {
      const domWidth = document.documentElement.offsetWidth || document.body.offsetWidth
      this.isMobile = domWidth <= 800 ? true : false
    }
    window.addEventListener('resize', getDomWidth)
    getDomWidth()
  }
}
</script>

<style scoped>
/* .index {
  .header {
    .plat-info {
      display: flex;
    }
  }
} */
.index .header .plat-collapse {
  position: absolute;
}
.index .header .plat-info {
  display: flex;
  width: 100%;
  justify-content: center;
}
.index .header .plat-info .plat-logo-wrap {
  margin-right: 10px;
}
.index .header .plat-info .plat-name {
  font-family: "Times New Roman", Times, serif;
  font-size: 18px;
  color: #f8ae1c;
  font-weight: 700;
}
.index .header .plat-info .plat-desc {
  font-size: 12px;
  color: #f6c94a;
}
.index .header {
  background: url('../assets/img/header-bg.png') no-repeat;
  height: 120px;
  background-size: cover;
  display: flex;
  align-items: center;
}
.index .header .sign {
  display: flex;
  justify-content: flex-end;
}
.index .register-wrap {
  height: 120px;
  background-color: #494949;
  display: flex;
  width: 100%;
  justify-content: center;
  align-items: center;
}
.index .register-wrap .register-message {
  color: #f0e70a;
  font-size: 16px;
  margin-right: 30px;
}
.index .register-wrap .register {
  color: #000;
  font-size: 16px;
  border: none;
  background-color: #f6ad33;
  width: 172px;
  height: 49px;
  line-height: 49px;
  text-align: center;
  border-radius: 3px;
}
.index button {
  background-color: #f6ad33;
  color: #000;
  border: none;
  padding-left: 25px;
  padding-right: 25px;
}
</style>
