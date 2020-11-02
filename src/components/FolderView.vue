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
    paddingLeft: String,
  },
})
export default class FolderView extends Vue {}
</script>

<style></style>
