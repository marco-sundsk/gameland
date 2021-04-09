<template>
  <div id="app">
    <header>
      <img class="plat-logo" src="./assets/img/logo.png" alt="">
      <b-navbar v-if="!isMobile" class="container" type="dark">
        <div class="col-6 nav-item">
          <div class="user">
            <div class="user-logo">
              <img src="./assets/img/icon-user.png" alt="">
            </div>
            <div class="user-info">
              <div class="user-name">Wellcome, {{currentUser.accountId}} !</div>
              <div class="user-gpt">GPT: {{gptBalance | nearToNum}}</div>
            </div>
          </div>
        </div>
        <div class="col-6 nav-item">
          <button class="back">BACK</button>
        </div>
      </b-navbar>
      <b-navbar v-else class="container mobile-header" type="dark">
        <div class="nav-item first"></div>
        <div class="nav-item">
          <div class="user">
            <div class="user-logo">
              <img src="./assets/img/icon-user.png" alt="">
            </div>
            <div class="user-info">
              <div class="user-name">{{currentUser.accountId}}</div>
              <div class="user-gpt">GPT: {{gptBalance | nearToNum}}</div>
            </div>
          </div>
        </div>
        <div class="nav-item">
          <button class="back">BACK</button>
        </div>
      </b-navbar>
    </header>
    <main class="main">
      <angle-land :isMobile="isMobile" :currentUser="currentUser" @getGptBalance="getGptBalance" :gptBalance="gptBalance"></angle-land>
    </main>
  </div>
</template>

<script>
import AngleLand from './components/AngleLand'
export default {
  name: 'App',
  data () {
    return {
      isMobile: '',
      currentUser: {
        accountId: '',
        account_id: '',
        balance: ''
      },
      gptBalance: ''
    }
  },
  components: {
    AngleLand
  },
  methods: {
    // 获取游戏币余额
    async getGptBalance () {
      try {
        const balance = await window.contract_gamecoin.ft_balance_of({
          account_id: this.currentUser.account_id
        })
        this.gptBalance = balance
      } catch (err) {
        console.error(err)
      }
    },
  },
  created () {
    if (window.walletConnection.isSignedIn()) {
      this.currentUser = window.currentUser
      this.getGptBalance()
    } else {
      window.location.href = window.location.origin
    }
  },
  mounted () {
    let getDomWidth = () => {
      const domWidth = document.documentElement.offsetWidth || document.body.offsetWidth
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
        border: 1px solid #f8ae1c;
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
  .mobile-header {
      display: flex;
      justify-content: space-between;
      .first {
        width: 50px;
        height: 40px;
      }
      .user {
        .user-logo {
          width: 40px;
          height: 40px;
          margin-right: 5px;
        }
        .user-info {
          .user-name {
            font-size: 14px;
          }
        }
      }
      .back {
        width: 50px;
        height: 32px;
        font-size: 14px;
      }
    }
}
</style>
