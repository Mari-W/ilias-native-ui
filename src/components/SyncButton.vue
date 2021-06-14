<template>
  <v-btn
      text
      v-if="this.$route.path !== '/'"
      @click="sync"
      :loading="this.syncing"
  >
    <span class="mr-2">Sync</span>
    <v-icon>mdi-reload</v-icon>

    <v-dialog
        v-model="dialog"
        max-width="600"
    >
      <v-card color="error">
        <v-card-title class="text-button">
          AN ERROR OCCURRED :(
        </v-card-title>
        <v-divider/>
        <v-card-text class="text-button title text-wrap mt-4">{{ error }}</v-card-text>
        <v-card-actions>
          <v-btn text target="_blank" href="https://github.com/Mari-W/ilias-native-ui/issues">report</v-btn>
          <v-spacer/>
          <v-btn text @click="dialog=false">close</v-btn>
        </v-card-actions>
      </v-card>
    </v-dialog>
  </v-btn>
</template>

<script>
import {invoke} from "@tauri-apps/api/tauri";
import store from "../store";

export default {
  name: "SyncButton",
  data() {
    return {
      syncing: false,
      dialog: false,
      error: null,
    }
  },
  watch: {
    $route(to, from) {
      if (from.path === "/" && to.path === "/root/") {
        this.sync()
      }
    }
  },
  methods: {
    sync() {
      this.syncing = true;
      this.$router.push("/root/syncing")
      let upd = setInterval(() => {
        this.$router.push("/root/updated")
      }, 3000);
      invoke("sync", {root: store.state.iliasDir}).then(() => {
        clearInterval(upd)
        this.syncing = false;
        this.$router.push("/root/finished")
      }).catch((e) => {
        this.error = e;
        this.dialog = true;
        clearInterval(upd)
        this.syncing = false;
        this.$router.push("/root/finished")
      })
    }
  }
}
</script>

<style scoped>

</style>
