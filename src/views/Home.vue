<template>
  <v-container>
    <file-tree/>
  </v-container>
</template>

<script>
import FileTree from "../components/FileTree";

export default {
  name: 'Home',

  components: {
    FileTree
  },
}
</script>
