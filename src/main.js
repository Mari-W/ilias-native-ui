import Vue from 'vue'
import App from './App.vue'
import router from './router'
import store from './store'
import VueCookie from 'vue-cookie'
import Vuetify from "vuetify";

Vue.config.productionTip = false

Vue.use(VueCookie);
Vue.use(Vuetify);

const vuetify = new Vuetify({});

Array.prototype.sortBy = function(p) {
    return this.slice(0).sort(function(a,b) {
        return (a[p] > b[p]) ? 1 : (a[p] < b[p]) ? -1 : 0;
    });
}

new Vue({
    router,
    store,
    vuetify,
    render: h => h(App)
}).$mount('#app')
