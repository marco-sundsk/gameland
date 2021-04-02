<template>
  <div id="root" class="main">
    <div class="bg-dark">
      <b-navbar class="container py-3" type="dark">
        <!-- <img :src="require('./assets/neardice-logo.png')" alt="Near Dice" height="32px" class="mr-1"> -->
        <b-navbar-toggle target="nav-collapse"></b-navbar-toggle>
        <b-collapse id="nav-collapse" is-nav>
          <b-navbar-nav>
            <b-nav-item class="">
              <div class="flex-center">
                <svg style="margin-right: 5px" t="1617356883986" class="icon" viewBox="0 0 1024 1024" version="1.1" xmlns="http://www.w3.org/2000/svg" p-id="3493" width="16" height="16"><path d="M512 561.152c-153.088 0-277.504-124.416-277.504-277.504 0-153.088 124.416-277.504 277.504-277.504s277.504 124.416 277.504 277.504c0 153.088-124.416 277.504-277.504 277.504z m0-478.72c-111.104 0-201.216 90.112-201.216 201.216 0 111.104 90.112 201.216 201.216 201.216s201.216-90.112 201.216-201.216c0-111.104-90.112-201.216-201.216-201.216z" fill="#1296db" p-id="3494"></path><path d="M969.216 1012.736c-20.992 0-38.4-11.264-38.4-32.256 0-230.912-187.904-418.816-418.816-418.816s-418.816 187.904-418.816 418.816c0 20.992-16.896 32.256-38.4 32.256s-38.4-11.264-38.4-32.256c0-272.896 222.208-495.104 495.104-495.104s495.104 222.208 495.104 495.104c0.512 20.992-16.384 32.256-37.376 32.256z" fill="#1296db" p-id="3495"></path><path d="M977.92 1013.248H45.568c-15.872 0-29.184-13.312-29.184-29.184v-7.68c0-15.872 13.312-29.184 29.184-29.184h931.84c15.872 0 29.184 13.312 29.184 29.184v7.68c0.512 15.872-12.8 29.184-28.672 29.184z" fill="#1296db" p-id="3496"></path></svg>
                {{accountId}}
              </div>
              <div class="flex-center">
                <svg style="margin-right: 5px;" t="1617356760965" class="icon" viewBox="0 0 1024 1024" version="1.1" xmlns="http://www.w3.org/2000/svg" p-id="2553" width="16" height="16"><path d="M810.666667 367.786667c0 21.290667-9.088 41.130667-25.770667 58.154666 9.088 2.858667 16.64 8.533333 25.770667 11.349334V367.786667z m-341.333334 89.386666c7.594667 0 13.653333-1.408 21.248-1.408a323.072 323.072 0 0 1 177.493334-52.48c15.146667 0 30.293333 2.816 45.482666 4.224 59.178667-28.373333 95.573333-68.138667 95.573334-110.72v-52.48C810.666667 156.288 658.944 85.333333 469.333333 85.333333 281.216 85.333333 128 156.288 128 244.309333V298.24c0 87.978667 153.216 158.933333 341.333333 158.933333zM366.165333 768c-6.058667-22.698667-10.624-46.848-10.624-70.954667 0-11.349333 1.536-22.698667 3.029334-34.090666C225.109333 645.973333 128 597.674667 128 526.72v89.429333C128 688.512 228.138667 748.117333 366.165333 768z" fill="#1296db" p-id="2554"></path><path d="M363.093333 595.2c11.648-38.826667 30.634667-74.666667 56.917334-106.026667C256.469333 481.706667 128 426.453333 128 341.333333v94.08c0 76.16 99.285333 138.88 235.093333 159.786667z m286.165334-159.786667c-135.808 0-246.784 112-246.784 252.373334S512 938.666667 649.258667 938.666667C786.474667 938.666667 896 826.666667 896 686.293333s-110.933333-250.88-246.741333-250.88zM379.136 822.186667C234.581333 808.746667 128 754.986667 128 675.84v94.08C128 864 275.456 938.666667 456.533333 938.666667c5.845333 0 11.648-1.493333 17.493334-1.493334-39.424-28.373333-71.552-68.693333-94.890667-114.986666z" fill="#1296db" p-id="2555"></path></svg>
                {{this.formatGPT(this.leftCount)}} GPT(s)
              </div>
            </b-nav-item>
            <!-- <b-nav-item class="ml-3" href="#"><strong>{{accountId}}</strong></b-nav-item>
            <b-nav-item class="ml-3" href="#"><strong>{{this.formatGPT(this.leftCount)}} GPT(s) </strong></b-nav-item> -->
            <!-- <b-nav-item href="#" disabled>Rules</b-nav-item> -->
          </b-navbar-nav>

          <b-modal id="modal-1" title="How To Play?" hide-footer>
            <p class="my-1">On home page, user can see the whole status of playground without login, i.e. an NEAR account is not necessary. He would have full imformation about owner account of this contract, dice price, the size of current jackpod and etc.</p>
            <p class="my-2">Then, user can login with NEAR account and buy several gamecoin. With coins bought, he can guess a number and roll dice again and again. If the dice point is equal to his guess, half of jackpod would belong to him. Otherwise the amount he paid for the dice would belong to the jackpod.</p>
            <p class="my-2">During playing, the latest 20 win records would appear and be auto refreshed on screen too.</p>
          </b-modal>

          <!-- Right aligned nav items -->
          <b-navbar-nav class="ml-auto">
            <button class="btn btn-info" style="float: right" v-on:click="back">
              back
            </button>

            <!-- <b-nav-item-dropdown text="Language" right>
              <b-dropdown-item href="#">EN</b-dropdown-item>
              <b-dropdown-item href="#">中文</b-dropdown-item>
            </b-nav-item-dropdown> -->
            <!-- <b-nav-item-dropdown right>
              <template #button-content>
                <em>User</em>
              </template>
              <b-dropdown-item href="#">Profile</b-dropdown-item>
              <b-dropdown-item href="#">Sign Out</b-dropdown-item>
            </b-nav-item-dropdown> -->
          </b-navbar-nav>
        </b-collapse>
      </b-navbar>
    </div>
    <div class="container pt-4">
      <SignedIn @getLeftCount="getLeftCount" :isMobile="isMobile"></SignedIn>
      <!-- <SignedIn /> -->
    </div>

    <footer
      class="bd-footer p-3 p-md-5 mt-5 bg-light text-center text-sm-start"
    >
      <div class="container text-center">
        <p><strong> GAMELAND - NEARDICE DAPP</strong></p>
        <span>Copyright 2021</span>
      </div>
    </footer>
  </div>
