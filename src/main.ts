import { createApp } from "vue";
import App from "./App.vue";
import "./styles.css";
import "./styles/luckyWheel.css";
import VueLuckyCanvas from '@lucky-canvas/vue'

createApp(App).use(VueLuckyCanvas).mount("#app");
