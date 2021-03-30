<template>
  <div id="root" class="main">
    <div class="bg-dark">
      <b-navbar class="container py-3" type="dark">
        <!-- <img :src="require('./assets/neardice-logo.png')" alt="Near Dice" height="32px" class="mr-1"> -->
        <b-navbar-toggle target="nav-collapse"></b-navbar-toggle>
        <b-collapse id="nav-collapse" is-nav>
          <b-navbar-nav>
            <b-nav-item class="ml-3" href="#"><strong>欢迎： {{accountId}}</strong></b-nav-item>
            <b-nav-item class="ml-3" href="#"><strong>余额： {{this.formatGPT(this.leftCount)}} GPT(s) </strong></b-nav-item>
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
      <SignedIn @getLeftCount="getLeftCount"></SignedIn>
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
  name: "App",
  components: {
    SignedIn,
  },
  data () {
    return {
      leftCount: ''
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

