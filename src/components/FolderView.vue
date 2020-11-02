<template>
  <div :style="{ margin: '25px', 'margin-left': paddingLeft }">
    {{ path }}
  </div>
</template>

<script lang="ts">
import { Options, Vue } from "vue-class-component";

@Options({
  props: {
    path: String,
    paddingLeft: Number,
  },
})
export default class FolderView extends Vue {}
</script>

<style scoped>
</style>
