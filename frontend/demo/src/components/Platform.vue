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
      <button v-b-toggle.sidebar-backdrop>
        STATUS
        <img src="../assets/img/icon-menu-status.png" alt="">
      </button>
      <button v-b-toggle.sidebar-contract>RULES</button>
    </div>
  </div>
</template>

<script>
export default {
  created () {
    // this.getMetadata()
  },
  props: {
    contractInfo: {
      require: true
    },
    metadata: {
      require: true
    }
  }
}
</script>

<style>
.plat {
  color: #f8ae1c;
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
  background-color: #44d690;
  border-radius: 3px 3px 3px 0;
}
.plat .plat-left .info .plat-name {
  font-family: "Times New Roman", Times, serif;
  padding-top: 5px;
  display: inline-block;
  position: relative;
  font-size: 20px;
  font-weight: 700;
}
.plat .plat-left .info .plat-desc {
  font-size: 14px;
  color: #f6c94a;
}
.plat .plat-right {
  width: 200px;
  justify-content: space-around;
  display: flex;
  align-items: flex-end;
}
.plat .plat-right button {
  color: #f8ae1c;
  background-color: transparent;
  border: none;
}
</style>