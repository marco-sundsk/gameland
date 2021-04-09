<template>
  <div id="app">
    <header>
      <img class="plat-logo" src="./assets/img/logo.png" alt="">
      <b-navbar class="container" type="dark">
        <div class="col-6 nav-item">
          <div class="user">
            <div class="user-logo">
              <img src="./assets/img/icon-user.png" alt="">
            </div>
            <div class="user-info">
              <div class="user-name">Wellcome, James !</div>
              <div class="user-gpt">GPT: 690,000,00</div>
            </div>
          </div>
        </div>
        <div class="col-6 nav-item">
          <button class="back">BACK</button>
        </div>
      </b-navbar>
    </header>
    <main>
      <lucky-box :isMobile="isMobile"></lucky-box>
    </main>
  </div>
</template>

<script>
import LuckyBox from './components/LuckyBox'
export default {
  name: 'App',
  data () {
    return {
      isMobile: ''
    }
  },
  components: {
    LuckyBox
  },
  mounted () {
    let getDomWidth = () => {
      const domWidth = document.documentElement.offsetWidth || document.body.offsetWidth
      console.log(domWidth)
      this.isMobile = domWidth <= 767 ? true : false
    }
    window.addEventListener('resize', getDomWidth)
    getDomWidth()
  }
}
</script>

<style lang="less">
header {
  height: 120px;
  background: url('./assets/img/header-bg.png') no-repeat;
  background-size: cover;
  position: relative;
  .plat-logo {
    position: absolute;
    left: 20px;
    top: 50%;
    transform: translateY(-50%);
  }
  .container {
    height: 100%;
    .user {
      display: flex;
      align-items: center;
      .user-logo {
        background-color: red;
        border-radius: 50%;
        display: flex;
        width: 60px;
        height: 60px;
        justify-content: center;
        align-items: center;
        margin-right: 10px;
      }
      .user-info {
        .user-name {
          color: #f8ae1c;
          font-size: 18px;
        }
        .user-gpt {
          color: #f6c94a;
          font-weight: 300;
          font-size: 12px;
        }
      }
    }
    .back {
      border: none;
      background-color: #f8ae1c;
      color: #000;
      border-radius: 3px;
      width: 100px;
      height: 40px;
      font-size: 14px;
      float: right;
    }
  }
}
</style>
