<template>
  <div v-for="entry in entries" :key="entry">
    <EntryButton v-bind:label="entry"/>
  </div>
</template>

<script lang="ts">
import { Options, Vue } from 'vue-class-component';
import EntryButton from './EntryButton.vue'

@Options({
  props: {
    entries: Array,
  },
  components: {
    EntryButton
  },
})
export default class FolderView extends Vue {}
</script>

<style>
div {
  display: flex;
  flex-direction: column;
}
</style>
