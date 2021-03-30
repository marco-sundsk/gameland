<template>
  <div class="user">
    <div class="vld-parent">
      <loading :active.sync="isLoading" :can-cancel="true" :is-full-page="true">
        <template #overlay>
        </template>
      </loading>
    </div>
    <div class="user-info">
      <div class="user-header" v-b-toggle.sidebar-user>
        <img src="../assets/img/icon-user.png" alt="">
      </div>
      <div class="user-name">{{currentUser.accountId}}</div>
    </div>
    <div class="config">
      <button>
        <img src="../assets/img/icon-user-settins.png" alt="">
      </button>
    </div>
    <div class="logout">
      <button @click="logout">
        <img src="../assets//img/icon-logout.png" alt="">
      </button>
    </div>

    <b-sidebar
      id="sidebar-user"
      title="Sidebar with backdrop"
      backdrop-variant="transparent"
      no-header
      backdrop
      shadow
      right
      :visible="isBuySidebar"
    >
      <template #default="{ hide }">
        <div class="sidebar-header">
          <button @click="hide">
            <img src="../assets/img/icon-close.png" alt="">
          </button>
        </div>
        <div class="sidebar-body">
          <div style="line-height: 40px;">您的NEAR余额为：{{currentUser.balance | nearToNum}}</div>
          <div style="line-height: 40px;">您的游戏余额为：{{gptBalance | nearToNum}} GPT</div>
          <b-tabs content-class="mt-3">
            <b-tab v-if="!tabShow" @click="showTab" title="我想卖出》" active>
              <div class="tab-title">
                买入Gpt
              </div>
              <div class="tab-info">
                <div class="left">当前GPT买价：{{getMintPrice}} NEAR</div>
              </div>
              <form ref="buyForm">
                <b-form-group
                  label-for="buy"
                  invalid-feedback="NEAR is required"
                  :state="buyState"
                >
                  <b-form-input id="buy" placeholder="请输入NEAR币金额" v-model="buyNear" required @input="buyNearInp"></b-form-input>
                </b-form-group>

              </form>
              <!-- <b-form-input type="text" placeholder="请输入NEAR币金额" v-model="buyNear"  @input="buyNearInp"> -->
              <div class="convert">可兑换的GPT数额：<span>{{buyGpt}}</span></div>
              <div class="button-wrap">
                <button @click="buyHandleSubmit">确认购买</button>
                <button @click="hide">取消购买</button>
              </div>
            </b-tab>

            <b-tab v-else @click="showTab" title="我想买入》">
              <div class="tab-title">
                卖出Gpt
              </div>
              <div class="tab-info">
                <div class="left">当前GPT卖价：{{getMintPrice}} NEAR</div>
                <!-- <div class="right">您的游戏余额为：{{gptBalance | nearToNum}} GPT</div> -->
              </div>
              <form ref="sellForm">
                <b-form-group
                  label-for="sell"
                  invalid-feedback="GPT is required"
                  :state="sellState"
                >
                  <b-form-input id="sell" placeholder="请输入游戏币金额" v-model="sellGpt" @input="sellGptInp" required></b-form-input>
  
                </b-form-group>

              </form>
              <!-- <b-form-input type="text" placeholder="请输入游戏币金额" v-model="sellGpt" @input="sellGptInp"> -->
              <div class="convert">可兑换的NEAR数额：<span>{{sellNear}}</span></div>
              <div class="button-wrap">
                <button @click="sellHandleSubmit">确认卖出</button>
                <button @click="hide">取消卖出</button>
              </div>
            </b-tab>
          </b-tabs>        
        </div>
      </template>
    </b-sidebar>
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
  created () {
    if (window.localStorage.getItem('sidebarFlag')) {
      window.localStorage.removeItem('sidebarFlag')
      this.isBuySidebar = true
    }
    this.getGptBalance()
  },
  props: {
    contractInfo: {
      require: true
    },
    currentUser: {
      require: true
    }
  },
  data () {
    return {
      isBuySidebar: false,
      isLoading: false,
      tabShow: false,
      gas: Math.pow(10, 14).toString(),
      buyState: null,
      submittedNames: [],
      buyGpt: '',
      buyNear: '',
      sellState: null,
      sellGpt: '',
      sellNear: '',
      gptBalance: '',
      buyGptFlag: '',
      sellGptFlag: '',
      buyNearFlag: '',
      sellNearFlag: ''
    }
  },
  computed: {
    getMintPrice () {
      return (this.contractInfo.mint_price / 1000)
    }
  },
  methods: {
    logout () {
      this.$emit('logout')
    },
    updateUser () {
      this.$emit('updateUser')
    },
    showTab () {
      this.tabShow = !this.tabShow
    },
    toNear (num) {
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
    async buyPlayToken () {
      try {
        this.isLoading = true
        window.localStorage.setItem('sidebarFlag', true)
        await window.contract_platform.buy_playtoken(
          {},
          this.gas,
          this.toNear(this.buyNear)
        )
        this.isLoading = false
      } catch (err) {
        this.isLoading = false
        console.error(err)
      }
    },
    async sellPlayToken () {
      try {
        this.isLoading = true
        await window.contract_platform.sell_playtoken(
          { amount: this.toNear(this.sellGpt) },
          this.gas
        )
        this.updateUser()
        this.getGptBalance()
        this.isLoading = false
      } catch (err) {
        this.isLoading = false
        console.error(err)
      }
    },
    buyNearInp (value) {
      if (value) {
        this.buyGpt = (value / (this.contractInfo.mint_price / 1000)).toFixed(4)
      } else {
        this.buyGpt = ''
      }
    },
    buyGptInp (value) {
      if (value) {
        this.buyNear = (value * (this.contractInfo.mint_price / 1000)).toFixed(4)
      } else {
        this.buyNear = ''
      }
    },
    sellNearInp (value) {
      if (value) {
        this.sellGpt = (value / (this.contractInfo.mint_price / 1000)).toFixed(4)
      } else {
        this.sellGpt = ''
      }
    },
    sellGptInp (value) {
      if (value) {
        this.sellNear = (value * (this.contractInfo.mint_price / 1000)).toFixed(4)
      } else {
        this.sellNear = ''
      }
    },
    async getGptBalance () {
      const gptBalance = await window.contract_gamecoin.ft_balance_of({
        account_id: this.currentUser.accountId
      })
      this.gptBalance = gptBalance
    },
    checkFormValidity (form) {
      if (form === 'buyForm') {
        const valid = this.$refs[form].checkValidity()
        this.buyState = valid
        return valid
      } else {
        const valid = this.$refs[form].checkValidity()
        this.sellState = valid
        return valid
      }
    },
    buyResetModal () {
      this.buyGpt = '',
      this.buyNear = ''
      this.buyState = null
    },
    sellResetModal () {
      this.sellGpt = ''
      this.sellNear = ''
      this.sellState = null  
    },
    buyHandleSubmit () {
      // Exit when the form isn't valid
      if (!this.checkFormValidity('buyForm')) {
        return
      }
      // Push the name to submitted names
      this.buyPlayToken(this.buyNear)
      // Hide the modal manually
    },
    async sellHandleSubmit () {
      if (!this.checkFormValidity('sellForm')) {
        return
      }
      // Push the name to submitted names
      await this.sellPlayToken(this.sellGpt)
      // Hide the modal manually
      this.$nextTick(() => {
        this.updateUser()
        this.getGptBalance()
      })
    },
    nearRule (value) {
      if (value) {
        var newValue = value.toString()
        newValue = newValue.replace(/[^\d.]/g, '') // 清除“数字”和“.”以外的字符
        newValue = newValue.replace(/\.{2,}/g, '.') // 只保留第一个. 清除多余的
        newValue = newValue.replace('.', '$#$').replace(/\./g, '').replace('$#$', '.')
        if (newValue[0] && newValue[0] === '.') {
          newValue = '0' + newValue
        }
        return newValue
      }
    }
  },
  watch: {
    buyGpt (value) {
      const newValue = this.nearRule(value)
      this.$nextTick(() => {
        this.buyGpt = newValue
      })
    },
    buyNear (value) {
      const newValue = this.nearRule(value)
      this.$nextTick(() => {
        this.buyNear = newValue
      })
    },
    sellGpt (value) {
      const newValue = this.nearRule(value)
      this.$nextTick(() => {
        this.sellGpt = newValue
      })
    },
    sellNear (value) {
      const newValue= this.nearRule(value)
      this.$nextTick(() => {
        this.sellNear = newValue
      })
    }
  }
}
</script>

