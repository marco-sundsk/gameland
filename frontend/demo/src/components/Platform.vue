<template>
  <div class="plat">
    <div class="plat-left">
      <div class="logo">
        <img src="../assets/img/logo.png" alt="">
      </div>
      <div class="info">
        <div class="plat-name">
          <div class="plat-version">v{{metadata.version}}</div>
          {{metadata.name}}
        </div>
        <div class="plat-desc">{{metadata.description}}</div>
      </div>
    </div>
    <div class="plat-right">
      <button>平台状态</button>
      <button>平台政策</button>
    </div>
  </div>
</template>

<script>
export default {
  created () {
    console.log(this.metadata)
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
      console.log(metadata)
      this.metadata = metadata
    }
  }
}
</script>

<style scoped>
.plat {
  color: #fff;
  display: flex;
  width: 100%;
  justify-content: space-between;
}
.plat .plat-left {
  display: flex;
  align-items: center;
}
.plat .plat-left .logo {
  margin-right: 10px;
}
.plat .plat-left .info .plat-name .plat-version {
  right: 0;
  top: 0;
  transform: translate(110%, -10%);
  box-shadow: 2px 2px 5px 2px rgb(32, 32, 32);
  position: absolute;
  line-height: 15px;
  padding: 0 5px;
  height: 15px;
  color: #000;
  font-size: 12px;
  text-align: center;
  background-color: #fff;
  border-radius: 3px 3px 3px 0;
}
.plat .plat-left .info .plat-name {
  padding-top: 5px;
  display: inline-block;
  position: relative;
  font-size: 20px;
  font-weight: 500;
}
.plat .plat-left .info .plat-desc {
  font-size: 14px;
  color: #d9d9d9;
}
.plat .plat-right {
  width: 200px;
  justify-content: space-around;
  display: flex;
  align-items: flex-end;
}
.plat .plat-right button {
  color: #fff;
  background-color: transparent;
  border: none;
}
</style>