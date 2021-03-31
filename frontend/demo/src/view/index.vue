<template>
  <div class="index">
    <div class="header">
      <b-navbar class="container" type="dark">
        <!-- 平台元信息 -->
        <div class="col-6 nav-item">
          <platform :contractInfo="contractInfo"></platform>
        </div>
        <!-- 用户信息 -->
        <!-- <div class="col-md-4 nav-item">
          <user-block :contractInfo="contractInfo" :currentUser="currentUser"></user-block>
        </div> -->

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
    </div>
    <!-- 游戏信息 -->
    <div class="game">
      <game-list></game-list>
    </div>
    <!-- 注册一栏 -->
    <div class="register-wrap">
      <div class="register-message">Game Zone Open Beta Test, Fill application from to get free game coins.</div>
      <button class="register">Apply Now</button>
    </div>
    <!-- footer -->
    <div class="footer">
      <footers></footers>
    </div>
  </div>
</template>

<script>
import getConfig from "../config"
import { login,logout } from "../utils"
import UserBlock from '../components/UserBlock'
import Platform from '../components/Platform'
import GameList from '../components/GameList'
import Footers from '../components/Footer'

const nearConfig = getConfig("development")
window.networkId = nearConfig.networkId;

export default {
  name: "App",
  created() {
    if (window.currentUser) {
      this.currentUser = window.currentUser
    }
    this.getContractInfo()
    document.title = "GAMELAND - NEARDICE"
  },
  components: {
    UserBlock,
    Platform,
    GameList,
    Footers
  },
  data () {
    return {
      rollCount: 1,
      contractInfo: {},
      currentUser: {
        accountId: '',
        balance: ''
      }
    }
  },
  computed: {
    isSignedIn () {
      return window.walletConnection.isSignedIn()
    }
  },
  methods: {
    login() {
      console.log("calling utils.login")
      login()
    },
    logout: logout,
    async getContractInfo () {
      const contractInfo = await window.contract_gamecoin.get_contract_info()
      this.contractInfo = contractInfo
    },
    async updateUser () {
      await window.getCurrentUser()
      this.currentUser = window.currentUser
    }
  }
}
</script>

<style scoped>
.index .header {
  background: url('../assets/img/header-bg.png') no-repeat;
  height: 120px;
  background-size: cover;
  display: flex;
  align-items: center;
}
.index .header{
  color: #fff;
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
