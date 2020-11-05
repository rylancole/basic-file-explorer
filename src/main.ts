import { createApp, h } from "vue";
import App from "./App.vue";
// @ts-ignore
import WaveUI from "wave-ui";
import "wave-ui/dist/wave-ui.css";
import "@mdi/font/css/materialdesignicons.min.css";

const app = createApp({
  render: () => h(App),
});

new WaveUI(app, {
  // Some Wave UI options.
});

app.mount("#app");
