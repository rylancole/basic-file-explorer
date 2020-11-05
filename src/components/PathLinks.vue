<template>
  {{ pathList }}
</template>

<script lang="ts">
import { Options, Vue } from "vue-class-component";

@Options({
  props: {
    path: String,
  },
  computed: {
    pathList() {
      return this.path.split("/");
    },
  },
})
export default class PathLinks extends Vue {}
</script>

<style></style>
