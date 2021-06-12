<template>
  <v-app>
    <app-bar/>
    <v-main>
      <router-view/>
    </v-main>
  </v-app>
</template>

<script>

import AppBar from "./components/AppBar";
export default {
  name: 'App',
  components: {AppBar},
  data: () => ({
    //
  }),
};
</script>

<style lang="scss" scoped>
.v-application {
  font-family: 'Roboto', monospace;
  font-weight: 400;
  font-size: 0.875rem;
  letter-spacing: .0892857143em;
}
</style>
