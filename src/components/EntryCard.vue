<template>
  <w-card
    tile
    no-border
    @click="$emit('entry-click')"
    @dblclick="$emit('entry-dblclick')"
    @contextmenu.prevent="handleOpenMenu"
    :style="cssStyles"
  >
    <w-menu v-model="showMenu" right>
      <template #activator="{ on }">
        <div v-on="on" />
      </template>
      <w-list
        v-model="selectedMenuItem"
        :items="items"
        hover
        @item-click="handleItemClick"
      />
    </w-menu>
    <w-icon class="mr1" size="56px" color="blue">{{ iconValue }}</w-icon>
    <p class="body">{{ shortenedLabel }}</p>
  </w-card>
</template>

<script lang="ts">
import { Options, Vue } from "vue-class-component";

@Options({
  props: {
    label: String,
    iconValue: String,
    isSelected: Boolean,
  },
  data() {
    return {
      showMenu: false,
      items: [{ label: "Move", callback: function () {console.log("I am a function")} }, { label: "Rename", callback: function () {return "I am a function"}  }, { label: "Trash", callback: function () {return "I am a function"} }],
      selectedMenuItem: null,
    };
  },
  emits: ["entry-click", "entry-dblclick", "entry-contextmenu"],
  computed: {
    shortenedLabel() {
      if (this.label.length > 19) {
        return this.label.slice(0, 16) + "...";
      }
      return this.label;
    },
    cssStyles() {
      const bgColor = this.isSelected ? "whitesmoke" : "white";
      return {
        "--bg-color": bgColor,
      };
    },
  },
  methods: {
    handleItemClick(event: any) {
      event.callback();
      this.handleCloseMenu();
    },
    handleOpenMenu() {
      this.showMenu = true;
    },
    handleCloseMenu() {
      this.showMenu = false;
    },
  },
})
export default class EntryCard extends Vue {}
</script>

<style>
.w-card {
  width: 96px;
  text-align: center;
  background-color: var(--bg-color);
  position: relative;
}

.w-card:hover {
  background-color: whitesmoke;
}

p {
  word-wrap: break-word;
}

.menu-icon-container {
  position: absolute;
  right: 10px;
}
</style>
