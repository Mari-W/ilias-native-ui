<template>
  <v-container>
    <v-row align="center" justify="center">
      <v-col>
        <v-card flat color="transparent">
          <v-card-title class="flex-nowrap">
            <v-btn
                text
                large
                @click="back"
                class="mr-10 mb-1"
                :disabled="this.path === ''"
            >
              <v-icon>mdi-arrow-left-bold</v-icon>
              <span class="ml-2">Back</span>
            </v-btn>
            <v-autocomplete
                rounded
                label="SEARCH"
                type="text"
                :items="getItemsRec(items)"
                item-text="name"
                v-model="search"
                return-object
                :search-input.sync="searchInput"
                @change="searched(search)"
                hide-selected
                append-icon=""
            ></v-autocomplete>
          </v-card-title>
        </v-card>
      </v-col>
    </v-row>
    <v-row class="text-center" align="center" justify="center"
           v-bind="getItems"
           v-for="item in getItems"
           :key="item.name"
    >
      <v-col>
        <v-tooltip bottom color="transparent" disabled>
          <template v-slot:activator="{ on, attrs }">
            <v-card rounded @click="go(item)" hover v-bind="attrs"
                    v-on="on" :disabled="name(item).startsWith('.')" :loading="name(item).startsWith('.')">
              <v-card-title class="text-truncate flex-nowrap text-button">
                <v-icon large left class="mr-5">{{ icon(item) }}</v-icon>
                <span>{{ name(item) }}  <span
                    v-if="version(item) !== 1">| Version {{ version(item) }}</span></span></v-card-title>
            </v-card>
          </template>
          <span>{{ item.path }}</span>
        </v-tooltip>
      </v-col>
    </v-row>
  </v-container>
</template>

<script>
import {readDir} from "@tauri-apps/api/fs";
import store from "../store";
import {invoke} from "@tauri-apps/api/tauri";

export default {
  name: 'FileTree',

  data: () => ({
    history: [],
    path: "",
    items: [],
    search: null,
    searchInput: ''
  }),
  watch: {
    // eslint-disable-next-line no-unused-vars
    $route(to, _) {
      if (to.path === "/root/updated") {
        this.index(true)
        this.$router.push("/root/syncing")
      } else if (to.path === "root/finished") {
        this.index(true)
        this.$router.push("/root/")
      }
    },
  },
  beforeMount() {
    this.index()
  },
  computed: {
    getItems() {
      return this.items.sortBy('name');
    },
    console: () => console,
    window: () => window,
  },
  methods: {
    index(reload = false) {
      readDir(store.state.iliasDir, {recursive: true}).then((entries) => {
        if (reload) {
          let pieces = this.path.split("/")
          let new_items = entries;
          let new_history = [];
          let new_path = ""
          pieces.forEach(piece => {
            if (piece.length > 1) {
              new_items.forEach(item => {
                if (item.name === piece) {
                  new_history.push({
                    items: new_items,
                    path: new_path
                  })
                  new_items = item.children
                  new_path = new_path + "/" + item.name
                }
              })
            }
          })
          this.items = new_items;
          this.history = new_history;
          this.path = new_path;
        } else {
          this.items = entries;
        }
      })
    },
    go(item) {
      if (item.children != null) {
        this.history.push({
          items: this.items,
          path: this.path
        })

        this.items = item.children
        this.path = this.path + "/" + item.name
      } else {
        invoke("open", {path: item.path})
      }
    },
    searched(item) {
      this.$nextTick(() => {
        this.searchInput = '';
        this.search = null;
        this.go(item)
      });
    },
    back() {
      let old = this.history.pop();
      if (old !== undefined) {
        this.items = old.items;
        this.path = old.path;
      } else {
        this.path = "/"
      }
    },
    version(item) {
      if (item.name.includes("_V")) {
        const regex = /_V([0-9])+/
        let match = item.name.match(regex)
        if (match[1] !== undefined) {
          return match[1]
        }
      }
      return 1
    },
    name(item) {
      let version = this.version(item);
      if (item.name.startsWith(".")) {
        return item.name.replaceAll("_", " ")
      } else if (version !== 1) {
        return item.name.replace("_V" + version.toString(), "").replaceAll("_", " ")
      } else {
        return item.name.replaceAll("_", " ")
      }
    },
    icon(item) {
      if (item.children != null) {
        return "mdi-folder"
      } else {
        if (item.name.endsWith("pdf")) {
          return "mdi-file-pdf"
        } else if (item.name.endsWith("mp4")) {
          return "mdi-file-video"
        } else if (item.name.endsWith("png") || item.name.endsWith("jpeg") || item.name.endsWith("jpg")) {
          return "mdi-file-image"
        } else if (item.name.endsWith("zip") || item.name.endsWith("7z") || item.name.endsWith("gz") || item.name.endsWith("rar")) {
          return "mdi-file-multiple"
        } else {
          return "mdi-file"
        }
      }
    },
    getItemsRec(inp) {
      let items = []
      inp.forEach(item => {
        let name = this.name(item)
        if (!name.startsWith(".")) {
          items.push({
            name: name,
            path: item.path,
            children: item.children
          })
          if (item.children != null) {
            items = items.concat(this.getItemsRec(item.children))
          }
        }
      });
      return items
    },
  }
}
</script>
