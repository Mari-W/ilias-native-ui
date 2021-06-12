<template>
  <v-btn
      text
      @click="change"
  >
    <span class="mr-2">Theme</span>
    <v-icon>mdi-theme-light-dark</v-icon>
  </v-btn>
</template>

<script>
export default {
  name: "DarkModeButton",
  methods: {
    change() {
      if (this.$cookie.get('theme') !== 'dark') {
        this.$cookie.set('theme', 'dark');
        this.$vuetify.theme.dark = true;
      } else {
        this.$cookie.set('theme', 'default');
        this.$vuetify.theme.dark = false;
      }
    },
  }
}
</script>
