<template>
  <div class="lucky-box container">
    <div class="vld-parent">
      <loading :active.sync="isLoading" :is-full-page="true">
      </loading>
    </div>
    <div class="lucky-main row">
      <div v-if="!isMobile" class="col-md-3 mt-4 mt-md-5 left">
        <!-- <table>
          <thead>
            <tr>
              <td>123</td>
              <td>123</td>
              <td>123</td>
            </tr>
          </thead>
          <tbody>
            <tr>
              <td>123</td>
              <td>123</td>
              <td>123</td>
            </tr>
            <tr>
              <td>123</td>
              <td>123</td>
              <td>123</td>
            </tr>
            <tr>
              <td>123</td>
              <td>123</td>
              <td>123</td>
            </tr>
          </tbody>
        </table> -->
        <div class="game-info-title">当前游戏信息</div>
        <div class="game-info-winning">当前奖池： {{contractInfo.jack_pod | nearToNum}} GPT</div>
        <div class="game-info-pol">第{{contractInfo.current_round}}轮</div>
        <div class="game-info-tutorial">玩一次需要：{{contractInfo.play_fee | nearToNum}} GPT</div>
      </div>
      <div class="col-md-5 center mt-4 mt-md-5 pl-md-4 pr-md-4">
        <div class="center-wrap" ref="centerWrap">
          <div 
            ref="maketItem" 
            v-for="(item, index) in maketList" 
            :key="index" 
            class="maket-item"
            :class="{ 'bg-yellow': getMaketColor(item) === 'yellow', 'bg-green': getMaketColor(item) === 'green', active: active === item }"
            @click="isActive(item)"
            ></div>
        </div>
      </div>
      <div class="col-md-4 mt-4 mt-md-5 right-wrap">
        <div :class="rightStyle">
          <div class="title">The total score will be 4 to 36</div>
          <div v-if="!isMobile" class="info">
            <p>Simple intoductions:</p>
            <p>Ut placerat pharetra ipsum, in dignissim dui dapibus non. Suspendisse potenti. Phasellus volutpat nisi quis.</p>
          </div>
          <div class="button-wrap">
            <button class="cancel" @click="cancel">Cancel</button>
            <button class="confirm" @click="play">Confirm</button>
          </div>
        </div>
        <div class="win-info">{{winInfo}}</div>
      </div>
    </div>
    <div class="row">
      <div class="col-md-3"></div>
      <div class="col-md-5">
        <h2>Recent Winners</h2>
      </div>
      <div class="col-md-4"></div>
    </div>
    <div class="row">
      <div class="col-md-3"></div>
      <div class="col-md-5">
        <div class="win-table">
          <table align="center">
            <thead>
              <tr>
                <th>轮次号</th>
                <th v-if="!isMobile">中奖时间</th>
                <th>中奖地块</th>
                <th>中奖者</th>
                <th>中奖金额</th>
              </tr>
            </thead>
            <tbody>
              <tr v-for="item in winList" :key="item.height">
                <th scope="row">{{item.round}}</th>
                <td v-if="!isMobile">{{item.ts | changeTime}}</td>
                <td>{{item.lucky_number}}</td>
                <td>{{item.user}}</td>
                <td><span style="color: green">{{item.amount | nearToNum}}</span> </td>
                <!-- <td>{{item.ts}}</td> -->
              </tr>
            </tbody>
          </table>
        </div>
      </div>
      <div class="col-md-4"></div>
    </div>
  </div>
</template>

