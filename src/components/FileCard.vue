<template>
  <EntryCard
    v-if="entry"
    :label="entry.label"
    :iconValue="'mdi mdi-file-outline'"
  />
</template>

<script lang="ts">
import { Options, Vue } from "vue-class-component";
import EntryCard from "./EntryCard.vue";

@Options({
  props: {
    entry: Object,
  },
  components: {
    EntryCard,
  },
})
export default class FileCard extends Vue {}
</script>

<style></style>
