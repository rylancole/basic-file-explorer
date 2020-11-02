<template>
  <w-app>
    <NavBar
      :entries="entries"
      :width="navBarWidth + 'px'"
      :setPathEnd="setPathEnd"
    />
    <FolderView :path="PATH + pathEnd" :paddingLeft="navBarWidth + 25 + 'px'" />
  </w-app>
</template>

<script lang="ts">
import { Options, Vue } from "vue-class-component";
import { promisified } from "tauri/api/tauri";
import NavBar from "./components/NavBar.vue";
import FolderView from "./components/FolderView.vue";

@Options({
  components: {
    FolderView,
    NavBar,
  },
  data() {
    return {
      entries: [],
      PATH: "/Users/rylancole/Documents/3D Printer",
      pathEnd: "/Local",
      navBarWidth: 175,
    };
  },
  created() {
    const res = this.getEntries(this.PATH);
    res
      .then((response: any) => {
        const { entries } = response;
        this.entries = entries;
      })
      .catch((error: any) => console.log(error));
  },
  methods: {
    getEntries(path: String) {
      return promisified({
        cmd: "getEntries",
        path: path,
      })
        .then((response: any) => {
          return response;
        })
        .catch((error) => {
          throw error;
        });
    },
    setPathEnd(value: String) {
      this.pathEnd = value;
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
