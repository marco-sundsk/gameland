<template>
  <div class="user">
    <div class="user-name">
      欢迎 {{currentUser.accountId}}
    </div>
    <div class="user-surplus" v-b-modal.modal-buy>
      余额 {{currentUser.balance | nearToNum}}
    </div>
    <div class="game-surplus" v-b-modal.modal-sell>
      游戏余额 {{gptBalance | nearToNum}}
    </div>

    <!-- 购买模态框 -->
    <b-modal
      id="modal-buy"
      ref="modal"
      title="购买GPT 当前GPT买价"
      @show="buyResetModal"
      @hidden="buyResetModal"
      @ok="buyHandleOk"
    >
      <form ref="buyForm" @submit.stop.prevent="handleSubmit">
        <b-form-group
          label="NEAR"
          label-for="buy-near-input"
          invalid-feedback="NEAR is required"
          :state="buyState"
        >
          <b-form-input
            id="buy-near-input"
            v-model="buyNear"
            :state="buyState"
            max="3"
            @input="buyNearInp"
            required
          ></b-form-input>
        </b-form-group>

        <b-form-group
          label="GPT"
          label-for="buy-gpt-input"
          invalid-feedback="GPT is required"
          :state="buyState"
        >
          <b-form-input
            id="buy-gpt-inpu"
            v-model="buyGpt"
            :state="buyState"
            max="3"
            @input="buyGptInp"
            required
          ></b-form-input>
        </b-form-group>
      </form>
    </b-modal>

    <b-modal
      id="modal-sell"
      ref="modal"
      title="回收GPT 当前GPT回收价"
      @show="sellResetModal"
      @hidden="sellResetModal"
      @ok="sellHandleOk"
    >
      <form ref="sellForm" @submit.stop.prevent="handleSubmit">
        <b-form-group
          label="GPT"
          label-for="sell-gpt-input"
          invalid-feedback="GPT is required"
          :state="sellState"
        >
          <b-form-input
            id="sell-gpt-inpu"
            v-model="sellGpt"
            :state="sellState"
            max="3"
            @input="sellGptInp"
            required
          ></b-form-input>
        </b-form-group>
        <b-form-group
          label="NEAR"
          label-for="sell-near-input"
          invalid-feedback="NEAR is required"
          :state="sellState"
        >
          <b-form-input
            id="sell-near-input"
            v-model="sellNear"
            :state="sellState"
            max="3"
            @input="sellNearInp"
            required
          ></b-form-input>
        </b-form-group>

      </form>
    </b-modal>
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