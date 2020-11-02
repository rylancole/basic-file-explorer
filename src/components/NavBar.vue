<template>
  <w-drawer v-model="showNavBar" persistent left no-overlay width="175px">
    <w-list :items="items" hover @click="handleItemClick"/>
  </w-drawer>
</template>

<script lang="ts">
import { Options, Vue } from 'vue-class-component';

@Options({
  props: {
    entries: Array,
  },
  computed: {
    items () {
      let items = [];
      for (let entry of this.entries) {
        items.push({ label: entry });
      }
      return items;
    },
    btnLabel () {
      if(this.showNavBar) {
        return "Close";
      }
      else {
        return "Open";
      }
    }
  },
  methods: {
    handleItemClick (mouseEvent: any) {
      console.log(mouseEvent.target.outerText);
    },
    handleViewToggle () {
      this.showNavBar = !this.showNavBar;
    }
  },
  data () {
    return {
      showNavBar: true,
    }
  },
})
export default class NavBar extends Vue {};
</script>

<style>

</style>
