<template>
  <div>
    <div class="vld-parent">
      <loading :active.sync="isLoading" :can-cancel="true" :is-full-page="fullPage"></loading>
    </div>
    <main>
      <div class="contianer">
        <div class="row">
          <div class="col-md-3">
            <form v-on:submit.prevent="buyDice" class="mt-5">
              <table class="table shadow" style="border: solid 1px #dee2e6; color: #fff; text-align: left;">
                <thead>
                  <tr>
                    <th scope="col">Type</th>
                    <!-- <th scope="col">PR</th> -->
                    <th scope="col">odds</th>
                  </tr>
                </thead>
                <tbody>
                  <tr>
                    <td>Big/Small</td>
                    <td>1:1</td>
                  </tr>
                  <tr>
                    <td>Odd/Even</td>
                    <td>1:1</td>
                  </tr>
                  <tr>
                    <td>Specific Triples</td>
                    <td>1:150</td>
                  </tr>
                  <tr>
                    <td>Any Triple</td>
                    <td>1:24</td>
                  </tr>
                  <tr>
                    <td>Dice Combinations</td>
                    <td>1:5</td>
                  </tr>
                  <tr>
                    <td>Specific Doubles</td>
                    <td>1:8</td>
                  </tr>
                </tbody>
              </table>
            </form>
          </div>
          <div class="col-md-5">
            <div class="mt-5">
              <div class="dice-wrap">
                <img v-for="(item, index) in dicePoint" :key="index" :src="diceImg(item)" alt="" width="100px">
                <!-- <img src="../assets/img/6.png" alt="" width="100px">
                <img src="../assets/img/6.png" alt="" width="100px"> -->
              </div>
              <div class="game-over text-center" style="border-bottom: 2px solid #000; min-height: 40px; line-height: 80px; margin-bottom: 40px;">{{gameOver}}</div>
              <!-- 奖池信息 -->
              <h2 class="text-white text-center">Jackpot:</h2>
              <p class="text-center"><span class="display-4" style="letter-spacing: 0.5rem">{{jackpot}}</span>GPT</p>
              <!-- 统计信息 -->
              <!-- <div class="row">
                <div class="col-4">
                  <h5 class="text-white text-center">players:</h5>
                  <p class="text-center"><span class="display-4" style="letter-spacing: 0.5rem">{{statistic.play_count}}</span></p>
                </div>
                <div class="col-4">
                  <h5 class="text-white text-center">winners:</h5>
                  <p class="text-center"><span class="display-4" style="letter-spacing: 0.5rem">{{statistic.winner_count}}</span></p>
                </div>
                <div class="col-4">
                  <h5 class="text-white text-center">rewarded:</h5>
                  <p class="text-center"><span class="display-4" style="letter-spacing: 0.5rem">{{statistic.reward_sum | nearToNum(0)}}</span></p>
                </div>
              </div> -->

              
            </div>
          </div>
          <div class="col-md-4">
            <b-form v-on:submit.prevent="rollDice" class="shadow mt-5">

              <b-tabs pills card>
                <!-- 类型1 -->
                <b-tab title="Big/Small" active>
                  <my-form
                    @rollDice="rollDice"
                    title="The total score will be from 11 to 17 (inclusive) for big, from 4 to 10 (inclusive) for small, with the exception of a triple"
                    type="Big"
                    category="1"
                  >
                  </my-form>
                </b-tab>
                <!-- 类型2 -->
                <b-tab title="Odd/Even">
                  <my-form
                    @rollDice="rollDice"
                    type="Odd"
                    title="The total score will be an odd/even number with the exception of a triple"
                    category="2"
                  ></my-form>
                </b-tab>

                <b-tab title="Specific Triples">
                  <my-form
                    @rollDice="rollDice" 
                    title="A specific number will appear on all three dice"
                    type="Specific"
                    category="3"
                  ></my-form>
                </b-tab>

                <b-tab title="Any Triple">
                  <my-form
                    @rollDice="rollDice"
                    title="Any of the triples will appear"
                    type="Any"
                    category="4"
                  ></my-form>
                </b-tab>

                <b-tab title="Dice Combinations">
                  <my-form
                    @rollDice="rollDice"
                    title="Two of the dice will show a specific combination of two different numbers (for example, a 3 and a 4)"
                    type="Dice"
                    category="5"
                  ></my-form>
                </b-tab>

                <b-tab title="Specific Doubles">
                  <my-form
                    @rollDice="rollDice"
                    title="A specific number will appear on at least two of the three dice"
                    type="SpecificDbl"
                    category="6"
                  ></my-form>
                </b-tab>
              </b-tabs>
            </b-form>
          </div>
        </div>
        <p class="text-center display-4 text-white">Recent Wins</p>
        <table class="table table-hover col-8" style="border: solid 1px #dee2e6;background: #fff; margin: 0 auto;">
          <thead>
            <tr>
              <th scope="col">time</th>
              <th scope="col">category</th>
              <th scope="col">player</th>
              <th scope="col">odds</th>
              <th scope="col">reward</th>
            </tr>
          </thead>
          <tbody>
            <tr v-for="item in winList" :key="item.height">
              <th scope="row">{{item.ts | changeTime}}</th>
              <td>{{categoryToString(item.category)}}</td>
              <td>{{item.user}}</td>
              <td>{{item.odds}}</td>
              <td><span style="color: green">{{item.reward | nearToNum(2)}}</span> </td>
              <!-- <td>{{item.ts}}</td> -->
            </tr>
          </tbody>
        </table>
      </div>
    </main>
  </div>
