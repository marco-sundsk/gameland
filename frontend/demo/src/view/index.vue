<template>
  <div class="index">
    <div class="bg-dark">
      <b-navbar class="container py-3" type="dark">
        <!-- 平台元信息 -->
        <div class="col-md-4 nav-item">
          <platform :contractInfo="contractInfo"></platform>
        </div>
        <!-- 用户信息 -->
        <div class="col-md-4 nav-item">
          <user-block :contractInfo="contractInfo" :currentUser="currentUser"></user-block>
        </div>

        <!-- <div class="col-md-3">
          <form v-on:submit.prevent="buyDice" class="shadow mt-1">
            <fieldset ref="fieldset" class="pt-1">
              <div class="pb-1">
                <span class="text-white">Spend </span>
                <select name="rollCount" v-model="rollCount" id="roll" class="ml-2 mr-2">
                  <option value="1" selected="selected">1</option>
                  <option value="2">2</option>
                  <option value="3">3</option>
                  <option value="4">4</option>
                  <option value="5">5</option>
                  <option value="10">10</option>
                  <option value="50">50</option>
                </select>
                <span class="text-white"> Near</span>
                <button id="buy_dice" class="btn btn-danger btn-sm ml-2">
                  Buy GPT
                </button>
              </div>
            </fieldset>
          </form>
        </div> -->

        <div class="col-md-4 nav-item">
          <b-navbar-nav>
            <button class="btn btn-info" v-on:click="logout" v-show="isSignedIn">
              Sign Out
            </button>
            <button class="btn btn-info" v-on:click="login" v-show="!isSignedIn">
              Sign In
            </button>
          </b-navbar-nav>
        </div>
      </b-navbar>
    </div>
    <div class="game">
      <game-list></game-list>
    </div>
  </div>
</template>

<script>
import getConfig from "../config"
import { login,logout } from "../utils"
import UserBlock from '../components/UserBlock'
import Platform from '../components/Platform'
import GameList from '../components/GameList'

const nearConfig = getConfig("development")
window.networkId = nearConfig.networkId;

export default {
  name: "App",
  created() {
    this.currentUser = window.currentUser
    this.getContractInfo()
    document.title = "GAMELAND - NEARDICE"
  },
  components: {
    UserBlock,
    Platform,
    GameList
  },
  data () {
    return {
      rollCount: 1,
      contractInfo: {},
      currentUser: ''
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
.index .bg-dark{
  color: #fff;
}
.bg-dark .container .nav-item {
  display: flex;
  justify-content: center;
}
</style>
