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
      <button @click="logout">
        <img src="../assets//img/icon-logout.png" alt="">
      </button>
    </div>
  </div>
</template>

<script>
export default {
  props: {
    contractInfo: {
      require: true
    },
    currentUser: {
      require: true
    }
  },
  methods: {
    logout () {
      this.$emit('logout')
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
</style>
