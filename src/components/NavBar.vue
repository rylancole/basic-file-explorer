<template>
  <w-drawer v-model="showNavBar" persistent left no-overlay v-bind:width="width">
    <w-list v-model="selected" :items="items" hover @click="handleItemClick" />
  </w-drawer>
</template>

<script lang="ts">
import { Options, Vue } from "vue-class-component";
const PWD = '.';

@Options({
  props: {
    entries: Array,
    setPathEnd: Function,
    width: String,
  },
  methods: {
    handleItemClick(mouseEvent: any) {
      let label = mouseEvent.target.outerText;
      if(label === PWD) {
        this.setPathEnd('');
      } else {
        this.setPathEnd('/'+label);
      }
      this.selected = label;
    },
  },
  data() {
    return {
      showNavBar: true,
      default: { label: PWD, is_folder: true },
      selected: 'Local',
    };
  },
  computed: {
    items() {
      let items = [this.default];
      for (let entry of this.entries) {
        if(entry.is_folder) {
          items.push(entry);
        }
      }
      return items;
    }
  }
})
export default class NavBar extends Vue {}
</script>

<style></style>
