<template>
  <div class="game-list">
    <b-list-group horizontal>
      <b-list-group-item v-for="(item, index) in groupItem" :key="index">
        <div class="game-info">
          <div class="game-logo">
            <img :src="item.logo_url" alt="" width="20px" height="20px">
          </div>
          <div class="game-version">v{{item.version}}</div>
        </div>
        <div class="game-thumb">
          <img :src="item.thumb_url" :title="item.description" alt="" width="200px" height="200px" @click="goPlay(item)">
        </div>
        <div class="pub-state">{{item.pubState}}</div>
        <div class="user-state">{{item.userState}}</div>
      </b-list-group-item>
    </b-list-group>
  </div>
</template>

<script>
export default {
  created () {
    this.getGameList()
  },
  data () {
    return {
      groupItem: []
    }
  },
  methods: {
    async getGameList () {
      try {
        const gameList = []
        const game = await window.contract_platform.list_shops({
          from_index: 0,
          limit: 100
        })
        if (game.length !== 0) {
          game.forEach( async  ({shop_id}) => {
            const gameInfo = await window.contract_game.gl_metadata({
              account_id: shop_id
            })
            const pubState = await window.contract_game.gl_pub_state()
            const userState = await window.contract_game.gl_user_state({
              user_id: shop_id
            })
            gameInfo.pubState = pubState
            gameInfo.userState = userState
            gameList.push(gameInfo)
          })
          this.groupItem = gameList
          // console.log(gameList)
        }
      } catch (err) {
        console.erro(err)
      }
    },
    goPlay (item) {
      const origin = window.location.origin
      // window.open('https://www.baidu.com')
      window.open(origin + item.link_url)
    }
  }
}
</script>

<style scoped>
.game-list .list-group-item {
  flex: none;
  max-width: 500px;
  width: 80vw;
  height: 300px;
  margin: 0 50px 50px;
}
.game-list .list-group {
  display: flex;
  width: 100%;
  background-color: skyblue;
  justify-content: space-around;
  flex-wrap: nowrap;
  overflow: auto;
  padding: 30px;
}
.game-list .list-group-item .game-info {
  width: 100%;
  display: flex;
  justify-content: space-between;
  align-items: center;
}
.game-list .list-group-item .game-thumb {
  display: flex;
  justify-content: center;
  align-items: center;
}
</style>