</template>

<script>
import "./global.css";
import getConfig from "./config";
import SignedIn from "./components/SignedIn.vue";
import { login,logout } from "./utils";

const nearConfig = getConfig("development");
window.networkId = nearConfig.networkId;

export default {
  created() {
    if (!window.walletConnection.isSignedIn()) {
      window.location.href = window.location.origin
    }
    document.title = "GAMELAND - NEARDICE";
    this.getLeftCount()
  },
  mounted () {
    const getDomWidth = () => {
      const domWidth = document.documentElement.offsetWidth || document.body.offsetWidth
      this.isMobile = domWidth <= 767 ? true : false
    }
    window.addEventListener('resize', getDomWidth)
    getDomWidth()
  },
  name: "App",
  components: {
    SignedIn,
  },
  data () {
    return {
      leftCount: '',
      isMobile: false
    }
  },
  computed: {
    isSignedIn() {
      return window.walletConnection.isSignedIn();
    },
    accountId() {
      return window.accountId;
    },
  },
  methods: {
    back () {
      window.location.href = window.location.origin
    },
    login() {
      console.log("calling utils.login")
      login()
    },
    logout: logout,
    getLeftCount() {
      // get user gamecoin balance
      window.contract_gamecoin
        .ft_balance_of({ account_id: window.accountId })
        .then((leftCount) => {
          this.leftCount = leftCount;
          console.log('query gamecoin balance OK, return' + this.leftCount);
        });
    },
    formatGPT: function (amount) {
      if (!amount) return
      const amount_str = amount.toString();
      if (amount_str.length < 20) {
        return "0";
      } else {
        // right trim 20 digits first
        const short_amount_str = amount_str.substr(
          0,
          amount_str.length - 20
        );
        // console.log("formatGPT:short_amount_str:" + short_amount_str);
        const int_part = short_amount_str.substr(0, short_amount_str.length - 4);
        const float_part = short_amount_str.substr(int_part.length, 4);
        return int_part + "." +float_part;
      }
    }
  }
};
</script>

<style>
  .flex-center {
    display: flex;
    align-items: center;
  }
</style>
