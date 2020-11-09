<template>
  <EntryCard
    v-if="entry"
    :label="entry.label"
    :isSelected="isSelected"
    :iconValue="'mdi mdi-folder'"
    v-on:entry-click="$emit('folder-click', entry)"
    v-on:entry-dblclick="$emit('folder-dblclick', entry)"
  />
</template>

<script lang="ts">
import { Options, Vue } from "vue-class-component";
import EntryCard from "./EntryCard.vue";

@Options({
  props: {
    entry: Object,
    selected: Array,
  },
  components: {
    EntryCard,
  },
  computed: {
    isSelected() {
      return this.selected.includes(this.entry.full_path);
    },
  },
  emits: ["folder-click", "folder-dblclick"],
})
export default class FolderCard extends Vue {}
</script>

<style></style>