<style>
.user {
  display: flex;
  width: 200px;
  /* width: 100%; */
  justify-content: space-around;
  align-items: center;
  color: #f8ae1c;
}
.user .user-info {
  display: flex;
  flex-wrap: wrap;
  justify-content: center;
  position: relative;
}
.user .user-info .user-header {
  width: 40px;
  border-radius: 50%;
  overflow: hidden;
  background-color: red;
  display: flex;
  justify-content: center;
  align-items: center;
  margin: 0 30px;
}
.user .user-info .user-header img {
  width: 100%;
}

.user .user-info .user-name {
  position: absolute;
  bottom: -20px;
  left: 50%;
  transform: translateX(-50%);
  font-size: 15px;
}

.user .config {
  margin-right: 30px;
}

.user .logout button,
.user .config button {
  display: flex;
  justify-content: center;
  align-items: center;
  width: 40px;
  height: 40px;
  border: none;
  background-color: transparent;
}
.user .b-sidebar {
  border-radius: 0px !important;
  background-color: #2f2f2f !important;
  color: #f8ae1c !important;
  text-align: left !important;
  width: 100%;
  max-width: 500px !important;
}
.user .b-sidebar .sidebar-header {
  display: flex;
  width: 100%;
  justify-content: flex-end;
  padding-top: 20px;
  padding-right: 20px;
}
.user .b-sidebar .sidebar-header button {
  background-color: transparent;
  border: none;
}
.user .b-sidebar .sidebar-body {
  position: absolute;
  left: 50%;
  top: 50%;
  transform: translate(-50%, -50%);
  width: 80%;
  /* margin: 0 auto; */
}
.user .b-sidebar .sidebar-body .nav-tabs {
  display: block;
  border: none;
}
.user .b-sidebar .sidebar-body .nav-tabs .nav-item {
  padding: 0;
  float: right;
}
.user .b-sidebar .sidebar-body .nav-tabs .nav-item .nav-link{
  padding: 0;
  border: none;
  background-color: transparent;
  color: #f6c94a;
  font-weight: 200;
}
.user .b-sidebar .sidebar-body .tab-title {
  font-size: 20px;
  padding-bottom: 30px;
  border-bottom: 1px solid #f8ae1c;
}
.user .b-sidebar .sidebar-body .tab-info {
  display: flex;
  height: 30px;
  align-items: center;
}
.user .b-sidebar .sidebar-body .tab-info .left {
  font-weight: 200;
  font-size: 12px;
  padding-right: 10px;
}
.user .b-sidebar .sidebar-body .tab-info .right {
  font-size: 12px;
}
.user .b-sidebar .sidebar-body input {
  margin-top: 30px;
  width: 100%;
  height: 60px;
  border: #f8ae1c 1px solid;
  background-color: #f7c9493b;
  padding: 20px;
  color: #f8ae1c;
  font-weight: 300;
}
.user .b-sidebar .sidebar-body input::-webkit-input-placeholder {
  color: #f8ae1c !important;
}
.user .b-sidebar .sidebar-body .convert {
  font-weight: 300;
  line-height: 50px;
  font-size: 16px;
}
.user .b-sidebar .sidebar-body .convert span {
  font-weight: 500;
}

.user .b-sidebar .sidebar-body .button-wrap {
  padding-top: 50px;
  display: flex;
  width: 75%;
  justify-content: space-around;
  margin: 0 auto;
}

.user .b-sidebar .sidebar-body .button-wrap button {
  background-color: #f8ae1c;
  color: #000;
  border: none;
  width: 120px;
  height: 40px;
  border-radius: 3px;
  font-weight: 500;
  font-size: 18px;
}
.user .b-sidebar .sidebar-body .button-wrap button:nth-of-type(2) {
  background-color: #fee9c1;
  font-weight: 300;
}
</style>
