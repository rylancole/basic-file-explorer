<template>
  <div :style="{ margin: '25px', 'margin-left': paddingLeft }">
    <PathLinks :path="path" />
    <div class="entry-view">
      <template v-for="entry in entries" :key="entry.label">
        <FolderCard
          v-if="entry.is_folder"
          :entry="entry"
          :setFolderViewPath="setFolderViewPath"
        ></FolderCard>
        <FileCard v-else :entry="entry"></FileCard>
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
    setFolderViewPath: Function,
  },
  data() {
    return {
      entries: [],
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
