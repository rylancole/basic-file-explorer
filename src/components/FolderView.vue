<template>
  <div>
    {{ msg }}
  </div>
</template>

<script lang="ts">
import { Options, Vue } from 'vue-class-component';

@Options({
  data () {
    return {
      msg: "Hello World",
    };
  },
});
export default class FolderView extends Vue {};
</script>

<style scoped>
div {
  margin-left: 200px;
  margin-right: 25px;
  margin-top: 25px;
  margin-bottom: 25px;
}
</style>
