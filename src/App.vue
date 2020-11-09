<template>
  <w-app>
    <NavBar
      :entries="entries"
      :width="navBarWidth + 'px'"
      :setFolderViewPath="setFolderViewPath"
    />
    <FolderView
      :path="folderViewPath"
      :paddingLeft="navBarWidth + 25 + 'px'"
      v-on:set-folder-view-path="setFolderViewPath"
    />
  </w-app>
</template>

<script lang="ts">
import { Options, Vue } from "vue-class-component";
import NavBar from "./components/NavBar.vue";
import FolderView from "./components/FolderView.vue";
import { DEFAULT_PATH } from "./constants";
import getEntries from "./utils/getEntries";

@Options({
  components: {
    FolderView,
    NavBar,
  },
  data() {
    return {
      entries: [],
      navBarPath: DEFAULT_PATH,
      folderViewPath: DEFAULT_PATH,
      navBarWidth: 175,
    };
  },
  created() {
    const res = this.getEntries(this.navBarPath);
    res
      .then((response: any) => {
        const { entries } = response;
        this.entries = entries;
      })
      .catch((error: any) => console.log(error));
  },
  methods: {
    getEntries,
    setFolderViewPath(path: String) {
      this.folderViewPath = path;
    },
  },
})
export default class App extends Vue {}
</script>

<style>
#app {
  font-family: Avenir, Helvetica, Arial, sans-serif;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  color: #2c3e50;
}
</style>
