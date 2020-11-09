<template>
  <w-drawer
    v-model="showNavBar"
    persistent
    left
    no-overlay
    v-bind:width="width"
  >
    <w-list
      v-if="entries.length != 0"
      v-model="selected"
      :items="items"
      hover
      @item-click="handleItemClick"
    />
  </w-drawer>
</template>

<script lang="ts">
import { Options, Vue } from "vue-class-component";
import { DEFAULT_PATH } from "../constants";
const PWD = ".";

@Options({
  props: {
    entries: Array,
    setFolderViewPath: Function,
    width: String,
  },
  methods: {
    handleItemClick(item: { value: String; full_path: String }) {
      this.setFolderViewPath(item.full_path);
      this.selected = item.value;
    },
  },
  data() {
    return {
      showNavBar: true,
      default: {
        value: "",
        label: PWD,
        full_path: DEFAULT_PATH,
        is_folder: true,
      },
      selected: "Local",
    };
  },
  computed: {
    items() {
      let items = [this.default];
      for (let entry of this.entries) {
        if (entry.is_folder) {
          items.push({ value: entry.label, ...entry });
        }
      }
      return items;
    },
  },
})
export default class NavBar extends Vue {}
</script>

<style></style>
