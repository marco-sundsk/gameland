<template>
  <div class="plat">
    <div class="logo-info">
      <img :src="metadata.logo_url" alt="" width="20px" height="20px">
      <div class="logo-right">
        <div class="logo-name">{{metadata.name}}</div>
        <div class="version">v{{metadata.version}}</div>
      </div>
    </div>
    <div class="descripton">{{metadata.description}}</div>
    <div class="btn-wrap">
      <button v-b-modal.modal-1>平台状态</button>
      <button v-b-modal.modal-2>平台政策</button>
    </div>

    <b-modal id="modal-1" title="平台状态" hide-footer>
      <p>用户人数: <span>{{contractInfo.account_num}}</span></p>
      <p>入驻游戏: <span>{{contractInfo.shop_num}}</span></p>
      <p>游戏币投放: <span>{{contractInfo.total_supply | nearToNum}}</span></p>
      <p>抵押规模: <span>{{contractInfo.total_collateral | nearToNum}}</span></p>
      <p>平台盈利: <span>{{contractInfo.sudoer_profit | nearToNum}}</span></p>
    </b-modal>

    <b-modal id="modal-2" title="平台政策" hide-footer>
      <p>币价: <span>{{contractInfo.mint_price}}</span></p>
      <p>游戏费: <span>{{getSudoer(contractInfo.sudoer_fee_play, contractInfo.shop_fee_play)}}</span></p>
      <p>赢家税: <span>{{getSudoer(contractInfo.sudoer_fee_win, contractInfo.shop_fee_win)}}</span></p>
      <p>铸币费: <span>{{getSudoer(contractInfo.burn_ratio)}}</span></p>
      <p>平台归属: <span>{{contractInfo.owner}}</span></p>
    </b-modal>
  </div>
</template>

<script>
export default {
  created () {
    this.getMetadata()
  },
  props: {
    contractInfo: {
      require: true
    }
  },
  data () {
    return {
      metadata: {}
    }
  },
  computed: {
    getSudoer () {
      return (sudoer, shop) => {
        if (!sudoer) return ''
        if (!shop) {
          const {denominator, numerator} = sudoer
          return numerator + '/' + denominator
        } else {
          const sudoerNumerator = sudoer.numerator
          const shopNumerator = shop.numerator
          // const {denominator, numerator} = sudoer
          return (shopNumerator + sudoerNumerator) + '/' + '1000'
        }
      }
    }
  },
  methods: {
    async getMetadata () {
      const metadata = await window.contract_platform.metadata()
      this.metadata = metadata
    }
  }
}
</script>

<style scoped>
.plat .logo-info {
  display: flex;
  align-items: center;
}
</style>