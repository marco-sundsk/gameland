<template>
  <div>
    <div class="vld-parent">
      <loading :active.sync="isLoading" :can-cancel="true" :is-full-page="fullPage"></loading>
    </div>
    <main>
      <h1 class="shadow py-2" v-show="isSignedIn">
        Welcome {{ accountId }}, You have {{ this.formatGPT(this.leftCount) }} GPT(s) 
      </h1>

      <div class="contianer">
        <div class="row">
          <div class="col-md-3">
            <form v-on:submit.prevent="buyDice" class="shadow mt-5 py-4">
              <fieldset ref="fieldset">
                <div class="form-group py-3">
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
          </div>
          <div class="col-md-6">
            <div class="mt-5">
              <h2 class="text-white text-center">Jackpot:</h2>
              <p class="text-center"><span class="display-3" style="letter-spacing: 0.5rem">{{jackpot}}</span>GPT</p>
              <p class="text-center display-4 text-white">Recent Wins</p>
              <table class="table table-hover" style="border: solid 1px #dee2e6;background: #fff">
                <thead>
                  <tr>
                    <th scope="col">Height</th>
                    <th scope="col">Username</th>
                    <th scope="col">Prize</th>
                    <!-- <th scope="col">TS</th> -->
                  </tr>
                </thead>
                <tbody>
                  <tr v-for="item in winList" :key="item.height">
                    <th scope="row">{{item.height}}</th>
                    <td>{{item.user}}</td>
                    <td><span style="color: green">{{formatAmount(item.amount)}}</span> </td>
                    <!-- <td>{{item.ts}}</td> -->
                  </tr>
                </tbody>
              </table>
            </div>
          </div>
          <div class="col-md-3">
            <form v-on:submit.prevent="rollDice" class="shadow mt-5 py-4">
              <fieldset ref="fieldset">
                <div>
                  <span class="text-white">Select number to bet</span><br>
                  <ul>
                    <li class="number-item"><a :class="active1" @click="chooseNumber(1)">1</a></li>
                    <li class="number-item"><a :class="active2" @click="chooseNumber(2)">2</a></li>
                    <li class="number-item"><a :class="active3" @click="chooseNumber(3)">3</a></li>
                    <li class="number-item"><a :class="active4" @click="chooseNumber(4)">4</a></li>
                    <li class="number-item"><a :class="active5" @click="chooseNumber(5)">5</a></li>
                    <li class="number-item"><a :class="active6" @click="chooseNumber(6)">6</a></li>
                  </ul>
                  <button id="roll_dice"  class="btn btn-warning btn-sm ml-2 mt-2">
                    Roll
                  </button>
                </div>
              </fieldset>
            </form>
          </div>
        </div>
      </div>
    </main>
  </div>
</template>

<script>
import { logout } from "../utils";
// Import component
import Loading from 'vue-loading-overlay';
// Import stylesheet
import 'vue-loading-overlay/dist/vue-loading.css';

