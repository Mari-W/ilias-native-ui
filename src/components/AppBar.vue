<template>
  <v-container>
    <v-app-bar app color="primary" dense>
      <v-app-bar-title>ILIAS NATIVE</v-app-bar-title>


      <v-spacer/>

      <v-btn
          href="https://github.com/Mari-W/ilias-native-ui/"
          target="_blank"
          text
      >
        <span class="mr-2">Help</span>
        <v-icon>mdi-open-in-new</v-icon>
      </v-btn>

      <dark-mode-button/>

      <v-divider v-if="this.$route.path !== '/'" class="mr-2 ml-2" vertical inset></v-divider>

      <sync-button/>

      <v-btn
          text
          v-if="this.$route.path !== '/'"
          @click="logout"
          :disabled="this.$route.path !== '/root/'"
      >
        <span class="mr-2">Logout</span>
        <v-icon>mdi-logout</v-icon>
      </v-btn>
    </v-app-bar>
  </v-container>
</template>

<script>
import DarkModeButton from "../components/DarkModeButton";
import SyncButton from "./SyncButton";

export default {
  name: "AppBar",
  components: {SyncButton, DarkModeButton},

  methods: {
    logout() {
      this.$store.commit('logout')
      this.$router.push("/")
    },
    sync() {
      this.$store.dispatch('sync')
    }
  },
  watch: {
    // eslint-disable-next-line no-unused-vars
    syncing(newValue, _) {
      this.syncing = newValue;
    }
  },
  computed: {
    syncing() {
      return this.$store.state.syncing
    }
  }
}
</script>

<style scoped>
</style>
