import Vue from "vue"
import { BootstrapVue } from 'bootstrap-vue'
import App from "./App.vue"
import moment from 'moment'


// Import Bootstrap an BootstrapVue CSS files (order is important)
import 'bootstrap/dist/css/bootstrap.css'
import 'bootstrap-vue/dist/bootstrap-vue.css'
import { initContract } from "./utils"

// Make BootstrapVue available throughout your project
Vue.use(BootstrapVue)

Vue.filter('changeNear', function (num) {
  const ratio = '000000000000000000000000'
  if (num === '0') return '0'
  if (num.indexOf('.') !== -1) {
    const arr = num.split('.')
    if (arr[0] !== 0) {
      return arr[0] + arr[1] + ratio.slice(0, (ratio.length - arr[1].length))
    } else {
      return arr[1] + ratio.slice(0, (ratio.length - arr[1].length))
    }
  } else {
    return num + ratio
  }
})

Vue.filter('nearToNum', function (num, fix = 2) {
  return (num / 1e24).toFixed(fix)
})

Vue.filter('changeTime', function (value) {
  const ts = Number((value / 1e6).toFixed(0))
  return moment(ts).format('YYYY-MM-DD HH:mm:ss')
})


Vue.config.productionTip = false

window.nearInitPromise = initContract()
  .then(() => {
    new Vue({
      render: h => h(App),
    }).$mount("#app")
  })
  