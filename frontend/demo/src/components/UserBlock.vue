<template>
  <div class="user">
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
      <button>
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
    >
      <template #default="{ hide }">
        <div class="sidebar-header">
          <button @click="hide">
            <img src="../assets/img/icon-close.png" alt="">
          </button>
        </div>
        <div class="sidebar-body">
          <b-tabs content-class="mt-3">
            <b-tab title="我想卖出" active>
              <div class="tab-title">
                
              </div>
            </b-tab>
            <b-tab title="我想买入"><p>I'm the second tab</p></b-tab>
          </b-tabs>        
        </div>
      </template>
    </b-sidebar>
  </div>
</template>

<script>
export default {
  created () {
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
  },
  methods: {
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
    async buyPlayToken (num) {
      await window.contract_platform.buy_playtoken(
        {},
        this.gas,
        this.toNear(num)
      )
    },
    async sellPlayToken (num) {
      await window.contract_platform.sell_playtoken(
        { amount: this.toNear(num) },
        this.gas
      )
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
    buyHandleOk(bvModalEvt) {
      // Prevent modal from closing
      bvModalEvt.preventDefault()
      // Trigger submit handler
      this.buyHandleSubmit()
    },
    sellHandleOk (bvModalEvt) {
      bvModalEvt.preventDefault()
      // Trigger submit handler
      this.sellHandleSubmit()
    },
    buyHandleSubmit () {
      // Exit when the form isn't valid
      if (!this.checkFormValidity('buyForm')) {
        return
      }
      // Push the name to submitted names
      this.buyPlayToken(this.buyNear)
      // Hide the modal manually
      this.$nextTick(() => {
        this.$bvModal.hide('modal-buy')
      })
    },
    async sellHandleSubmit () {
      if (!this.checkFormValidity('sellForm')) {
        return
      }
      // Push the name to submitted names
      await this.sellPlayToken(this.sellGpt)
      // Hide the modal manually
      this.$nextTick(() => {
        this.$bvModal.hide('modal-sell')
        this.$parent.$parent.updateUser()
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
  border: none;
}
.user .b-sidebar .sidebar-body .nav-tabs .nav-item .nav-link{
  border: none;
  background-color: transparent;
  color: #f6c94a;
  font-weight: 200;
}
</style>