</template>

<script>
import { logout } from "../utils";
// Import component
import Loading from 'vue-loading-overlay';
import MyForm from './MyForm'
// Import stylesheet
import 'vue-loading-overlay/dist/vue-loading.css';

export default {
  name: "SignedIn",

  components: {
    Loading,
    MyForm
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
      fullPage: true,
      selected: '',
      statistic: {},
      dicePoint: [6, 6, 6],
      gameOver: ''
    };
  },

  created() {
    this.getLeftCount()
    this.getWinHistory()
    this.getContactInfo()
    // this.getStatistic()
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
    diceImg () {
      return item => {
        return require(`../assets/img/${item}.png`)
      }
    },
    categoryToString () {
      return item => {
        switch (item) {
        case 1:
          return 'Big/Small'
        case 2:
          return 'Odd/Even'
        case 3:
          return 'Specific Triples'
        case 4:
          return 'Any Triple'
        case 5:
          return 'Dice Combinations'
        case 6:
          return 'Specific Doubles'
        default :
          return null
        }
      }
    }
  },

  methods: {
    async getStatistic () {
      const statistic = await window.contract_dice.gl_statistic()
      this.statistic = statistic
    },
    getLeftCount() {
      // get user gamecoin balance
      window.contract_gamecoin
        .ft_balance_of({ account_id: window.accountId })
        .then((leftCount) => {
          this.leftCount = leftCount;
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

    rollDice: async function (category, amount, guess1 = 0, guess2 = 0) {
      if(!this.isSignedIn){
        alert('Please Sign In')
        return
      }
      
      if (parseFloat(this.formatGPT(this.leftCount)) < 1){
        alert('Sorry, you need to buy more GPT')
        return
      }

      this.isLoading = true
      // disable the form while the value gets updated on-chain
      // this.$refs.fieldset.disabled = true;

      try {
        // make an update call to the smart contract
        await window.contract_platform
          .play({
            shop_id: 'dicemaster.testnet',
            amount: amount,
            op: `{\"category\": ${category}, \"guess1\": ${guess1}, \"guess2\": ${guess2}, \"guess3\": 0}`,
          },
          this.gas,
          0)
          .then((result) => {
            this.isLoading = false
            const res = JSON.parse(result)
            if (res.ret_code != 0) {
              this.gameOver = res.reason
              return
            }else if (res.reward_amount != '0') {
              this.gameOver = 'You win ' + this.formatAmount(res.reward_amount)
            } else {
              this.gameOver = 'You lose'
            }
            this.getLeftCount()
            this.getWinHistory()
            this.$emit('getLeftCount')
            
            this.dicePoint = res.dice_point
            console.log(res)
            // parse res and do following
      
            this.jackpot = this.formatAmount(res.jackpot_left)

          });
      } catch (e) {
        console.log(e); //re-throw
      } finally {
        // re-enable the form, whether the call succeeded or failed
        // this.$refs.fieldset.disabled = false;
      }
    },

    getWinHistory: async function () {
      try {
        // make an update call to the smart contract
        await window.contract_dice
          .get_win_history({
            from_index: 0,
            limit: 20,
          })
          .then((res) => {
            console.log(res)
            this.winList = res
          });
      } catch (e) {
        console.log(e); //re-throw
      }
    },

    getContactInfo: async function () {
      try {
        // make an update call to the smart contract
        await window.contract_dice
          .get_contract_info({})
          .then((res) => {
            this.jackpot = this.formatAmount(res.jackpot);
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
      
    },
    logout: logout,
  },
};
</script>

<style>
  .dice-wrap {
    display: flex;
    width: 100%;
    justify-content: space-around;
  }
  .game-over {
    color: rgb(222, 235, 45);
    font-size: 30px;
  }
  .nav-link {
    color: rgb(190, 202, 24);
  }
  .nav-link:hover {
    color: rgb(190, 197, 96);;
  }
</style>
