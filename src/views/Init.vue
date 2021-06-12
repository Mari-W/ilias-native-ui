<template>
  <v-container>
    <login/>
  </v-container>
</template>

<script>
import Login from '../components/Login'
export default {
  name: 'Init',

  created() {
    const get = this.$cookie.get('theme');
    if (get === 'dark')
      this.$vuetify.theme.dark = true;
    else if (get === 'default') {
      this.$vuetify.theme.dark = false;
    } else {
      const system = window.matchMedia && window.matchMedia('(prefers-color-scheme: dark)').matches
      this.$cookie.set('theme', system ? 'dark' : 'default');
      this.$vuetify.theme.dark = system;
    }
    this.$vuetify.theme.themes.dark.primary = '#607D8B'
    this.$vuetify.theme.themes.light.primary = '#607D8B'
    this.$vuetify.theme.themes.dark.secondary = '#455A64'
    this.$vuetify.theme.themes.light.secondary = '#455A64'
  },

  components: {
    Login,
  },
}
</script>
