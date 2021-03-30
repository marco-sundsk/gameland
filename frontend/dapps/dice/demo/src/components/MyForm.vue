<template>
  <div class="my-form">
    <p>
      {{title}}
    </p>
    <!-- type类型判断显示合适内容 -->
    <!-- big类型 -->
    <b-form-radio-group class="row" v-if="type == 'Big'">
      <label class="col-3">Bet for:</label>
      <div class="col-1"></div>
      <b-form-radio class="col-3 mr-0" v-model="selected" name="some-radios" value="1">Big</b-form-radio>
      <b-form-radio class="col-3 mr-0" v-model="selected" name="some-radios" value="2">Small</b-form-radio>
    </b-form-radio-group>

    <!-- odd类型 -->
    <b-form-radio-group class="row" v-if="type === 'Odd'">
      <label class="col-3">Bet for:</label>
      <div class="col-1"></div>
      <b-form-radio class="col-3 mr-0" v-model="selected" name="some-radios" value="1">Odd</b-form-radio>
      <b-form-radio class="col-3 mr-0" v-model="selected" name="some-radios" value="2">Even</b-form-radio>
    </b-form-radio-group>

    <!-- Specific类型 -->
    <b-form-radio-group class="row" v-if="type === 'Specific'">
      <label class="col-3">Bet for:</label>
      <div class="col-1"></div>
      <b-form-select class="col-6" v-model="selected" :options="specificOptions"></b-form-select>
    </b-form-radio-group>

    <!-- Any 类型 -->
    <b-form-radio-group class="row" v-if="type === 'Any'">
      
    </b-form-radio-group>

    <!-- Dice 类型 -->
    <b-form-radio-group class="row" v-if="type === 'Dice'">
      <label class="col-3">Bet for:</label>
      <div class="col-1"></div>
      <b-form-select class="col-7" v-model="dice1Selected" :options="dice1Options"></b-form-select>
      <div class="col-3"></div>
      <div class="col-1"></div>
      <b-form-select class="col-7" v-model="dice2Selected" :options="dice2Options"></b-form-select>
    </b-form-radio-group>

    <!-- SpecificDbl 类型 -->
    <b-form-radio-group class="row" v-if="type === 'SpecificDbl'">
      <label class="col-3">Bet for:</label>
      <div class="col-1"></div>
      <b-form-select class="col-7" v-model="selected" :options="specificDblOptions"></b-form-select>
    </b-form-radio-group>

    <!-- 输入框 -->
    <div class="row mt-2">
      <label for="inp" class="col-3 mt-1">amount:</label>
      <div class="col-1"></div>
      <b-form-input id="inp" class="col-7" v-model="amount"></b-form-input>
    </div>
    <div class="row mt-4">
      <b-button class="col-6" type="reset" variant="danger">Cancel</b-button>
      <b-button class="col-6" variant="primary" @click="rollDice">Play</b-button>
    </div>
  </div>
</template>

<script>
export default {
  data () {
    return {
      selected: '',
      dice1Selected: '',
      dice2Selected: '',
      amount: '',
      specificOptions: [
        { value: 1, text: 1 },
        { value: 2, text: 2 },
        { value: 3, text: 3 },
        { value: 4, text: 4 },
        { value: 5, text: 5 },
        { value: 6, text: 6 }
      ],
      dice1Options: [
        { value: 1, text: 1 },
        { value: 2, text: 2 },
        { value: 3, text: 3 },
        { value: 4, text: 4 },
        { value: 5, text: 5 },
        { value: 6, text: 6 }
      ],
      dice2Options: [
        { value: 1, text: 1 },
        { value: 2, text: 2 },
        { value: 3, text: 3 },
        { value: 4, text: 4 },
        { value: 5, text: 5 },
        { value: 6, text: 6 }
      ],
      specificDblOptions: [
        { value: 1, text: 1 },
        { value: 2, text: 2 },
        { value: 3, text: 3 },
        { value: 4, text: 4 },
        { value: 5, text: 5 },
        { value: 6, text: 6 }
      ]
    }
  },
  methods: {
    rollDice () {
      if (this.type == 'Dice') {
        this.$emit('rollDice', this.category, this.toNear(this.amount), this.dice1Selected, this.dice2Selected)
        // this.dice1Selected = ''
        // this.dice2Selected = ''
        // this.amount = ''
      } else if (this.type == 'Any') {
        this.$emit('rollDice', this.category, this.toNear(this.amount))
        // this.amount = ''
      } else {
        this.$emit('rollDice', this.category, this.toNear(this.amount), this.selected)
        // this.selected = ''
        // this.amount = ''
      }
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
    }
  },
  props: ['title', 'type', 'category'],
  watch: {
    amount (value) {
      if (value) {
        var newValue = value.toString()
        newValue = newValue.replace(/[^\d.]/g, '') // 清除“数字”和“.”以外的字符
        newValue = newValue.replace(/\.{2,}/g, '.') // 只保留第一个. 清除多余的
        newValue = newValue.replace('.', '$#$').replace(/\./g, '').replace('$#$', '.')
        if (newValue[0] && newValue[0] === '.') {
          newValue = '0' + newValue
        }
        this.$nextTick(() => {
          this.amount = newValue
        })
      }
    }
  }
}
</script>
