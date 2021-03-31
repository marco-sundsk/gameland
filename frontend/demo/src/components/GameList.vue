<template>
  <div class="game">
    <div class="game-left">
      <div class="game-left-title">
        HOT MINI-GAMES
      </div>
      <div class="game-left-desc">
        Lightweight, fun, pastime on chain
      </div>
      <div class="game-left-see">
        <button>
          <span>
            <img src="../assets/img/icon-demo-more.png" alt="">
            See all demos
          </span>
        </button>
      </div>
    </div>
    <div class="game-list">
      <div class="game-wrap" v-for="(item, index) in groupChange(groupItem)" :key="index">
        <div class="game-logo">
          <img :src="getLogoUrl(item.shopId)" alt="" title="description" width="321" height="217" @click="goPlay(item)">
        </div>
        <div class="game-name">
          {{item.name}}
          <div class="game-version">v{{item.version}}</div>
        </div>
        <div class="game-message">{{item.pubState}}</div>
        <div class="game-message">{{item.userState}}</div>
      </div>
    </div>
  </div>
</template>

<script>
import { getShop } from '../utils.js'

export default {
  created () {
    this.getGameList()
  },
  data () {
    return {
      groupItem: []
    }
  },
  computed: {
    getLogoUrl () {
      return shopId => {
        return 'http://demo.gamezone.network' + '/dapps-img/' + shopId + '.thumb.png'
      }
    },
    groupChange () {
      return (list) => {
        if (list.length == 0) {
          return []
        } else {
          const myList = list.slice().sort((a, b) => a.name.charCodeAt(0) - b.name.charCodeAt(0))
          return myList
        }
      }
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
            console.log('shop_id', shop_id)
            let contractGame = await getShop(shop_id)
            const gameInfo = await contractGame.gl_metadata({})
            const pubState = await contractGame.gl_pub_state()
            const userState = await contractGame.gl_user_state({
              user_id: shop_id
            })
            gameInfo.pubState = pubState
            gameInfo.userState = userState
            gameInfo.shopId = shop_id
            gameList.push(gameInfo)
          })
          this.groupItem = gameList
          console.log(gameList)
        }
      } catch (err) {
        console.erro(err)
      }
    },
    goPlay (item) {
      const origin = window.location.origin
      // window.open('https://www.baidu.com')
      window.open(origin + '/' + item.shopId)
    }
  }
}
</script>

<style scoped>
.game {
  max-width: 1600px;
  box-sizing: border-box;
  padding: 112px 0 120px;
  margin: 0 auto;
  background-color: #2f2f2f;
  display: flex;
  width: 100%;
  /* justify-content: space-between; */
}
.game .game-left {
  padding: 0 100px;
}
.game .game-left .game-left-title {
  font-size: 34px;
  color: #f8ae1c;
  margin-bottom: 16px;
}
.game .game-left .game-left-desc {
  color: #f6c94a;
  font-size: 14px;
}
.game .game-left .game-left-see {
  padding-top: 80px;
}
.game .game-left .game-left-see button {
  color: #000;
  font-size: 14px;
  background-color: #f8ae1c;
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
.game .game-list {
  display: flex;
  flex: 1;
  justify-content: space-around;
  color: #f6c94a;
  font-size: 12px;
}
.game .game-list .game-wrap {
  width: 350px;
}
.game .game-list .game-wrap .game-logo img {
  cursor: pointer;
}
.game .game-list .game-wrap .game-name {
  font-size: 20px;
  font-weight: 500;
  color: #f8ae1c;
  position: relative;
  display: inline-block;
  line-height: 40px;
}
.game .game-list .game-wrap .game-name .game-version {
  right: 0;
  top: 5px;
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
.game .game-list .game-wrap .game-message {
  line-height: 20px;
}
</style>