<script>
import Loading from 'vue-loading-overlay';
// Import stylesheet
import 'vue-loading-overlay/dist/vue-loading.css';
export default {
  components: {
    Loading
  },
  data () {
    return {
      maketList: [],
      employMaketList: '',
      contractInfo: '',
      winList: [],
      gas: Math.pow(10, 14).toString(),
      active: '',
      isLoading: false,
      winInfo: ''
    }
  },
  props: ['isMobile', 'currentUser', 'gptBalance'],
  computed: {
    // 根据是否被占用和当前用户id选择合适的颜色
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
    },
    rightStyle () {
      return this.isMobile ? 'mobile-right' : 'right'
    }
  },
  methods: {
    changeItemHeight () {
      this.$nextTick(() => {
        const maketItemList = this.$refs.maketItem
        if (this.maketList.length <= 0) return
        let result = 0
        const centerWrap = this.$refs.centerWrap
        maketItemList.forEach(item => {
          result = result < item.offsetWidth ? item.offsetWidth : result
        })
        maketItemList.forEach(item => {
          item.style.height = result + 'px'
        })
        centerWrap.style.height = result * 6 + (5 * 8) + 'px'
      })
    },
    // 获取游戏被占用的土地列表
    async getEmployMaketList () {
      try {
        const employMaketList = await window.contract_angleLand.get_maket_info()
        this.employMaketList = employMaketList
      } catch (err) {
        console.error(err)
      }
    },
    // 获取游戏信息
    async getContractInfo () {
      try {
        const contractInfo = await window.contract_angleLand.get_contract_info()
        this.contractInfo = contractInfo
        const maketList = []
        for (let i = 0; i < contractInfo.house_count; i++) {
          maketList.push(i + 1)
        }
        this.maketList = maketList
      } catch (err) {
        console.erro(err)
      }
    },
    // 成为土块拥有者
    async play () {
      try {
        if (!this.active) {
          alert('请选择地块')
        } else if (this.gptBalance < 1000000000000000000000000) {
          alert('游戏余额不足')
        } else {
          this.isLoading = true
          const info = await window.contract_platform.play({
            shop_id: 'angleland.testnet',
            amount: '1000000000000000000000000',
            op: this.active.toString()
          }, this.gas)
          const winInfo = JSON.parse(info)
          if (winInfo.reward_amount != '0') {
            this.getContractInfo()
            this.getWinList()
            this.winInfo = `中奖地皮为 ${winInfo.god_choosen} \n 幸运者 ${winInfo.lucky_guy} \n 获得 ${(winInfo.reward_amount / 1e24).toFixed(2)} GPT`
          } else {
            this.winInfo = `中奖地皮为 ${winInfo.god_choosen}, 无人中奖`
          }
          this.$emit('getGptBalance')
          this.getEmployMaketList()
          this.active = ''
          this.isLoading = false
        }
      } catch (err) {
        this.isLoading = false
        this.active = ''
        this.winInfo = ''
        alert('购买失败，可能该地已被占用')
        console.error(err)
      }
    },
    // 取消选地
    cancel () {
      this.active = ''
    },
    // 获取赢家历史列表
    async getWinList () {
      try {
        const winList = await window.contract_angleLand.get_win_history({
          from_index: 0,
          limit: 100
        })
        this.winList = winList
      } catch (err) {
        console.error(err)
      }
    },
    isActive (item) {
      if (!this.getMaketColor(item)) {
        this.active = item
      } else {
        alert('此地已被占用')
      }
    }
  },
  created () {
    this.getWinList()
    this.$nextTick(async () => {
      try {
        this.getEmployMaketList()
        await this.getContractInfo()
        this.changeItemHeight()
        window.addEventListener('resize', this.changeItemHeight)
      } catch (err) {
        console.error(err)
      }
    })
  }
}
</script>

