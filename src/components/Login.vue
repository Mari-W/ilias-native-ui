<template>
  <v-container fill-height>

    <v-dialog
        v-model="dialog"
        persistent
        max-width="600"
    >
      <v-card>
        <v-card-title>
          SET ILIAS DIRECTORY
        </v-card-title>
        <v-card-subtitle class="mt-3">this is where the ilias directory tree with all its documents will be placed
        </v-card-subtitle>
        <v-divider class="mb-11"/>
        <v-card-text>
          <v-form>
            <v-text-field
                filled
                rounded
                dense
                label="DIRECTORY PATH"
                type="text"
                v-model="path"></v-text-field>
          </v-form>
        </v-card-text>

        <v-card-subtitle class="error--text title text-center" v-if="dialog_msg"> {{ dialog_msg }}</v-card-subtitle>

        <v-divider/>

        <v-card-actions>
          <v-spacer></v-spacer>
          <v-btn
              color="primary"
              text
              @click="create"
          >
            Continue
          </v-btn>
        </v-card-actions>
      </v-card>
    </v-dialog>

    <v-row class="text-center mt-10" align="center" justify="center">
      <v-col cols="12" sm="5">
        <v-card flat>
          <v-card-title class="justify-center" v-if="initialized">
            LOGIN using RZ ACCOUNT
          </v-card-title>
          <v-card-title class="justify-center" v-else>
            LOGGING into ILIAS..
          </v-card-title>
          <v-card-subtitle v-if="loading">
            <v-progress-linear
                indeterminate
                color="primary"
                class="mt-6 mb-5"
            ></v-progress-linear>
          </v-card-subtitle>
          <v-card-subtitle v-else>
            <v-divider class="mt-6 mb-5"/>
          </v-card-subtitle>
          <v-card-text v-if="initialized">
            <v-form>
              <v-text-field
                  filled
                  rounded
                  dense
                  label="USERNAME"
                  type="text"
                  v-model="username"></v-text-field>
              <v-text-field
                  filled
                  rounded
                  dense
                  label="PASSWORD"
                  type="password"
                  v-model="password"></v-text-field>
            </v-form>
          </v-card-text>
          <v-card-subtitle class="error--text title" v-if="msg"> {{ msg }}</v-card-subtitle>
          <v-card-subtitle v-if="initialized">
            <v-divider/>
          </v-card-subtitle>
          <v-card-actions v-if="initialized">
            <v-btn x-large :disabled="loading" color="primary" text @click="runLogin">Login</v-btn>
          </v-card-actions>
        </v-card>
      </v-col>
    </v-row>
  </v-container>
</template>

<script>
import {invoke} from '@tauri-apps/api/tauri'
import store from "../store";
import {createDir, readDir} from "@tauri-apps/api/fs";
import {homeDir} from "@tauri-apps/api/path";

export default {
  name: 'Login',

  data: () => ({
    password: '',
    username: '',
    msg: null,
    loading: false,
    initialized: false,

    dialog: false,
    path: '',
    dialog_msg: null,
  }),

  created() {
    if (store.state.iliasDir == null) {
      homeDir().then((r) => {
        this.path = r + "Ilias"
        readDir(this.path).then(() => {
          store.commit('setIliasDir', this.path)
          if (store.state.credentials == null) {
            this.initialized = true;
          } else {
            this.login(store.state.credentials);
          }
        }).catch(() => {
          this.dialog = true;

        })
      })
    } else {
      readDir(store.state.iliasDir).then(() => {
        if (store.state.credentials == null) {
          this.initialized = true;
        } else {
          this.login(store.state.credentials);
        }
      }).catch(() => {
        store.commit("setIliasDir", null)
        homeDir().then((r) => {
          this.path = r + "Ilias"
          this.dialog = true;
        })
      })
    }
  },

  methods: {
    login(credentials) {
      this.loading = true;
      invoke('login', credentials).then(() => {
        store.commit('login', credentials)
        this.loading = false;
        this.$router.push("/root/")
      }).catch((msg) => {
        this.msg = msg;
        store.commit('logout')
        this.loading = false;
        this.initialized = true;
      })
    },
    runLogin() {
      this.login({username: this.username, password: this.password})
    },
    create() {
      createDir(this.path, {recursive: true}).catch((e) => {
        this.dialog_msg = e;
      }).then(() => {
        store.commit("setIliasDir", this.path)
        this.dialog = false;
        if (store.state.credentials == null) {
          this.initialized = true;
        } else {
          this.login(store.state.credentials);
        }
      })
    },
  }
}
</script>
