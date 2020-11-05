<template>
  <EntryCard
    v-if="entry"
    :label="entry.label"
    :iconValue="'mdi mdi-folder'"
    :handleClick="handleClick"
  />
</template>

<script lang="ts">
import { Options, Vue } from "vue-class-component";
import EntryCard from "./EntryCard.vue";

@Options({
  props: {
    entry: Object,
    setFolderViewPath: Function,
  },
  components: {
    EntryCard,
  },
  methods: {
    handleClick() {
      this.setFolderViewPath(this.entry.full_path);
    }
  }
})
export default class FolderCard extends Vue {}
</script>

<style></style>
