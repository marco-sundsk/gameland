<template>
  <div class="mobile-sidebar">
    <b-sidebar
      id="sidebar-mobile"
      title="Sidebar with backdrop"
      no-header
      shadow
    >
      <template #default="{ hide }">
        <div class="sidebar-header">
          <button @click="deHide(hide)">
            <img src="../assets/img/icon-close.png" alt="">
          </button>
        </div>
        <div class="sidebar-body">
          <div class="sidebar-main">
            <div class="main-top">
              <div class="nav-list">
                <div class="nav-item">
                  <button v-b-toggle.sidebar-backdrop>
                    <img src="../assets/img/icon-status.png" alt="" width="30">
                    <span>STATUS</span>
                    <img src="../assets/img/icon-menu-status.png" alt="" width="15">
                  </button>
                </div>
                <div class="nav-item">
                  <button v-b-toggle.sidebar-contract>
                    <img src="../assets/img/icon-policy.png" alt="" width="30">
                    <span>RULES</span>
                  </button>
                </div>
              </div>
            </div>
            <div class="main-bottom">
              <div v-if="isSignedIn" class="user-wrap">
                <div class="user-logo">
                  <div class="logo">
                    <img src="../assets/img/icon-user.png" alt="" width="40px">
                  </div>
                  <div class="user-name">{{currentUser.accountId}}</div>
                </div>
                <div class="user-wallet">
                  <div class="wallet-item">
                    NEAR BALANCE: {{currentUser.balance | nearToNum}} NEAR
                  </div>
                  <div class="wallet-item">
                    GAMECOIN BALANCE: {{gptBalance | nearToNum}} GPT
                  </div>
                </div>
                <button class="change" v-b-toggle.sidebar-user>Borrow GPT</button>
                <button class="logout" @click="logout">Logout</button>
              </div>
              <div v-else class="login-wrap">
                <button class="signin" @click="login">Login</button>
              </div>
            </div>
          </div>
        </div>
      </template>
    </b-sidebar>
  </div>
</template>

<script>
export default {
  props: {
    currentUser: {
      require: true
    },
    gptBalance: {
      require: true
    }
  },
  computed: {
    isSignedIn () {
      return window.walletConnection.isSignedIn()
    }
  },
  methods: {
    deHide (hide) {
      this.$emit('domMove')
      hide()
    },
    logout () {
      this.$emit('logout')
    },
    login () {
      this.$emit('login')
    }
  }
}
</script>

<style>
.mobile-sidebar .b-sidebar {
  border-radius: 0px !important;
  background-color: #494949 !important;
  color: #f8ae1c !important;
  text-align: left !important;
  width: 80%;
  max-width: 500px !important;
}
.mobile-sidebar .b-sidebar .sidebar-header {
  display: flex;
  width: 100%;
  justify-content: flex-end;
  align-items: flex-start;
  padding-top: 20px;
  padding-right: 20px;
  height: 100px;
  position: absolute;
  left: 0;
  top: 0;
  z-index: 999;
}
.mobile-sidebar .b-sidebar .sidebar-header button {
  background-color: transparent;
  border: none;
}
.mobile-sidebar .b-sidebar .sidebar-body {
  /* height: auto; */
  width: 100%;
  display: flex;
  justify-content: center;
  height: 100%;
  position: absolute;
  left: 0;
  top: 0;
  padding-top: 100px;
}
.mobile-sidebar .b-sidebar .sidebar-body .sidebar-main {
  width: 80%;
  border-top: 1px solid #f8ae1c;
  height: 100%;
  padding: 10px 10px 80px;
  display: flex;
  flex-direction: column;
  justify-content: space-between;
}
.mobile-sidebar .b-sidebar .sidebar-body .sidebar-main .main-top .nav-list  .nav-item {
  padding: 20px 0;
}
.mobile-sidebar .b-sidebar .sidebar-body .sidebar-main .main-top .nav-list button {
  background-color: transparent;
  border: none;
  color: #f8ae1c;
  display: flex;
  align-items: center;
  font-size: 18px;
}
.mobile-sidebar .b-sidebar .sidebar-body .sidebar-main .main-top .nav-list button img {
  margin-right: 10px;
  vertical-align: middle;
}
.mobile-sidebar .b-sidebar .sidebar-body .sidebar-main .main-top .nav-list button span {
  text-align: justify;
  /* width: 70px; */
  display: inline-block;
  text-align-last: justify;
  margin-right: 5px;
}
.mobile-sidebar .b-sidebar .sidebar-body .sidebar-main .main-bottom .user-wrap .user-logo {
  margin-bottom: 20px;
  text-align: center;
}
.mobile-sidebar .b-sidebar .sidebar-body .sidebar-main .main-bottom .user-wrap .user-logo .logo {
  width: 70px;
  height: 70px;
  border-radius: 50%;
  background-color: red;
  display: flex;
  justify-content: center;
  align-items: center;
  margin: 0 auto 7px;
}
.mobile-sidebar .b-sidebar .sidebar-body .sidebar-main .main-bottom .user-wrap .user-logo .user-name {
  font-size: 20px;
}
.mobile-sidebar .b-sidebar .sidebar-body .sidebar-main .main-bottom .user-wrap .user-wallet {
  font-size: 14px;
  font-weight: 300;
  color: #f6c94a;
  text-align: center;
  margin-bottom: 25px;
}
.mobile-sidebar .b-sidebar .sidebar-body .sidebar-main .main-bottom .user-wrap .user-wallet .wallet-item {
  /* position: relative;
  transform: translateX(-50%);
  left: 50%;
  display: flex;
  width: auto; */
}
.mobile-sidebar .b-sidebar .sidebar-body .sidebar-main .main-bottom .user-wrap .change {
  background-color: #f8ae1c;
  color: #000;
  font-size: 20px;
  margin: 0 auto 40px;
  border-radius: 3px;
  border: none;
  display: block;
  width: 200px;
  height: 40px;
}
.mobile-sidebar .b-sidebar .sidebar-body .sidebar-main .main-bottom .user-wrap .logout {
  background-color: transparent;
  color: #000;
  font-size: 20px;
  margin: 0 auto;
  border-radius: 3px;
  border: none;
  display: block;
  width: 200px;
  height: 40px;
}
.mobile-sidebar .b-sidebar .sidebar-body .sidebar-main .main-bottom .login-wrap .signin {
  background-color: #f8ae1c;
  color: #000;
  font-size: 20px;
  margin: 0 auto;
  border-radius: 3px;
  border: none;
  display: block;
  width: 200px;
  height: 40px;
}
</style>