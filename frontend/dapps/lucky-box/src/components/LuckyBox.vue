<template>
  <div class="lucky-box container">
    <div class="vld-parent">
      <loading :active.sync="isLoading" :is-full-page="true">
      </loading>
    </div>
    <div class="lucky-main row">
      <div v-if="!isMobile" class="col-md-3 mt-4 mt-md-5 left">
        <p class="game-info-title">当前游戏信息</p>
        <p>盒子数：{{contract.box_count}}</p>
        <p>当前轮次：{{contract.current_round}}</p>
        <p>当前轮次开始时间：{{contract.round_start_ts | changeTime}}</p>
        <p>轮次持续的时长：{{contract.round_period}} 秒</p>
        <p>当前领先的盒子：{{contract.cur_win_box}}</p>
      </div>
      <div class="col-md-6 center mt-4 mt-md-5 pl-md-4 pr-md-4">
        <div v-for="(item, index) in boxList" :key="index" class="center-wrap">
          <div v-show="item.data && item.data >= 13" :class="errorLogStyle">Your bet has exceeded the limit</div>
          <div class="box-left">
            <div class="text-progress">Counter : {{item.total_amount | nearToNum}} GPT</div>
            <div v-if="item.total_amount != '0'" class="my-progress" :style="getProgressWidth(item.total_amount)">
              <div class="bg-progress"></div>
              <svg class="waves" xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" viewBox="0 24 150 28" preserveAspectRatio="none" shape-rendering="auto">
                <defs>
                  <path id="gentle-wave" d="M-160 44c30 0 58-18 88-18s 58 18 88 18 58-18 88-18 58 18 88 18 v44h-352z" />
                </defs>
                <g class="parallax">
                  <use xlink:href="#gentle-wave" x="48" y="0" fill="#567fa1" />
                  <use xlink:href="#gentle-wave" x="48" y="7" fill="rgba(48,164,206)" />
                </g>
              </svg>
            </div>
          </div>
          <div class="box-right" :class="{active: item.box_id === isAcitve}" @click="changActive(item.box_id)">
            <span :class="{'text-red': item.data && item.data >= 13}">{{item.data}}</span>
            <div class="computed">
              <div class="add" @click="add(item.box_id)">+</div>
              <div class="subtract" @click="subtract(item.box_id)">-</div>
            </div>
          </div>
        </div>
      </div>
      <div class="col-md-3 mt-4 mt-md-5 right-wrap">
        <div :class="rightStyle">
          <div class="title">The total score will be 1 to 12</div>
          <div v-if="!isMobile" class="info">
            <p>Simple intoductions:</p>
            <p>Ut placerat pharetra ipsum, in dignissim dui dapibus non. Suspendisse potenti. Phasellus volutpat nisi quis.</p>
          </div>
          <div class="button-wrap">
            <button class="cancel" @click="cancel">Cancel</button>
            <button class="confirm" @click="play">Confirm</button>
          </div>
        </div>
        <p v-if="playInfo" class="playInfo" >已投盒子：{{playInfo.box_id}}</p>
        <div class="settle">
          <button v-if="getGameState(contract.round_start_ts)" @click="settle">Settle</button>
        </div>
      </div>
    </div>
    <div class="row">
      <div class="col-md-3"></div>
      <div class="col-md-6">
        <h2>Recent Winners</h2>
      </div>
      <div class="col-md-3"></div>
    </div>
    <div class="row">
      <div class="col-md-3"></div>
      <div class="col-md-6">
        <div class="win-table container">
          <table align="center">
            <thead>
              <tr>
                <td>轮次</td>
                <td>时间</td>
                <td>奖励金额</td>
                <td>奖励人数</td>
              </tr>
            </thead>
            <tbody>
              <tr v-for="(item, index) in winList" :key="index">
                <td>{{item.round}}</td>
                <td>{{item.ts | changeTime}}</td>
                <td style="color: #f8ae1c;">{{item.total_reward | nearToNum}}</td>
                <td>
                  <b-dropdown
                    split
                    :text="getObjLength(item.winners)"
                    class="m-2"
                  >
                    <b-dropdown-item v-for="(value, key) in item.winners" :key="key" href="#">
                      {{key}}
                    </b-dropdown-item>
                  </b-dropdown>
                </td>
              </tr>
            </tbody>
          </table>
        </div>
      </div>
      <div class="col-md-3"></div>
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
      boxList: [ ],
      isAcitve: 1,
      gas: Math.pow(10, 14).toString(),
      boxCount: {},
      totalAmount: 0,
      currentBox: [],
      isLoading: false,
      winList: [],
      contract: '',
      playInfo: '',
      interval: ''
    }
  },
  computed: {
    rightStyle () {
      return this.isMobile ? 'mobile-right' : 'right'
    },
    errorLogStyle () {
      return this.isMobile ? 'error-log-mobile' : 'error-log'
    },
    accountId () {
      return window.accountId
    },
    getProgressWidth () {
      return (num) => {
        return {
          width: (num / this.totalAmount * 100).toFixed() + '%'
        }
      }
    },
    getObjLength () {
      return (obj) => {
        // if (obj == null || obj == undefined) {
        //   return
        // }
        // if (typeof(obj) != Object) {
        //   return
        // }
        return (Object.keys(obj).length).toString()
      }
    },
    // 获取当前游戏时间
    getGameState () {
      return (time) => {
        if (time == '0') return false
        if (time) {
          const endTime = Number(time) / 1e6 + 3600000
          const currentTime = new Date().getTime()
          if (endTime <= currentTime) {
            return true
          } else {
            return false
          }
        } else {
          return false
        }
      }
    }
  },
  props: ['isMobile', 'currentUser', 'gptBalance'],
  methods: {
    // 游戏结束
    async settle () {
      try {
        await this.getContract()
        if (this.getGameState(this.contract.round_start_ts)) {
          this.isLoading = true
          const settle = await window.contract_platform.settle({
            shop_id: 'luckybox.testnet',
            op: 'settle'
          }, this.gas)
          console.log(settle)
          await this.getContract()
          await this.getBoxesInfo()
          this.isLoading = false
        } else {
          alert('本轮未结束')
        }
      } catch (err) {
        console.error(err)
      }
    },
    changActive (index) {
      this.isAcitve = index
    },
    // 加
    add (index) {
      const num = this.boxList[index - 1].data ? this.boxList[index - 1].data : 0
      this.$set(this.boxList[index - 1], 'data', num + 1)
      const idIndex = this.currentBox.findIndex(item => item.id == index)
      if (idIndex == -1) {
        this.currentBox.push({
          id: index,
          data: num + 1
        })
      } else {
        this.$set(this.currentBox, idIndex, {
          id: index,
          data: num + 1
        })
      }
      // this.currentBox.id = index
      // this.currentBox.data = num + 1
    },
    // 减
    subtract (index) {
      const num = this.boxList[index - 1].data ? this.boxList[index - 1].data : 0
      const idIndex = this.currentBox.findIndex(item => item.id == index)
      if (num <= 0) {
        return
      }
      if (num === 1) {
        this.currentBox.splice(idIndex, 1)
        this.$set(this.boxList[index - 1], 'data', '')
        return
      }
      this.$set(this.boxList[index - 1], 'data', num - 1)
      if (idIndex == -1) {
        this.currentBox.push({
          id: index,
          data: num + 1
        })
      } else {
        this.$set(this.currentBox, idIndex, {
          id: index,
          data: num + -1
        })
      }
      // this.currentBox.id = index
      // this.currentBox.data = num + 1
    },
    // 获取当前游戏信息
    async getContract () {
      try {
        const contract = await window.contract_game.get_contract_info()
        this.contract = contract
        this.boxCount = contract.box_count
      } catch (err) {
        console.error(err)
      }
    },
    // 获取box信息
    async getBoxesInfo () {
      const boxesInfo = await window.contract_game.get_boxes_info()
      const boxLit = []
        let totalAmount = 0
        for (let i = 0; i < this.boxCount; i ++) {
          const result = {
            'box_id': i + 1,
            'total_amount': 0
          }
          let box = ''
          if (boxesInfo[i + 1]) {
            box = JSON.parse(JSON.stringify(boxesInfo[i + 1]))
          } else {
            box = result
          }
          box.data = ''
          totalAmount += Number(box.total_amount)
          boxLit.push(box)
        }
        this.totalAmount = totalAmount
        this.boxList = boxLit
    },
    // 将数字转换为near
    toNear (num) {
      num = num.toString()
      const ratio = '000000000000000000000000'
      if (num === '0') return '0'
      if (num.indexOf('.') !== -1) {
        const arr = num.split('.')
        if (arr[0] !== 0) {
          return arr[0] + arr[1] + ratio.slice(0, (ratio.length - arr[1].length))
        } else {
          return arr[1] + ratio.slice(0, (ratio.length - arr[1].length))
        }
      } else {
        return num + ratio
      }
    },
    // 开始游戏
    async play () {
      try {
        if (this.contract.round_start_ts != '0' && this.getGameState(this.contract.round_start_ts)) {
          alert('本轮结束')
          return
        }
        if (this.currentBox.length > 0) {
          const resultArr = this.currentBox.filter(item => item.data >= 13)
          if (resultArr.length > 0) {
            alert(`单个盒子投币数不得大于13`)
            return
          }
          this.isLoading = true
          const playInfo = await window.contract_platform.play({
            "shop_id": 'luckybox.testnet',
            "amount": `${this.toNear(this.currentBox[0].data)}`,
            "op": `${this.currentBox[0].id}`
          }, this.gas)
          console.log(playInfo)
          this.playInfo = JSON.parse(playInfo)
          await this.getContract()
          await this.getBoxesInfo()
          this.isLoading = false
        } else {
          alert('请投币')
        }
      } catch (err) {
        console.error(err)
      }
    },
    // 获取赢家历史
    async getWinHistory () {
      try {
        const winList = await window.contract_game.get_win_history({
          'from_index': 0,
          limit: 999
        })
        this.winList = winList
      } catch (err) {
        console.error(err)
      }
    },
    cancel () {
      this.isAcitve = 1
      this.currentBox = []
      for (let i = 0; i < this.boxList.length; i++) {
        this.$set(this.boxList[i], 'data', '')
      }
    }
  },
  async created () {
    this.getWinHistory()
    await this.getContract()
    await this.getBoxesInfo()
    this.interval = setInterval(this.getContract, 5000)
  },
  destroyed () {
    if (this.interval) {
      clearInterval(this.interval)
    }
  }
}
</script>

