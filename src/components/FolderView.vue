<template>
  <div
    :style="{ margin: '25px', 'margin-left': paddingLeft }"
    @click="clearSelectedEntries"
  >
    <PathLinks :path="path" v-on:path-link-click="handlePathLinkClick" />
    <div class="entry-view">
      <template v-for="entry in entries" :key="entry.label">
        <FolderCard
          v-if="entry.is_folder"
          :entry="entry"
          :selected="selected_entries"
          v-on:folder-click="handleEntryClick"
          v-on:folder-dblclick="handleFolderDoubleClick"
        ></FolderCard>
        <FileCard
          v-else
          :entry="entry"
          :selected="selected_entries"
          v-on:file-click="handleEntryClick"
        ></FileCard>
      </template>
    </div>
  </div>
</template>

<script lang="ts">
import { Options, Vue } from "vue-class-component";
import FolderCard from "./FolderCard.vue";
import FileCard from "./FileCard.vue";
import PathLinks from "./PathLinks.vue";
import getEntries from "../utils/getEntries";

@Options({
  components: {
    FolderCard,
    FileCard,
    PathLinks,
  },
  props: {
    path: String,
    paddingLeft: String,
  },
  emits: ["set-folder-view-path"],
  data() {
    return {
      entries: [],
      selected_entries: [],
    };
  },
  watch: {
    path() {
      this.updateEntries();
    },
  },
  created() {
    this.updateEntries();
  },
  methods: {
    getEntries,
    updateEntries() {
      const res = this.getEntries(this.path);
      res
        .then((response: any) => {
          const { entries } = response;
          this.entries = entries;
        })
        .catch((error: any) => console.log(error));
    },
    handlePathLinkClick(path: String) {
      this.$emit("set-folder-view-path", path);
    },
    handleEntryClick(entry: { full_path: String }) {
      this.selected_entries = [entry.full_path];
    },
    handleFolderDoubleClick(entry: { full_path: String }) {
      this.$emit("set-folder-view-path", entry.full_path);
    },
    clearSelectedEntries() {
      this.selected_entries = [];
    },
  },
})
export default class FolderView extends Vue {}
</script>

<style scoped>
.entry-view {
  display: flex;
  flex-wrap: wrap;
}
</style>
