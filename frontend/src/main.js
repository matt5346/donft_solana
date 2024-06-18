import { createApp } from "vue";
import App from "@/App.vue";
import store from "@/store";
import router from "@/router";
import Notifications from "@kyvg/vue3-notification";

import IconComponent from "@/components/IconComponent/Icon";
import "solana-wallets-vue/styles.css";


const app = createApp(App)
  .use(store)
  .use(router)
  .component("IconComponent", IconComponent);

app.use(Notifications);
app.mount("#app");