<style lang="less" scoped>
.text-red {
  color: #d53c3b !important;
}
.lucky-box {
  color: #fff;
  .row {
    margin: 0;
  }
  .lucky-main {
    .left {
      align-self: start;
      border: 1px solid #f8ae1c;
      text-align: center;
      background-color: rgba(0, 0, 0, 0.3);
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
        width: 100%;
        justify-content: space-between;
        margin-bottom: 15px;
        position: relative;
        .error-log {
          height: 50px;
          line-height: 50px;
          padding: 0 20px;
          background-color: #f8af1cdc;
          border-radius: 3px;
          color: #d53c3b;
          position: absolute;
          right: 0;
          transform: translateX(105%);
          z-index: 999;
        }
        .error-log-mobile {
          height: 50px;
          line-height: 50px;
          padding: 0 20px;
          background-color: #f8af1cdc;
          border-radius: 3px;
          color: #d53c3b;
          position: absolute;
          right: 95px;
          z-index: 999;
        }
        .box-left {
          max-width: 427px;
          flex: 1;
          width: 80%;
          border: 1px solid #f8af1cad;
          height: 50px;
          border-radius: 3px;
          background-color: #f8af1c4b;
          line-height: 50px;
          color: #000;
          font-size: 16px;
          text-indent: 2rem;
          font-weight: 300;
          position: relative;
        }
        .box-right {
          cursor: pointer;
          width: 80px;
          height: 50px;
          border: 1px solid #f8ae1c;
          margin-left: 15px;
          border-radius: 3px;
          background-color: #f8af1ca9;
          text-align: center;
          line-height: 50px;
          font-size: 22px;
          color: #000;
          display: flex;
          span {
            flex: 1;
          }
          .computed {
            width: 30px;
            height: 100%;
            .subtract,
            .add {
              height: 50%;
              line-height: 50%;
              display: flex;
              justify-content: center;
              align-items: center;
              background-color: #f8ae1c;
              cursor: pointer;
            }
            .add {
              border-bottom: 1px solid #000;
            }
          }
        }
        .box-right:not(.active) {
          width: 50px;
          height: 50px;
          border: 1px solid #f8ae1c;
          margin-left: 15px;
          margin-right: 30px;
          border-radius: 3px;
          background-color: #f8af1ca9;
          text-align: center;
          line-height: 50px;
          font-size: 22px;
          color: #000;
          .computed {
            display: none;
          }
        }
      }
      .center-wrap:last-of-type {
        margin: 0;
      }
    }
    .right-wrap {
      padding: 0;
      .playInfo {
        margin: 0;
        line-height: 40px;
        color: #f8ae1c;
      }
      .settle {
        width: 100%;
        display: flex;
        justify-content: center;
        align-items: center;
        padding-top: 20px;
        button {
            border: none;
            border-radius: 3px;
            background-color: #f8ae1c;
            width: 100px;
            height: 40px;
        }
      }
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
  .text-progress {
    position: absolute;
    z-index: 999;
  }
  .my-progress {
    min-width: 48px;
    display: flex;
    height: 48px;
    width: 0%;
    align-items: center;
    justify-content: flex-start;
    .bg-progress {
      background-color: rgba(48,164,206);
      flex: 1;
      width: 100%;
      height: 100%;
      /* border: 1px solid rgba(48,164,206); */
    }
    .waves {
      width: 48px;
      height: 48px;
      transform: rotate(90deg);
      flex-shrink: 0;
    }
  }
  .parallax > use {
    animation: move-forever 25s cubic-bezier(.55,.5,.45,.5)     infinite;
  }
  .parallax > use:nth-child(1) {
    animation-delay: -2s;
    animation-duration: 7s;
  }
  .parallax > use:nth-child(2) {
    animation-delay: -3s;
    animation-duration: 10s;
  }
  .parallax > use:nth-child(3) {
    animation-delay: -4s;
    animation-duration: 13s;
  }
  .parallax > use:nth-child(4) {
    animation-delay: -5s;
    animation-duration: 20s;
  }
  @keyframes move-forever {
    0% {
    transform: translate3d(-90px,0,0);
    }
    100% { 
      transform: translate3d(85px,0,0);
    }
  }
  /*Shrinking for mobile*/
  @media (max-width: 768px) {
    .waves {
      height:40px;
      min-height:40px;
    }
    .content {
      height:30vh;
    }
    h1 {
      font-size:24px;
    }
  }
}
</style>

<style lang="less">
.lucky-box {
  .dropdown {
    color: #fff !important;
    .btn-secondary  {
      color: #fff;
      font-size: 12px;
      border: none !important;
      background-color: transparent !important;
      cursor: default !important;
    }
    .btn-secondary {
      border: none !important;
      box-shadow: none !important;
    }
    .dropdown-toggle {
      cursor: pointer !important;
    }
  }
}
</style>
