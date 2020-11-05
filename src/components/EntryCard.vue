<template>
  <w-card no-border @click="doHandleClick">
    <w-icon class="mr1" size="56px" color="blue">{{ iconValue }}</w-icon>
    <p>{{ shortenedLabel }}</p>
  </w-card>
</template>

<script lang="ts">
import { Options, Vue } from "vue-class-component";

@Options({
  props: {
    label: String,
    iconValue: String,
    handleClick: Function,
  },
  computed: {
    shortenedLabel() {
      if (this.label.length > 19) {
        return this.label.slice(0, 16) + "...";
      }
      return this.label;
    },
  },
  methods: {
    doHandleClick(event: any) {
      if (this.handleClick) {
        this.handleClick({ MouseEvent: event, label: this.label });
      }
    },
  },
})
export default class EntryCard extends Vue {}
</script>

<style>
.w-card {
  width: 96px;
  text-align: center;
}

.w-card:hover {
  background-color: whitesmoke;
}

p {
  font-size: 14px;
  word-wrap: break-word;
}
</style>
