import Vue from 'vue'
import Vuex from 'vuex'
import createPersistedState from "vuex-persistedstate";
import SecureLS from "secure-ls";

const ls = new SecureLS({isCompression: false});

Vue.use(Vuex)

const store = new Vuex.Store({
    state: {
        credentials: null,
        iliasDir: null,
        syncing: false,

        text: "",
        color: "",
        timeout: "",
    },
    mutations: {
        login(state, payload) {
            state.credentials = payload
        },
        logout(state) {
            state.credentials = null
        },
        setIliasDir(state, iliasDir) {
            state.iliasDir = iliasDir
        },
    },
    actions: {

    },
    modules: {},
    plugins: [
        createPersistedState({
            storage: {
                getItem: (key) => ls.get(key),
                setItem: (key, value) => ls.set(key, value),
                removeItem: (key) => ls.remove(key),
            },
        }),
    ],
})
export default store
