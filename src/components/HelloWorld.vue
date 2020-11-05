<template>
  {{ msg }}
</template>

<script lang="ts">
import { Options, Vue } from "vue-class-component";

@Options({
  data() {
    return {
      msg: "Hello World",
    };
  },
})
export default class HelloWorld extends Vue {}
</script>

<style></style>
