import Vue from 'vue'
import VueRouter from 'vue-router'
import Init from '../views/Init.vue'
import Home from "../views/Home";

Vue.use(VueRouter)

const routes = [
    {
        path: '/',
        name: 'Init',
        component: Init
    },
    {
        path: '/root/*',
        name: 'Home',
        component: Home
    },
]

const router = new VueRouter({
    mode: 'history',
    base: process.env.BASE_URL,
    routes
})

export default router