<style lang="less" scoped>
.text-red {
  color: #d53c3b !important;
}
.bg-green {
  background: linear-gradient(to bottom right, #4fe1c9, #1f8993) !important;
}
.bg-yellow {
  background: linear-gradient(to bottom right, #fad156, #e28f24) !important;
}
.active {
  background: url('../assets/img/filed-selected.png') no-repeat center center !important;
  background-size: 100% 100% !important;
}
.lucky-box {
  color: #fff;
  .row {
    margin: 0;
  }
  .lucky-main {
    .left {
      text-align: center;
      line-height: 40px;
      align-self: start;
      border: 1px solid #f8ae1c;
      background-color: rgba(0, 0, 0, 0.2);
      padding: 10px;
      table {
        margin: 0 auto;
        width: 100%;
        color: #fff;
        font-size: 12px;
        text-align: center;
        thead {
          border-bottom: 1px solid #f8ae1c;
          tr {
            td {
              padding-bottom: 15px;
            }
          }
        }
        tbody {
          tr {
            td {
              line-height: 30px;
            }
          }
        }
      }
    }
    .center {
      padding: 0;
      align-self: start;
      .center-wrap {
        display: flex;
        flex-wrap: wrap;
        justify-content: space-between;
        align-content: space-between;
        .maket-item {
          width: 15%;
          height: 10%;
          border-radius: 3px;
          background: linear-gradient(to bottom, #fff5e1, #fee9c2);
          cursor: pointer;
          /* border: 2px solid red; */
        }
      }
    }
    .right-wrap {
      padding: 0;
      .right {
        border: 1px solid #f8ae1c;
        background-color: rgba(0, 0, 0, 0.2);
        padding: 20px;
        position: relative;
        height: 300px;
        display: flex;
        flex-wrap: wrap;
        align-content: space-around;
        .title {
          /* margin-bottom: 40px; */
          font-size: 14px;
        }
        .info {
          color: #d7af4b;
          font-size: 14px;
          p {
            margin: 0;
            line-height: 22px;
          }
        }
        .button-wrap {
          /* position: absolute; */
          width: 100%;
          /* left: 0; */
          /* bottom: 20px; */
          display: flex;
          justify-content: space-around;
          button {
            border: none;
            border-radius: 3px;
            background-color: red;
            width: 100px;
            height: 40px;
          }
          .cancel {
            background-color: #fee8c1;
            font-weight: 300;
          }
          .confirm {
            background-color: #f8ae1c;
          }
        }
      }
      .mobile-right {
        padding: 0;
        .title {
          font-size: 14px;
        }
        .button-wrap {
          padding-top: 30px;
          padding-bottom: 10px;
          width: 270px;
          display: flex;
          margin: 0 auto;
          justify-content: space-around;
          button {
            border: none;
            border-radius: 2px;
            background-color: red;
            width: 110px;
            height: 45px;
            font-size: 20px;
          }
          .cancel {
            background-color: #fee8c1;
            font-weight: 300;
          }
          .confirm {
            background-color: #f8ae1c;
          }
        }
      }
      .win-info {
        color: #f8ae1c;
        font-size: 20px;
        text-align: center;
        line-height: 40px;
        white-space: pre-wrap;
      }
    }
  }
  h2 {
    text-align: center;
    padding: 30px 0 10px;
  }
  .win-table {
    /* max-width: 450px; */
    width: 100%;
    height: 270px;
    overflow: auto;
    padding: 20px;
    border: 1px solid #f8ae1c;
    margin: 0 auto;
    margin-bottom: 100px;
    background-color: rgba(0, 0, 0, 0.2);
    table {
        margin: 0 auto;
        width: 100%;
        color: #fff;
        font-size: 12px;
        text-align: center;
        thead {
          border-bottom: 1px solid #f8ae1c;
          tr {
            td {
              padding-bottom: 15px;
            }
          }
        }
        tbody {
          tr {
            td {
              line-height: 30px;
            }
          }
        }
      }
  }
  .win-table::-webkit-scrollbar {
    /*滚动条整体样式*/
    width: 11px;  /*高宽分别对应横竖滚动条的尺寸*/
    height: 1px;
  }
  .win-table::-webkit-scrollbar-thumb {
    /*滚动条里面小方块*/
    border-radius: 10px;
    /* box-shadow   : inset 0 0 5px rgba(0, 0, 0, 0.2); */
    /* background: #f8ae1c; */
    border: 3px solid rgba(0, 0, 0, 0);
    box-shadow: 8px 0 0 #f8af1cb4 inset;
  }
  .win-table::-webkit-scrollbar-track {
    /*滚动条里面轨道*/
    background: transparent;
  }
}
</style>
