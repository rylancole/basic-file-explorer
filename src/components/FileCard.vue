<template>
  <EntryCard
    v-if="entry"
    :label="entry.label"
    :isSelected="isSelected"
    :iconValue="'mdi mdi-file-outline'"
    v-on:entry-click="$emit('file-click', entry)"
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
  emits: ["file-click"],
  computed: {
    isSelected() {
      return this.selected.includes(this.entry.full_path);
    },
  },
})
export default class FileCard extends Vue {}
</script>

<style></style>
