<template>
  <div class="home">
    <header>
      <b-container fluid class="bv-example-row">
        <!-- <b-navbar toggleable="lg" type="dark" variant="info"> -->
          <b-row>
            <b-col cols="8">
              <b-row>
                <div class="user-name">欢迎： {{currentUser.accountId}}</div>
                <div class="balance ml-auto">余额 {{gptBalance | nearToNum}} GPT</div>
              </b-row>
            </b-col>
            <b-col cols="4">
              <b-row>
                <b-button variant="info" class="ml-auto" @click="signIn" v-if="isSignedIn">sign Out</b-button>
                <b-button variant="info" class="ml-auto" @click="signIn" v-if="!isSignedIn">sign In</b-button>
              </b-row>
            </b-col>
          </b-row>
        <!-- </b-navbar> -->
      </b-container>
    </header>
    <main>
      <b-container fluid class="bv-example-row">
        <b-row>
          <b-col cols="3" class="game-info">
            <div class="game-info-title">当前游戏信息</div>
            <div class="game-info-winning">当前奖池： {{contractInfo.jack_pod | nearToNum}} GPT</div>
            <div class="game-info-pol">第{{contractInfo.current_round}}轮</div>
            <div class="game-info-tutorial">玩一次需要{{contractInfo.play_fee | nearToNum}}</div>
          </b-col>
          <b-col cols="9" class="game-wrap">
            <b-row class="maket-wrap">
              <b-col
                ref="maketItem"
                v-for="(item, index) in maketList"
                :key="index"
                cols="6"
                lg="2"
                md="3"
                sm="4"
                class="maket-item"
                :class="{ 'bg-yellow': getMaketColor(item) === 'yellow', 'bg-green': getMaketColor(item) === 'green' }"
                @dblclick="play(item)"
                >{{item}}</b-col>
            </b-row>
          </b-col>
        </b-row>
      </b-container>
    </main>
    <footer>
      <div class="footer-message">
        <div class="footer-title">赢取历史信息</div>
        <div class="footer-main">
          <table class="table table-hover" style="border: solid 1px #dee2e6;background: #fff">
            <thead>
              <tr>
                <th scope="col">轮次号</th>
                <th scope="col">中奖时间</th>
                <th scope="col">中间地块</th>
                <th scope="col">中奖者</th>
                <th scope="col">中奖金额</th>
                <!-- <th scope="col">TS</th> -->
              </tr>
            </thead>
            <tbody>
              <tr v-for="item in winList" :key="item.height">
                <th scope="row">{{item.round}}</th>
                <td>{{item.ts}}</td>
                <td>{{item.lucky_number}}</td>
                <td>{{item.user}}</td>
                <td><span style="color: green">{{item.amount | nearToNum}}</span> </td>
                <!-- <td>{{item.ts}}</td> -->
              </tr>
            </tbody>
          </table>
        </div>
      </div>
    </footer>
  </div>
</template>

<script>
import { login, logout } from '../utils/utils.js'
export default {
  name: 'Home',
  data () {
    return {
      maketList: [],
      currentUser: '',
      gptBalance: '',
      employMaketList: '',
      contractInfo: '',
      winList: [],
      gas: Math.pow(10, 14).toString()
    }
  },
  computed: {
    isSignedIn () {
      return window.walletConnection.isSignedIn()
    },
    // 根据是否被占用选择合适的颜色
    getMaketColor () {
      return (index) => {
        for (let key in this.employMaketList) {
          if (index === Number(key) && this.currentUser.accountId === this.employMaketList[key]) {
            return 'green'
          }
          else if (index === Number(key)) {
            return 'yellow'
          }
        }
        return false
      }
    }
  },
  methods: {
    // 登录
    signIn () {
      login()
    },
    // 退登
    signOut () {
      logOut()
    },
    // 获取游戏币余额
    async getGptBalance () {
      const balance = await window.contract_gamecoin.ft_balance_of({
        account_id: this.currentUser.account_id
      })
      this.gptBalance = balance
    },
    // 获取游戏被占用的土地列表
    async getEmployMaketList () {
      const employMaketList = await window.contract_angleLand.get_maket_info()
      console.log(employMaketList)
      this.employMaketList = employMaketList
    },
    // 获取游戏信息
    async getContractInfo () {
      const contractInfo = await window.contract_angleLand.get_contract_info()
      this.contractInfo = contractInfo
      const maketList = []
      for (let i = 0; i < contractInfo.house_count; i++) {
        maketList.push(i + 1)
      }
      this.maketList = maketList
    },
    // 动态控制地块与宽度相同
    changeItemHeight () {
      this.$nextTick(() => {
        const maketItemList = this.$refs.maketItem
        if (this.maketList.length <= 0) return
        let result = 0
        maketItemList.forEach(item => {
          result = result < item.offsetWidth ? item.offsetWidth : result
        })
        maketItemList.forEach(item => {
          item.style.height = result + 'px'
        })
      })
    },
    // 双击成为土块拥有者
    async play (index) {
      console.log(123)
      await window.contract_platform.play({
        shop_id: 'angleland.testnet',
        amount: '1000000000000000000000000',
        op: index.toString()
      }, this.gas)
      this.getGptBalance()
      this.getEmployMaketList()
    },
    // 获取赢家历史列表
    async getWinList () {
      const winList = await window.contract_angleLand.get_win_history({
        from_index: 0,
        limit: 100
      })
      this.winList = winList
    }
  },
  created () {
    this.getWinList()
    this.currentUser = window.currentUser
    this.$nextTick(async () => {
      this.getGptBalance()
      this.getEmployMaketList()
      await this.getContractInfo()
      // this.getMaketList()
      this.$nextTick(() => {
        this.changeItemHeight()
        window.addEventListener('resize', this.changeItemHeight)
      })
    })
  }
}
</script>

<style lang="less" scoped>
  .bg-yellow {
    background-color: yellow;
  }
  .bg-green {
    background-color: green;
  }
  .home {
    header {
      background-color: skyblue;
      height: 80px;
      display: flex;
      align-items: center;
      .row {
        align-items: center;
      }
    }
    main {
      .game-info {
        display: flex;
        background-color: red;
        justify-content: space-around;
        flex-direction: column;
        text-align: center;
      }
      .game-wrap {
        /* height: 600px; */
        /* background-color: blue; */
        .maket-wrap {
          border-collapse:separate;
          .maket-item {
            cursor: pointer;
            border: 1px solid #000;
            box-sizing: border-box;
            /* background-color: blue; */
            &:nth-of-type(odd) {
              /* background-color: yellow; */
            }
          }
        }
      }
    }
    footer {
      .footer-message {
        width: 60%;
        text-align: center;
        margin: 0 auto;
      }
    }
  }
</style>
