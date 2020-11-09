<template>
  <div class="path-links">
    <w-button
      bg-color="white"
      icon="mdi mdi-arrow-left-thick"
      @click="$emit('path-link-click', backPath)"
    />
    <div v-for="item in pathList" :key="item.label">
      <w-button
        bg-color="white"
        @click="$emit('path-link-click', item.full_path)"
        >{{ item.label }}</w-button
      >
    </div>
  </div>
</template>

<script lang="ts">
import { Options, Vue } from "vue-class-component";

@Options({
  props: {
    path: String,
    setFolderViewPath: Function,
  },
  emits: ["path-link-click"],
  computed: {
    pathList() {
      const wholeList = this.path.split("/");

      let pathList = [];
      let pathString = "";

      for (let item of wholeList) {
        if (item != "") {
          pathString += "/" + item;
          pathList.push({ full_path: pathString, label: item });
        }
      }
      return pathList;
    },
    backPath() {
      const wholeList = this.path.split("/");

      let pathList = [];

      for (let item of wholeList) {
        if (item != "") {
          pathList.push(item);
        }
      }
      const n = pathList.length;

      const backPath = pathList.slice(0, n - 1).join("/");
      return "/" + backPath;
    },
  },
  methods: {
    handleLinkClick(item: { full_path: String }) {
      this.setFolderViewPath(item.full_path);
    },
    handleBackClick() {
      this.setFolderViewPath(this.backPath);
    },
  },
})
export default class PathLinks extends Vue {}
</script>

<style>
.path-links {
  display: flex;
  flex-wrap: wrap;
}

.path-links div::before {
  content: "/";
}
</style>
