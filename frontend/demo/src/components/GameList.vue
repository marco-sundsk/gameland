<template>
  <div class="game">
    <div class="game-left">
      <div class="game-left-title">
        热门小游戏
      </div>
      <div class="game-left-desc">
        轻量，有趣，随时随地，理性杀时间小游戏
      </div>
      <div class="game-left-see">
        <button>
          <span>
            <img src="../assets/img/demo-more-icon.png" alt="">
            See all demos
          </span>
        </button>
      </div>
    </div>
    <div class="game-list"></div>
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
.game {
  max-width: 1600px;
  box-sizing: border-box;
  padding: 112px 0 120px;
}
.game .game-left {
  padding: 0 100px;
}
.game .game-left .game-left-title {
  font-size: 34px;
  color: #58627c;
  margin-bottom: 16px;
}
.game .game-left .game-left-desc {
  color: #58627c;
  font-size: 14px;
}
.game .game-left .game-left-see {
  padding-top: 80px;
}
.game .game-left .game-left-see button {
  color: #e8edfa;
  font-size: 14px;
  background-color: #000;
  border-radius: 5px;
  border: none;
  width: 172px;
  height: 45px;
  display: flex;
  justify-content: center;
  align-items: center;
}
.game .game-left .game-left-see button img {
  vertical-align: baseline;
  margin-right: 10px;
}
</style>
