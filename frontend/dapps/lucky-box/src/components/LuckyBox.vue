<template>
  <div class="lucky-box container">
    <div class="lucky-main row">
      <div v-if="!isMobile" class="col-md-3 mt-4 mt-md-5 left">
        <table>
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
        </table>
      </div>
      <div class="col-md-6 center mt-4 mt-md-5 pl-md-4 pr-md-4">
        <div v-for="(item, index) in boxList" :key="index" class="center-wrap">
          <div v-show="item.data && item.data >= 13" class="error-log">Your bet has exceeded the limit</div>
          <div class="box-left">Counter: 56.00 GPT</div>
          <div class="box-right" :class="{active: item.index === isAcitve}" @click="changActive(item.index)">
            <span :class="{'text-red': item.data && item.data >= 13}">{{item.data}}</span>
            <div class="computed">
              <div class="add" @click="add(index)">+</div>
              <div class="subtract" @click="subtract(index)">-</div>
            </div>
          </div>
        </div>
      </div>
      <div class="col-md-3 right mt-4 mt-md-5">
        <div class="title">The total score will be 1 to 12</div>
        <div class="info">
          <p>Simple intoductions:</p>
          <p>Ut placerat pharetra ipsum, in dignissim dui dapibus non. Suspendisse potenti. Phasellus volutpat nisi quis.</p>
        </div>
        <div class="button-wrap">
          <button class="cancel">Cancel</button>
          <button class="confirm">Confirm</button>
        </div>
      </div>
    </div>
    <h2>Recent Winners</h2>
    <div class="win-table container">
      <table align="center">
        <thead>
          <tr>
            <td>Height</td>
            <td>Username</td>
            <td>Prize</td>
          </tr>
        </thead>
        <tbody>
          <tr>
            <td>Height</td>
            <td>Username</td>
            <td>Prize</td>
          </tr>
          <tr>
            <td>Height</td>
            <td>Username</td>
            <td>Prize</td>
          </tr>
          <tr>
            <td>Height</td>
            <td>Username</td>
            <td>Prize</td>
          </tr>
          <tr>
            <td>Height</td>
            <td>Username</td>
            <td>Prize</td>
          </tr>
          <tr>
            <td>Height</td>
            <td>Username</td>
            <td>Prize</td>
          </tr>
          <tr>
            <td>Height</td>
            <td>Username</td>
            <td>Prize</td>
          </tr>
          <tr>
            <td>Height</td>
            <td>Username</td>
            <td>Prize</td>
          </tr>
          <tr>
            <td>Height</td>
            <td>Username</td>
            <td>Prize</td>
          </tr>
          <tr>
            <td>Height</td>
            <td>Username</td>
            <td>Prize</td>
          </tr>
          <tr>
            <td>Height</td>
            <td>Username</td>
            <td>Prize</td>
          </tr>
          <tr>
            <td>Height</td>
            <td>Username</td>
            <td>Prize</td>
          </tr>
          <tr>
            <td>Height</td>
            <td>Username</td>
            <td>Prize</td>
          </tr>
          <tr>
            <td>Height</td>
            <td>Username</td>
            <td>Prize</td>
          </tr>
          <tr>
            <td>Height</td>
            <td>Username</td>
            <td>Prize</td>
          </tr>
          <tr>
            <td>Height</td>
            <td>Username</td>
            <td>Prize</td>
          </tr>
        </tbody>
      </table>
    </div>
  </div>
</template>

<script>
export default {
  data () {
    return {
      boxList: [
        {

          index: 1,
          data: ''
        },
        {
          index: 2,
          data: ''
        },
        {
          index: 3,
          data: ''
        },
        {
          index: 4,
          data: ''
        },
        {
          index: 5,
          data: ''
        },
        {
          index: 6,
          data: ''
        }
      ],
      isAcitve: 1
    }
  },
  props: ['isMobile'],
  methods: {
    changActive (index) {
      this.isAcitve = index
    },
    add (index) {
      const num = this.boxList[index].data ? this.boxList[index].data : 0
      this.$set(this.boxList[index], 'data', num + 1)
    },
    subtract (index) {
      const num = this.boxList[index].data ? this.boxList[index].data : 0
      if (num <= 0) {
        return
      }
      if (num === 1) {
        this.$set(this.boxList[index], 'data', '')
        return
      }
      this.$set(this.boxList[index], 'data', num - 1)
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
    .right {
      border: 1px solid #f8ae1c;
      background-color: rgba(0, 0, 0, 0.3);
      padding: 20px;
      position: relative;
      .title {
        margin-bottom: 40px;
        font-size: 14px;
      }
      .info {
        color: #d7af4b;
        font-size: 12px;
        p {
          margin: 0;
          line-height: 22px;
        }
      }
      .button-wrap {
        position: absolute;
        width: 100%;
        left: 0;
        bottom: 20px;
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
  }
  h2 {
    text-align: center;
    padding: 30px 0 10px;
  }
  .win-table {
    max-width: 450px;
    width: 90%;
    height: 270px;
    overflow: auto;
    padding: 20px;
    border: 1px solid #f8ae1c;
    margin-bottom: 100px;
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
