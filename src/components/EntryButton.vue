<template>
  <button @click="handleClick">{{ label }}</button>
</template>

<script lang="ts">
import { Options, Vue } from 'vue-class-component';

@Options({
  props: {
    label: String,
  }, 
  methods: {
    handleClick () {
      console.log("Click "+this.label)
    }
  }
})

export default class EntryButton extends Vue {}
</script>

<!-- Add "scoped" attribute to limit CSS to this component only -->
<style scoped>
button {
  background-color: white;
  border: none;
  text-align: center;
  text-decoration: none;
  display: inline-block;
  font-size: 16px;
}
button:hover {
  background-color: grey;
}
</style>
