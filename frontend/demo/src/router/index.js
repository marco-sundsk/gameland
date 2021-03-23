import Vue from 'vue'
import VueRouter from 'vue-router'
import Index from '../view/index.vue'
import Gameland from '../view/game/gameLand.vue'

Vue.use(VueRouter)

const routes = [
  {
    path: '/*',
    name: 'Index',
    component: Index
  },
  {
    path: '/gameland',
    name: 'gameland',
    // route level code-splitting
    // this generates a separate chunk (about.[hash].js) for this route
    // which is lazy-loaded when the route is visited.
    component: Gameland
  }
]

const router = new VueRouter({
  mode: 'history',
  routes
})

export default router