export default {
  name: "SignedIn",

  components: {
    Loading
  },
  data: function () {
    return {
      // 100T gas = 10^14
      gas: Math.pow(10, 14).toString(),
      at: "000000000000000000000000",
      savedGreeting: "",
      newGreeting: "",
      leftCount: 0,
      rollCount: 1,
      rollNumber: 1,
      jackpot: "--",
      winList: {},
      active1: "active",
      active2: "",
      active3: "",
      active4: "",
      active5: "",
      active6: "",
      isLoading: false,
      fullPage: true
    };
  },

  created() {
    this.getLeftCount()
    this.getWinHistory()
    this.getContactInfo()
  },

  computed: {
    isSignedIn() {
      return window.walletConnection
        ? window.walletConnection.isSignedIn()
        : false;
    },
    accountId() {
      return window.accountId;
    },
    contractId() {
      return window.contract_platform ? window.contract_platform.contractId : null;
    },
    networkId() {
      return window.networkId;
    },
  },

  methods: {
    chooseNumber(num) {
      console.log(num)
      this.rollNumber = num
      if (num==1) {
        this.active1 = 'active'
        this.active2 = ''
        this.active3 = ''
        this.active4 = ''
        this.active5 = ''
        this.active6 = ''
      }
      if (num==2) {
        this.active1 = ''
        this.active2 = 'active'
        this.active3 = ''
        this.active4 = ''
        this.active5 = ''
        this.active6 = ''
      }
      if (num==3) {
        this.active1 = ''
        this.active2 = ''
        this.active3 = 'active'
        this.active4 = ''
        this.active5 = ''
        this.active6 = ''
      }
      if (num==4) {
        this.active1 = ''
        this.active2 = ''
        this.active3 = ''
        this.active4 = 'active'
        this.active5 = ''
        this.active6 = ''
      }
      if (num==5) {
        this.active1 = ''
        this.active2 = ''
        this.active3 = ''
        this.active4 = ''
        this.active5 = 'active'
        this.active6 = ''
      }
      if (num==6) {
        this.active1 = ''
        this.active2 = ''
        this.active3 = ''
        this.active4 = ''
        this.active5 = ''
        this.active6 = 'active'
      }
    },
    getLeftCount() {
      // get user gamecoin balance
      window.contract_gamecoin
        .ft_balance_of({ account_id: window.accountId })
        .then((leftCount) => {
          this.leftCount = leftCount;
          console.log('query gamecoin balance OK, return' + this.leftCount);
        });
    },
    showDice: function (num) {
      this.diceNum=num
      this.diceStart = !this.diceStart;
    },

    buyDice: async function () {
      if(!this.isSignedIn){
        alert('Please Sign In')
        return
      }
      // fired on form submit button used to update the greeting

      // disable the form while the value gets updated on-chain
      this.$refs.fieldset.disabled = true;

      try {
        // call gameland to buy gamecoin
        await window.contract_platform.buy_playtoken(
          { },
          this.gas,
          parseInt(this.rollCount) + this.at
        );
      } catch (e) {
        console.log(e);
      } finally {
        // re-enable the form, whether the call succeeded or failed
        this.$refs.fieldset.disabled = false;
      }
    },

    rollDice: async function () {
      if(!this.isSignedIn){
        alert('Please Sign In')
        return
      }
      
      if (parseFloat(this.formatGPT(this.leftCount)) < 1){
        alert('Sorry, you need to buy more GPT')
        return
      }

      this.isLoading = true;
      // disable the form while the value gets updated on-chain
      this.$refs.fieldset.disabled = true;

      try {
        // make an update call to the smart contract
        await window.contract_platform
          .play({
            shop_id: "neardice.testnet",
            amount: "1000000000000000000000000",
            op: "" + this.rollNumber,
          },
          this.gas,
          0)
          .then((result) => {
            this.isLoading = false;
            this.getLeftCount();
            this.getWinHistory();

            console.log(result);

            const res = JSON.parse(result);
            // parse res and do following
            if (res.dice_point === res.user_guess) {
              const reward_amount = this.formatAmount(res.reward_amount);
              alert("Congratulations to you, you win " + reward_amount + ' GPT!');
            } else {
              alert("You lose, the number is " + res.dice_point);
            }
            this.jackpot = this.formatAmount(res.jackpod_left);

          });
      } catch (e) {
        console.log(e); //re-throw
      } finally {
        // re-enable the form, whether the call succeeded or failed
        this.$refs.fieldset.disabled = false;
      }
    },

    getWinHistory: async function () {
      try {
        // make an update call to the smart contract
        await window.contract_game
          .get_win_history({
            from_index: 0,
            limit: 20,
          })
          .then((res) => {
            this.winList = res;
            console.log(res);
          });
      } catch (e) {
        console.log(e); //re-throw
      }
    },

    getContactInfo: async function () {
      try {
        // make an update call to the smart contract
        await window.contract_game
          .get_contract_info({})
          .then((res) => {
            this.jackpot = this.formatAmount(res.jack_pod);
            console.log(res);
          });
      } catch (e) {
        console.log(e); //re-throw
      }
    },

    formatAmount: function (amount) {
      const reward_amount = amount.toString();
      const temp_amount = reward_amount.substr(
        0,
        reward_amount.length - 20
      );
      const int_part = temp_amount.substr(0, temp_amount.length - 4);
      const float_part = temp_amount.substr(int_part.length, 4);
      return int_part + "." +float_part;
    },
    formatGPT: function (amount) {
      const amount_str = amount.toString();
      // right trim 20 digits first
      const short_amount_str = amount_str.substr(
        0,
        amount_str.length - 20
      );
      console.log("formatGPT:short_amount_str:" + short_amount_str);
      const int_part = short_amount_str.substr(0, short_amount_str.length - 4);
      const float_part = short_amount_str.substr(int_part.length, 4);
      return int_part + "." +float_part;
    },
    logout: logout,
  },
};
</script>
