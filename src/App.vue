<template>
  <w-app>
    <NavBar v-bind:entries="entries" />
    <FolderView />
  </w-app>
</template>

<script lang="ts">
import { Options, Vue } from 'vue-class-component';
import { promisified } from 'tauri/api/tauri'
import NavBar from './components/NavBar.vue';
import FolderView from './components/FolderView.vue';

@Options({
  components: {
    FolderView,
    NavBar,
  },
  data () {
    return {
      entries: []
    }
  },
  created () {
    this.getEntries();
  },
  methods: {
    getEntries () {
      promisified({
        cmd: "getChildren",
        path: "/Users/rylancole/Documents/3D Printer"
      }).then((response: any) => {
        const { entries } = response;
        this.entries = entries;
      }).catch(error => {
        console.log(error)
      })
    }
  }
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
